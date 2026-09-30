"""🧪️ W2-W-norm-2: renders each subset's differential case from its committed vectors — the Python reference
adapter (`🐍️.py`: KINDS + VECTORS), the feature (`🥒️.feature`: one mutate and one inverse row per kind plus the
carrier identity) and the oracle manifest's catalog, owner manifest and coverage count (`🔮️oracles/🔣️.json`).

  python3 🧪️w2w-norm-2-cases.py render <artifact>
"""

import json
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
ARTIFACTS = {"en1996": "🪨️en1996", "din16798": "🌬️din16798", "din18599": "⚡️din18599", "din4108": "🧱️din4108"}
CASES = {"en1996": "🪨️mutate-en1996-1", "din16798": "🌬️mutate-din16798-1", "din18599": "⚡️mutate-din18599-1", "din4108": "🧱️mutate-din4108-1"}
NAMES = {"en1996": ("EN 1996", "En1996Mutation"), "din16798": ("DIN EN 16798", "Din16798Mutation"), "din18599": ("DIN V 18599", "Din18599Mutation"), "din4108": ("DIN 4108", "Din4108Mutation")}
ASSETS = {"en1996": "asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio", "din16798": "asset://🎬️demo/🗣️.dsl.semio", "din18599": "asset://🎬️demo/🗣️.dsl.semio", "din4108": "asset://🎬️demo/🗣️.dsl.semio"}
WITNESS = "🧾️wire-witness"

SHAPES = {
    "en1996": """  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The {count} kinds address a masonry
  building at four depths — document scalars (`change-annex`, `change-storeys`), wall fields by position
  (`change-wall-height {{index}}`), opening and load-case fields inside a wall (`{{wallIndex, index}}`)
  and concentrated loads inside a load case (`{{wallIndex, loadCaseIndex, index}}`). That nested
  position addressing is where a second reading written from the addressing convention alone can land
  on the wrong collection, and it is the part of this subset the differential actually tests.""",
    "din16798": """  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The {count} kinds edit an indoor-
  environment building: document scalars (`change-theta-rm`, `change-envelope-n50`), zone fields
  addressed by the zone's native id (`{{zoneId}}`), ventilation-system fields addressed by `{{ventId}}`
  — whose collection is `ventSystems`, so the id itself, not a spelling, locates it — and the insert
  and remove pairs of both collections, whose inverses must restore the removed record at its position.""",
    "din18599": """  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. Nine kinds change a document scalar,
  six `specify-`/`update-` kinds replace one whole system facet, two `replace-` kinds replace the zone
  and element lists, `change-element-u` addresses one element by its native id, and `update-climate`
  addresses the composed climate CHILD, whose content-addressed handle no document here specifies: its
  row is the committed invariant refusal, which both sides must reject bit-identically; its applied wire
  form is witnessed payload-only under `🧾️wire-witness` and held by the crate's payload law.""",
    "din4108": """  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. The {count} kinds edit a thermal-
  protection building at three depths — document scalars, zone/element/bridge fields addressed by
  native id, and window and layer fields addressed inside their zone or element (`{{zoneId, windowId}}`,
  `{{elementId, index}}`) — plus the insert, remove and reorder kinds of every collection, whose
  inverses must restore the build-up exactly, position included.""",
}


def subset(artifact):
    return f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/{ARTIFACTS[artifact]}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def load(path):
    with open(path) as handle:
        return json.load(handle)


def rust_kinds(artifact):
    source = open(f"{subset(artifact)}/🧬️schema/🧬️mutations/🦀️.rs").read()
    block = re.search(r"pub const KINDS: &\[&str\] = &\[(.*?)\];", source, re.S).group(1)
    return re.findall(r'"([^"]+)"', block)


def leaves(artifact):
    root = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    out = {}
    for name in sorted(os.listdir(root)):
        descriptor = f"{root}/{name}/🔣️.json"
        if os.path.exists(descriptor) and os.path.exists(f"{root}/{name}/🧬️schema/🔣️.json"):
            data = load(descriptor)
            out[data["semanticKind"]] = {"dir": name, "variant": data["aggregateVariant"], "outcomes": data["outcomeClasses"]}
    return out


def vectors(artifact, known):
    root = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
    out = {}
    for kind, leaf in known.items():
        scenarios = sorted(name for name in os.listdir(f"{root}/{leaf['dir']}") if name != WITNESS) if os.path.isdir(f"{root}/{leaf['dir']}") else []
        applied = [name for name in scenarios if load(f"{root}/{leaf['dir']}/{name}/🎯️outcome/🔣️.json")["status"] == "applied"]
        row = (applied or scenarios)[0]
        out[kind] = {"scenario": row, "scenarios": scenarios}
    return out


def slug(directory):
    return re.search(r"[a-z0-9][a-z0-9-]*$", directory).group(0)


def renders(directory, kind):
    return slug(directory) == kind


# region Python
def python_adapter(artifact, kinds, known, rows):
    standard, aggregate = NAMES[artifact]
    envelope = f"norm.{artifact}.dsl"
    kind_lines = "\n".join(f'    "{kind}",' for kind in kinds)
    vector_lines = "\n".join(f'    "{kind}": ("{known[kind]["dir"]}", "{rows[kind]["scenario"]}"),' for kind in kinds)
    return f'''"""🐍️ {standard}'s contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is `semio_norm_vocabulary`, the ONE
independent Python implementation of the norm mutation vocabulary, imported here rather than copied. Its
module docstring carries the survey that established no third-party library reads or writes `s.norm.*`
and the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own committed
specification vectors and its own committed example document.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind of `{aggregate}`, in the committed catalog's (declaration) order.
KINDS = [
{kind_lines}
]

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory).
VECTORS = {{
{vector_lines}
}}

#: 🗣️ The real committed {standard} document, read where the domain already keeps it.
DSL_ASSET = "{ASSETS[artifact]}"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "{envelope}"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("{standard}", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
'''
# endregion Python


# region Feature
def table(kinds, known, rows):
    cells = [("id", "dir", "fixture")] + [(kind, known[kind]["dir"], rows[kind]["scenario"]) for kind in kinds]
    widths = [max(len(cell[column]) for cell in cells) for column in range(3)]
    return "\n".join("      | " + " | ".join(cell[column].ljust(widths[column]) for column in range(3)) + " |" for cell in cells)


def feature(artifact, kinds, known, rows):
    standard, aggregate = NAMES[artifact]
    examples = table(kinds, known, rows)
    vector = lambda: "\n".join([
        "    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json",
        "    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json",
        "    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json",
        "    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json",
    ])
    return f"""@capability-{artifact}-1-mutate
@oracle-{artifact}-1-python-independent
@comparison-ordered-json-v1
@mutations-{artifact}-1-any
Feature: Apply every typed {standard} mutation against an independent Python implementation
  `s.norm.{artifact}` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `{artifact}` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `{aggregate}`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the {len(kinds)} kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

{SHAPES[artifact].format(count=len(kinds))}

  Each side asserts the same laws in role — the applied document must BE the committed after-snapshot,
  an `applied` vector must move the document and a `rejected` one must leave it bit-identical, and the
  mutation followed by its OWN computed inverse must restore the before-snapshot exactly. `inverse-`
  projects BOTH the mutated and the restored document, because the restored one is always the
  before-snapshot and projecting only it would make the differential vacuous.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed example
  `{ASSETS[artifact]}`. The carrier has no published grammar (the subset's `📖️.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder), so the two implementations are compared on the
  envelope preamble, the ordered `key=value` fields, the table rows as written, and the digest and
  length of what each side re-emitted — never on a mapping from carrier tokens to the JSON snapshot's
  enum spellings, which is stated nowhere.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
{vector()}
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
{examples}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
{vector()}
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
{examples}

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed {standard} document from the parsed carrier
    Given the real committed text artifact {ASSETS[artifact]}
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
"""
# endregion Feature


# region Manifest
def manifest(artifact, kinds, known, rows):
    path = f"{subset(artifact)}/🔮️oracles/🔣️.json"
    data = load(path)
    case = CASES[artifact]
    for oracle in data["oracles"]:
        if oracle["id"] != f"{artifact}-1-python-independent":
            continue
        oracle["nativeSecondImplementation"]["fixtureCoverage"]["vectors"] = len(kinds)
        text = oracle["rationale"]
        text = re.sub(r"`\.\./\.\./\.\./\.\./\.\./🧪️tests/[^`]*`", f"`../🧪️tests/{case}/🐍️.py` (engine `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`)", text)
        text = re.sub(r"all \d+ kinds", f"all {len(kinds)} kinds", text)
        text = re.sub(r"its \d+-kind mutation vocabulary", f"its {len(kinds)}-kind mutation vocabulary", text)
        text = re.sub(r"with \d+ committed fixture vector\(s\)", f"with {len(kinds)} committed fixture vector(s)", text)
        oracle["rationale"] = text
    catalog = data["mutationCatalogs"][0]
    catalog["kinds"] = kinds
    catalog["vectors"] = [
        {"mutationId": kind, "sourceMutationDirectoryName": known[kind]["dir"], "mutationDirectoryName": known[kind]["dir"], "scenarios": [{"id": slug(name), "directoryName": name} for name in rows[kind]["scenarios"]]}
        for kind in kinds if renders(known[kind]["dir"], kind)
    ]
    owner = data["mutationManifests"][0]
    template = owner["mutations"][0] if owner["mutations"] else {}
    owner["mutations"] = [
        {"id": kind, "capability": f"{artifact}-1-mutate", "payloadSchema": template.get("payloadSchema", "🧬️.schema.json"), "outcomes": known[kind]["outcomes"], "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": known[kind]["variant"]}, "oracleRequirements": [{"capability": f"{artifact}-1-mutate", "qualifyingKind": "verified-native-second-implementation"}]}
        for kind in kinds
    ]
    for stale in ("family", "kinds"):
        data.pop(stale, None)
    return json.dumps(data, ensure_ascii=False, indent=2) + "\n"
# endregion Manifest


def render(artifact):
    kinds = rust_kinds(artifact)
    known = leaves(artifact)
    if set(kinds) != set(known):
        raise SystemExit(f"{artifact}: KINDS and leaf descriptors disagree: {sorted(set(kinds) ^ set(known))}")
    rows = vectors(artifact, known)
    case = f"{subset(artifact)}/🧪️tests/{CASES[artifact]}"
    with open(f"{case}/🐍️.py", "w") as handle:
        handle.write(python_adapter(artifact, kinds, known, rows))
    with open(f"{case}/🥒️.feature", "w") as handle:
        handle.write(feature(artifact, kinds, known, rows))
    text = manifest(artifact, kinds, known, rows)
    with open(f"{subset(artifact)}/🔮️oracles/🔣️.json", "w") as handle:
        handle.write(text)
    skipped = [kind for kind in kinds if not renders(known[kind]["dir"], kind)]
    print(f"{artifact}: {len(kinds)} kinds; catalog vectors {len(kinds) - len(skipped)}; leaf directories not rendering their kind: {skipped}")


if __name__ == "__main__":
    render(sys.argv[2])
