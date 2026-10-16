// 🧮️ Canonical closure counter (readRustSourceClosureV1) on the merged view (build input + carved output) for the seven WP2 crate roots; lib and test rows.
import {readFileSync, existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const VIEW = process.env.VIEW!;
const PLUG = VIEW + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const roots: Record<string, string> = JSON.parse(process.env.ROOTS!);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string): Promise<Uint8Array | null> => {
  try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return new Uint8Array(0); throw e; }
};
const rows: [string, string | undefined, string[]][] = [
  ["lib native", undefined, []],
  ["lib wasm32-wasip2", "wasm32-wasip2", []],
  ["lib wasm32-unknown-unknown", "wasm32-unknown-unknown", []],
  ["lib testing-features", undefined, ["feature=artifact-app-testing", "feature=media-turn-testing", "feature=reserved-producer-testing", "feature=original-actor-context-testing"]],
  ["test native", undefined, ["test"]],
  ["test wasm32-wasip2", "wasm32-wasip2", ["test"]],
];
for (const [crate, rel] of Object.entries(roots)) {
  const entry = PLUG + "/" + rel;
  for (const [label, target, extra] of rows) {
    const cfg = [...cfgOf(target), "debug_assertions", ...extra];
    try {
      const inputs = await readRustSourceClosureV1([entry], VIEW, cfg, read, {manifestDirectory: entry.replace(/\/[^/]*$/, ""), generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log([crate, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", inputs.filter(i => i.kind === "embedded").length].join("\t"));
    } catch (e) { console.log([crate, label, "ERROR", String((e as Error).message).slice(0, 160)].join("\t")); }
  }
}
