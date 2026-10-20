import { test, expect, mock } from "bun:test";
import { builtinModules } from "node:module";
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
Bun.plugin({
  name: "reg-old-resolve",
  setup(build) {
    build.onResolve({ filter: /^\.\.?\// }, (args) => {
      const importer = mode === "old" && args.importer === oldFile ? path.join(realDir, "x.ts") : args.importer;
      const target = mode === "old" && args.importer === oldFile ? path.resolve(realDir, args.path) : undefined;
      try { return { path: Bun.resolveSync(target ?? args.path, target ? realDir : path.dirname(importer)) }; } catch { return target ? { path: target } : undefined; }
    });
    build.onResolve({ filter: /^[^./]/ }, (args) => {
      try { Bun.resolveSync(args.path, path.dirname(args.importer)); return undefined; } catch {}
      const id = `${args.path}#${args.importer}`;
      const names: string[] = [];
      try {
        const src = fs.readFileSync(args.importer === oldFile || args.importer.startsWith("reg-stub") ? oldFile : args.importer, "utf8");
        const esc = args.path.replace(/[.*+?^${}()|[\]\\\/]/g, "\\$&");
        for (const m of src.matchAll(new RegExp(`import\\s+(?:type\\s+)?(?:[\\w$]+\\s*,\\s*)?\\{([^}]*)\\}\\s*from\\s*["']${esc}["']`, "g"))) for (const part of m[1].split(",")) { const p = part.trim(); if (!p || p.startsWith("type ")) continue; const nm = p.split(/\s+as\s+/)[0].trim(); if (/^[\w$]+$/.test(nm)) names.push(nm); }
        for (const m of src.matchAll(new RegExp(`export\\s*\\{([^}]*)\\}\\s*from\\s*["']${esc}["']`, "g"))) for (const part of m[1].split(",")) { const nm = part.trim().split(/\s+as\s+/)[0].trim(); if (/^[\w$]+$/.test(nm)) names.push(nm); }
      } catch {}
      stubImports.set(id, names);
      return { path: id, namespace: "reg-stub" };
    });
    build.onLoad({ filter: /.*/, namespace: "reg-stub" }, (args) => {
      const names = [...new Set(stubImports.get(args.path) ?? [])];
      return { loader: "js", contents: `const s = () => new Proxy(function () {}, { get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : s()), apply: () => s(), construct: () => s() });\nexport default s();\n${names.map((n) => `export const ${n} = s();`).join("\n")}\n` };
    });
  },
});
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
    if (spec.startsWith("node:") || spec.startsWith("bun") || builtins.has(spec)) continue;
    let resolved: string | undefined;
    if (spec.startsWith(".")) {
      const abs = path.resolve(dir, spec);
      try { resolved = Bun.resolveSync(abs, dir); } catch { resolved = undefined; }
      if (resolved && inScope(resolved)) { walk(resolved, path.dirname(resolved)); continue; }
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
const logs: string[] = [];
const origLog = console.log;
const origErr = console.error;
console.log = (...a: unknown[]) => { logs.push(a.map(String).join(" ")); };
console.error = (...a: unknown[]) => { logs.push("E " + a.map(String).join(" ")); };
const m = await import(entryFile);
const argsJson = env.FD_ARGS ?? "[]";
let outcome: string;
try {
  const r = await m[fnName](...JSON.parse(argsJson));
  outcome = "ok " + JSON.stringify(r);
} catch (e) { outcome = "throw " + (e instanceof Error ? e.message : String(e)); }
console.log = origLog;
console.error = origErr;
fs.writeFileSync(out, JSON.stringify({ mode, outcome, logs }));
console.log("RESULT", mode, outcome.slice(0, 200), "logs", logs.length, "stubbed", missing.size);
test("fn diff recorded", () => { expect(outcome.length).toBeGreaterThan(0); });
