"""Realistic EN 1993 base document for mutation vectors: the high-strength-connection example plus the showcase rows of the seven newer collections, parsed from `special_parts_showcase()`."""
import json, re
from common import *


def parse_literal(v, ty, prop=None, defs=None):
    v = v.strip()
    if re.fullmatch(r"\w+::\w+", v):
        variants = defs[prop["$ref"].split("/")[-1]]["enum"]
        want = v.split("::")[1].lower()
        return next(x for x in variants if x.lower() == want)
    if v.endswith(".into()"):
        return v[: -len(".into()")].strip('"')
    if v == "String::new()":
        return ""
    if v in ("true", "false"):
        return v == "true"
    if v.startswith("vec!"):
        return []
    n = v.replace("_", "")
    if ty == "integer":
        return int(float(n))
    return float(n)


def base_doc(art, defs):
    ex = art.any + "/🖼️assets/🔩️high-strength-connection/snapshot.json"
    doc = json.loads(rd(ex))
    rs = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    fn = ""
    for name in ("special_parts_showcase", "frame_portal_showcase"):
        part = rs[rs.index("pub fn " + name):]
        fn += part[: part.index("\n    }\n")] + "\n"
    for m in re.finditer(r"doc\.(\w+)\.push\((\w+) \{ (.*) \}\);", fn):
        coll, name, body = m.groups()
        if coll not in ("storeys", "restraints", "built_up_members", "weld_groups", "parametric_sections", "hollow_joints", "joint_rows", "frame_nodes", "frame_bars"):
            continue
        props = defs[name]["properties"]
        row = {}
        for f in re.finditer(r"(\w+): (.*?)(?=, \w+: |$)", body):
            key = camel(f.group(1))
            if key in props:
                row[key] = parse_literal(f.group(2), props[key].get("type"), props[key], defs)
        doc[camel(coll)].append(row)
    return doc
