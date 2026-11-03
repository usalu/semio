"""EN 1994 nested `actions` lanes: grows `En1994{Beams,Columns,Slabs}ActionsRows` from `modified` to `removed` (`{id, index}`), `inserted` (`{index, row}`) and `modified`, with a local `KeyOf` marker per row type. Rust lane, four schema twins, null-padded fixtures. Idempotent. `python3 -I lanes.py [--apply]`."""
import glob, json, os, re, sys

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
DIFF = ANY + "/🧬️schema/🔺️diff"
FIX = ANY + "/🧪️tests/🧫️fixtures"
APPLY = "--apply" in sys.argv
LANES = [("Beams", "beam", "🏗️beam-rows", "CharacteristicAction"), ("Columns", "column", "🏛️column-rows", "ColumnAction"), ("Slabs", "slab", "🧱️slab-rows", "CharacteristicAction")]
DERIVE = '''#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
'''
RS = '''/// 🔑️ Local key marker of the {single} `actions` rows (`id`): the removal and the inverse name rows by it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1994{coll}ActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1994_model::{row}> for En1994{coll}ActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1994_model::{row}) -> String {
        row.id.clone()
    }
}

/// ➖️ An `actions` row removed from base position `index`, named by its `id`.
{derive}pub struct En1994{coll}ActionsRemoved {
    pub id: String,
    pub index: usize,
}
/// 📌️ An `actions` row inserted at final position `index`.
{derive}pub struct En1994{coll}ActionsInserted {
    pub index: usize,
    pub row: semio_s_artifact_norm_en1994_model::{row},
}
/// 🔺️ Positional diff of `actions` rows: removed base positions with their ids, inserted rows at final positions and modified base-position patches, each strictly ascending.
{derive}pub struct En1994{coll}ActionsRows {
    pub removed: Vec<En1994{coll}ActionsRemoved>,
    pub inserted: Vec<En1994{coll}ActionsInserted>,
    pub modified: Vec<En1994{coll}ActionsPatch>,
}
impl En1994{coll}ActionsRows {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.modified.is_empty()
    }
    pub(super) fn strictly_ascending(indices: impl Iterator<Item = usize>) -> bool {
        let mut previous: Option<usize> = None;
        for index in indices {
            if previous.is_some_and(|before| before >= index) {
                return false;
            }
            previous = Some(index);
        }
        true
    }
    pub(super) fn apply_rows(&self, base: &[semio_s_artifact_norm_en1994_model::{row}]) -> Result<Vec<semio_s_artifact_norm_en1994_model::{row}>, protocol::MutationApplyError> {
        let out_of_range = |index: usize| protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("row position {index} is out of range")).at([index.to_string()]);
        if !(Self::strictly_ascending(self.removed.iter().map(|removed| removed.index)) && Self::strictly_ascending(self.inserted.iter().map(|inserted| inserted.index)) && Self::strictly_ascending(self.modified.iter().map(|patch| patch.index))) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index-order", "row positions must be strictly ascending"));
        }
        if let Some(removed) = self.removed.iter().find(|removed| removed.index >= base.len()) {
            return Err(out_of_range(removed.index));
        }
        if let Some(removed) = self.removed.iter().find(|removed| <En1994{coll}ActionKey as protocol::list_delta::KeyOf<semio_s_artifact_norm_en1994_model::{row}>>::key_of(&base[removed.index]) != removed.id) {
            return Err(missing_target(format!("action {} at row {}", removed.id, removed.index)).at([removed.index.to_string()]));
        }
        if let Some(patch) = self.modified.iter().find(|patch| patch.index >= base.len()) {
            return Err(out_of_range(patch.index));
        }
        if let Some(patch) = self.modified.iter().find(|patch| self.removed.binary_search_by_key(&patch.index, |removed| removed.index).is_ok()) {
            return Err(missing_target(format!("modified row {} is removed by the same diff", patch.index)).at([patch.index.to_string()]));
        }
        let mut rows = Vec::with_capacity(base.len());
        for (index, row) in base.iter().enumerate() {
            if self.removed.binary_search_by_key(&index, |removed| removed.index).is_ok() {
                continue;
            }
            let mut row = row.clone();
            if let Ok(at) = self.modified.binary_search_by_key(&index, |patch| patch.index) {
                self.modified[at].apply_to_row(&mut row)?;
            }
            rows.push(row);
        }
        for inserted in &self.inserted {
            if inserted.index > rows.len() {
                return Err(out_of_range(inserted.index));
            }
            rows.insert(inserted.index, inserted.row.clone());
        }
        Ok(rows)
    }
    pub(super) fn inverse_rows(&self, base: &[semio_s_artifact_norm_en1994_model::{row}]) -> Self {
        let final_index = |index: usize| {
            let mut position = index - self.removed.iter().filter(|removed| removed.index < index).count();
            for inserted in &self.inserted {
                if inserted.index <= position {
                    position += 1;
                }
            }
            position
        };
        Self {
            removed: self.inserted.iter().map(|inserted| En1994{coll}ActionsRemoved { id: <En1994{coll}ActionKey as protocol::list_delta::KeyOf<semio_s_artifact_norm_en1994_model::{row}>>::key_of(&inserted.row), index: inserted.index }).collect(),
            inserted: self.removed.iter().filter_map(|removed| base.get(removed.index).map(|row| En1994{coll}ActionsInserted { index: removed.index, row: row.clone() })).collect(),
            modified: self.modified.iter().filter_map(|patch| base.get(patch.index).map(|row| patch.inverse_from_row(row, final_index(patch.index)))).collect(),
        }
    }
    pub(super) fn absorb_rows(&mut self, other: Self) {
        enum Origin {
            New(usize),
            Base(usize),
        }
        let origin = |position: usize| -> Origin {
            if let Some(at) = self.inserted.iter().position(|inserted| inserted.index == position) {
                return Origin::New(at);
            }
            let mut base_index = position - self.inserted.iter().filter(|inserted| inserted.index < position).count();
            for removed in &self.removed {
                if removed.index <= base_index {
                    base_index += 1;
                }
            }
            Origin::Base(base_index)
        };
        let removed_origins: Vec<(Origin, String)> = other.removed.iter().map(|removed| (origin(removed.index), removed.id.clone())).collect();
        let patch_origins: Vec<Origin> = other.modified.iter().map(|patch| origin(patch.index)).collect();
        let mut dropped = vec![false; self.inserted.len()];
        let mut removed_base: Vec<En1994{coll}ActionsRemoved> = Vec::new();
        for (found, id) in removed_origins {
            match found {
                Origin::New(at) => dropped[at] = true,
                Origin::Base(base_index) => removed_base.push(En1994{coll}ActionsRemoved { id, index: base_index }),
            }
        }
        for (found, mut patch) in patch_origins.into_iter().zip(other.modified) {
            match found {
                Origin::New(at) => {
                    let _ = patch.apply_to_row(&mut self.inserted[at].row);
                }
                Origin::Base(base_index) => {
                    patch.index = base_index;
                    match self.modified.iter_mut().find(|existing| existing.index == base_index) {
                        Some(existing) => existing.merge(patch),
                        None => self.modified.push(patch),
                    }
                }
            }
        }
        let mut survivors: Vec<En1994{coll}ActionsInserted> = Vec::new();
        for (at, inserted) in std::mem::take(&mut self.inserted).into_iter().enumerate() {
            if dropped[at] {
                continue;
            }
            let mut position = inserted.index - other.removed.iter().filter(|removed| removed.index < inserted.index).count();
            for placed in &other.inserted {
                if placed.index <= position {
                    position += 1;
                }
            }
            survivors.push(En1994{coll}ActionsInserted { index: position, row: inserted.row });
        }
        survivors.extend(other.inserted);
        survivors.sort_by_key(|inserted| inserted.index);
        self.inserted = survivors;
        self.removed.extend(removed_base);
        self.removed.sort_by_key(|removed| removed.index);
        self.removed.dedup_by_key(|removed| removed.index);
        let removed = &self.removed;
        self.modified.retain(|patch| removed.binary_search_by_key(&patch.index, |entry| entry.index).is_err());
        self.modified.sort_by_key(|patch| patch.index);
    }
}
'''


def render(coll, single, row):
    return RS.replace("{derive}", DERIVE).replace("{coll}", coll).replace("{single}", single).replace("{row}", row).replace("{\n", "{\n")


def rust():
    out = {}
    for coll, single, d, row in LANES:
        p = DIFF + "/" + d + "/🦀️.rs"
        s = open(p, encoding="utf-8").read()
        a = s.index("/// 🔺️ Positional diff of `actions` rows") if "ActionKey" not in s else s.index("/// 🔑️ Local key marker of the %s `actions`" % single)
        b = s.index("/// 🩹️ Sparse patch of the `%s` row" % coll.lower())
        out[p] = s[:a] + render(coll, single, row) + s[b:]
    return out


JSON_TWIN = lambda coll, row: {
    "En1994%sActionsRemoved" % coll: {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}},
    "En1994%sActionsInserted" % coll: {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0}, "row": {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/" + row}}},
    "En1994%sActionsRows" % coll: {"type": "object", "additionalProperties": False, "required": ["removed", "inserted", "modified"], "properties": {"removed": {"type": "array", "items": {"$ref": "#/$defs/En1994%sActionsRemoved" % coll}}, "inserted": {"type": "array", "items": {"$ref": "#/$defs/En1994%sActionsInserted" % coll}}, "modified": {"type": "array", "items": {"$ref": "#/$defs/En1994%sActionsPatch" % coll}}}},
}


def twins():
    out = {}
    p = DIFF + "/🔣️.json"
    raw = open(p, encoding="utf-8").read()
    j = json.loads(raw)
    defs = j["$defs"]
    ref = defs["En1994BeamsInserted"]["properties"]["row"]
    prefix = ref["$ref"].rsplit("/", 1)[0] + "/"
    new, new_before = {}, {}
    for coll, single, d, row in LANES:
        new_before["En1994%sActionsPatch" % coll] = ["En1994%sActionsInserted" % coll]
        t = JSON_TWIN(coll, row)
        for k in t:
            t[k] = json.loads(json.dumps(t[k]).replace("https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/", prefix))
        new.update(t)
    ordered = {}
    for k, v in defs.items():
        if k.endswith("ActionsPatch") and k in new_before:
            ordered[new_before[k][0]] = new[new_before[k][0]]
        if k.endswith("ActionsRows"):
            ordered[k.replace("Rows", "Removed")] = new[k.replace("Rows", "Removed")]
        ordered[k] = new[k] if k in new else v
    j["$defs"] = ordered
    out[p] = json.dumps(j, indent=2, ensure_ascii=False) + "\n"

    p = DIFF + "/🟦️.ts"
    s = open(p, encoding="utf-8").read()
    for coll, single, d, row in LANES:
        if "En1994%sActionsRemoved" % coll in s:
            continue
        iface = "export interface En1994%sActionsRows {\n  modified: En1994%sActionsPatch[];\n}\n" % (coll, coll)
        assert iface in s, iface
        s = s.replace(iface, "export interface En1994%sActionsRemoved {\n  id: string;\n  index: number;\n}\n\nexport interface En1994%sActionsInserted {\n  index: number;\n  row: %s;\n}\n\nexport interface En1994%sActionsRows {\n  removed: En1994%sActionsRemoved[];\n  inserted: En1994%sActionsInserted[];\n  modified: En1994%sActionsPatch[];\n}\n" % (coll, coll, row, coll, coll, coll, coll))
        old = "export const parseEn1994%sActionsRows: NormWireReader<En1994%sActionsRows> = normWireObject<En1994%sActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994%sActionsPatch))) });\n" % (coll, coll, coll, coll)
        assert old in s, old
        rem = "export const parseEn1994%sActionsRemoved: NormWireReader<En1994%sActionsRemoved> = normWireObject<En1994%sActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0})) });\n" % (coll, coll, coll)
        ins = "export const parseEn1994%sActionsInserted: NormWireReader<En1994%sActionsInserted> = normWireObject<En1994%sActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {\"minimum\":0})), row: normWireRequired(parse%s) });\n" % (coll, coll, coll, row)
        new = "export const parseEn1994%sActionsRows: NormWireReader<En1994%sActionsRows> = normWireObject<En1994%sActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1994%sActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994%sActionsInserted))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994%sActionsPatch))) });\n" % ((coll,) * 6)
        s = s.replace(old, rem + ins + new)
    if "type CharacteristicAction" not in s.split("\n", 4)[3]:
        s = s.replace("import { type AnnexChoice, type CompositeBeam,", "import { type AnnexChoice, type CharacteristicAction, type ColumnAction, type CompositeBeam,", 1).replace("parseAnnexChoice, parseCompositeBeam,", "parseAnnexChoice, parseCharacteristicAction, parseColumnAction, parseCompositeBeam,", 1)
    out[p] = s

    p = DIFF + "/🔗️.graphql"
    s = open(p, encoding="utf-8").read()
    for coll, single, d, row in LANES:
        if "En1994%sActionsRemoved" % coll in s:
            continue
        old = "type En1994%sActionsRows {\n  modified: [En1994%sActionsPatch!]!\n}\n" % (coll, coll)
        assert old in s, old
        s = s.replace(old, "type En1994%sActionsRemoved {\n  id: String!\n  index: Int!\n}\n\ntype En1994%sActionsInserted {\n  index: Int!\n  row: %s!\n}\n\ntype En1994%sActionsRows {\n  removed: [En1994%sActionsRemoved!]!\n  inserted: [En1994%sActionsInserted!]!\n  modified: [En1994%sActionsPatch!]!\n}\n" % (coll, coll, row, coll, coll, coll, coll))
    out[p] = s

    p = DIFF + "/🛰️.proto"
    s = open(p, encoding="utf-8").read()
    for coll, single, d, row in LANES:
        if "En1994%sActionsRemoved" % coll in s:
            continue
        old = "message En1994%sActionsRows {\n  repeated En1994%sActionsPatch modified = 1;\n}\n" % (coll, coll)
        assert old in s, old
        s = s.replace(old, "message En1994%sActionsRemoved {\n  string id = 1;\n  uint64 index = 2;\n}\n\nmessage En1994%sActionsInserted {\n  uint64 index = 1;\n}\n\nmessage En1994%sActionsRows {\n  repeated En1994%sActionsRemoved removed = 1;\n  repeated En1994%sActionsInserted inserted = 2;\n  repeated En1994%sActionsPatch modified = 3;\n}\n" % ((coll,) * 6))
    out[p] = s
    return out


def pad(node, key=None):
    if isinstance(node, dict):
        if key == "actions" and set(node) == {"modified"}:
            return {"removed": [], "inserted": [], "modified": node["modified"]}
        return {k: pad(v, k) for k, v in node.items()}
    if isinstance(node, list):
        return [pad(v, key) for v in node]
    return node


def fixtures():
    out = {}
    for p in glob.glob(FIX + "/**/🔣️.diff.json", recursive=True):
        raw = open(p, encoding="utf-8").read()
        if '"actions"' not in raw:
            continue
        j = json.loads(raw)
        n = pad(j)
        if n != j:
            out[p] = json.dumps(n, indent=2, ensure_ascii=False) + "\n" if raw.startswith("{\n") else json.dumps(n, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
    return out


if __name__ == "__main__":
    files = {}
    for part in (rust, twins, fixtures):
        files.update(part())
    for p, text in files.items():
        old = open(p, encoding="utf-8").read()
        if old != text:
            print("change", p.replace(ANY, "…"))
            if APPLY:
                open(p, "w", encoding="utf-8").write(text)
    print("files", len(files), "applied" if APPLY else "dry")
