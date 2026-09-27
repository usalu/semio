import { readdir, readFile } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();
const artifacts = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
const formats = ["📷️png", "📸️jpg", "🎞️gif", "🪟️bmp", "🖼️tiff", "🎨️svg", "🔊️wav", "🎵️mp3", "🎥️mp4", "📼️avi", "🗽️obj", "🔺️stl", "🧱️ply", "🧊️gltf", "🏗️ifc", "📐️step", "🖊️dwg", "🖋️dxf", "☁️las", "🧿️semio"];
const setSnapshotMutationDirs = [
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations",
  ...["📦️object", "🔺️mesh", "📊️table", "🧊️brep", "🧰️kit", "🕸️graph", "🔤️text", "🖊️drawing"].map(
    (subset) => `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/${subset}/🧬️schema/🧬️mutations`,
  ),
];

async function walk(directory: string, output: string[] = []): Promise<string[]> {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = join(directory, entry.name);
    if (entry.isDirectory()) await walk(file, output);
    else output.push(file);
  }
  return output;
}

async function assignedFiles(): Promise<string[]> {
  return (await walk(artifacts)).filter((file) => formats.some((format) => file.split("/").includes(format)));
}

async function editors(): Promise<string[]> {
  return (await assignedFiles()).filter((file) => file.endsWith("/✏️editor/🦀️.rs"));
}

async function verifyControllers(): Promise<number> {
  const files = await editors();
  const definitions = (await Promise.all(formats.map(async (format) =>
    Promise.all((await walk(join(artifacts, format))).filter((file) => file.endsWith("/🦀️.rs")).map((file) => readFile(file, "utf8"))),
  ))).flat().join("\n");
  const mismatches: Array<{ file: string; expected: string; found: string[] }> = [];
  for (const file of files) {
    const source = await readFile(file, "utf8");
    const dialect = source.match(/const DIALECT: Dialect = ([A-Z0-9_]+);/)?.[1];
    if (!dialect) throw new Error(`missing editor dialect constant in ${file}`);
    const escaped = dialect.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const definition = definitions.match(new RegExp(`pub const ${escaped}: Dialect = Dialect \\{ artifact_kind: "([^"]+)", standard: StandardId\\("([^"]+)"\\), subset: (?:SubsetId\\("([^"]+)"\\)|SubsetId::ANY) \\};`));
    if (!definition) throw new Error(`missing dialect definition for ${dialect}`);
    const expected = `${definition[1]}@${definition[2]}/${definition[3] ?? "*"}#editor`;
    const found = [...source.matchAll(/"(s\.stdio\.[^"]+#[a-z-]+)"/g)].map((match) => match[1]).filter((value) => value.endsWith("#editor"));
    if (found.length !== 2 || found.some((value) => value !== expected)) mismatches.push({ file, expected, found });
  }
  if (mismatches.length) throw new Error(`controller/dialect mismatches:\n${JSON.stringify(mismatches, null, 2)}`);
  return files.length;
}

async function verifyRollout(): Promise<number> {
  const files = await editors();
  const violations: string[] = [];
  for (const file of files) {
    const source = await readFile(file, "utf8");
    const required = ["SnapshotEditingEditor for", "snapshot_details_window_definition()", "SNAPSHOT_DETAILS_BODY_KEY", "snapshot_details_split_layout"];
    const mode = join(file.slice(0, -"🦀️.rs".length), "🎭️modes/✏️edit/🦀️.rs");
    const modeSource = await readFile(mode, "utf8");
    if (required.slice(0, 3).some((token) => !source.includes(token)) || !modeSource.includes(required[3])) violations.push(file);
  }
  if (violations.length) throw new Error(`incomplete Details/editor rollout:\n${violations.join("\n")}`);
  for (const directory of setSnapshotMutationDirs) {
    const source = await readFile(join(root, directory, "🦀️.rs"), "utf8");
    if (!source.includes("SetSnapshot")) throw new Error(`missing SetSnapshot aggregate leaf: ${directory}`);
  }
  return files.length;
}

async function verifyTruthfulWindows(): Promise<number> {
  const files = (await assignedFiles()).filter((file) => file.endsWith("/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"));
  const violations: string[] = [];
  for (const file of files) {
    const source = await readFile(file, "utf8");
    if (source.includes("MediaWindowKit::editable_window_kind()") || source.includes("ImageWindowKit::editable_window_kind()")) violations.push(file);
    if (source.includes("MeshWindowKit::editable_window_kind()") && (!file.includes("/🧿️semio/") || (!file.includes("/🔺️mesh/") && !file.includes("/🧊️brep/")))) violations.push(file);
  }
  if (violations.length) throw new Error(`advertised no-op native windows:\n${[...new Set(violations)].join("\n")}`);
  return files.length;
}

async function verifyDeadCommands(): Promise<number> {
  const files = await editors();
  const violations: string[] = [];
  for (const file of files) {
    const source = await readFile(file, "utf8");
    if (/^\s*SetVertex,|SeekMedia \{ position_ms: u64 \}|_ => Ok\(Emit::default\(\)\)/m.test(source)) violations.push(file);
  }
  if (violations.length) throw new Error(`dead native commands remain:\n${violations.join("\n")}`);
  return files.length;
}

async function verifyCompactMediaSchemas(): Promise<number> {
  const png = join(artifacts, "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const wav = join(artifacts, "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  for (const file of [join(png, "🩹️patch-pixels/🔣️.json"), join(png, "🩹️patch-pixels/🧬️schema/🔣️.json"), join(png, "🔣️.json"), join(wav, "🩹️patch-data/🔣️.json"), join(wav, "🩹️patch-data/🧬️schema/🔣️.json"), join(wav, "🔣️.json")]) JSON.parse(await readFile(file, "utf8"));
  const pngProtocol = await readFile(join(png, "💾️binary/📡️.protocol.semio"), "utf8");
  const wavProtocol = await readFile(join(wav, "💾️binary/📡️.protocol.semio"), "utf8");
  if (!pngProtocol.includes("record patch-pixels tag=18") || !wavProtocol.includes("record patch-data tag=4")) throw new Error("compact media mutation protocol tags are missing");
  return 2;
}

const command = process.argv[2] ?? "verify";
const result: Record<string, number> = {};
if (command === "verify" || command === "controllers") result.controllers = await verifyControllers();
if (command === "verify" || command === "rollout") result.rollout = await verifyRollout();
if (command === "verify" || command === "truthful-windows") result.truthfulWindows = await verifyTruthfulWindows();
if (command === "verify" || command === "dead-commands") result.deadCommands = await verifyDeadCommands();
if (command === "verify" || command === "compact-media-schemas") result.compactMediaSchemas = await verifyCompactMediaSchemas();
if (!Object.keys(result).length) throw new Error("usage: bun 📜️script.ts <verify|controllers|rollout|truthful-windows|dead-commands|compact-media-schemas>");
console.log(JSON.stringify(result, null, 2));
