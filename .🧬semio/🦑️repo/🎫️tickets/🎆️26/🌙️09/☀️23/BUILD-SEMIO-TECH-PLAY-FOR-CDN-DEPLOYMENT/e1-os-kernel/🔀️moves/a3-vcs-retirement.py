#!/usr/bin/env python3
"""♻️ E1 move A3: the `RetireOwned` implementations of version-graph types (`HistoryPageStack`, `ArtifactHistoryKey`,
`ArtifactGroupVisibility`, `Author`) out of the store retirement module into `🌿️vcs/♻️retirement/🦀️.rs`. A foreign
trait for a vcs type must live with the type once vcs is its own crate; this removes the vcs -> store references."""
import os, sys
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"
ret_p, vcs_p, new_p = f"{M}/🏪️store/♻️retirement/🦀️.rs", f"{M}/🌿️vcs/🦀️.rs", f"{M}/🌿️vcs/♻️retirement/🦀️.rs"
if os.path.exists(new_p):
    sys.exit("vcs retirement already exists")
ret, vcs = open(ret_p, encoding="utf-8").read(), open(vcs_p, encoding="utf-8").read()

def cut(text, start_anchor, end_anchor):
    a = text.index(start_anchor)
    b = text.index(end_anchor, a) + len(end_anchor)
    return text[:a] + text[b:], text[a:b]

ret, history = cut(ret, "struct HistoryPageRetirement<T: RetireOwned> {", "    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }\n}\n")
ret, key = cut(ret, "artifact_retire_struct!(crate::os_vcs::ArtifactHistoryKey { index, generation });\n", "\n")
ret, group = cut(ret, "struct GroupVisibilityRetirement {", "    fn retirement_element_copy_bytes() -> usize { size_of::<Self>() }\n}\n")
ret, author = cut(ret, "artifact_retire_struct!(crate::os_vcs::Author { id, name, avatar });\n", "\n")
moved = history + "\n" + key + group + "\n" + author
assert moved.count("crate::os_vcs::") == 6, moved.count("crate::os_vcs::")
moved = moved.replace("crate::os_vcs::", "super::")
assert "os_store" not in moved and "super::super" not in moved

header = '''//! ♻️ Retirement of the version-graph owners: history pages, history keys, group visibility and authors release
//! through the neutral `RetireOwned` contract beside the types they own, so every layer above the version graph
//! (store, sync, composition) retires them without owning their protocol.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetirementStep, controlled::ControlledRetirement};
use std::mem::ManuallyDrop;

'''
os.makedirs(os.path.dirname(new_p), exist_ok=True)
open(new_p, "w", encoding="utf-8").write(header + moved)
open(ret_p, "w", encoding="utf-8").write(ret)
anchor = '#[path = "🌱️genesis/🦀️.rs"]\nmod genesis;\npub use genesis::{ArtifactGenesis, ArtifactGenesisWitness};\n'
assert vcs.count(anchor) == 1
open(vcs_p, "w", encoding="utf-8").write(vcs.replace(anchor, anchor + '\n#[path = "♻️retirement/🦀️.rs"]\nmod retirement;\n'))
print("A3 applied")
