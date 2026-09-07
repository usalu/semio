#!/usr/bin/env python3
"""🔮️ W13 — teaches all three languages the whole fem2d corpus, not one row per kind.

Before this wave each subset's `🥒️.feature` named exactly ONE committed vector per kind, in a
single `spec-vector` `Scenario Outline`, so 73 of the 98 committed vectors were Rust-only evidence
(W10 finding F9, W11 §5). Scenario ids must be unique inside a feature and both adapters register
handlers by FULL scenario id, so widening the corpus is a paired edit in three files per subset.

What this script writes, per subset:

* `🥒️.feature` — two new `Scenario Outline`s. `@id-frame-vector` replays the steel-frame happy path
  W10 authored (scenario id `frame-vector-<kind>`); `@id-reject` replays every refusal and no-op
  vector (scenario id `reject-<kind>-<n>`, numbered in catalog order).
* `🐍️.py` — the independent Python reference, regenerated with the HARDENED rules and a
  `reject_handler` whose projection is `{model, refusal}`. The refusal half is what makes the new
  rows a real differential rather than two implementations agreeing that nothing happened: both
  sides must raise the same `code`, the same `level` and the same `target`.
* `🦀️.rs` — the Rust SUBJECT half, with `spec_vector` generalized to any committed vector (keyed by
  scenario id rather than by kind) and a `reject` handler projecting the same `{model, refusal}`.

The `🐍️.py` is written to BOTH copies — the subset-owned case and the `🌐️any` duplicate — because
they are byte-identical today and nothing in the file depends on its location. The `🦀️.rs` is
written only to the subset-owned case: the `🌐️any` duplicate deliberately dropped the
committed-vector Outline (its `include_str!` paths cannot reach sideways past the escape guard) and
carries only `mutate-`/`inverse-`.

Usage:
    uv run python 🔨️w13-fem2d-oracles.py            # write
    uv run python 🔨️w13-fem2d-oracles.py --check    # report differences, write nothing
"""

# region 🔖️Imports
import json
import os
import re
import sys

# endregion 🔖️Imports


# region 🔖️Paths
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")

CASES = {
    "🏋️load": ("🏋️mutate-fem2d-1-load", "🏋️mutate-fem2d-1-any-load", "load"),
    "📈️analysis": ("📈️mutate-fem2d-1-analysis", "📈️mutate-fem2d-1-any-analysis", "analysis"),
    "🕸️mesh": ("🕸️mutate-fem2d-1-mesh", "🕸️mutate-fem2d-1-any-mesh", "mesh"),
    "🛡️boundary": ("🛡️mutate-fem2d-1-boundary", "🛡️mutate-fem2d-1-any-boundary", "boundary"),
    "🧱️material": ("🧱️mutate-fem2d-1-material", "🧱️mutate-fem2d-1-any-material", "material"),
}
"""🗂️ Per subset: its own case directory, the `🌐️any` duplicate, and the subset's plain name."""
# endregion 🔖️Paths


# region 🔖️Corpus
def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def corpus(subset):
    """🗂️ Every committed vector of one subset, per kind, in catalog order, classified.

    `happy` (`applied`, no diagnostics) / `reject` (`rejected`) / `noop` (`applied` with a warning).
    The catalog is the registry the coordinator itself checks against disk, so reading the order
    from it keeps the feature's rows and the mount list in the same sequence.
    """
    catalog = read(os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json"))
    found = {}
    for declaration in catalog["mutationCatalogs"]:
        for vector in declaration["vectors"]:
            rows = []
            for scenario in vector["scenarios"]:
                root = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", vector["mutationDirectoryName"], "🧪️tests", scenario["directoryName"])
                outcome = read(os.path.join(root, "🎯️outcome", "🔣️.json"))
                family = "reject" if outcome["status"] == "rejected" else ("noop" if outcome.get("messages") else "happy")
                rows.append({"id": scenario["id"], "directory": scenario["directoryName"], "family": family, "kind": vector["mutationId"], "mutation_directory": vector["mutationDirectoryName"]})
            found[vector["mutationId"]] = rows
    return found


def existing_spec_fixture(subset):
    """📜️ The one fixture each kind's pre-existing `spec-vector` row already names in the feature."""
    text = open(os.path.join(SUBSETS, subset, "🧪️tests", CASES[subset][0], "🥒️.feature"), encoding="utf-8").read()
    outline = text.split("@id-spec-vector", 1)[1].split("\n  @id-", 1)[0]
    found = {}
    for line in outline.splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")] if line.strip().startswith("|") else []
        if len(cells) == 3 and cells[0] not in ("id",):
            found[cells[0]] = cells[2]
    return found


def plan(subset):
    """🗺️ Per kind: the pre-existing spec row, the steel-frame row, and the numbered refusal rows."""
    rows = corpus(subset)
    pinned = existing_spec_fixture(subset)
    built = {}
    for kind, entries in rows.items():
        happy = [entry for entry in entries if entry["family"] == "happy"]
        others = [entry for entry in entries if entry["family"] != "happy"]
        assert len(happy) == 2, "%s: expected exactly two happy vectors, found %d" % (kind, len(happy))
        assert kind in pinned, "%s: the feature names no spec-vector fixture" % kind
        spec = next(entry for entry in happy if entry["directory"] == pinned[kind])
        frame = next(entry for entry in happy if entry["directory"] != pinned[kind])
        built[kind] = {"spec": spec, "frame": frame, "rejects": others}
    return built


# endregion 🔖️Corpus


# region 🔖️Feature
FRAME_OUTLINE = """
  @id-frame-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> steel-frame specification vector through both implementations
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
{frame_rows}
"""

REJECT_OUTLINE = """
  @id-reject
  @level-exhaustive
  @mode-differential
  Scenario Outline: Refuse the committed <id> vector in both implementations, for the same reason
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then both implementations leave the document exactly where it was and report the same diagnostic code, level and address
    Examples:
{reject_rows}
"""

FEATURE_PROSE = """
  THE WHOLE CORPUS, NOT ONE ROW PER KIND (ticket `26/09/06/FEM-PLUGIN-END-TO-END`). Case discovery
  is explicit in all three languages, so until this wave the two `Scenario Outline`s below did not
  exist and every committed vector but one per kind was Rust-only evidence. `frame-vector-<kind>`
  replays the second happy path — the two-storey braced steel frame — and `reject-<kind>-<n>`
  replays every refusal and no-op the kind declares, holding BOTH implementations to the same
  diagnostic code, level and address rather than merely to an unchanged document.
"""


def table(rows):
    """📋️ A Gherkin `Examples` table, padded so the columns line up."""
    header = ["id", "dir", "fixture"]
    body = [header] + rows
    widths = [max(len(row[at]) for row in body) for at in range(3)]
    return "\n".join("    | " + " | ".join(cell.ljust(widths[at]) for at, cell in enumerate(row)) + " |" for row in body)


def render_feature(subset, built):
    """🥒️ The subset feature, with this wave's two Outlines appended (and refreshed on a rerun)."""
    path = os.path.join(SUBSETS, subset, "🧪️tests", CASES[subset][0], "🥒️.feature")
    text = open(path, encoding="utf-8").read()
    text = text.split("\n  @id-frame-vector")[0].rstrip("\n") + "\n"
    if FEATURE_PROSE.strip() not in text:
        head, rest = text.split("\n  @id-mutate", 1)
        text = head + "\n" + FEATURE_PROSE + "\n  @id-mutate" + rest
    frame_rows = [[kind, built[kind]["frame"]["mutation_directory"], built[kind]["frame"]["directory"]] for kind in sorted(built)]
    reject_rows = [["%s-%d" % (kind, at + 1), entry["mutation_directory"], entry["directory"]] for kind in sorted(built) for at, entry in enumerate(built[kind]["rejects"])]
    text = text + FRAME_OUTLINE.format(frame_rows=table(frame_rows)) + REJECT_OUTLINE.format(reject_rows=table(reject_rows))
    return path, text.rstrip("\n") + "\n"


# endregion 🔖️Feature


# region 🔖️Python
PYTHON = '''#!/usr/bin/env python3
"""🏗️ An INDEPENDENT second implementation of the `s.fem.fem2d` structural model and this
subset's typed mutations ({kinds_prose}), in Python, serving as this case's differential oracle.
Relocated out of the artifact-level `mutate-fem2d-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`, and
extended to the vocabulary's REFUSALS in ticket `26/09/06/FEM-PLUGIN-END-TO-END`.

**Why a second implementation and not a third-party library.** What this vocabulary edits is the
MODEL, not the analysis: nine id-keyed collections and one settings record. A finite-element solver
(`code_aster`, `OpenSees`, `anastruct`, `PyNite`) computes displacements and forces from a model;
none of them reads `.dsl.semio`, none defines this document, and none of them has an opinion about
whether deleting a still-referenced material is legal. What a reference genuinely can adjudicate is
the model algebra, and that is what this file implements, from the specification, in another
language. It carries the FULL nine-member model shape — not only this subset's own collections —
because every scenario asserts, in role, that a mutation moved exactly the one member it was meant
to and left the other eight untouched.

**What it was written from.**

* ``../../../🌐️any/🧬️schema/📸️snapshot/🔣️.json`` — `Fem2dSnapshot` is exactly those nine
  members, `additionalProperties: false`. Since the 26/09/06 wave its record `$defs` are no longer
  empty: they carry the field shapes AND the admissibility bounds (`exclusiveMinimum` on a modulus,
  a density, an area, a second moment, a thickness, a mesh size and the deformation scale; the open
  Poisson interval; `minItems: 3` on a region outline; `minimum: 1` on each mode count). Every bound
  this file enforces is read from there.
* ``…/🧬️schema/🧬️mutations/<kind>/🧬️.schema.json`` — the per-kind wire payloads, internally tagged
  with `mutation`.
* the committed `(before, mutation, after, outcome)` specification vectors — where the referential
  rules are written down: which `delete-` refuses while referrers exist and which is deliberately
  cascade-free, and which diagnostic code and address each refusal carries.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")
"""🗂️ The nine members `Fem2dSnapshot` declares — and the cross-language projection. Every
member is validated on every scenario regardless of which one this subset's kinds write, because
the model always carries all nine."""

COLLECTIONS = {{
    "node": ("nodes", "node", "newNode"),
    "element": ("elements", "element", "newElement"),
    "region": ("regions", "region", "newRegion"),
    "material": ("materials", "material", "newMaterial"),
    "section": ("sections", "section", "newSection"),
    "support": ("supports", "support", "newSupport"),
    "load-case": ("loadCases", "loadCase", None),
    "combination": ("combinations", "combination", None),
}}
"""🗂️ Per noun: its collection, the argument `create-` carries, and the one `replace-` carries when
the vocabulary has a `replace-` for it at all."""

KINDS = {kinds_tuple}
"""🏷️ This subset's own kinds, in the catalog's declared order."""

REFUSALS = {refusals}
"""🚫️ How many refusal-or-no-op vectors each kind declares — the `reject-<kind>-<n>` rows the
feature's `@id-reject` Outline carries, numbered in the catalog's own order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {{kind: tag_of(kind) for kind in KINDS}}

RECORDS = {{
    "nodes": {{"id", "x", "y"}},
    "elements": {{"kind", "id", "start", "end", "materialId", "sectionId"}},
    "regions": {{"id", "name", "outline", "holes", "thickness", "materialId", "meshSize"}},
    "materials": {{"id", "name", "e", "nu", "rho"}},
    "sections": {{"id", "name", "area", "iy"}},
    "supports": {{"id", "nodeId", "fixed"}},
    "loadCases": {{"id", "name", "loads", "selfWeight"}},
    "combinations": {{"id", "name", "terms"}},
}}
"""🧱️ The members each record carries, as the snapshot schema spells them — the FULL nine-member
model shape, needed to validate the collections this subset's own kinds do not write."""

LOADS = {{"nodal": {{"kind", "id", "nodeId", "dof", "value"}}, "memberUdl": {{"kind", "id", "elementId", "wx", "wy"}}, "area": {{"kind", "id", "regionId", "pressure"}}}}
"""🏋️ The three load variants, as the schema and the committed vectors spell them."""

DUPLICATE_ID, ID_MISMATCH, INVARIANT = "mutation.duplicate-id", "mutation.id-mismatch", "mutation.invariant"
TARGET_MISSING, TARGET_REFERENCED, NO_OP = "mutation.target-missing", "mutation.target-referenced", "mutation.no-op"
"""🚦️ The closed diagnostic vocabulary. The three Fatal codes say the PAYLOAD is inadmissible on any
base; the two Error codes say THIS base cannot host it; `no-op` is a Warning beside an applied,
empty change."""


class Refusal(AssertionError):
    """🚫️ One diagnostic raised instead of a change — an `AssertionError` so a caller that only
    knows the scenario failed still learns why."""

    def __init__(self, code, level, target, message):
        super().__init__("%s [%s] %s" % (code, level, message))
        self.code, self.level, self.target, self.message = code, level, list(target), message


def fatal(code, target, message):
    raise Refusal(code, "fatal", target, message)


def error(code, target, message):
    raise Refusal(code, "error", target, message)


def warn(code, message):
    raise Refusal(code, "warning", [], message)


# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the model to the shape and the bounds the snapshot schema declares, and to id
    uniqueness within every collection — including load ids within one case."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a fem2d model must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["analysis"]) != {{"modalCount", "bucklingCount", "deformationScale"}}:
        raise AssertionError("analysis must carry exactly the three declared settings, found %r" % sorted(document["analysis"]))
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for case in document["loadCases"]:
        loads = []
        for load in case["loads"]:
            if load.get("kind") not in LOADS or set(load) != LOADS[load["kind"]]:
                raise AssertionError("load %r of case %r is not one of the three declared variants" % (load, case["id"]))
            loads.append(load["id"])
        if len(set(loads)) != len(loads):
            raise AssertionError("case %r carries a duplicate load id: %r" % (case["id"], loads))


def document_of(payload):
    """📥️ Reads a fem2d model out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


def find(items, identifier):
    """🔎️ The index of an id in a collection, or `None`."""
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def noun_of(kind):
    """🏷️ The noun a `create-`/`delete-`/`replace-` kind names."""
    return kind.split("-", 1)[1]


# endregion 🔖️Document


# region 🔖️Bounds
def finite(*values):
    return all(isinstance(value, (int, float)) and math.isfinite(float(value)) for value in values)


def check_node(record):
    """📍️ The snapshot schema's node coordinates are plain numbers, so a NaN or an infinity is not
    one — a non-finite ordinate poisons every stiffness matrix the node enters."""
    if not finite(record["x"], record["y"]):
        fatal(INVARIANT, [record["id"]], 'Node "%s" carries a non-finite coordinate.' % record["id"])


def check_material(record):
    """🧱️ `e` and `rho` carry `exclusiveMinimum: 0`; `nu` carries the open interval (-1, 0.5), where
    the upper limit is incompressibility and a singular plane constitutive matrix."""
    identifier = record["id"]
    if not finite(record["e"], record["nu"], record["rho"]):
        fatal(INVARIANT, [identifier], 'Material "%s" carries a non-finite property.' % identifier)
    if record["e"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a positive Young\\'s modulus.' % identifier)
    if record["rho"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a positive density.' % identifier)
    if not -1.0 < record["nu"] < 0.5:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a Poisson ratio in (-1, 0.5).' % identifier)


def check_section(record):
    """📏️ `area` and `iy` carry `exclusiveMinimum: 0` — at zero the member has no axial or no
    bending stiffness while still claiming to be there."""
    identifier = record["id"]
    if not finite(record["area"], record["iy"]):
        fatal(INVARIANT, [identifier], 'Section "%s" carries a non-finite property.' % identifier)
    if record["area"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Section "%s" needs a positive area.' % identifier)
    if record["iy"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Section "%s" needs a positive second moment of area.' % identifier)


def ring_area(ring):
    """📐️ The signed shoelace area of a closed ring; the sign carries the winding."""
    total = 0.0
    for at, point in enumerate(ring):
        following = ring[(at + 1) % len(ring)]
        total += point[0] * following[1] - following[0] * point[1]
    return total / 2.0


def inside(point, ring):
    """🎯️ Crossing-number containment with the boundary counted as INSIDE, so a hole may touch the
    outline it is cut from (a notch) but not leave it."""
    for at, start in enumerate(ring):
        end = ring[(at + 1) % len(ring)]
        cross = (end[0] - start[0]) * (point[1] - start[1]) - (end[1] - start[1]) * (point[0] - start[0])
        span = min(start[0], end[0]) - 1e-12 <= point[0] <= max(start[0], end[0]) + 1e-12 and min(start[1], end[1]) - 1e-12 <= point[1] <= max(start[1], end[1]) + 1e-12
        if abs(cross) <= 1e-12 and span:
            return True
    crossings = False
    for at, start in enumerate(ring):
        end = ring[(at + 1) % len(ring)]
        if (start[1] > point[1]) != (end[1] > point[1]):
            crossing = start[0] + (point[1] - start[1]) * (end[0] - start[0]) / (end[1] - start[1])
            if point[0] < crossing:
                crossings = not crossings
    return crossings


def check_region(record):
    """🗺️ `outline` carries `minItems: 3`, `thickness` and `meshSize` carry `exclusiveMinimum: 0`,
    and a hole is a polygon lying inside the outline — otherwise the meshing pass produces an empty
    or self-overlapping triangulation with no diagnostic naming the edit that caused it."""
    identifier = record["id"]
    outline = [tuple(point) for point in record["outline"]]
    if len(outline) < 3:
        fatal(INVARIANT, [identifier], 'Region "%s" needs an outline of at least three points.' % identifier)
    if any(not finite(*point) for point in outline):
        fatal(INVARIANT, [identifier], 'Region "%s" carries a non-finite outline coordinate.' % identifier)
    if ring_area(outline) == 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" has a degenerate outline enclosing zero area.' % identifier)
    if not finite(record["thickness"]) or record["thickness"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" needs a positive thickness.' % identifier)
    if not finite(record["meshSize"]) or record["meshSize"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" needs a positive mesh size.' % identifier)
    for at, hole in enumerate(record["holes"]):
        ring = [tuple(point) for point in hole]
        if len(ring) < 3:
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d needs at least three points.' % (identifier, at))
        if any(not finite(*point) for point in ring):
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d carries a non-finite coordinate.' % (identifier, at))
        if ring_area(ring) == 0.0:
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d is degenerate.' % (identifier, at))
        if any(not inside(point, outline) for point in ring):
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d leaves the outline.' % (identifier, at))


def check_analysis(settings):
    """⚙️ Both counts carry `minimum: 1` and the scale `exclusiveMinimum: 0` — asking for zero modes
    asks the solver for an empty spectrum."""
    if settings["modalCount"] < 1:
        fatal(INVARIANT, [], "Analysis settings need at least one modal mode.")
    if settings["bucklingCount"] < 1:
        fatal(INVARIANT, [], "Analysis settings need at least one buckling mode.")
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        fatal(INVARIANT, [], "Analysis settings need a positive deformation scale.")


# endregion 🔖️Bounds


# region 🔖️References
def resolve_node(document, node_id):
    if find(document["nodes"], node_id) is None:
        error(TARGET_MISSING, [node_id], 'Node "%s" does not exist.' % node_id)


def resolve_material(document, material_id):
    if find(document["materials"], material_id) is None:
        error(TARGET_MISSING, [material_id], 'Material "%s" does not exist.' % material_id)


def resolve_element(document, record):
    """🔗️ The four foreign keys every element carries, in the order the committed vectors read
    them: `start`, `end`, `materialId`, `sectionId`."""
    resolve_node(document, record["start"])
    resolve_node(document, record["end"])
    resolve_material(document, record["materialId"])
    if find(document["sections"], record["sectionId"]) is None:
        error(TARGET_MISSING, [record["sectionId"]], 'Section "%s" does not exist.' % record["sectionId"])


def resolve_load(document, load):
    """🔗️ The one target a load variant carries — the same resolution whether the load arrives
    inside a new case or is attached to an existing one."""
    if load["kind"] == "nodal":
        resolve_node(document, load["nodeId"])
    elif load["kind"] == "memberUdl":
        if find(document["elements"], load["elementId"]) is None:
            error(TARGET_MISSING, [load["elementId"]], 'Element "%s" does not exist.' % load["elementId"])
    elif find(document["regions"], load["regionId"]) is None:
        error(TARGET_MISSING, [load["regionId"]], 'Region "%s" does not exist.' % load["regionId"])


def loads_naming(document, variant, key, target):
    return ["%s/%s" % (case["id"], load["id"]) for case in document["loadCases"] for load in case["loads"] if load["kind"] == variant and load[key] == target]


REFERRERS = {{
    "delete-element": ("Element", "member UDL", lambda document, target: loads_naming(document, "memberUdl", "elementId", target)),
    "delete-material": ("Material", "element or region", lambda document, target: [item["id"] for item in document["elements"] if item["materialId"] == target] + [item["id"] for item in document["regions"] if item["materialId"] == target]),
    "delete-section": ("Section", "element", lambda document, target: [item["id"] for item in document["elements"] if item["sectionId"] == target]),
    "delete-region": ("Region", "area load", lambda document, target: loads_naming(document, "area", "regionId", target)),
    "delete-load-case": ("Load case", "combination term", lambda document, target: [item["id"] for item in document["combinations"] if any(term["caseId"] == target for term in item["terms"])]),
    "delete-combination": ("Combination", "combination term", lambda document, target: [item["id"] for item in document["combinations"] if item["id"] != target and any(term["caseId"] == target for term in item["terms"])]),
}}
"""🧷️ The six `delete-` verbs that refuse while referrers exist.

`delete-node` is EXEMPT and the committed vector says so in its own directory name: a plan node is a
drafting coordinate and dropping one an element still names is the SPECIFIED behaviour.
`delete-support` needs no guard at all — no record in this vocabulary names a support id.
"""


def check_referrers(document, kind, target):
    if kind not in REFERRERS:
        return
    label, blocker, lookup = REFERRERS[kind]
    found = lookup(document, target)
    if found:
        error(TARGET_REFERENCED, [target] + found, '%s "%s" is still referenced by %d %s(s): %s.' % (label, target, len(found), blocker, ", ".join(found)))


# endregion 🔖️References


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def case_of(document, identifier):
    """📋️ One load case, or a refusal — a mutation that addressed nothing is never a silent no-op."""
    at = find(document["loadCases"], identifier)
    if at is None:
        error(TARGET_MISSING, [identifier], 'Load case "%s" does not exist.' % identifier)
    return document["loadCases"][at]


def check_record(document, noun, record):
    """🛡️ The payload validation a noun's `create-`/`replace-` twins SHARE. Running the same
    function from both is the whole point: the two used to disagree, and a reader could not tell
    which one was right."""
    if noun == "node":
        check_node(record)
    elif noun == "element":
        resolve_element(document, record)
    elif noun == "material":
        check_material(record)
    elif noun == "section":
        check_section(record)
    elif noun == "support":
        resolve_node(document, record["nodeId"])
    elif noun == "region":
        resolve_material(document, record["materialId"])
        check_region(record)
    elif noun == "load-case":
        for load in record["loads"]:
            resolve_load(document, load)
    elif noun == "combination":
        for term in record["terms"]:
            known = find(document["loadCases"], term["caseId"]) is not None or find(document["combinations"], term["caseId"]) is not None
            if not known:
                error(TARGET_MISSING, [term["caseId"]], 'Load case or combination "%s" does not exist.' % term["caseId"])


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model or raising a [`Refusal`]."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        check_analysis(mutation["settings"])
        if mutation["settings"] == result["analysis"]:
            warn(NO_OP, "Analysis settings are unchanged.")
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        case = case_of(result, mutation["caseId"])
        if kind == "add-load":
            load = copy.deepcopy(mutation["load"])
            resolve_load(result, load)
            if find(case["loads"], load["id"]) is not None:
                warn(NO_OP, 'Load "%s" already exists in case "%s".' % (load["id"], case["id"]))
            case["loads"].append(load)
        elif kind == "remove-load":
            at = find(case["loads"], mutation["loadId"])
            if at is None:
                error(TARGET_MISSING, [mutation["loadId"]], 'Load "%s" does not exist in case "%s".' % (mutation["loadId"], case["id"]))
            case["loads"].pop(at)
        else:
            if case["selfWeight"] == mutation["newSelfWeight"]:
                warn(NO_OP, 'Load case "%s" self-weight is already set.' % case["id"])
            case["selfWeight"] = mutation["newSelfWeight"]
    else:
        noun = noun_of(kind)
        collection, create_argument, replace_argument = COLLECTIONS[noun]
        items = result[collection]
        if kind.startswith("create-"):
            record = copy.deepcopy(mutation[create_argument])
            if find(items, record["id"]) is not None:
                fatal(DUPLICATE_ID, [record["id"]], 'A %s with id "%s" already exists.' % (noun, record["id"]))
            check_record(result, noun, record)
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                error(TARGET_MISSING, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
            check_referrers(result, kind, mutation["id"])
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                error(TARGET_MISSING, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
            record = copy.deepcopy(mutation[replace_argument])
            if record["id"] != mutation["id"]:
                fatal(ID_MISMATCH, [mutation["id"], record["id"]], 'A replace-%s may not rename "%s" to "%s".' % (noun, mutation["id"], record["id"]))
            check_record(result, noun, record)
            if items[at] == record:
                warn(NO_OP, '%s "%s" is already equal to the replacement value.' % (noun.capitalize(), mutation["id"]))
            items[at] = record
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the model it applies to.

    Note what the vocabulary can and cannot express: no `create-` verb carries an index, so the
    inverse of a delete is exact only for a TRAILING record — the feature's rows are chosen
    accordingly and say so.
    """
    kind = kind_of(mutation)
    if kind == "update-analysis-settings":
        return {{"mutation": TAGS[kind], "settings": copy.deepcopy(document["analysis"])}}
    if kind == "add-load":
        return {{"mutation": TAGS["remove-load"], "caseId": mutation["caseId"], "loadId": mutation["load"]["id"]}}
    if kind == "remove-load":
        case = case_of(document, mutation["caseId"])
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("inverse of %s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        return {{"mutation": TAGS["add-load"], "caseId": mutation["caseId"], "load": copy.deepcopy(case["loads"][at])}}
    if kind == "change-load-case-self-weight":
        return {{"mutation": TAGS[kind], "caseId": mutation["caseId"], "newSelfWeight": case_of(document, mutation["caseId"])["selfWeight"]}}
    noun = noun_of(kind)
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    if kind.startswith("create-"):
        return {{"mutation": TAGS["delete-%s" % noun], "id": mutation[create_argument]["id"]}}
    at = find(document[collection], mutation["id"])
    if at is None:
        raise AssertionError("inverse of %s: %r is not in %s" % (kind, mutation["id"], collection))
    held = copy.deepcopy(document[collection][at])
    if kind.startswith("delete-"):
        return {{"mutation": TAGS["create-%s" % noun], create_argument: held}}
    return {{"mutation": TAGS[kind], "id": mutation["id"], replace_argument: held}}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ Every FORWARD row moves the model, so a forward application must move it. A mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass.

    Deliberately NOT applied to the `reject-` rows: their whole claim is that the model does not
    move, and `refused` below is the law that holds them to it instead."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)


def refused(scenario, before, after, declared, raised):
    """🚫️ The law the refusal rows carry in place of `observable`: the document must be exactly
    where it was, and the diagnostic must be the one the committed `🎯️outcome` declares — code,
    level and address. An implementation that refused for the WRONG reason would otherwise pass on
    an unchanged document alone."""
    if before != after:
        raise AssertionError("%s: a refused or no-op mutation must leave the model exactly where it was" % scenario)
    if declared["status"] == "rejected":
        expected, address = declared["code"], declared.get("path", [])
        if raised["level"] not in ("error", "fatal"):
            raise AssertionError("%s: the vector declares a rejection, this implementation raised it at %r" % (scenario, raised["level"]))
    else:
        messages = declared.get("messages", [])
        if len(messages) != 1:
            raise AssertionError("%s: an applied-but-unchanged vector declares exactly one diagnostic, found %r" % (scenario, messages))
        expected, address = messages[0]["code"], messages[0].get("target", [])
    if raised["code"] != expected:
        raise AssertionError("%s: the vector declares %r, this implementation raised %r" % (scenario, expected, raised["code"]))
    if list(address) != raised["target"]:
        raise AssertionError("%s: the vector declares the address %r, this implementation reported %r" % (scenario, list(address), raised["target"]))


def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    comparison cannot make on its own: an implementation that re-derived a sibling collection on
    every edit — renumbering ids, re-sorting sections — would still land on the right value for the
    member it meant to write."""
    if kind == "update-analysis-settings":
        written = "analysis"
    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        written = "loadCases"
    else:
        written = COLLECTIONS[noun_of(kind)][0]
    moved = [name for name in MEMBERS if before[name] != after[name]]
    if moved != [written]:
        raise AssertionError("%s: this verb writes %s and nothing else, but %r moved" % (scenario, written, moved))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the member and index that failed to come back."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored[name] == original[name]:
            continue
        if name == "analysis":
            raise AssertionError("inverse-%s: analysis came back as %r, not %r" % (kind, restored[name], original[name]))
        was = [record["id"] for record in original[name]]
        now = [record["id"] for record in restored[name]]
        if was != now:
            raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, now, was))
        for at, (left, right) in enumerate(zip(original[name], restored[name])):
            if left != right:
                raise AssertionError("inverse-%s: %s[%d] (%s) came back as %s, not %s" % (kind, name, at, left["id"], json.dumps(right, sort_keys=True)[:200], json.dumps(left, sort_keys=True)[:200]))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True)[:300], json.dumps(committed[name], sort_keys=True)[:300]))


# endregion 🔖️Laws


# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the real derived timber portal frame."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        touches_one("mutate-%s" % kind, kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived frame and then its OWN computed inverse.

    The projection carries BOTH models; projecting only the restored one would make every row
    project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({{"mutated": applied, "restored": restored}})

    return handler


def spec_vector_handler(kind, scenario):
    """📐️ Replays one committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("%s: the committed vector carries a %s payload" % (scenario, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        observable(scenario, before, applied)
        touches_one(scenario, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def reject_handler(kind, scenario):
    """🚫️ Replays one committed REFUSAL or no-op vector.

    The projection deliberately carries the diagnostic as well as the model. Two implementations
    that both merely decline to move the document would agree vacuously; making them agree on the
    `code`, the `level` and the `target` is what turns these rows into evidence that they refuse
    for the SAME reason.
    """

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        declared = json_fixture(ctx, "🎯️outcome")
        if kind_of(mutation) != kind:
            raise AssertionError("%s: the committed vector carries a %s payload" % (scenario, kind_of(mutation)))
        if before != after:
            raise AssertionError("%s: a refusal vector's committed after-model must equal its before-model" % scenario)
        try:
            applied = apply_mutation(before, mutation)
            raise AssertionError("%s: this implementation APPLIED a payload the vector declares refused" % scenario)
        except Refusal as raised:
            applied = before
            reported = {{"code": raised.code, "level": raised.level, "target": raised.target}}
        refused(scenario, before, applied, declared, reported)
        return outcome_of({{"model": applied, "refusal": reported}})

    return handler


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison.

    Four families, matching the feature's four `Scenario Outline`s: the two real-model laws, the two
    committed happy paths (`spec-vector-` and `frame-vector-`) and every refusal (`reject-…-<n>`).
    """
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        for prefix in ("spec-vector", "frame-vector"):
            scenario = "%s-%s" % (prefix, kind)
            built = built.oracle(scenario, spec_vector_handler(kind, scenario))
        for index in range(1, REFUSALS[kind] + 1):
            scenario = "reject-%s-%d" % (kind, index)
            built = built.oracle(scenario, reject_handler(kind, scenario))
    return built


# endregion 🔖️Registration
'''
# endregion 🔖️Python


# region 🔖️Rust
RUST_VECTOR = '''fn vector(scenario: &str) -> Vector {{
    match scenario {{
{arms}        other => panic!("{case}: no committed specification vector is registered for scenario {{other:?}}"),
    }}
}}
'''

RUST_ARM = '''        "{scenario}" => Vector {{
            before: include_str!("{prefix}{path}/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("{prefix}{path}/🦠️mutation/🔣️.json"),
            after: include_str!("{prefix}{path}/📸️snapshot/➡️after/🔣️.json"),
            diff: {diff},
            outcome: include_str!("{prefix}{path}/🎯️outcome/🔣️.json"),
        }},
'''

RUST_HANDLERS = '''    /// 📐️ Replays one committed handcrafted specification vector, addressed by SCENARIO id rather
    /// than by kind — every kind now carries several. This is where the evidence the case carried
    /// before the relocation still lives, undiminished: the applied model is held to the committed
    /// after-snapshot, the produced delta to the committed `🔺️diff`, and the diagnostics to the
    /// committed `🎯️outcome`.
    pub fn committed_vector(kind: &'static str, scenario: &str) -> impl Fn(&Context) -> Result<Outcome, String> {
        let scenario = scenario.to_string();
        move |_ctx: &Context| {
            let committed = vector(&scenario);
            let report = report_of(&scenario, committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {
                return Err(format!("{scenario}: the applied model is not the committed after-snapshot — {first}"));
            }
            match committed.diff {
                Some(text) => {
                    if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(text)) {
                        return Err(format!("{scenario}: the produced delta is not the committed 🔺️diff — {first}"));
                    }
                }
                None => return Err(format!("{scenario}: a forward vector must carry a committed 🔺️diff")),
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            touches_one(&scenario, kind, member(&report, "base")?, applied)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// 🚫️ Replays one committed REFUSAL or no-op vector. The projection carries the diagnostic
    /// beside the model on purpose: two implementations that merely both decline to move a document
    /// agree vacuously, and what these rows are evidence for is that they refuse for the SAME
    /// reason — same code, same level, same address.
    pub fn reject(kind: &'static str, scenario: &str) -> impl Fn(&Context) -> Result<Outcome, String> {
        let scenario = scenario.to_string();
        move |_ctx: &Context| {
            let committed = vector(&scenario);
            if committed.diff.is_some() {
                return Err(format!("{scenario}: a refusal vector carries 🔺️diff/🚫️.absent, never a committed delta"));
            }
            let report = report_of(&scenario, committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "base")?) {
                return Err(format!("{scenario}: a refused or no-op mutation must leave the model exactly where it was — {first}"));
            }
            let raised = members(&report, "messages")?;
            let first = raised.first().ok_or_else(|| format!("{scenario}: the vector declares a refusal, the implementation raised nothing"))?;
            declared_outcome_holds(kind, &raised, &canonical(committed.outcome))?;
            let refusal = Json::Object(vec![
                ("code".to_string(), Json::String(first.str("code"))),
                ("level".to_string(), Json::String(level_of(&first.str("level")))),
                ("target".to_string(), Json::Array(strings(first, "target").into_iter().map(Json::String).collect())),
            ]);
            let projection = Json::Object(vec![("model".to_string(), applied.clone()), ("refusal".to_string(), refusal)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }
    //#endregion 🔖️Handlers
'''

RUST_REGISTRATION = '''        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
        for &(scenario, kind) in COMMITTED {
            built = built.subject(scenario, subject::committed_vector(kind, scenario));
        }
        for &(scenario, kind) in REFUSED {
            built = built.subject(scenario, subject::reject(kind, scenario));
        }
        return built;
'''


def rust_tables(built):
    """📇️ The two scenario→kind tables the registration walks, and the `vector` match arms."""
    committed, refused, arms = [], [], []
    for kind in sorted(built):
        for prefix, entry in (("spec-vector", built[kind]["spec"]), ("frame-vector", built[kind]["frame"])):
            scenario = "%s-%s" % (prefix, kind)
            committed.append((scenario, kind))
            arms.append((scenario, entry, True))
        for at, entry in enumerate(built[kind]["rejects"]):
            scenario = "reject-%s-%d" % (kind, at + 1)
            refused.append((scenario, kind))
            arms.append((scenario, entry, False))
    return committed, refused, arms


def render_rust(subset, built):
    """🦀️ Rewrites the subset's SUBJECT adapter in place, by anchored replacement only."""
    case = CASES[subset][0]
    path = os.path.join(SUBSETS, subset, "🧪️tests", case, "🦀️.rs")
    text = open(path, encoding="utf-8").read()
    committed, refused, arms = rust_tables(built)

    text = text.replace("    diff: &'static str,\n", "    diff: Option<&'static str>,\n", 1)

    prefix = "../../🧬️schema/🧬️mutations/"
    rendered = "".join(
        RUST_ARM.format(scenario=scenario, prefix=prefix, path="%s/🧪️tests/%s" % (entry["mutation_directory"], entry["directory"]), diff=('Some(include_str!("%s%s/🧪️tests/%s/🔺️diff/🔣️.json"))' % (prefix, entry["mutation_directory"], entry["directory"])) if forward else "None")
        for scenario, entry, forward in arms
    )
    anchor = next(needle for needle in ("fn vector(kind: &str) -> Vector {", "fn vector(scenario: &str) -> Vector {") if needle in text)
    start = text.index(anchor)
    end = text.index("\n}\n", start) + len("\n}\n")
    text = text[:start] + RUST_VECTOR.format(arms=rendered, case=case) + text[end:]

    start = text.index("    /// 📐️ Replays one committed handcrafted specification vector")
    end = text.index("    //#endregion 🔖️Handlers", start) + len("    //#endregion 🔖️Handlers\n")
    text = text[:start] + RUST_HANDLERS + text[end:]

    old = """        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built;
"""
    if old in text:
        text = text.replace(old, RUST_REGISTRATION, 1)

    tables = "//#region 🔖️Scenarios\n"
    tables += "/// 📇️ Every FORWARD committed vector this subset owns, as `(scenario id, kind)`. Two per kind:\n"
    tables += "/// the pre-existing `spec-vector-<kind>` and the steel-frame `frame-vector-<kind>`.\n"
    tables += "const COMMITTED: &[(&str, &str)] = &[\n" + "".join('    ("%s", "%s"),\n' % row for row in committed) + "];\n\n"
    tables += "/// 📇️ Every REFUSAL or no-op vector this subset owns, numbered in the catalog's own order.\n"
    tables += "const REFUSED: &[(&str, &str)] = &[\n" + "".join('    ("%s", "%s"),\n' % row for row in refused) + "];\n"
    tables += "//#endregion 🔖️Scenarios\n\n"
    text = re.sub(r"//#region 🔖️Scenarios.*?//#endregion 🔖️Scenarios\n\n", "", text, flags=re.S)
    marker = "//#region 🔖️Fixtures\n"
    text = text.replace(marker, tables + marker, 1)

    text = text.replace("let _ = (KINDS, UNOBSERVABLE, vector as fn(&str) -> Vector);", "let _ = (KINDS, UNOBSERVABLE, COMMITTED, REFUSED, vector as fn(&str) -> Vector);", 1)
    return path, text


# endregion 🔖️Rust


# region 🔖️Emit
def render_python(subset, built):
    kinds = [kind for kind in sorted(built)]
    order = [vector["mutationId"] for declaration in read(os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json"))["mutationCatalogs"] for vector in declaration["vectors"]]
    kinds = [kind for kind in order if kind in built]
    tuple_text = "(" + ", ".join('"%s"' % kind for kind in kinds) + ("," if len(kinds) == 1 else "") + ")"
    prose = ", ".join("`%s`" % kind for kind in kinds)
    refusals = "{" + ", ".join('"%s": %d' % (kind, len(built[kind]["rejects"])) for kind in kinds) + "}"
    return PYTHON.format(kinds_tuple=tuple_text, kinds_prose=prose, refusals=refusals)


def write(path, text, check):
    existing = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    if existing == text:
        return "same"
    if check:
        return "differs"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "written"


def main():
    check = "--check" in sys.argv[1:]
    tally = {}

    def record(state):
        tally[state] = tally.get(state, 0) + 1

    for subset, (case, duplicate, _name) in sorted(CASES.items()):
        built = plan(subset)
        path, text = render_feature(subset, built)
        record(write(path, text, check))
        python = render_python(subset, built)
        record(write(os.path.join(SUBSETS, subset, "🧪️tests", case, "🐍️.py"), python, check))
        record(write(os.path.join(SUBSETS, "🌐️any", "🧪️tests", duplicate, "🐍️.py"), python, check))
        path, text = render_rust(subset, built)
        record(write(path, text, check))
    print("features + references + subjects: %s" % ", ".join("%s %d" % item for item in sorted(tally.items())))
    return 1 if check and tally.get("differs") else 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Emit
