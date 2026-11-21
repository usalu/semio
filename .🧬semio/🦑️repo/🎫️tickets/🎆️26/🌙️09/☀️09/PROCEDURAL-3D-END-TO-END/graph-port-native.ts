import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
const root = process.cwd();
const output = join(import.meta.dirname, "🗑️generated/graph-port");
await mkdir(output, { recursive: true });
const files = [
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔌️graph-ports/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔌️graph-ports/🔣️.json",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs"
];
const [port, fixture, preview, renderer, outline, geometry, cases, presentation] = await Promise.all([
  ...files.map(file => readFile(join(root, file), "utf8")),
  readFile(join(root, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs"), "utf8"),
  readFile(join(import.meta.dirname, "graph-port-native.test.rs"), "utf8"),
  readFile(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🦀️.rs"), "utf8"),
]);
const section = (source, first, last) => {
  const start = source.indexOf(first);
  const end = source.indexOf(last, start + first.length);
  if (start < 0 || end < 0) throw Error("Missing source section " + first);
  return source.slice(start, end);
};
const fixtureData = JSON.parse(fixture);
const tuples = rows => rows.map(values => "(" + values.map(v => JSON.stringify(v)).join(", ") + ")").join(",\n");
const codecCases = tuples(fixtureData.cases.map(c => [c.widget,c.channel,c.input,c.code,c.target]));
const instanceCases = tuples(fixtureData.cases.filter(c => c.instance).map(c => [c.widget,c.channel,c.input,c.index,c.instance]));
const wireCases = tuples(fixtureData.wireEndpoints.map(c => [c.sourceNode,c.sourceEndpoint,c.sourceChannel,c.sourceCode,c.targetNode,c.targetEndpoint,c.targetChannel,c.targetCode]));
const collisionCases = tuples(fixtureData.interactionCollision.targets.map(c => [c.granularity, c.target]));
const proof = join(output, "actual.rs");
const binary = join(output, "graph-port");
await writeFile(proof, [
  "mod graph_ports { " + port + "\n }",
  "mod dag_wire { " + section(presentation, "pub fn dag_endpoint(", "pub(crate) fn dag_visual_kind") + "\n }",
  cases.replace("CODEC_FIXTURE", codecCases).replace("INSTANCE_FIXTURE", instanceCases).replace("WIRE_FIXTURE", wireCases).replace("COLLISION_FIXTURE", collisionCases).replace("COLLISION_RAW", JSON.stringify(fixtureData.interactionCollision.raw)),
  section(preview, "/// 🎯️ Raw IDs decoded once", "/// 🪪️ Reads exact retained channel packs"),
  section(renderer, "fn port_label", "fn node_record_to_spec"),
  section(geometry, "pub struct PreviewChannelAddress", "#[derive(Clone, Debug, PartialEq, Eq)]"),
  section(outline, "fn node_ports", "/// 🖱️ Publishes the same graph hover target"),
].join("\n"));
const compile = Bun.spawn(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024", "--test", proof, "-C", "strip=debuginfo", "-o", binary], { stdout:"inherit", stderr:"inherit" });
const result = await compile.exited;
if (result !== 0) process.exit(result);
process.exit(await Bun.spawn([binary], { stdout:"inherit", stderr:"inherit" }).exited);
