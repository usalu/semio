#!/usr/bin/env bun
/**
 * 🔏️ Slice L-S2 authority edit: removes the launch-file rows from the nested-cargo projection catalog, the launch
 * entries from `🔣️taxonomy.json` (generator contract input/output, the two `os-registry-launch` member kinds), re-pins
 * the catalog digest and records the reseal in the frozen seal ledger. Dry by default; `--apply` writes the three files
 * back to back after a compare-and-swap re-read, or writes nothing.
 */
import { createHash } from "node:crypto";
import { readFileSync, renameSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const paths = {
  asset: join(root, library, "🖼️assets/📽️nested-cargo-package-projection/🔣️.json"),
  taxonomy: join(root, library, "🔣️taxonomy.json"),
  ledger: join(root, library, "🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json"),
};
const ticket = "2026/09/23/DASHBOARD-LAUNCH-COCKPIT", date = "2026-10-07";
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const once = (text: string, anchor: string, label: string): void => {
  const count = text.split(anchor).length - 1;
  if (count !== 1) throw new Error(`${label}: anchor occurs ${count} times, expected exactly once`);
};
const cut = (text: string, anchor: string, label: string, replacement = ""): string => { once(text, anchor, label); return text.replace(anchor, () => replacement); };

const before = { asset: readFileSync(paths.asset, "utf8"), taxonomy: readFileSync(paths.taxonomy, "utf8"), ledger: readFileSync(paths.ledger, "utf8") };

const catalog = JSON.parse(before.asset) as { referenceConsumers: { packageId: string; path: string }[] };
if (JSON.stringify(catalog, null, 2) + "\n" !== before.asset) throw new Error("catalog is not canonical two-space JSON");
const previousSeal = sha(before.asset);
const removed = [
  { index: 1, path: ".vscode/launch.json", reason: "Generated editor launch file: `plugin-registry:generate` no longer renders it and the recording ticket deletes it." },
  { index: 2, path: ".vscode/🧩️launch.seed.jsonc", reason: "Authored seed of the removed editor launch file: no generator reads it and the recording ticket deletes it." },
].map((row) => {
  const consumer = catalog.referenceConsumers[row.index];
  if (consumer?.path !== row.path) throw new Error(`referenceConsumers[${row.index}] is ${JSON.stringify(consumer?.path)}, expected ${JSON.stringify(row.path)}`);
  return { ...row, row: consumer };
});
catalog.referenceConsumers.splice(1, 2);
const asset = JSON.stringify(catalog, null, 2) + "\n", seal = sha(asset);
for (const { path } of removed) if (asset.includes(JSON.stringify(path))) throw new Error(`catalog still names ${path}`);

let taxonomy = before.taxonomy;
const memberKind = (id: string, owner: string, member: string): string => `    ${JSON.stringify(id)}: {\n      "ownerKindIds": [\n        ${JSON.stringify(owner)}\n      ],\n      "memberNames": [\n        ${JSON.stringify(member)}\n      ],\n      "source": "registry"\n    },\n`;
taxonomy = cut(taxonomy, memberKind("os-registry-launch", "registry", "🚀️launch"), "os-registry-launch kind");
taxonomy = cut(taxonomy, memberKind("members-of-os-registry-launch", "os-registry-launch", "🏷️name-prefix"), "members-of-os-registry-launch kind");
taxonomy = cut(taxonomy, `        ".vscode/🧩️launch.seed.jsonc",\n`, "plugin-registry seed input");
taxonomy = cut(taxonomy, `        {\n          "path": ".vscode/launch.json",\n          "inclusion": "tracked"\n        },\n`, "plugin-registry launch output");
taxonomy = cut(taxonomy, `"authorityCatalogSha256": "${previousSeal}"`, "catalog digest pin", `"authorityCatalogSha256": "${seal}"`);
const expected = JSON.parse(before.taxonomy);
if (expected.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogSha256 !== previousSeal) throw new Error("taxonomy does not pin the live catalog");
delete expected.semanticDirectoryMemberKinds["os-registry-launch"];
delete expected.semanticDirectoryMemberKinds["members-of-os-registry-launch"];
const registry = expected.generatorContracts["plugin-registry"];
registry.inputPatterns = registry.inputPatterns.filter((path: string) => path !== ".vscode/🧩️launch.seed.jsonc");
registry.outputRoots = registry.outputRoots.filter((output: { path: string }) => output.path !== ".vscode/launch.json");
expected.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogSha256 = seal;
if (JSON.stringify(JSON.parse(taxonomy)) !== JSON.stringify(expected)) throw new Error("taxonomy text edit differs from the intended structural edit");
if (/launch/u.test(taxonomy)) throw new Error("taxonomy still mentions launch");

const reseal = [
  "      {",
  `        "seal": ${JSON.stringify(seal)},`,
  `        "recordedBy": ${JSON.stringify(ticket)},`,
  `        "evidence": { "revision": ${JSON.stringify(ticket)}, "date": ${JSON.stringify(date)} },`,
  `        "reason": ${JSON.stringify("The dashboard is the only developer control plane: the plugin registry generator stops rendering the editor launch file and reading its seed, and the recording ticket deletes both files. The two reference-consumer rows naming them leave the catalog; packages, mappings, adapters, token transforms and every other consumer row stay byte-for-byte unchanged.")},`,
  `        "consumerDeletions": [`,
  removed.map((row) => `          { "path": ${JSON.stringify(row.path)}, "reason": ${JSON.stringify(row.reason)}, "revision": ${JSON.stringify(ticket)}, "index": ${row.index}, "row": ${JSON.stringify(row.row)} }`).join(",\n"),
  "        ]",
  "      }",
].join("\n");
const ledgerTail = `        ]\n      }\n    ] },\n    "readme-license-owner-leaves-v1"`;
const ledger = cut(before.ledger, ledgerTail, "nested-cargo ledger tail", `        ]\n      },\n${reseal}\n    ] },\n    "readme-license-owner-leaves-v1"`);
const parsedLedger = JSON.parse(ledger) as { catalogs: Record<string, { seal: string; reseals: { seal: string; consumerDeletions?: { index: number; row: unknown }[] }[] }> };
const entry = parsedLedger.catalogs["nested-cargo-packages-v1"]!;
if (entry.reseals.at(-2)?.seal !== previousSeal || entry.reseals.at(-1)?.seal !== seal) throw new Error("ledger chain does not end previous seal -> new seal");
const replay = JSON.parse(asset) as { referenceConsumers: unknown[] };
for (const deletion of [...entry.reseals.at(-1)!.consumerDeletions!].sort((left, right) => left.index - right.index)) replay.referenceConsumers.splice(deletion.index, 0, deletion.row);
if (JSON.stringify(replay, null, 2) + "\n" !== before.asset) throw new Error("reversing the recorded consumer deletions does not reproduce the previous catalog");

console.log(JSON.stringify({ previousSeal, seal, assetBytes: [Buffer.byteLength(before.asset), Buffer.byteLength(asset)], taxonomyBytes: [Buffer.byteLength(before.taxonomy), Buffer.byteLength(taxonomy)], ledgerBytes: [Buffer.byteLength(before.ledger), Buffer.byteLength(ledger)], removed: removed.map(({ index, path }) => ({ index, path })) }, null, 2));
if (!process.argv.includes("--apply")) { console.log("dry run: nothing written (pass --apply)"); process.exit(0); }
for (const key of ["asset", "taxonomy", "ledger"] as const) if (readFileSync(paths[key], "utf8") !== before[key]) throw new Error(`${key} changed while the edit was prepared; nothing written, re-run`);
for (const [key, text] of [["asset", asset], ["taxonomy", taxonomy], ["ledger", ledger]] as const) {
  const temporary = `${paths[key]}.l-s2-${process.pid}`;
  writeFileSync(temporary, text);
  renameSync(temporary, paths[key]);
}
console.log("applied: catalog, taxonomy and ledger written");
