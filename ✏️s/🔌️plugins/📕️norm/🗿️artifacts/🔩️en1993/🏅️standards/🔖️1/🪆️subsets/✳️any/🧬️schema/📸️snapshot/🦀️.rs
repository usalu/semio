//! 🧬️ En1993 snapshot schema — complete steel-structure subject (SI base units).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};

use framework_schema::ArtifactSchema;

//#region 🧱️RowGroups
pub use semio_s_artifact_norm_en1993_model_rows_materials::*;
pub use semio_s_artifact_norm_en1993_model_rows_members::*;
pub use semio_s_artifact_norm_en1993_model_rows_actions::*;
pub use semio_s_artifact_norm_en1993_model_rows_joints::*;
pub use semio_s_artifact_norm_en1993_model_rows_fatigue_fire::*;
pub use semio_s_artifact_norm_en1993_model_rows_structures::*;
pub use semio_s_artifact_norm_en1993_model_rows_frame::*;
//#endregion 🧱️RowGroups


//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1993", layout = "lines")]
#[artifact_schema(id = "s.norm-eurocode-structural.en1993")]
pub struct En1993Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<SteelMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<SteelSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<SteelMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_cases: Vec<LoadCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub member_actions: Vec<MemberAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub joints: Vec<SteelJoint>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[dsl(table)]
    #[state(artifact)]
    pub fire_exposures: Vec<FireExposure>,
    #[dsl(table)]
    #[state(artifact)]
    pub cold_formed_members: Vec<ColdFormedMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub plated_panels: Vec<PlatedPanel>,
    #[dsl(table)]
    #[state(artifact)]
    pub silo_shells: Vec<SiloShell>,
    #[dsl(table)]
    #[state(artifact)]
    pub tension_components: Vec<TensionComponent>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridge_fatigue: Vec<BridgeFatigue>,
    #[dsl(table)]
    #[state(artifact)]
    pub tower_legs: Vec<TowerLeg>,
    #[dsl(table)]
    #[state(artifact)]
    pub piles: Vec<SteelPile>,
    #[dsl(table)]
    #[state(artifact)]
    pub crane_runways: Vec<CraneRunway>,
    #[dsl(table)]
    #[state(artifact)]
    pub storeys: Vec<Storey>,
    #[dsl(table)]
    #[state(artifact)]
    pub restraints: Vec<Restraint>,
    #[dsl(table)]
    #[state(artifact)]
    pub built_up_members: Vec<BuiltUpMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub weld_groups: Vec<WeldGroup>,
    #[dsl(table)]
    #[state(artifact)]
    pub parametric_sections: Vec<ParametricSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub hollow_joints: Vec<HollowJoint>,
    #[dsl(table)]
    #[state(artifact)]
    pub joint_rows: Vec<JointRow>,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_nodes: Vec<FrameNode>,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_bars: Vec<FrameBar>,
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub joint_plates: Vec<JointPlate>,
    #[dsl(table)]
    #[state(artifact)]
    pub foundation_blocks: Vec<FoundationBlock>,
    #[state(artifact)]
    pub reliability_class: ReliabilityClass,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_actions: Vec<FrameAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub frame_member_bases: Vec<FrameMemberBasis>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_stresses: Vec<FatigueStress>,
    #[dsl(table)]
    #[state(artifact)]
    pub buckling_lengths: Vec<BucklingLength>,
    #[dsl(table)]
    #[state(artifact)]
    pub forming_gains: Vec<FormingGain>,
    #[dsl(table)]
    #[state(artifact)]
    pub material_ductility: Vec<MaterialDuctility>,
    #[dsl(table)]
    #[state(artifact)]
    pub fastened_joints: Vec<FastenedJoint>,
}

/// 🌱 Initial subject of a live evaluation session: the worked example.
impl Default for En1993Snapshot {
    fn default() -> Self {
        Self::example(semio_s_plugin_norm_compliance::AnnexChoice::En)
    }
}

impl En1993Snapshot {
    /// 📸️ Reference EN 1993 subject under the explicitly chosen annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self { annex: annex, ..Self::compliant_heb240_frame() }
    }

    /// ✅️ Realistic multi-part compliant steel subject (DE-NA) exercising parts 1–6.
    pub fn compliant_heb240_frame() -> Self {
        Self {
            annex: AnnexChoice::De,
            reliability_class: ReliabilityClass::RC2,
            materials: vec![SteelMaterial {
                ramberg_osgood_n: 0.0,
                id: "mat-s355".into(),
                grade: "S355".into(),
                fy: 355.0e6,
                fu: 510.0e6,
                e_modulus: 210.0e9,
                g_modulus: 81.0e9,
                subgrade: "J2".into(),
                kind: MaterialKind::Carbon,
            }],
            sections: vec![catalogue_heb240()],
            members: vec![SteelMember {
                warping_support: WarpingSupport::Fork,
                web_material_id: String::new(),
                torsion_load: TorsionLoad::Point,
                torque_position: 0.5,
                stainless_forming: StainlessForming::Unset,
                design_temperature: 0.0,
                toughness_stress_ratio: 0.75,
                radiation_shift: 0.0,
                stress_shift: 0.0,
                margin_shift: 0.0,
                strain_rate_shift: 0.0,
                cold_forming_radius: 0.0,
                id: "member-b1".into(),
                label: "Beam-column B1".into(),
                member_type: SteelMemberType::BeamColumn,
                section_id: "sec-heb240".into(),
                material_id: "mat-s355".into(),
                length: 4.0,
                buckling_length_y: 4.0,
                buckling_length_z: 4.0,
                ltb_length: 4.0,
                ltb_restraint_spacing: 2.0,
                load_application: LoadApplication::ShearCenter,
                end_moment_ratio_psi: -1.0,
                moment_diagram: MomentDiagram::Linear,
                deflection_limit_ratio: 300.0,
                analysis: AnalysisKind::Elastic,
            }],
            load_cases: vec![
                LoadCase { id: "g-permanent".into(), name: "Permanent G".into(), kind: ActionKind::Permanent, category: "self".into() },
                LoadCase { id: "q-imposed".into(), name: "Imposed Q".into(), kind: ActionKind::Imposed, category: "office".into() },
                LoadCase { id: "w-wind".into(), name: "Wind".into(), kind: ActionKind::Wind, category: "wind".into() },
            ],
            member_actions: vec![
                MemberAction {
                    id: "act-b1-g".into(),
                    member_id: "member-b1".into(),
                    load_case_id: "g-permanent".into(),
                    action: DesignAction { n: 100_000.0, vy: 0.0, vz: 20_000.0, my: 40_000.0, mz: 0.0, t: 0.0 },
                },
                MemberAction {
                    id: "act-b1-q".into(),
                    member_id: "member-b1".into(),
                    load_case_id: "q-imposed".into(),
                    action: DesignAction { n: 43_333.0, vy: 0.0, vz: 15_333.0, my: 17_333.0, mz: 0.0, t: 0.0 },
                },
            ],
            joints: vec![SteelJoint {
                plate_fy: 0.0,
                plate_width: 0.0,
                bolt_to_web: 0.0,
                extension_to_flange: 0.0,
                extension_edge: 0.0,
                alpha_factor: 0.0,
                bolt_stress_area: 0.0,
                bolt_grip_length: 0.0,
                column_flange_thickness: 0.0,
                column_web_thickness: 0.0,
                column_web_depth: 0.0,
                column_web_shear_area: 0.0,
                column_root_radius: 0.0,
                column_fy: 0.0,
                column_web_stress: 0.0,
                beam_flange_thickness: 0.0,
                compression_lever: 0.0,
                moment_demand: 0.0,
                beam_moment_resistance: 0.0,
                column_moment_resistance: 0.0,
                beam_inertia: 0.0,
                beam_span: 0.0,
                braced_frame: 0,
                stiffness_ratio: 0.0,
                transformation_parameter: 0.0,
                assumed_class: StiffnessClass::Unset,
                base_plate_length: 0.0,
                concrete_fcd: 0.0,
                concrete_modulus: 0.0,
                concrete_area_a0: 0.0,
                concrete_area_a1: 0.0,
                joint_coefficient: 0.0,
                anchor_area: 0.0,
                anchor_length: 0.0,
                base_normal_force: 0.0,
                base_moment: 0.0,
                base_lever_left: 0.0,
                base_lever_right: 0.0,
                beam_web_thickness: 0.0,
                beam_web_fy: 0.0,
                weld_force_longitudinal: 0.0,
                weld_force_transverse: 0.0,
                id: "joint-j1".into(),
                kind: JointKind::Bolted,
                member_id: "member-b1".into(),
                bolt_class: "8.8".into(),
                bolt_diameter: 0.020,
                bolt_rows: 2,
                bolts_per_row: 2,
                pitch: 0.060,
                gauge: 0.060,
                end_distance: 0.040,
                edge_distance: 0.040,
                shear_planes: 1,
                plate_thickness: 0.010,
                plate_fu: 510.0e6,
                weld_throat: 0.0,
                weld_length: 0.0,
                weld_fu: 510.0e6,
                weld_grade: "S355".into(),
                actions: vec![
                    JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 30_000.0, tension: 0.0 },
                    JointForceAction { id: "jf-q".into(), load_case_id: "q-imposed".into(), shear: 23_333.0, tension: 0.0 },
                ],
                category: JointCategory::A,
                friction_mu: 0.50,
                preload_force: 0.0,
                slip_factor_ks: 1.0,
                friction_surfaces: 1,
            }],
            fatigue_details: vec![FatigueDetail {
                stress_history: String::new(),
                consequence: FatigueConsequence::Low,
                id: "fat-1".into(),
                member_id: "member-b1".into(),
                category: 71,
                method: FatigueMethod::DamageTolerant,
                spectrum: vec![
                    FatigueBand { id: "fb-1".into(), delta_sigma: 50.0e6, cycles: 1.0e6 },
                    FatigueBand { id: "fb-2".into(), delta_sigma: 40.0e6, cycles: 1.0e6 },
                ],
            }],
            fire_exposures: vec![FireExposure {
                id: "fire-1".into(),
                member_id: "member-b1".into(),
                rating: FireRating::R60,
                protection_thickness: 0.020,
                section_factor: 150.0,
                mu0: 0.50,
                design_temperature: 550.0,
                protection_conductivity: 0.20,
                protection_density: 800.0,
                protection_specific_heat: 1700.0,
            }],
            cold_formed_members: vec![ColdFormedMember {
                purlin_span: 0.0,
                purlin_load: 0.0,
                purlin_lateral_ratio: 0.0,
                flange_width: 0.0,
                lip_length: 0.0,
                web_height: 0.0,
                web_slope: 90.0,
                opposite_flange_width: 0.0,
                spring_factor: 1.0,
                corner_radius: 0.0,
                flange_stress: 0.0,
                shear_force: 0.0,
                support_reaction: 0.0,
                support_length: 0.0,
                crippling_category: 2,
                stiffened_support: 0,
                bending_moment: 0.0,
                moment_resistance: 0.0,
                sheeting_stiffness: 0.0,
                required_restraint_stiffness: 0.0,
                id: "cf-1".into(),
                b_bar: 0.080,
                thickness: 0.003,
                k_sigma: 4.0,
                psi: 1.0,
                fy: 350.0e6,
                gross_resistance: 80_000.0,
                actions: vec![
                    ForceAction { id: "cf-g".into(), load_case_id: "g-permanent".into(), force: 15_000.0 },
                    ForceAction { id: "cf-q".into(), load_case_id: "q-imposed".into(), force: 10_000.0 },
                ],
            }],
            plated_panels: vec![PlatedPanel {
                stiffener_shape: StiffenerShape::None,
                stiffener_depth: 0.0,
                stiffener_thickness: 0.0,
                stiffener_flange_width: 0.0,
                stiffener_flange_thickness: 0.0,
                lateral_pressure: 0.0,
                lateral_pressure_coefficient: 0.0,
                steel_grade: "".into(),
                steel_subgrade: "".into(),
                design_temperature: 0.0,
                stiffener_spacing: 0.0,
                corrugation_flat: 0.0,
                corrugation_projection: 0.0,
                corrugation_depth: 0.0,
                web_height: 0.0,
                stiffener_inertia: 0.0,
                stiffener_area: 0.0,
                stiffener_eccentricity: 0.0,
                stiffener_alpha: 0.34,
                compression_stress: 0.0,
                half_width: 0.0,
                equivalent_span: 0.0,
                hogging: 0,
                patch_type: PatchLoadType::Unset,
                patch_force: 0.0,
                patch_length: 0.0,
                patch_offset: 0.0,
                flange_thickness: 0.0,
                flange_width: 0.0,
                flange_fy: 0.0,
                longitudinal_offset: 0.0,
                longitudinal_inertia: 0.0,
                patch_moment: 0.0,
                patch_moment_resistance: 0.0,
                bending_moment: 0.0,
                plastic_moment: 0.0,
                flange_moment: 0.0,
                shear_force: 0.0,
                transverse_stiffener_inertia: 0.0,
                flange_induced_k: 0.0,
                flange_area: 0.0,
                stress_x: 0.0,
                stress_z: 0.0,
                shear_stress: 0.0,
                alpha_cr: 0.0,
                id: "pp-1".into(),
                a: 1.2,
                b: 0.6,
                thickness: 0.012,
                fy: 355.0e6,
                k_sigma: 4.0,
                actions: vec![
                    ForceAction { id: "pp-g".into(), load_case_id: "g-permanent".into(), force: 80.0e6 },
                    ForceAction { id: "pp-q".into(), load_case_id: "q-imposed".into(), force: 40.0e6 },
                ],
            }],
            silo_shells: vec![SiloShell {
                ring_spacing: 0.0,
                ring_area: 0.0,
                ring_inertia: 0.0,
                ring_offset: 0.0,
                length: 0.0,
                fabrication_class: FabricationToleranceClass::Unset,
                meridional_stress: 0.0,
                hoop_stress: 0.0,
                shell_shear_stress: 0.0,
                boundary_factor_axial: 1.0,
                boundary_factor_hoop: 0.0,
                boundary_factor_shear: 0.0,
                wall_friction: 0.0,
                plan_area: 0.0,
                plan_perimeter: 0.0,
                liquid_density: 0.0,
                liquid_height: 0.0,
                course_elevation: 0.0,
                internal_pressure: 0.0,
                external_pressure: 0.0,
                temperature_change: 0.0,
                id: "silo-1".into(),
                thickness: 0.008,
                radius: 3.0,
                depth: 6.0,
                k: 0.5,
                gamma: 9_000.0,
                fy: 235.0e6,
            }],
            tension_components: vec![TensionComponent {
                partial_factor: 0.0,
                cable_length: 0.0,
                cable_weight: 0.0,
                cable_horizontal_force: 0.0,
                cable_modulus: 0.0,
                cable_stress: 0.0,
                cable_unit_weight: 0.0,
                thread_factor: 0.0,
                rod_stress_area: 0.0,
                rod_fu: 0.0,
                pin_diameter: 0.0,
                pin_plate_thickness: 0.0,
                pin_gap: 0.0,
                pin_outer_thickness: 0.0,
                pin_inner_thickness: 0.0,
                pin_fup: 0.0,
                pin_fyp: 0.0,
                pin_plate_fy: 0.0,
                id: "ten-1".into(),
                f_uk: 650_000.0,
                f_k: 480_000.0,
                actions: vec![
                    ForceAction { id: "ten-g".into(), load_case_id: "g-permanent".into(), force: 120_000.0 },
                    ForceAction { id: "ten-q".into(), load_case_id: "q-imposed".into(), force: 80_000.0 },
                ],
            }],
            bridge_fatigue: vec![BridgeFatigue {
                deck_thickness: 0.0,
                trough_spacing: 0.0,
                trough_width: 0.0,
                trough_thickness: 0.0,
                crossbeam_spacing: 0.0,
                surfacing_thickness: 0.0,
                wheel_load: 0.0,
                wheel_patch: 0.0,
                consequence: FatigueConsequence::Low,
                heavy_vehicle_weight: 0.0,
                reference_weight: 0.0,
                observed_traffic: 0.0,
                design_life: 0.0,
                second_lane_traffic: 0.0,
                second_lane_weight: 0.0,
                lambda_1: 0.0,
                lambda_max: 0.0,
                detail_id: String::new(),
                plate_thickness: 0.0,
                id: "br-1".into(),
                member_id: "member-b1".into(),
                lambda: 1.0,
                phi2: 1.0,
                delta_sigma_p: 35.0e6,
                category: 71,
                method: FatigueMethod::DamageTolerant,
            }],
            tower_legs: vec![TowerLeg {
                lattice_kind: LatticeKind::Unset,
                slenderness: 0.0,
                diameter: 0.0,
                height: 0.0,
                bending_stiffness: 0.0,
                mass_per_length: 0.0,
                strouhal: 0.0,
                log_decrement: 0.0,
                air_density: 0.0,
                mean_wind_speed: 0.0,
                crossing_rate: 0.0,
                design_life: 0.0,
                k_factor: 0.0,
                kw_factor: 0.0,
                lateral_force_coefficient: 0.0,
                permitted_amplitude_ratio: 0.0,
                ring_mode: 0.0,
                shell_thickness: 0.0,
                guy_area: 0.0,
                guy_angle: 0.0,
                guy_length: 0.0,
                id: "tw-1".into(),
                member_id: "member-b1".into(),
                force_coefficient: 1.2,
                dynamic_factor: 1.0,
                actions: vec![
                    ForceAction { id: "tw-g".into(), load_case_id: "g-permanent".into(), force: 80_000.0 },
                    ForceAction { id: "tw-w".into(), load_case_id: "w-wind".into(), force: 40_000.0 },
                ],
            }],
            piles: vec![SteelPile {
                environment: SheetPileEnvironment::Unset,
                design_life: 0.0,
                wall_thickness: 0.0,
                interlock_bending_factor: 0.0,
                interlock_stiffness_factor: 0.0,
                section_modulus: 0.0,
                shear_area: 0.0,
                design_moment: 0.0,
                design_shear: 0.0,
                interlock_shear: 0.0,
                id: "pile-1".into(),
                section_id: "sec-heb240".into(),
                material_id: "mat-s355".into(),
                driving_stress: 200.0e6,
                embedded_length: 8.0,
                shaft_perimeter: 0.96,
                actions: vec![
                    ForceAction { id: "pile-g".into(), load_case_id: "g-permanent".into(), force: 200_000.0 },
                    ForceAction { id: "pile-q".into(), load_case_id: "q-imposed".into(), force: 100_000.0 },
                ],
            }],
            crane_runways: vec![CraneRunway {
                max_wheel_load: 0.0,
                load_factor: 0.0,
                rail_inertia: 0.0,
                rail_fixed: 0,
                flange_effective_width: 0.0,
                flange_thickness: 0.0,
                wheel_eccentricity: 0.0,
                torsion_constant: 0.0,
                longitudinal_stress: 0.0,
                rail_radius: 0.0,
                rail_contact_width: 0.0,
                crane_nu: 0.0,
                crane_kq: 0.0,
                damage_exponent: 0.0,
                stress_range: 0.0,
                detail_category_stress: 0.0,
                fatigue_phi: 0.0,
                id: "crane-1".into(),
                member_id: "member-b1".into(),
                wheel_contact_length: 0.10,
                dispersion: 0.05,
                web_thickness: 0.010,
                fy: 355.0e6,
                phi: 1.25,
                actions: vec![
                    ForceAction { id: "cr-g".into(), load_case_id: "g-permanent".into(), force: 20_000.0 },
                    ForceAction { id: "cr-q".into(), load_case_id: "q-imposed".into(), force: 30_000.0 },
                ],
            }],
            storeys: vec![],
            restraints: vec![],
            built_up_members: vec![],
            weld_groups: vec![],
            parametric_sections: vec![],
            hollow_joints: vec![],
            joint_rows: vec![],
            frame_nodes: vec![],
            frame_bars: vec![],
            placements: vec![],
            joint_plates: vec![],
            foundation_blocks: vec![],
            material_ductility: vec![],
            forming_gains: vec![],
            buckling_lengths: vec![],
            fatigue_stresses: vec![],
            frame_member_bases: vec![],
            fastened_joints: vec![],
            frame_actions: vec![],
        }
    }

    /// ❌️ Non-compliant multi-part subject with failures across parts.
    pub fn noncompliant_overloaded_frame() -> Self {
        let mut doc = Self::compliant_heb240_frame();
        if let Some(m) = doc.members.iter_mut().find(|m| m.id == "member-b1") {
            m.buckling_length_y = 12.0;
            m.buckling_length_z = 12.0;
            m.ltb_length = 12.0;
            m.ltb_restraint_spacing = 12.0;
            m.analysis = AnalysisKind::Plastic;
        }
        if let Some(act) = doc.member_actions.iter_mut().find(|a| a.id == "act-b1-g") {
            act.action.n = 2_100_000.0;
            act.action.my = 333_000.0;
            act.action.vz = 296_000.0;
        }
        if let Some(act) = doc.member_actions.iter_mut().find(|a| a.id == "act-b1-q") {
            act.action.n = 0.0;
            act.action.my = 0.0;
            act.action.vz = 0.0;
        }
        if let Some(j) = doc.joints.iter_mut().find(|j| j.id == "joint-j1") {
            j.actions = vec![
                JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 100_000.0, tension: 0.0 },
                JointForceAction { id: "jf-q".into(), load_case_id: "q-imposed".into(), shear: 66_667.0, tension: 0.0 },
            ];
            j.bolt_rows = 1;
            j.end_distance = 0.025;
        }
        if let Some(f) = doc.fire_exposures.iter_mut().find(|f| f.id == "fire-1") {
            f.mu0 = 0.85;
            f.protection_thickness = 0.008;
        }
        if let Some(fat) = doc.fatigue_details.iter_mut().find(|f| f.id == "fat-1") {
            fat.spectrum = vec![FatigueBand { id: "fb-1".into(), delta_sigma: 90.0e6, cycles: 2.0e6 }];
        }
        if let Some(cf) = doc.cold_formed_members.iter_mut().find(|c| c.id == "cf-1") {
            cf.actions = vec![ForceAction { id: "cf-g".into(), load_case_id: "g-permanent".into(), force: 120_000.0 }];
            cf.gross_resistance = 40_000.0;
        }
        if let Some(pp) = doc.plated_panels.iter_mut().find(|p| p.id == "pp-1") {
            pp.actions = vec![ForceAction { id: "pp-g".into(), load_case_id: "g-permanent".into(), force: 300.0e6 }];
            pp.thickness = 0.006;
        }
        if let Some(s) = doc.silo_shells.iter_mut().find(|s| s.id == "silo-1") {
            s.thickness = 0.003;
            s.depth = 20.0;
        }
        if let Some(t) = doc.tension_components.iter_mut().find(|t| t.id == "ten-1") {
            t.actions = vec![ForceAction { id: "ten-g".into(), load_case_id: "g-permanent".into(), force: 500_000.0 }];
            t.f_k = 200_000.0;
        }
        if let Some(b) = doc.bridge_fatigue.iter_mut().find(|b| b.id == "br-1") {
            b.delta_sigma_p = 90.0e6;
        }
        if let Some(tw) = doc.tower_legs.iter_mut().find(|t| t.id == "tw-1") {
            tw.actions = vec![
                ForceAction { id: "tw-g".into(), load_case_id: "g-permanent".into(), force: 1_500_000.0 },
                ForceAction { id: "tw-w".into(), load_case_id: "w-wind".into(), force: 800_000.0 },
            ];
        }
        if let Some(p) = doc.piles.iter_mut().find(|p| p.id == "pile-1") {
            p.driving_stress = 400.0e6;
            p.actions = vec![ForceAction { id: "pile-g".into(), load_case_id: "g-permanent".into(), force: 3_000_000.0 }];
        }
        if let Some(c) = doc.crane_runways.iter_mut().find(|c| c.id == "crane-1") {
            c.web_thickness = 0.004;
            c.actions = vec![ForceAction { id: "cr-q".into(), load_case_id: "q-imposed".into(), force: 200_000.0 }];
            c.phi = 1.35;
        }
        doc
    }

    /// 🏗 The portal of [`Self::frame_portal_showcase`] loaded by load cases instead of node loads: 20 kN/m permanent and 15 kN/m imposed on the beam and 12 kN wind at the left eaves, the member of its bars with the frame-derived force basis.
    pub fn frame_cases_showcase() -> Self {
        let mut doc = Self::frame_portal_showcase();
        doc.frame_nodes.iter_mut().for_each(|n| {
            n.load_x = 0.0;
            n.load_z = 0.0;
        });
        let line = |id: &str, case: &str, fz: f64| FrameAction { id: id.into(), load_case_id: case.into(), target_id: "bar-2".into(), kind: FrameActionKind::BarUniform, fx: 0.0, fz, my: 0.0, start_ratio: 0.0, end_ratio: 1.0 };
        doc.frame_actions = vec![
            line("fa-g-beam", "g-permanent", -20_000.0),
            line("fa-q-beam", "q-imposed", -15_000.0),
            FrameAction { id: "fa-w-eaves".into(), load_case_id: "w-wind".into(), target_id: "node-b".into(), kind: FrameActionKind::NodalForce, fx: 12_000.0, fz: 0.0, my: 0.0, start_ratio: 0.0, end_ratio: 1.0 },
        ];
        doc.frame_member_bases = vec![FrameMemberBasis { id: "basis-b1".into(), member_id: "member-b1".into(), basis: MemberForceBasis::FrameDerived }];
        doc
    }

    /// 🕸 Compliant frame extended by a single-bay portal of general frame nodes and bars (fixed bases, 6 m span, 4 m height, 200 kN on each eaves node and 20 kN horizontal): the subject of the frame eigenvalue tests.
    pub fn frame_portal_showcase() -> Self {
        let mut doc = Self::compliant_heb240_frame();
        doc.frame_nodes.push(FrameNode { id: "node-a".into(), label: "Base left".into(), x: 0.0, z: 0.0, support: FrameNodeSupport::Fixed, load_x: 0.0, load_z: 0.0 });
        doc.frame_nodes.push(FrameNode { id: "node-b".into(), label: "Eaves left".into(), x: 0.0, z: 4.0, support: FrameNodeSupport::Free, load_x: 20000.0, load_z: -200000.0 });
        doc.frame_nodes.push(FrameNode { id: "node-c".into(), label: "Eaves right".into(), x: 6.0, z: 4.0, support: FrameNodeSupport::Free, load_x: 0.0, load_z: -200000.0 });
        doc.frame_nodes.push(FrameNode { id: "node-d".into(), label: "Base right".into(), x: 6.0, z: 0.0, support: FrameNodeSupport::Fixed, load_x: 0.0, load_z: 0.0 });
        doc.frame_bars.push(FrameBar { id: "bar-1".into(), member_id: "member-b1".into(), label: "Left column".into(), start_node_id: "node-a".into(), end_node_id: "node-b".into(), release_start: FrameBarRelease::Rigid, release_end: FrameBarRelease::Rigid });
        doc.frame_bars.push(FrameBar { id: "bar-2".into(), member_id: "member-b1".into(), label: "Beam".into(), start_node_id: "node-b".into(), end_node_id: "node-c".into(), release_start: FrameBarRelease::Rigid, release_end: FrameBarRelease::Rigid });
        doc.frame_bars.push(FrameBar { id: "bar-3".into(), member_id: "member-b1".into(), label: "Right column".into(), start_node_id: "node-d".into(), end_node_id: "node-c".into(), release_start: FrameBarRelease::Rigid, release_end: FrameBarRelease::Rigid });
        doc
    }

    /// 🏛️ Special-parts showcase extended by one populated row of every annex block: a ring-stiffened cylinder under external pressure (`ring-1`), a corrugated web (`corr-1`), an orthotropic deck (`deck-1`) and a purlin restrained by sheeting (`purlin-1`).
    pub fn annex_showcase() -> Self {
        let mut doc = Self::special_parts_showcase();
        let mut ring = doc.silo_shells[0].clone();
        ring.id = "ring-1".into();
        ring.radius = 1.0;
        ring.thickness = 0.008;
        ring.fy = 355.0e6;
        ring.length = 3.0;
        ring.fabrication_class = FabricationToleranceClass::B;
        ring.boundary_factor_hoop = 1.0;
        ring.external_pressure = 0.5e6;
        ring.internal_pressure = 0.0;
        ring.meridional_stress = 0.0;
        ring.hoop_stress = 0.0;
        ring.shell_shear_stress = 0.0;
        ring.wall_friction = 0.0;
        ring.plan_area = 0.0;
        ring.plan_perimeter = 0.0;
        ring.liquid_density = 0.0;
        ring.liquid_height = 0.0;
        ring.course_elevation = 0.0;
        ring.temperature_change = 0.0;
        ring.ring_spacing = 1.5;
        ring.ring_area = 0.0015;
        ring.ring_inertia = 2.0e-6;
        ring.ring_offset = 0.08;
        doc.silo_shells.push(ring);
        let mut web = doc.plated_panels[0].clone();
        web.id = "corr-1".into();
        web.thickness = 0.002;
        web.fy = 355.0e6;
        web.shear_force = 30_000.0;
        web.corrugation_flat = 0.14;
        web.corrugation_projection = 0.04;
        web.corrugation_depth = 0.05;
        web.web_height = 1.0;
        doc.plated_panels.push(web);
        let mut deck = doc.bridge_fatigue[0].clone();
        deck.id = "deck-1".into();
        deck.heavy_vehicle_weight = 400_000.0;
        deck.reference_weight = 480_000.0;
        deck.observed_traffic = 0.5e6;
        deck.design_life = 100.0;
        deck.second_lane_traffic = 0.0;
        deck.second_lane_weight = 0.0;
        deck.lambda_1 = 1.0;
        deck.lambda_max = 2.0;
        deck.phi2 = 1.2;
        deck.category = 71;
        deck.detail_id = "rib-to-deck-partial-penetration-plate".into();
        deck.plate_thickness = 0.014;
        deck.deck_thickness = 0.014;
        deck.trough_spacing = 0.6;
        deck.trough_width = 0.3;
        deck.trough_thickness = 0.008;
        deck.crossbeam_spacing = 3.0;
        deck.surfacing_thickness = 0.07;
        deck.wheel_load = 60_000.0;
        deck.wheel_patch = 0.4;
        doc.bridge_fatigue.push(deck);
        let mut purlin = doc.cold_formed_members[0].clone();
        purlin.id = "purlin-1".into();
        purlin.thickness = 0.002;
        purlin.fy = 350.0e6;
        purlin.flange_width = 0.06;
        purlin.lip_length = 0.02;
        purlin.web_height = 0.2;
        purlin.sheeting_stiffness = 4000.0;
        purlin.moment_resistance = 15000.0;
        purlin.purlin_span = 6.0;
        purlin.purlin_load = 2500.0;
        purlin.purlin_lateral_ratio = 0.15;
        doc.cold_formed_members.push(purlin);
        let mut stiffened = doc.plated_panels[0].clone();
        stiffened.id = "stiff-1".into();
        stiffened.a = 3.0;
        stiffened.b = 1.5;
        stiffened.thickness = 0.010;
        stiffened.fy = 355.0e6;
        stiffened.compression_stress = 100.0e6;
        stiffened.stiffener_area = 0.001;
        stiffened.stiffener_inertia = 8.333_333_333_333_334e-7;
        stiffened.stiffener_eccentricity = 0.055;
        stiffened.stiffener_spacing = 0.375;
        stiffened.alpha_cr = 0.0;
        doc.plated_panels.push(stiffened);
        doc
    }

    /// 🔭️ Compliant frame extended by one populated row of every special part of EN 1993-1-3 to 6 and 1-10: the subject of the wiring tests.
    pub fn special_parts_showcase() -> Self {
        let mut doc = Self::compliant_heb240_frame();
        doc.storeys.push(Storey { id: "storey-1".into(), member_id: "member-b1".into(), label: "Storey 1".into(), height: 4.0, vertical_load: 400000.0, horizontal_load: 20000.0, drift: 0.002, columns: 2, plastic_analysis: 0, building_height: 4.0, bracing_load: 0.0, bracing_length: 0.0, bracing_deflection: 0.0 });
        doc.restraints.push(Restraint { id: "restraint-1".into(), member_id: "member-b1".into(), label: "Restraint 1".into(), start_support: RestraintSupport::Fork, end_support: RestraintSupport::Fork, intermediate_position: 0.0, torsional_spring: 0.0, lateral_spring_position: 0.0, lateral_spring_stiffness: 0.0, load_height: 0.0, load_distribution: RestraintDistribution::Udl, design_moment: 100000.0, design_axial: 0.0, general_method: 0, hinge_to_restraint: 0.0 });
        doc.built_up_members.push(BuiltUpMember { chord_shape: ChordShape::Channel, chord_depth: 0.3, chord_width: 0.1, chord_flange_thickness: 0.0165, chord_web_thickness: 0.01, id: "built-up-1".into(), member_id: "member-b1".into(), label: "Built-up 1".into(), kind: BuiltUpKind::Battened, chord_area: 0.00538, chord_inertia: 1.336e-05, chord_distance: 0.3, panel_spacing: 0.8, planes: 2, batten_inertia: 5e-06, diagonal_area: 0.0, diagonal_length: 0.0, design_axial: 400000.0, design_moment: 5000.0, back_to_back: 0, min_radius: 0.0, chain_spacing: 0.0 });
        doc.weld_groups.push(WeldGroup { id: "weld-group-1".into(), member_id: "member-b1".into(), label: "Weld group 1".into(), pattern: WeldPattern::TwoLines, throat: 0.005, width: 0.1, depth: 0.2, shear_force: 100000.0, eccentricity: 0.15, steel_fu: 510000000.0, beta_w: 0.9 });
        doc.parametric_sections.push(ParametricSection { id: "param-1".into(), section_id: "sec-heb240".into(), label: "Parametric HEB 240".into(), shape: ParametricShape::WeldedI, height: 0.24, width: 0.24, web_thickness: 0.01, flange_thickness: 0.017, tolerance: 0.05 });
        doc.hollow_joints.push(HollowJoint { id: "hollow-1".into(), member_id: "member-b1".into(), label: "K gap joint".into(), chord_width: 0.12, chord_height: 0.12, chord_thickness: 0.008, chord_fy: 355000000.0, chord_stress: 0.0, brace1_width: 0.08, brace1_height: 0.08, brace1_thickness: 0.005, brace1_angle: 45.0, brace1_force: 60000.0, brace2_width: 0.08, brace2_height: 0.08, brace2_thickness: 0.005, brace2_angle: 45.0, brace2_force: 60000.0, gap: 0.03, brace_fy: 355000000.0 });
        doc.joint_rows.push(JointRow { id: "row-1".into(), joint_id: "joint-ep".into(), label: "Row 1 (extended)".into(), row_type: JointRowType::Extended, lever: 0.27, bolt_to_web: 0.04, edge_distance: 0.04, extension_to_flange: 0.04, extension_edge: 0.04, alpha_factor: 6.5, pitch_to_previous: 0.0, bolts: 2, bolt_diameter: 0.02, bolt_stress_area: 0.000245, grip_length: 0.05, bolt_class: "10.9".into(), group_with_previous: 0 });
        doc.joint_rows.push(JointRow { id: "row-2".into(), joint_id: "joint-ep".into(), label: "Row 2 (first below the flange)".into(), row_type: JointRowType::FirstBelow, lever: 0.18, bolt_to_web: 0.04, edge_distance: 0.04, extension_to_flange: 0.0, extension_edge: 0.0, alpha_factor: 6.5, pitch_to_previous: 0.09, bolts: 2, bolt_diameter: 0.02, bolt_stress_area: 0.000245, grip_length: 0.05, bolt_class: "10.9".into(), group_with_previous: 0 });
        if let Some(row) = doc.members.iter_mut().find(|r| r.id == "member-b1") {
            row.stainless_forming = StainlessForming::Unset;
            row.design_temperature = -20.0;
            row.toughness_stress_ratio = 0.75;
            row.radiation_shift = 0.0;
            row.stress_shift = 0.0;
            row.margin_shift = 0.0;
            row.strain_rate_shift = 0.0;
            row.cold_forming_radius = 0.0;
        }
        if let Some(row) = doc.cold_formed_members.iter_mut().find(|r| r.id == "cf-1") {
            row.flange_width = 0.06;
            row.lip_length = 0.02;
            row.web_height = 0.15;
            row.web_slope = 90.0;
            row.opposite_flange_width = 0.06;
            row.spring_factor = 1.0;
            row.corner_radius = 0.004;
            row.flange_stress = 150000000.0;
            row.shear_force = 6000.0;
            row.support_reaction = 4000.0;
            row.support_length = 0.05;
            row.crippling_category = 2;
            row.stiffened_support = 0;
            row.bending_moment = 800.0;
            row.moment_resistance = 3000.0;
            row.sheeting_stiffness = 4000.0;
            row.required_restraint_stiffness = 1000.0;
        }
        if let Some(row) = doc.plated_panels.iter_mut().find(|r| r.id == "pp-1") {
            row.stiffener_inertia = 4e-07;
            row.stiffener_area = 0.0012;
            row.stiffener_eccentricity = 0.06;
            row.stiffener_alpha = 0.34;
            row.compression_stress = 100000000.0;
            row.half_width = 0.3;
            row.equivalent_span = 6.0;
            row.hogging = 0;
            row.patch_type = PatchLoadType::Through;
            row.patch_force = 150000.0;
            row.patch_length = 0.1;
            row.patch_offset = 0.0;
            row.flange_thickness = 0.02;
            row.flange_width = 0.3;
            row.flange_fy = 355000000.0;
            row.longitudinal_offset = 0.0;
            row.longitudinal_inertia = 0.0;
            row.patch_moment = 400000.0;
            row.patch_moment_resistance = 1500000.0;
            row.bending_moment = 600000.0;
            row.plastic_moment = 2000000.0;
            row.flange_moment = 1500000.0;
            row.shear_force = 400000.0;
            row.transverse_stiffener_inertia = 6e-06;
            row.stiffener_shape = StiffenerShape::Tee;
            row.stiffener_depth = 0.12;
            row.stiffener_thickness = 0.01;
            row.stiffener_flange_width = 0.08;
            row.stiffener_flange_thickness = 0.012;
            row.lateral_pressure = 20000.0;
            row.lateral_pressure_coefficient = 0.5;
            row.flange_induced_k = 0.55;
            row.flange_area = 0.006;
            row.stress_x = 120000000.0;
            row.stress_z = 30000000.0;
            row.shear_stress = 40000000.0;
            row.alpha_cr = 3.0;
        }
        if let Some(row) = doc.silo_shells.iter_mut().find(|r| r.id == "silo-1") {
            row.length = 6.0;
            row.fabrication_class = FabricationToleranceClass::B;
            row.meridional_stress = 40000000.0;
            row.hoop_stress = 30000000.0;
            row.shell_shear_stress = 10000000.0;
            row.boundary_factor_axial = 1.0;
            row.boundary_factor_hoop = 1.0;
            row.boundary_factor_shear = 1.0;
            row.wall_friction = 0.4;
            row.plan_area = 28.3;
            row.plan_perimeter = 18.8;
            row.liquid_density = 1000.0;
            row.liquid_height = 6.0;
            row.course_elevation = 2.0;
            row.internal_pressure = 0.0;
            row.external_pressure = 0.0;
            row.temperature_change = 0.0;
        }
        if let Some(row) = doc.tension_components.iter_mut().find(|r| r.id == "ten-1") {
            row.partial_factor = 1.5;
            row.cable_length = 30.0;
            row.cable_weight = 120.0;
            row.cable_horizontal_force = 100000.0;
            row.cable_modulus = 160000000000.0;
            row.cable_stress = 400000000.0;
            row.cable_unit_weight = 80000.0;
            row.thread_factor = 0.9;
            row.rod_stress_area = 0.000353;
            row.rod_fu = 800000000.0;
            row.pin_diameter = 0.04;
            row.pin_plate_thickness = 0.025;
            row.pin_gap = 0.002;
            row.pin_outer_thickness = 0.02;
            row.pin_inner_thickness = 0.025;
            row.pin_fup = 800000000.0;
            row.pin_fyp = 640000000.0;
            row.pin_plate_fy = 355000000.0;
        }
        if let Some(row) = doc.bridge_fatigue.iter_mut().find(|r| r.id == "br-1") {
            row.heavy_vehicle_weight = 445000.0;
            row.reference_weight = 480000.0;
            row.observed_traffic = 500000.0;
            row.design_life = 100.0;
            row.second_lane_traffic = 100000.0;
            row.second_lane_weight = 400000.0;
            row.lambda_1 = 1.6;
            row.lambda_max = 2.0;
            row.detail_id = String::new();
            row.plate_thickness = 0.014;
        }
        if let Some(row) = doc.tower_legs.iter_mut().find(|r| r.id == "tw-1") {
            row.lattice_kind = LatticeKind::Leg;
            row.slenderness = 80.0;
            row.diameter = 0.8;
            row.height = 30.0;
            row.bending_stiffness = 4000000000.0;
            row.mass_per_length = 150.0;
            row.strouhal = 0.2;
            row.log_decrement = 0.05;
            row.air_density = 1.25;
            row.mean_wind_speed = 8.0;
            row.crossing_rate = 0.3;
            row.design_life = 50.0;
            row.k_factor = 0.13;
            row.kw_factor = 0.6;
            row.lateral_force_coefficient = 0.5;
            row.permitted_amplitude_ratio = 0.1;
            row.ring_mode = 2.0;
            row.shell_thickness = 0.01;
            row.guy_area = 0.0005;
            row.guy_angle = 45.0;
            row.guy_length = 40.0;
        }
        if let Some(row) = doc.piles.iter_mut().find(|r| r.id == "pile-1") {
            row.environment = SheetPileEnvironment::PollutedSoil;
            row.design_life = 50.0;
            row.wall_thickness = 0.012;
            row.interlock_bending_factor = 0.9;
            row.interlock_stiffness_factor = 0.8;
            row.section_modulus = 0.0018;
            row.shear_area = 0.004;
            row.design_moment = 350000.0;
            row.design_shear = 120000.0;
            row.interlock_shear = 400000.0;
        }
        if let Some(row) = doc.crane_runways.iter_mut().find(|r| r.id == "crane-1") {
            row.max_wheel_load = 150000.0;
            row.load_factor = 1.35;
            row.rail_inertia = 1e-05;
            row.rail_fixed = 1;
            row.flange_effective_width = 0.2;
            row.flange_thickness = 0.02;
            row.wheel_eccentricity = 0.025;
            row.torsion_constant = 2e-06;
            row.longitudinal_stress = 60000000.0;
            row.rail_radius = 0.25;
            row.rail_contact_width = 0.05;
            row.crane_nu = 0.5;
            row.crane_kq = 0.25;
            row.damage_exponent = 3.0;
            row.stress_range = 40000000.0;
            row.detail_category_stress = 71000000.0;
            row.fatigue_phi = 1.1;
        }
        let template = doc.joints[0].clone();
        let mut end_plate = SteelJoint { plate_fy: 0.0, plate_width: 0.0, bolt_to_web: 0.0, extension_to_flange: 0.0, extension_edge: 0.0, alpha_factor: 0.0, bolt_stress_area: 0.0, bolt_grip_length: 0.0, column_flange_thickness: 0.0, column_web_thickness: 0.0, column_web_depth: 0.0, column_web_shear_area: 0.0, column_root_radius: 0.0, column_fy: 0.0, column_web_stress: 0.0, beam_flange_thickness: 0.0, compression_lever: 0.0, moment_demand: 0.0, beam_moment_resistance: 0.0, column_moment_resistance: 0.0, beam_inertia: 0.0, beam_span: 0.0, braced_frame: 0, stiffness_ratio: 0.0, transformation_parameter: 0.0, assumed_class: StiffnessClass::Unset, base_plate_length: 0.0, concrete_fcd: 0.0, concrete_modulus: 0.0, concrete_area_a0: 0.0, concrete_area_a1: 0.0, joint_coefficient: 0.0, anchor_area: 0.0, anchor_length: 0.0, base_normal_force: 0.0, base_moment: 0.0, base_lever_left: 0.0, base_lever_right: 0.0, beam_web_thickness: 0.0, beam_web_fy: 0.0, weld_force_longitudinal: 0.0, weld_force_transverse: 0.0, id: "joint-ep".into(), kind: JointKind::EndPlate, bolt_class: "10.9".into(), bolt_rows: 2, bolts_per_row: 2, pitch: 0.09, gauge: 0.11, end_distance: 0.04, edge_distance: 0.04, plate_thickness: 0.02, weld_throat: 0.006, ..template.clone() };
        end_plate.plate_fy = 355000000.0;
        end_plate.plate_width = 0.2;
        end_plate.bolt_to_web = 0.04;
        end_plate.extension_to_flange = 0.04;
        end_plate.extension_edge = 0.04;
        end_plate.alpha_factor = 6.5;
        end_plate.bolt_stress_area = 0.000245;
        end_plate.bolt_grip_length = 0.05;
        end_plate.column_flange_thickness = 0.017;
        end_plate.column_web_thickness = 0.01;
        end_plate.column_web_depth = 0.164;
        end_plate.column_web_shear_area = 0.0024;
        end_plate.column_root_radius = 0.021;
        end_plate.column_fy = 355000000.0;
        end_plate.column_web_stress = 50000000.0;
        end_plate.beam_flange_thickness = 0.0107;
        end_plate.compression_lever = 0.27;
        end_plate.moment_demand = 80000.0;
        end_plate.beam_moment_resistance = 110000.0;
        end_plate.column_moment_resistance = 200000.0;
        end_plate.beam_inertia = 8.356e-05;
        end_plate.beam_span = 6.0;
        end_plate.braced_frame = 1;
        end_plate.stiffness_ratio = 0.2;
        end_plate.transformation_parameter = 1.0;
        end_plate.assumed_class = StiffnessClass::Rigid;
        let mut base_plate = SteelJoint { plate_fy: 0.0, plate_width: 0.0, bolt_to_web: 0.0, extension_to_flange: 0.0, extension_edge: 0.0, alpha_factor: 0.0, bolt_stress_area: 0.0, bolt_grip_length: 0.0, column_flange_thickness: 0.0, column_web_thickness: 0.0, column_web_depth: 0.0, column_web_shear_area: 0.0, column_root_radius: 0.0, column_fy: 0.0, column_web_stress: 0.0, beam_flange_thickness: 0.0, compression_lever: 0.0, moment_demand: 0.0, beam_moment_resistance: 0.0, column_moment_resistance: 0.0, beam_inertia: 0.0, beam_span: 0.0, braced_frame: 0, stiffness_ratio: 0.0, transformation_parameter: 0.0, assumed_class: StiffnessClass::Unset, base_plate_length: 0.0, concrete_fcd: 0.0, concrete_modulus: 0.0, concrete_area_a0: 0.0, concrete_area_a1: 0.0, joint_coefficient: 0.0, anchor_area: 0.0, anchor_length: 0.0, base_normal_force: 0.0, base_moment: 0.0, base_lever_left: 0.0, base_lever_right: 0.0, beam_web_thickness: 0.0, beam_web_fy: 0.0, weld_force_longitudinal: 0.0, weld_force_transverse: 0.0, id: "joint-bp".into(), kind: JointKind::BasePlate, bolt_class: "8.8".into(), bolt_rows: 1, bolts_per_row: 2, edge_distance: 0.05, plate_thickness: 0.03, ..template.clone() };
        base_plate.plate_fy = 355000000.0;
        base_plate.plate_width = 0.3;
        base_plate.bolt_to_web = 0.05;
        base_plate.bolt_stress_area = 0.000245;
        base_plate.column_flange_thickness = 0.017;
        base_plate.column_web_thickness = 0.01;
        base_plate.column_fy = 355000000.0;
        base_plate.base_plate_length = 0.35;
        base_plate.concrete_fcd = 16700000.0;
        base_plate.concrete_modulus = 31000000000.0;
        base_plate.concrete_area_a0 = 0.098;
        base_plate.concrete_area_a1 = 0.5;
        base_plate.joint_coefficient = 0.667;
        base_plate.anchor_area = 0.000245;
        base_plate.anchor_length = 0.2;
        base_plate.base_normal_force = -300000.0;
        base_plate.base_moment = 60000.0;
        base_plate.base_lever_left = 0.15;
        base_plate.base_lever_right = 0.15;
        doc.joints.push(end_plate);
        doc.joints.push(base_plate);
        let mut splice = template.clone();
        splice.id = "joint-sp".into();
        splice.moment_demand = 80_000.0;
        splice.actions = vec![JointForceAction { id: "jf-sp".into(), load_case_id: "g-permanent".into(), shear: 30_000.0, tension: 0.0 }];
        doc.joints.push(splice);
        doc.joint_plates.push(JointPlate { id: "plate-fin".into(), joint_id: "joint-j1".into(), label: "Fin plate".into(), role: "finPlate".into(), plate_height: 0.24, plate_width: 0.1, plate_thickness: 0.01, plate_count: 1, plate_fy: 355000000.0, plate_fu: 470000000.0, bolt_class: "8.8".into(), bolt_diameter: 0.02, hole_diameter: 0.022, bolt_rows: 3, bolts_per_row: 1, pitch: 0.07, gauge: 0.0, end_distance: 0.04, edge_distance: 0.04, shear_planes: 1, eccentricity: 0.06, weld_throat: 0.005, weld_fu: 470000000.0, weld_beta_w: 0.9, group_method: BoltGroupMethod::Elastic });
        doc.joint_plates.push(JointPlate { id: "plate-flange".into(), joint_id: "joint-sp".into(), label: "Flange cover plate".into(), role: "flangeCover".into(), plate_height: 0.22, plate_width: 0.2, plate_thickness: 0.014, plate_count: 1, plate_fy: 355000000.0, plate_fu: 470000000.0, bolt_class: "8.8".into(), bolt_diameter: 0.02, hole_diameter: 0.022, bolt_rows: 3, bolts_per_row: 2, pitch: 0.07, gauge: 0.1, end_distance: 0.05, edge_distance: 0.05, shear_planes: 1, eccentricity: 0.0, weld_throat: 0.0, weld_fu: 0.0, weld_beta_w: 0.0, group_method: BoltGroupMethod::Elastic });
        doc.joint_plates.push(JointPlate { id: "plate-web".into(), joint_id: "joint-sp".into(), label: "Web cover plates".into(), role: "webCover".into(), plate_height: 0.18, plate_width: 0.1, plate_thickness: 0.008, plate_count: 2, plate_fy: 355000000.0, plate_fu: 470000000.0, bolt_class: "8.8".into(), bolt_diameter: 0.02, hole_diameter: 0.022, bolt_rows: 2, bolts_per_row: 1, pitch: 0.07, gauge: 0.0, end_distance: 0.04, edge_distance: 0.04, shear_planes: 2, eccentricity: 0.05, weld_throat: 0.0, weld_fu: 0.0, weld_beta_w: 0.0, group_method: BoltGroupMethod::Elastic });
        doc.foundation_blocks.push(FoundationBlock { id: "block-bp".into(), joint_id: "joint-bp".into(), label: "Foundation block".into(), block_length: 0.9, block_width: 0.9, block_depth: 0.8, concrete_fck: 30000000.0, grout_fck: 40000000.0, grout_thickness: 0.03, anchor_count: 2, anchor_class: "8.8".into(), anchor_diameter: 0.024, anchor_stress_area: 0.000353, anchor_embedment: 0.4, anchor_edge_distance: 0.2 });
        let stainless_steel = SteelMaterial { ramberg_osgood_n: 0.0, id: "mat-1.4301".into(), grade: "1.4301".into(), fy: 210.0e6, fu: 520.0e6, e_modulus: 200.0e9, g_modulus: 77.0e9, subgrade: String::new(), kind: MaterialKind::Stainless };
        let mut stainless_member = doc.members[0].clone();
        stainless_member.id = "member-ss".into();
        stainless_member.label = "Stainless beam-column".into();
        stainless_member.material_id = stainless_steel.id.clone();
        stainless_member.stainless_forming = StainlessForming::ColdFormedOpen;
        doc.member_actions.push(MemberAction { id: "act-ss-g".into(), member_id: stainless_member.id.clone(), load_case_id: "g-permanent".into(), action: DesignAction { n: 40_000.0, vy: 0.0, vz: 10_000.0, my: 15_000.0, mz: 0.0, t: 0.0 } });
        doc.fire_exposures.push(FireExposure { id: "fire-ss".into(), member_id: stainless_member.id.clone(), mu0: 0.30, design_temperature: 550.0, ..doc.fire_exposures[0].clone() });
        let mut pipeline = doc.silo_shells[0].clone();
        pipeline.id = "pipe-1".into();
        pipeline.radius = 0.3;
        pipeline.thickness = 0.012;
        pipeline.depth = 1.0;
        pipeline.length = 0.0;
        pipeline.fabrication_class = FabricationToleranceClass::Unset;
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
}

pub fn catalogue_heb240() -> SteelSection {
    SteelSection {
        id: "sec-heb240".into(),
        designation: "HEB 240".into(),
        kind: SectionKind::RolledI,
        h: 0.240,
        b: 0.240,
        tw: 0.010,
        tf: 0.017,
        r: 0.021,
        area: 0.0106,
        shear_area_y: 0.00512,
        shear_area_z: 0.00240,
        iy: 1.126e-4,
        iz: 3.923e-5,
        it: 1.037e-6,
        iw: 4.762e-7,
        w_el_y: 9.38e-4,
        w_el_z: 3.27e-4,
        w_pl_y: 1.053e-3,
        w_pl_z: 4.985e-4,
        area_net: 0.0095,
        fabrication: HollowFabrication::Unset,
    }
}

/// 📒 HEB 260 rolled I-section (next larger after HEB 240).
pub fn catalogue_heb260() -> SteelSection {
    SteelSection {
        id: "sec-heb260".into(),
        designation: "HEB 260".into(),
        kind: SectionKind::RolledI,
        h: 0.260,
        b: 0.260,
        tw: 0.010,
        tf: 0.0175,
        r: 0.024,
        area: 0.0118,
        shear_area_y: 0.00570,
        shear_area_z: 0.00260,
        iy: 1.492e-4,
        iz: 5.135e-5,
        it: 1.259e-6,
        iw: 7.362e-7,
        w_el_y: 1.148e-3,
        w_el_z: 3.95e-4,
        w_pl_y: 1.283e-3,
        w_pl_z: 6.02e-4,
        area_net: 0.0106,
        fabrication: HollowFabrication::Unset,
    }
}

/// 📒 HEB 280 rolled I-section.
pub fn catalogue_heb280() -> SteelSection {
    SteelSection {
        id: "sec-heb280".into(),
        designation: "HEB 280".into(),
        kind: SectionKind::RolledI,
        h: 0.280,
        b: 0.280,
        tw: 0.0105,
        tf: 0.018,
        r: 0.024,
        area: 0.0131,
        shear_area_y: 0.00635,
        shear_area_z: 0.00294,
        iy: 1.926e-4,
        iz: 6.595e-5,
        it: 1.454e-6,
        iw: 1.107e-6,
        w_el_y: 1.376e-3,
        w_el_z: 4.71e-4,
        w_pl_y: 1.532e-3,
        w_pl_z: 7.17e-4,
        area_net: 0.0118,
        fabrication: HollowFabrication::Unset,
    }
}

/// 📒 HEB 300 rolled I-section.
pub fn catalogue_heb300() -> SteelSection {
    SteelSection {
        id: "sec-heb300".into(),
        designation: "HEB 300".into(),
        kind: SectionKind::RolledI,
        h: 0.300,
        b: 0.300,
        tw: 0.011,
        tf: 0.019,
        r: 0.027,
        area: 0.0149,
        shear_area_y: 0.00720,
        shear_area_z: 0.00330,
        iy: 2.517e-4,
        iz: 8.563e-5,
        it: 1.877e-6,
        iw: 1.651e-6,
        w_el_y: 1.678e-3,
        w_el_z: 5.71e-4,
        w_pl_y: 1.869e-3,
        w_pl_z: 8.70e-4,
        area_net: 0.0134,
        fabrication: HollowFabrication::Unset,
    }
}

/// 📒 Ordered HEB catalogue for remediation section upsizing.
pub fn rolled_heb_catalogue() -> Vec<SteelSection> {
    vec![catalogue_heb240(), catalogue_heb260(), catalogue_heb280(), catalogue_heb300()]
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1993_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1993_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1993_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1993_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    semio_s_plugin_norm_app_surface::artifact_kind_spec("en1993", "EN 1993", EN1993_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1993_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm-eurocode-structural.en1993", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1993_DOCUMENT_SCHEMA: &str = "semio.norm.en1993/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;