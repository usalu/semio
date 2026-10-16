"""✂️ Splits the mutation root (edit half, message ledger mounts), the canonical text file, the format crypto region and the wire-core additions."""
import os, re
from rlib import *

MUT = R + "🎮️mutation/🦀️.rs"
EDIT = R + "🎮️mutation/✏️edit/🦀️.rs"
CANON_TEXT = R + "🚪️io/📝️text/🧵️canonical/🦀️.rs"
FRAG_IDS = R + "🚪️io/📝️text/🧵️canonical/🆔️ids/🦀️.rs"
FRAG_FOREIGN = R + "🚪️io/📝️text/🧵️canonical/🌐️foreign/🦀️.rs"
FRAG_POLICY = R + "🚪️io/📝️text/🧵️canonical/🧾️policy/🦀️.rs"
CANON_MUT = R + "🎮️mutation/🧵️canonical/🦀️.rs"
FORMAT = R + "📐️format/🦀️.rs"
WIRE_CORE = R + "🧾️wire/🦀️.rs"
IDS = R + "🆔️ids/🦀️.rs"
FOREIGN = R + "🎮️mutation/🌐️foreign/🦀️.rs"
MSG_GLUE = R + "🎮️mutation/📨️messages/" + PACKAGE + "/🦀️.rs"

MESSAGE_MODS = ["message_composition", "message_ledger_retirement", "message_accumulation", "message_selection", "message_copy", "replay_retirement", "prepared"]

MOVED = {}

def register_moved(old_file, key, text):
    for name in top_defs(text):
        MOVED[(old_file, name)] = key

def mount_block(text, mod):
    rx = re.compile(r'(?:#\[cfg\(test\)\]\n)?#\[path\s*=\s*"[^"]*"\]\n(?:pub )?mod ' + mod + r';\n(?:pub use ' + mod + r'::[^;]*;\n)*')
    ms = list(rx.finditer(text))
    assert len(ms) == 1, (mod, len(ms))
    m = ms[0]
    return text[:m.start()] + text[m.end():], m.group(0)

def rebase(block, old_dir, new_dir):
    def sub(m):
        absolute = os.path.normpath(os.path.join(old_dir, m.group(1)))
        return '#[path = "' + os.path.relpath(absolute, new_dir) + '"]'
    return re.sub(r'#\[path\s*=\s*"([^"]*)"\]', sub, block)

def replace_region(text, name, new):
    L = text.split("\n")
    hits = [r for r in region_lines(text) if r[0] == name]
    assert len(hits) == 1, (name, hits)
    n, a, b, d = hits[0]
    return "\n".join(L[:a] + ([new.rstrip("\n")] if new else []) + L[b + 1:])

DOC_EDIT = ("//! ✏️ The recorded Edit: the transaction reference, the per-operation mutation meta, the provenance origin and the canonical field cursors that seal them.\n"
            "//!\n"
            "//! Sits between the mutation contract below and the causal envelopes above; the optimistic replica and the authority record the same bytes.\n\n")

def cut_mutation_root():
    t = read_base(MUT)
    msgs = []
    for mod in MESSAGE_MODS:
        t, blk = mount_block(t, mod)
        msgs.append(rebase(blk, os.path.dirname(MUT), os.path.dirname(MSG_GLUE)))
    t, canon_blk = mount_block(t, "canonical_fields")
    meta = region_slice(t, "🔖️Meta")
    origin = region_slice(t, "🔖️Origin")
    meta_wo, outcome = mount_block(meta, "outcome_code_tests")
    t = replace_region(t, "🔖️Meta", outcome)
    t = replace_region(t, "🔖️Origin", "")
    t = re.sub(r"\n{3,}", "\n\n", t).rstrip("\n") + "\n"
    register_moved(MUT, "edit", meta_wo)
    register_moved(MUT, "edit", origin)
    edit_dir = os.path.dirname(EDIT)
    body = rebase(meta_wo, os.path.dirname(MUT), edit_dir) + "\n\n" + origin + "\n\n" + rebase(canon_blk, os.path.dirname(MUT), edit_dir)
    canon_mount = '#[path = "' + os.path.relpath(CANON_TEXT, edit_dir) + '"]\npub mod canonical;\n'
    write(MUT, t)
    write(EDIT, DOC_EDIT + body.rstrip("\n") + "\n\n" + canon_mount)
    glue = ("//! 📨️ Mutation message ledgers glue — wiring only. Domain lives at the owner `🦀️.rs` files.\n\n"
            + "\n".join(b.rstrip("\n") for b in msgs) + "\n")
    return glue

HELPERS = ('fn absent()->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical original protocol field ordinal is absent")}\n'
           'fn selected(ordinal:&mut usize,present:bool)->bool {if !present{return false;}if *ordinal==0{return true;}*ordinal-=1;false}\n')

def cut_canonical_fragments():
    t = read_base(CANON_TEXT)
    its = items(t)
    def target(it):
        head = it["name"] or ""
        m = re.search(r"(?:for\s+)?([A-Z][A-Za-z0-9_]*)\s*(?:<[^>]*>)?\s*$", head)
        return m.group(1) if m else None
    groups = {"ids": ["HybridLogicalTimestamp", "MutationId", "ActorId", "ArtifactId", "SchemaId", "PayloadHash"], "foreign": ["ForeignTarget"], "policy": ["UndoPolicy"]}
    take = {k: [] for k in groups}
    spans = []
    for it in its:
        if it["kind"] != "impl": continue
        ty = target(it)
        for g, names in groups.items():
            if ty in names:
                take[g].append(it["text"]); spans.append((it["start"], it["end"]))
    out = []; last = 0
    for a, b in sorted(spans):
        out.append(t[last:a]); last = b
    out.append(t[last:])
    rest = re.sub(r"\n{3,}", "\n\n", "".join(out))
    assert len(take["ids"]) == 7 and len(take["foreign"]) == 2 and len(take["policy"]) == 1, {k: len(v) for k, v in take.items()}
    def fragment(doc, body, types, parent_types):
        text = "\n".join(body)
        used = bare_ids(text)
        uses = []
        names = [n for n in ("ArtifactCanonicalJsonTree", "ArtifactCanonicalJsonNode as Node", "ArtifactCanonicalJsonText") if n.split(" ")[0] in used or ("Node" in used and n.endswith("Node"))]
        pj = []
        if "ArtifactCanonicalJsonTree" in used: pj.append("ArtifactCanonicalJsonTree")
        if "Node" in used: pj.append("ArtifactCanonicalJsonNode as Node")
        if "ArtifactCanonicalJsonText" in used: pj.append("ArtifactCanonicalJsonText")
        head = doc + "\n"
        if parent_types: head += "use super::{" + ", ".join(parent_types) + "};\n"
        if pj: head += "use semio_framework_pack_json::{" + ", ".join(pj) + "};\n"
        helpers = ""
        if "ValueError" in used:
            vals = ["ValueError"] + (["ValueRefusalKind"] if "absent" in used else [])
            head += "use semio_framework_value::{" + ", ".join(vals) + "};\n"
        if "absent" in used or "selected" in used:
            helpers = "\n" + HELPERS
        return head + helpers + "\n" + text + "\n"
    ids_t = fragment("//! 🆔️ Canonical JSON trees of the replication identifiers and the hybrid logical timestamp.", take["ids"], None, groups["ids"])
    foreign_t = fragment("//! 🌐️ Canonical JSON tree of the foreign target.", take["foreign"], None, ["ForeignTarget"])
    policy_t = fragment("//! 🧾️ Canonical JSON tree of the undo policy.", take["policy"], None, ["UndoPolicy"])
    write(CANON_TEXT, rest)
    write(FRAG_IDS, ids_t); write(FRAG_FOREIGN, foreign_t); write(FRAG_POLICY, policy_t)

def mount_line(owner_file, frag, mod):
    return '\n#[path = "' + os.path.relpath(frag, os.path.dirname(owner_file)) + '"]\nmod ' + mod + ';\n'

def add_fragment_mounts():
    for owner, frag in ((IDS, FRAG_IDS), (FOREIGN, FRAG_FOREIGN), (WIRE_CORE, FRAG_POLICY)):
        t = read_stage_or_base(owner)
        write(owner, t.rstrip("\n") + "\n" + mount_line(owner, frag, "canonical_tree"))

RETIRE_MUT = ["ForeignTarget", "PayloadHash", "UndoPolicy"]

def cut_canonical_retire():
    t = read_base(CANON_MUT)
    its = items(t)
    spans = []
    for it in its:
        txt = it["text"]
        if txt.startswith("semio_framework_value::artifact_retire_struct!(ForeignTarget") or txt.startswith("semio_framework_value::artifact_retire_leaf!(UndoPolicy") or (it["kind"] == "impl" and (it["name"] or "").endswith("for PayloadHash")):
            spans.append((it["start"], it["end"], txt))
    assert len(spans) == 3, [s[2][:60] for s in spans]
    out = []; last = 0
    for a, b, _ in sorted(spans):
        out.append(t[last:a]); last = b
    out.append(t[last:])
    rest = "".join(out)
    rest = rest.replace("use super::{Edit,MutationMeta,ForeignTarget};", "use super::{Edit,MutationMeta};")
    assert "use crate::{PayloadHash,UndoPolicy};\n" in rest
    rest = rest.replace("use crate::{PayloadHash,UndoPolicy};\n", "")
    rest = re.sub(r"\n{3,}", "\n\n", rest)
    write(CANON_MUT, rest)
    return [s[2] for s in sorted(spans)]

def cut_format_crypto():
    t = read_base(FORMAT)
    rest, region = cut_region(t, "🔖️Crypto")
    write(FORMAT, rest)
    return region

def wire_core_additions(crypto_region, combinators):
    t = read_stage_or_base(WIRE_CORE)
    anchor = "//#region 🧪️Tests\n"
    assert t.count(anchor) == 1
    comb_region = "//#region 🔖️Combinators\n" + "\n".join(combinators) + "\n//#endregion 🔖️Combinators\n\n"
    t = t.replace(anchor, crypto_region.rstrip("\n") + "\n\n" + comb_region + anchor)
    use_anchor = "use semio_framework_replication_error::ProtocolError;\n"
    assert t.count(use_anchor) == 1
    t = t.replace(use_anchor, use_anchor + "use semio_framework_replication_crypto::RecordHasher;\n")
    register_moved(FORMAT, "wire-core", crypto_region)
    write(WIRE_CORE, t)

TEST_WIRE = R + "🧾️wire/🧪️tests/🔬️unit/🦀️.rs"
IDS_TEST = R + "🆔️ids/🧪️tests/🔬️unit/🦀️.rs"
FACADE_TEST = FACADE_PKG + "/../../🧪️tests/🔬️facade/🦀️.rs"

def split_wire_tests():
    t = read_base(TEST_WIRE)
    rest, hlc = cut_region(t, "🔖️HybridLogicalTimestamp")
    rest, ident = cut_region(rest, "🔖️Identifiers")
    rest, cat = cut_region(rest, "🔖️ArtifactInferenceCatalog")
    rest = rest.replace("use semio_framework_schema_registry::{artifact_inference_catalog_len, artifact_inference_descriptor_registered, ArtifactInferenceDescriptor, FacetLeaves, register_artifact_inference_descriptor, with_artifact_inference_catalog};\n", "")
    rest = re.sub(r"\n{3,}", "\n\n", rest)
    write(TEST_WIRE, rest)
    ids_test = "//! 🧪️ Hybrid logical timestamp and identifier laws.\n\nuse super::*;\n\n" + hlc + "\n\n" + ident + "\n"
    write(IDS_TEST, ids_test)
    registry_use = "use semio_framework_schema_registry::{artifact_inference_catalog_len, artifact_inference_descriptor_registered, ArtifactInferenceDescriptor, FacetLeaves, register_artifact_inference_descriptor, with_artifact_inference_catalog};\n"
    return registry_use, cat

OUTCOME_TEST = R + "🎮️mutation/🧪️tests/🧪️outcome-code/🦀️.rs"
TXREF_TEST = R + "🎮️mutation/🧪️tests/🧪️transaction-ref/🦀️.rs"

def move_value_refusal_test():
    t = read_base(OUTCOME_TEST)
    rest, removed = cut_items(t, ["replication_value_refusals_keep_kind_through_owned_paths_and_terminal_messages"])
    assert len(removed) == 1
    body = removed[0]
    assert body.lstrip().startswith("#[test]"), body[:60]
    write(OUTCOME_TEST, rest)
    x = read_base(TXREF_TEST)
    write(TXREF_TEST, x.rstrip("\n") + "\n\n" + body.strip("\n") + "\n")

def add_ids_test_mount():
    t = read_stage_or_base(IDS)
    write(IDS, t.rstrip("\n") + '\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n')

if __name__ == "__main__":
    g = cut_mutation_root()
    print(g)
