/** 📦️ Writes the authored Draw scenario's own sealed command-bundle authority (the declaration-only nested Rust package bundle its projection golden was authored against, recovered from the pre-cut taxonomy) into the scenario fixture, byte-preserving every other line. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd(), library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixturePath = join(library, "🧫️fixtures/🖍️draw-source-scenario/🔣️.json");
const historical = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const commandBundle = { directoryKinds: { fsm: historical.semanticDirectoryKinds.fsm }, descendantContract: historical.semanticDescendantContracts["draw-editor-command-bundle-v1"] };
if (commandBundle.descendantContract.realizedNodeCount !== 16 || !commandBundle.directoryKinds.fsm) throw new Error("Historical taxonomy does not carry the declaration-only package bundle");
const text = readFileSync(fixturePath, "utf8"), anchor = '    "commandDirectoryName": "🖱️canvas-pointer-down"\n  },\n';
if (text.includes('"commandBundle"') || text.split(anchor).length !== 2) throw new Error("Owner anchor is missing, ambiguous, or already followed by a command bundle");
const rendered = JSON.stringify(commandBundle, null, 2).split("\n").map((line, index) => (index === 0 ? line : `  ${line}`)).join("\n");
const output = text.replace(anchor, `${anchor}  "commandBundle": ${rendered},\n`);
const parsed = JSON.parse(output);
if (JSON.stringify(parsed.commandBundle) !== JSON.stringify(commandBundle)) throw new Error("Rendered command bundle does not round-trip");
writeFileSync(fixturePath, output);
console.log(JSON.stringify({ nodes: commandBundle.descendantContract.requiredNodes.length, bytes: output.length - text.length }));
