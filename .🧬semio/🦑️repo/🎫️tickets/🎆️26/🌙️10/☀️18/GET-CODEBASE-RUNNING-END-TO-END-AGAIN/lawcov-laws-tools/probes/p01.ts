import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { policyApaBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts";
const root = mkdtempSync(join(tmpdir(), "p01-"));
const files: Record<string, string> = {
  "✏️s/🔌️plugins/🔭️p/🗿️artifacts/🔸️k/ui.ts": "/**\n * explains why localStorage is banned\n */\nconst note = \"fetch(\";\n",
  "✏️s/🔌️plugins/🔭️p/🗿️artifacts/🔸️k/🦀️.rs": "pub fn a() { let _ = HostEffect::Notify; }\n",
};
for (const [p, t] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), t); }
console.log(JSON.stringify(policyApaBreaches(root).map((b) => [b.id, b.line])));
rmSync(root, { recursive: true, force: true });
