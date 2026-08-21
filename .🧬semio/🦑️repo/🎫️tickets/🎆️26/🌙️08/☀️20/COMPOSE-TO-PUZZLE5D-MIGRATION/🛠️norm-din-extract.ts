// 🛠️ Ticket-local extraction helper (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION).
// Reads each norm mutation leaf's own 🦠️mutation + 🔺️diff oracle and prints a table row.
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const rows: any[] = [];
for (const entry of readdirSync(root).sort()) {
  const leaf = join(root, entry);
  if (!existsSync(join(leaf, "🦠️mutation/🦀️component.rs"))) continue;
  const mut = readFileSync(join(leaf, "🦠️mutation/🦀️component.rs"), "utf8");
  const dif = readFileSync(join(leaf, "🔺️diff/🦀️component.rs"), "utf8");
  const inv = readFileSync(join(leaf, "↩️inverse/🦀️component.rs"), "utf8");
  const struct = mut.match(/^pub struct ([A-Za-z0-9]+)/m)?.[1] ?? null;
  const fields = [...mut.matchAll(/^\s+pub ([a-z0-9_]+): ([A-Za-z0-9:<>]+),/gm)].map((m) => [m[1], m[2]]);
  const kind = mut.match(/kind: "([^"]+)"/)?.[1] ?? null;
  const record = mut.match(/record: "([^"]+)"/)?.[1] ?? null;
  const renameAll = /serde\(rename_all/.test(mut);
  const diffFields = [...dif.matchAll(/Diff \{\s*([a-z0-9_]+):/g)].map((m) => m[1]);
  const guards = [...dif.matchAll(/MutationOutcome::(fatal|error|empty|new)/g)].map((m) => m[1]);
  const guardConds = [...dif.matchAll(/^\s+if ([^{]+)\{/gm)].map((m) => m[1].trim());
  rows.push({ dir: entry, struct, fields, kind, record, renameAll, diffFields, guards, guardConds, inverseSets: [...inv.matchAll(/base\.([a-z0-9_]+)/g)].map((m) => m[1]) });
}
console.log(JSON.stringify(rows, null, 1));
