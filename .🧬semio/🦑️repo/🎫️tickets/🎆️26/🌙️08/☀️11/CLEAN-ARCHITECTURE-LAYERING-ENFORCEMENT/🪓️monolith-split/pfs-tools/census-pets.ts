// 🧮️ Source-closure rows of the pets crates for the lib target and for the unit-test selection (cfg test, testkit), via the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const P = ROOT + "/🧰️framework/🛍️products/🐾️pets";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const crates: [string, string, string][] = [
  ["semio-framework-pets", P + "/🦀️.rs", P + "/📦️packages/🦀️rust"],
  ["semio-framework-pets-contract", P + "/🔨️modules/📜️contract/🦀️.rs", P + "/🔨️modules/📜️contract/📦️packages/🦀️rust"],
  ["semio-framework-pets-kinematics", P + "/🔨️modules/⚙️kinematics/🦀️.rs", P + "/🔨️modules/⚙️kinematics/📦️packages/🦀️rust"],
  ["semio-framework-pets-clearance", P + "/🔨️modules/🚧️clearance/🦀️.rs", P + "/🔨️modules/🚧️clearance/📦️packages/🦀️rust"],
  ["semio-framework-pets-traversal", P + "/🔨️modules/🧭️traversal/🦀️.rs", P + "/🔨️modules/🧭️traversal/📦️packages/🦀️rust"],
  ["semio-framework-pets-interaction", P + "/🔨️modules/🖱️interaction/🦀️.rs", P + "/🔨️modules/🖱️interaction/📦️packages/🦀️rust"],
  ["semio-framework-pets-character", P + "/🔨️modules/🎭️character/🦀️.rs", P + "/🔨️modules/🎭️character/📦️packages/🦀️rust"],
  ["semio-framework-pets-staging", P + "/🔨️modules/🎬️staging/🦀️.rs", P + "/🔨️modules/🎬️staging/📦️packages/🦀️rust"],
];
const only = process.env.ENTRY_OVERRIDE;
for (const [name, entry, dir] of crates) {
  for (const [label, extra] of [["lib", []], ["test", ["test", ...(name.endsWith("contract") ? ["feature=testkit"] : [])]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...host, "debug_assertions", ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : "ok"].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 160)].join("\t")); }
  }
}
