// 🧾️ Lists canonical closure members of one lib root (path, kind, bytes, activeLines) and totals; EXTRA="test,feature=x" adds cfg tokens; ROOT_DIR overrides repo root for sandboxes.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = process.env.ROOT_DIR ?? "/Users/ueli/Documents/semio";
const [entry, pkgDir] = process.argv.slice(2);
const extra = (process.env.EXTRA ?? "").split(",").filter(Boolean);
const target = process.env.TARGET_CFG ? process.env.TARGET_CFG.split(",") : null;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const inputs = await readRustSourceClosureV1([entry!], ROOT, [...(target ?? host), "debug_assertions", ...extra], read, {manifestDirectory: pkgDir!, generated: () => undefined});
let b = 0, l = 0;
for (const i of inputs) { b += i.bytes; l += i.activeLines; if (!process.env.TOTAL_ONLY) console.log([i.path.replace(ROOT + "/", "").replace(/^🧰️framework\/🛍️products\/💻️os\/🔨️modules\//, "OS/"), i.kind, i.bytes, i.activeLines].join("\t")); }
console.log(["TOTAL", "files", inputs.length, "bytes", b, "lines", l, (inputs.length > 32 || b > 262144 || l > 4000) ? "OVER" : (inputs.length > 28 || b > 230000 || l > 3500) ? "over-target" : "ok"].join("\t"));
