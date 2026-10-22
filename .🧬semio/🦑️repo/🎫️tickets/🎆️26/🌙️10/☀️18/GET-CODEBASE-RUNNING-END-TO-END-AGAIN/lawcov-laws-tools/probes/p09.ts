import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { policyDiscoverCrateDirs, policyScopeKey } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
const root = mkdtempSync(join(realpathSync(tmpdir()), "p09-"));
const files: Record<string, string> = {
  "✏️s/🔌️plugins/🔭️alpha/📦️packages/🦀️rust/Cargo.toml": '[package]\nname = "alpha"\n[package.metadata.semio]\nrole = "plugin"\n',
  "✏️s/🔌️plugins/🔭️alpha/📦️packages/🦀️rust/🦀️.rs": "pub fn a() {}\n",
  "✏️s/🔌️plugins/🔭️alpha/🦀️.rs": "pub fn plugin() {}\n",
  "✏️s/🔌️plugins/🔭️beta/📦️packages/🦀️rust/Cargo.toml": '[package]\nname = "beta"\n[package.metadata.semio]\nrole = "plugin"\n',
  "✏️s/🔌️plugins/🔭️beta/📦️packages/🦀️rust/🦀️.rs": "pub fn a() {}\n",
};
for (const [p, t] of Object.entries(files)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), t); }
try { console.log(JSON.stringify(policyDiscoverCrateDirs(root))); console.log(policyScopeKey(root, "✏️s/🔌️plugins/🔭️alpha/x/y.rs")); } catch (e) { console.log("ERR", String(e).slice(0, 400)); }
rmSync(root, { recursive: true, force: true });
