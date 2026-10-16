// 🔒️ Lock propagation: corrects the puzzle artifact locks (non-member dev features) and adds the new path dependencies to every other lock that carries a changed consumer package.
import {DRY, log, read, lockEnsure, lockRemoveDependency, depPackages} from "./dd.ts";
import {existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
const Z = "✏️s/🔌️plugins/🧩️puzzle";
for (const art of ["◻️2d", "🖐️5d"]) {
  const lock = `${Z}/🗿️artifacts/${art}/Cargo.lock`;
  lockRemoveDependency(lock, "semio-s-plugin-puzzle-board-scene", "semio-framework-pack-json");
  lockRemoveDependency(lock, "semio-s-plugin-puzzle-board-scene", "serde_json");
  lockEnsure(lock, "semio-s-plugin-puzzle-jobs", "0.1.0", ["semio-framework-plugin"], []);
  lockEnsure(lock, "semio-s-plugin-puzzle-retained", "0.1.0", ["semio-framework-pack-json", "semio-s-plugin-puzzle-jobs"], []);
}
const I = "✏️s/🔌️plugins/📜️imperative/🧩️extensions";
const ext = ["🎮️control", "📣️effect", "🧠️logic", "🧮️math", "📝️text"].map(e => ({name: `semio-s-plugin-imperative-${e.replace(/^[^a-z]+/u, "")}`, deps: depPackages(`${I}/${e}/📦️packages/🦀️rust/Cargo.toml`, `${I}/${e}/Cargo.toml`)}));
type Consumer = {pkg: string; add: {name: string; deps: string[]}[]};
const consumers: Consumer[] = [
  {pkg: "semio-s-artifact-puzzle-2d", add: [{name: "semio-s-plugin-puzzle-board-scene", deps: ["semio-framework-value"]}, {name: "semio-s-plugin-puzzle-jobs", deps: []}, {name: "semio-s-plugin-puzzle-retained", deps: []}]},
  {pkg: "semio-s-artifact-puzzle-5d", add: [{name: "semio-s-plugin-puzzle-board-scene", deps: ["semio-framework-value"]}, {name: "semio-s-plugin-puzzle-jobs", deps: []}, {name: "semio-s-plugin-puzzle-retained", deps: []}]},
  {pkg: "semio-s-artifact-wfc-2d", add: [{name: "semio-s-wfc-sqlite-bitmap", deps: ["semio-framework-os-kernel", "semio-framework-value"]}]},
  {pkg: "semio-s-artifact-wfc-grid2d", add: [{name: "semio-s-wfc-sqlite-bitmap", deps: ["semio-framework-os-kernel", "semio-framework-value"]}]},
  {pkg: "semio-s-artifact-imperative-procedure-snapshot", add: ext},
  {pkg: "semio-s-artifact-stdio-mp3-mutations", add: [{name: "semio-s-artifact-stdio-mp3-io-metadata", deps: ["semio-s-artifact-stdio-mp3-metadata"]}]},
];
const own = new Set(["✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock", "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock", "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock", "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/Cargo.lock", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/Cargo.lock"]);
const locks = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", "*Cargo.lock"], {cwd: "/Users/ueli/Documents/semio", maxBuffer: 1 << 28}).toString("utf8").split("\0").filter(p => p && !p.startsWith(".🧬semio/") && existsSync(`/Users/ueli/Documents/semio/${p}`));
let touched = 0;
for (const lock of locks) {
  if (own.has(lock)) continue;
  const text = read(lock);
  for (const consumer of consumers) {
    if (!text.includes(`name = "${consumer.pkg}"\n`)) continue;
    for (const d of consumer.add) {
      if (!new RegExp(`name = "${d.name}"\\n`).test(read(lock))) lockEnsure(lock, d.name, "0.1.0", d.deps, [consumer.pkg]);
      else lockEnsure(lock, d.name, "0.1.0", [], [consumer.pkg]);
    }
    touched++;
  }
}
log(`${DRY ? "dry run" : "written"}: ${touched} consumer blocks across other locks`);
