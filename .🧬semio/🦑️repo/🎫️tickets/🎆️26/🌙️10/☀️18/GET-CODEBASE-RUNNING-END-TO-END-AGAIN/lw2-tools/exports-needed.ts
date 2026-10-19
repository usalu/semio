import { writeFileSync } from "node:fs";
const D = "✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/";
const P = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/";
const SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/";
type Entry = { targetFile: string; proposedSubpath: string; symbols: string[]; importers: string[] };
const asset = (artifact: string, example: string, leaf: string, subpath: string, importers: string[]): Entry => ({ targetFile: `${P}${artifact}/${SUB}${example}/🖼️assets/${leaf}/🗣️.dsl.semio`, proposedSubpath: subpath, symbols: ["default (?raw text)"], importers });
const byPackage: Record<string, Record<string, Entry>> = {
  "@semio-tech/puzzle-2d": {},
  "@semio-tech/puzzle-3d-ts": {},
  "@semio-tech/puzzle-3d-rs": {},
  "@semio-tech/puzzle-js": {},
  "@semio-tech/puzzle-wasm": {},
};
const add = (pkg: string, e: Entry) => { byPackage[pkg]![e.targetFile] = e; };
const s2 = [D + "🎭️2d-fixtures/🧪️.story.tsx"], s3 = [D + "🎭️3d-world/🧪️.story.tsx"], s5 = [D + "🎭️5d-timeline/🧪️.story.tsx"];
add("@semio-tech/puzzle-2d", asset("◻️2d", "🌲️concrete-forest", "🌲️forest", "./examples/concrete-forest/dsl", s2));
add("@semio-tech/puzzle-2d", asset("◻️2d", "🏗️nakagin-capsule-tower", "🏢️tower", "./examples/nakagin-capsule-tower/dsl", s2));
for (const pkg of ["@semio-tech/puzzle-3d-ts", "@semio-tech/puzzle-3d-rs"]) { add(pkg, asset("🧊️3d", "🌲️concrete-forest", "🌲️forest", "./examples/concrete-forest/dsl", s3)); add(pkg, asset("🧊️3d", "🏗️nakagin-capsule-tower", "🏢️tower", "./examples/nakagin-capsule-tower/dsl", s3)); }
add("@semio-tech/puzzle-js", asset("🖐️5d", "🌲️concrete-forest", "🌲️forest", "./examples/concrete-forest/dsl", s5));
add("@semio-tech/puzzle-js", asset("🖐️5d", "🏗️nakagin-capsule-tower", "🏢️tower", "./examples/nakagin-capsule-tower/dsl", s5));
add("@semio-tech/puzzle-js", asset("🖐️5d", "🌙️capsule-dream", "🌙️dream", "./examples/capsule-dream/dsl", s5));
add("@semio-tech/puzzle-wasm", { targetFile: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js", proposedSubpath: "(declare the built wasm package as a dependency of the dev composition project, or expose `.`)", symbols: ["BoardSession", "default"], importers: [...s2, ...s3, ...s5] });
const removals = [
  { package: "@semio-tech/framework-os", exportKey: "./modules/store/child/schema", glueFile: "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts", reason: "ArtifactChild schema and parser are owned by the neutral schema module; the glue has no importer left", replacement: "@semio-tech/framework/schema/artifact-reference" },
  { package: "@semio-tech/framework-os", exportKey: "./modules/store/link/schema", glueFile: "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts", reason: "ArtifactLink schema and parser are owned by the neutral schema module; the glue has no importer left", replacement: "@semio-tech/framework/schema/artifact-reference" },
  { package: "@semio-tech/framework-os", exportKey: "./modules/store/blob/schema", glueFile: "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/📦️blob/🧬️schema/🟦️.ts", reason: "BlobRef schema and parser are owned by the neutral schema module; the glue has no importer left", replacement: "@semio-tech/framework/schema/artifact-reference" },
];
const summary = Object.entries(byPackage).map(([pkg, m]) => ({ package: pkg, newExportEntries: Object.keys(m).length, importerEdges: Object.values(m).reduce((n, e) => n + e.importers.length, 0) }));
writeFileSync(new URL("./exports-needed.json", import.meta.url), JSON.stringify({ summary, byPackage, removals }, null, 1) + "\n");
console.log(JSON.stringify(summary));
