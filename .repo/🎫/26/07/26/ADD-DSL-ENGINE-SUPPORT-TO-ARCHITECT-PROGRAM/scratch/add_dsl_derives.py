#!/usr/bin/env python3
"""One-time mechanical transformation: append dsl::DslRecord/dsl::DslScalar to the derive lists of
every Program-reachable struct/enum in architect/program/rs/lib.rs. Ticket-scoped scratch script,
not a permanent repo script (see .repo ticket rules)."""
import re
import sys

PATH = "/Users/ueli/Documents/semio/architect/program/rs/lib.rs"

RECORD_STRUCTS = [
    # kernel shared types
    "Ownership", "TextField", "TaggedNote", "TimestampMeta", "EntityHeader", "QuantitySpec",
    # program singular fields
    "ProgramMeta", "ProjectDefinition", "Governance",
    # registers (Program's Vec<T> fields)
    "Stakeholder", "UserProfile", "Activity", "Function", "ProgramElement", "QuantityRequirement",
    "Relationship", "Adjacency", "Process", "FlowRequirement", "AccessRule", "OperationalRequirement",
    "Equipment", "Resource", "StorageRequirement", "EnvironmentalRequirement", "HumanFactorRequirement",
    "AccessibilityRequirement", "PrivacyRequirement", "SafetyRequirement", "SecurityRequirement",
    "RegulatoryRequirement", "SiteContext", "OrganizationalRequirement", "ServiceRequirement",
    "InfrastructureRequirement", "InformationRequirement", "CommunicationRequirement",
    "WayfindingRequirement", "ScheduleRequirement", "FlexibilityRequirement", "GrowthPlan",
    "SustainabilityRequirement", "ResilienceRequirement", "CostRequirement", "DeliveryConstraint",
    "Risk", "Conflict", "Requirement", "PriorityRecord", "Scenario", "OptionEvaluation", "Decision",
    "ValidationRecord", "PerformanceCriterion", "QualityRecord", "DocumentRecord", "Assumption",
    "ConstraintRecord", "ComplianceRecord", "ApprovalRecord", "MeetingRecord", "ChangeRecord",
    "CollaborationRecord", "AnalysisRecord", "ReportRecord", "SearchFilter", "StatusRecord",
    "Workshop", "Survey", "Issue", "AuditEvent", "TemplateRecord", "KnowledgeRecord",
    "BenchmarkRecord", "TraceLink",
]

SCALAR_ENUMS = [
    "Priority", "LifecycleStatus", "TraceKind",
    "InfluenceLevel", "EngagementLevel", "UserCategory", "ProgramElementKind", "FunctionKind",
    "FlowKind", "PrivacyKind", "SafetyDomain", "SecurityControlKind", "StorageClass",
    "EnvironmentalParameter", "HumanFactorAspect", "AccessMode", "RelationshipKind", "AdjacencyKind",
    "ConnectionKind", "SeparationKind", "FlowDirection", "AccessLevel", "RiskLevel", "ConflictKind",
    "RequirementKind", "ValidationStatus", "AnalysisKind", "ReportKind", "IssueSeverity",
    "AuditAction", "CostBasis", "DeliveryPhase",
]

DOCUMENT_STRUCTS = ["Program"]


def find_decl_line(lines, name, kind):
    """kind: 'struct' or 'enum'. Returns index of the `pub struct NAME` / `pub enum NAME` line."""
    pat = re.compile(r'^\s*pub ' + kind + r' ' + re.escape(name) + r'\b')
    matches = [i for i, l in enumerate(lines) if pat.match(l)]
    if len(matches) != 1:
        raise SystemExit(f"expected exactly 1 declaration for {kind} {name}, found {len(matches)}: {matches}")
    return matches[0]


def find_derive_line_above(lines, decl_index, name):
    i = decl_index - 1
    while i >= 0:
        stripped = lines[i].strip()
        if stripped.startswith("#[derive("):
            return i
        if stripped.startswith("#[") or stripped.startswith("///") or stripped == "":
            i -= 1
            continue
        break
    raise SystemExit(f"could not find #[derive(...)] above {name} (stopped at line {i+1}: {lines[i]!r})")


def append_trait_to_derive(line, trait_name):
    if trait_name in line:
        return line
    assert line.rstrip().endswith(")]"), f"unexpected derive line shape: {line!r}"
    idx = line.rindex(")]")
    return line[:idx] + f", {trait_name}" + line[idx:]


def main():
    with open(PATH, "r") as f:
        lines = f.read().split("\n")

    changed = 0
    for name in RECORD_STRUCTS:
        decl = find_decl_line(lines, name, "struct")
        derive_i = find_derive_line_above(lines, decl, name)
        new_line = append_trait_to_derive(lines[derive_i], "dsl::DslRecord")
        if new_line != lines[derive_i]:
            lines[derive_i] = new_line
            changed += 1

    for name in SCALAR_ENUMS:
        decl = find_decl_line(lines, name, "enum")
        derive_i = find_derive_line_above(lines, decl, name)
        new_line = append_trait_to_derive(lines[derive_i], "dsl::DslScalar")
        if new_line != lines[derive_i]:
            lines[derive_i] = new_line
            changed += 1

    for name in DOCUMENT_STRUCTS:
        decl = find_decl_line(lines, name, "struct")
        derive_i = find_derive_line_above(lines, decl, name)
        new_line = append_trait_to_derive(lines[derive_i], "dsl::DslDocument")
        if new_line != lines[derive_i]:
            lines[derive_i] = new_line
            changed += 1
        # insert #[dsl(extension = "...")] right after the derive line if not already present
        if not lines[derive_i + 1].strip().startswith("#[dsl("):
            indent = re.match(r'^(\s*)', lines[derive_i]).group(1)
            lines.insert(derive_i + 1, f'{indent}#[dsl(extension = "architect", layout = "lines")]')
            changed += 1

    with open(PATH, "w") as f:
        f.write("\n".join(lines))

    print(f"applied {changed} edits")


if __name__ == "__main__":
    main()
