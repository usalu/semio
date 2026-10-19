import { test, expect } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";

const env = process.env;
const mode = env.REG_MODE!;
const realFile = env.REG_REAL_FILE!;
const oldFile = env.REG_OLD_FILE!;
const out = env.REG_OUT!;
const shim = path.join(import.meta.dir, "vitest-shim.ts");
const realDir = path.dirname(realFile);
Bun.plugin({
  name: "mod-trace-resolve",
  setup(build) {
    build.onResolve({ filter: /^vitest$/ }, () => ({ path: shim }));
    build.onResolve({ filter: /^\.\.?\// }, (args) => (mode === "old" && args.importer === oldFile ? { path: path.resolve(realDir, args.path) } : undefined));
  },
});
let error: string | undefined;
try { await import(mode === "old" ? oldFile : realFile); } catch (e) { error = e instanceof Error ? `${e.message}`.slice(0, 300) : String(e); }
const { trace } = await import(shim);
fs.writeFileSync(out, JSON.stringify({ mode, error, count: trace.length, trace }));
console.log("RESULT", mode, error ?? "ok", trace.length, "events");
test("module registration trace recorded", () => { expect(true).toBe(true); });
