/**
 * 🤝️ The coordination vocabulary of `s.bim.model@1` (WP-16: clash sets, rule sets and issues). `r3-f1-gen-model.ts` spreads these rows into its single model, so Rust, JSON Schema, TypeScript,
 * GraphQL and proto are generated from here and never drift.
 *
 * A clash set is an authored pair of selectors (side A and side B) with a tolerance and a clearance; the clashes between the elements the selectors pick are the inferred `clash-sets`, grouped
 * and never stored. A rule is an authored numeric code check (a limit, a severity and a scope); its findings are the inferred `rule-results`. An issue is an authored record (BCF topic): title,
 * status, priority, assignee, the elements it concerns, an optional clash it was raised from and an optional viewpoint (orbit camera, section box, isolated elements); its comments are keyed
 * records of their own, so a BCF round trip is a sequence of ordinary events.
 */
type Field = { name: string; type: string; doc?: string };

const f = (spec: string): Field[] =>
  spec.split(",").map((part) => part.trim()).filter(Boolean).map((part) => {
    const at = part.indexOf(":");
    return { name: part.slice(0, at).trim(), type: part.slice(at + 1).trim() };
  });

export const coordinationUnitEnums = [
  {
    name: "ElementClass",
    doc: "🧩️ The kind of building element a selector names (the families that have a solid): walls, curtain walls, windows, doors, columns, beams, slabs, ceilings, roofs, stairs, ramps, railings, wall sweeps, placed components and routed MEP elements.",
    variants: ["Wall", "CurtainWall", "Window", "Door", "Column", "Beam", "Slab", "Ceiling", "Roof", "Stair", "Ramp", "Railing", "WallSweep", "Component", "Mep"],
  },
  {
    name: "RuleKind",
    doc: "📏️ The numeric code checks a rule can make: the clear height of spaces (minimum), the riser height (maximum), the tread depth (minimum) and the clear width (minimum) of stairs, the clear width of doors (minimum), the slope of ramps (maximum, rise over run), the width of corridors (minimum) and the net floor area of a compartment zone (maximum).",
    variants: ["MinClearHeight", "MaxRiser", "MinTread", "MinStairWidth", "MinDoorWidth", "MaxRampSlope", "MinCorridorWidth", "MaxCompartmentArea"],
  },
  { name: "RuleSeverity", doc: "🚦️ How serious a violation of a rule is: an error, a warning or a note.", variants: ["Error", "Warning", "Note"] },
  { name: "IssueStatus", doc: "🏁️ Where an issue stands: open, in progress, resolved (fixed, not yet confirmed) or closed.", variants: ["Open", "InProgress", "Resolved", "Closed"] },
  { name: "IssuePriority", doc: "⏫️ How urgent an issue is.", variants: ["Low", "Normal", "High", "Critical"] },
];

export const coordinationDataEnums: unknown[] = [];

export const coordinationStructs = [
  {
    name: "ElementSelector",
    doc: "🎯️ A selector of building elements: every element that is of one of the classes, stands on one of the storeys, is in one of the phases and is among the ids; an empty list does not restrict. The ids of elements that do not exist select nothing.",
    fields: f("classes:vec:ElementClass, storeys:vec:string, phases:vec:Phase, ids:vec:string"),
  },
  {
    name: "RuleScope",
    doc: "🎯️ Where a rule applies: the storeys, phases and element ids it covers (an empty list does not restrict) and, for the rules about spaces and zones, the usage of the spaces or the category of the zones it covers.",
    fields: f("storeys:vec:string, phases:vec:Phase, ids:vec:string, filter:string"),
  },
  {
    name: "ClashRef",
    doc: "💥️ The clash an issue was raised from: the clash set and the two elements that clash.",
    fields: f("set:string, first:string, second:string"),
  },
  {
    name: "SectionBox",
    doc: "🧊️ An axis-aligned box in building-datum metres: the section box of a viewpoint hides everything outside it.",
    copy: true,
    fields: f("min:Point3, max:Point3"),
  },
  {
    name: "IssueViewpoint",
    doc: "📷️ A viewpoint of an issue: the orbit camera of the 3D view, an optional section box and the elements that stay visible while all others are hidden (empty shows everything).",
    fields: f("camera:ViewCamera, section:opt:SectionBox, isolate:vec:string"),
  },
  {
    name: "ClashSet",
    doc: "💥️ A clash set: the elements of side A are tested against the elements of side B. Two elements clash hard when their solids interpenetrate by more than the tolerance and softly when they stay closer than the clearance; touching or sharing a face is no clash. The clashes are inferred, grouped and never stored.",
    entity: { collection: "clash_sets", plural: "ClashSets" },
    fields: f("name:string, a:ElementSelector, b:ElementSelector, tolerance:f64, clearance:f64"),
  },
  {
    name: "Rule",
    doc: "⚖️ A rule: one numeric code check with a limit, a severity and a scope. Its findings (the elements that break the limit, with the measured value) are inferred and never stored.",
    entity: { collection: "rules", plural: "Rules" },
    fields: f("name:string, kind:RuleKind, limit:f64, severity:RuleSeverity, scope:RuleScope"),
  },
  {
    name: "Issue",
    doc: "🚩️ An issue (a BCF topic): what is wrong, who raised it and who has to fix it, the elements it concerns, the clash it was raised from and the viewpoint that shows it. Comments are records of their own.",
    entity: { collection: "issues", plural: "Issues" },
    fields: f("title:string, description:string, status:IssueStatus, priority:IssuePriority, assignee:string, author:string, created:string, labels:vec:string, elements:vec:string, clash:opt:ClashRef, viewpoint:opt:IssueViewpoint"),
  },
  {
    name: "IssueComment",
    doc: "💬️ One comment of an issue: who wrote it, when and what.",
    entity: { collection: "issue_comments", plural: "IssueComments" },
    fields: f("issue:string, author:string, date:string, text:string"),
  },
];

export const coordinationFieldDocs: Record<string, string> = {
  "ElementSelector.classes": "The element classes selected; empty selects every class.",
  "ElementSelector.storeys": "Ids of the storeys the elements stand on; empty selects every storey. An opening stands on the storey of its host.",
  "ElementSelector.phases": "The construction phases selected; empty selects every phase. An element that carries no phase counts as new, an opening takes the phase of its host.",
  "ElementSelector.ids": "Ids of the elements selected; empty selects every element. Ids that name no element select nothing.",
  "RuleScope.storeys": "Ids of the storeys the rule covers; empty covers every storey.",
  "RuleScope.phases": "The construction phases the rule covers; empty covers every phase.",
  "RuleScope.ids": "Ids of the elements the rule covers (the stair, door, ramp, space or zone it checks); empty covers every element.",
  "RuleScope.filter": "For the space rules the usage of the spaces covered (compared without regard to case), for the compartment rule the category of the zones covered; empty covers all.",
  "ClashRef.set": "The clash set that reported the clash.",
  "ClashRef.first": "The first element of the clash (side A).",
  "ClashRef.second": "The second element of the clash (side B).",
  "SectionBox.min": "The corner with the smallest coordinates.",
  "SectionBox.max": "The corner with the largest coordinates.",
  "IssueViewpoint.camera": "The orbit camera of the 3D view: the point it looks at, its azimuth and pitch and the distance to the target.",
  "IssueViewpoint.section": "Optional section box that hides everything outside it; absent shows the whole model.",
  "IssueViewpoint.isolate": "Ids of the elements that stay visible while all others are hidden; empty hides nothing.",
  "ClashSet.name": "The name of the clash set as listed in the clash panel.",
  "ClashSet.a": "Side A: the elements tested against side B.",
  "ClashSet.b": "Side B: the elements side A is tested against. When both sides select the same element it is not tested against itself and a pair is reported once.",
  "ClashSet.tolerance": "Penetration in metres below which an overlap is not reported as a hard clash (zero reports every interpenetration).",
  "ClashSet.clearance": "Distance in metres below which two elements that do not touch are reported as a soft clash (zero reports hard clashes only).",
  "Rule.name": "The name of the rule as listed in the rules panel.",
  "Rule.kind": "Which measure the rule checks and in which direction (minimum or maximum).",
  "Rule.limit": "The limit: metres for heights, depths and widths, a ratio of rise to run for the ramp slope, square metres for the compartment area.",
  "Rule.severity": "How serious a violation is.",
  "Rule.scope": "Where the rule applies.",
  "Issue.title": "The headline of the issue.",
  "Issue.description": "What is wrong, in full.",
  "Issue.status": "Where the issue stands.",
  "Issue.priority": "How urgent the issue is.",
  "Issue.assignee": "Who has to fix the issue; empty is unassigned.",
  "Issue.author": "Who raised the issue.",
  "Issue.created": "When the issue was raised, as year-month-day or year-month-dayThour:minute:second with an optional Z or offset.",
  "Issue.labels": "Free labels of the issue (for example Clash or MEP).",
  "Issue.elements": "Ids of the elements the issue concerns. An element that is deleted later stays named here as history.",
  "Issue.clash": "Optional: the clash the issue was raised from.",
  "Issue.viewpoint": "Optional: the viewpoint that shows the issue.",
  "IssueComment.issue": "The issue the comment belongs to.",
  "IssueComment.author": "Who wrote the comment.",
  "IssueComment.date": "When the comment was written, as year-month-day or year-month-dayThour:minute:second with an optional Z or offset.",
  "IssueComment.text": "The comment.",
};

export const coordinationCollections = ["clash_sets", "rules", "issues", "issue_comments"];
