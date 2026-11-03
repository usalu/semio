//! 🧬️ EN 1993 mutation unit tests — typed leaf and row kinds.

use semio_s_plugin_norm_compliance::AnnexChoice;
use crate::mutation_bridge::apply_en1993_mutation;
use semio_s_artifact_norm_en1993_mut_general::change_annex;
use crate::En1993Mutation;
use semio_s_artifact_norm_en1993_model::{SteelMember, En1993Snapshot};

fn base() -> En1993Snapshot {
    En1993Snapshot::compliant_heb240_frame()
}

#[test]
fn change_annex_switches_national_annex() {
    let before = base();
    let mutation = En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.annex, AnnexChoice::En);
}

#[test]
fn insert_and_remove_member_round_trip() {
    let before = base();
    let member = SteelMember {
        warping_support: semio_s_artifact_norm_en1993_model::WarpingSupport::Fork,
        web_material_id: String::new(),
        torsion_load: semio_s_artifact_norm_en1993_model::TorsionLoad::Point,
        torque_position: 0.5,
        stainless_forming: semio_s_artifact_norm_en1993_model::StainlessForming::Unset,
        design_temperature: 0.0,
        toughness_stress_ratio: 0.75,
        radiation_shift: 0.0,
        stress_shift: 0.0,
        margin_shift: 0.0,
        strain_rate_shift: 0.0,
        cold_forming_radius: 0.0,
        id: "member-x".into(),
        label: "X".into(),
        member_type: semio_s_artifact_norm_en1993_model::SteelMemberType::Beam,
        section_id: "sec-heb240".into(),
        material_id: "mat-s355".into(),
        length: 3.0,
        buckling_length_y: 3.0,
        buckling_length_z: 3.0,
        ltb_length: 3.0,
        ltb_restraint_spacing: 1.5,
        load_application: semio_s_artifact_norm_en1993_model::LoadApplication::ShearCenter,
        end_moment_ratio_psi: -1.0,
        moment_diagram: semio_s_artifact_norm_en1993_model::MomentDiagram::Linear,
        deflection_limit_ratio: 300.0,
        analysis: semio_s_artifact_norm_en1993_model::AnalysisKind::Elastic,
    };
    let insert = En1993Mutation::InsertMember(semio_s_artifact_norm_en1993_mut_general::insert_member::InsertMember { index: Some(before.members.len()), member: member.clone() });
    let (mid, _) = apply_en1993_mutation(&before, &insert).expect("insert");
    assert_eq!(mid.members.len(), before.members.len() + 1);
    let remove = En1993Mutation::RemoveMember(semio_s_artifact_norm_en1993_mut_general::remove_member::RemoveMember { index: mid.members.len() - 1 });
    let (after, _) = apply_en1993_mutation(&mid, &remove).expect("remove");
    assert_eq!(after.members.len(), before.members.len());
}

const SPECIAL_LEAVES: &[(&str, &str, &str, char)] = &[
    ("members", "member-b1", "stainlessForming", 's'),
    ("members", "member-b1", "designTemperature", 'f'),
    ("members", "member-b1", "toughnessStressRatio", 'f'),
    ("members", "member-b1", "radiationShift", 'f'),
    ("members", "member-b1", "stressShift", 'f'),
    ("members", "member-b1", "marginShift", 'f'),
    ("members", "member-b1", "strainRateShift", 'f'),
    ("members", "member-b1", "coldFormingRadius", 'f'),
    ("coldFormedMembers", "cf-1", "flangeWidth", 'f'),
    ("coldFormedMembers", "cf-1", "lipLength", 'f'),
    ("coldFormedMembers", "cf-1", "webHeight", 'f'),
    ("coldFormedMembers", "cf-1", "webSlope", 'f'),
    ("coldFormedMembers", "cf-1", "oppositeFlangeWidth", 'f'),
    ("coldFormedMembers", "cf-1", "springFactor", 'f'),
    ("coldFormedMembers", "cf-1", "cornerRadius", 'f'),
    ("coldFormedMembers", "cf-1", "flangeStress", 'f'),
    ("coldFormedMembers", "cf-1", "shearForce", 'f'),
    ("coldFormedMembers", "cf-1", "supportReaction", 'f'),
    ("coldFormedMembers", "cf-1", "supportLength", 'f'),
    ("coldFormedMembers", "cf-1", "cripplingCategory", 'u'),
    ("coldFormedMembers", "cf-1", "stiffenedSupport", 'u'),
    ("coldFormedMembers", "cf-1", "bendingMoment", 'f'),
    ("coldFormedMembers", "cf-1", "momentResistance", 'f'),
    ("coldFormedMembers", "cf-1", "sheetingStiffness", 'f'),
    ("coldFormedMembers", "cf-1", "requiredRestraintStiffness", 'f'),
    ("platedPanels", "pp-1", "stiffenerInertia", 'f'),
    ("platedPanels", "pp-1", "stiffenerArea", 'f'),
    ("platedPanels", "pp-1", "stiffenerEccentricity", 'f'),
    ("platedPanels", "pp-1", "stiffenerAlpha", 'f'),
    ("platedPanels", "pp-1", "compressionStress", 'f'),
    ("platedPanels", "pp-1", "halfWidth", 'f'),
    ("platedPanels", "pp-1", "equivalentSpan", 'f'),
    ("platedPanels", "pp-1", "hogging", 'u'),
    ("platedPanels", "pp-1", "patchType", 's'),
    ("platedPanels", "pp-1", "patchForce", 'f'),
    ("platedPanels", "pp-1", "patchLength", 'f'),
    ("platedPanels", "pp-1", "patchOffset", 'f'),
    ("platedPanels", "pp-1", "flangeThickness", 'f'),
    ("platedPanels", "pp-1", "flangeWidth", 'f'),
    ("platedPanels", "pp-1", "flangeFy", 'f'),
    ("platedPanels", "pp-1", "longitudinalOffset", 'f'),
    ("platedPanels", "pp-1", "longitudinalInertia", 'f'),
    ("platedPanels", "pp-1", "patchMoment", 'f'),
    ("platedPanels", "pp-1", "patchMomentResistance", 'f'),
    ("platedPanels", "pp-1", "bendingMoment", 'f'),
    ("platedPanels", "pp-1", "plasticMoment", 'f'),
    ("platedPanels", "pp-1", "flangeMoment", 'f'),
    ("platedPanels", "pp-1", "shearForce", 'f'),
    ("platedPanels", "pp-1", "transverseStiffenerInertia", 'f'),
    ("platedPanels", "pp-1", "flangeInducedK", 'f'),
    ("platedPanels", "pp-1", "flangeArea", 'f'),
    ("platedPanels", "pp-1", "stressX", 'f'),
    ("platedPanels", "pp-1", "stressZ", 'f'),
    ("platedPanels", "pp-1", "shearStress", 'f'),
    ("platedPanels", "pp-1", "alphaCr", 'f'),
    ("siloShells", "silo-1", "length", 'f'),
    ("siloShells", "silo-1", "fabricationClass", 's'),
    ("siloShells", "silo-1", "meridionalStress", 'f'),
    ("siloShells", "silo-1", "hoopStress", 'f'),
    ("siloShells", "silo-1", "shellShearStress", 'f'),
    ("siloShells", "silo-1", "boundaryFactorAxial", 'f'),
    ("siloShells", "silo-1", "boundaryFactorHoop", 'f'),
    ("siloShells", "silo-1", "boundaryFactorShear", 'f'),
    ("siloShells", "silo-1", "wallFriction", 'f'),
    ("siloShells", "silo-1", "planArea", 'f'),
    ("siloShells", "silo-1", "planPerimeter", 'f'),
    ("siloShells", "silo-1", "liquidDensity", 'f'),
    ("siloShells", "silo-1", "liquidHeight", 'f'),
    ("siloShells", "silo-1", "courseElevation", 'f'),
    ("siloShells", "silo-1", "internalPressure", 'f'),
    ("siloShells", "silo-1", "externalPressure", 'f'),
    ("siloShells", "silo-1", "temperatureChange", 'f'),
    ("joints", "joint-ep", "plateFy", 'f'),
    ("joints", "joint-ep", "plateWidth", 'f'),
    ("joints", "joint-ep", "boltToWeb", 'f'),
    ("joints", "joint-ep", "extensionToFlange", 'f'),
    ("joints", "joint-ep", "extensionEdge", 'f'),
    ("joints", "joint-ep", "alphaFactor", 'f'),
    ("joints", "joint-ep", "boltStressArea", 'f'),
    ("joints", "joint-ep", "boltGripLength", 'f'),
    ("joints", "joint-ep", "columnFlangeThickness", 'f'),
    ("joints", "joint-ep", "columnWebThickness", 'f'),
    ("joints", "joint-ep", "columnWebDepth", 'f'),
    ("joints", "joint-ep", "columnWebShearArea", 'f'),
    ("joints", "joint-ep", "columnRootRadius", 'f'),
    ("joints", "joint-ep", "columnFy", 'f'),
    ("joints", "joint-ep", "columnWebStress", 'f'),
    ("joints", "joint-ep", "beamFlangeThickness", 'f'),
    ("joints", "joint-ep", "compressionLever", 'f'),
    ("joints", "joint-ep", "momentDemand", 'f'),
    ("joints", "joint-ep", "beamMomentResistance", 'f'),
    ("joints", "joint-ep", "columnMomentResistance", 'f'),
    ("joints", "joint-ep", "beamInertia", 'f'),
    ("joints", "joint-ep", "beamSpan", 'f'),
    ("joints", "joint-ep", "bracedFrame", 'u'),
    ("joints", "joint-ep", "stiffnessRatio", 'f'),
    ("joints", "joint-ep", "transformationParameter", 'f'),
    ("joints", "joint-ep", "assumedClass", 's'),
    ("joints", "joint-ep", "basePlateLength", 'f'),
    ("joints", "joint-ep", "concreteFcd", 'f'),
    ("joints", "joint-ep", "concreteModulus", 'f'),
    ("joints", "joint-ep", "concreteAreaA0", 'f'),
    ("joints", "joint-ep", "concreteAreaA1", 'f'),
    ("joints", "joint-ep", "jointCoefficient", 'f'),
    ("joints", "joint-ep", "anchorArea", 'f'),
    ("joints", "joint-ep", "anchorLength", 'f'),
    ("joints", "joint-ep", "baseNormalForce", 'f'),
    ("joints", "joint-ep", "baseMoment", 'f'),
    ("joints", "joint-ep", "baseLeverLeft", 'f'),
    ("joints", "joint-ep", "baseLeverRight", 'f'),
    ("tensionComponents", "ten-1", "partialFactor", 'f'),
    ("tensionComponents", "ten-1", "cableLength", 'f'),
    ("tensionComponents", "ten-1", "cableWeight", 'f'),
    ("tensionComponents", "ten-1", "cableHorizontalForce", 'f'),
    ("tensionComponents", "ten-1", "cableModulus", 'f'),
    ("tensionComponents", "ten-1", "cableStress", 'f'),
    ("tensionComponents", "ten-1", "cableUnitWeight", 'f'),
    ("tensionComponents", "ten-1", "threadFactor", 'f'),
    ("tensionComponents", "ten-1", "rodStressArea", 'f'),
    ("tensionComponents", "ten-1", "rodFu", 'f'),
    ("tensionComponents", "ten-1", "pinDiameter", 'f'),
    ("tensionComponents", "ten-1", "pinPlateThickness", 'f'),
    ("tensionComponents", "ten-1", "pinGap", 'f'),
    ("tensionComponents", "ten-1", "pinOuterThickness", 'f'),
    ("tensionComponents", "ten-1", "pinInnerThickness", 'f'),
    ("tensionComponents", "ten-1", "pinFup", 'f'),
    ("tensionComponents", "ten-1", "pinFyp", 'f'),
    ("tensionComponents", "ten-1", "pinPlateFy", 'f'),
    ("bridgeFatigue", "br-1", "heavyVehicleWeight", 'f'),
    ("bridgeFatigue", "br-1", "referenceWeight", 'f'),
    ("bridgeFatigue", "br-1", "observedTraffic", 'f'),
    ("bridgeFatigue", "br-1", "designLife", 'f'),
    ("bridgeFatigue", "br-1", "secondLaneTraffic", 'f'),
    ("bridgeFatigue", "br-1", "secondLaneWeight", 'f'),
    ("bridgeFatigue", "br-1", "lambda1", 'f'),
    ("bridgeFatigue", "br-1", "lambdaMax", 'f'),
    ("bridgeFatigue", "br-1", "detailId", 's'),
    ("bridgeFatigue", "br-1", "plateThickness", 'f'),
    ("towerLegs", "tw-1", "latticeKind", 's'),
    ("towerLegs", "tw-1", "slenderness", 'f'),
    ("towerLegs", "tw-1", "diameter", 'f'),
    ("towerLegs", "tw-1", "height", 'f'),
    ("towerLegs", "tw-1", "bendingStiffness", 'f'),
    ("towerLegs", "tw-1", "massPerLength", 'f'),
    ("towerLegs", "tw-1", "strouhal", 'f'),
    ("towerLegs", "tw-1", "logDecrement", 'f'),
    ("towerLegs", "tw-1", "airDensity", 'f'),
    ("towerLegs", "tw-1", "meanWindSpeed", 'f'),
    ("towerLegs", "tw-1", "crossingRate", 'f'),
    ("towerLegs", "tw-1", "designLife", 'f'),
    ("towerLegs", "tw-1", "kFactor", 'f'),
    ("towerLegs", "tw-1", "kwFactor", 'f'),
    ("towerLegs", "tw-1", "lateralForceCoefficient", 'f'),
    ("towerLegs", "tw-1", "permittedAmplitudeRatio", 'f'),
    ("towerLegs", "tw-1", "ringMode", 'f'),
    ("towerLegs", "tw-1", "shellThickness", 'f'),
    ("towerLegs", "tw-1", "guyArea", 'f'),
    ("towerLegs", "tw-1", "guyAngle", 'f'),
    ("towerLegs", "tw-1", "guyLength", 'f'),
    ("piles", "pile-1", "environment", 's'),
    ("piles", "pile-1", "designLife", 'f'),
    ("piles", "pile-1", "wallThickness", 'f'),
    ("piles", "pile-1", "interlockBendingFactor", 'f'),
    ("piles", "pile-1", "interlockStiffnessFactor", 'f'),
    ("piles", "pile-1", "sectionModulus", 'f'),
    ("piles", "pile-1", "shearArea", 'f'),
    ("piles", "pile-1", "designMoment", 'f'),
    ("piles", "pile-1", "designShear", 'f'),
    ("piles", "pile-1", "interlockShear", 'f'),
    ("craneRunways", "crane-1", "maxWheelLoad", 'f'),
    ("craneRunways", "crane-1", "loadFactor", 'f'),
    ("craneRunways", "crane-1", "railInertia", 'f'),
    ("craneRunways", "crane-1", "railFixed", 'u'),
    ("craneRunways", "crane-1", "flangeEffectiveWidth", 'f'),
    ("craneRunways", "crane-1", "flangeThickness", 'f'),
    ("craneRunways", "crane-1", "wheelEccentricity", 'f'),
    ("craneRunways", "crane-1", "torsionConstant", 'f'),
    ("craneRunways", "crane-1", "longitudinalStress", 'f'),
    ("craneRunways", "crane-1", "railRadius", 'f'),
    ("craneRunways", "crane-1", "railContactWidth", 'f'),
    ("craneRunways", "crane-1", "craneNu", 'f'),
    ("craneRunways", "crane-1", "craneKq", 'f'),
    ("craneRunways", "crane-1", "damageExponent", 'f'),
    ("craneRunways", "crane-1", "stressRange", 'f'),
    ("craneRunways", "crane-1", "detailCategoryStress", 'f'),
    ("craneRunways", "crane-1", "fatiguePhi", 'f'),
];

#[test]
fn every_special_part_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in SPECIAL_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

#[test]
fn core_entity_leaves_resolve_to_one_typed_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = base();
    let cases = [
        (format!("materials[id={}].fy", before.materials[0].id), 123.0e6_f64),
        (format!("sections[id={}].area", before.sections[0].id), 0.0123),
        (format!("members[id={}].length", before.members[0].id), 9.0),
    ];
    for (path, number) in cases {
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value: number.to_value() }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

const ANNEX_LEAVES: &[(&str, &str, &str, char)] = &[
    ("siloShells", "silo-1", "ringSpacing", 'f'),
    ("siloShells", "silo-1", "ringArea", 'f'),
    ("siloShells", "silo-1", "ringInertia", 'f'),
    ("siloShells", "silo-1", "ringOffset", 'f'),
    ("platedPanels", "pp-1", "corrugationFlat", 'f'),
    ("platedPanels", "pp-1", "corrugationProjection", 'f'),
    ("platedPanels", "pp-1", "corrugationDepth", 'f'),
    ("platedPanels", "pp-1", "webHeight", 'f'),
    ("bridgeFatigue", "br-1", "deckThickness", 'f'),
    ("bridgeFatigue", "br-1", "troughSpacing", 'f'),
    ("bridgeFatigue", "br-1", "troughWidth", 'f'),
    ("bridgeFatigue", "br-1", "troughThickness", 'f'),
    ("bridgeFatigue", "br-1", "crossbeamSpacing", 'f'),
    ("bridgeFatigue", "br-1", "surfacingThickness", 'f'),
    ("bridgeFatigue", "br-1", "wheelLoad", 'f'),
    ("bridgeFatigue", "br-1", "wheelPatch", 'f'),
    ("coldFormedMembers", "cf-1", "purlinSpan", 'f'),
    ("coldFormedMembers", "cf-1", "purlinLoad", 'f'),
    ("coldFormedMembers", "cf-1", "purlinLateralRatio", 'f'),
];

#[test]
fn every_annex_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in ANNEX_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

const STIFFENED_LEAVES: &[(&str, &str, &str, char)] = &[
    ("platedPanels", "pp-1", "stiffenerSpacing", 'f'),
];

#[test]
fn every_stiffened_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in STIFFENED_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

const BUILTUPOUTLINE_LEAVES: &[(&str, &str, &str, char)] = &[
    ("builtUpMembers", "joint-ep", "chordShape", 's'),
    ("builtUpMembers", "joint-ep", "chordDepth", 'f'),
    ("builtUpMembers", "joint-ep", "chordWidth", 'f'),
    ("builtUpMembers", "joint-ep", "chordFlangeThickness", 'f'),
    ("builtUpMembers", "joint-ep", "chordWebThickness", 'f'),
];

#[test]
fn every_builtupoutline_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in BUILTUPOUTLINE_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

const V7_LEAVES: &[(&str, &str, &str, char)] = &[
    ("platedPanels", "pp-1", "stiffenerShape", 'c'),
    ("platedPanels", "pp-1", "stiffenerDepth", 'f'),
    ("platedPanels", "pp-1", "stiffenerThickness", 'f'),
    ("platedPanels", "pp-1", "stiffenerFlangeWidth", 'f'),
    ("platedPanels", "pp-1", "stiffenerFlangeThickness", 'f'),
    ("platedPanels", "pp-1", "lateralPressure", 'f'),
    ("platedPanels", "pp-1", "lateralPressureCoefficient", 'f'),
    ("members", "member-b1", "webMaterialId", 's'),
    ("members", "member-b1", "torsionLoad", 's'),
    ("members", "member-b1", "torquePosition", 'f'),
    ("fatigueDetails", "fat-1", "stressHistory", 's'),
];

#[test]
fn every_v7_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in V7_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'c' => 1.0_f64.to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

const V8_LEAVES: &[(&str, &str, &str, char)] = &[
    ("members", "member-b1", "warpingSupport", 'c'),
];

#[test]
fn every_v8_leaf_resolves_to_one_typed_leaf_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (collection, id, field, kind) in V8_LEAVES {
        let path = format!("{collection}[id={id}].{field}");
        let value = match kind {
            's' => "x".to_string().to_value(),
            'c' => 1.0_f64.to_value(),
            'u' => 3_u32.to_value(),
            _ => 123.0_f64.to_value(),
        };
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}

#[test]
fn the_choice_leaves_take_the_enum_token_a_choice_grip_writes() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = En1993Snapshot::special_parts_showcase();
    for (path, token) in [("platedPanels[id=pp-1].stiffenerShape", "Angle"), ("platedPanels[id=pp-1].stiffenerShape", "None"), ("members[id=member-b1].warpingSupport", "Cantilever"), ("members[id=member-b1].warpingSupport", "FixedFork")] {
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.to_string(), value: token.to_string().to_value() }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        apply_en1993_mutation(&before, &mutations[0]).unwrap_or_else(|error| panic!("{path}: {error:?}"));
    }
}

#[test]
    use semio_framework_value::ToValue;
    let before = base();
    let cases = [
    ];
    for (path, number) in cases {
    }
}