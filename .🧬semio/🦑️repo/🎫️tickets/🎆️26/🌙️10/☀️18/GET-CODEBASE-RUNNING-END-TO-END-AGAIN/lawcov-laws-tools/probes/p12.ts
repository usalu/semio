import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { policyEmojiPrefixBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p12-"));
const files: Record<string, string> = {
  "✏️s/🔌️plugins/🧪️probe/plain-dir/a.txt": "",
  "✏️s/🔌️plugins/🧪️probe/plain.ts": "",
  "✏️s/🔌️plugins/🧪️probe/🧭️first/a.txt": "",
  "✏️s/🔌️plugins/🧪️probe/🧭️second/a.txt": "",
  "✏️s/🔌️plugins/🧪️probe/📁️generic/a.txt": "",
  "✏️s/🔌️plugins/🧪️probe/🧾️ok.ts": "",
  "✏️s/🔌️plugins/🧪️probe/🧾️🧭️double.ts": "",
  "✏️s/🔌️plugins/🧪️probe/🧾️ spaced.ts": "",
  "✏️s/🔌️plugins/🧪️probe/🦀rust-bare/a.txt": "",
};
for (const [p, t] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), t); }
spawnSync("git", ["init", "--quiet"], { cwd: root });
console.log(JSON.stringify(policyEmojiPrefixBreaches(root).map((b) => [b.id, b.kind]), null, 0).replaceAll("],[", "],\n["));
rmSync(root, { recursive: true, force: true });
