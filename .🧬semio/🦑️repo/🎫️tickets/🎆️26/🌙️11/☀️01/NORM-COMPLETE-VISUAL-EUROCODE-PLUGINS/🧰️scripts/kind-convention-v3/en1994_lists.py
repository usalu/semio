"""📋 EN 1994 diff lanes on `list_delta!`: the positional hand-written lanes `beams`, `columns`, `slabs`, `placements` (and their nested `actions` lanes) become keyed `list_delta!` lanes with local key markers
(`list: Vec<Row>, key: String = by En1994BeamKey`, the E0117-safe form), row patches by a local `nested_patch!`/`scalar_patch!` (the en1992 pattern; `within` writes a leaf of a nested struct that every row owns).

Stage 1 `rust`: reads the old row modules for the patch fields and their targets, writes the four new modules, the macros and key markers in the diff root, and switches apply/absorb/inverse to the `list_delta` API.
Stage 2 `kinds`: rewrites the `diff` leaf of every kind that builds a `modified: vec![Patch { index, .. }]` or `removed: vec![index]` lane (key = the payload id the leaf already resolved to the position).
Stage 3 `twins`: json, ts, graphql, proto of the diff. Stage 4 `fixtures`: the committed diff vectors (positions become ids). `python -I en1994_lists.py [rust|kinds|twins|fixtures|all]`, idempotent.

📎 [en1992 macros it mirrors](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs)
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ANY = NORM + "/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
DIFF = ANY + "/🧬️schema/🔺️diff"
MUT = ANY + "/🧬️schema/🧬️mutations"
MODEL = "semio_s_artifact_norm_en1994_model::"
LANES = [
    dict(coll="beams", P="Beams", module="🏗️beam-rows", key="En1994BeamKey", rowty="CompositeBeam", actty="CharacteristicAction", ak="En1994BeamsActionKey", single="beam"),
    dict(coll="columns", P="Columns", module="🏛️column-rows", key="En1994ColumnKey", rowty="CompositeColumn", actty="ColumnAction", ak="En1994ColumnsActionKey", single="column"),
    dict(coll="slabs", P="Slabs", module="🧱️slab-rows", key="En1994SlabKey", rowty="CompositeSlab", actty="CharacteristicAction", ak="En1994SlabsActionKey", single="slab"),
    dict(coll="placements", P="Placements", module="📍️placement-rows", key="En1994PlacementKey", rowty="Placement", actty=None, ak=None, single="placement"),
]
CHANGED = []


def write(path, text):
    if not os.path.exists(path) or rd(path) != text:
        open(path, "w", encoding="utf-8").write(text)
        CHANGED.append(path)


def once(text, old, new):
    assert text.count(old) == 1, (old[:100], text.count(old))
    return text.replace(old, new)


def struct_fields(text, name):
    m = re.search(r"pub struct %s \{(.*?)\n\}" % name, text, re.S)
    out = []
    for field, ty in re.findall(r"pub (\w+): ([^,\n]+),", m.group(1)):
        if field == "index":
            continue
        out.append((field, re.sub(r"^Option<(.*)>$", r"\1", ty)))
    return out


def targets(text, patch_name):
    m = re.search(r"impl %s \{\n\s*pub\(super\) fn apply_to_row.*?\n    \}\n" % patch_name, text, re.S)
    return dict(re.findall(r"if let Some\(value\) = &self\.(\w+) \{\n\s+row\.([\w\.]+) = value\.clone\(\);", m.group(0)))


def lane_facts(lane):
    path = DIFF + "/" + lane["module"] + "/🦀️.rs"
    text = rd(path)
    if "scalar_patch!" in text or "nested_patch!" in text:
        return None
    facts = dict(lane)
    patch = "En1994%sPatch" % lane["P"]
    fields, mapping = struct_fields(text, patch), targets(text, patch)
    facts["set"] = [(f, t) for f, t in fields if f != "actions" and "." not in mapping.get(f, f)]
    facts["within"] = [(f, mapping[f], t) for f, t in fields if f != "actions" and "." in mapping.get(f, f)]
    facts["has_actions"] = any(f == "actions" for f, _ in fields)
    if lane["actty"]:
        facts["action_fields"] = struct_fields(text, "En1994%sActionsPatch" % lane["P"])
    return facts


MACROS = '''
/// 🔑️ Key extractors of the foreign model rows (`id`), local markers for `list_delta!`.
macro_rules! row_key {
    ($($key:ident for $row:ident;)+) => {$(
        #[doc = concat!("🔑️ Key extractor of the foreign `", stringify!($row), "` rows (`id`).")]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $key;
        impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1994_model::$row> for $key {
            type Key = String;
            fn key_of(row: &semio_s_artifact_norm_en1994_model::$row) -> String { row.id.clone() }
        }
    )+};
}

row_key! {
    En1994BeamKey for CompositeBeam;
    En1994ColumnKey for CompositeColumn;
    En1994SlabKey for CompositeSlab;
    En1994PlacementKey for Placement;
    En1994BeamsActionKey for CharacteristicAction;
    En1994ColumnsActionKey for ColumnAction;
    En1994SlabsActionKey for CharacteristicAction;
}

/// 🩹️ Sparse scalar patch of one keyed row: the struct, its commit, coalescing, inverse and emptiness.
macro_rules! scalar_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { $($f:ident: $t:ty),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>),+ }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })+
                Ok(())
            }
            fn absorb(&mut self, later: Self) { $(if later.$f.is_some() { self.$f = later.$f; })+ }
            fn inverse(&self, row: &$row) -> Self { Self { $($f: self.$f.as_ref().map(|_| row.$f.clone())),+ } }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())+ }
        }
    };
}

/// 🩹️ Sparse patch of one keyed row with direct leaves (`set`), leaves of nested structs every row owns (`within`) and owned row lanes (`rows`): the struct, its commit, coalescing, inverse and emptiness.
macro_rules! nested_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { set { $($f:ident: $t:ty),* $(,)? } within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } rows { $($lane:ident : $rows:ty = $label:literal),* $(,)? } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>,)* $(pub $w: Option<$wt>,)* $(pub $lane: Option<$rows>,)* }

        impl protocol::list_delta::RowPatch<$row> for $name {
            #[allow(unused_variables)]
            fn commit_into(&self, row: &mut $row, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })*
                $(if let Some(value) = &self.$w { row.$obj.$inner = value.clone(); })*
                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto(&row.$lane, capability).map_err(|error| error.under([$label]))?; })*
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$f.is_some() { self.$f = later.$f; })*
                $(if later.$w.is_some() { self.$w = later.$w; })*
                $(if let Some(theirs) = later.$lane {
                    match self.$lane.as_mut() {
                        Some(mine) => mine.absorb(theirs),
                        None => self.$lane = Some(theirs),
                    }
                    self.$lane = self.$lane.take().filter(|rows| !rows.is_empty());
                })*
            }
            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($f: self.$f.as_ref().map(|_| row.$f.clone()),)*
                    $($w: self.$w.as_ref().map(|_| row.$obj.$inner.clone()),)*
                    $($lane: self.$lane.as_ref().map(|rows| rows.inverse(&row.$lane)).filter(|rows| !rows.is_empty()),)*
                }
            }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())* $(&& self.$w.is_none())* $(&& self.$lane.as_ref().map_or(true, |rows| rows.is_empty()))* }
        }
    };
}
'''


def delta(P, row, key, patch, doc):
    return '''protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ %s
    pub En1994%sRows { removal: En1994%sRemoved, insertion: En1994%sInserted, relocation: En1994%sMoved, modification: En1994%sModified, row: %s%s, patch: En1994%sPatch, list: Vec<%s%s>, key: String = by %s }
}
''' % (doc, P, P, P, P, P, MODEL, row, P, MODEL, row, key)


def module_rs(f):
    P = f["P"]
    out = "//! %s EN 1994 diff lane `%s`: the keyed %s rows%s. See [`super`].\nuse super::*;\n\n" % ({"Beams": "🏗️", "Columns": "🏛️", "Slabs": "🧱️", "Placements": "📍️"}[P], f["coll"], f["single"], " and their `actions` lane" if f["actty"] else "")
    if f["actty"]:
        fields = ", ".join("%s: %s" % (n, t) for n, t in f["action_fields"])
        out += "scalar_patch! {\n    /// 🩹️ Sparse patch of the `actions` row addressed by its key.\n    En1994%sActionsPatch for %s%s { %s }\n}\n\n" % (P, MODEL, f["actty"], fields)
        sub = dict(f, P=P + "Actions")
        out += delta(P + "Actions", f["actty"], f["ak"], None, "Row delta of the `actions` list of a %s (rows keyed by `id`)." % f["single"]).replace("En1994%sActionsActions" % P, "En1994%sActions" % P) + "\n"
    if f["actty"]:
        sets = ", ".join("%s: %s" % (n, t) for n, t in f["set"])
        within = ", ".join("%s => %s : %s" % (n, path.replace(".", " . "), t) for n, path, t in f["within"])
        out += "nested_patch! {\n    /// 🩹️ Sparse patch of the `%s` row addressed by its key.\n    En1994%sPatch for %s%s {\n        set { %s }\n        within { %s }\n        rows { actions: En1994%sActionsRows = \"actions\" }\n    }\n}\n\n" % (f["coll"], P, MODEL, f["rowty"], sets, within, P)
    else:
        sets = ", ".join("%s: %s" % (n, t) for n, t in f["set"])
        out += "scalar_patch! {\n    /// 🩹️ Sparse patch of the `placements` row addressed by its key.\n    En1994PlacementsPatch for %sPlacement { %s }\n}\n\n" % (MODEL, sets)
    out += delta(P, f["rowty"], f["key"], None, "Row delta of the `%s` list (rows keyed by `id`)." % f["coll"])
    return out


def rust():
    facts = [x for x in (lane_facts(l) for l in LANES) if x]
    if not facts:
        print("rust: already converted")
        return
    for f in facts:
        text = module_rs(f)
        text = re.sub(r"pub En1994(\w+)ActionsRows \{ removal: En1994\w+Removed, insertion: En1994\w+Inserted, relocation: En1994\w+Moved, modification: En1994\w+Modified, row: ([\w:]+), patch: En1994\w+Patch, list:", lambda m: m.group(0), text)
        write(DIFF + "/" + f["module"] + "/🦀️.rs", text)
    root = DIFF + "/🦀️.rs"
    text = rd(root)
    text = once(text, "#[path = \"🏗️beam-rows/🦀️.rs\"]", MACROS.lstrip("\n") + "\n#[path = \"🏗️beam-rows/🦀️.rs\"]")
    write(root, text)
    inv = DIFF + "/🔁️apply-and-inverse/🦀️.rs"
    t = rd(inv)
    t = t.replace(".absorb_rows(theirs)", ".absorb(theirs)").replace("fn apply(&self, unpublished: &mut En1994Snapshot, _capability: protocol::ApplyCapability)", "fn apply(&self, unpublished: &mut En1994Snapshot, capability: protocol::ApplyCapability)")
    t = re.sub(r"rows\.apply_rows\(&base\.(\w+)\)", r"rows.commit_onto(&base.\1, capability)", t)
    t = re.sub(r"rows\.inverse_rows\(&base\.(\w+)\)", r"rows.inverse(&base.\1)", t)
    write(inv, t)


def brace_end(text, start):
    depth = 0
    for i in range(start, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    raise ValueError("unbalanced")


def key_map(text):
    """var -> key expression, from the lookups the leaf already does."""
    keys = {}
    for m in re.finditer(r"let Some\((\w+)\) = [\w\.\[\]]+\.iter\(\)\.position\(\|\w+\| \w+\.id == (payload\.\w+)\)", text):
        keys[m.group(1)] = m.group(2)
    for m in re.finditer(r"let Some\(\((\w+), \w+\)\) = [\w\.]+\.iter\(\)\.enumerate\(\)\.find\(\|\(_, \w+\)\| \w+\.id == (payload\.\w+)\)", text):
        keys[m.group(1)] = m.group(2)
    return keys


MOD = re.compile(r"(En1994\w+Rows) \{(\s*)modified: vec!\[(En1994\w+Patch) \{(\s*)index(?:: (\w+))?,")


def rewrite_modified(text, keys):
    while True:
        m = MOD.search(text)
        if not m:
            return text
        var = m.group(5) or "index"
        key = keys.get(var)
        if key is None:
            raise KeyError(var)
        body_start = m.end() - 0
        patch_open = text.index("{", m.start(3))
        patch_close = brace_end(text, patch_open)
        rest = text[m.end():patch_close]
        tail = re.match(r"\s*\],\s*\.\.Default::default\(\)\s*\}", text[patch_close + 1:])
        assert tail, text[patch_close:patch_close + 80]
        replacement = "%s::modification(&%s, %s {%s%s})" % (m.group(1), key, m.group(3), m.group(4) if rest.strip() else " ", rest.lstrip())
        text = text[: m.start()] + replacement + text[patch_close + 1 + tail.end():]


def kinds():
    changed, manual = 0, []
    for path in sorted(glob.glob(MUT + "/*/🔺️diff/🦀️.rs")):
        text = rd(path)
        if "modified: vec![" not in text and "removed: vec![index]" not in text:
            continue
        out = text
        try:
            out = rewrite_modified(out, key_map(out))
        except (KeyError, AssertionError, ValueError) as error:
            manual.append((path.split("/")[-3], repr(error)))
            continue
        out = re.sub(r"(En1994(\w+)Rows) \{\s*removed: vec!\[index\],\s*\.\.Default::default\(\)\s*\}", lambda m: "%s::removal(&base.%s, index)" % (m.group(1), m.group(2).lower()), out)
        if out != text:
            write(path, out)
            changed += 1
    print("kinds", changed, "manual", manual)


def main():
    stage = sys.argv[1] if len(sys.argv) > 1 else "all"
    if stage in ("rust", "all"):
        rust()
    if stage in ("kinds", "all"):
        kinds()
    print(len(set(CHANGED)), "files")


if __name__ == "__main__":
    main()
