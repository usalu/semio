#!/usr/bin/env python3
"""🧱️ Derives the `mutate-block-2d-1` case fixture ONCE from the committed real node-kind document.

Provenance, in full:

* Input ``✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/…/📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio``
  — the artifact's own committed example, and a real one: the *Hexagonal Cut Concrete Forest Left*
  node kind with its real camera, its six real handle kinds and their real HSL colours, its ELEVEN
  real handles at their real radian angles around a 0.36 rim, and its one real compatibility rule.
  All of that is carried across unchanged.
* Output ``…/🧪️tests/mutate-block-2d-1/🧫️fixtures/🧱️hexagonal-cut-concrete-forest-left.snapshot.json``.

What the carrier does NOT hold, and where those values come from — stated rather than invented:

* the node kind's `variant`, `icon` and `unit`: the carrier's `node-kind` block writes only `id`,
  `name`, `label` and `description`, so all three are the empty string the schema's `type: "string"`
  admits;
* `presentation`: the carrier writes an EMPTY block, and the snapshot requires a `shape`, so the
  presentation is taken VERBATIM from the committed `✏️rename-node-kind` vector's before-snapshot
  (a circle of radius 0.5, `#2288cc`, icon kind `icon.valve`);
* `attributes` and `authors`: the carrier's tables are empty, so `remove-attribute` and
  `remove-author` would address nothing — the committed vectors' own `material`/`brass` attribute and
  `author-ada` are taken verbatim, from `🚫️remove-attribute` and `🚷️remove-author`;
* one spare handle kind `hk-ground`, taken verbatim from `🌱️create-handle-kind`, because every one of
  the six committed handle kinds is REFERENCED by a handle and `delete-handle-kind` needs an
  unreferenced target. It is appended LAST, so the committed kinds keep their indices and the spare
  is the trailing one — which is what makes the inverse of a delete exact in a vocabulary whose
  `create-` verbs carry no index.
"""

# region 🔖️Imports
import json
import os
import re
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
def tokens_of(line):
    """✂️ Splits a carrier row into tokens, keeping a `"quoted phrase"` whole."""
    return [part[1:-1] if part.startswith('"') else part for part in re.findall(r'"[^"]*"|\S+', line)]


def section_body(text, header):
    """🧱️ The body of a `name … { … }` carrier section, matched by brace depth."""
    at = text.index(header)
    start = text.index("{", at) + 1
    depth, cursor = 1, start
    while depth:
        if text[cursor] == "{":
            depth += 1
        elif text[cursor] == "}":
            depth -= 1
        cursor += 1
    return text[start : cursor - 1]


def fields_of(text):
    """🔧️ The `key="value"` pairs of a braced carrier block."""
    return {key: value[1:-1] if value.startswith('"') else value for key, value in re.findall(r'(\S+?)=("[^"]*"|\S+)', text)}


def angle(token):
    """📐️ A carrier angle — written in radians with a `rad` suffix."""
    return float(token[:-3]) if token.endswith("rad") else float(token)


# endregion 🔖️Carrier


# region 🔖️Derivation
def committed(root, kind_dir, tail):
    """🧫️ One committed specification-vector file of this subset, read whole."""
    directory = os.path.join(root, "🧬️schema/🧬️mutations", kind_dir, "🧪️tests")
    fixture = sorted(os.listdir(directory))[0]
    return json.load(open(os.path.join(directory, fixture, tail, "🔣️component.json"), encoding="utf-8"))


def derive(source, subset_root, target):
    """🧬️ Writes the derived document: the committed node kind, plus the members the carrier omits."""
    text = open(source, encoding="utf-8").read()
    node_kind = fields_of(section_body(text, "node-kind {"))
    camera = fields_of(section_body(text, "camera2d {"))
    handle_kinds = [{"id": r[0], "name": r[1], "label": r[2], "color": r[3], "defaultWireKind": r[4]} for r in map(tokens_of, section_body(text, "handle-kinds [").strip().split("\n"))]
    handles = [{"id": r[0], "handleKind": r[1], "angle": angle(r[2]), "radius": float(r[3])} for r in map(tokens_of, section_body(text, "handles [").strip().split("\n"))]
    compatibility = [{"id": r[0], "source": r[1], "target": r[2], "bidirectional": r[3] == "true"} for r in map(tokens_of, section_body(text, "compatibility [").strip().split("\n"))]
    if len(handles) != 11 or len(handle_kinds) != 6:
        raise SystemExit("the committed example is expected to carry six handle kinds and eleven handles")

    document = {
        "schema": text.split("schema=", 1)[1].split("\n", 1)[0],
        "nodeKind": {"id": node_kind["id"], "name": node_kind["name"], "label": node_kind["label"], "variant": "", "description": node_kind["description"], "icon": "", "unit": ""},
        "presentation": committed(subset_root, "✏️rename-node-kind", "📸️snapshot/⬅️before")["presentation"],
        "handleKinds": handle_kinds + [committed(subset_root, "🌱️create-handle-kind", "🦠️mutation")["handleKind"]],
        "handles": handles,
        "compatibility": compatibility,
        "attributes": [entry for entry in committed(subset_root, "🚫️remove-attribute", "📸️snapshot/⬅️before")["attributes"]],
        "authors": [entry for entry in committed(subset_root, "🚷️remove-author", "📸️snapshot/⬅️before")["authors"]],
        "camera2d": {"x": float(camera["x"]), "y": float(camera["y"]), "zoom": float(camera["zoom"])},
        "meta": {"description": fields_of(section_body(text, "meta {")).get("description", "")},
    }
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d handle kind(s), %d handle(s), %d rule(s), %d attribute(s), %d author(s))" % (target, len(document["handleKinds"]), len(document["handles"]), len(document["compatibility"]), len(document["attributes"]), len(document["authors"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2], sys.argv[3])
