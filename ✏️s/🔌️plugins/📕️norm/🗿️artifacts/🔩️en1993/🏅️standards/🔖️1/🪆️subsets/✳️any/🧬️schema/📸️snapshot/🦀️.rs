//! 🧬️ En1993 snapshot schema — complete steel-structure subject (SI base units).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1993", layout = "lines")]
#[artifact_schema(id = "s.norm.en1993")]
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
        let mut base_plate = SteelJoint { plate_fy: 0.0, plate_width: 0.0, bolt_to_web: 0.0, extension_to_flange: 0.0, extension_edge: 0.0, alpha_factor: 0.0, bolt_stress_area: 0.0, bolt_grip_length: 0.0, column_flange_thickness: 0.0, column_web_thickness: 0.0, column_web_depth: 0.0, column_web_shear_area: 0.0, column_root_radius: 0.0, column_fy: 0.0, column_web_stress: 0.0, beam_flange_thickness: 0.0, compression_lever: 0.0, moment_demand: 0.0, beam_moment_resistance: 0.0, column_moment_resistance: 0.0, beam_inertia: 0.0, beam_span: 0.0, braced_frame: 0, stiffness_ratio: 0.0, transformation_parameter: 0.0, assumed_class: StiffnessClass::Unset, base_plate_length: 0.0, concrete_fcd: 0.0, concrete_modulus: 0.0, concrete_area_a0: 0.0, concrete_area_a1: 0.0, joint_coefficient: 0.0, anchor_area: 0.0, anchor_length: 0.0, base_normal_force: 0.0, base_moment: 0.0, base_lever_left: 0.0, base_lever_right: 0.0, beam_web_thickness: 0.0, beam_web_fy: 0.0, weld_force_longitudinal: 0.0, weld_force_transverse: 0.0, id: "joint-bp".into(), kind: JointKind::BasePlate, bolt_class: "8.8".into(), bolt_rows: 1, bolts_per_row: 2, edge_distance: 0.05, plate_thickness: 0.03, ..template };
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

impl StiffenerShape {
    /// 🔢️ Every outline in token order, the options of its choice grip.
    pub const ALL: [Self; 4] = [Self::None, Self::Flat, Self::Tee, Self::Angle];
}

impl WarpingSupport {
    /// 🎚️ Every end condition in token order, the options of its choice grip.
    pub const ALL: [Self; 4] = [Self::Fork, Self::Fixed, Self::FixedFork, Self::Cantilever];
}

/// 📒 Ordered HEB catalogue for remediation section upsizing.
pub fn rolled_heb_catalogue() -> Vec<SteelSection> {
    vec![catalogue_heb240(), catalogue_heb260(), catalogue_heb280(), catalogue_heb300()]
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1993_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1993_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1993_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1993_model_proto::LEAF };
//#endregion 🍃️FacetLeaves


/// 🔩 Material family of a steel grade: carbon or stainless steel (EN 1993-1-1 / EN 1993-1-4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialKind {
    Carbon,
    Stainless,
}

/// 🧱 Steel material grade entry (SI: fy/fu/E/G in Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMaterial {
    pub id: String,
    pub grade: String,
    pub fy: f64,
    pub fu: f64,
    pub e_modulus: f64,
    pub g_modulus: f64,
    pub subgrade: String,
    pub kind: MaterialKind,
    pub ramberg_osgood_n: f64,
}

/// 📐 Kind of a steel section: rolled I, welded I, rectangular or square hollow section, circular hollow section or angle (equal or unequal legs; its y-y and z-z values are the principal u-u and v-v values, EN 1993-1-1 Figure 1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SectionKind {
    RolledI,
    WeldedI,
    Hss,
    Chs,
    Angle,
}

/// 📐️ Steel section with full property table (SI: m, m², m³, m⁴).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelSection {
    pub id: String,
    pub designation: String,
    pub kind: SectionKind,
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    pub r: f64,
    pub area: f64,
    pub shear_area_y: f64,
    pub shear_area_z: f64,
    pub iy: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
    pub w_el_y: f64,
    pub w_el_z: f64,
    pub w_pl_y: f64,
    pub w_pl_z: f64,
    pub area_net: f64,
    /// 🏭 Fabrication route of a hollow section for the buckling curve of EN 1993-1-1 Table 6.2: hot finished (curve a, a0 for S460) or cold formed (curve c); Unset for sections that are not hollow. A hollow section left Unset has no buckling curve, so its buckling rows stay open.
    pub fabrication: HollowFabrication,
}

/// 🌀️ Torsion and warping end conditions of a member (EN 1993-1-1 6.2.7): fork, fixed, fixed-fork or cantilever.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum WarpingSupport {
    Fork,
    Fixed,
    FixedFork,
    Cantilever,
}

/// 🔩 Member type: beam or beam-column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SteelMemberType {
    Beam,
    Column,
    BeamColumn,
    Brace,
    Tie,
}

/// ⬇️ Level of the transverse load relative to the shear centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LoadApplication {
    ShearCenter,
    TopFlange,
    BottomFlange,
}

/// 📈 Shape of the bending moment diagram between the lateral restraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MomentDiagram {
    Linear,
    Uniform,
    Parabolic,
}

/// 🧮 Global analysis type (elastic or plastic).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum AnalysisKind {
    Elastic,
    Plastic,
}

/// 🔩 Forming of a stainless steel member (EN 1993-1-4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StainlessForming {
    Unset,
    ColdFormedOpen,
    WeldedOpen,
    Hollow,
}

/// 🌀 Application of the torsion action on a fork-supported member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum TorsionLoad {
    Point,
    Udl,
}

/// 🏗️ Structural member (lengths in m; ψ dimensionless).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMember {
    pub id: String,
    pub label: String,
    pub member_type: SteelMemberType,
    pub section_id: String,
    pub material_id: String,
    pub length: f64,
    pub buckling_length_y: f64,
    pub buckling_length_z: f64,
    pub ltb_length: f64,
    pub ltb_restraint_spacing: f64,
    pub load_application: LoadApplication,
    pub end_moment_ratio_psi: f64,
    pub moment_diagram: MomentDiagram,
    pub deflection_limit_ratio: f64,
    pub analysis: AnalysisKind,
    pub stainless_forming: StainlessForming,
    pub design_temperature: f64,
    pub toughness_stress_ratio: f64,
    pub radiation_shift: f64,
    pub stress_shift: f64,
    pub margin_shift: f64,
    pub strain_rate_shift: f64,
    pub cold_forming_radius: f64,
    pub web_material_id: String,
    pub torsion_load: TorsionLoad,
    pub torque_position: f64,
    pub warping_support: WarpingSupport,
}

/// 🏋️ Design action effects (SI: N, N·m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DesignAction {
    pub n: f64,
    pub vy: f64,
    pub vz: f64,
    pub my: f64,
    pub mz: f64,
    pub t: f64,
}

/// 🏋 Kind of an action of a load case (EN 1990 Table A1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ActionKind {
    Permanent,
    Imposed,
    Wind,
    Snow,
    Accidental,
}

/// 📋 Load case identity.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadCase {
    pub id: String,
    pub name: String,
    pub kind: ActionKind,
    pub category: String,
}

/// 🔗 Member action under a load case.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberAction {
    pub id: String,
    pub member_id: String,
    pub load_case_id: String,
    pub action: DesignAction,
}

/// 📉 Characteristic scalar force under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ForceAction {
    pub id: String,
    pub load_case_id: String,
    pub force: f64,
}

/// 🔩 Characteristic joint forces under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointForceAction {
    pub id: String,
    pub load_case_id: String,
    pub shear: f64,
    pub tension: f64,
}

/// 📊 Fatigue stress-range band (Δσ_i, n_i) for Palmgren-Miner.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueBand {
    pub id: String,
    pub delta_sigma: f64,
    pub cycles: f64,
}

/// 🔗 Assumed stiffness class of a joint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StiffnessClass {
    Unset,
    Rigid,
    SemiRigid,
    Pinned,
}

/// 🔗 Kind of a steel joint (EN 1993-1-8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointKind {
    Bolted,
    Welded,
    EndPlate,
    BasePlate,
}

/// 🔩 Category A, B or C of a bolted connection (EN 1993-1-8 Table 3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointCategory {
    A,
    B,
    C,
}

/// 🔩 Bolted or welded joint.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelJoint {
    pub id: String,
    pub kind: JointKind,
    pub member_id: String,
    pub bolt_class: String,
    pub bolt_diameter: f64,
    pub bolt_rows: u32,
    pub bolts_per_row: u32,
    pub pitch: f64,
    pub gauge: f64,
    pub end_distance: f64,
    pub edge_distance: f64,
    pub shear_planes: u32,
    pub plate_thickness: f64,
    pub plate_fu: f64,
    pub weld_throat: f64,
    pub weld_length: f64,
    pub weld_fu: f64,
    pub weld_grade: String,
    #[dsl(table)]
    pub actions: Vec<JointForceAction>,
    pub category: JointCategory,
    pub friction_mu: f64,
    pub preload_force: f64,
    pub slip_factor_ks: f64,
    pub friction_surfaces: u32,
    pub plate_fy: f64,
    pub plate_width: f64,
    pub bolt_to_web: f64,
    pub extension_to_flange: f64,
    pub extension_edge: f64,
    pub alpha_factor: f64,
    pub bolt_stress_area: f64,
    pub bolt_grip_length: f64,
    pub column_flange_thickness: f64,
    pub column_web_thickness: f64,
    pub column_web_depth: f64,
    pub column_web_shear_area: f64,
    pub column_root_radius: f64,
    pub column_fy: f64,
    pub column_web_stress: f64,
    pub beam_flange_thickness: f64,
    pub compression_lever: f64,
    pub moment_demand: f64,
    pub beam_moment_resistance: f64,
    pub column_moment_resistance: f64,
    pub beam_inertia: f64,
    pub beam_span: f64,
    pub braced_frame: u32,
    pub stiffness_ratio: f64,
    pub transformation_parameter: f64,
    pub assumed_class: StiffnessClass,
    pub base_plate_length: f64,
    pub concrete_fcd: f64,
    pub concrete_modulus: f64,
    pub concrete_area_a0: f64,
    pub concrete_area_a1: f64,
    pub joint_coefficient: f64,
    pub anchor_area: f64,
    pub anchor_length: f64,
    pub base_normal_force: f64,
    pub base_moment: f64,
    pub base_lever_left: f64,
    pub base_lever_right: f64,
    /// 🔩 Beam web thickness t_wb of an end plate joint for the beam web in tension F_t,wb,Rd = b_eff,t,wb t_wb f_y,wb / gamma_M0 (EN 1993-1-8 6.2.6.8); 0 = not given, the component is then not assessed and no value is assumed.
    #[dsl(unit = "m")]
    pub beam_web_thickness: f64,
    /// 🔩 Yield strength f_y,wb of the beam web of an end plate joint for the beam web in tension (EN 1993-1-8 6.2.6.8); 0 = not given, the component is then not assessed and no value is assumed.
    #[dsl(unit = "Pa")]
    pub beam_web_fy: f64,
    /// 🔥 Design force on the fillet weld group of the joint along the weld axis F_w,par,Ed, the source of tau_par in the directional method of EN 1993-1-8 4.5.3.2 (throat stress = force / (a l_w)); 0 = not given, the force then does not enter the weld check and no value is assumed.
    #[dsl(unit = "N")]
    pub weld_force_longitudinal: f64,
    /// 🔥 Design force on the fillet weld group of the joint perpendicular to the weld axis along a leg F_w,perp,Ed, the source of sigma_perp and tau_perp in the directional method of EN 1993-1-8 4.5.3.2 (throat stress = force / (a l_w)); 0 = not given, the force then does not enter the weld check and no value is assumed.
    #[dsl(unit = "N")]
    pub weld_force_transverse: f64,
}

/// ♻ Fatigue assessment method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FatigueMethod {
    DamageTolerant,
    SafeLife,
}

/// 🔄 Fatigue detail category assessment.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueDetail {
    pub id: String,
    pub member_id: String,
    pub category: u8,
    pub method: FatigueMethod,
    #[dsl(table)]
    pub spectrum: Vec<FatigueBand>,
    pub stress_history: String,
    /// 🔄 Consequence class of a fatigue failure for the partial factor γ_Mf of EN 1993-1-9 Table 3.1: with the assessment method (damage tolerant or safe life) it selects one of the four registry rows ndp.en1993.1-9.gamma-mf.<method>.<consequence>.
    pub consequence: FatigueConsequence,
}

/// ⏱️ Standard fire resistance class of a member (EN 1993-1-2 4.2.5): R30, R60, R90 or R120.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FireRating {
    R30,
    R60,
    R90,
    R120,
}

/// 🔥 Fire exposure on a member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireExposure {
    pub id: String,
    pub member_id: String,
    pub rating: FireRating,
    pub protection_thickness: f64,
    pub section_factor: f64,
    pub mu0: f64,
    pub design_temperature: f64,
    pub protection_conductivity: f64,
    pub protection_density: f64,
    pub protection_specific_heat: f64,
}

/// ❄️ Cold-formed thin-gauge member (1-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColdFormedMember {
    pub id: String,
    pub b_bar: f64,
    pub thickness: f64,
    pub k_sigma: f64,
    pub psi: f64,
    pub fy: f64,
    pub gross_resistance: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub flange_width: f64,
    pub lip_length: f64,
    pub web_height: f64,
    pub web_slope: f64,
    pub opposite_flange_width: f64,
    pub spring_factor: f64,
    pub corner_radius: f64,
    pub flange_stress: f64,
    pub shear_force: f64,
    pub support_reaction: f64,
    pub support_length: f64,
    pub crippling_category: u32,
    pub stiffened_support: u32,
    pub bending_moment: f64,
    pub moment_resistance: f64,
    pub sheeting_stiffness: f64,
    pub required_restraint_stiffness: f64,
    pub purlin_span: f64,
    pub purlin_load: f64,
    pub purlin_lateral_ratio: f64,
}

/// 🧱️ Outline of a transverse stiffener of a plated panel (EN 1993-1-5 9.2.1): none, flat bar, tee or angle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StiffenerShape {
    None,
    Flat,
    Tee,
    Angle,
}

/// 📍 Patch load type of EN 1993-1-5 Figure 6.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum PatchLoadType {
    Unset,
    Through,
    OneFlangeNearEnd,
    EndFlange,
}

/// 🧱 Plated panel for EN 1993-1-5.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PlatedPanel {
    pub id: String,
    pub a: f64,
    pub b: f64,
    pub thickness: f64,
    pub fy: f64,
    pub k_sigma: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub stiffener_inertia: f64,
    pub stiffener_area: f64,
    pub stiffener_eccentricity: f64,
    pub stiffener_alpha: f64,
    pub compression_stress: f64,
    pub half_width: f64,
    pub equivalent_span: f64,
    pub hogging: u32,
    pub patch_type: PatchLoadType,
    pub patch_force: f64,
    pub patch_length: f64,
    pub patch_offset: f64,
    pub flange_thickness: f64,
    pub flange_width: f64,
    pub flange_fy: f64,
    pub longitudinal_offset: f64,
    pub longitudinal_inertia: f64,
    pub patch_moment: f64,
    pub patch_moment_resistance: f64,
    pub bending_moment: f64,
    pub plastic_moment: f64,
    pub flange_moment: f64,
    pub shear_force: f64,
    pub transverse_stiffener_inertia: f64,
    pub flange_induced_k: f64,
    pub flange_area: f64,
    pub stress_x: f64,
    pub stress_z: f64,
    pub shear_stress: f64,
    pub alpha_cr: f64,
    pub corrugation_flat: f64,
    pub corrugation_projection: f64,
    pub corrugation_depth: f64,
    pub web_height: f64,
    pub stiffener_spacing: f64,
    pub stiffener_shape: StiffenerShape,
    pub stiffener_depth: f64,
    pub stiffener_thickness: f64,
    pub stiffener_flange_width: f64,
    pub stiffener_flange_thickness: f64,
    pub lateral_pressure: f64,
    pub lateral_pressure_coefficient: f64,
    /// 🏷 Steel grade of the plate for the brittle fracture check of EN 1993-1-10 2.1 (Table 2.1 column, e.g. S355); empty = not given, the plate is then not assessed and no grade is assumed.
    pub steel_grade: String,
    /// 🏷 Sub-grade of the plate (JR, J0, J2, K2, ...) for Table 2.1 of EN 1993-1-10; empty = not given.
    pub steel_subgrade: String,
    /// 🌡 Reference temperature T_Ed of the plate in °C for EN 1993-1-10 2.1 (minimum member temperature with the shifts already applied); used with the plate grade, the stress level is taken as the highest transcribed column 0.75 f_y(t).
    pub design_temperature: f64,
}

/// 🏭 Fabrication tolerance class of a silo shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FabricationToleranceClass {
    Unset,
    A,
    B,
    C,
}

/// 🛢️ Silo / shell of revolution.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SiloShell {
    pub id: String,
    pub thickness: f64,
    pub radius: f64,
    pub depth: f64,
    pub k: f64,
    pub gamma: f64,
    pub fy: f64,
    pub length: f64,
    pub fabrication_class: FabricationToleranceClass,
    pub meridional_stress: f64,
    pub hoop_stress: f64,
    pub shell_shear_stress: f64,
    pub boundary_factor_axial: f64,
    pub boundary_factor_hoop: f64,
    pub boundary_factor_shear: f64,
    pub wall_friction: f64,
    pub plan_area: f64,
    pub plan_perimeter: f64,
    pub liquid_density: f64,
    pub liquid_height: f64,
    pub course_elevation: f64,
    pub internal_pressure: f64,
    pub external_pressure: f64,
    pub temperature_change: f64,
    pub ring_spacing: f64,
    pub ring_area: f64,
    pub ring_inertia: f64,
    pub ring_offset: f64,
}

/// 🪢 Tension component / cable (1-11).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TensionComponent {
    pub id: String,
    pub f_uk: f64,
    pub f_k: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub partial_factor: f64,
    pub cable_length: f64,
    pub cable_weight: f64,
    pub cable_horizontal_force: f64,
    pub cable_modulus: f64,
    pub cable_stress: f64,
    pub cable_unit_weight: f64,
    pub thread_factor: f64,
    pub rod_stress_area: f64,
    pub rod_fu: f64,
    pub pin_diameter: f64,
    pub pin_plate_thickness: f64,
    pub pin_gap: f64,
    pub pin_outer_thickness: f64,
    pub pin_inner_thickness: f64,
    pub pin_fup: f64,
    pub pin_fyp: f64,
    pub pin_plate_fy: f64,
}

/// 🌉 Bridge fatigue parameters (EN 1993-2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeFatigue {
    pub id: String,
    pub member_id: String,
    pub lambda: f64,
    pub phi2: f64,
    pub delta_sigma_p: f64,
    pub category: u8,
    pub method: FatigueMethod,
    pub heavy_vehicle_weight: f64,
    pub reference_weight: f64,
    pub observed_traffic: f64,
    pub design_life: f64,
    pub second_lane_traffic: f64,
    pub second_lane_weight: f64,
    pub lambda_1: f64,
    pub lambda_max: f64,
    pub detail_id: String,
    pub plate_thickness: f64,
    pub deck_thickness: f64,
    pub trough_spacing: f64,
    pub trough_width: f64,
    pub trough_thickness: f64,
    pub crossbeam_spacing: f64,
    pub surfacing_thickness: f64,
    pub wheel_load: f64,
    pub wheel_patch: f64,
    /// 🌉 Consequence class of a fatigue failure of the bridge detail for the partial factor γ_Mf of EN 1993-1-9 Table 3.1: with the assessment method it selects one of the four registry rows ndp.en1993.1-9.gamma-mf.<method>.<consequence>.
    pub consequence: FatigueConsequence,
}

/// 🗼 Lattice member kind of a tower leg.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LatticeKind {
    Unset,
    Leg,
    Diagonal,
    Redundant,
}

/// 🗼 Tower leg (EN 1993-3-1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TowerLeg {
    pub id: String,
    pub member_id: String,
    pub force_coefficient: f64,
    pub dynamic_factor: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub lattice_kind: LatticeKind,
    pub slenderness: f64,
    pub diameter: f64,
    pub height: f64,
    pub bending_stiffness: f64,
    pub mass_per_length: f64,
    pub strouhal: f64,
    pub log_decrement: f64,
    pub air_density: f64,
    pub mean_wind_speed: f64,
    pub crossing_rate: f64,
    pub design_life: f64,
    pub k_factor: f64,
    pub kw_factor: f64,
    pub lateral_force_coefficient: f64,
    pub permitted_amplitude_ratio: f64,
    pub ring_mode: f64,
    pub shell_thickness: f64,
    pub guy_area: f64,
    pub guy_angle: f64,
    pub guy_length: f64,
}

/// 🌊 Corrosion environment of a sheet pile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SheetPileEnvironment {
    Unset,
    UndisturbedSoil,
    PollutedSoil,
    CompactedFill,
    NonCompactedFill,
}

/// 🪵 Steel pile (EN 1993-5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelPile {
    pub id: String,
    pub section_id: String,
    pub material_id: String,
    pub driving_stress: f64,
    pub embedded_length: f64,
    pub shaft_perimeter: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub environment: SheetPileEnvironment,
    pub design_life: f64,
    pub wall_thickness: f64,
    pub interlock_bending_factor: f64,
    pub interlock_stiffness_factor: f64,
    pub section_modulus: f64,
    pub shear_area: f64,
    pub design_moment: f64,
    pub design_shear: f64,
    pub interlock_shear: f64,
}

/// 🏗️ Crane runway local web check (EN 1993-6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CraneRunway {
    pub id: String,
    pub member_id: String,
    pub wheel_contact_length: f64,
    pub dispersion: f64,
    pub web_thickness: f64,
    pub fy: f64,
    pub phi: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub max_wheel_load: f64,
    pub load_factor: f64,
    pub rail_inertia: f64,
    pub rail_fixed: u32,
    pub flange_effective_width: f64,
    pub flange_thickness: f64,
    pub wheel_eccentricity: f64,
    pub torsion_constant: f64,
    pub longitudinal_stress: f64,
    pub rail_radius: f64,
    pub rail_contact_width: f64,
    pub crane_nu: f64,
    pub crane_kq: f64,
    pub damage_exponent: f64,
    pub stress_range: f64,
    pub detail_category_stress: f64,
    pub fatigue_phi: f64,
}

/// 🪜 Storey of a frame: sway stability of EN 1993-1-1 5.2 and 5.3 (α_cr, imperfection, bracing).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Storey {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub height: f64,
    pub vertical_load: f64,
    pub horizontal_load: f64,
    pub drift: f64,
    pub columns: u32,
    pub plastic_analysis: u32,
    pub building_height: f64,
    pub bracing_load: f64,
    pub bracing_length: f64,
    pub bracing_deflection: f64,
}

/// ⚓ Support condition at the end of a restrained member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum RestraintSupport {
    Fork,
    Fixed,
}

/// 📈 Load and moment distribution of a restrained member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum RestraintDistribution {
    Uniform,
    Udl,
    Linear,
}

/// 🧷 Supports and restraints of a member: end supports, an intermediate support, continuous torsional and discrete lateral springs, load height; M_cr by finite elements (EN 1993-1-1 6.3.2, 6.3.4, 6.3.5.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Restraint {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub start_support: RestraintSupport,
    pub end_support: RestraintSupport,
    pub intermediate_position: f64,
    pub torsional_spring: f64,
    pub lateral_spring_position: f64,
    pub lateral_spring_stiffness: f64,
    pub load_height: f64,
    pub load_distribution: RestraintDistribution,
    pub design_moment: f64,
    pub design_axial: f64,
    pub general_method: u32,
    pub hinge_to_restraint: f64,
}

/// ⬜ Chord section outline shape of a built-up member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ChordShape {
    Unset,
    Channel,
    I,
    Rhs,
    Chs,
}

/// 🪜 Kind of a built-up compression member: laced, battened or closely spaced (EN 1993-1-1 6.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BuiltUpKind {
    Laced,
    Battened,
    Closely,
}

/// 🪜 Built-up compression member: laced, battened or closely spaced chords (EN 1993-1-1 6.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BuiltUpMember {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub kind: BuiltUpKind,
    pub chord_area: f64,
    pub chord_inertia: f64,
    pub chord_distance: f64,
    pub panel_spacing: f64,
    pub planes: u32,
    pub batten_inertia: f64,
    pub diagonal_area: f64,
    pub diagonal_length: f64,
    pub design_axial: f64,
    pub design_moment: f64,
    pub back_to_back: u32,
    pub min_radius: f64,
    pub chain_spacing: f64,
    pub chord_shape: ChordShape,
    pub chord_depth: f64,
    pub chord_width: f64,
    pub chord_flange_thickness: f64,
    pub chord_web_thickness: f64,
}

/// 🪡️ Arrangement of the weld lines of a weld group (EN 1993-1-8 4.5.3.3): one line, two parallel lines or the four sides of a box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum WeldPattern {
    Line,
    TwoLines,
    Box,
}

/// 🧵 Weld group under in-plane shear and torsion: elastic vector method (EN 1993-1-8 4.5.3.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WeldGroup {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub pattern: WeldPattern,
    pub throat: f64,
    pub width: f64,
    pub depth: f64,
    pub shear_force: f64,
    pub eccentricity: f64,
    pub steel_fu: f64,
    pub beta_w: f64,
}

/// 📏 Shape of a parametric steel section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ParametricShape {
    WeldedI,
    Rhs,
    Chs,
    Angle,
}

/// 📐 Parametric section: welded I, rectangular and circular hollow sections by their dimensions, compared with the section row (A, I_y, I_z, W_el,y, W_pl,y).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ParametricSection {
    pub id: String,
    pub section_id: String,
    pub label: String,
    pub shape: ParametricShape,
    pub height: f64,
    pub width: f64,
    pub web_thickness: f64,
    pub flange_thickness: f64,
    pub tolerance: f64,
}

/// 🕳 Welded RHS truss joint with a gap (K and N joints): chord face failure and brace effective width (EN 1993-1-8 7.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HollowJoint {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub chord_width: f64,
    pub chord_height: f64,
    pub chord_thickness: f64,
    pub chord_fy: f64,
    pub chord_stress: f64,
    pub brace1_width: f64,
    pub brace1_height: f64,
    pub brace1_thickness: f64,
    pub brace1_angle: f64,
    pub brace1_force: f64,
    pub brace2_width: f64,
    pub brace2_height: f64,
    pub brace2_thickness: f64,
    pub brace2_angle: f64,
    pub brace2_force: f64,
    pub gap: f64,
    pub brace_fy: f64,
}

/// 🔩 Row position of a bolt row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointRowType {
    Extended,
    FirstBelow,
    Inner,
}

/// 🔩 Bolt row of an end plate joint: lever arm, T-stub geometry (m, e, m_x, e_x, alpha), pitch to the previous row, bolts and group effect for the EN 1993-1-8 component method.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointRow {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub row_type: JointRowType,
    pub lever: f64,
    pub bolt_to_web: f64,
    pub edge_distance: f64,
    pub extension_to_flange: f64,
    pub extension_edge: f64,
    pub alpha_factor: f64,
    pub pitch_to_previous: f64,
    pub bolts: u32,
    pub bolt_diameter: f64,
    pub bolt_stress_area: f64,
    pub grip_length: f64,
    pub bolt_class: String,
    pub group_with_previous: u32,
}

/// ⚓ Support of a frame node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameNodeSupport {
    Free,
    Pinned,
    Fixed,
    Roller,
}

/// 🕸 Node of a general planar frame in the x-z plane (z up) with its support and nodal design loads: input of the mechanics-kernel eigenvalue α_cr (EN 1993-1-1 5.2.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub z: f64,
    pub support: FrameNodeSupport,
    pub load_x: f64,
    pub load_z: f64,
}

/// 🔗 End connection of a frame bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameBarRelease {
    Rigid,
    Pinned,
    SemiRigid,
}

/// 🏗️ Action of one load case on the frame (EN 1990 6.4.3): a point force or moment at a node, or a uniform line load on a bar between two positions along it; the load cases are combined by the EN 1990 rules and every combination is solved on its own.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameAction {
    pub id: String,
    pub load_case_id: String,
    pub target_id: String,
    pub kind: FrameActionKind,
    pub fx: f64,
    pub fz: f64,
    pub my: f64,
    pub start_ratio: f64,
    pub end_ratio: f64,
}

/// 🧭 Where the design forces of a steel member come from: the frame analysis (envelope over the combinations of its bars) or its explicit member actions (EN 1993-1-1 5.2.2, EN 1990 6.4.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameMemberBasis {
    pub id: String,
    pub member_id: String,
    pub basis: MemberForceBasis,
}

/// 🪛 🪛 Mechanically fastened joint of cold-formed sheets (EN 1993-1-3 8.3): self-tapping screws or blind rivets in shear and tension with the sheet thicknesses and strengths, the fastener spacing and the shear resistance of the fastener from tests.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FastenedJoint {
    pub id: String,
    /// 🥫 Id of the cold-formed member the joint belongs to.
    pub member_id: String,
    /// 🪛 Self-tapping screw or blind rivet.
    pub kind: FastenerKind,
    /// ⌀ Nominal diameter d of the screw or rivet.
    pub diameter: f64,
    /// ⭕ Diameter d_w of the washer or head that bears on the sheet in tension (pull-over); 0 = not given.
    pub washer_diameter: f64,
    /// 🔢 Number of fasteners n that share the force.
    pub count: u32,
    /// 🔢 Number of fasteners in the net section across the width.
    pub per_row: u32,
    /// 📏 Thickness t_1 of the sheet on the head side.
    pub head_thickness: f64,
    /// 📏 Thickness t_2 of the sheet or support the fastener ends in.
    pub base_thickness: f64,
    /// 💪 Ultimate tensile strength of the sheet t_1.
    pub head_strength: f64,
    /// 💪 Ultimate tensile strength of the sheet t_2.
    pub base_strength: f64,
    /// ↦ Distance e_1 from the fastener to the end of the sheet in the direction of the force.
    pub end_distance: f64,
    /// ↥ Distance e_2 from the fastener to the edge of the sheet perpendicular to the force.
    pub edge_distance: f64,
    /// ↔ Spacing p_1 of the fasteners in the direction of the force.
    pub pitch: f64,
    /// ↕ Spacing p_2 of the fastener rows perpendicular to the force.
    pub gauge: f64,
    /// ↔ Width b of the sheet in the net section across the fasteners.
    pub sheet_width: f64,
    /// ➡ Shear force on the fastened joint, shared by the n fasteners.
    pub shear_force: f64,
    /// ⬆ Tension force on the fastened joint (pull-out and pull-over), shared by the n fasteners.
    pub tension_force: f64,
    /// 🧪 Design shear resistance of one fastener from the manufacturer or tests; 0 = not given, only the sheet resistances are checked.
    pub shear_resistance: f64,
}

/// 🏗️ Kind of a frame action: point force at a node, point moment at a node or uniform line load on a bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameActionKind {
    NodalForce,
    NodalMoment,
    BarUniform,
}

/// 🧭 Source of the design forces of a member: the envelope of the frame bars of the member or its explicit member actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MemberForceBasis {
    FrameDerived,
    Explicit,
}

/// 🏭 Fabrication route of a hollow section (EN 1993-1-1 Table 6.2): hot finished or cold formed; Unset for sections that are not hollow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum HollowFabrication {
    Unset,
    HotFinished,
    ColdFormed,
}

/// 🔩 Method for the force on each bolt of a bolt group: elastic vector method or instantaneous centre of rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BoltGroupMethod {
    Elastic,
    InstantaneousCentre,
}

/// 🧯 Residual stress state of a fatigue detail: as welded, welded and stress relieved or non-welded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StressRelief {
    AsWelded,
    StressRelieved,
    NonWelded,
}

/// ⛓ End condition of a column at one node of a frame: framed, fixed or pinned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum NodeBRestraint {
    Framed,
    Fixed,
    Pinned,
}

impl std::fmt::Display for NodeBRestraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl NodeBRestraint {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Framed => "Framed",
            Self::Fixed => "Fixed",
            Self::Pinned => "Pinned",
        }
    }
}

/// ⛓ End condition of a column at one node of a frame: framed, fixed or pinned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum NodeARestraint {
    Framed,
    Fixed,
    Pinned,
}

impl std::fmt::Display for NodeARestraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl NodeARestraint {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Framed => "Framed",
            Self::Fixed => "Fixed",
            Self::Pinned => "Pinned",
        }
    }
}

/// ↔ Buckling mode of a frame: nodes held (non-sway) or free to translate (sway).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameSway {
    NonSway,
    Sway,
}

impl std::fmt::Display for FrameSway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FrameSway {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonSway => "NonSway",
            Self::Sway => "Sway",
        }
    }
}

/// 🧭 Buckling plane of a column: about the major or about the minor axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BucklingPlane {
    MajorAxis,
    MinorAxis,
}

/// 🏭 Forming process of a cold-formed section: cold roll forming or other forming.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FormingProcess {
    RollForming,
    OtherForming,
}

/// 🪛 Mechanical fastener of cold-formed sheets: self-tapping screw or blind rivet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FastenerKind {
    SelfTappingScrew,
    BlindRivet,
}

/// ⚠ Consequence class of a fatigue failure (EN 1993-1-9 Table 3.1): low consequence or high consequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FatigueConsequence {
    Low,
    High,
}

impl std::fmt::Display for FatigueConsequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FatigueConsequence {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::High => "High",
        }
    }
}

impl std::fmt::Display for FastenerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FastenerKind {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfTappingScrew => "SelfTappingScrew",
            Self::BlindRivet => "BlindRivet",
        }
    }
}

impl std::fmt::Display for FormingProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FormingProcess {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollForming => "RollForming",
            Self::OtherForming => "OtherForming",
        }
    }
}

impl std::fmt::Display for BucklingPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BucklingPlane {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MajorAxis => "MajorAxis",
            Self::MinorAxis => "MinorAxis",
        }
    }
}

impl std::fmt::Display for StressRelief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StressRelief {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AsWelded => "AsWelded",
            Self::StressRelieved => "StressRelieved",
            Self::NonWelded => "NonWelded",
        }
    }
}

impl std::fmt::Display for BoltGroupMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BoltGroupMethod {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elastic => "Elastic",
            Self::InstantaneousCentre => "InstantaneousCentre",
        }
    }
}

impl std::fmt::Display for HollowFabrication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl HollowFabrication {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "Unset",
            Self::HotFinished => "HotFinished",
            Self::ColdFormed => "ColdFormed",
        }
    }
}

impl std::fmt::Display for MemberForceBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MemberForceBasis {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameDerived => "frameDerived",
            Self::Explicit => "explicit",
        }
    }
}

impl FrameActionKind {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodalForce => "nodalForce",
            Self::NodalMoment => "nodalMoment",
            Self::BarUniform => "barUniform",
        }
    }
}

impl std::fmt::Display for FrameActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🪢 Bar of a general planar frame between two frame nodes; section and material come from the referenced steel member, end releases make hinges.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameBar {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub start_node_id: String,
    pub end_node_id: String,
    pub release_start: FrameBarRelease,
    pub release_end: FrameBarRelease,
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Placement {
    pub id: String,
    /// ↔️ Origin x [m].
    pub x: f64,
    /// ↕️ Origin y [m].
    pub y: f64,
    /// ⬆️ Origin z [m].
    pub z: f64,
    /// 🧭️ Yaw about +Z [rad].
    pub yaw: f64,
}

/// 🧩 Plate of a fin plate or splice joint: role, plate size and steel, bolt pattern, eccentricity of the bolt line to the support and weld to the support or member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointPlate {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub role: String,
    pub plate_height: f64,
    pub plate_width: f64,
    pub plate_thickness: f64,
    pub plate_count: u32,
    pub plate_fy: f64,
    pub plate_fu: f64,
    pub bolt_class: String,
    pub bolt_diameter: f64,
    pub hole_diameter: f64,
    pub bolt_rows: u32,
    pub bolts_per_row: u32,
    pub pitch: f64,
    pub gauge: f64,
    pub end_distance: f64,
    pub edge_distance: f64,
    pub shear_planes: u32,
    pub eccentricity: f64,
    pub weld_throat: f64,
    pub weld_fu: f64,
    pub weld_beta_w: f64,
    /// 🔩 Distribution of the load over the bolt group of the plate (EN 1993-1-8 3.12): Elastic is the vector method with the polar moment of the bolts, InstantaneousCentre is the plastic method with the load-deformation law of the bolts about the instantaneous centre of rotation.
    pub group_method: BoltGroupMethod,
}

/// 🧱 Concrete block, grout layer and anchor bolts under a base plate joint: block size and strength, grout thickness and strength, anchors in tension.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FoundationBlock {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub block_length: f64,
    pub block_width: f64,
    pub block_depth: f64,
    pub concrete_fck: f64,
    pub grout_fck: f64,
    pub grout_thickness: f64,
    pub anchor_count: u32,
    pub anchor_class: String,
    pub anchor_diameter: f64,
    pub anchor_stress_area: f64,
    pub anchor_embedment: f64,
    pub anchor_edge_distance: f64,
}

/// 🧪 🧪 Elongation at failure and ultimate strain of a steel material (EN 1993-1-1 3.2.2): the values of the material certificate that, with f_u/f_y, decide whether a material is ductile enough for plastic global analysis.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MaterialDuctility {
    pub id: String,
    /// 🧪 Id of the steel material the certificate values belong to.
    pub material_id: String,
    /// 📏 Elongation at failure of the material as a fraction (0.15 = 15 %).
    pub elongation: f64,
    /// 📏 Strain eps_u at the ultimate strength f_u as a fraction.
    pub ultimate_strain: f64,
}

/// 🥫 🥫 Enhanced yield strength of a cold-formed member from the work hardening of the bends (EN 1993-1-3 3.2.2): process, ultimate strength of the base material, gross area and number of 90 degree bends with a small radius.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FormingGain {
    pub id: String,
    /// 🥫 Id of the cold-formed member whose f_y is checked against the enhanced yield strength.
    pub member_id: String,
    /// 🏭 Cold roll forming or other forming (brake pressing) selects the coefficient k of f_ya.
    pub forming: FormingProcess,
    /// 💪 Ultimate tensile strength of the base material before forming.
    pub ultimate_strength: f64,
    /// ▭ Gross cross-section area A_g of the member.
    pub gross_area: f64,
    /// 🔢 Number of 90 degree bends with an inner radius r <= 5 t (bends of other angles count proportionally).
    pub bends: u32,
    /// 🥫 Yield strength the design uses for the member: the basic f_yb, or an enhanced value that must not exceed the average yield strength f_ya of the cold forming (EN 1993-1-3 3.2.2); 0 = the basic yield strength of the member is used.
    #[dsl(unit = "Pa")]
    pub used_yield: f64,
}

/// 🪜 🪜 Buckling length of a column in a frame from the end restraint of the adjacent members (EN 1993-1-1 6.3.1.3 with the nomogram of the effective length factor): sway or non-sway mode, the stiffness sums of columns and beams at both ends, and the member whose buckling length it checks.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BucklingLength {
    pub id: String,
    /// 🧱 Id of the column whose buckling length is derived.
    pub member_id: String,
    /// 🧭 Plane of the frame: buckling about the major axis (length L_cr,y) or the minor axis (L_cr,z).
    pub plane: BucklingPlane,
    /// ↔ Non-sway or sway buckling mode of the frame.
    pub sway: FrameSway,
    /// ⛓ End condition at node A (the lower end of a column): framed into columns and beams, fixed (rigid) or pinned.
    pub restraint_a: NodeARestraint,
    /// 🧮 Sum of I_c/L_c of the columns at node A, including the column itself.
    pub column_stiffness_a: f64,
    /// 🧮 Sum of alpha I_b/L_b of the beams at node A with the far-end factor alpha of each beam.
    pub beam_stiffness_a: f64,
    /// ⛓ End condition at node B (the upper end of a column): framed into columns and beams, fixed (rigid) or pinned.
    pub restraint_b: NodeBRestraint,
    /// 🧮 Sum of I_c/L_c of the columns at node B, including the column itself.
    pub column_stiffness_b: f64,
    /// 🧮 Sum of alpha I_b/L_b of the beams at node B with the far-end factor alpha of each beam.
    pub beam_stiffness_b: f64,
}

/// ⏳ ⏳ Stress range of one fatigue detail from the fatigue load model (EN 1993-1-9 5.1 to 5.4): the stresses at the detail fibre from the member actions of the highest and the lowest load case, the reduction of the compressive part for non-welded or stress-relieved details, the geometric stress concentration, the plate thickness for the size effect and the number of cycles of the range.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueStress {
    pub id: String,
    /// 🔗 Id of the fatigue detail (category, method) the range belongs to.
    pub detail_id: String,
    /// ⬆ Id of the load case whose member actions give the largest stress at the detail fibre (fatigue load model in its unfavourable position).
    pub max_case_id: String,
    /// ⬇ Id of the load case whose member actions give the smallest stress at the detail fibre.
    pub min_case_id: String,
    /// 📍 Distance z of the detail fibre from the y axis, positive in the direction of positive bending moment M_y compression: sigma = N/A + M_y z/I_y - M_z y/I_z.
    pub fibre_z: f64,
    /// 📍 Distance y of the detail fibre from the z axis.
    pub fibre_y: f64,
    /// 🧯 As-welded details take the full range; stress-relieved welded and non-welded details take the compressive part of the cycle reduced (EN 1993-1-9 5.4).
    pub relief: StressRelief,
    /// 📐 Geometric stress concentration factor k_f of the nominal stress at the detail (EN 1993-1-9 5.2); 0 = not given, the nominal stress is used unchanged.
    pub concentration: f64,
    /// 📏 Thickness t of the cracked plate for the size effect k_s = (25/t)^n of the detail strength (EN 1993-1-9 7.1); 0 = not given, no size effect is applied.
    pub thickness: f64,
    /// 🔁 Number of cycles of this stress range in the design life.
    pub cycles: f64,
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_s_plugin_norm_app_surface::artifact_kind_spec("en1993", "EN 1993", EN1993_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1993_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1993", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1993_DOCUMENT_SCHEMA: &str = "semio.norm.en1993/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;

impl SteelMemberType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beam => "beam",
            Self::Column => "column",
            Self::BeamColumn => "beamColumn",
            Self::Brace => "brace",
            Self::Tie => "tie",
        }
    }
}

impl std::fmt::Display for SteelMemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LoadApplication {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShearCenter => "shearCenter",
            Self::TopFlange => "topFlange",
            Self::BottomFlange => "bottomFlange",
        }
    }
}

impl std::fmt::Display for LoadApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MomentDiagram {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Uniform => "uniform",
            Self::Parabolic => "parabolic",
        }
    }
}

impl std::fmt::Display for MomentDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AnalysisKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elastic => "elastic",
            Self::Plastic => "plastic",
        }
    }
}

impl std::fmt::Display for AnalysisKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StainlessForming {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::ColdFormedOpen => "coldFormedOpen",
            Self::WeldedOpen => "weldedOpen",
            Self::Hollow => "hollow",
        }
    }
}

impl std::fmt::Display for StainlessForming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TorsionLoad {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Point => "point",
            Self::Udl => "udl",
        }
    }
}

impl std::fmt::Display for TorsionLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PatchLoadType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Through => "through",
            Self::OneFlangeNearEnd => "oneFlangeNearEnd",
            Self::EndFlange => "endFlange",
        }
    }
}

impl std::fmt::Display for PatchLoadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FabricationToleranceClass {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

impl std::fmt::Display for FabricationToleranceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StiffnessClass {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Rigid => "rigid",
            Self::SemiRigid => "semiRigid",
            Self::Pinned => "pinned",
        }
    }
}

impl std::fmt::Display for StiffnessClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LatticeKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Leg => "leg",
            Self::Diagonal => "diagonal",
            Self::Redundant => "redundant",
        }
    }
}

impl std::fmt::Display for LatticeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SheetPileEnvironment {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::UndisturbedSoil => "undisturbedSoil",
            Self::PollutedSoil => "pollutedSoil",
            Self::CompactedFill => "compactedFill",
            Self::NonCompactedFill => "nonCompactedFill",
        }
    }
}

impl std::fmt::Display for SheetPileEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RestraintSupport {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fork => "fork",
            Self::Fixed => "fixed",
        }
    }
}

impl std::fmt::Display for RestraintSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RestraintDistribution {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uniform => "uniform",
            Self::Udl => "udl",
            Self::Linear => "linear",
        }
    }
}

impl std::fmt::Display for RestraintDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ChordShape {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Channel => "channel",
            Self::I => "i",
            Self::Rhs => "rhs",
            Self::Chs => "chs",
        }
    }
}

impl std::fmt::Display for ChordShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl WeldPattern {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::TwoLines => "twoLines",
            Self::Box => "box",
        }
    }
}

impl std::fmt::Display for WeldPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointRowType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extended => "extended",
            Self::FirstBelow => "firstBelow",
            Self::Inner => "inner",
        }
    }
}

impl std::fmt::Display for JointRowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FrameNodeSupport {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Pinned => "pinned",
            Self::Fixed => "fixed",
            Self::Roller => "roller",
        }
    }
}

impl std::fmt::Display for FrameNodeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FrameBarRelease {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rigid => "rigid",
            Self::Pinned => "pinned",
            Self::SemiRigid => "semiRigid",
        }
    }
}

impl std::fmt::Display for FrameBarRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FireRating {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::R30 => "r30",
            Self::R60 => "r60",
            Self::R90 => "r90",
            Self::R120 => "r120",
        }
    }
}

impl std::fmt::Display for FireRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FatigueMethod {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DamageTolerant => "damage_tolerant",
            Self::SafeLife => "safe_life",
        }
    }
}

impl std::fmt::Display for FatigueMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MaterialKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Carbon => "carbon",
            Self::Stainless => "stainless",
        }
    }
}

impl std::fmt::Display for MaterialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SectionKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RolledI => "rolledI",
            Self::WeldedI => "weldedI",
            Self::Hss => "hss",
            Self::Chs => "chs",
            Self::Angle => "angle",
        }
    }
}

impl std::fmt::Display for SectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ActionKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permanent => "permanent",
            Self::Imposed => "imposed",
            Self::Wind => "wind",
            Self::Snow => "snow",
            Self::Accidental => "accidental",
        }
    }
}

impl std::fmt::Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bolted => "bolted",
            Self::Welded => "welded",
            Self::EndPlate => "endPlate",
            Self::BasePlate => "basePlate",
        }
    }
}

impl std::fmt::Display for JointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointCategory {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

impl std::fmt::Display for JointCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BuiltUpKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Laced => "laced",
            Self::Battened => "battened",
            Self::Closely => "closely",
        }
    }
}

impl std::fmt::Display for BuiltUpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ParametricShape {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WeldedI => "weldedI",
            Self::Rhs => "rhs",
            Self::Chs => "chs",
            Self::Angle => "angle",
        }
    }
}

impl std::fmt::Display for ParametricShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
