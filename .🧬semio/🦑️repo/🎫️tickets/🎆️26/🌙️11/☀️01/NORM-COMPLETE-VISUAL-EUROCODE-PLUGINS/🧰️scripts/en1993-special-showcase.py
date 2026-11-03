"""Emits `En1993Snapshot::special_parts_showcase()` from the `show` values of en1993-special-spec.py (exec'd by en1993-special-gen.py)."""

ROWS = {"SteelMember": "member-b1", "ColdFormedMember": "cf-1", "PlatedPanel": "pp-1", "SiloShell": "silo-1", "TensionComponent": "ten-1", "BridgeFatigue": "br-1", "TowerLeg": "tw-1", "SteelPile": "pile-1", "CraneRunway": "crane-1", "FatigueDetail": "fat-1"}
BASE_SHOW = {"base_plate_length": 0.35, "concrete_fcd": 16.7e6, "concrete_modulus": 31.0e9, "concrete_area_a0": 0.098, "concrete_area_a1": 0.5, "joint_coefficient": 0.667, "anchor_area": 245.0e-6, "anchor_length": 0.2, "base_normal_force": -300000.0, "base_moment": 60000.0, "base_lever_left": 0.15, "base_lever_right": 0.15}
BP_SHARED = {"plate_fy", "plate_width", "bolt_to_web", "column_flange_thickness", "column_web_thickness", "column_fy", "bolt_stress_area"}
BP_OVERRIDE = {"plate_width": 0.3, "bolt_to_web": 0.05}


def lit(fd, v):
    if fd["ty"] == "String":
        return f'"{v}".into()' if v != "" else "String::new()"
    return str(int(v)) if fd["ty"] == "u32" else repr(float(v))


def assigns(var, ent, indent, only=None, skip=None, override=None):
    out = ""
    for fd in SPEC[ent]["fields"]:
        if (only is not None and fd["name"] not in only) or (skip is not None and fd["name"] in skip):
            continue
        v = (override or {}).get(fd["name"], fd["show"])
        out += f"{indent}{var}.{fd['name']} = {lit(fd, v)};\n"
    return out


TAIL = """        let stainless_steel = SteelMaterial { id: "mat-1.4301".into(), grade: "1.4301".into(), fy: 210.0e6, fu: 520.0e6, e_modulus: 200.0e9, g_modulus: 77.0e9, subgrade: String::new(), kind: "stainless".into() };
        let mut stainless_member = doc.members[0].clone();
        stainless_member.id = "member-ss".into();
        stainless_member.label = "Stainless beam-column".into();
        stainless_member.material_id = stainless_steel.id.clone();
        stainless_member.stainless_forming = "coldFormedOpen".into();
        doc.member_actions.push(MemberAction { id: "act-ss-g".into(), member_id: stainless_member.id.clone(), load_case_id: "g-permanent".into(), action: DesignAction { n: 40_000.0, vy: 0.0, vz: 10_000.0, my: 15_000.0, mz: 0.0, t: 0.0 } });
        doc.fire_exposures.push(FireExposure { id: "fire-ss".into(), member_id: stainless_member.id.clone(), mu0: 0.30, design_temperature: 550.0, ..doc.fire_exposures[0].clone() });
        let mut pipeline = doc.silo_shells[0].clone();
        pipeline.id = "pipe-1".into();
        pipeline.radius = 0.3;
        pipeline.thickness = 0.012;
        pipeline.depth = 1.0;
        pipeline.length = 0.0;
        pipeline.fabrication_class = String::new();
        pipeline.wall_friction = 0.0;
        pipeline.plan_area = 0.0;
        pipeline.plan_perimeter = 0.0;
        pipeline.liquid_density = 0.0;
        pipeline.liquid_height = 0.0;
        pipeline.course_elevation = 0.0;
        pipeline.internal_pressure = 4.0e6;
        pipeline.external_pressure = 0.1e6;
        pipeline.temperature_change = 30.0;
        doc.silo_shells.push(pipeline);
        doc.materials.push(stainless_steel);
        doc.members.push(stainless_member);
        doc
    }
"""


def showcase():
    p = SCHEMA / "📸️snapshot/🦀️.rs"
    s = rd(p)
    if "special_parts_showcase" in s:
        return
    body = "    /// 🔭️ Compliant frame extended by one populated row of every special part of EN 1993-1-3 to 6 and 1-10: the subject of the wiring tests.\n    pub fn special_parts_showcase() -> Self {\n        let mut doc = Self::compliant_heb240_frame();\n"
    for ent, rid in ROWS.items():
        body += f'        if let Some(row) = doc.{VAR[ent]}.iter_mut().find(|r| r.id == "{rid}") {{\n' + assigns("row", ent, "            ") + "        }\n"
    body += "        let template = doc.joints[0].clone();\n"
    body += '        let mut end_plate = SteelJoint { id: "joint-ep".into(), kind: "endPlate".into(), bolt_class: "10.9".into(), bolt_rows: 2, bolts_per_row: 2, pitch: 0.09, gauge: 0.11, end_distance: 0.04, edge_distance: 0.04, plate_thickness: 0.02, weld_throat: 0.006, ..template.clone() };\n'
    body += assigns("end_plate", "SteelJoint", "        ", skip=set(BASE_SHOW))
    body += '        let mut base_plate = SteelJoint { id: "joint-bp".into(), kind: "basePlate".into(), bolt_class: "8.8".into(), bolt_rows: 1, bolts_per_row: 2, edge_distance: 0.05, plate_thickness: 0.03, ..template };\n'
    body += assigns("base_plate", "SteelJoint", "        ", only=BP_SHARED | set(BASE_SHOW), override={**BASE_SHOW, **BP_OVERRIDE})
    body += "        doc.joints.push(end_plate);\n        doc.joints.push(base_plate);\n" + TAIL
    marker = "        doc\n    }\n}\n\npub fn catalogue_heb240"
    i = s.index(marker)
    j = i + len("        doc\n    }\n")
    wr(p, s[:j] + "\n" + body + s[j:])


def leaf_test():
    p = SCHEMA / "🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
    t = rd(p)
    if f"{TAG}_LEAVES" in t:
        return
    rows = ""
    for ent, spec in SPEC.items():
        rid = ROWS.get(ent, "joint-ep")
        for fd in spec["fields"]:
            kind = {"String": "s", "u32": "u", "f64": "f"}[fd["ty"]]
            rows += f'    ("{spec["collection"]}", "{rid}", "{camel(fd["name"])}", \'{kind}\'),\n'
    t = t.rstrip("\n") + f"""

const {TAG}_LEAVES: &[(&str, &str, &str, char)] = &[
{rows}];

#[test]
fn every_{TAG.lower()}_leaf_resolves_to_one_typed_leaf_kind() {{
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in {TAG}_LEAVES {{
        let path = format!("{{collection}}[id={{id}}].{{field}}");
        let value = match kind {{
            's' => "x".to_string().to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        }};
        let mutations = crate::resolve(&before, &NormEdit::SetField {{ path: path.clone(), value }}).unwrap_or_else(|error| panic!("{{path}}: {{error}}"));
        assert_eq!(mutations.len(), 1, "{{path}}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{{path}}");
    }}
}}
"""
    wr(p, t)
