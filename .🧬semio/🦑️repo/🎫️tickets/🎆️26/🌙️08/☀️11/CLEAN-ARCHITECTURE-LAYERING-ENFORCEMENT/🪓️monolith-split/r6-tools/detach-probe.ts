import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { transformTestClosure } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
const root = mkdtempSync(join(tmpdir(), "probe-"));
const layout: Record<string, string> = {
  "🦀️.rs": '#[path = "."]\npub mod m {\n    #[path = "m/🦀️.rs"]\n    mod component;\n    pub use component::*;\n    #[path = "m/mutations/🦀️.rs"]\n    pub mod mutations;\n}\n',
  "m/🦀️.rs": "pub struct Cfg;\npub struct CfgMutation;\npub struct CfgDelta;\n",
  "m/mutations/🦀️.rs": '#[path = "set/🦀️.rs"]\nmod set;\npub use set::S;\n',
  "m/mutations/set/🦀️.rs": 'use super::super::{Cfg, CfgDelta, CfgMutation};\npub struct S;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
  "m/mutations/set/🧪️tests/🦀️.rs": "use super::*;\n#[test]\nfn t() { let _ = Cfg; let _ = CfgMutation; let _ = S; }\n",
};
for (const [p, c] of Object.entries(layout)) { mkdirSync(dirname(join(root, p)), { recursive: true }); writeFileSync(join(root, p), c); }
for (const aggregate of [false]) {
  const r = transformTestClosure(root, join(root, "🦀️.rs"), layout["🦀️.rs"]!, new Set(), { fixtures: false, aggregate, hoist: false, detach: true });
  console.log(r.edits.get(join(root, "m/mutations/set/🧪️tests/🦀️.rs")));
}
rmSync(root, { recursive: true, force: true });
