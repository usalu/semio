import fs from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { pathToFileURL } from "node:url";
const { readRustSourceClosureV1 } = await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const owners = [["retained","🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust"],["remodel-diff","✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust"],["remodel-model","✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust"],["retained-key","🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔎️key/📦️packages/🦀️rust"],["job-context","🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust"],["job-work-grant","🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant/📦️packages/🦀️rust"]];
for (const [name, directory] of owners) {
  const entry = resolve(directory, "🦀️.rs");
  for (const cfg of [[], ["test"]]) {
    const files = await readRustSourceClosureV1([entry], resolve("."), cfg, async path => { try { return await fs.readFile(path); } catch (error) { if (error.code === "ENOENT") return null; throw error; } }, { manifestDirectory: dirname(resolve(directory, "Cargo.toml")), generated: () => undefined });
    console.log(JSON.stringify({ name, cfg, entry, files: files.length, bytes: files.reduce((sum, row) => sum + row.bytes, 0), activeLines: files.reduce((sum, row) => sum + row.activeLines, 0) }));
  }
}

