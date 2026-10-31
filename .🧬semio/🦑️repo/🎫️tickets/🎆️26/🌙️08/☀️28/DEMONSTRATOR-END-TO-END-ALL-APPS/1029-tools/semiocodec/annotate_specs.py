#!/usr/bin/env python3
"""Authors the `annotations` of every committed record spec: the two facts the record graph cannot carry.

* `single`: the `block(statements)` fields whose Rust field is ONE enum value (`class: ElementClass`, `Option<..>`), not a `Vec` of them — read off the
  snapshot sources (`#[dsl(statements, block)] pub <key>: <Type>`), unambiguous per key across the 17 subsets;
* `tags`: the statements fields whose enum is tagged under another key than `kind` (`SlideShape` is `#[value(tag = "shapeKind")]`, because its `Placeholder` variant has a field named `kind`);
* `verbatim`: the statements fields whose enum has no `rename_all_fields` (every enum but `PathSegment`, `CadEntity`, `BrepCurve`, `BrepCurve2`, `BrepSurface`, `GeometryRef`), so the variant payload keys are the Rust names;
* `nested`: the records that flatten a JSON sub-object (`ArtifactRef.dialect`) into their own fields;
* `float32`: the float fields whose Rust type is `f32` (`SemioRgba`, material `metallic`/`roughness`, drawing `opacity`, audio `samples`), as the `float`
  entries of the protobuf mirrors.

usage: annotate_specs.py   (rewrites the `annotations` key of every `🗺️record-spec/🔣️.json`)
"""
import json
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets")
SINGLE_KEYS = {"class", "curve", "entity", "geometry", "kind", "pcurve", "property", "root", "surface", "value"}
MANY_KEYS = {"blocks", "cells", "children", "items", "notes", "segments", "shapes"}
FLOAT32_KEYS = {"metallic", "roughness", "opacity", "samples"}
RGBA = {"r", "g", "b", "a"}
CAMEL_VARIANT_FIELDS = {("🖊️drawing", "segments"), ("📐️cad", "entity"), ("🧊️brep", "curve"), ("🧊️brep", "pcurve"), ("🧊️brep", "surface"), ("🏛️model", "geometry")}
SHAPE_KEYWORDS = {"picture", "placeholder", "table", "text-box"}

for path in sorted(ROOT.glob("*/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json")):
    document = json.loads(path.read_text(encoding="utf-8"))
    single, float32, tags, verbatim, nested = [], [], [], [], []
    for record_index, record in enumerate(document["graph"]):
        keys = {field["key"] for field in record}
        if keys == {"artifact-id", "artifact-kind", "standard", "subset"}:
            nested.append([record_index, "dialect", ["artifact-kind", "standard", "subset"]])
        for field in record:
            shape = field["shape"]
            inner = shape["item"] if shape["kind"] == "block" else shape
            if inner["kind"] == "statements":
                if (path.parts[-5], field["key"]) not in CAMEL_VARIANT_FIELDS:
                    verbatim.append([record_index, field["id"]])
                if {keyword for keyword, _ in inner["variants"]} == SHAPE_KEYWORDS:
                    tags.append([record_index, field["id"], "shapeKind"])
                if field["key"] in SINGLE_KEYS:
                    single.append([record_index, field["id"]])
                elif field["key"] not in MANY_KEYS:
                    raise SystemExit("%s: statements field %r is neither single nor many" % (path, field["key"]))
            leaf = shape["item"] if shape["kind"] == "list" else shape
            if leaf["kind"] == "float" and (field["key"] in FLOAT32_KEYS or (keys == RGBA and field["key"] in RGBA)):
                float32.append([record_index, field["id"]])
    annotations = {key: value for key, value in (("single", single), ("float32", float32), ("tags", tags), ("verbatim", verbatim), ("nested", nested)) if value}
    if annotations:
        document["annotations"] = annotations
    else:
        document.pop("annotations", None)
    path.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(path.parts[-5], annotations)
