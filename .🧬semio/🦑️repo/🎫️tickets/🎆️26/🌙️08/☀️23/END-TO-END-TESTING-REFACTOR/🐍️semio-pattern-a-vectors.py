#!/usr/bin/env python3
"""🧫️ Independent Python implementation of the `.dsl.semio` grammar and of the
`SemioCadMutation` / `SemioDocumentMutation` / `SemioFlowMutation` specification. Run once to derive
the committed `(before, mutation, after)` specification vectors for the three Pattern-A semio cases
of ticket 26/08/23/END-TO-END-TESTING-REFACTOR.

It reads the REAL committed example artifacts under
`✳️any/📚️examples/{📐️drawing,📄️memo,🌊️pipeline}/🖼️assets/🗣️example.dsl.semio`, decodes them from the
committed grammar, applies each declared mutation kind by hand, and writes the vectors. None of this
repository's own Rust is consulted or executed — that is the point: the vectors are a SECOND,
independently written implementation of the same specification, in a different language. Each case's
`identity-round-trip` scenario then asserts that the production `parse_dsl` of the same real artifact
equals the `before` snapshot written here, which is what keeps this decoder honest.
"""

import copy, json, os, struct

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
EXAMPLES = f"{ROOT}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples"
TESTS = f"{ROOT}/🧪️tests"

# ---------------------------------------------------------------- grammar primitives

def strip_brackets(s):
    assert s.startswith("[") and s.endswith("]"), f"not bracketed: {s!r}"
    return s[1:-1]

def split_top_level(s, sep=","):
    out, depth, start = [], 0, 0
    for i, ch in enumerate(s):
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        elif ch == sep and depth == 0:
            out.append(s[start:i])
            start = i + 1
    out.append(s[start:])
    return out

def dec_str(h):
    return bytes.fromhex(h).decode("utf-8")

def dec_bytes(h):
    return list(bytes.fromhex(h))

def dec_bool(s):
    assert s in ("0", "1"), s
    return s == "1"

def dec_point2(s):
    x, y = split_top_level(strip_brackets(s))
    return {"x": float(x), "y": float(y)}

def dec_list(s, dec):
    return [dec(p) for p in split_top_level(strip_brackets(s)) if p != ""]

def dec_option(s, dec):
    parts = split_top_level(strip_brackets(s))
    if parts == ["0"]:
        return None
    assert parts[0] == "1", parts
    return dec(parts[1])

def dec_f64_bits(s):
    return struct.unpack("<d", struct.pack("<Q", int(s)))[0]

def body_lines(text):
    lines = [l.strip() for l in text.strip().splitlines()]
    return [l for l in lines[1:] if l]

def field(lines, name):
    for line in lines:
        if line.startswith(name + "="):
            return line[len(name) + 1:]
    raise KeyError(name)

# ---------------------------------------------------------------- flow decoder

def dec_flow_param(s):
    k, v = split_top_level(strip_brackets(s))
    return {"key": dec_str(k), "value": dec_str(v)}

def dec_port_ref(s):
    n, p = split_top_level(strip_brackets(s))
    return {"node": dec_str(n), "port": dec_str(p)}

def dec_flow_node(s):
    i, k, l, params, pos = split_top_level(strip_brackets(s))
    return {"id": dec_str(i), "kind": dec_str(k), "label": dec_str(l), "params": dec_list(params, dec_flow_param), "position": dec_point2(pos)}

def dec_flow_edge(s):
    i, f, t, k = split_top_level(strip_brackets(s))
    return {"id": dec_str(i), "from": dec_port_ref(f), "to": dec_port_ref(t), "kind": dec_str(k)}

def decode_flow(text):
    lines = body_lines(text)
    return {"schema": dec_str(field(lines, "schema")), "nodes": dec_list(field(lines, "nodes"), dec_flow_node), "edges": dec_list(field(lines, "edges"), dec_flow_edge)}

# ---------------------------------------------------------------- cad decoder

def dec_cad_entity(s):
    tag, parts = s[0], split_top_level(strip_brackets(s[1:]))
    if tag == "L":
        a, b = parts
        return {"kind": "line", "a": dec_point2(a), "b": dec_point2(b)}
    if tag == "A":
        c, r, sa, ea = parts
        return {"kind": "arc", "center": dec_point2(c), "radius": float(r), "start_angle": float(sa), "end_angle": float(ea)}
    if tag == "C":
        c, r = parts
        return {"kind": "circle", "center": dec_point2(c), "radius": float(r)}
    if tag == "E":
        c, m, ratio, sp, ep = parts
        return {"kind": "ellipse", "center": dec_point2(c), "major_axis_end": dec_point2(m), "ratio": float(ratio), "start_param": float(sp), "end_param": float(ep)}
    if tag == "P":
        v, closed = parts
        return {"kind": "polyline", "vertices": dec_list(v, dec_point2), "closed": dec_bool(closed)}
    if tag == "T":
        p, h, r, c = parts
        return {"kind": "text", "position": dec_point2(p), "height": float(h), "rotation": float(r), "content": dec_str(c)}
    if tag == "I":
        b, ip, sc, r = parts
        return {"kind": "insert", "block_name": dec_str(b), "insertion_point": dec_point2(ip), "scale": dec_point2(sc), "rotation": float(r)}
    if tag == "S":
        p1, p2, p3, p4 = parts
        return {"kind": "solid", "p1": dec_point2(p1), "p2": dec_point2(p2), "p3": dec_point2(p3), "p4": dec_point2(p4)}
    if tag == "D":
        d, t, m, txt = parts
        return {"kind": "dimension", "def_point": dec_point2(d), "text_position": dec_point2(t), "measurement": float(m), "text": dec_str(txt)}
    raise ValueError(f"cad entity tag {tag!r}")

def dec_cad_layer(s):
    n, c, lt, v = split_top_level(strip_brackets(s))
    return {"name": dec_str(n), "colorIndex": int(c), "lineType": dec_str(lt), "visible": dec_bool(v)}

def dec_cad_record(s):
    h, l, e = split_top_level(strip_brackets(s))
    return {"handle": dec_str(h), "layer": dec_str(l), "entity": dec_cad_entity(e)}

def dec_cad_block(s):
    n, bp, ents = split_top_level(strip_brackets(s))
    return {"name": dec_str(n), "basePoint": dec_point2(bp), "entities": dec_list(ents, dec_cad_record)}

def decode_cad(text):
    lines = body_lines(text)
    return {
        "schema": dec_str(field(lines, "schema")),
        "layers": dec_list(field(lines, "layers"), dec_cad_layer),
        "blocks": dec_list(field(lines, "blocks"), dec_cad_block),
        "entities": dec_list(field(lines, "entities"), dec_cad_record),
    }

# ---------------------------------------------------------------- document decoder

def dec_run_style(s):
    bold, italic, underline, size, font, color, link = split_top_level(strip_brackets(s))
    return {
        "bold": dec_bool(bold),
        "italic": dec_bool(italic),
        "underline": dec_bool(underline),
        "size": dec_option(size, dec_f64_bits),
        "font": dec_option(font, dec_str),
        "color": dec_option(color, dec_str),
        "link": dec_option(link, dec_str),
    }

def dec_run(s):
    text, style = split_top_level(strip_brackets(s))
    return {"text": dec_str(text), "style": dec_run_style(style)}

def dec_list_item(s):
    return {"blocks": dec_list(strip_brackets(s), dec_doc_block)}

def dec_cell(s):
    return {"blocks": dec_list(strip_brackets(s), dec_doc_block)}

def dec_row(s):
    return {"cells": dec_list(strip_brackets(s), dec_cell)}

def dec_doc_block(s):
    tag, inner = s[0], strip_brackets(s[1:])
    if tag == "P":
        style_id, runs = split_top_level(inner)
        return {"kind": "paragraph", "style_id": dec_option(style_id, dec_str), "runs": dec_list(runs, dec_run)}
    if tag == "H":
        level, style_id, runs = split_top_level(inner)
        return {"kind": "heading", "level": int(level), "style_id": dec_option(style_id, dec_str), "runs": dec_list(runs, dec_run)}
    if tag == "L":
        ordered, items = split_top_level(inner)
        return {"kind": "list", "ordered": dec_bool(ordered), "items": dec_list(items, dec_list_item)}
    if tag == "T":
        return {"kind": "table", "rows": dec_list(inner, dec_row)}
    if tag == "C":
        language, text = split_top_level(inner)
        return {"kind": "code", "language": dec_option(language, dec_str), "text": dec_str(text)}
    if tag == "Q":
        return {"kind": "quote", "blocks": dec_list(inner, dec_doc_block)}
    if tag == "I":
        image_id, alt, width, height = split_top_level(inner)
        return {"kind": "image", "image_id": dec_str(image_id), "alt": dec_str(alt), "width": dec_option(width, dec_f64_bits), "height": dec_option(height, dec_f64_bits)}
    if tag == "B":
        return {"kind": "pageBreak"}
    raise ValueError(f"doc block tag {tag!r}")

def dec_doc_style(s):
    i, n, b = split_top_level(strip_brackets(s))
    return {"id": dec_str(i), "name": dec_str(n), "basedOn": dec_option(b, dec_str)}

def dec_doc_image(s):
    i, m, b = split_top_level(strip_brackets(s))
    return {"id": dec_str(i), "mime": dec_str(m), "bytes": dec_bytes(b)}

def decode_document(text):
    lines = body_lines(text)
    return {
        "schema": dec_str(field(lines, "schema")),
        "styles": dec_list(field(lines, "styles"), dec_doc_style),
        "images": dec_list(field(lines, "images"), dec_doc_image),
        "blocks": dec_list(field(lines, "blocks"), dec_doc_block),
    }

# ---------------------------------------------------------------- emission

def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()

def emit(case, vectors):
    d = f"{TESTS}/{case}/🧫️fixtures"
    os.makedirs(d, exist_ok=True)
    for kind, mutation, before, after in vectors:
        with open(f"{d}/🦠️{kind}.json", "w", encoding="utf-8") as f:
            json.dump({"kind": kind, "mutation": mutation, "before": before, "after": after}, f, ensure_ascii=False, indent=2)
            f.write("\n")
    print(f"{case}: {len(vectors)} vectors")

def node_by(snapshot, key, ident, coll):
    return next(x for x in snapshot[coll] if x[key] == ident)

def plain_style():
    return {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}

def run(text, **style):
    st = plain_style()
    st.update(style)
    return {"text": text, "style": st}

# ---------------------------------------------------------------- flow vectors

def flow_vectors(base):
    v = []

    def after(fn):
        a = copy.deepcopy(base)
        fn(a)
        return a

    v.append(("no-mutation", {"mutation": "noMutation"}, base, copy.deepcopy(base)))

    snap = after(lambda a: a["nodes"][1].__setitem__("label", "Terminal"))
    v.append(("set-snapshot", {"mutation": "setSnapshot", "snapshot": copy.deepcopy(snap)}, base, snap))

    n3 = {"id": "n3", "kind": "transform", "label": "Normalize", "params": [{"key": "mode", "value": "strict"}], "position": {"x": 60.0, "y": 10.0}}
    v.append(("insert-node", {"mutation": "insertNode", "node": copy.deepcopy(n3)}, base, after(lambda a: a["nodes"].append(copy.deepcopy(n3)))))

    v.append(("remove-node", {"mutation": "removeNode", "id": "n2"}, base, after(lambda a: a.__setitem__("nodes", [n for n in a["nodes"] if n["id"] != "n2"]))))
    v.append(("set-node-kind", {"mutation": "setNodeKind", "id": "n1", "kind": "generator"}, base, after(lambda a: a["nodes"][0].__setitem__("kind", "generator"))))
    v.append(("set-node-label", {"mutation": "setNodeLabel", "id": "n1", "label": "Quelle"}, base, after(lambda a: a["nodes"][0].__setitem__("label", "Quelle"))))
    v.append(("set-node-position", {"mutation": "setNodePosition", "id": "n2", "position": {"x": 240.0, "y": 15.5}}, base, after(lambda a: a["nodes"][1].__setitem__("position", {"x": 240.0, "y": 15.5}))))
    v.append(("set-node-param", {"mutation": "setNodeParam", "id": "n1", "key": "count", "value": "7"}, base, after(lambda a: a["nodes"][0]["params"][0].__setitem__("value", "7"))))
    v.append(("remove-node-param", {"mutation": "removeNodeParam", "id": "n1", "key": "unit"}, base, after(lambda a: a["nodes"][0].__setitem__("params", [p for p in a["nodes"][0]["params"] if p["key"] != "unit"]))))

    e2 = {"id": "e2", "from": {"node": "n2", "port": "out"}, "to": {"node": "n1", "port": "in"}, "kind": "feedback"}
    v.append(("insert-edge", {"mutation": "insertEdge", "edge": copy.deepcopy(e2)}, base, after(lambda a: a["edges"].append(copy.deepcopy(e2)))))
    v.append(("remove-edge", {"mutation": "removeEdge", "id": "e1"}, base, after(lambda a: a.__setitem__("edges", []))))

    def swap_endpoints(a):
        a["edges"][0]["from"] = {"node": "n2", "port": "out"}
        a["edges"][0]["to"] = {"node": "n1", "port": "in"}

    v.append(("set-edge-endpoints", {"mutation": "setEdgeEndpoints", "id": "e1", "from": {"node": "n2", "port": "out"}, "to": {"node": "n1", "port": "in"}}, base, after(swap_endpoints)))
    v.append(("set-edge-kind", {"mutation": "setEdgeKind", "id": "e1", "kind": "control"}, base, after(lambda a: a["edges"][0].__setitem__("kind", "control"))))
    return v

# ---------------------------------------------------------------- cad vectors

def cad_vectors(base):
    v = []

    def after(fn):
        a = copy.deepcopy(base)
        fn(a)
        return a

    v.append(("no-mutation", {"mutation": "noMutation"}, base, copy.deepcopy(base)))

    snap = after(lambda a: a["layers"][1].__setitem__("colorIndex", 3))
    v.append(("set-snapshot", {"mutation": "setSnapshot", "snapshot": copy.deepcopy(snap)}, base, snap))

    hidden = {"name": "hidden", "colorIndex": 8, "lineType": "HIDDEN", "visible": False}
    v.append(("add-layer", {"mutation": "addLayer", "layer": copy.deepcopy(hidden)}, base, after(lambda a: a["layers"].append(copy.deepcopy(hidden)))))
    v.append(("remove-layer", {"mutation": "removeLayer", "name": "dim"}, base, after(lambda a: a.__setitem__("layers", [l for l in a["layers"] if l["name"] != "dim"]))))
    v.append(("set-layer", {"mutation": "setLayer", "name": "0", "color_index": 5}, base, after(lambda a: a["layers"][0].__setitem__("colorIndex", 5))))

    window = {"name": "window", "basePoint": {"x": 0.0, "y": 0.0}, "entities": [{"handle": "we1", "layer": "0", "entity": {"kind": "line", "a": {"x": 0.0, "y": 0.0}, "b": {"x": 0.0, "y": 1.0}}}]}
    v.append(("add-block", {"mutation": "addBlock", "block": copy.deepcopy(window)}, base, after(lambda a: a["blocks"].append(copy.deepcopy(window)))))
    v.append(("remove-block", {"mutation": "removeBlock", "name": "door"}, base, after(lambda a: a.__setitem__("blocks", []))))
    v.append(("set-block-base-point", {"mutation": "setBlockBasePoint", "name": "door", "base_point": {"x": 2.5, "y": -1.0}}, base, after(lambda a: a["blocks"][0].__setitem__("basePoint", {"x": 2.5, "y": -1.0}))))

    h9 = {"handle": "h9", "layer": "0", "entity": {"kind": "circle", "center": {"x": 9.0, "y": 9.0}, "radius": 0.5}}
    v.append(("add-entity", {"mutation": "addEntity", "entity": copy.deepcopy(h9)}, base, after(lambda a: a["entities"].append(copy.deepcopy(h9)))))
    # 🧭️ `h8` is the LAST entity on purpose. `RemoveEntity`'s inverse is `AddEntity`, and `entities`
    # is a NAME-keyed collection whose `apply_named` pushes additions at the end — so removing a
    # non-final entity and undoing it restores the value but not the position, and the inverse law
    # would legitimately fail. Flipping this target to `h2` is the one-line change that exposes it
    # once the Rust subject phase compiles again; see the ticket report's findings.
    v.append(("remove-entity", {"mutation": "removeEntity", "handle": "h8"}, base, after(lambda a: a.__setitem__("entities", [e for e in a["entities"] if e["handle"] != "h8"]))))
    v.append(("set-entity-layer", {"mutation": "setEntityLayer", "handle": "h1", "layer": "dim"}, base, after(lambda a: a["entities"][0].__setitem__("layer", "dim"))))

    wide = {"kind": "circle", "center": {"x": 5.0, "y": 5.0}, "radius": 4.0}
    v.append(("set-entity-geometry", {"mutation": "setEntityGeometry", "handle": "h2", "entity": copy.deepcopy(wide)}, base, after(lambda a: a["entities"][1].__setitem__("entity", copy.deepcopy(wide)))))

    be2 = {"handle": "be2", "layer": "0", "entity": {"kind": "line", "a": {"x": 1.0, "y": 0.0}, "b": {"x": 1.0, "y": 1.0}}}
    v.append(("add-block-entity", {"mutation": "addBlockEntity", "block_name": "door", "entity": copy.deepcopy(be2)}, base, after(lambda a: a["blocks"][0]["entities"].append(copy.deepcopy(be2)))))
    v.append(("remove-block-entity", {"mutation": "removeBlockEntity", "block_name": "door", "handle": "be1"}, base, after(lambda a: a["blocks"][0].__setitem__("entities", []))))
    v.append(("set-block-entity-layer", {"mutation": "setBlockEntityLayer", "block_name": "door", "handle": "be1", "layer": "dim"}, base, after(lambda a: a["blocks"][0]["entities"][0].__setitem__("layer", "dim"))))

    leaf = {"kind": "arc", "center": {"x": 0.0, "y": 0.0}, "radius": 1.0, "start_angle": 0.0, "end_angle": 90.0}
    v.append(("set-block-entity-geometry", {"mutation": "setBlockEntityGeometry", "block_name": "door", "handle": "be1", "entity": copy.deepcopy(leaf)}, base, after(lambda a: a["blocks"][0]["entities"][0].__setitem__("entity", copy.deepcopy(leaf)))))
    return v

# ---------------------------------------------------------------- document vectors

def document_vectors(base):
    v = []

    def after(fn):
        a = copy.deepcopy(base)
        fn(a)
        return a

    v.append(("no-mutation", {"mutation": "noMutation"}, base, copy.deepcopy(base)))

    snap = after(lambda a: a["styles"][0].__setitem__("name", "Überschrift 1"))
    v.append(("set-snapshot", {"mutation": "setSnapshot", "snapshot": copy.deepcopy(snap)}, base, snap))

    item_two = {"kind": "paragraph", "style_id": None, "runs": [run("item two")]}
    nested_path = {"segments": [{"kind": "listItem", "block_index": 2, "item": 0}], "index": 1}
    v.append(("insert-block", {"mutation": "insertBlock", "path": copy.deepcopy(nested_path), "block": copy.deepcopy(item_two)}, base, after(lambda a: a["blocks"][2]["items"][0]["blocks"].insert(1, copy.deepcopy(item_two)))))

    v.append(("remove-block", {"mutation": "removeBlock", "path": {"segments": [], "index": 7}}, base, after(lambda a: a["blocks"].pop(7))))

    replaced = {"kind": "paragraph", "style_id": None, "runs": [run("Replaced body")]}
    v.append(("set-block-content", {"mutation": "setBlockContent", "path": {"segments": [], "index": 1}, "block": copy.deepcopy(replaced)}, base, after(lambda a: a["blocks"].__setitem__(1, copy.deepcopy(replaced)))))

    v.append(("set-paragraph-style", {"mutation": "setParagraphStyle", "path": {"segments": [], "index": 1}, "style_id": "heading1"}, base, after(lambda a: a["blocks"][1].__setitem__("style_id", "heading1"))))
    v.append(("set-heading-level", {"mutation": "setHeadingLevel", "path": {"segments": [], "index": 0}, "level": 3}, base, after(lambda a: a["blocks"][0].__setitem__("level", 3))))
    v.append(("set-list-ordered", {"mutation": "setListOrdered", "path": {"segments": [], "index": 2}, "ordered": False}, base, after(lambda a: a["blocks"][2].__setitem__("ordered", False))))

    quote_path = {"segments": [{"kind": "quote", "block_index": 5}], "index": 0}
    v.append(("set-run-text", {"mutation": "setRunText", "path": copy.deepcopy(quote_path), "run_index": 0, "text": "zitiert"}, base, after(lambda a: a["blocks"][5]["blocks"][0]["runs"][0].__setitem__("text", "zitiert"))))

    styled = {"bold": False, "italic": True, "underline": False, "size": 11.0, "font": "Inter", "color": "#202020", "link": None}
    v.append(("set-run-style", {"mutation": "setRunStyle", "path": {"segments": [], "index": 1}, "run_index": 0, "style": copy.deepcopy(styled)}, base, after(lambda a: a["blocks"][1]["runs"][0].__setitem__("style", copy.deepcopy(styled)))))

    def retarget_image(a):
        a["blocks"][6].update({"image_id": "img1", "alt": "Floor plan", "width": 320.0, "height": 240.0})

    v.append(("set-image-block", {"mutation": "setImageBlock", "path": {"segments": [], "index": 6}, "image_id": "img1", "alt": "Floor plan", "width": 320.0, "height": 240.0}, base, after(retarget_image)))

    caption = {"id": "caption", "name": "Caption", "basedOn": "normal"}
    v.append(("insert-style", {"mutation": "insertStyle", "style": copy.deepcopy(caption)}, base, after(lambda a: a["styles"].append(copy.deepcopy(caption)))))
    v.append(("remove-style", {"mutation": "removeStyle", "id": "heading1"}, base, after(lambda a: a.__setitem__("styles", []))))
    v.append(("set-style-name", {"mutation": "setStyleName", "id": "heading1", "name": "Title Heading"}, base, after(lambda a: a["styles"][0].__setitem__("name", "Title Heading"))))
    v.append(("set-style-based-on", {"mutation": "setStyleBasedOn", "id": "heading1", "based_on": None}, base, after(lambda a: a["styles"][0].__setitem__("basedOn", None))))

    img2 = {"id": "img2", "mime": "image/jpeg", "bytes": [255, 216, 255]}
    v.append(("insert-image", {"mutation": "insertImage", "image": copy.deepcopy(img2)}, base, after(lambda a: a["images"].append(copy.deepcopy(img2)))))
    v.append(("remove-image", {"mutation": "removeImage", "id": "img1"}, base, after(lambda a: a.__setitem__("images", []))))
    v.append(("set-image-bytes", {"mutation": "setImageBytes", "id": "img1", "mime": "image/gif", "bytes": [71, 73, 70]}, base, after(lambda a: a["images"][0].update({"mime": "image/gif", "bytes": [71, 73, 70]}))))
    return v

# ---------------------------------------------------------------- driver

if __name__ == "__main__":
    flow = decode_flow(read(f"{EXAMPLES}/🌊️pipeline/🖼️assets/🗣️example.dsl.semio"))
    cad = decode_cad(read(f"{EXAMPLES}/📐️drawing/🖼️assets/🗣️example.dsl.semio"))
    doc = decode_document(read(f"{EXAMPLES}/📄️memo/🖼️assets/🗣️example.dsl.semio"))
    emit("mutate-semio-flow", flow_vectors(flow))
    emit("mutate-semio-cad", cad_vectors(cad))
    emit("mutate-semio-document", document_vectors(doc))
