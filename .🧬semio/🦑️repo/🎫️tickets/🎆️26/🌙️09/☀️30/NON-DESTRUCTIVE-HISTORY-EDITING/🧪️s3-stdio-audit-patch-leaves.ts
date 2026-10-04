/**
 * 🔎️ S3-STDIO: runs the framework's collecting input reader (`mutationInputAudit`, the `schema mutation-inputs` gate's reader) over
 * every stdio `patch-snapshot` leaf payload schema, including the leaves the central catalog does not list yet, resolving `$ref`s
 * through every `$id`-carrying JSON schema of the stdio plugin. Prints one line per leaf and exits 1 on any finding.
 *
 * Usage (repo root): bun ./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s3-stdio-audit-patch-leaves.ts
 */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { mutationInputAudit } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

const ROOT = "✏️s/🔌️plugins/🗄️stdio";
const documents = new Map<string, Record<string, unknown>>();
const leaves: string[] = [];

function walk(directory: string): void {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "node_modules" || entry.name.startsWith("🗑️")) continue;
      walk(path);
    } else if (entry.name === "🔣️.json") {
      let document: unknown;
      try { document = JSON.parse(readFileSync(path, "utf8")); } catch { continue; }
      if (document && typeof document === "object" && typeof (document as { $id?: unknown }).$id === "string") {
        documents.set((document as { $id: string }).$id, document as Record<string, unknown>);
        if (/\/mutation\/patch-snapshot\/schema\.json$/u.test((document as { $id: string }).$id)) leaves.push(path);
      }
    }
  }
}

walk(ROOT);
let findings = 0;
for (const path of leaves.sort()) {
  const audit = mutationInputAudit(JSON.parse(readFileSync(path, "utf8")), (id) => documents.get(id));
  findings += audit.findings.length;
  const inputs = audit.inputs.map((input) => input.id).join(", ");
  console.log(`${audit.findings.length === 0 ? "ok  " : "FAIL"} ${path.replace(`${ROOT}/🗿️artifacts/`, "")} inputs=[${inputs}]${audit.findings.map((finding) => ` ${finding.code}@${finding.pointer}`).join("")}`);
}
console.log(`${leaves.length} patch-snapshot leaves, ${findings} finding(s)`);
process.exit(findings === 0 ? 0 : 1);
