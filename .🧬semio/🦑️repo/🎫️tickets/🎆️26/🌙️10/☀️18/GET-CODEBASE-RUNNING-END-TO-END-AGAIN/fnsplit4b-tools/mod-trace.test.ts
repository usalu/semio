import { test, expect, mock } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { builtinModules } from "node:module";

const env = process.env;
const mode = env.REG_MODE!;
const realFile = env.REG_REAL_FILE!;
const oldFile = env.REG_OLD_FILE!;
const out = env.REG_OUT!;
const shim = path.join(import.meta.dir, "vitest-shim.ts");
const realDir = path.dirname(realFile);
const argIndex = 0;
const sourceArg = 0;
const stubFn = (): any => new Proxy(function () {}, { get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : stubFn()), apply: () => stubFn(), construct: () => stubFn() as object });
const builtins = new Set(builtinModules);
const escRx = (s: string) => s.replace(/[.*+?^${}()|[\]\\\/]/g, "\\$&");
const namesFrom = (src: string, spec: string) => {
  const names: string[] = [];
  const e = escRx(spec);
  for (const m of src.matchAll(new RegExp(`(?:import|export)\\s+(?:type\\s+)?(?:[\\w$]+\\s*,\\s*)?\\{([^}]*)\\}\\s*from\\s*["']${e}["']`, "g")))
    for (const part of m[1].split(",")) {
      const p = part.trim();
      if (!p || p.startsWith("type ")) continue;
      const nm = p.split(/\s+as\s+/)[0].trim();
      if (/^[\w$]+$/.test(nm)) names.push(nm);
    }
  for (const m of src.matchAll(new RegExp(`\\{([^}]*)\\}\\s*=\\s*await\\s+import\\(\\s*["']${e}["']`, "g")))
    for (const part of m[1].split(",")) {
      const nm = part.trim().split(/\s*:\s*/)[0].trim();
      if (/^[\w$]+$/.test(nm)) names.push(nm);
    }
  return names;
};
const missing = new Map<string, Set<string>>();
const visited = new Set<string>();
const scopeRoot = process.env.REG_SCOPE ? path.resolve(process.env.REG_SCOPE) : realDir;
const inScope = (f: string) => f === oldFile || f.startsWith(scopeRoot + "/");
const walk = (file: string, dir: string) => {
  if (visited.has(file)) return;
  visited.add(file);
  let src = "";
  try { src = fs.readFileSync(file, "utf8"); } catch { return; }
  const specs = new Set<string>();
  for (const m of src.matchAll(/(?:from\s*|import\s*\(\s*|import\s+)["']([^"']+)["']/g)) specs.add(m[1]);
  for (const spec of specs) {
    if (spec.startsWith("node:") || spec.startsWith("bun") || builtins.has(spec) || spec === "vitest") continue;
    let resolved: string | undefined;
    if (spec.startsWith(".")) {
      const abs = path.resolve(dir, spec);
      try { resolved = Bun.resolveSync(abs, dir); } catch { resolved = undefined; }
      if (resolved && inScope(resolved)) { walk(resolved, path.dirname(resolved)); continue; }
      if (resolved && resolved.endsWith(".json")) continue;
      const key = resolved ?? abs;
      const set = missing.get(key) ?? new Set<string>();
      for (const n of namesFrom(src, spec)) set.add(n);
      missing.set(key, set);
      continue;
    }
    const set = missing.get(spec) ?? new Set<string>();
    for (const n of namesFrom(src, spec)) set.add(n);
    missing.set(spec, set);
  }
};
const entryFile = mode === "old" ? oldFile : realFile;
walk(entryFile, mode === "old" ? realDir : path.dirname(realFile));
for (const [spec, names] of missing) mock.module(spec, () => Object.fromEntries([["default", stubFn()], ...[...names].map((n) => [n, stubFn()])]));
Bun.plugin({
  name: "mod-trace-resolve",
  setup(build) {
    build.onLoad({ filter: /\.tsx?$/ }, async (args) => {
      const text = await Bun.file(args.path).text();
      const loader = args.path.endsWith("x") ? "tsx" : "ts";
      if (!inScope(args.path) || !/["']vitest["']/.test(text)) return { contents: text, loader };
      return { contents: text.replace(/(["'])vitest\1/g, JSON.stringify(shim)), loader };
    });
    build.onResolve({ filter: /^\.\.?\// }, (args) => (mode === "old" && args.importer === oldFile ? { path: path.resolve(realDir, args.path) } : undefined));
  },
});
let error: string | undefined;
try { await import(mode === "old" ? oldFile : realFile); } catch (e) { error = e instanceof Error ? `${e.message}`.slice(0, 300) : String(e); }
const { trace } = await import(shim);
fs.writeFileSync(out, JSON.stringify({ mode, error, count: trace.length, trace }));
console.log("RESULT", mode, error ?? "ok", trace.length, "events");
test("module registration trace recorded", () => { expect(true).toBe(true); });
