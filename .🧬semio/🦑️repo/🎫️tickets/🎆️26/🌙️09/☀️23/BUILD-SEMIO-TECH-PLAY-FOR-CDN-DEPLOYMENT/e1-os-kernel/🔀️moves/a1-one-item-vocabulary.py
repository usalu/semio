#!/usr/bin/env python3
"""🎮️ E1 move A1: one-item admission vocabulary out of the store root into `🏪️store/🎮️one-item/🦀️.rs`, and the
declared-rows law (`mutation_inverse_rows_declaration_failures`) out of the spr command layer beside the footprint it
admits. Removes the only spr -> store reference. Anchored on exact text; refuses to run twice."""
import sys
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"
store_p, cmd_p, spr_p = f"{M}/🏪️store/🦀️.rs", f"{M}/📡️spr/🎮️command/🦀️.rs", f"{M}/📡️spr/🦀️.rs"
new_p = f"{M}/🏪️store/🎮️one-item/🦀️.rs"
store, cmd, spr = (open(p, encoding="utf-8").read() for p in (store_p, cmd_p, spr_p))

start = store.index("pub const ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS: usize = 65_536;\n")
end_anchor = "/// 🎟️ The Store validates the original child's exact receipt and prepared-root witness.\n"
e = store.index(end_anchor, start)
e = store.index("\n}\n", e) + 3
block = store[start:e]
assert block.count("pub fn validate_original_preparation_step") == 1 and "ArtifactStoreOneItemLiveAuthority" not in block
block = block.replace("[`ArtifactStore::fold_batch_item`]", "`ArtifactStore::fold_batch_item`")

law_doc = "/// 🐘️ The declared-rows law of an aggregate (design §20.5, audit CLOSURE-4)"
ls = cmd.index(law_doc)
le = cmd.index("    failures\n}\n", cmd.index("pub fn mutation_inverse_rows_declaration_failures", ls)) + len("    failures\n}\n")
law = cmd[ls:le]
law = law.replace("crate::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS", "ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS").replace("crate::ArtifactStoreOneItemFootprint", "ArtifactStoreOneItemFootprint")
law = law.replace("admitted by `ArtifactStoreOneItemFootprint::for_gesture`", "admitted by [`ArtifactStoreOneItemFootprint::for_gesture`]")
assert "crate::" not in law

header = '''//! 🎮️ One-item publication admission vocabulary: the fixed one-item ceilings, the exact footprint a lane declares
//! before preparation, the five-currency scheduler grant, the replay checkpoint and the step every retained one-item
//! owner reports, plus the declared-rows law that proves an aggregate's leaves against that ceiling. It names protocol,
//! version-graph and retained-value vocabulary only, never the Store engine, so every facet that prepares one item
//! (canonical edit, history, composition, the Store) depends on it in one direction.
//!
//! See `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT) for the layering.

use crate::os_spr::Mutation;
use crate::os_vcs::VcsError;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::ValueError;

'''
names = ["ARTIFACT_STORE_ONE_ITEM_ID_BYTES", "ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS", "ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES", "ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES", "ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS",
         "ArtifactStoreOneItemCheckpoint", "ArtifactStoreOneItemFootprint", "ArtifactStoreOneItemGrant", "ArtifactStoreOneItemPreparationStep", "mutation_inverse_rows_declaration_failures", "validate_original_preparation_step"]
mount = '#[path = "🎮️one-item/🦀️.rs"]\nmod one_item;\npub use one_item::{' + ", ".join(names) + "};\n"

import os
if os.path.exists(new_p):
    sys.exit("one-item module already exists")
open(new_p, "w", encoding="utf-8").write(header + block.rstrip("\n") + "\n\n" + law)
open(store_p, "w", encoding="utf-8").write(store[:start] + mount + store[e:])
open(cmd_p, "w", encoding="utf-8").write(cmd[:ls] + cmd[le:].lstrip("\n") if cmd[le:].startswith("\n") else cmd[:ls] + cmd[le:])
old = "mutation_input_schema_failures, mutation_inverse_rows_declaration_failures, mutation_label_failures"
assert spr.count(old) == 1
open(spr_p, "w", encoding="utf-8").write(spr.replace(old, "mutation_input_schema_failures, mutation_label_failures"))
print("A1 applied")
