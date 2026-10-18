import { test, expect } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { createHash } from "node:crypto";

const mode = process.env.SRS_MODE!;
const realDir = process.env.SRS_REAL_DIR!;
const oldFile = process.env.SRS_OLD_FILE!;
const out = process.env.SRS_OUT!;
Bun.plugin({
  name: "srs-old-resolve",
  setup(build) {
    if (process.env.SRS_MUTATE) {
      const [from, to] = process.env.SRS_MUTATE.split("=>");
      const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      build.onLoad({ filter: new RegExp(`^${esc(realDir)}/.*$`) }, (args) => ({ contents: fs.readFileSync(args.path, "utf8").split(from!).join(to!), loader: "ts" }));
    }
    build.onResolve({ filter: /^\.\.?\// }, (args) => (mode === "old" && args.importer === oldFile ? { path: path.resolve(realDir, args.path) } : undefined));
  },
});
const trace: string[] = [];
const stub = (): any => new Proxy(function () {}, {
  get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : stub()),
  apply: () => stub(),
  construct: () => stub() as object,
});
const normalize = (source: string) => source.split("\n").map((l) => l.trim()).join("\n").replace(/(\.\.\/)+/g, "../");
const depth: string[] = [];
const it: any = new Proxy((name: string, fn: () => unknown) => { trace.push(`${depth.join(" > ")} :: it ${name} #${createHash("sha256").update(normalize(String(fn))).digest("hex").slice(0, 16)}`); }, { get: (t, k) => (k in t ? (t as any)[k] : stub()) });
const describe: any = (name: string, fn: () => void) => { trace.push(`${depth.join(" > ")} :: describe+ ${name}`); depth.push(name); fn(); depth.pop(); trace.push(`${depth.join(" > ")} :: describe- ${name}`); };
const vitest: any = { describe, it, expect: stub(), vi: stub() };
const dependencies: any = new Proxy({}, { get: () => stub() });
const entry = mode === "old" ? oldFile : path.join(realDir, "🟦️.ts");
const m = await import(entry);
let error: string | undefined;
try { await m.registerTests1(vitest, dependencies, { directory: "/x", url: "file:///x/y.ts" }); } catch (e) { error = e instanceof Error ? `${e.message}\n${e.stack?.split("\n").slice(0, 3).join("\n")}` : String(e); }
fs.writeFileSync(out, JSON.stringify({ mode, error, count: trace.length, trace }, null, 0));
console.log("RESULT", mode, error ?? "ok", trace.length, "events");
test("srs registration trace recorded", () => { expect(trace.length).toBeGreaterThan(0); });
