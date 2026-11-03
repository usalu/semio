"""🚿 EN 1992 `members[].tightness` set and clear kinds: `change-member-tightness {memberId, newTightness}` and `clear-member-tightness {memberId}`.

`python -I en1992_tightness_kinds.py` writes the two leaves (payload, diff, inverse, four schema twins, kind manifest), mounts them in `member-fields`, registers them in the mutation root (use, enum, KINDS, ts, json union), the edit rules
(set only: `SetFieldRule` has no clear), the binary protocol (tags allocated after the highest existing tag), the text codec, the oracle manifest, the mutate harness (py, rs, feature), the leaf taxonomy and the committed vectors. Idempotent. Needs the diff change of
`en1992_tightness_diff.py` (patch fields `tightness`, `clearTightness`).

📎 [fire-rating kind used as the enum template](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-member-fire-rating/🦀️.rs)
"""
import copy, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ANY = NORM + "/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = ANY + "/🧬️schema/🧬️mutations"
FX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
CHANGED = []
PROTOCOL = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
SET = dict(kind="change-member-tightness", rec="ChangeMemberTightness", mod="change_member_tightness", folder="🚿change-member-tightness", tag=0, verb="change", record="ChangedMemberTightness")
CLR = dict(kind="clear-member-tightness", rec="ClearMemberTightness", mod="clear_member_tightness", folder="🧽clear-member-tightness", tag=0, verb="clear", record="ClearedMemberTightness")
KINDS = (SET, CLR)


CHECK = "--check" in sys.argv


def commit(path, text):
    """Additive write: a missing file is created, an existing one only grows (every old line stays); anything else is kept and reported."""
    old = rd(path) if os.path.exists(path) else None
    if old == text:
        return
    if old is not None and not set(old.split("\n")) <= set(text.split("\n")):
        print("kept (would lose lines):", path)
        return
    CHANGED.append(path)
    if not CHECK:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)


def write(path, text):
    if not os.path.exists(path):
        commit(path, text)


def edit(path, fn):
    commit(path, fn(rd(path)))


def once(text, old, new):
    assert text.count(old) == 1, (old[:90], text.count(old))
    return text.replace(old, new)


def after_once(text, anchor, add):
    return once(text, anchor, anchor + add)


def leaf_rs(k, other):
    fields = "    pub member_id: String,\n" + ("    pub new_tightness: TightnessClass,\n" if k is SET else "")
    imports = "use semio_s_artifact_norm_en1992_model::{En1992Snapshot, TightnessClass};\n" if k is SET else "use semio_s_artifact_norm_en1992_model::En1992Snapshot;\n"
    en = "Change tightness class of member {}" if k is SET else "Clear tightness class of member {}"
    de = "Dichtigkeitsklasse von Bauteil {} ändern" if k is SET else "Dichtigkeitsklasse von Bauteil {} entfernen"
    bounds = "From<crate::%s::%s> + From<crate::%s::%s>" % (SET["mod"], SET["rec"], CLR["mod"], CLR["rec"])
    return '''//! %s `%s` - %s the tightness class (EN 1992-3 Table 7.105) of one member.

use semio_s_artifact_norm_en1992_diff::En1992Diff;
%s
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %s {
%s}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for %s where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + %s {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "%s", entity: "member-tightness", kind: "%s", record: "%s" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("%s", self.member_id), &format!("%s", self.member_id))
    }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for %s where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + %s {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { crate::%s::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::%s::inverse::inverse(self, base)?
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % ("🚿" if k is SET else "🧽", k["kind"], "sets" if k is SET else "clears", imports, k["rec"], fields, k["rec"], bounds, k["verb"], k["kind"], k["record"], en, de, k["rec"], bounds, k["mod"], k["mod"])


def diff_rs(k):
    head = '''use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersRows, En1992MembersPatch};
use crate::%s::%s;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &%s, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
''' % (k["mod"], k["rec"], k["rec"])
    if k is SET:
        return head + '''    if m.tightness == Some(payload.new_tightness) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { tightness: Some(payload.new_tightness), ..Default::default() })),
        ..Default::default()
    })
}
'''
    return head + '''    if m.tightness.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member has no tightness class.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { clear_tightness: Some(true), ..Default::default() })),
        ..Default::default()
    })
}
'''


def inverse_rs(k):
    other = CLR if k is SET else SET
    tail = '''    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
'''
    if k is SET:
        body = tail + '''    vec![match m.tightness {
        Some(previous) => Op::from(ChangeMemberTightness { member_id: payload.member_id.clone(), new_tightness: previous }),
        None => Op::from(ClearMemberTightness { member_id: payload.member_id.clone() }),
    }]
'''
    else:
        body = tail + '''    match m.tightness {
        Some(previous) => vec![Op::from(ChangeMemberTightness { member_id: payload.member_id.clone(), new_tightness: previous })],
        None => Vec::new(),
    }
'''
    return '''use crate::%s::%s;
use crate::%s::%s;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &%s, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_tightness::ChangeMemberTightness> + From<crate::clear_member_tightness::ClearMemberTightness> {
    Ok((|| {
%s    })())
}
''' % (k["mod"], k["rec"], other["mod"], other["rec"], k["rec"], body)


def kind_json(k):
    return dumps({
        "schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + k["folder"], "semanticKind": k["kind"],
        "displayName": " ".join(w.capitalize() for w in k["kind"].split("-")), "emoji": k["folder"][0], "aggregateVariant": k["rec"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": k["tag"],
        "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]})


def schema_json(k):
    template = json.loads(rd(M + "/🔥️change-member-fire-rating/🧬️schema/🔣️.json"))
    props = {"memberId": template["properties"]["memberId"]}
    required = ["memberId"]
    if k is SET:
        props["newTightness"] = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1992/snapshot.json#/$defs/TightnessClass", "x-semio-ui": {"role": "value", "label": {"en": "Tightness class", "de": "Dichtigkeitsklasse"}, "description": {"en": "Tc0, Tc1 or Tc2 (EN 1992-3 Table 7.105).", "de": "Tc0, Tc1 oder Tc2 (EN 1992-3 Tabelle 7.105)."}, "group": "tightness", "order": 20}, "x-semio-rust": {"type": "TightnessClass"}}
        required.append("newTightness")
    return dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1992/mutation/%s/schema.json" % k["kind"], "title": k["rec"], "type": "object", "additionalProperties": False, "required": required, "properties": props})


def schema_ts(k):
    head = '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
''' % (k["folder"][0], k["kind"], k["rec"])
    if k is SET:
        return head + '''import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type TightnessClass, parseTightnessClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberTightness {
  memberId: string;
  newTightness: TightnessClass;
}

export const parseChangeMemberTightness: NormWireReader<ChangeMemberTightness> = normWireObject<ChangeMemberTightness>({ memberId: normWireRequired(normWireString), newTightness: normWireRequired(parseTightnessClass) });
'''
    return head + '''import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ClearMemberTightness {
  memberId: string;
}

export const parseClearMemberTightness: NormWireReader<ClearMemberTightness> = normWireObject<ClearMemberTightness>({ memberId: normWireRequired(normWireString) });
'''


def leaf(k):
    d = M + "/" + k["folder"]
    write(d + "/🦀️.rs", leaf_rs(k, None))
    write(d + "/🔺️diff/🦀️.rs", diff_rs(k))
    write(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
    write(d + "/🔣️.json", kind_json(k))
    write(d + "/🧬️schema/🔣️.json", schema_json(k))
    write(d + "/🧬️schema/🟦️.ts", schema_ts(k))
    write(d + "/🧬️schema/🛰️.proto", 'syntax = "proto3";\nmessage %s {\n  string memberId = 1;\n%s}\n' % (k["rec"], "  string newTightness = 2;\n" if k is SET else ""))
    write(d + "/🧬️schema/🔗️.graphql", "type %s {\n  memberId: String!\n%s}\n" % (k["rec"], "  newTightness: JSON!\n" if k is SET else ""))


def mounts(text):
    if "change_member_tightness" in text:
        return text
    add = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (k["folder"], k["mod"]) for k in KINDS)
    return once(text, "//#endregion 🪢️Mounts", add + "//#endregion 🪢️Mounts")


def root(text):
    if "ChangeMemberTightness" in text:
        return text
    text = after_once(text, "use semio_s_artifact_norm_en1992_mut_member_fields::change_member_relative_humidity;\n", "use semio_s_artifact_norm_en1992_mut_member_fields::{change_member_tightness, clear_member_tightness};\n")
    text = re.sub(r"(    ChangeCaseEntryId\(change_case_entry_id::ChangeCaseEntryId\),\n)", lambda m: m.group(1) + "".join("    %s(%s::%s),\n" % (k["rec"], k["mod"], k["rec"]) for k in KINDS), text, count=1)
    assert "ClearMemberTightness(" in text
    return after_once(text, '    "change-case-entry-id",\n', "".join('    "%s",\n' % k["kind"] for k in KINDS))


def root_ts(text):
    if "ChangeMemberTightness" in text:
        return text
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k["rec"], k["rec"], k["folder"]) for k in KINDS)
    text = after_once(text, 'import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";\n', imports)
    text = once(text, "  | { ChangeCaseEntryId: ChangeCaseEntryId };\n", "  | { ChangeCaseEntryId: ChangeCaseEntryId }\n" + "".join("  | { %s: %s }" % (k["rec"], k["rec"]) + (";\n" if k is CLR else "\n") for k in KINDS))
    return once(text, "  ChangeCaseEntryId: parseChangeCaseEntryId,\n", "  ChangeCaseEntryId: parseChangeCaseEntryId,\n" + "".join("  %s: parse%s,\n" % (k["rec"], k["rec"]) for k in KINDS))


def root_json(text):
    if "ChangeMemberTightness" in text:
        return text
    tail = "\n  ]\n}"
    assert text.rstrip("\n").endswith("    }" + tail)
    block = ',\n    {\n      "type": "object",\n      "additionalProperties": false,\n      "required": [\n        "%s"\n      ],\n      "properties": {\n        "%s": {\n          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1992/mutation/%s/schema.json"\n        }\n      }\n    }'
    return text.rstrip("\n")[: -len(tail)] + "".join(block % (k["rec"], k["rec"], k["kind"]) for k in KINDS) + tail + "\n"


def edit_rules(text):
    if "change-member-tightness" in text:
        return text
    return after_once(text, '        SetFieldRule { path: "members[].relativeHumidity", kind: "change-member-relative-humidity", selectors: &[SelectorKey::Id("memberId")], value: "newValue" },\n', '        SetFieldRule { path: "members[].tightness", kind: "change-member-tightness", selectors: &[SelectorKey::Id("memberId")], value: "newTightness" },\n')


def protocol(text):
    if "change-member-tightness" in text:
        return text
    return text.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k["kind"], k["tag"]) for k in KINDS)


def text_codec(text):
    if "ChangeMemberTightness" in text:
        return text
    use = "use semio_s_artifact_norm_en1992_mut_member_fields::change_member_relative_humidity;\n"
    assert text.count(use) == 2
    text = text.replace(use, use + "use semio_s_artifact_norm_en1992_mut_member_fields::{change_member_tightness, clear_member_tightness};\n")
    printer = '        En1992Mutation::ChangeMemberRelativeHumidity(p) => format!("change-member-relative-humidity member-id={} new-value={}", enc_json(&p.member_id), p.new_value),\n'
    text = after_once(text, printer, '        En1992Mutation::ChangeMemberTightness(p) => format!("change-member-tightness member-id={} new-tightness={}", enc_json(&p.member_id), enc_json(&p.new_tightness)),\n        En1992Mutation::ClearMemberTightness(p) => format!("clear-member-tightness member-id={}", enc_json(&p.member_id)),\n')
    m = re.search(r'        "change-member-relative-humidity" => Ok\(.*\n', text)
    assert m
    parse = '        "change-member-tightness" => Ok(En1992Mutation::ChangeMemberTightness(change_member_tightness::ChangeMemberTightness { member_id: dec_json(&get("member-id")?)?, new_tightness: dec_json(&get("new-tightness")?)? })),\n        "clear-member-tightness" => Ok(En1992Mutation::ClearMemberTightness(clear_member_tightness::ClearMemberTightness { member_id: dec_json(&get("member-id")?)? })),\n'
    return text.replace(m.group(0), m.group(0) + parse)


def oracle(text):
    if "change-member-tightness" in text:
        return text
    pats = [re.compile(r'        \{\n          "mutationId": "change-member-fire-rating",.*?\n        \},\n', re.S), re.compile(r'        \{\n          "id": "change-member-fire-rating",.*?\n        \},\n', re.S)]
    for pat in pats:
        m = pat.search(text)
        assert m, pat
        blocks = "".join(m.group(0).replace("change-member-fire-rating", k["kind"]).replace("ChangeMemberFireRating", k["rec"]).replace("🔥️", k["folder"][0]) for k in KINDS)
        text = text[: m.end()] + blocks + text[m.end():]
    return once(text, '        "change-case-entry-id"\n', '        "change-case-entry-id",\n        "change-member-tightness",\n        "clear-member-tightness"\n')


def harness_py(text):
    if "change-member-tightness" in text:
        return text
    text = after_once(text, '    "change-case-entry-id",\n', "".join('    "%s",\n' % k["kind"] for k in KINDS))
    return after_once(text, '    "change-case-entry-id": ("📇️change-case-entry-id", "✅apply"),\n', "".join('    "%s": ("%s", "✅apply"),\n' % (k["kind"], k["folder"]) for k in KINDS))


def harness_rs(text):
    if "change-member-tightness" in text:
        return text
    text = after_once(text, '    "change-case-entry-id",\n', "".join('    "%s",\n' % k["kind"] for k in KINDS))
    m = re.search(r'        "change-member-fire-rating" => \(\n.*?\n        \),\n', text, re.S)
    assert m
    blocks = "".join(m.group(0).replace("change-member-fire-rating", k["kind"]).replace("🔥️", k["folder"][0]) for k in KINDS)
    return text[: m.end()] + blocks + text[m.end():]


def feature(text):
    if "change-member-tightness" in text:
        return text
    row = "      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |\n"
    assert text.count(row) == 2
    return text.replace(row, row + "".join("      | %s | %s | ✅apply  |\n" % (k["kind"], k["folder"]) for k in KINDS))


def taxonomy():
    t = json.loads(rd(TAXONOMY))
    name = "🏛️en1992"
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    new = [{"aggregateVariant": k["rec"], "artifact": name, "entity": "member-tightness", "kind": k["kind"], "module": k["folder"], "physicalLayout": "direct", "record": k["record"], "source": "%s/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % (name, k["folder"]), "standard": "🔖️1", "subset": "✳️any", "type": k["rec"], "verb": k["verb"]} for k in KINDS if k["kind"] not in have]
    t["rows"][last + 1: last + 1] = [r for r in new if r["kind"] not in have]
    out = dumps(t)
    commit(TAXONOMY, out)


def patch_null(patch):
    return {key: None for key in patch}


def vectors():
    src = FX + "/💧change-member-relative-humidity/✅apply"
    before = json.loads(rd(src + "/🔣️.before.json"))
    diff = json.loads(rd(src + "/🔣️.diff.json"))
    for k in KINDS:
        dst = FX + "/" + k["folder"] + "/✅apply"
        if os.path.exists(dst + "/🔣️.mutation.json"):
            continue
        after = copy.deepcopy(before)
        member = next(m for m in after["members"] if m["id"] == "wall-W1")
        mutation = {"memberId": "wall-W1"}
        patch = {key: None for key in diff["members"]["modified"][0]["patch"]}
        if k is SET:
            member["tightness"] = "Tc1"
            mutation["newTightness"] = "Tc1"
            patch["tightness"] = "Tc1"
        else:
            member["tightness"] = None
            patch["clearTightness"] = True
        out = copy.deepcopy(diff)
        out["members"]["modified"][0]["patch"] = patch
        write(dst + "/🔣️.before.json", dumps(before))
        write(dst + "/🔣️.after.json", dumps(after))
        write(dst + "/🔣️.mutation.json", dumps({k["rec"]: mutation}))
        write(dst + "/🔣️.outcome.json", dumps({"status": "applied"}))
        write(dst + "/🔣️.diff.json", dumps(out))


def allocate_tags():
    text = rd(PROTOCOL) if os.path.exists(PROTOCOL) else ""
    have = {m[0]: int(m[1]) for m in re.findall(r"^record (\S+) tag=(\d+)", text, re.M)}
    top = max(have.values(), default=-1)
    for k in KINDS:
        if k["kind"] in have:
            k["tag"] = have[k["kind"]]
        else:
            top += 1
            k["tag"] = top


def main():
    allocate_tags()
    for k in KINDS:
        leaf(k)
    edit(M + "/📐️member-fields/🦀️.rs", mounts)
    edit(M + "/🦀️.rs", root)
    edit(M + "/🟦️.ts", root_ts)
    edit(M + "/🔣️.json", root_json)
    edit(M + "/🧭️edit-rules/🦀️.rs", edit_rules)
    edit(PROTOCOL, protocol)
    edit(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs", text_codec)
    edit(ANY + "/🔮️oracles/🔣️.json", oracle)
    edit(ANY + "/🧪️tests/🏛️mutate-en1992-1/🐍️.py", harness_py)
    edit(ANY + "/🧪️tests/🏛️mutate-en1992-1/🦀️.rs", harness_rs)
    edit(ANY + "/🧪️tests/🏛️mutate-en1992-1/🥒️.feature", feature)
    taxonomy()
    vectors()
    print(len(set(CHANGED)), "files", "(would change)" if CHECK else "")
    sys.exit(1 if CHECK and CHANGED else 0)


if __name__ == "__main__":
    main()
