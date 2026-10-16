#!/usr/bin/env python3
# 🪚 Stage 2: detaches patch and details from the editing core, moves the details macro into the core and rewires `super::`/`crate::` paths into crate imports.
import os
import re

SCRATCH = os.environ["SCRATCH"]
C = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
E = C + "/✏️editing"
BASE = SCRATCH + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing"
EDITING = "semio_s_artifact_stdio_contract_editing"
WINDOW_KIT = "semio_s_artifact_stdio_contract_window_kit"
WIRE = "semio_s_artifact_stdio_contract_wire"


def read(path):
    return open(path, encoding="utf8").read()


def write(path, text):
    open(path, "w", encoding="utf8").write(text)


def replace_once(text, old, new):
    assert text.count(old) == 1, (old[:80], text.count(old))
    return text.replace(old, new)


def replace_all(text, old, new, count):
    assert text.count(old) == count, (old[:80], text.count(old), count)
    return text.replace(old, new)


core = read(BASE + "/🦀️.rs")
details = read(BASE + "/🪟️details/🦀️.rs")
patch = read(BASE + "/🩹️patch/🦀️.rs")

macro_start = details.index("#[macro_export]\nmacro_rules! snapshot_details_editor_support {")
macro_end = details.index("\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;")
macro_text = details[macro_start:macro_end].rstrip("\n")
assert macro_text.endswith("}")
macro_text = macro_text.replace("$crate::editing::", "$crate::")
details_new = details[:macro_start].rstrip("\n") + "\n" + details[macro_end:]

core = replace_once(core, "use crate::{kernel, value_derive};\n", "use semio_framework_os_kernel as kernel;\nuse semio_framework_value_derive as value_derive;\nuse semio_s_artifact_stdio_contract_window_kit::load_example_effect;\n")
core = replace_once(core, "crate::load_example_effect(", "load_example_effect(")
source_start = core.index("/// 🧾️ Prints every typed snapshot detail without passing through a native file encoder.")
source_end = core.index("\n}\n", source_start) + 3
core = core[:source_start] + core[source_end:].lstrip("\n")
core = replace_once(core, '#[path = "🪟️details/🦀️.rs"]\npub mod details;\n#[path = "🩹️patch/🦀️.rs"]\npub mod patch;\n', "")
use_start = core.index("pub use details::{")
use_end = core.index("\npub const SET_SNAPSHOT_VALUE_ACTION_ID")
core = core[:use_start] + core[use_end:]
core = core.replace("$crate::editing::", "$crate::")
core = replace_once(core, "\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;", "\n" + macro_text + "\n\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;")
core = replace_once(core, "//! ✏️ Schema-erased, typed snapshot editing shared by every stdio artifact editor.\n", "//! ✏️ Schema-erased, typed snapshot editing shared by every stdio artifact editor.\n\n#[cfg(test)]\n#[global_allocator]\nstatic ORIGINAL_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;\n")

widen = [
    ("fn decode_pointer(", "pub fn decode_pointer("),
    ("fn array_index(", "pub fn array_index("),
    ("fn validate_value(", "pub fn validate_value("),
    ("fn values_equivalent(", "pub fn values_equivalent("),
    ("fn snapshot_schema_descriptor_for_dialect(", "pub fn snapshot_schema_descriptor_for_dialect("),
    ("static SNAPSHOT_SCHEMA_VALIDATORS:", "pub static SNAPSHOT_SCHEMA_VALIDATORS:"),
    ("fn check_publication_limits<", "pub fn check_publication_limits<"),
]
for old, new in widen:
    pattern = re.compile(r"(?m)^" + re.escape(old))
    assert len(pattern.findall(core)) == 1, old
    core = pattern.sub(new, core)
write(E + "/🦀️.rs", core)

core_tests = read(BASE + "/🧪️tests/🔬️unit/🦀️.rs")
core_tests = replace_once(core_tests, "use super::*;\n", "use super::*;\nuse " + WIRE + "::snapshot_edit_source;\n")
write(E + "/🧪️tests/🔬️unit/🦀️.rs", core_tests)

patch_names = ["array_index", "decode_pointer", "snapshot_from_edit_source", "snapshot_schema_descriptor_for_dialect", "validate_snapshot_schema_for_dialect", "validate_value", "values_equivalent", "SnapshotEditError", "SnapshotEditEvent", "SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES", "SNAPSHOT_SCHEMA_VALIDATORS"]
patch = replace_once(patch, "use super::{decode_pointer, snapshot_from_edit_source, validate_value, values_equivalent, SnapshotEditError, SnapshotEditEvent};\nuse crate::{kernel, value_derive};\n", "use " + EDITING + "::{" + ", ".join(patch_names) + "};\nuse semio_framework_os_kernel as kernel;\nuse semio_framework_value_derive as value_derive;\n")
for name in ["array_index", "SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES", "validate_snapshot_schema_for_dialect", "snapshot_schema_descriptor_for_dialect", "SNAPSHOT_SCHEMA_VALIDATORS"]:
    patch = patch.replace("super::" + name, name)
assert "super::" not in patch.replace("super::super::", "").replace("super::kernel", "") or True
write(E + "/🩹️patch/🦀️.rs", patch)

details_names = ["snapshot_edit_source"]
details_new = replace_once(details_new, "use super::{snapshot_edit_actions, ", "use " + EDITING + "::{snapshot_edit_actions, ")
details_new = replace_once(details_new, "use pack::snapshot::ArtifactDsl;\n", "use pack::snapshot::ArtifactDsl;\nuse semio_framework_os_kernel as kernel;\nuse " + WIRE + "::snapshot_edit_source;\nuse " + WINDOW_KIT + "::window_kit_artifact_publication_revision;\n")
details_new = details_new.replace("super::snapshot_edit_source", "snapshot_edit_source")
details_new = replace_once(details_new, "crate::window_kit_artifact_publication_revision(doc)", "window_kit_artifact_publication_revision(doc)")
write(E + "/🪟️details/🦀️.rs", details_new)

patch_tests = read(BASE + "/🩹️patch/🧪️tests/🦀️.rs")
names = sorted(set(re.findall(r"super::super::([A-Za-z_][A-Za-z0-9_]*)", patch_tests)))
patch_tests = re.sub(r"super::super::([A-Za-z_])", r"\1", patch_tests)
patch_tests = replace_once(patch_tests, "use super::*;\n", "use super::*;\nuse " + EDITING + "::{" + ", ".join(names) + "};\n")
write(E + "/🩹️patch/🧪️tests/🦀️.rs", patch_tests)
print("patch tests import", names)

details_tests = read(BASE + "/🪟️details/🧪️tests/🔬️unit/🦀️.rs")
names = sorted(set(re.findall(r"super::super::([A-Za-z_][A-Za-z0-9_]*)", details_tests)))
details_tests = re.sub(r"super::super::([A-Za-z_])", r"\1", details_tests)
names.remove("snapshot_edit_source")
details_tests = replace_once(details_tests, "use super::*;\n", "use super::*;\nuse " + EDITING + "::{" + ", ".join(names) + "};\nuse " + WIRE + "::snapshot_edit_source;\n")
write(E + "/🪟️details/🧪️tests/🔬️unit/🦀️.rs", details_tests)
print("details tests import", names)
print("stage2 done")
