"""🧪️ W2-W-norm-3: authors the EN 1998 specification vectors for the CURRENT 29-kind `En1998Mutation` vocabulary.

The pre-652 flat vocabulary (49 `change-<field>` kinds) and all of its vectors are gone; the mutate case, its Python
oracle, its Rust adapter and its catalog still named them. This script is the single source for the replacement:

* `stage`   — writes each vector's `🦠️mutation`, `🎯️outcome` and `⬅️before` (the Rust-encoded `🏢️seismic-multipart`
              example, read from the dump) plus the canonical per-vector Rust test and the `🔬️fixture` module list;
* `settle`  — reads the `[DEBUG] VECTOR <leaf>/<scenario> <after> <diff>` lines the temporary Rust generator prints and
              writes `➡️after` and `🔺️diff`;
* `surface` — rewrites the catalog (`🔮️oracles/🔣️.json`), the feature's Examples tables, the Python adapter's
              KINDS/VECTORS and the Rust adapter's KINDS/fixture table from the same list.
"""
import copy
import json
import os
import re
import shutil
import sys

SUBSET = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = f"{SUBSET}/🧬️schema/🧬️mutations"
FIXTURES = f"{SUBSET}/🧫️fixtures/🧬️mutations"
CASE = f"{SUBSET}/🧪️tests/🫨️mutate-en1998-1"
ORACLE = f"{SUBSET}/🔮️oracles/🔣️.json"


def camel(kind):
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)


def building(base):
    annex = copy.deepcopy(base["buildings"][0])
    annex.update({"id": "bldg-annex", "name": "Office annex RC frame", "planWidthM": 12.0, "planLengthM": 10.0})
    annex["storeys"] = annex["storeys"][:2]
    annex["members"] = annex["members"][:1]
    return annex


def renamed(record, identity, **fields):
    copied = copy.deepcopy(record)
    copied.update({"id": identity, **fields})
    return copied


#: 🧫️ (kind, leaf directory, scenario directory, payload from the base document), in `En1998Mutation` declaration order.
VECTORS = [
    ("change-annex", "📎️change-annex", "🌍️switches-to-en", lambda b: {"newAnnex": "en"}),
    ("update-site", "🌚️update-site", "🌋️zone-3-soft-soil", lambda b: {"site": {**b["site"], "seismicZone": "zone3", "aGr": 0.8, "deGroundCombo": "C-S"}}),
    ("insert-building", "➕️insert-building", "🏢️adds-an-annex", lambda b: {"index": 1, "building": building(b)}),
    ("remove-building", "➖️remove-building", "🏚️drops-the-office", lambda b: {"index": 0}),
    ("change-system-v-rd-n", "💪️change-system-v-rd-n", "💪️stronger-y", lambda b: {"buildingIndex": 0, "systemIndex": 1, "newBaseShearResistanceN": 1500000.0}),
    ("change-storey-permanent-gk-n", "⚖️change-storey-permanent-gk-n", "⚖️heavier", lambda b: {"buildingIndex": 0, "storeyIndex": 0, "newPermanentGkN": 2500000.0}),
    ("change-storey-stiffness-x", "📐️change-storey-stiffness-x", "📐️softer", lambda b: {"buildingIndex": 0, "storeyIndex": 3, "newStiffnessX": 120000000.0}),
    ("change-storey-drift-xm", "📏️change-storey-drift-xm", "📏️more-drift", lambda b: {"buildingIndex": 0, "storeyIndex": 1, "newDriftXM": 0.012}),
    ("change-building-plan-regular", "🧭️change-building-plan-regular", "🧭️twist", lambda b: {"buildingIndex": 0, "newPlanRegular": False}),
    ("change-elevation-regular", "📏️change-elevation-regular", "🏙️irregular", lambda b: {"buildingIndex": 0, "newElevationRegular": False}),
    ("change-member-detailing", "✅️change-member-detailing", "✅️beam-unfit", lambda b: {"buildingIndex": 0, "memberIndex": 1, "newDetailingCompatibleWithQ": False}),
    ("change-masonry-wall-ratio", "🧱️change-masonry-wall-ratio", "🧱️four-pct", lambda b: {"buildingIndex": 0, "newMasonryWallAreaRatio": 0.04}),
    ("insert-bridge", "🌉insert-bridge", "🌉️adds-a-viaduct", lambda b: {"index": 1, "bridge": {**renamed(b["bridges"][0], "br-2"), "periodRatio": 1.6, "vRdN": 2400000.0, "variables": [{**b["bridges"][0]["variables"][0], "id": "br-2-q1"}]}}),
    ("change-bridge-v-rd-n", "🛑️change-bridge-v-rd-n", "🛑️stronger-pier", lambda b: {"index": 0, "newVRdN": 2500000.0}),
    ("insert-assessment", "🔧insert-assessment", "🔧️adds-a-kl3-check", lambda b: {"index": 1, "assessment": {**renamed(b["assessments"][0], "as-2"), "knowledgeLevel": "kl3", "limitState": "nc", "rKN": 650000.0}}),
    ("change-assessment-rkn", "🏋️change-assessment-rkn", "🏋️stronger", lambda b: {"index": 0, "newRKN": 650000.0}),
    ("insert-silo", "🫙insert-silo", "🫙️adds-a-grain-silo", lambda b: {"index": 1, "silo": {**renamed(b["silos"][0], "si-2"), "heightM": 16.0, "radiusM": 3.0}}),
    ("insert-tank", "🛢insert-tank", "🛢️adds-a-water-tank", lambda b: {"index": 1, "tank": {**renamed(b["tanks"][0], "tk-2"), "heightM": 8.0, "radiusM": 6.0}}),
    ("insert-foundation", "🪨insert-foundation", "🪨️adds-a-pad", lambda b: {"index": 1, "foundation": {**renamed(b["foundations"][0], "fd-2"), "areaM2": 120.0}}),
    ("insert-retaining-wall", "🧱️insert-retaining-wall", "🧱️adds-a-wall", lambda b: {"index": 1, "wall": {**renamed(b["retainingWalls"][0], "rw-2"), "heightM": 6.0}}),
    ("insert-tower", "🗼insert-tower", "🏭️adds-a-chimney", lambda b: {"index": 1, "tower": {**renamed(b["towers"][0], "tw-2"), "isChimney": True, "heightM": 60.0, "variables": [{**b["towers"][0]["variables"][0], "id": "tw-2-q1"}]}}),
    ("change-tower-m-rd-nm", "↪️change-tower-m-rd-nm", "↪️stronger-base", lambda b: {"index": 0, "newMRdNm": 9500000.0}),
    ("remove-bridge", "➖️remove-bridge", "🌉️drops-the-viaduct", lambda b: {"index": 0}),
    ("remove-assessment", "➖️remove-assessment", "🔧️drops-the-check", lambda b: {"index": 0}),
    ("remove-silo", "➖️remove-silo", "🫙️drops-the-silo", lambda b: {"index": 0}),
    ("remove-tank", "➖️remove-tank", "🛢️drops-the-tank", lambda b: {"index": 0}),
    ("remove-foundation", "➖️remove-foundation", "🪨️drops-the-pad", lambda b: {"index": 0}),
    ("remove-retaining-wall", "➖️remove-retaining-wall", "🧱️drops-it", lambda b: {"index": 0}),
    ("remove-tower", "➖️remove-tower", "🗼️drops-the-tower", lambda b: {"index": 0}),
]


def pretty(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def slug(scenario):
    return re.sub(r"^[^a-z0-9]+", "", scenario)


def module(kind, scenario):
    return "vector_" + (kind + "_" + slug(scenario)).replace("-", "_")


TEST = '''//! 🧪️ Committed vector `{leaf}` / `{scenario}`: the mutation reaches the committed after-snapshot and its own inverse restores the before-snapshot.
use crate::{{En1998Diff, En1998Mutation, En1998Snapshot}};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🎯️outcome/🔣️.json");
fn before() -> En1998Snapshot {{ pack::json::from_json_str(BEFORE).expect("before") }}
fn mutation() -> En1998Mutation {{ pack::json::from_json_str(MUTATION).expect("mutation") }}
fn apply(mutation: &En1998Mutation, base: &En1998Snapshot) -> En1998Snapshot {{
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "{kind} raised {{:?}}", raised.messages());
    <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(raised.diff(), base).expect("apply")
}}
#[test]
fn applies_to_committed_after_and_diff() {{
    let base = before();
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1998Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "{kind} must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1998Snapshot>(AFTER).expect("after"));
}}
#[test]
fn inverse_restores_before() {{
    let base = before();
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "{kind} changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}}
#[test]
fn declared_outcome_holds() {{
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}}
'''


def stage(base_path):
    base = json.load(open(base_path, encoding="utf-8"))
    modules = []
    for kind, leaf, scenario, payload in VECTORS:
        tests = f"{MUTATIONS}/{leaf}/🧪️tests"
        if os.path.isdir(tests):
            for stale in os.listdir(tests):
                if stale != scenario:
                    shutil.rmtree(f"{tests}/{stale}")
        stem = f"{FIXTURES}/{leaf}/{scenario}"
        tag = json.load(open(f"{MUTATIONS}/{leaf}/🧬️schema/🔣️.json", encoding="utf-8"))["properties"]["mutation"]["const"]
        write(f"{stem}/🦠️mutation/🔣️.json", pretty({"mutation": tag, **payload(base)}))
        write(f"{stem}/🎯️outcome/🔣️.json", pretty({"status": "applied"}))
        write(f"{stem}/📸️snapshot/⬅️before/🔣️.json", pretty(base))
        write(f"{tests}/{scenario}/🦀️.rs", TEST.format(leaf=leaf, scenario=scenario, kind=kind))
        modules.append(f'#[path = "../../{leaf}/🧪️tests/{scenario}/🦀️.rs"]\nmod {module(kind, scenario)};\n')
    write(f"{MUTATIONS}/🧪️tests/🔬️fixture/🦀️.rs", "//! 🧫️ One canonical test per committed EN 1998 specification vector.\n" + "".join(modules))
    print("staged", len(VECTORS), "vectors")


def settle(capture):
    pattern = re.compile(r"^\[DEBUG\] VECTOR (\S+) (\{.*\}) (\{.*\})$")
    settled = 0
    for line in open(capture, encoding="utf-8"):
        match = pattern.match(line.rstrip("\n"))
        if match:
            stem = f"{FIXTURES}/{match.group(1)}"
            write(f"{stem}/📸️snapshot/➡️after/🔣️.json", pretty(json.loads(match.group(2))))
            write(f"{stem}/🔺️diff/🔣️.json", pretty(json.loads(match.group(3))))
            settled += 1
    print("settled", settled, "of", len(VECTORS))


FEATURE = """@capability-en1998-1-mutate
@oracle-en1998-1-python-independent
@comparison-ordered-json-v1
@mutations-en1998-1-any
Feature: Apply every typed EN 1998 mutation against an independent Python implementation
  `s.norm.en1998` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1998` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1998Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this
  subset's kind list, vectors and carrier. It is written from the repository's own specification of what
  a semantic mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention
  and the derivation rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `shared://` fixture, so neither side holds a transcription that could
  drift. All twenty-nine vectors start from the committed `🏢️seismic-multipart` example, the one
  document that carries every EN 1998 part at once — an office RC frame (part 1) with two lateral
  systems, four storeys and two members, a bridge (part 2), an assessed element (part 3), a silo and a
  tank (part 4), a foundation and a retaining wall (part 5) and a tower (part 6). The vocabulary has
  three addressing shapes and each is exercised: the document-level `change-annex` and the whole-facet
  `update-site`; the positional `insert-<part>` / `remove-<part>` pairs and `change-<part>-<field>
  {index}` of every part list; and the NESTED `{buildingIndex, storeyIndex|systemIndex|memberIndex}`
  addressing of the building's own storeys, lateral systems and members, which is where a resolution
  into the wrong list would still find a field of the right name. Each side asserts the same laws in
  role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot
  exactly, list position included. `parity` adds that two implementations, in two languages, reach the
  same document.

  `inverse-` projects BOTH the mutated and the restored document: for every `insert-<part>` the inverse
  is the matching `remove-<part>` at the inserted position, and for every `remove-<part>` it is the
  `insert-<part>` of the removed record, so the mutated half is what distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `📚️examples/🏢️seismic-rc-frame/🖼️assets/🏢️seismic-rc-frame/🗣️.dsl.semio` — a named
  reinforced-concrete frame. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides
  are compared at the envelope preamble, the ordered `key=value` fields and the digest and length of
  what each re-emitted. The Rust side additionally proves it PARSED the document: the committed binary
  twin must decode to the same document as the text, through a separately written codec.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
{rows}
  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
{rows}
  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1998 document from the parsed carrier
    Given the real committed text artifact asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio
    And its committed binary twin asset://🏢️seismic-rc-frame/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
"""


def table():
    width = [max(len(entry[i]) for entry in [("id", "dir", "fixture")] + [(k, l, s) for k, l, s, _ in VECTORS]) for i in range(3)]
    line = lambda cells: "      | " + " | ".join(cell.ljust(width[i]) for i, cell in enumerate(cells)) + " |\n"
    return line(("id", "dir", "fixture")) + "".join(line((kind, leaf, scenario)) for kind, leaf, scenario, _ in VECTORS)


def variant(kind):
    return "".join(part.capitalize() for part in kind.split("-"))


def surface():
    kinds = [kind for kind, _, _, _ in VECTORS]
    variants = {}
    for kind, leaf, _, _ in VECTORS:
        descriptor = json.load(open(f"{MUTATIONS}/{leaf}/🔣️.json", encoding="utf-8"))
        assert descriptor["semanticKind"] == kind, (leaf, descriptor["semanticKind"])
        variants[kind] = (descriptor["aggregateVariant"], descriptor["outcomeClasses"])
    oracle = json.load(open(ORACLE, encoding="utf-8"))
    catalog = oracle["mutationCatalogs"][0]
    catalog["kinds"] = kinds
    catalog["vectors"] = [{"mutationId": kind, "sourceMutationDirectoryName": leaf, "mutationDirectoryName": leaf, "scenarios": [{"id": slug(scenario), "directoryName": scenario}]} for kind, leaf, scenario, _ in VECTORS]
    manifest = oracle["mutationManifests"][0]
    template = manifest["mutations"][0]
    manifest["mutations"] = [{**copy.deepcopy(template), "id": kind, "outcomes": variants[kind][1], "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": variants[kind][0]}} for kind in kinds]
    entry = oracle["oracles"][0]
    entry["rationale"] = entry["rationale"].replace("`../../../../../🧪️tests/mutate-en1998-1/🐍️component.py`, all 49 kinds of `En1998Mutation`", f"the shared norm reference engine `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py` fed by `../🧪️tests/🫨️mutate-en1998-1/🐍️.py`, all {len(kinds)} kinds of `En1998Mutation`").replace("for its 49-kind mutation vocabulary", f"for its {len(kinds)}-kind mutation vocabulary").replace("with 49 committed fixture vector(s)", f"with {len(kinds)} committed fixture vector(s)")
    entry["nativeSecondImplementation"]["fixtureCoverage"]["vectors"] = len(kinds)
    write(ORACLE, json.dumps(oracle, ensure_ascii=False, indent=2) + "\n")
    write(f"{CASE}/🥒️.feature", FEATURE.replace("{rows}", table()))
    python = open(f"{CASE}/🐍️.py", encoding="utf-8").read()
    head, rest = python.split("KINDS = [", 1)
    _, tail = rest.split("#: 🗣️ The real committed EN 1998 document", 1)
    body = "KINDS = [\n" + "".join(f'    "{kind}",\n' for kind in kinds) + "]\n\n#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).\nVECTORS = {\n" + "".join(f'    "{kind}": ("{leaf}", "{scenario}"),\n' for kind, leaf, scenario, _ in VECTORS) + "}\n\n#: 🗣️ The real committed EN 1998 document"
    write(f"{CASE}/🐍️.py", head + body + tail)
    rust = open(f"{CASE}/🦀️.rs", encoding="utf-8").read()
    head, rest = rust.split("const KINDS: &[&str] = &[", 1)
    _, rest = rest.split("];", 1)
    rust = head + "const KINDS: &[&str] = &[\n" + "".join(f'    "{kind}",\n' for kind in kinds) + "];" + rest
    head, rest = rust.split("    match kind {\n", 1)
    _, tail = rest.split("        other => panic!(", 1)
    arms = "".join(f'''        "{kind}" => (
            include_str!("../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🎯️outcome/🔣️.json"),
        ),
''' for kind, leaf, scenario, _ in VECTORS)
    write(f"{CASE}/🦀️.rs", head + "    match kind {\n" + arms + "        other => panic!(" + tail)
    print("surfaced", len(kinds), "kinds")


if __name__ == "__main__":
    {"stage": lambda: stage(sys.argv[2]), "settle": lambda: settle(sys.argv[2]), "surface": surface}[sys.argv[1]]()
