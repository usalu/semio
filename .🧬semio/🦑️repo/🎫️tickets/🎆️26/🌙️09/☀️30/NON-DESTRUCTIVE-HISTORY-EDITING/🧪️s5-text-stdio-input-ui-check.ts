/**
 * 🔬️ S5-TEXT-STDIO pre-flight: reads every leaf of the bundle `🧪️s5-text-stdio-input-ui.py --emit-bundle` wrote with the
 * framework's own collecting reader (`mutationInputAudit`, the reader the `schema mutation-inputs` gate and the time-travel
 * editor use) before and after the declarations, resolving cross-document `$ref`s by `$id` over the schema documents on disk
 * (the bundle's own text wins). Fails on every finding the declarations ADD; prints the findings they clear and, per input,
 * whether the reader now yields the declared widget.
 *
 *   bun 🧪️s5-text-stdio-input-ui-check.ts <bundle.json>
 */
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { mutationInputAudit } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

const repo = resolve(import.meta.dir, "../../../../../../..");
const bundlePath = process.argv[2];
if (bundlePath === undefined) throw new Error("usage: bun 🧪️s5-text-stdio-input-ui-check.ts <bundle.json>");
const bundle = JSON.parse(readFileSync(bundlePath, "utf8")) as Record<string, { before: string; after: string }>;
if (Object.keys(bundle).length === 0) throw new Error("refusing to run: the bundle is empty");

const documents = new Map<string, unknown>();
const skipped = new Set(["node_modules", "target", "dist", "🧫️fixtures", "🧪️tests", "🗑️generated", "🔮️oracles", "📚️examples"]);
const index = (directory: string): void => {
  for (const entry of readdirSync(join(repo, directory), { withFileTypes: true })) {
    if (entry.name.startsWith(".") || skipped.has(entry.name)) continue;
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory()) index(path);
    else if (entry.name.endsWith(".json")) {
      try {
        const document = JSON.parse(readFileSync(join(repo, path), "utf8"));
        if (typeof document?.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document);
      } catch {}
    }
  }
};
for (const root of ["✏️s/🔌️plugins/🗄️stdio", "✏️s/🔌️plugins/✒️writer", "✏️s/🔌️plugins/🔱️trinity", "✏️s/🔌️plugins/🌿️vcs", "🧰️framework/🔨️modules"]) index(root);
const after = new Map(documents);
for (const { after: text } of Object.values(bundle)) {
  const document = JSON.parse(text);
  if (typeof document.$id === "string") after.set(document.$id, document);
}

const key = (finding: { code: string; pointer: string }): string => `${finding.code} ${finding.pointer}`;
let added = 0;
let cleared = 0;
let inputs = 0;
for (const [file, texts] of Object.entries(bundle).sort(([left], [right]) => left.localeCompare(right))) {
  const before = mutationInputAudit(JSON.parse(texts.before), (id) => documents.get(id) as never);
  const now = mutationInputAudit(JSON.parse(texts.after), (id) => after.get(id) as never);
  const known = new Set(before.findings.map(key));
  const remaining = new Set(now.findings.map(key));
  inputs += now.inputs.length;
  cleared += [...known].filter((finding) => !remaining.has(finding)).length;
  for (const finding of now.findings) {
    if (known.has(key(finding))) continue;
    added += 1;
    console.log(`ADDED ${file}: ${finding.message}`);
  }
}
console.log(`${Object.keys(bundle).length} leaf file(s), ${inputs} input(s) read after the declarations; ${cleared} finding(s) cleared, ${added} added`);
process.exit(added === 0 ? 0 : 1);
