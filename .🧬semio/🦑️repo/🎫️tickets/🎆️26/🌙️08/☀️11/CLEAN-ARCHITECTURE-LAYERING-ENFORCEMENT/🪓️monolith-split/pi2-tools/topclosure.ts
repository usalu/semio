// 🧮️ pi2 per-entry closure lister: for each entry .rs file (env ENTRIES = JSON list of repo-relative paths) prints its own test-cfg closure (source + embedded files) through the canonical counter. Output JSON: {entry: [[path, bytes, lines, kind], ...]}. env PKGDIR = facade package dir (manifest dir for env!("CARGO_MANIFEST_DIR")), OUT = output file.
import {readFileSync, writeFileSync} from "node:fs";
import {resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const dir = resolve(ROOT, process.env.PKGDIR!);
const cfg = [...hostCfg, "debug_assertions", "test"];
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as any).code === "ENOENT") return null; throw e; } };
const entries: string[] = JSON.parse(readFileSync(process.env.ENTRIES!, "utf8"));
const out: Record<string, [string, number, number, string][]> = {};
for (const e of entries) {
  try {
    const inputs = await readRustSourceClosureV1([resolve(ROOT, e)], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
    out[e] = inputs.map((i: any) => [i.path.replace(ROOT + "/", ""), i.bytes, i.activeLines, i.kind]);
  } catch (err) { out[e] = [["ERROR " + String(err).slice(0, 200), 0, 0, "error"]]; }
}
writeFileSync(process.env.OUT!, JSON.stringify(out));
