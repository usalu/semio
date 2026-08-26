#!/usr/bin/env python3
"""🏛️ An INDEPENDENT second implementation of the `s.architect.program` architectural-brief document
and its 266 typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `program` document is an
architectural BRIEF: 66 id-keyed registers holding stakeholders, users, activities, functions,
elements, requirements of sixteen families, records, constraints, two edge registers and three
document-level facets. No interchange format models that — IFC carries a built model rather than the
brief that preceded it, and neither it nor any requirements-management schema has a notion of this
document's closed 266-verb algebra — and none of them reads `.dsl.semio`. What a reference genuinely
can adjudicate is that algebra, and the argument that it cannot is refuted inside this repository:
the fifteen `📕️norm` Python references were written from the same two derivation documents this
case's rationale used to call its own vocabulary un-adjudicable.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json`` — the 266 payload
  shapes: 66 `{id}` payloads, 64 `{id,newName}` payloads, 66 whole-record payloads, and the six
  facet payloads `newTitle`/`newMeta`, `newCode`/`newProject`, `newFramework`/`newGovernance`.
* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the register list and
  its order.
* `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`
  rules 1, 2 and 4 — which verbs a document-level facet, an id-keyed register and an edge register
  each yield.
* the 266 committed `(before, mutation, after, outcome)` specification vectors, which are the only
  statement of three things: that a `connect-` verb NORMALISES the edge it appends (the
  `connects-reception-to-waiting` vector's `normalized` flips false to true while the payload's own
  value is false), that a `delete`/`rename`/`replace` against an absent id is
  `mutation.target-missing` with the id as its path rather than a no-op, and that the snapshot's
  47th register is serialized `artifacts` even though the committed JSON Schema requires `documents`.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only, and no file under `🧬️mutations/<slug>/{🦠️mutation,↩️inverse,🔺️diff}/🦀️component.rs` was read.
The verb table below is derived from the schemas and the vectors, and was checked by replaying all
266 committed vectors offline before the oracle was registered.

**Two registers and one scenario this implementation REFUSES, by clause rather than by absence.**

* `create-knowledge-record` and `create-benchmark-record`. `knowledge` and `benchmarks` are the only
  two of the 66 registers that a committed snapshot carries as a COMPOSED CHILD HANDLE
  (`{"childId": "architect-knowledge-7904dd65836c8ff4", "target": {…}}`) rather than as an array of
  records. Their `create` vectors are the only two in the case whose whole observable effect is that
  `childId` changing — to `architect-knowledge-b3743ce016d5422b` and
  `architect-benchmarks-ebb8ef7bad26edae`. That value is a content address of the child
  `s.stdio.semio@v1/table` document AFTER the row is appended, and no document in this repository
  states the addressing function, the child table's canonical encoding, or where the child's existing
  rows come from. This implementation refuses to guess rather than to hard-code the committed answer.
  The other six kinds over those two registers are NOT refused: a `delete`/`rename`/`replace` against
  a child handle finds no row, and `mutation.target-missing` is stateable.
* `identity-round-trip`. See `refuse_carrier` in the Handlers region.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
KINDS = (
    "create-information-requirement",
    "delete-information-requirement",
    "rename-information-requirement",
    "replace-information-requirement",
    "create-sustainability-requirement",
    "delete-sustainability-requirement",
    "rename-sustainability-requirement",
    "replace-sustainability-requirement",
    "create-accessibility-requirement",
    "delete-accessibility-requirement",
    "rename-accessibility-requirement",
    "replace-accessibility-requirement",
    "create-conflict",
    "delete-conflict",
    "rename-conflict",
    "replace-conflict",
    "create-option-evaluation",
    "delete-option-evaluation",
    "rename-option-evaluation",
    "replace-option-evaluation",
    "create-function",
    "delete-function",
    "rename-function",
    "replace-function",
    "create-risk",
    "delete-risk",
    "rename-risk",
    "replace-risk",
    "create-decision",
    "delete-decision",
    "rename-decision",
    "replace-decision",
    "create-validation-record",
    "delete-validation-record",
    "rename-validation-record",
    "replace-validation-record",
    "create-priority-record",
    "delete-priority-record",
    "rename-priority-record",
    "replace-priority-record",
    "create-flow-requirement",
    "delete-flow-requirement",
    "rename-flow-requirement",
    "replace-flow-requirement",
    "create-environmental-requirement",
    "delete-environmental-requirement",
    "rename-environmental-requirement",
    "replace-environmental-requirement",
    "create-workshop",
    "delete-workshop",
    "rename-workshop",
    "replace-workshop",
    "create-scenario",
    "delete-scenario",
    "rename-scenario",
    "replace-scenario",
    "create-benchmark-record",
    "delete-benchmark-record",
    "rename-benchmark-record",
    "replace-benchmark-record",
    "create-activity",
    "delete-activity",
    "rename-activity",
    "replace-activity",
    "create-infrastructure-requirement",
    "delete-infrastructure-requirement",
    "rename-infrastructure-requirement",
    "replace-infrastructure-requirement",
    "create-organizational-requirement",
    "delete-organizational-requirement",
    "rename-organizational-requirement",
    "replace-organizational-requirement",
    "create-issue",
    "delete-issue",
    "rename-issue",
    "replace-issue",
    "create-approval-record",
    "delete-approval-record",
    "rename-approval-record",
    "replace-approval-record",
    "create-stakeholder",
    "delete-stakeholder",
    "rename-stakeholder",
    "replace-stakeholder",
    "create-quality-record",
    "delete-quality-record",
    "rename-quality-record",
    "replace-quality-record",
    "create-resilience-requirement",
    "delete-resilience-requirement",
    "rename-resilience-requirement",
    "replace-resilience-requirement",
    "create-assumption",
    "delete-assumption",
    "rename-assumption",
    "replace-assumption",
    "create-cost-requirement",
    "delete-cost-requirement",
    "rename-cost-requirement",
    "replace-cost-requirement",
    "create-document",
    "delete-document",
    "rename-document",
    "replace-document",
    "create-schedule-requirement",
    "delete-schedule-requirement",
    "rename-schedule-requirement",
    "replace-schedule-requirement",
    "create-growth-plan",
    "delete-growth-plan",
    "rename-growth-plan",
    "replace-growth-plan",
    "create-performance-criterion",
    "delete-performance-criterion",
    "rename-performance-criterion",
    "replace-performance-criterion",
    "create-operational-requirement",
    "delete-operational-requirement",
    "rename-operational-requirement",
    "replace-operational-requirement",
    "create-requirement",
    "delete-requirement",
    "rename-requirement",
    "replace-requirement",
    "create-site-context",
    "delete-site-context",
    "rename-site-context",
    "replace-site-context",
    "create-template-record",
    "delete-template-record",
    "rename-template-record",
    "replace-template-record",
    "create-report-record",
    "delete-report-record",
    "rename-report-record",
    "replace-report-record",
    "create-audit-event",
    "delete-audit-event",
    "rename-audit-event",
    "replace-audit-event",
    "create-knowledge-record",
    "delete-knowledge-record",
    "rename-knowledge-record",
    "replace-knowledge-record",
    "create-regulatory-requirement",
    "delete-regulatory-requirement",
    "rename-regulatory-requirement",
    "replace-regulatory-requirement",
    "create-change-record",
    "delete-change-record",
    "rename-change-record",
    "replace-change-record",
    "create-communication-requirement",
    "delete-communication-requirement",
    "rename-communication-requirement",
    "replace-communication-requirement",
    "create-resource",
    "delete-resource",
    "rename-resource",
    "replace-resource",
    "create-status-record",
    "delete-status-record",
    "rename-status-record",
    "replace-status-record",
    "create-process",
    "delete-process",
    "rename-process",
    "replace-process",
    "create-search-filter",
    "delete-search-filter",
    "rename-search-filter",
    "replace-search-filter",
    "create-access-rule",
    "delete-access-rule",
    "rename-access-rule",
    "replace-access-rule",
    "create-privacy-requirement",
    "delete-privacy-requirement",
    "rename-privacy-requirement",
    "replace-privacy-requirement",
    "create-relationship",
    "delete-relationship",
    "rename-relationship",
    "replace-relationship",
    "create-quantity-requirement",
    "delete-quantity-requirement",
    "rename-quantity-requirement",
    "replace-quantity-requirement",
    "create-analysis-record",
    "delete-analysis-record",
    "rename-analysis-record",
    "replace-analysis-record",
    "create-storage-requirement",
    "delete-storage-requirement",
    "rename-storage-requirement",
    "replace-storage-requirement",
    "create-meeting-record",
    "delete-meeting-record",
    "rename-meeting-record",
    "replace-meeting-record",
    "create-survey",
    "delete-survey",
    "rename-survey",
    "replace-survey",
    "create-delivery-constraint",
    "delete-delivery-constraint",
    "rename-delivery-constraint",
    "replace-delivery-constraint",
    "create-constraint-record",
    "delete-constraint-record",
    "rename-constraint-record",
    "replace-constraint-record",
    "create-compliance-record",
    "delete-compliance-record",
    "rename-compliance-record",
    "replace-compliance-record",
    "create-service-requirement",
    "delete-service-requirement",
    "rename-service-requirement",
    "replace-service-requirement",
    "create-equipment",
    "delete-equipment",
    "rename-equipment",
    "replace-equipment",
    "create-security-requirement",
    "delete-security-requirement",
    "rename-security-requirement",
    "replace-security-requirement",
    "create-collaboration-record",
    "delete-collaboration-record",
    "rename-collaboration-record",
    "replace-collaboration-record",
    "create-safety-requirement",
    "delete-safety-requirement",
    "rename-safety-requirement",
    "replace-safety-requirement",
    "create-user-profile",
    "delete-user-profile",
    "rename-user-profile",
    "replace-user-profile",
    "create-human-factor-requirement",
    "delete-human-factor-requirement",
    "rename-human-factor-requirement",
    "replace-human-factor-requirement",
    "create-flexibility-requirement",
    "delete-flexibility-requirement",
    "rename-flexibility-requirement",
    "replace-flexibility-requirement",
    "create-wayfinding-requirement",
    "delete-wayfinding-requirement",
    "rename-wayfinding-requirement",
    "replace-wayfinding-requirement",
    "create-program-element",
    "delete-program-element",
    "rename-program-element",
    "replace-program-element",
    "connect-adjacency",
    "disconnect-adjacency",
    "connect-trace",
    "disconnect-trace",
    "rename-meta",
    "replace-meta",
    "rename-project",
    "replace-project",
    "rename-governance",
    "replace-governance",
)
"""🏷️ Every kind the catalog declares, in its declared order — 266 of them: create/delete/rename/
replace over 64 id-keyed registers, connect/disconnect over the two edge registers, and rename/
replace over the three document-level facets."""

REGISTER_OF = {
    "information-requirement": "information",
    "sustainability-requirement": "sustainability",
    "accessibility-requirement": "accessibility",
    "conflict": "conflicts",
    "option-evaluation": "options",
    "function": "functions",
    "risk": "risks",
    "decision": "decisions",
    "validation-record": "validations",
    "priority-record": "priorities",
    "flow-requirement": "flows",
    "environmental-requirement": "environmental",
    "workshop": "workshops",
    "scenario": "scenarios",
    "benchmark-record": "benchmarks",
    "activity": "activities",
    "infrastructure-requirement": "infrastructure",
    "organizational-requirement": "organizational",
    "issue": "issues",
    "approval-record": "approvals",
    "stakeholder": "stakeholders",
    "quality-record": "quality",
    "resilience-requirement": "resilience",
    "assumption": "assumptions",
    "cost-requirement": "costs",
    "document": "artifacts",
    "schedule-requirement": "schedules",
    "growth-plan": "growth",
    "performance-criterion": "performance",
    "operational-requirement": "operations",
    "requirement": "requirements",
    "site-context": "siteContext",
    "template-record": "templates",
    "report-record": "reports",
    "audit-event": "auditEvents",
    "knowledge-record": "knowledge",
    "regulatory-requirement": "regulatory",
    "change-record": "changes",
    "communication-requirement": "communication",
    "resource": "resources",
    "status-record": "statusRecords",
    "process": "processes",
    "search-filter": "searchFilters",
    "access-rule": "accessRules",
    "privacy-requirement": "privacy",
    "relationship": "relationships",
    "quantity-requirement": "quantities",
    "analysis-record": "analyses",
    "storage-requirement": "storage",
    "meeting-record": "meetings",
    "survey": "surveys",
    "delivery-constraint": "delivery",
    "constraint-record": "constraints",
    "compliance-record": "complianceRecords",
    "service-requirement": "services",
    "equipment": "equipment",
    "security-requirement": "security",
    "collaboration-record": "collaboration",
    "safety-requirement": "safety",
    "user-profile": "users",
    "human-factor-requirement": "humanFactors",
    "flexibility-requirement": "flexibility",
    "wayfinding-requirement": "wayfinding",
    "program-element": "elements",
    "adjacency": "adjacencies",
    "trace": "traces",
    "meta": "meta",
    "project": "project",
    "governance": "governance",
}
"""🗄️ Which member of the snapshot each noun addresses. The 66 array registers are named by
`📸️snapshot/🔣️component.json`; the pairing with a verb's noun is the derivation `📓️derivation-rules.md`
rule 2 describes and is spelled out here because it is not always the noun's own plural —
`user-profile` lives in `users`, `program-element` in `elements`, `document` in `artifacts`."""

FACETS = {"meta": ("title", "newTitle", "newMeta"), "project": ("code", "newCode", "newProject"), "governance": ("framework", "newFramework", "newGovernance")}
"""📄️ The three document-level facets of `📓️derivation-rules.md` rule 1: which scalar their `rename`
writes, what its argument is called, and what their `replace` payload is called."""

EDGES = ("adjacency", "trace")
"""🧲 The two edge registers of rule 4 — `connect`/`disconnect` instead of `create`/`delete`."""

CHILD_HANDLE_REGISTERS = ("knowledge", "benchmarks")
"""🧷 The two registers a committed snapshot carries as a composed child handle rather than as an
array. Every id is absent from them, which is why their `delete`/`rename`/`replace` vectors pin
`mutation.target-missing`, and their `create` is what this implementation refuses."""

TARGET_MISSING = "mutation.target-missing"
"""🚨️ The one diagnostic code the committed outcome vectors raise."""


def camel(noun):
    """🔤️ lowerCamelCase of a hyphenated noun — the whole-record payload key of a `create`,
    `replace` or `connect`, exactly as `🧬️mutations/🔣️component.json` names it."""
    head, *rest = noun.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def parts(kind):
    """✂️ A kind split into its verb and its noun."""
    verb, *rest = kind.split("-")
    return verb, "-".join(rest)


def tag_of(kind):
    """🔖️ The `mutation` discriminator a committed payload carries — lowerCamelCase of the kind."""
    return camel(kind)


TAGS = {kind: tag_of(kind) for kind in KINDS}
KIND_OF_TAG = {tag: kind for kind, tag in TAGS.items()}

REGISTERS = tuple(dict.fromkeys(REGISTER_OF[noun] for noun in REGISTER_OF))
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds a snapshot to the shape every committed vector agrees on: the three scalar facets,
    the 66 registers, and `artifacts` rather than the `documents` the JSON Schema requires."""
    for member in ("schema", "meta", "project", "governance"):
        if member not in document:
            raise AssertionError("%s: a program snapshot must carry %r" % (where, member))
    for register in REGISTERS:
        if register not in document:
            raise AssertionError("%s: a program snapshot must carry the %r register" % (where, register))
    for register in CHILD_HANDLE_REGISTERS:
        held = document[register]
        if isinstance(held, dict) and set(held) != {"childId", "target"}:
            raise AssertionError("%s: the composed %r child handle must carry exactly childId and target, found %r" % (where, register, sorted(held)))
    for register in REGISTERS:
        rows = document[register]
        if not isinstance(rows, list):
            continue
        seen = set()
        for row in rows:
            identity = row.get("id")
            if identity is None:
                raise AssertionError("%s: a %s row carries no id" % (where, register))
            if identity in seen:
                raise AssertionError("%s: %s carries %r twice" % (where, register, identity))
            seen.add(identity)


def index_of(rows, identity):
    """🔎️ Where a row with this id sits, or `None`."""
    for at, row in enumerate(rows):
        if row.get("id") == identity:
            return at
    return None


def addressed_id(kind, payload):
    """🎯️ Which id a kind addresses: its own `id` argument, or the id of the record it carries."""
    verb, noun = parts(kind)
    if verb in ("delete", "disconnect", "rename"):
        return payload["id"]
    return payload[camel(noun)]["id"]
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind, answering the new document and the diagnostic codes it raised.

    Five families, one per rule of `📓️derivation-rules.md`: a facet `rename` writes one scalar and a
    facet `replace` swaps the whole facet (rule 1); `create` appends, `delete` removes, `rename`
    writes `name` and `replace` swaps the record IN PLACE, all addressed by id (rule 2); `connect`
    appends a NORMALISED edge and `disconnect` removes it (rule 4). Anything addressed by an id the
    document does not hold is `mutation.target-missing`, and the document is returned untouched.
    """
    verb, noun = parts(kind)
    register = REGISTER_OF[noun]
    document = copy.deepcopy(document)
    if noun in FACETS:
        scalar, rename_argument, replace_argument = FACETS[noun]
        if verb == "rename":
            document[register][scalar] = payload[rename_argument]
        else:
            document[register] = copy.deepcopy(payload[replace_argument])
        return document, []
    rows = document[register]
    if not isinstance(rows, list):
        if verb in ("create", "connect"):
            raise AssertionError(unstateable(kind, register))
        return document, [TARGET_MISSING]
    if verb in ("create", "connect"):
        record = copy.deepcopy(payload[camel(noun)])
        if verb == "connect":
            record = normalised(record)
        rows.append(record)
        return document, []
    at = index_of(rows, addressed_id(kind, payload))
    if at is None:
        return document, [TARGET_MISSING]
    if verb in ("delete", "disconnect"):
        rows.pop(at)
    elif verb == "rename":
        rows[at]["name"] = payload["newName"]
    else:
        rows[at] = copy.deepcopy(payload[camel(noun)])
    return document, []


def normalised(record):
    """🧲 A `connect` verb normalises the edge it appends. The `connects-reception-to-waiting` vector
    is the only statement of it: its payload carries `normalized` false and its after-snapshot
    carries true, with nothing else changed. An edge record that declares no `normalized` member —
    `trace` — is appended as given."""
    if "normalized" not in record:
        return record
    record = copy.deepcopy(record)
    record["normalized"] = True
    return record


def unstateable(kind, register):
    """🚧️ The refusal for the two `create` verbs over a composed child handle."""
    return (
        "%s: this implementation refuses this kind rather than guessing it. The %r register is carried as a COMPOSED CHILD HANDLE "
        "({childId, target}), not as an array, so this verb's whole observable effect is the committed after-snapshot's new `childId` — a "
        "content address of the child `s.stdio.semio@v1/table` document AFTER the row is appended. No document in this repository states the "
        "addressing function, the child table's canonical encoding, or where the child's existing rows come from, so a second implementation "
        "cannot reproduce it. The remedy is to publish the child-addressing rule, not to relax the comparison." % (kind, register)
    )


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. A mutation that addressed an absent id has no inverse, because it moved
    nothing. `create` inverts to `delete` and `connect` to `disconnect` — which is exact only for a
    TRAILING record, since no `create`/`connect` verb in this vocabulary carries an index. That is a
    property of the closed schema, not of an implementation."""
    verb, noun = parts(kind)
    register = REGISTER_OF[noun]
    if noun in FACETS:
        scalar, rename_argument, replace_argument = FACETS[noun]
        if verb == "rename":
            return [(kind, {rename_argument: document[register][scalar]})]
        return [(kind, {replace_argument: copy.deepcopy(document[register])})]
    rows = document[register]
    if not isinstance(rows, list):
        return []
    if verb in ("create", "connect"):
        return [(("delete-" if verb == "create" else "disconnect-") + noun, {"id": addressed_id(kind, payload)})]
    record = next((row for row in rows if row.get("id") == addressed_id(kind, payload)), None)
    if record is None:
        return []
    if verb in ("delete", "disconnect"):
        return [(("create-" if verb == "delete" else "connect-") + noun, {camel(noun): copy.deepcopy(record)})]
    if verb == "rename":
        return [(kind, {"id": record["id"], "newName": record["name"]})]
    return [(kind, {camel(noun): copy.deepcopy(record)})]
# endregion 🔖️Verbs


# region 🔖️Laws
def declared_codes(outcome):
    """🚨️ The `mutation.*` codes a committed `🎯️outcome` vector declares — a `messages` array when
    it applied with diagnostics, the single `code` that refused it when it did not."""
    listed = [message.get("code") for message in outcome.get("messages", []) if message.get("code")]
    if listed:
        return listed
    single = outcome.get("code")
    return [single] if single else []


def raises_declared(kind, raised, outcome):
    """⚖️ The committed outcome claim, asserted in role."""
    wanted = declared_codes(outcome)
    if raised != wanted:
        raise AssertionError("mutate-%s: raised %r, the committed 🎯️outcome vector declares %r" % (kind, raised, wanted))
    status = outcome.get("status")
    if status == "rejected" and not raised:
        raise AssertionError("mutate-%s: the committed vector declares a refusal, but this implementation applied it" % kind)
    if status == "applied" and raised:
        raise AssertionError("mutate-%s: the committed vector declares it applied, but this implementation raised %r" % (kind, raised))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if produced.get(member) != committed.get(member):
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:400], json.dumps(committed.get(member), sort_keys=True)[:400]))


def observable(kind, before, after, exempt):
    """👁️ A mutation the committed vector declares APPLIED must move the compared projection. The
    exemptions are named, not inferred: they are exactly the six kinds whose vector pins a refusal."""
    if kind in exempt:
        return
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def touches_one(kind, before, after):
    """🎯️ Every kind writes exactly ONE member of the document — the check an after-snapshot
    comparison cannot make on its own, and the one that would catch a verb that silently reordered a
    second register."""
    moved = [member for member in sorted(set(before) | set(after)) if before.get(member) != after.get(member)]
    if len(moved) > 1:
        raise AssertionError("mutate-%s: moved %r; every kind in this vocabulary writes exactly one member" % (kind, moved))
    expected = REGISTER_OF[parts(kind)[1]]
    if moved and moved[0] != expected:
        raise AssertionError("mutate-%s: moved %r, but this kind addresses %r" % (kind, moved[0], expected))


def restores(kind, restored, original):
    """↩️ The inverse law, asserted in role: field for field, index for index."""
    for member in sorted(set(restored) | set(original)):
        if restored.get(member) != original.get(member):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:400], json.dumps(original.get(member), sort_keys=True)[:400]))
# endregion 🔖️Laws


# region 🔖️Plan
GUARD_VECTORS = ("delete-benchmark-record", "rename-benchmark-record", "replace-benchmark-record", "delete-knowledge-record", "rename-knowledge-record", "replace-knowledge-record")
"""👁️ The six kinds whose committed vector pins a REFUSAL rather than an effect, so before and after
are the same document and the observability law cannot hold. They are exactly the delete/rename/
replace verbs over the two composed-child-handle registers."""


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


def addressed(ctx, kind):
    """🧭️ The kind this row claims, checked against the doc string the feature's Examples table
    filled in, so a mis-wired registration is an error rather than a silent pass."""
    spec = json.loads(doc_string(ctx))
    if spec.get("kind") != kind:
        raise AssertionError("%s: the feature's doc string states %r" % (ctx.scenario["id"], spec.get("kind")))
    return spec


def payload_of(ctx, kind):
    """🦠️ The committed mutation payload, checked to carry this kind's own discriminator."""
    payload = json_fixture(ctx, "🦠️mutation")
    if payload.get("mutation") != TAGS[kind]:
        raise AssertionError("%s: the committed vector carries a %r payload, not %r" % (ctx.scenario["id"], payload.get("mutation"), TAGS[kind]))
    return {key: value for key, value in payload.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the committed diagnostic codes, observability and the single-member footprint."""

    def handler(ctx):
        addressed(ctx, kind)
        before = json_fixture(ctx, "⬅️before")
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        validate(before, "mutate-%s" % kind)
        applied, raised = apply_mutation(before, kind, payload_of(ctx, kind))
        raises_declared(kind, raised, outcome)
        equals_committed(kind, applied, after)
        validate(applied, "mutate-%s" % kind)
        observable(kind, before, applied, GUARD_VECTORS)
        touches_one(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to its committed before-snapshot and then its OWN computed inverse, and
    requires the committed before-snapshot back."""

    def handler(ctx):
        addressed(ctx, kind)
        before = json_fixture(ctx, "⬅️before")
        payload = payload_of(ctx, kind)
        validate(before, "inverse-%s" % kind)
        current, _raised = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current, _step_raised = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def refuse_carrier(ctx):
    """🚧️ `identity-round-trip` reads this subset's own `.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence.

    1. The committed grammar `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
       is the repository-wide placeholder: its whole body is `payload = OCTET+`, and its `header`
       production declares `"schema" SP "stdio.json"` — which the committed artifact contradicts on
       its first line with `semio architect.program.dsl v1`.
    2. The notation FLATTENS nested records into `key=key=value`. A row header reads
       `ownership=consultant-ids=[ ] participant-ids=[ ] tags=[ ] notes=[ ]`, and only a field table
       can say that `consultant-ids` and `participant-ids` belong to `ownership` while `tags` does
       not. The committed JSON Schema is not that table: all seventy record `$defs` in
       `🧬️schema/📸️snapshot/🔣️component.json` are `{"type": "object", "additionalProperties": true}`
       with no `properties` at all, and 133 of the mutation payload objects likewise.
    3. That same row header writes `tags` and `notes`, which no committed snapshot vector carries on
       any record of any register, so no document states whether they belong to the compared
       projection.

    The 532 mutation scenarios of this case ARE adjudicated, by the verb table above and the 266
    committed vectors. This one is not, and what is missing is a real grammar for this carrier — the
    same finding `mutate-iso16757-1` and `mutate-vdi3805-1` report against their own subsets.
    """
    raise AssertionError(
        "identity-round-trip: this subset's `.dsl.semio` carrier cannot be read by a second implementation. Its committed grammar is the "
        "repository-wide placeholder `payload = OCTET+` whose header production declares `\"schema\" SP \"stdio.json\"`, contradicted by the "
        "artifact's own first line `semio architect.program.dsl v1`; the notation flattens nested records into `key=key=value` "
        "(`ownership=consultant-ids=[ ] participant-ids=[ ] tags=[ ] notes=[ ]`) with no committed field table to bound them, because all "
        "seventy record `$defs` of `🧬️schema/📸️snapshot/🔣️component.json` are `{\"type\": \"object\", \"additionalProperties\": true}` with no "
        "`properties`; and it writes `tags` and `notes` members that no committed snapshot vector carries on any record. Publishing a real "
        "grammar for this carrier closes it. Read %d bytes of the committed artifact and refused to guess their meaning."
        % len(ctx.fixture_bytes(uri_in(ctx, "🗣️example.dsl.semio")))
    )
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", refuse_carrier)
# endregion 🔖️Registration
