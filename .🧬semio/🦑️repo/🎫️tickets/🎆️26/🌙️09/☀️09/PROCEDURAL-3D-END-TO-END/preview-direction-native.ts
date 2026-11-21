import { readdir, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const checkout = process.cwd();
const generated = join(import.meta.dirname, "🗑️generated");
const log = await readFile(join(generated, "preview-eval-directional-ports.log"), "utf8").catch(() => "");
const buildDir = process.argv[2] ?? log.match(/^CARGO_BUILD_BUILD_DIR=(.+)$/m)?.[1];
if (!buildDir) throw new Error("Pass the admitted Cargo build directory as the first argument");
const cache = join(buildDir, "prototype/build");
const output = join(generated, "preview-direction");
await mkdir(output, { recursive: true });
const libraries = (await Promise.all((await readdir(cache, { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(async ({ name }) =>
  (await readdir(join(cache, name), { withFileTypes: true })).filter((entry) => entry.isDirectory()).map(({ name: id }) => join(cache, name, id, "out"))))).flat();
const cacheArgs = libraries.flatMap((dir) => ["-L", "dependency=" + dir]);
const jsonCrate = "semio_framework_pack_json";
const jsonOutputs = libraries.filter(path => path.includes("semio-framework-pack-json"));
const jsonArtifacts = (await Promise.all(jsonOutputs.map(async path => (await readdir(path).catch(() => [])).filter(name => name.startsWith("lib" + jsonCrate) && name.endsWith(".rlib")).map(name => join(path, name))))).flat();
const jsonArtifact = jsonArtifacts.find(path => process.argv[3] && path.endsWith("-" + process.argv[3] + ".rlib")) ?? jsonArtifacts.at(-1);
if (!jsonArtifact) throw new Error("The admitted Cargo build has no pack JSON rlib");
const rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024"];
const run = async (argv) => {
  const command = Bun.spawn(argv, { stdout: "inherit", stderr: "inherit" });
  const status = await command.exited;
  if (status !== 0) process.exit(status);
};
const [geometry, edit, viewer, preview, tests] = await Promise.all([
  readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs"), "utf8"),
  readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"), "utf8"),
  readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"), "utf8"),
  readFile(join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs"), "utf8"),
  readFile(join(import.meta.dirname, "preview-direction-native.test.rs"), "utf8"),
]);
const slice = (source, start, end) => {
  const begin = source.indexOf(start);
  const finish = source.indexOf(end, begin + start.length);
  if (begin === -1 || finish === -1) throw Error("Source section absent " + start + " " + end);
  return source.slice(begin, finish);
};
const meshTableSource = join(checkout,"✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/📦️preview-mesh-table/🦀️.rs");
const graphPorts = join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔌️graph-ports/🦀️.rs");
const fixture = join(checkout, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/preview-channel-directions.json");
const inlineFixture = join(dirname(fixture), "preview-inline-overlap.json");
const nativeOverlapFixture = join(dirname(fixture), "preview-retained-overlap.json");
const meshCacheFixture = join(dirname(fixture), "preview-mesh-cache.json");
const largeHoverFixture = join(dirname(fixture), "preview-large-hover-group.json");
const groupEvidence = join(output, "large-hover.json");
const interactionSources = await readFile(join(checkout, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs"), "utf8");
const witnessSource = join(output, "actual.rs");
const witness = join(output, "preview-direction");
await writeFile(witnessSource, [
  tests.replace("DIRECTION_FIXTURE", JSON.stringify(fixture)).replace("GRAPH_PORT_SOURCE", JSON.stringify(graphPorts)).replace("INLINE_FIXTURE", JSON.stringify(inlineFixture)).replace("NATIVE_OVERLAP_FIXTURE", JSON.stringify(nativeOverlapFixture)).replace("MESH_CACHE_FIXTURE", JSON.stringify(meshCacheFixture)).replace("LARGE_HOVER_FIXTURE", JSON.stringify(largeHoverFixture)).replace("LARGE_HOVER_EVIDENCE", JSON.stringify(groupEvidence)),
  slice(interactionSources, "/// 🧭️ Expands a renderer", "/// 🕹️ Decodes `interactionSelect`"),
  slice(preview, "/// 🎯️ Raw IDs decoded once", "/// 🔌️ Geometry endpoints reached"),
  '#[path = '+JSON.stringify(meshTableSource)+'] mod shared_mesh_table;',
  "pub use shared_mesh_table::*;",
  slice(geometry, "pub fn preview_tolerance", "/// 🌊️ The flow-domain id"),
  slice(geometry, "pub fn is_brep_geometry_handle", "/// 🎯️ Headroom"),
  slice(geometry, "fn visit_retained_preview_items", "/// 🖌️ What the preview"),
  slice(geometry, "pub fn preview_mesh_retention_handles", "/// 📨 Extension invocations"),
  "mod editor { use super::*;",
  "#[derive(Clone, Debug, PartialEq)]",
  slice(edit, "pub struct PreviewPayload", "/// 🧊️ Projects a published evaluation"),
  slice(edit, "fn vec3_json", "//#endregion 🔖️PreviewPipeline"),
  "}",
  "mod viewer { use super::*; use crate as preview_eval;",
  slice(viewer, "pub struct ViewPreviewPayload", "/// 🧭️ World-3d selection payload"),
  "}",
].join("\n"));
await run([...rustc, "--test", witnessSource, "-C", "strip=debuginfo",
  "--extern", jsonCrate + "=" + jsonArtifact,
  "--extern", jsonCrate + "=" + jsonArtifact.replace(/\.rlib$/, ".rmeta"),
  ...cacheArgs, "-o", witness,
]);
await run([witness]);
