// 🔒️ Patches every lockfile that lists the wfc engine with the carved engine crates (after the multi-version lock fix).
import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {patchCargoLock} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const WFC = "✏️s/🔌️plugins/🀄️wfc";
const FACADE = "semio-s-plugin-wfc-engine";
const deps: Record<string, string[]> = {foundation: [], kernel: ["foundation"], job: ["foundation"], solve: ["foundation", "job", "kernel"], lattice: ["foundation", "kernel", "solve"]};
const carved = Object.entries(deps).map(([id, list]) => ({name: `${FACADE}-${id}`, dependsOn: list.map((dep) => `${FACADE}-${dep}`)}));
const paths = ["Cargo.lock", "🏭️bridge/Cargo.lock", "🗿️artifacts/◻️2d/Cargo.lock", "🗿️artifacts/🔲️grid2d/Cargo.lock", "🗿️artifacts/🖼️bitmap/Cargo.lock", "🗿️artifacts/🧊️3d/Cargo.lock", "🗿️artifacts/🧱️grid3d/Cargo.lock"].map((path) => `${WFC}/${path}`).concat(["🌎️hub/Cargo.lock"]);
const oracleBlock = `[[package]]\nname = "${FACADE}-oracle"\nversion = "0.1.0"\ndependencies = [\n${["semio-framework-dispatch-macros", "semio-framework-geometry", "semio-framework-graph-engine", "semio-framework-job", "semio-framework-job-publication", "semio-framework-job-worker", "semio-framework-job-worker-authority", "semio-framework-os-kernel", "semio-framework-pack", "semio-framework-pack-json", "semio-framework-value", "semio-framework-value-derive", `${FACADE}-foundation`, `${FACADE}-job`, "serde_json"].map((entry) => ` "${entry}",`).join("\n")}\n]\n`;
for (const path of paths) {
  let text = patchCargoLock(readFileSync(join(ROOT, path), "utf8"), FACADE, carved);
  if (path === `${WFC}/Cargo.lock`) {
    const blocks = text.split(/\n(?=\[\[package\]\]\n)/u);
    const out = blocks.flatMap((block) => {
      const name = /name = "([^"]+)"/u.exec(block)?.[1];
      if (name === `${FACADE}-kernel` || name === `${FACADE}-solve` || name === `${FACADE}-job`) block = block.replace(`"${FACADE}-lattice",`, `"${FACADE}-lattice",`).replace(/(dependencies = \[\n)([\s\S]*?)(\n\])/u, (_, head, body, tail) => `${head}${[...body.split("\n"), ` "${FACADE}-oracle",`].sort().join("\n")}${tail}`);
      return name === `${FACADE}-solve` ? [oracleBlock.trimEnd(), block] : [block];
    });
    text = out.map((block) => block.trimEnd()).join("\n\n") + "\n";
  }
  writeFileSync(join(ROOT, path), text);
  console.log(`patched ${path}`);
}
