#!/usr/bin/env python3
"""♻️ Derives the `mutate-rewrite-1` case fixture ONCE from the committed real rule document.

Provenance, in full — every value in the output is read out of one committed file, none is invented:

* Input ``✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  — the artifact's own committed demo document: a graph-rewrite RULE over the Nakagin Capsule Tower
  ground floor. Its `before-fixture-json` is the real two-node, one-edge tower fragment with the real
  UUIDs, port ids and transform properties; its `lhs-json` is the real left-hand pattern with its
  `whereClause`; its `rhs-json` is the real right-hand side with its `set` list and its declared
  `label` parameter; its `parameter-bindings` and `rule-layout` are the real committed ones.
* Output ``…/🧪️tests/mutate-rewrite-1/🧫️fixtures/♻️nakagin-ground-floor.snapshot.json``.

Why the reading is done here rather than in the case's Python reference: the `.rewrite.dsl.semio`
carrier has no prose document and mixes three different value encodings in one file — a
backslash-escaped quoted string for `before-fixture-json` and `lhs-json`, a braced block for
`parameter-bindings` and `rule-layout`, and a fenced ```json block for `rhs-json`. Which encoding a
value gets is not stated anywhere, so a reference that guessed the rule and then claimed byte-exact
reproduction would be asserting a specification that does not exist. The reading below is a
one-off, and the fields it produces are what check it.
"""

# region 🔖️Imports
import json
import os
import re
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
def quoted(text, key):
    """🔤️ A backslash-escaped quoted carrier value: `key="…"`, ending at the first unescaped quote."""
    at = text.index('%s="' % key) + len(key) + 2
    out, escaped = [], False
    for character in text[at:]:
        if escaped:
            out.append({"n": "\n", "t": "\t", "r": "\r", '"': '"', "\\": "\\"}.get(character, character))
            escaped = False
        elif character == "\\":
            escaped = True
        elif character == '"':
            return "".join(out)
        else:
            out.append(character)
    raise SystemExit("unterminated quoted value for %r" % key)


def fenced(text, key):
    """📦️ A fenced carrier value: ``key=```json … ``` ``."""
    body = text.split("%s=```json\n" % key, 1)[1]
    return body.split("\n```", 1)[0]


def block(text, key):
    """🧱️ A braced carrier block: `key={ … }`, returned as its inner lines."""
    body = text.split("%s={\n" % key, 1)[1]
    return [line.strip() for line in body.split("\n}", 1)[0].split("\n") if line.strip()]


def bindings(text):
    """🔧️ `parameter-bindings` — `key="value"` pairs, one per line."""
    out = {}
    for line in block(text, "parameter-bindings"):
        key, _, value = line.partition("=")
        out[key.strip()] = value.strip().strip('"')
    return out


def layout(text):
    """📐️ `rule-layout` — `key=x=N y=N` pairs, several to a line."""
    out = {}
    for line in block(text, "rule-layout"):
        for key, x, y in re.findall(r"(\S+?)=x=(-?[\d.]+) y=(-?[\d.]+)", line):
            out[key] = {"x": float(x), "y": float(y)}
    return out


# endregion 🔖️Carrier


# region 🔖️Derivation
def derive(source, target):
    """🧬️ Writes the derived snapshot: the five members `RewriteSnapshot` declares, read as committed."""
    text = open(source, encoding="utf-8").read()
    snapshot = {
        "beforeFixtureJson": quoted(text, "before-fixture-json"),
        "lhsJson": quoted(text, "lhs-json"),
        "rhsJson": fenced(text, "rhs-json"),
        "parameterBindings": bindings(text),
        "ruleLayout": layout(text),
    }
    fixture = json.loads(snapshot["beforeFixtureJson"])
    if fixture["name"] != "Nakagin Capsule Tower — Ground Floor" or len(fixture["nodes"]) != 2:
        raise SystemExit("the committed before-fixture is expected to be the two-node Nakagin ground floor")
    json.loads(snapshot["lhsJson"])
    json.loads(snapshot["rhsJson"])
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(snapshot, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d binding(s), %d layout point(s))" % (target, len(snapshot["parameterBindings"]), len(snapshot["ruleLayout"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2])
