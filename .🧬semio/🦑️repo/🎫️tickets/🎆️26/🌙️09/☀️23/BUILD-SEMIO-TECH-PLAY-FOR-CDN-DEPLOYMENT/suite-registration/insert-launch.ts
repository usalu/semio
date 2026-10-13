import { readFileSync, writeFileSync } from "node:fs";

type Entry = { name: string; command: string; env?: Record<string, string> };
const nx = "bun nx run";
const rust = (name: string, project: string, args: string): Entry => ({ name, command: `${nx} ${project}:test --skip-nx-cache --excludeTaskDependencies -- long ${args} -- --nocapture`, env: { NEXTEST_SUCCESS_OUTPUT: "immediate" } });
const feature = "--features component-app-assembly ";
const entries: Entry[] = [
  { name: "⚖️test-uncontrolled-owners-retirement🌱️🟦️", command: `${nx} @semio-tech/framework-value:test-uncontrolled-owners-source --skip-nx-cache --excludeTaskDependencies`, env: { NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false" } },
  { name: "⚖️test-retained-structural-refusal🔌️plugin🟦️", command: `${nx} @semio-tech/framework-plugin:test-retained-structural-refusal-source --skip-nx-cache --excludeTaskDependencies`, env: { NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false" } },
  { name: "⚖️test-retained-structural-refusal🔌️plugin🦀️", command: `${nx} @semio-tech/framework-plugin:test-retained-structural-refusal-native --skip-nx-cache --excludeTaskDependencies`, env: { NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NEXTEST_SUCCESS_OUTPUT: "immediate", SEMIO_TEST_ARTIFACT_DIR: "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/retained-structural-refusal" } },
  rust("⚖️test-preparation-law🎥️shooting🦀️", "@semio-tech/shooting-shooting-rs", "--test preparation_law"),
  rust("⚖️test-authoring-law🎥️shooting🦀️", "@semio-tech/shooting-shooting-rs", "--test authoring_law"),
  rust("⚖️test-preparation-law📷️png🦀️", "@semio-tech/stdio-png-rs", "--test preparation_law"),
  rust("⚖️test-preparation-law🪟️bmp🦀️", "@semio-tech/stdio-bmp-rs", "--test preparation_law"),
  rust("⚖️test-preparation-law🕸️dag🦀️", "@semio-tech/dag-dag-rs", "--test preparation_law"),
  rust("⚖️test-preparation-law🧿️stdio-semio🦀️", "@semio-tech/stdio-semio-rs", `${feature}--test preparation_law`),
  rust("⚖️test-preparation-law🌦️epw🦀️", "@semio-tech/stdio-epw-rs", "--test preparation_law"),
  rust("⚖️test-preparation-law🔤️txt🦀️", "@semio-tech/stdio-txt-rs", "--test preparation_law"),
  rust("⚖️test-paged-seal-digest📡️replication🦀️", "@semio-tech/framework-replication-rs", "--test paged_seal_digest"),
  rust("⚖️test-retirement-law🧊️block-3d🦀️", "@semio-tech/block-3d-rs", `${feature}--test retirement_law`),
  rust("⚖️test-retirement-law✒️writer🦀️", "@semio-tech/writer-writer-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law🔌️wires🦀️", "@semio-tech/reasoning-wires-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law📋️forms🦀️", "@semio-tech/forms-forms-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law📐️cad🦀️", "@semio-tech/cad-cad-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law🎬️sequence🦀️", "@semio-tech/sequence-sequence-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law🪐️space🦀️", "@semio-tech/framework-space-space-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law🗂️collection🦀️", "@semio-tech/framework-space-collection-rs", "--test retirement_law"),
  rust("⚖️test-retirement-law🌊️flow🦀️", "semio-framework-os-flow-core", "--test flow_retirement_law"),
];

const anchorName = "⚖️test-original-bulk-vec-retained-oracle🌱️🟦️";
const base = 900.0586096;

for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8");
  const present = entries.filter((entry) => text.includes(`"name": "${entry.name}"`));
  if (present.length) throw new Error(`${file} already holds ${present.map((entry) => entry.name).join(", ")}`);
  const nameAt = text.indexOf(`"name": "${anchorName}"`);
  if (nameAt < 0) throw new Error(`${file} lacks the anchor ${anchorName}`);
  const opener = text.lastIndexOf("    {\n", nameAt);
  const closer = text.indexOf("\n    },\n", nameAt);
  if (opener < 0 || closer < 0 || closer - opener > 2000) throw new Error(`${file} anchor block is not a plain entry`);
  const end = closer + "\n    },\n".length;
  const blocks = entries.map((entry, index) => {
    const order = Number((base + (index + 1) * 1e-9).toFixed(9));
    const object = { name: entry.name, type: "node-terminal", request: "launch", command: entry.command, cwd: "${workspaceFolder}", ...(entry.env ? { env: entry.env } : {}), presentation: { group: "4_gate", order } };
    return JSON.stringify(object, null, 2).split("\n").map((line) => `    ${line}`).join("\n") + ",\n";
  });
  const next = text.slice(0, end) + blocks.join("") + text.slice(end);
  writeFileSync(file, next);
  console.log(`${file}: inserted ${entries.length} entries`);
}
