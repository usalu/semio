import { readFileSync, writeFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/";
const E = "🧰️framework/", F = "✏️s/🔌️plugins/🏗️fem/";
const edits: [string, string, string[]][] = [
  [F + "📦️packages/🟦️typescript/📋️project.json", "verify-combination-stress", ["verify", "fem-combination-stress"]],
  [F + "🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json", "test-material-source", ["verify", "fem2d-material", "source"]],
  [F + "🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json", "test-material-native", ["verify", "fem2d-material", "native"]],
  [F + "🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json", "test-material-source", ["verify", "fem3d-material", "source"]],
  [F + "🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json", "test-material-native", ["verify", "fem3d-material", "native"]],
  [E + "📦️packages/🟦️typescript/📋️project.json", "window-utility-check", ["verify", "window-utility-check"]],
  [E + "📦️packages/🦀️rust/📋️project.json", "test-staged-payload-funding", ["verify", "staged-payload-funding"]],
  [E + "📦️packages/🦀️rust/📋️project.json", "test-retained-pack-encoding", ["verify", "retained-pack-encoding"]],
  [E + "📦️packages/🦀️rust/📋️project.json", "test-window-utility-native", ["verify", "window-utility-native"]],
  [E + "🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📋️project.json", "value-admission-check", ["verify", "value-admission-check"]],
  [E + "🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📋️project.json", "value-admission-native-check", ["verify", "value-admission-check", "--native"]],
  [E + "🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json", "test-native-receiving-original-source", ["verify", "native-receiving-original-source"]],
  [E + "🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json", "test-retained-equality-source", ["verify", "value-retained-equality", "source"]],
  [E + "🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json", "test-retained-equality-native", ["verify", "value-retained-equality", "native"]],
  [E + "🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📋️project.json", "test-retained-equality-source", ["verify", "artifact-reference-retained-equality", "source"]],
  [E + "🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📋️project.json", "test-retained-equality-native", ["verify", "artifact-reference-retained-equality", "native"]],
  [E + "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json", "viewport-presence-check", ["verify", "viewport-presence-check"]],
  [E + "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json", "operation-diagnostic-host-check", ["verify", "operation-diagnostic-host-check"]],
];
for (const [rel, target, words] of edits) {
  const file = root + rel;
  let text = readFileSync(file, "utf8");
  const start = text.indexOf(`"${target}": {`);
  if (start < 0) throw new Error(`missing target ${target} in ${rel}`);
  const key = text.indexOf('"workspaceCommand": [', start);
  if (key < 0) throw new Error(`missing workspaceCommand ${target}`);
  const open = key + '"workspaceCommand": ['.length - 1;
  const close = text.indexOf("]", open);
  const body = text.slice(open + 1, close);
  const multi = body.includes("\n");
  let replacement: string;
  if (multi) {
    const indent = /\n([ \t]*)"/.exec(body)![1]!;
    const closeIndent = /\n([ \t]*)$/.exec(body)![1]!;
    replacement = "\n" + words.map((word) => `${indent}${JSON.stringify(word)}`).join(",\n") + "\n" + closeIndent;
  } else replacement = words.map((word) => JSON.stringify(word)).join(", ");
  text = text.slice(0, open + 1) + replacement + text.slice(close);
  writeFileSync(file, text);
  console.log(`ok ${target}: ${words.join(" ")}`);
}
