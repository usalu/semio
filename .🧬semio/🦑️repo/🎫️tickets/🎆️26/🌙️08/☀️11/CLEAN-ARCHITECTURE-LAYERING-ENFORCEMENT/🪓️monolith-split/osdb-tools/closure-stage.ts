// 🧾️ Closure members of one staged crate. Usage: bun closure-stage.ts <sp dir> <lib> [lib|test]
import {existsSync, readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const [SPD, lib, mode] = process.argv.slice(2);
const STAGE_ROOT = SPD + "/stage";
const STAGE = STAGE_ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db";
const crates: {lib: string; root: string}[] = JSON.parse(readFileSync(SPD + "/crates.json", "utf8"));
const c = crates.find(x => x.lib === lib)!;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p.startsWith(STAGE_ROOT + "/") && !existsSync(p) ? "/Users/ueli/Documents/semio/" + p.slice(STAGE_ROOT.length + 1) : p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const entry = STAGE + "/" + c.root;
const extra = mode === "test" ? ["test", "feature=testkit", "feature=claimed-backend-laws"] : [];
const inputs = await readRustSourceClosureV1([entry], STAGE_ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: entry.replace(/\/🦀️\.rs$/u, "") + "/📦️packages/🦀️rust", generated: () => undefined});
for (const i of inputs) console.log([i.path.replace(STAGE + "/", ""), i.kind, i.bytes, i.activeLines].join("\t"));
