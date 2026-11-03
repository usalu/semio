"""🩹 EN 1992 `members[].tightness` in the diff: the local `nested_patch!` macro gains an `optional { field: Type => clear_flag }` clause (a set field plus a clear marker, so a patch can tell "clear" from "leave"), the member patch uses it, the four diff twins and every committed member patch (null padded) follow.

`python -I en1992_tightness_diff.py` is idempotent. Patch wire: `tightness: TightnessClass | null` (set) and `clearTightness: boolean | null` (`true` clears; absent/null leaves).

📎 [diff crate](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs)
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ANY = NORM + "/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"
DIFF = ANY + "/🧬️schema/🔺️diff"
CHANGED = []


def edit(path, fn):
    text = rd(path)
    out = fn(text)
    if out != text:
        open(path, "w", encoding="utf-8").write(out)
        CHANGED.append(path)


def once(text, old, new):
    assert text.count(old) == 1, (old[:80], text.count(old))
    return text.replace(old, new)


def macro(text):
    if "optional {" in text:
        return text
    text = once(text, "within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } rows {", "within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } optional { $($o:ident : $ot:ty => $oflag:ident),* $(,)? } rows {")
    text = once(text, "$(pub $w: Option<$wt>,)* $(pub $lane: Option<$rows>,)* }", "$(pub $w: Option<$wt>,)* $(pub $o: Option<$ot>, pub $oflag: Option<bool>,)* $(pub $lane: Option<$rows>,)* }")
    text = once(text, "                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto", "                $(if self.$oflag == Some(true) { row.$o = None; } else if let Some(value) = &self.$o { row.$o = Some(value.clone()); })*\n                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto")
    text = once(text, "                $(if later.$w.is_some() { self.$w = later.$w; })*\n", "                $(if later.$w.is_some() { self.$w = later.$w; })*\n                $(if later.$oflag == Some(true) { self.$oflag = Some(true); self.$o = None; } else if later.$o.is_some() { self.$o = later.$o; self.$oflag = None; })*\n")
    text = once(text, "                    $($w: self.$w.as_ref().and_then(|_| (|| Some(row.$obj.as_ref()?.$inner.clone()))()),)*\n", "                    $($w: self.$w.as_ref().and_then(|_| (|| Some(row.$obj.as_ref()?.$inner.clone()))()),)*\n                    $($o: if self.$o.is_some() || self.$oflag == Some(true) { row.$o.clone() } else { None }, $oflag: if (self.$o.is_some() || self.$oflag == Some(true)) && row.$o.is_none() { Some(true) } else { None },)*\n")
    text = once(text, "$(&& self.$w.is_none())* $(&& self.$lane.as_ref()", "$(&& self.$w.is_none())* $(&& self.$o.is_none() && self.$oflag != Some(true))* $(&& self.$lane.as_ref()")
    text = once(text, "        within {  }\n        rows { actions: En1992AnchorsActionsRows", "        within {  }\n        optional {  }\n        rows { actions: En1992AnchorsActionsRows")
    text = re.sub(r"(    En1992MembersPatch for semio_s_artifact_norm_en1992_model::RcMember \{\n.*?\n        within \{[^\n]*\}\n)(        rows \{)", lambda m: m.group(1) + "        optional { tightness: semio_s_artifact_norm_en1992_model::TightnessClass => clear_tightness }\n" + m.group(2), text, count=1, flags=re.S)
    assert "optional { tightness" in text
    return text


def twin_json(text):
    if "clearTightness" in text:
        return text
    ref = '{"anyOf":[{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1992/snapshot.json#/$defs/RcMember/properties/tightness"},{"type":"null"}]}'
    flag = '{"anyOf":[{"type":"boolean"},{"type":"null"}]}'
    text = once(text, '"relativeHumidity","longitudinal","actions"', '"relativeHumidity","tightness","clearTightness","longitudinal","actions"')
    anchor = '"longitudinal":{"anyOf":[{"$ref":"#/$defs/En1992MembersLongitudinalRows"}'
    return once(text, anchor, '"tightness":%s,"clearTightness":%s,' % (ref, flag) + anchor)


def twin_ts(text):
    if "clearTightness" in text:
        return text
    text = once(text, "  relativeHumidity: number | null;\n  longitudinal: En1992MembersLongitudinalRows | null;", "  relativeHumidity: number | null;\n  tightness: TightnessClass | null;\n  clearTightness: boolean | null;\n  longitudinal: En1992MembersLongitudinalRows | null;")
    text = once(text, "relativeHumidity: normWireRequired(normWireNullable(normWireNumber)), longitudinal:", "relativeHumidity: normWireRequired(normWireNullable(normWireNumber)), tightness: normWireRequired(normWireNullable(normWireRef(() => parseTightnessClass))), clearTightness: normWireRequired(normWireNullable(normWireBoolean)), longitudinal:")
    return text


def twin_graphql(text):
    if "clearTightness" in text:
        return text
    return once(text, "  relativeHumidity: Float\n  longitudinal: En1992MembersLongitudinalRows", "  relativeHumidity: Float\n  tightness: TightnessClass\n  clearTightness: Boolean\n  longitudinal: En1992MembersLongitudinalRows")


def twin_proto(text):
    if "clear_tightness" in text:
        return text
    return once(text, "  optional En1992MembersStudRingsRows stud_rings = 46;\n}", "  optional En1992MembersStudRingsRows stud_rings = 46;\n  optional TightnessClass tightness = 47;\n  optional bool clear_tightness = 48;\n}")


PATCH = re.compile(r'("relativeHumidity": [^\n]*,\n)(\s*)("longitudinal": (?:null|\{))')


def fixtures():
    for path in glob.glob(ANY + "/🧪️tests/**/*.json", recursive=True):
        text = rd(path)
        if '"relativeHumidity"' not in text or '"clearTightness"' in text:
            continue
        out = PATCH.sub(lambda m: m.group(1) + m.group(2) + '"tightness": null,\n' + m.group(2) + '"clearTightness": null,\n' + m.group(2) + m.group(3), text)
        if out != text:
            open(path, "w", encoding="utf-8").write(out)
            CHANGED.append(path)


def main():
    edit(DIFF + "/🦀️.rs", macro)
    edit(DIFF + "/🔣️.json", twin_json)
    edit(DIFF + "/🟦️.ts", twin_ts)
    edit(DIFF + "/🔗️.graphql", twin_graphql)
    edit(DIFF + "/🛰️.proto", twin_proto)
    fixtures()
    print(len(CHANGED), "files")


if __name__ == "__main__":
    main()
