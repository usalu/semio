#!/usr/bin/env python3
"""🗂️ Derives the `mutate-curate-1` case fixture ONCE from the committed real curation example.

Provenance, in full:

* Input ``✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  — the artifact's own committed demo document: a `s.stdio.semio@v1/kit` catalogue child handle and
  ten real stock entries across beams, windows and slabs, each with its real availability, its real
  typology path and its real dimensioned geometry, against an EMPTY curation.
* Output ``…/🧪️tests/mutate-curate-1/🧫️fixtures/🗂️timber-kit.snapshot.json``.

What is carried over unchanged: the catalogue handle and all ten stock entries, id for id,
availability for availability, typology path for typology path and geometry for geometry. The
carrier does not carry a `name` or a `moduleId` for a stock entry at all, so both are written as the
empty string the schema's `type: "string"` admits, and that is stated here rather than invented.

What is derived: the curation itself, which the committed example leaves empty — so
`delete-curated-item` and `change-curated-item-count` would address nothing and their inverse law
would be vacuous. Three of the ten REAL stock ids are curated, one per typology family, each at HALF
its own committed availability, rounded down:

    beam-glulam-gl24h          availability 24 → count 12
    window-tilt-turn-120x140   availability 14 → count  7
    slab-clt-160               availability 20 → count 10

Every id and every number therefore traces to the committed file. Nothing is invented.

Note recorded while writing this: this repository's own `parse_curate_dsl` currently REJECTS that
committed example (`TextError { message: "expected Text, found Absent", … line 1, column 1 }`), which
is why the fixture is derived by reading the carrier here rather than by running the subset's codec.
That failure predates this ticket's conversion work and is reported as a finding, not routed around.
"""

# region 🔖️Imports
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
GEOMETRY_FIELDS = {"box": ("width", "height", "depth"), "frame": ("width", "height", "depth", "profile"), "slab": ("width", "depth", "thickness")}
"""📐️ The dimension names each geometry recipe declares, from the committed snapshot JSON Schema."""


def number(token):
    """🔢️ A dimensioned carrier scalar — the carrier writes metres with an `m` suffix."""
    return float(token[:-1]) if token.endswith("m") else float(token)


def catalog_handle(text):
    """🔗️ The `s.stdio.semio@v1/kit` catalogue child the committed document composes."""
    child = text.split("child_id=", 1)[1].split(" ", 1)[0]
    target = text.split('target="', 1)[1].split('"', 1)[0]
    artifact, _, dialect = target.partition("!")
    kind, _, rest = dialect.partition("@")
    standard, _, subset = rest.partition("/")
    return {"childId": child, "target": {"artifactId": artifact, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}}


def stock_entries(text):
    """📦️ Every committed stock entry, in the order the carrier writes them.

    The carrier's own layout puts an entry's geometry recipe at the START of the line that follows
    its `typology-path`, so the member list is read as one flat token stream rather than line by
    line.
    """
    blob = text[text.index("stock-extra=[") + len("stock-extra=[") : text.index("\ncurated ")]
    tokens = blob.replace("\n", " ").split()
    entries = []
    at = 0
    while at < len(tokens):
        if not tokens[at].startswith("id="):
            at += 1
            continue
        entry = {"id": tokens[at][3:], "name": "", "moduleId": "", "typologyPath": [], "availability": 0, "geometry": None}
        at += 1
        while at < len(tokens) and not tokens[at].startswith("id="):
            token = tokens[at]
            if token.startswith("availability="):
                entry["availability"] = int(token[len("availability=") :])
                at += 1
            elif token == "typology-path=[":
                at += 1
                while tokens[at] != "]":
                    entry["typologyPath"].append(tokens[at])
                    at += 1
                at += 1
            elif token in GEOMETRY_FIELDS:
                recipe = {"kind": token}
                at += 1
                for name in GEOMETRY_FIELDS[token]:
                    key, _, value = tokens[at].partition("=")
                    if key != name:
                        raise SystemExit("expected %s for a %s recipe, found %r" % (name, recipe["kind"], tokens[at]))
                    recipe[name] = number(value)
                    at += 1
                entry["geometry"] = recipe
            else:
                at += 1
        entries.append(entry)
    return entries


# endregion 🔖️Carrier


# region 🔖️Derivation
CURATED = ("beam-glulam-gl24h", "window-tilt-turn-120x140", "slab-clt-160")
"""🧺️ One real stock id per typology family, curated at half its own committed availability."""


def derive(source, target):
    """🧬️ Writes the derived snapshot: the committed catalogue and stock, plus a real curation."""
    text = open(source, encoding="utf-8").read()
    entries = stock_entries(text)
    if len(entries) != 10:
        raise SystemExit("the committed example is expected to carry ten stock entries, read %d" % len(entries))
    availability = {entry["id"]: entry["availability"] for entry in entries}
    missing = [identifier for identifier in CURATED if identifier not in availability]
    if missing:
        raise SystemExit("the committed example carries no stock entry %r" % missing)
    snapshot = {
        "catalog": catalog_handle(text),
        "stockExtra": entries,
        "curated": [{"objectId": identifier, "count": availability[identifier] // 2} for identifier in CURATED],
    }
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(snapshot, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d stock entr(ies), %d curated)" % (target, len(entries), len(snapshot["curated"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2])
