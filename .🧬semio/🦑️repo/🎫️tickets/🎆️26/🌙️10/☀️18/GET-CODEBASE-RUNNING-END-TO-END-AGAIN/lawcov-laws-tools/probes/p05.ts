import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { policyRawSpawnBreaches, policyStripTsCommentsAndStrings } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p05-"));
writeFileSync(join(root, "📜️script.ts"), "/* c */\nspawnSync(1);\n");
console.log(JSON.stringify(policyRawSpawnBreaches(root).map((b) => [b.id, b.line])), JSON.stringify(policyStripTsCommentsAndStrings("a /* c */ b")));
rmSync(root, { recursive: true, force: true });
