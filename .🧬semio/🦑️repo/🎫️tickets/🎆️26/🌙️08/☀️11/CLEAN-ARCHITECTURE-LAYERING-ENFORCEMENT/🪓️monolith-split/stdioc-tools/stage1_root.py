#!/usr/bin/env python3
# 🪚 Stage 1: cuts the 1742-line contract root into the payload-wire, assembly and window-kit nodes (source of truth: the pristine baseline copy).
import os
import re
import sys

SCRATCH = os.environ["SCRATCH"]
BASE = SCRATCH + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
C = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"
text = open(BASE + "/🦀️.rs", encoding="utf8").read()
lines = text.split("\n")


def at(anchor, start=0):
    hits = [i for i, line in enumerate(lines) if line.startswith(anchor) and i >= start]
    assert len(hits) >= 1, anchor
    return hits[0]


def unique(anchor):
    hits = [i for i, line in enumerate(lines) if line.startswith(anchor)]
    assert len(hits) == 1, (anchor, hits)
    return hits[0]


payload_start = unique("//#region 🧾️PayloadWire")
splices_doc = unique("/// 📥️ Reads the change set a text window published")
splices_end = splices_doc
while lines[splices_end] != "}":
    splices_end += 1
payload_end = unique("//#endregion 🧾️PayloadWire")
double_start = unique("//#region 🪆️DoubleOption")
double_end = unique("//#endregion 🪆️DoubleOption")
schema_doc = unique("/// 📚️ The `s.stdio.registry` shared schema documents")
base64_doc = unique("/// 🔡 Encodes bytes with the padded RFC 4648")
macro_doc = unique("/// 📡 Implements the canonical JSON text mutation codec")
example_start = unique("//#region 🎬️ExampleSwitch")
example_end = unique("//#endregion 🎬️ExampleSwitch")
tests_start = unique("#[cfg(test)]") if False else None

macro_end = example_start - 1
while lines[macro_end] != "}":
    macro_end -= 1
assert lines[macro_end] == "}" and macro_end > macro_doc

imports = {
    "FormatDescriptor": "use semio_framework_plugin::io::FormatDescriptor;",
}
plugin_names = ["ArtifactCapability", "ArtifactCapabilityKind", "ArtifactDeclaration", "ArtifactDefinition", "ArtifactDefinitionError", "ArtifactExecutableIdentity", "ArtifactIdentity", "ArtifactIdentityClaim", "ArtifactIdentityNamespace", "ArtifactKindSpec", "ArtifactLocale", "PluginAssemblyError"]


def used(name, body):
    return re.search(r"(?<![A-Za-z0-9_:])" + re.escape(name) + r"\b", body) is not None


def imports_for(body, extra=()):
    out = []
    if used("FormatDescriptor", body):
        out.append("use semio_framework_plugin::io::FormatDescriptor;")
    chosen = [name for name in plugin_names if used(name, body)]
    if len(chosen) == 1:
        out.append("use semio_framework_plugin::" + chosen[0] + ";")
    elif chosen:
        out.append("use semio_framework_plugin::{" + ", ".join(chosen) + "};")
    if used("value_derive", body):
        out.append("use semio_framework_value_derive as value_derive;")
    for alias, crate in (("kernel", "semio_framework_os_kernel"), ("value", "semio_framework_value"), ("diagnostic", "semio_framework_diagnostic"), ("locale", "semio_framework_ui_locale")):
        if re.search(r"(?<![A-Za-z0-9_:])" + alias + r"::", body):
            out.append(f"use {crate} as {alias};")
    coll = [name for name in ["BTreeMap", "BTreeSet"] if used(name, body)]
    if coll:
        out.append("use std::collections::{" + ", ".join(coll) + "};")
    out.extend(extra)
    return out


def join(*chunks):
    return "\n".join(chunks)


payload_a = "\n".join(lines[payload_start:splices_doc - 1])
payload_b = "\n".join(lines[splices_end + 2:payload_end + 1])
payload_b = payload_b.replace("[`window_kit_splices_argument`]", "`window_kit_splices_argument`")
double_body = "\n".join(lines[double_start:double_end + 1])
tail_wire = "\n".join(lines[base64_doc:macro_end + 1])
splices_fn = "\n".join(lines[splices_doc:splices_end + 1])
assembly_body = "\n".join(lines[schema_doc:base64_doc]).rstrip("\n")
assert assembly_body.count('include_str!("../🧬️schema/🔣️.json")') == 1
assembly_body = assembly_body.replace('include_str!("../🧬️schema/🔣️.json")', 'include_str!("../../🧬️schema/🔣️.json")')
example_body = "\n".join(lines[example_start:example_end + 1])

core_source = open(BASE + "/✏️editing/🦀️.rs", encoding="utf8").read()
source_start = core_source.index("/// 🧾️ Prints every typed snapshot detail without passing through a native file encoder.")
source_end = core_source.index("\n}\n", source_start) + 3
snapshot_source = "//#region 🧾️SnapshotSource\n" + core_source[source_start:source_end].rstrip("\n") + "\n//#endregion 🧾️SnapshotSource"
wire_body = join(payload_a, payload_b, "", double_body, "", snapshot_source, "", tail_wire)
wire = join(
    "//! 🧾️ Wire helpers every stdio artifact shares: operation payload JSON, draft splices, ordered rows, the double-option decoder, Base64, semantic fingerprints and the serde operation codec.",
    "",
    "pub use semio_framework_diagnostic as diagnostic;",
    "pub use semio_framework_os_kernel as kernel;",
    "pub use semio_framework_value as value;",
    "",
    "use semio_framework_value::ToValue;",
    "",
    wire_body,
    "",
    '#[cfg(test)]',
    '#[path = "../🧪️tests/🧾️payload-wire/🦀️.rs"]',
    "mod tests;",
    "",
)

wk_imports = imports_for(example_body, extra=["use semio_s_artifact_stdio_contract_wire::{draft_splices_from_json, snapshot_edit_source, DraftSplice};"])
example_body = example_body.replace("&editing::snapshot_edit_source(snapshot)", "&snapshot_edit_source(snapshot)")
wk_body = example_body.replace("//#endregion 🎬️ExampleSwitch", "")
required_anchor = "/// 🧱️ The staged value of ONE declared argument of a framework window-kit verb, as text."
assert wk_body.count(required_anchor) == 1
required_fn_start = wk_body.index("pub fn window_kit_required_text_argument(")
required_fn_end = wk_body.index("\n}\n", required_fn_start) + 3
wk_body = wk_body[:required_fn_end] + "\n" + splices_fn + "\n" + wk_body[required_fn_end:]
wk_body = wk_body.rstrip("\n") + "\n//#endregion 🎬️ExampleSwitch"
window_kit = join(
    "//! 🪟️ Window-kit conventions of the stdio artifacts: example switching, argument readers, addressed and structural tables, revisions and media import.",
    "",
    *wk_imports,
    "",
    wk_body,
    "",
    "#[cfg(test)]",
    '#[path = "../🧪️tests/🪟️window-kit/🦀️.rs"]',
    "mod tests;",
    "",
)

asm_imports = imports_for(assembly_body)
assembly = join(
    "//! 🏗️ Assembly of stdio artifact contributions: schema definitions, executable declarations, native codec receipts, capability ledgers and the validated contribution registry.",
    "",
    *asm_imports,
    "",
    '#[path = "../🪜️definition-hierarchy/🦀️.rs"]',
    "pub mod definition_hierarchy;",
    "use definition_hierarchy::StdioArtifactDefinition;",
    "",
    '#[path = "../../🦀️.rs"]',
    "pub mod registry;",
    "",
    assembly_body,
    "",
    "#[cfg(test)]",
    '#[path = "../🧪️tests/🔬️unit/🦀️.rs"]',
    "mod tests;",
    "",
    "#[cfg(test)]",
    '#[path = "../🧪️tests/📇️contributions/🦀️.rs"]',
    "mod contribution_tests;",
    "",
)


def write(relative, content):
    path = C + "/" + relative
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(content)


write("🧾️payload-wire/🦀️.rs", wire)
write("🪟️window-kit/🦀️.rs", window_kit)
write("🏗️assembly/🦀️.rs", assembly)

unit = open(BASE + "/🧪️tests/🔬️unit/🦀️.rs", encoding="utf8").read()
blocks = re.split(r"\n(?=#\[test\]\n)", unit)
header, tests = blocks[0], blocks[1:]
assert len(tests) == 4, len(tests)
names = [re.search(r"fn ([a-z_0-9]+)", block).group(1) for block in tests]
print(names)
by = dict(zip(names, tests))
write("🧪️tests/🔬️unit/🦀️.rs", "use super::*;\n\n" + "\n".join(by[n].rstrip("\n") + "\n" for n in names if n in ("runtime_capability_standard_vectors_preserve_each_declared_owner", "artifact_assembly_layout_and_identity_match_neutral_budget")))
write("🧪️tests/🧾️payload-wire/🦀️.rs", "use super::*;\nuse base64::Engine;\n\n" + by["standard_base64_matches_the_reference_implementation"].rstrip("\n") + "\n")
write("🧪️tests/🪟️window-kit/🦀️.rs", "use super::*;\n\n" + by["addressed_table_contract_requires_the_complete_cell_address_and_allows_empty_text"].rstrip("\n") + "\n")
print("stage1 done")
