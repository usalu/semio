import json, sys

SUBSETS = {
    "identity": {
        "entities": ["meta", "project"],
        "name": "Document identity and project definition — the two singleton facets every program document carries: schema-level metadata (ProgramMeta) and the client/site/budget project header (ProjectDefinition). Renamed/replaced only; never created or deleted, since a program document always has exactly one of each.",
    },
    "participants": {
        "entities": ["stakeholder", "user-profile"],
        "name": "People named by the brief — stakeholders (client-side roles with interest/influence) and user profiles (occupant/operator archetypes referenced by activities and accessibility requirements).",
    },
    "brief": {
        "entities": ["activity", "function", "program-element", "quantity-requirement"],
        "name": "The core programmatic content — what the building must accommodate: activities, functional uses, the program elements (rooms/spaces/zones) that house them, and the quantity requirements (areas, counts, ratios) that size them.",
    },
    "relations": {
        "entities": ["relationship", "adjacency", "trace"],
        "name": "Cross-references between program entities — generic typed relationships, spatial adjacency edges between elements (connect/disconnect only, normalized pairs), and traceability links (connect/disconnect only) from any record back to the requirement or decision that justifies it.",
    },
    "operations": {
        "entities": ["process", "flow-requirement", "access-rule", "operational-requirement"],
        "name": "How the building is operated day to day — workflows/processes, circulation and material/people flow requirements, access-control rules, and general operational requirements.",
    },
    "resources": {
        "entities": ["equipment", "resource", "storage-requirement"],
        "name": "Physical assets and stock the brief provisions for — equipment items, general resources, and storage requirements.",
    },
    "compliance": {
        "entities": [
            "environmental-requirement",
            "human-factor-requirement",
            "accessibility-requirement",
            "privacy-requirement",
            "safety-requirement",
            "security-requirement",
            "regulatory-requirement",
        ],
        "name": "Human-centered and regulated constraints on the design — environmental/comfort parameters, human-factors and accessibility (clear widths, WCAG conformance), privacy, life-safety, security, and cited regulatory clauses. Verified with genuinely distinct fields per kind (e.g. AccessibilityRequirement.clear_width_m/wcag_conformance vs EnvironmentalRequirement.parameter_kind/comfort_band), not a shared shape.",
    },
    "context": {
        "entities": [
            "site-context",
            "organizational-requirement",
            "service-requirement",
            "infrastructure-requirement",
            "information-requirement",
            "communication-requirement",
            "wayfinding-requirement",
        ],
        "name": "The surrounding context the design must fit — site conditions, the client organization's own structure, building services, infrastructure capacity, information/IT, communication, and wayfinding/signage requirements.",
    },
    "lifecycle": {
        "entities": [
            "schedule-requirement",
            "flexibility-requirement",
            "growth-plan",
            "sustainability-requirement",
            "resilience-requirement",
            "cost-requirement",
            "delivery-constraint",
        ],
        "name": "How the program evolves and is delivered over time — project schedule requirements, flexibility/adaptability, growth plans, sustainability and resilience targets, cost requirements, and delivery constraints.",
    },
    "risk": {
        "entities": ["risk", "conflict", "requirement", "priority-record"],
        "name": "The project risk register — risks, stakeholder/requirement conflicts, the generic (untyped) requirement record, and priority rankings over them.",
    },
    "decisions": {
        "entities": ["scenario", "option-evaluation", "decision", "validation-record"],
        "name": "The decision trail — scenarios explored, options evaluated against criteria, decisions made, and validation records confirming a decision or requirement was checked.",
    },
    "evaluation": {
        "entities": ["performance-criterion", "quality-record"],
        "name": "How outcomes are measured — performance criteria and quality records.",
    },
    "records": {
        "entities": [
            "document",
            "assumption",
            "constraint-record",
            "compliance-record",
            "approval-record",
            "meeting-record",
            "change-record",
            "collaboration-record",
            "analysis-record",
            "report-record",
        ],
        "name": "The project's paper trail — referenced documents, assumptions, constraints, compliance/approval records, meeting minutes, change records, collaboration notes, analyses, and reports.",
    },
    "utility": {
        "entities": ["search-filter", "status-record"],
        "name": "Editor-facing utility records — saved search filters and status records — not part of the architectural content itself.",
    },
    "engagement": {
        "entities": ["workshop", "survey", "issue", "audit-event"],
        "name": "Stakeholder engagement and tracking — workshops, surveys, issues, and audit events logging who changed what.",
    },
    "knowledge": {
        "entities": ["template-record", "knowledge-record", "benchmark-record"],
        "name": "Reference material reused across projects — templates, knowledge-base entries, and benchmark records.",
    },
    "governance": {
        "entities": ["governance"],
        "name": "The document's own governance block (approval authority, review cadence) — a singleton, renamed/replaced only.",
    },
}

entity_to_subset = {}
for subset, info in SUBSETS.items():
    for e in info["entities"]:
        assert e not in entity_to_subset, f"duplicate entity {e}"
        entity_to_subset[e] = subset

FULL_CRUD_VERBS = ["create", "rename", "replace", "delete"]
EDGE_VERBS = ["connect", "disconnect"]
SINGLETON_VERBS = ["rename", "replace"]

EDGE_ENTITIES = {"adjacency", "trace"}
SINGLETON_ENTITIES = {"meta", "project", "governance"}

def mutation_id_to_entity(mutation_id: str) -> str:
    for verb in FULL_CRUD_VERBS + EDGE_VERBS:
        prefix = verb + "-"
        if mutation_id.startswith(prefix):
            return mutation_id[len(prefix):]
    raise ValueError(mutation_id)

if __name__ == "__main__":
    p = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
    d = json.load(open(p))
    mm = d["mutationManifests"][0]
    muts = mm["mutations"]
    assert len(muts) == 266, len(muts)
    missing = []
    counts = {}
    for m in muts:
        entity = mutation_id_to_entity(m["id"])
        subset = entity_to_subset.get(entity)
        if subset is None:
            missing.append((m["id"], entity))
            continue
        m["subset"] = subset
        counts[subset] = counts.get(subset, 0) + 1
    if missing:
        print("MISSING MAPPINGS:", missing)
        sys.exit(1)
    print("counts:", counts)
    print("total:", sum(counts.values()))
    json.dump(d, open(p, "w"), indent=2, ensure_ascii=False)
    open(p, "a").write("\n")
    print("wrote", p)
