#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import json

rows = json.load(open("/tmp/catalog_rows.json", encoding="utf-8"))
by_rid = {r["rid"]: r for r in rows}

# ---- add_register_item_operation ----
# Preserve exact per-register item-construction expressions from the original file (label-based
# default_* builders / default_from_json fixtures) — only the OUTER wrapping changes.
ITEM_EXPR = {
    "elements": "default_element(label)",
    "stakeholders": "default_stakeholder(label)",
    "requirements": "default_requirement(label)",
    "risks": "default_risk(label)",
    "issues": "default_issue(label)",
    "functions": "default_function(label)",
    "users": "default_user(label)",
}

lines_add = []
lines_add.append("pub fn add_register_item_operation(program: &ProgramSnapshot, register: &str, label: &str) -> Option<(ProgramMutation, EntityId)> {")
lines_add.append("    let _ = program;")
lines_add.append("    macro_rules! create {")
lines_add.append("        ($variant:ident, $module:ident, $field:ident, $item:expr) => {{")
lines_add.append("            let item = $item;")
lines_add.append("            let id = item.header.id.clone();")
lines_add.append("            (ProgramMutation::$variant(leaves::$module::mutation::$variant { $field: item }), id)")
lines_add.append("        }};")
lines_add.append("    }")
lines_add.append("    Some(match register {")
SIMPLE = {r: e for r, e in ITEM_EXPR.items()}
CUSTOM = {
    "activities": '{\n            let item: crate::artifacts::program::Activity = default_from_json("activities", label, json!({ "code": "ACT", "category": "general", "activityType": "general" }))?;\n            create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, item)\n        }',
    "assumptions": 'create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, default_from_json::<crate::artifacts::program::Assumption>("assumptions", label, json!({ "statement": { "text": "" }, "validationStatus": "pending" }),)?)',
    "constraints": '{\n            create!(\n                $CREATE_VARIANT,\n                $CREATE_MOD,\n                $CREATE_FIELD,\n                default_from_json::<crate::artifacts::program::ConstraintRecord>("constraints", label, json!({ "constraintType": "general", "summary": { "text": "" }, "severity": "medium", "complianceStatus": "pending" }),)?\n            )\n        }',
    "compliance_records": '{\n            create!(\n                $CREATE_VARIANT,\n                $CREATE_MOD,\n                $CREATE_FIELD,\n                default_from_json::<crate::artifacts::program::ComplianceRecord>("compliance", label, json!({ "standardRef": "", "obligation": { "text": "" }, "complianceStatus": "pending", "severity": "medium" }),)?\n            )\n        }',
    "approvals": 'create!(\n            $CREATE_VARIANT,\n            $CREATE_MOD,\n            $CREATE_FIELD,\n            default_from_json::<crate::artifacts::program::ApprovalRecord>(\n                "approvals",\n                label,\n                json!({\n                    "approvalType": "general",\n                    "subjectId": EntityId::new_serial("subject", "approvalStatus"), "approvalStatus": "draft"\n                }),\n            )?\n        )',
    "meetings": 'create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, default_from_json::<crate::artifacts::program::MeetingRecord>("meetings", label, json!({ "meetingType": "workshop", "quorumMet": false, "meetingStatus": "draft" }),)?)',
    "analyses": 'create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, default_from_json::<AnalysisRecord>("analysis", label, json!({ "kind": "gap", "title": label, "outputSummary": { "text": "" } }),)?)',
    "reports": 'create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, default_from_json::<ReportRecord>("report", label, json!({ "kind": "executiveSummary", "title": label, "approvalStatus": "pending", "version": "0" }),)?)',
    "templates": 'create!($CREATE_VARIANT, $CREATE_MOD, $CREATE_FIELD, default_from_json::<crate::artifacts::program::TemplateRecord>("template", label, json!({ "templateType": "sector", "version": "1", "approvalStatus": "pending", "usageCount": 0 }),)?)',
}
TRACE_ARM = '''        "traces" => {
            let from = program.elements.first().map_or_else(|| EntityId::new_serial("from", "from"), |element| element.header.id.clone());
            let to = program.elements.get(1).map_or_else(|| EntityId::new_serial("to", "to"), |element| element.header.id.clone());
            let item = TraceLink::new(from, to, TraceKind::FunctionToProgramElement);
            let id = item.id.clone();
            (ProgramMutation::ConnectTrace(leaves::connect_trace::mutation::ConnectTrace { trace: item }), id)
        }'''

REGISTER_IDS_ORDER = ["stakeholders","users","activities","functions","elements","quantities","relationships","adjacencies","processes","flows","access_rules","operations","equipment","resources","storage","environmental","human_factors","accessibility","privacy","safety","security","regulatory","site_context","organizational","services","infrastructure","information","communication","wayfinding","schedules","flexibility","growth","sustainability","resilience","costs","delivery","risks","conflicts","requirements","priorities","scenarios","options","decisions","validations","performance","quality","documents","assumptions","constraints","compliance_records","approvals","meetings","changes","collaboration","analyses","reports","search_filters","status_records","workshops","surveys","issues","audit_events","templates","knowledge","benchmarks","traces"]

for rid in REGISTER_IDS_ORDER:
    if rid == "adjacencies":
        continue  # add_register_item_operation never supported adjacencies (no arm in original)
    if rid == "traces":
        lines_add.append(TRACE_ARM)
        continue
    r = by_rid[rid]
    if rid in CUSTOM:
        expr = CUSTOM[rid].replace("$CREATE_VARIANT", r["create_struct"]).replace("$CREATE_MOD", r["create_mod"]).replace("$CREATE_FIELD", r["create_field"])
        lines_add.append(f'        "{rid}" => {expr},' if not expr.startswith("{") else f'        "{rid}" => {expr}')
    elif rid in SIMPLE:
        lines_add.append(f'        "{rid}" => create!({r["create_struct"]}, {r["create_mod"]}, {r["create_field"]}, {SIMPLE[rid]}),')
    # registers absent from SIMPLE/CUSTOM/traces get no arm (matches original: falls to `_ => return None`)
lines_add.append("        _ => return None,")
lines_add.append("    })")
lines_add.append("}")
add_fn = "\n".join(lines_add)

# ---- remove_register_item_operation ----
lines_rm = []
lines_rm.append("pub fn remove_register_item_operation(register: &str, entity_id: EntityId) -> Option<ProgramMutation> {")
lines_rm.append("    macro_rules! delete {")
lines_rm.append("        ($variant:ident, $module:ident) => {")
lines_rm.append("            ProgramMutation::$variant(leaves::$module::mutation::$variant { id: entity_id })")
lines_rm.append("        };")
lines_rm.append("    }")
lines_rm.append("    Some(match register {")
for rid in REGISTER_IDS_ORDER:
    if rid == "adjacencies":
        lines_rm.append('        "adjacencies" => ProgramMutation::DisconnectAdjacency(leaves::disconnect_adjacency::mutation::DisconnectAdjacency { id: entity_id }),')
        continue
    if rid == "traces":
        lines_rm.append('        "traces" => ProgramMutation::DisconnectTrace(leaves::disconnect_trace::mutation::DisconnectTrace { id: entity_id }),')
        continue
    r = by_rid[rid]
    lines_rm.append(f'        "{rid}" => delete!({r["delete_struct"]}, {r["delete_mod"]}),')
lines_rm.append("        _ => return None,")
lines_rm.append("    })")
lines_rm.append("}")
rm_fn = "\n".join(lines_rm)

# ---- patch_register_item_operation ----
PATCH_REGISTERS = ["stakeholders", "elements", "adjacencies", "requirements", "risks", "issues", "functions", "users"]
lines_patch = []
lines_patch.append("fn merge_json_patch<T: Clone + Serialize + DeserializeOwned>(existing: &T, patch: &Value) -> Option<T> {")
lines_patch.append("    let mut value = serde_json::to_value(existing).ok()?;")
lines_patch.append("    let (Value::Object(base), Value::Object(patch_map)) = (&mut value, patch) else { return None };")
lines_patch.append("    for (key, entry) in patch_map {")
lines_patch.append("        base.insert(key.clone(), entry.clone());")
lines_patch.append("    }")
lines_patch.append("    serde_json::from_value(value).ok()")
lines_patch.append("}")
lines_patch.append("")
lines_patch.append("pub fn patch_register_item_operation(program: &ProgramSnapshot, register: &str, entity_id: EntityId, patch: Value) -> Option<ProgramMutation> {")
lines_patch.append("    Some(match register {")
for rid in PATCH_REGISTERS:
    if rid == "adjacencies":
        lines_patch.append('        "adjacencies" => {')
        lines_patch.append("            let existing = program.adjacencies.iter().find(|row| row.header.id == entity_id)?;")
        lines_patch.append("            let merged = merge_json_patch(existing, &patch)?;")
        lines_patch.append("            ProgramMutation::ConnectAdjacency(leaves::connect_adjacency::mutation::ConnectAdjacency { adjacency: merged })")
        lines_patch.append("        }")
        continue
    r = by_rid[rid]
    lines_patch.append(f'        "{rid}" => {{')
    lines_patch.append(f'            let existing = program.{rid}.iter().find(|row| row.header.id == entity_id)?;')
    lines_patch.append(f'            let merged = merge_json_patch(existing, &patch)?;')
    lines_patch.append(f'            ProgramMutation::{r["replace_struct"]}(leaves::{r["replace_mod"]}::mutation::{r["replace_struct"]} {{ {r["replace_field"]}: merged }})')
    lines_patch.append("        }")
lines_patch.append("        _ => return None,")
lines_patch.append("    })")
lines_patch.append("}")
patch_fn = "\n".join(lines_patch)

with open("/tmp/catalog_new_functions.rs", "w", encoding="utf-8") as f:
    f.write(add_fn + "\n\n" + rm_fn + "\n\n" + patch_fn + "\n")
print("wrote /tmp/catalog_new_functions.rs")
