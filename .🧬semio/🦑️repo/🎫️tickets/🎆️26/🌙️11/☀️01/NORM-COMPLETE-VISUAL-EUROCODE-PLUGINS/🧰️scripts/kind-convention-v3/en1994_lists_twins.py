"""🔣 EN 1994 diff twins (json schema, ts, graphql, proto) and committed diff vectors for the keyed `list_delta!` lanes.

Every lane `L` in `{Beams, Columns, Slabs, Placements}` and `{Beams, Columns, Slabs}Actions`: the patch loses its `index`, `L Rows` becomes `{removed: [L Removed{id,index}], inserted: [L Inserted{index,row}], moved: [L Moved{id,from,to}], modified: [L Modified{id,patch}]}`.
Vectors: `removed: [index]` -> `[{id, index}]` (id from the before snapshot), `modified: [{index, ..}]` -> `[{id, patch}]`, `moved: []` added, recursively for the `actions` lanes. `python -I en1994_lists_twins.py`, idempotent.
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *
sys.path.insert(0, os.path.dirname(os.path.dirname(HERE)) + "/🧰️scripts")
from schema_json import dump_schema

ANY = NORM + "/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
DIFF = ANY + "/🧬️schema/🔺️diff"
FX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/"
LANES = ["BeamsActions", "ColumnsActions", "SlabsActions", "Beams", "Columns", "Slabs", "Placements"]
ROW = {"BeamsActions": "CharacteristicAction", "ColumnsActions": "ColumnAction", "SlabsActions": "CharacteristicAction", "Beams": "CompositeBeam", "Columns": "CompositeColumn", "Slabs": "CompositeSlab", "Placements": "Placement"}
CHANGED = []


def write(path, text):
    if rd(path) != text:
        open(path, "w", encoding="utf-8").write(text)
        CHANGED.append(path)


def twin_json():
    path = DIFF + "/🔣️.json"
    text = rd(path)
    doc = json.loads(text)
    defs = doc["$defs"]
    if "En1994BeamsMoved" in defs:
        return
    new = {}
    for key, value in defs.items():
        new[key] = value
    for lane in LANES:
        pre = "En1994" + lane
        patch = defs[pre + "Patch"]
        patch["required"] = [r for r in patch["required"] if r != "index"]
        patch["properties"] = {k: v for k, v in patch["properties"].items() if k != "index"}
        new[pre + "Removed"] = {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}}
        new[pre + "Moved"] = {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0}, "to": {"type": "integer", "minimum": 0}}}
        new[pre + "Modified"] = {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": "#/$defs/" + pre + "Patch"}}}
        new[pre + "Rows"] = {"type": "object", "additionalProperties": False, "required": ["removed", "inserted", "moved", "modified"], "properties": {
            "removed": {"type": "array", "items": {"$ref": "#/$defs/" + pre + "Removed"}}, "inserted": {"type": "array", "items": {"$ref": "#/$defs/" + pre + "Inserted"}},
            "moved": {"type": "array", "items": {"$ref": "#/$defs/" + pre + "Moved"}}, "modified": {"type": "array", "items": {"$ref": "#/$defs/" + pre + "Modified"}}}}
    doc["$defs"] = dict(sorted(new.items()))
    write(path, dump_schema(doc))


def ts_block(pre, with_removed):
    removed = "export interface %(pre)sRemoved {\n  id: string;\n  index: number;\n}\n\n" % dict(pre=pre) if with_removed else ""
    return removed + """export interface %(pre)sMoved {
  id: string;
  from: number;
  to: number;
}

export interface %(pre)sModified {
  id: string;
  patch: %(pre)sPatch;
}
""" % dict(pre=pre)


def twin_ts():
    path = DIFF + "/🟦️.ts"
    text = rd(path)
    if "En1994BeamsMoved" in text:
        return
    for lane in LANES:
        pre = "En1994" + lane
        text = text.replace("export interface %sPatch {\n  index: number;\n" % pre, "export interface %sPatch {\n" % pre)
        text = re.sub(r"(export const parse%sPatch: NormWireReader<%sPatch> = normWireObject<%sPatch>\(\{ )index: normWireRequired\(normWireRange\(normWireInteger, \{\"minimum\":0\}\)\), " % (pre, pre, pre), r"\1", text)
        rows_if = re.search(r"export interface %sRows \{\n(?:  \w+: [^\n]*\n)+\}\n" % pre, text)
        assert rows_if, pre
        with_removed = ("export interface %sRemoved {" % pre) not in text
        text = text.replace(rows_if.group(0), """export interface %(pre)sRows {
  removed: %(pre)sRemoved[];
  inserted: %(pre)sInserted[];
  moved: %(pre)sMoved[];
  modified: %(pre)sModified[];
}

""" % dict(pre=pre) + ts_block(pre, with_removed))
        rows_parse = re.search(r"export const parse%sRows: NormWireReader<%sRows> = [^\n]*\n" % (pre, pre), text)
        assert rows_parse, pre
        new_parse = '''export const parse%(pre)sRows: NormWireReader<%(pre)sRows> = normWireObject<%(pre)sRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parse%(pre)sRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parse%(pre)sInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parse%(pre)sMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parse%(pre)sModified))) });
export const parse%(pre)sMoved: NormWireReader<%(pre)sMoved> = normWireObject<%(pre)sMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parse%(pre)sModified: NormWireReader<%(pre)sModified> = normWireObject<%(pre)sModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse%(pre)sPatch)) });
''' % dict(pre=pre)
        text = text.replace(rows_parse.group(0), new_parse)
        if not re.search(r"export const parse%sRemoved\b" % pre, text):
            text = text.replace(new_parse, new_parse + 'export const parse%(pre)sRemoved: NormWireReader<%(pre)sRemoved> = normWireObject<%(pre)sRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });\n' % dict(pre=pre))
    write(path, text)


def twin_graphql():
    path = DIFF + "/🔗️.graphql"
    text = rd(path)
    if "type En1994BeamsMoved" in text:
        return
    for lane in LANES:
        pre = "En1994" + lane
        text = re.sub(r"(type %sPatch \{\n)  index: Int!\n" % pre, r"\1", text)
        m = re.search(r"type %sRows \{\n(?:  [^\n]*\n)+\}\n" % pre, text)
        assert m, pre
        extra = "type %(pre)sMoved {\n  id: String!\n  from: Int!\n  to: Int!\n}\n\ntype %(pre)sModified {\n  id: String!\n  patch: %(pre)sPatch!\n}\n\n" % dict(pre=pre)
        if "type %sRemoved" % pre not in text:
            extra = "type %(pre)sRemoved {\n  id: String!\n  index: Int!\n}\n\n" % dict(pre=pre) + extra
        rows = "type %(pre)sRows {\n  removed: [%(pre)sRemoved!]!\n  inserted: [%(pre)sInserted!]!\n  moved: [%(pre)sMoved!]!\n  modified: [%(pre)sModified!]!\n}\n" % dict(pre=pre)
        text = text.replace(m.group(0), extra + rows)
    write(path, text)


def twin_proto():
    path = DIFF + "/🛰️.proto"
    text = rd(path)
    if "message En1994BeamsMoved" in text:
        return
    for lane in LANES:
        pre = "En1994" + lane
        text = re.sub(r"(message %sPatch \{\n)  uint64 index = 1;\n" % pre, r"\1", text)
        m = re.search(r"message %sRows \{\n(?:  [^\n]*\n)+\}\n" % pre, text)
        assert m, pre
        extra = "message %(pre)sMoved {\n  string id = 1;\n  uint64 from = 2;\n  uint64 to = 3;\n}\n\nmessage %(pre)sModified {\n  string id = 1;\n  %(pre)sPatch patch = 2;\n}\n\n" % dict(pre=pre)
        if "message %sRemoved" % pre not in text:
            extra = "message %(pre)sRemoved {\n  string id = 1;\n  uint64 index = 2;\n}\n\n" % dict(pre=pre) + extra
        rows = "message %(pre)sRows {\n  repeated %(pre)sRemoved removed = 1;\n  repeated %(pre)sInserted inserted = 2;\n  repeated %(pre)sMoved moved = 3;\n  repeated %(pre)sModified modified = 4;\n}\n" % dict(pre=pre)
        text = text.replace(m.group(0), extra + rows)
    write(path, text)


def lane_rows(node, base_rows, convert_rows):
    out = {"removed": [], "inserted": node.get("inserted", []), "moved": [], "modified": []}
    for entry in node.get("removed", []):
        if isinstance(entry, int):
            out["removed"].append({"id": base_rows[entry]["id"], "index": entry})
        else:
            out["removed"].append(entry)
    for patch in node.get("modified", []):
        if "patch" in patch:
            out["modified"].append(patch)
            continue
        patch = dict(patch)
        index = patch.pop("index")
        parent = base_rows[index]
        if isinstance(patch.get("actions"), dict):
            patch["actions"] = lane_rows(patch["actions"], parent["actions"], convert_rows)
        out["modified"].append({"id": parent["id"], "patch": patch})
    return out


def fixtures():
    for path in sorted(glob.glob(FX + "/*/*/🔣️.diff.json")):
        diff = json.loads(rd(path))
        if not any(isinstance(diff.get(c), dict) for c in ("beams", "columns", "slabs", "placements")):
            continue
        before = json.loads(rd(os.path.dirname(os.path.dirname(path)) + "/🔣️.before.json"))
        changed = False
        for coll in ("beams", "columns", "slabs", "placements"):
            node = diff.get(coll)
            if not isinstance(node, dict) or "moved" in node:
                continue
            diff[coll] = lane_rows(node, before[coll], None)
            changed = True
        if changed:
            write(path, dumps(diff))


def main():
    twin_json()
    twin_ts()
    twin_graphql()
    twin_proto()
    fixtures()
    print(len(set(CHANGED)), "files")


if __name__ == "__main__":
    main()
