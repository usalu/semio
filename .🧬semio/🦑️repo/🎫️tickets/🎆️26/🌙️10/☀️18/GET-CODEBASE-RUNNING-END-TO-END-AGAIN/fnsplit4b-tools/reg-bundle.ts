import * as fs from "node:fs";
import * as path from "node:path";
import { createHash } from "node:crypto";

const env = process.env;
const mode = env.REG_MODE!;
const realFile = env.REG_REAL_FILE!;
const oldFile = env.REG_OLD_FILE!;
const fnName = env.REG_FN!;
const out = env.REG_OUT!;
const argIndex = Number(env.REG_VITEST_ARG ?? "0");
const argc = Number(env.REG_ARGC ?? "3");
const sourceArg = Number(env.REG_SOURCE_ARG ?? String(argc - 1));
const realDir = path.dirname(realFile);
const stubImports = new Map<string, string[]>();
const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\/]/g, "\\$&");
const namesFrom = (src: string, spec: string) => {
  const names: string[] = [];
  const e = esc(spec);
  for (const m of src.matchAll(new RegExp(`(?:import|export)\\s+(?:type\\s+)?(?:[\\w$]+\\s*,\\s*)?\\{([^}]*)\\}\\s*from\\s*["']${e}["']`, "g")))
    for (const part of m[1].split(",")) {
      const p = part.trim();
      if (!p || p.startsWith("type ")) continue;
      const nm = p.split(/\s+as\s+/)[0].trim();
      if (/^[\w$]+$/.test(nm)) names.push(nm);
    }
  return names;
};
const stubSrc = (names: string[]) => `const s = () => new Proxy(function () {}, { get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : s()), apply: () => s(), construct: () => s() });\nexport default s();\n${[...new Set(names)].map((n) => `export const ${n} = s();`).join("\n")}\n`;
const missing = new Set<string>();
const plugin: Bun.BunPlugin = {
  name: "reg-stub",
  setup(build) {
    build.onResolve({ filter: /^\.\.?\// }, (args) => {
      if (mode === "old" && args.importer === oldFile) return { path: Bun.resolveSync(path.resolve(realDir, args.path), realDir) };
      return undefined;
    });
    build.onResolve({ filter: /^[^./]/ }, (args) => {
      if (args.path.startsWith("node:") || args.path === "bun" || args.path.startsWith("bun:")) return undefined;
      try { Bun.resolveSync(args.path, path.dirname(args.importer)); return undefined; } catch {}
      const id = `${args.path}#${args.importer}`;
      let src = "";
      try { src = fs.readFileSync(args.importer, "utf8"); } catch {}
      stubImports.set(id, namesFrom(src, args.path));
      missing.add(args.path);
      return { path: id, namespace: "reg-stub" };
    });
    build.onLoad({ filter: /.*/, namespace: "reg-stub" }, (args) => ({ loader: "js", contents: stubSrc(stubImports.get(args.path) ?? []) }));
  },
};
const entry = mode === "old" ? oldFile : realFile;
const dir = fs.mkdtempSync("/tmp/reg-bundle-");
const built = await Bun.build({ entrypoints: [entry], target: "bun", outdir: dir, plugins: [plugin], naming: "bundle.js" });
if (!built.success) { console.log("BUILD FAILED", built.logs.map(String).slice(0, 5).join("\n")); process.exit(2); }
const trace: string[] = [];
const stub = (): any => new Proxy(function () {}, { get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : stub()), apply: () => stub(), construct: () => stub() as object });
const normalize = (source: string) => source.split("\n").map((l) => l.trim()).join("\n").replace(/(\.\.\/)+/g, "../");
const depth: string[] = [];
const hash = (fn: unknown) => createHash("sha256").update(normalize(String(fn))).digest("hex").slice(0, 16);
const recorder = (kind: string): any => {
  const call = (...a: any[]) => {
    const name = typeof a[0] === "string" ? a[0] : "";
    const fn = a.find((x) => typeof x === "function");
    if (typeof a[0] !== "string" && !fn) return recorder(`${kind}()`);
    const nests = kind.startsWith("describe") || kind === "suite";
    trace.push(`${depth.join(" > ")} :: ${kind}${nests ? "+" : ""} ${name} #${fn && !nests ? hash(fn) : "none"}`);
    if (nests && fn) { depth.push(name); const r = fn(); depth.pop(); trace.push(`${depth.join(" > ")} :: ${kind}- ${name}`); return r; }
    return undefined;
  };
  return new Proxy(call, { get: (t, k) => (typeof k === "string" && k !== "then" ? recorder(`${kind}.${k}`) : (t as any)[k]), apply: (_t, _this, a) => call(...a) });
};
const vitest: any = new Proxy({}, { get: (_t, k) => (k === "expect" || k === "vi" ? stub() : typeof k === "string" && k !== "then" ? recorder(k) : undefined) });
const bag: any = new Proxy({}, { get: () => stub() });
let error: string | undefined;
try {
  const m = await import(path.join(dir, "bundle.js"));
  const args: any[] = [];
  for (let i = 0; i < argc; i += 1) args.push(i === argIndex ? vitest : i === sourceArg ? { directory: "/x", url: "file:///x/y.ts" } : bag);
  await m[fnName](...args);
} catch (e) { error = e instanceof Error ? `${e.message}\n${e.stack?.split("\n").slice(0, 3).join("\n")}` : String(e); }
fs.writeFileSync(out, JSON.stringify({ mode, error, count: trace.length, trace, stubbed: [...missing].slice(0, 40) }));
fs.rmSync(dir, { recursive: true, force: true });
console.log("RESULT", mode, error ?? "ok", trace.length, "events", "stubbed:", missing.size);
