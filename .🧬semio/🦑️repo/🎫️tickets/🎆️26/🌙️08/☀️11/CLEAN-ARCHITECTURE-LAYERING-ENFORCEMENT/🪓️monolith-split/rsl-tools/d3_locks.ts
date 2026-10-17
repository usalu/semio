// 🔗️ D3 lock propagation: adds the new package `semio-framework-os-store-engine-bundle` (with the dependency list of its manifest) and the edge engine -> bundle to every Cargo.lock that holds `semio-framework-os-store-engine`, and removes the edge engine -> diagnostic that the spill made unused. Dry run unless --write.
import {readFileSync} from "node:fs";
import {propagateEdges, propagateRemovals, DRY, log, abs} from "./lib.ts";
const NAME = "semio-framework-os-store-engine-bundle";
const manifest = readFileSync(abs("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/📦️packages/🦀️rust/Cargo.toml"), "utf8");
const deps = [...manifest.split("[dependencies]")[1]!.matchAll(/^([A-Za-z0-9_\-]+)\s*=/gm)].map(m => m[1]!);
log(`${DRY ? "(dry) " : ""}bundle dependencies: ${deps.length}`);
const {changed, skipped} = propagateEdges([{consumer: "semio-framework-os-store-engine", dep: NAME, dev: false}], [{name: NAME, version: "0.1.0", deps}]);
log(`edge engine->bundle: ${changed.length} locks changed`);
for (const s of skipped) log("  skipped " + s);
const removed = propagateRemovals([{consumer: "semio-framework-os-store-engine", dep: "semio-framework-diagnostic", dev: false}]);
log(`edge engine->diagnostic removed in ${removed.length} locks`);
