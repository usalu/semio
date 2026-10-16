// 🧮️ Census rows (canonical readRustSourceClosureV1) of the facade and the producer crates of the dashboard carve, lib and cfg-test selections.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const D = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const P = "📦️packages", R = "🦀️rust", RS = "🦀️.rs";
const crates: [string, string[], boolean][] = [
  ["semio-framework-repo-dashboard", [P, R, RS], false],
  ["-core", ["🧱️core", RS], true],
  ["-launch", ["🧾️launch", RS], false],
  ["-repo-domain", ["🏛️repo-domain", RS], false],
  ["-registry", ["🎮️registry", RS], false],
  ["-inventory", ["📚️inventory", RS], false],
  ["-wire", ["🌀️daemon", "🧵️wire", RS], false],
  ["-preferences", ["⚙️preferences", RS], false],
  ["-labels", ["🖥️terminal", "🌐️labels", RS], false],
  ["-daemon", ["🌀️daemon", RS], false],
  ["-terminal", ["🖥️terminal", RS], false],
  ["-cli", ["🧭️cli", RS], false],
];
for (const [name, parts, testkit] of crates) {
  const entry = [D, ...parts].join("/");
  const dir = name.startsWith("semio") ? [D, P, R].join("/") : [D, ...parts.slice(0, -1), P, R].join("/");
  for (const [label, extra] of [["lib", []], ["test", ["test", ...(testkit ? ["feature=testkit"] : [])]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      const over = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER-GATE" : (inputs.length > 28 || bytes > 230000 || lines > 3500 ? "over-target" : "ok");
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), over].join("\t"));
      if (process.env.LIST === name + ":" + label) for (const i of inputs) console.log("    ", i.kind, i.bytes, i.activeLines, i.path.replace(/^.*🎛️dashboard\//, ""));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
