"""🗺️ c4p9 split plans for Package 9: the seven architect program Rust modules (the catalog is cut by its generator) and the process3d editor lib root.
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `ranges` are 1-based inclusive line ranges of the pre-edit file; each starts at an item start (docs and attributes included) and ends at an item end or its trailing region marker.
`regions` shortcuts: `RG(key, first, last)` is the line range of the first-th to last-th top-level region (1-based) of the pre-edit file."""
import re
from c4plib import *

LIB_ARCH = f"{ANY[:ANY.index('/🏅️standards')]}/🦀️.rs"
LIB_P3D = f"{P3D}/🦀️.rs"


def top_regions(key):
    ls = read(f"{ORIG}/{key}.rs").split("\n")
    out, stack = [], []
    for i, l in enumerate(ls, 1):
        if re.match(r"^//\s?#region\b", l):
            stack.append(i)
        elif re.match(r"^//\s?#endregion\b", l) and stack:
            s = stack.pop()
            if not stack:
                out.append((s, i))
    return out


def RG(key, first, last):
    rs = top_regions(key)
    return (rs[first - 1][0], rs[last - 1][1])


def regs(rel_dir, tail=None):
    return f"{REGS}/{rel_dir}/🦀️.rs"


PLANS = {
    "inf": dict(
        rel=f"{SCHEMA}/💡️inferences/🦀️.rs", mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="🔀️adjacency-and-validation", mod="adjacency_and_validation", ranges=[(95, 207), (215, 631)], doc="🔀️ Program adjacency views and conflicts, and the structural validation of a program snapshot."),
            dict(folder="🎁️outputs-and-reports", mod="outputs_and_reports", ranges=[(639, 824), (832, 1161)], doc="🎁️ Program outputs (requirement lists, matrices, maps, schedules) and the structured program reports."),
            dict(folder="📊️status-search-and-analysis", mod="status_search_and_analysis", ranges=[(1169, 1326), (1334, 1529), (1537, 1945)], doc="📊️ Program status summary, register search and the requirement and cost analyses."),
        ]),
    "ctx": dict(
        rel=regs("🌐️context-requirements"), mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="🏢️site-and-organization-requirements", mod="site_and_organization_requirements", ranges=[RG("ctx", 1, 7)], doc="🏢️ Site context and the organizational, service, infrastructure, information, communication and wayfinding requirements."),
            dict(folder="🗓️schedule-growth-and-cost-requirements", mod="schedule_growth_and_cost_requirements", ranges=[RG("ctx", 8, 13)], doc="🗓️ Schedule, flexibility, growth, sustainability, resilience and cost requirements."),
            dict(folder="⚠️delivery-risk-and-conflict", mod="delivery_risk_and_conflict", ranges=[RG("ctx", 14, 16)], doc="⚠️ Delivery constraints, risks and conflicts."),
        ]),
    "dec": dict(
        rel=regs("📋️decisions"), mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="🎯️requirements-and-decisions", mod="requirements_and_decisions", ranges=[RG("dec", 1, 7)], doc="🎯️ Requirements, priorities, scenarios, option evaluations, decisions, validations and performance criteria."),
            dict(folder="📈️quality-change-and-reports", mod="quality_change_and_reports", ranges=[RG("dec", 8, 14)], doc="📈️ Quality, artifact, change, collaboration, analysis and report records plus search filters."),
        ]),
    "ops": dict(
        rel=regs("🔁️operations-requirements"), mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="⚙️process-flow-and-resources", mod="process_flow_and_resources", ranges=[RG("ops", 1, 7)], doc="⚙️ Processes, flow requirements, access rules, operational requirements, equipment, resources and storage."),
            dict(folder="🛡️human-factor-and-compliance", mod="human_factor_and_compliance", ranges=[RG("ops", 8, 14)], doc="🛡️ Environmental, human factor, accessibility, privacy, safety, security and regulatory requirements."),
        ]),
    "rec": dict(
        rel=regs("🗃️records"), mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="📝️status-workshop-and-audit-records", mod="status_workshop_and_audit_records", ranges=[RG("rec", 1, 7)], doc="📝️ Status, workshop, survey, issue, audit, template and knowledge records."),
            dict(folder="🧾️benchmark-compliance-and-governance", mod="benchmark_compliance_and_governance", ranges=[RG("rec", 8, 14)], doc="🧾️ Benchmark, assumption, constraint, compliance, approval and meeting records plus governance."),
        ]),
    "str": dict(
        rel=regs("🧭️program-structure"), mode="mod", lib=LIB_ARCH,
        children=[
            dict(folder="🏗️project-and-stakeholders", mod="project_and_stakeholders", ranges=[RG("str", 1, 5)], doc="🏗️ Program meta, project definition, stakeholders, user profiles and activities."),
            dict(folder="🧱️elements-and-relationships", mod="elements_and_relationships", ranges=[RG("str", 6, 10)], doc="🧱️ Functions, program elements, quantity requirements, relationships and adjacencies."),
        ]),
    "p3d": dict(
        rel=f"{P3D}/🦀️.rs", mode="mod", lib=LIB_P3D,
        children=[
            dict(folder="🧵️retained-commands", mod="retained_commands", ranges=[(222, 646)], doc="🧵️ Process3d retained command contracts, dispositions, bounded and resumable job factories."),
            dict(folder="📬️store-preparation", mod="store_preparation", ranges=[(648, 1203)], doc="📬️ Process3d config and artifact store preparation: retained byte measures, grants and the one-item factories."),
            dict(folder="🪟️editor-impl", mod="editor_impl", ranges=[(1266, 1700)], doc="🪟️ Process3d play app body rendering and the `ArtifactEditor` implementation."),
        ]),
}
ORDER = ["inf", "ctx", "dec", "ops", "rec", "str", "p3d"]
