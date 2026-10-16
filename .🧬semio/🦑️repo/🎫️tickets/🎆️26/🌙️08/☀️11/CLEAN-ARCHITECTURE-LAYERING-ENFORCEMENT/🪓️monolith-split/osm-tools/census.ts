import {readFileSync, existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const LIVE = "/Users/ueli/Documents/semio";
const SNAP = process.argv[2] === "snap" ? LIVE + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/osm-work" : null;
const M = "🧰️framework/🛍️products/💻️os/🔨️modules";
const crates: [string, string, string, string[]][] = [
  ["semio-framework-os-services", `${M}/🛎️services/📦️packages/🦀️rust`, "services", ["feature=default"]],
  ["semio-framework-os-mcp-http-transport", `${M}/🌉️mcp/📮️http-transport/📦️packages/🦀️rust`, "mcp", []],
  ["semio-framework-os-mcp-hub-remote", `${M}/🌉️mcp/🌍️hub-remote/📦️packages/🦀️rust`, "mcp", []],
  ["semio-framework-os-mcp-bridge-codec", `${M}/🌉️mcp/🧶️bridge-codec/📦️packages/🦀️rust`, "mcp", []],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
for (const [name, dir, snapKey, feats] of crates) {
  const root = SNAP ? `${SNAP}/snap-${snapKey}` : LIVE;
  const readFile = async (path: string): Promise<Uint8Array | null> => { try { return readFileSync(path); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
  const manifestDirectory = `${root}/${dir}`;
  const libRel = name.includes("services") ? `${M}/🛎️services/🦀️.rs` : `${M}/🌉️mcp/${dir.split("/").slice(-3)[0]}/🦀️.rs`;
  const lib = name.includes("services") ? `${manifestDirectory}/🦀️.rs` : `${root}/${libRel}`;
  for (const [label, cfg] of [["lib", [...hostCfg, "debug_assertions", ...feats]], ["lib+test", [...hostCfg, "debug_assertions", "test", ...feats]]] as [string, string[]][]) {
    try {
      const inputs = await readRustSourceClosureV1([lib], root, cfg, readFile, {manifestDirectory, generated: () => undefined});
      console.log(`${SNAP ? "BEFORE" : "AFTER "} ${name} ${label}: files=${inputs.length} bytes=${inputs.reduce((s, i) => s + i.bytes, 0)} activeLines=${inputs.reduce((s, i) => s + i.activeLines, 0)}`);
    } catch (e) { console.log(`${name} ${label}: ERROR ${String((e as Error).message).slice(0, 160)}`); }
  }
}
