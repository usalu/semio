// 🧮️ Canonical-counter census over an in-memory overlay: reads {abs path -> text} from OVERLAY, falls back to disk, and counts each given crate root for plain/test/test+features selections.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const overlay: Record<string, string> = process.env.OVERLAY ? JSON.parse(readFileSync(process.env.OVERLAY, "utf8")) : {};
const deleted = new Set<string>(process.env.DELETED ? JSON.parse(readFileSync(process.env.DELETED, "utf8")) : []);
const read = async (p: string) => {
  if (deleted.has(p)) return null;
  if (p in overlay) return new TextEncoder().encode(overlay[p]);
  try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return !p.endsWith(".rs") ? new Uint8Array() : null; throw e; }
};
const specs: {name: string; root: string; pkg: string; features?: string[]}[] = JSON.parse(readFileSync(process.env.SPECS!, "utf8"));
const out: string[] = [];
for (const s of specs) {
  for (const [vn, extra] of [["lib", []], ["test", ["test", "feature=testkit"]], ["test+fix", ["test", "feature=testkit", "feature=integration-fixtures"]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([s.root], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: s.pkg, generated: () => undefined});
      const bytes = inputs.reduce((a, i) => a + i.bytes, 0), lines = inputs.reduce((a, i) => a + i.activeLines, 0);
      const flag = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500 ? "warn" : "ok");
      out.push([s.name, vn, inputs.length, bytes, lines, flag].join("\t"));
    } catch (e) { out.push([s.name, vn, "ERROR", String((e as Error).message).slice(0, 180)].join("\t")); }
  }
}
console.log(out.join("\n"));
