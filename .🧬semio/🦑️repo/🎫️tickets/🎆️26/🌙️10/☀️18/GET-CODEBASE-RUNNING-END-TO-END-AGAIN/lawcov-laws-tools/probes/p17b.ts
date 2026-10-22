import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { policySchemaOverhaulPCBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/📐️schema-overhaul-pc/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p17-"));
const S = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any";
const files: Record<string, string> = { [S + "/🧬️schema/📸️snapshot/x"]: "", [S + "/🧬️schema/🔺️diff/📝️text/📖️.grammar.semio"]: "garbage\n" };
for (const [p, t] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), t); }
console.log(JSON.stringify(policySchemaOverhaulPCBreaches(root).map((b) => [b.id.replace(S, "S"), b.priority])));
rmSync(root, { recursive: true, force: true });
