/** 🔎️ Read-only: which generated crates name a test-support feature gate in their own root or mirror text, and whether they declare that feature. */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const owners = (JSON.parse(readFileSync(join(root, "📋️build-owners.json"), "utf8")) as { owners: { name: string; root: string }[] }).owners;
const rows: string[] = [];
let mention = 0, declared = 0, undeclared = 0;
for (const owner of owners) {
  if (!/^✏️s\/🔌️plugins\/[^/]+\/🗿️artifacts\//u.test(owner.root)) continue;
  const manifestPath = join(root, owner.root, "Cargo.toml");
  if (!existsSync(manifestPath)) continue;
  let toml: any; try { toml = Bun.TOML.parse(readFileSync(manifestPath, "utf8")); } catch { continue; }
  if (toml.lib?.path !== "🦀️.rs" || toml.package?.metadata?.semio?.role !== "library") continue;
  const texts = [join(root, owner.root, "🦀️.rs"), join(root, owner.root, "🪞️mirror/🦀️.rs")].filter(existsSync).map((file) => readFileSync(file, "utf8"));
  const features = new Set<string>();
  for (const text of texts) for (const match of text.matchAll(/cfg\((?:[^\n]*?)feature\s*=\s*"([^"]+)"/gu)) features.add(match[1]!);
  for (const feature of features) {
    mention += 1;
    if (toml.features && feature in toml.features) declared += 1;
    else { undeclared += 1; rows.push(`${owner.name}\t${feature}`); }
  }
}
console.log(`crate-feature mentions in root/mirror: ${mention}; declared ${declared}; undeclared ${undeclared}`);
console.log(rows.slice(0, 60).join("\n"));
