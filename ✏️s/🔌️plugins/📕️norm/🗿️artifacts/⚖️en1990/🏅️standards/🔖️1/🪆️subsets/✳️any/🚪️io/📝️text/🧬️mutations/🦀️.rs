//! ⚡️ EN 1990 — hand-rolled `OpText` for `En1990Mutation`; `OpBinary` is the protocol-tagged payload frame.

use crate::En1990Mutation;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_traffic_group_group;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_traffic_group_variable_id;
use semio_s_artifact_norm_en1990_mut_traffic_groups::change_reliability_member_id;
use semio_s_artifact_norm_en1990_mut_traffic_groups_rows::insert_traffic_group;
use semio_s_artifact_norm_en1990_mut_traffic_groups_rows::remove_traffic_group;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_material_strength_k;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_material_conv_factor;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_mat_partial_factor;
use semio_s_artifact_norm_en1990_mut_members_material::change_member_mat_design_strength;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_nominal_dimension;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_dimension_tolerance;
use semio_s_artifact_norm_en1990_mut_members_geometry::change_member_design_dimension;
use semio_s_artifact_norm_en1990_mut_members_analysis::change_member_member_alpha_cr;
use semio_s_artifact_norm_en1990_mut_members_analysis::change_member_second_order_analysis;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_damping_ratio;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_factor;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_target;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_value;
use semio_s_artifact_norm_en1990_mut_factor_overrides::change_factor_override_justif;
use semio_s_artifact_norm_en1990_mut_factor_overrides_rows::insert_factor_override;
use semio_s_artifact_norm_en1990_mut_factor_overrides_rows::remove_factor_override;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_frame_height;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_frame_drift;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_storey_height;
use semio_s_artifact_norm_en1990_mut_members_horizontal::change_member_storey_drift;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_track_gauge;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_line_speed;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_ballast_depth;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_deck_mass;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_natural_frequency;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_end_rotation;
use semio_s_artifact_norm_en1990_mut_bridge_sls_rail::change_bridge_basic_wind_speed;
use semio_s_artifact_norm_en1990_mut_design_cases::{insert_case, remove_case, change_case_number, change_case_text, change_case_label_en, change_case_label_de, change_case_entry_id};
use semio_s_artifact_norm_en1990_mut_actions_fields::change_permanent_kind;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_permanent_gk;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_category;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_qk;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_variable_group;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_accidental_ad;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_seismic_a_ek;
use semio_s_artifact_norm_en1990_mut_actions_fields::change_seismic_importance_class;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_label_en;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_label_de;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_str;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_geo;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_equ_stab;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_equ_destab;
use semio_s_artifact_norm_en1990_mut_members_resistance::change_member_rd_fat;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_span;
use semio_s_artifact_norm_en1990_mut_members_identity::change_member_precamber;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w1;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w2;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_deflection_w3;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_defl_limit_ratio;
use semio_s_artifact_norm_en1990_mut_members_deflection::change_member_defl_limit_ratio_w3;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_char;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_freq;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_sls_limit_qp;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_vibration_frequency;
use semio_s_artifact_norm_en1990_mut_members_vibration::change_member_vib_freq_min;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_member_id;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_acceleration;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_accel_limit;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_twist;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deck_twist_limit;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deflection;
use semio_s_artifact_norm_en1990_mut_bridge_sls::change_bridge_deflection_limit;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_member_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_action_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_effect_influence;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_member_id;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_detail_category;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_stress_range;
use semio_s_artifact_norm_en1990_mut_effects_fatigue::change_fatigue_cycles;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_dist;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_mean;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_resistance_sd;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_dist;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_mean;
use semio_s_artifact_norm_en1990_mut_reliability_analyses::change_reliability_action_sd;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_quantity;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_pairs;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_distribution;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_sample_size;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_mean;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_std_dev;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_variation_known;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_conversion_factor;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_required;
use semio_s_artifact_norm_en1990_mut_test_series::change_test_theory_variation;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1990_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1990_mut_design_basis_structure::change_structure_kind;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::insert_bridge_sls;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::remove_bridge_sls;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::insert_fatigue_spectrum;
use semio_s_artifact_norm_en1990_mut_bridge_fatigue_rows::remove_fatigue_spectrum;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_reliability_analysis;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_reliability_analysis;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_test_series;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_test_series;
use semio_s_artifact_norm_en1990_mut_analysis_rows::insert_placement;
use semio_s_artifact_norm_en1990_mut_analysis_rows::remove_placement;

use semio_s_artifact_norm_en1990_mut_accidental::{insert_accidental, insert_seismic, remove_accidental, remove_seismic};
use semio_s_artifact_norm_en1990_mut_design_basis::{change_annex, change_design_working_life_category, change_design_working_life_years, change_project_id, change_altitude_m, change_reference_period_years};
use semio_s_artifact_norm_en1990_mut_reliability::{change_beta_computed, change_consequence_class, change_inspection_level, change_reliability_class, change_supervision_level};
use semio_s_artifact_norm_en1990_mut_members::{insert_effect, insert_member, remove_effect, remove_member};
use semio_s_artifact_norm_en1990_mut_actions::{insert_permanent, insert_variable, remove_permanent, remove_variable};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_json<T: semio_framework_value::ToValue>(v: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(v))
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    let raw = dec_str(s)?;
    semio_framework_pack_json::from_json_str(&raw, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
/// 📍 An insert's optional `index=` argument: absent appends.
fn opt_index(args: &[(String, String)]) -> Result<Option<usize>, String> {
    args.iter().find(|(key, _)| key == "index").map_or(Ok(None), |(_, raw)| dec_json::<Option<usize>>(raw))
}
fn tokenize_args(rest: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    let mut chars = rest.chars().peekable();
    while chars.peek().is_some() {
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        let mut key = String::new();
        while let Some(c) = chars.peek().copied() {
            if c == '=' || c.is_whitespace() {
                break;
            }
            key.push(c);
            chars.next();
        }
        if key.is_empty() {
            return Err("empty key".into());
        }
        if chars.next() != Some('=') {
            return Err(format!("expected '=' after {key}"));
        }
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        let mut val = String::new();
        if chars.peek() == Some(&'"') {
            val.push(chars.next().unwrap());
            loop {
                match chars.next() {
                    Some('\\') => {
                        val.push('\\');
                        if let Some(n) = chars.next() {
                            val.push(n);
                        }
                    }
                    Some('"') => {
                        val.push('"');
                        break;
                    }
                    Some(c) => val.push(c),
                    None => return Err("unterminated string".into()),
                }
            }
        } else {
            while let Some(c) = chars.peek().copied() {
                if c.is_whitespace() {
                    break;
                }
                val.push(c);
                chars.next();
            }
        }
        out.push((key, val));
    }
    Ok(out)
}
//#endregion 🔖️ScalarCodec

//#region 🔖️OpTextCodec
fn print_en1990_mutation(mutation: &En1990Mutation) -> String {
    match mutation {
        En1990Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1990Mutation::ChangeProjectId(p) => format!("change-project-id new-project-id={}", enc_json(&p.new_project_id)),
        En1990Mutation::ChangeAltitudeM(p) => format!("change-altitude-m new-altitude-m={}", enc_json(&p.new_altitude_m)),
        En1990Mutation::ChangeConsequenceClass(p) => format!("change-consequence-class new-consequence-class={}", enc_json(&p.new_consequence_class)),
        En1990Mutation::ChangeReliabilityClass(p) => format!("change-reliability-class new-reliability-class={}", enc_json(&p.new_reliability_class)),
        En1990Mutation::ChangeDesignWorkingLifeCategory(p) => format!("change-design-working-life-category new-design-working-life-category={}", enc_json(&p.new_design_working_life_category)),
        En1990Mutation::ChangeDesignWorkingLifeYears(p) => format!("change-design-working-life-years new-design-working-life-years={}", enc_json(&p.new_design_working_life_years)),
        En1990Mutation::ChangeReferencePeriodYears(p) => format!("change-reference-period-years new-reference-period-years={}", enc_json(&p.new_reference_period_years)),
        En1990Mutation::ChangeSupervisionLevel(p) => format!("change-supervision-level new-supervision-level={}", enc_json(&p.new_supervision_level)),
        En1990Mutation::ChangeInspectionLevel(p) => format!("change-inspection-level new-inspection-level={}", enc_json(&p.new_inspection_level)),
        En1990Mutation::ChangeBetaComputed(p) => format!("change-beta-computed new-beta-computed={}", enc_json(&p.new_beta_computed)),
        En1990Mutation::InsertPermanent(p) => format!("insert-permanent index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemovePermanent(p) => format!("remove-permanent index={}", enc_json(&p.index)),
        En1990Mutation::InsertVariable(p) => format!("insert-variable index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveVariable(p) => format!("remove-variable index={}", enc_json(&p.index)),
        En1990Mutation::InsertAccidental(p) => format!("insert-accidental index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveAccidental(p) => format!("remove-accidental index={}", enc_json(&p.index)),
        En1990Mutation::InsertSeismic(p) => format!("insert-seismic index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveSeismic(p) => format!("remove-seismic index={}", enc_json(&p.index)),
        En1990Mutation::InsertMember(p) => format!("insert-member index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveMember(p) => format!("remove-member index={}", enc_json(&p.index)),
        En1990Mutation::InsertEffect(p) => format!("insert-effect index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveEffect(p) => format!("remove-effect index={}", enc_json(&p.index)),
        En1990Mutation::ChangePermanentKind(p) => format!("change-permanent-kind id={} new-kind={}", enc_json(&p.id), enc_json(&p.new_kind)),
        En1990Mutation::ChangePermanentGk(p) => format!("change-permanent-gk id={} new-gk={}", enc_json(&p.id), enc_json(&p.new_gk)),
        En1990Mutation::ChangeVariableCategory(p) => format!("change-variable-category id={} new-category={}", enc_json(&p.id), enc_json(&p.new_category)),
        En1990Mutation::ChangeVariableQk(p) => format!("change-variable-qk id={} new-qk={}", enc_json(&p.id), enc_json(&p.new_qk)),
        En1990Mutation::ChangeVariableGroup(p) => format!("change-variable-group id={} new-group={}", enc_json(&p.id), enc_json(&p.new_group)),
        En1990Mutation::ChangeAccidentalAd(p) => format!("change-accidental-ad id={} new-ad={}", enc_json(&p.id), enc_json(&p.new_ad)),
        En1990Mutation::InsertCase(p) => format!("insert-case index={} case={}", enc_json(&p.index), enc_json(&p.case)),
        En1990Mutation::RemoveCase(p) => format!("remove-case id={}", enc_json(&p.id)),
        En1990Mutation::ChangeCaseNumber(p) => format!("change-case-number case-id={} key={} value={}", enc_json(&p.case_id), enc_json(&p.key), enc_json(&p.value)),
        En1990Mutation::ChangeCaseText(p) => format!("change-case-text case-id={} key={} value={}", enc_json(&p.case_id), enc_json(&p.key), enc_json(&p.value)),
        En1990Mutation::ChangeCaseLabelEn(p) => format!("change-case-label-en case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
        En1990Mutation::ChangeCaseLabelDe(p) => format!("change-case-label-de case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
        En1990Mutation::ChangeCaseEntryId(p) => format!("change-case-entry-id case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
        En1990Mutation::ChangeSeismicAEk(p) => format!("change-seismic-a-ek id={} new-a-ek={}", enc_json(&p.id), enc_json(&p.new_a_ek)),
        En1990Mutation::ChangeSeismicImportanceClass(p) => format!("change-seismic-importance-class id={} new-importance-class={}", enc_json(&p.id), enc_json(&p.new_importance_class)),
        En1990Mutation::ChangeMemberLabelEn(p) => format!("change-member-label-en id={} new-label-en={}", enc_json(&p.id), enc_json(&p.new_label_en)),
        En1990Mutation::ChangeMemberLabelDe(p) => format!("change-member-label-de id={} new-label-de={}", enc_json(&p.id), enc_json(&p.new_label_de)),
        En1990Mutation::ChangeMemberRdStr(p) => format!("change-member-rd-str id={} new-rd-str={}", enc_json(&p.id), enc_json(&p.new_rd_str)),
        En1990Mutation::ChangeMemberRdGeo(p) => format!("change-member-rd-geo id={} new-rd-geo={}", enc_json(&p.id), enc_json(&p.new_rd_geo)),
        En1990Mutation::ChangeMemberRdEquStab(p) => format!("change-member-rd-equ-stab id={} new-rd-equ-stab={}", enc_json(&p.id), enc_json(&p.new_rd_equ_stab)),
        En1990Mutation::ChangeMemberRdEquDestab(p) => format!("change-member-rd-equ-destab id={} new-rd-equ-destab={}", enc_json(&p.id), enc_json(&p.new_rd_equ_destab)),
        En1990Mutation::ChangeMemberRdFat(p) => format!("change-member-rd-fat id={} new-rd-fat={}", enc_json(&p.id), enc_json(&p.new_rd_fat)),
        En1990Mutation::ChangeMemberSpan(p) => format!("change-member-span id={} new-span={}", enc_json(&p.id), enc_json(&p.new_span)),
        En1990Mutation::ChangeMemberPrecamber(p) => format!("change-member-precamber id={} new-precamber={}", enc_json(&p.id), enc_json(&p.new_precamber)),
        En1990Mutation::ChangeMemberDeflectionW1(p) => format!("change-member-deflection-w1 id={} new-deflection-w1={}", enc_json(&p.id), enc_json(&p.new_deflection_w1)),
        En1990Mutation::ChangeMemberDeflectionW2(p) => format!("change-member-deflection-w2 id={} new-deflection-w2={}", enc_json(&p.id), enc_json(&p.new_deflection_w2)),
        En1990Mutation::ChangeMemberDeflectionW3(p) => format!("change-member-deflection-w3 id={} new-deflection-w3={}", enc_json(&p.id), enc_json(&p.new_deflection_w3)),
        En1990Mutation::ChangeMemberDeflLimitRatio(p) => format!("change-member-defl-limit-ratio id={} new-deflection-limit-ratio={}", enc_json(&p.id), enc_json(&p.new_deflection_limit_ratio)),
        En1990Mutation::ChangeMemberDeflLimitRatioW3(p) => format!("change-member-defl-limit-ratio-w3 id={} new-deflection-limit-ratio-w3={}", enc_json(&p.id), enc_json(&p.new_deflection_limit_ratio_w3)),
        En1990Mutation::ChangeMemberSlsLimitChar(p) => format!("change-member-sls-limit-char id={} new-sls-limit-char={}", enc_json(&p.id), enc_json(&p.new_sls_limit_char)),
        En1990Mutation::ChangeMemberSlsLimitFreq(p) => format!("change-member-sls-limit-freq id={} new-sls-limit-freq={}", enc_json(&p.id), enc_json(&p.new_sls_limit_freq)),
        En1990Mutation::ChangeMemberSlsLimitQp(p) => format!("change-member-sls-limit-qp id={} new-sls-limit-qp={}", enc_json(&p.id), enc_json(&p.new_sls_limit_qp)),
        En1990Mutation::ChangeMemberVibrationFrequency(p) => format!("change-member-vibration-frequency id={} new-vibration-frequency={}", enc_json(&p.id), enc_json(&p.new_vibration_frequency)),
        En1990Mutation::ChangeMemberVibFreqMin(p) => format!("change-member-vib-freq-min id={} new-vibration-frequency-min={}", enc_json(&p.id), enc_json(&p.new_vibration_frequency_min)),
        En1990Mutation::ChangeBridgeMemberId(p) => format!("change-bridge-member-id id={} new-member-id={}", enc_json(&p.id), enc_json(&p.new_member_id)),
        En1990Mutation::ChangeBridgeDeckAcceleration(p) => format!("change-bridge-deck-acceleration id={} new-deck-acceleration={}", enc_json(&p.id), enc_json(&p.new_deck_acceleration)),
        En1990Mutation::ChangeBridgeDeckAccelLimit(p) => format!("change-bridge-deck-accel-limit id={} new-deck-acceleration-limit={}", enc_json(&p.id), enc_json(&p.new_deck_acceleration_limit)),
        En1990Mutation::ChangeBridgeDeckTwist(p) => format!("change-bridge-deck-twist id={} new-deck-twist={}", enc_json(&p.id), enc_json(&p.new_deck_twist)),
        En1990Mutation::ChangeBridgeDeckTwistLimit(p) => format!("change-bridge-deck-twist-limit id={} new-deck-twist-limit={}", enc_json(&p.id), enc_json(&p.new_deck_twist_limit)),
        En1990Mutation::ChangeBridgeDeflection(p) => format!("change-bridge-deflection id={} new-bridge-deflection={}", enc_json(&p.id), enc_json(&p.new_bridge_deflection)),
        En1990Mutation::ChangeBridgeDeflectionLimit(p) => format!("change-bridge-deflection-limit id={} new-bridge-deflection-limit={}", enc_json(&p.id), enc_json(&p.new_bridge_deflection_limit)),
        En1990Mutation::ChangeEffectMemberId(p) => format!("change-effect-member-id index={} new-member-id={}", enc_json(&p.index), enc_json(&p.new_member_id)),
        En1990Mutation::ChangeEffectActionId(p) => format!("change-effect-action-id index={} new-action-id={}", enc_json(&p.index), enc_json(&p.new_action_id)),
        En1990Mutation::ChangeEffectInfluence(p) => format!("change-effect-influence index={} new-influence={}", enc_json(&p.index), enc_json(&p.new_influence)),
        En1990Mutation::ChangeFatigueMemberId(p) => format!("change-fatigue-member-id id={} new-member-id={}", enc_json(&p.id), enc_json(&p.new_member_id)),
        En1990Mutation::ChangeFatigueDetailCategory(p) => format!("change-fatigue-detail-category id={} new-detail-category={}", enc_json(&p.id), enc_json(&p.new_detail_category)),
        En1990Mutation::ChangeFatigueStressRange(p) => format!("change-fatigue-stress-range id={} new-stress-range={}", enc_json(&p.id), enc_json(&p.new_stress_range)),
        En1990Mutation::ChangeFatigueCycles(p) => format!("change-fatigue-cycles id={} new-cycles={}", enc_json(&p.id), enc_json(&p.new_cycles)),
        En1990Mutation::ChangeReliabilityResistanceDist(p) => format!("change-reliability-resistance-dist id={} new-resistance-distribution={}", enc_json(&p.id), enc_json(&p.new_resistance_distribution)),
        En1990Mutation::ChangeReliabilityResistanceMean(p) => format!("change-reliability-resistance-mean id={} new-resistance-mean={}", enc_json(&p.id), enc_json(&p.new_resistance_mean)),
        En1990Mutation::ChangeReliabilityResistanceSd(p) => format!("change-reliability-resistance-sd id={} new-resistance-sd={}", enc_json(&p.id), enc_json(&p.new_resistance_sd)),
        En1990Mutation::ChangeReliabilityActionDist(p) => format!("change-reliability-action-dist id={} new-action-distribution={}", enc_json(&p.id), enc_json(&p.new_action_distribution)),
        En1990Mutation::ChangeReliabilityActionMean(p) => format!("change-reliability-action-mean id={} new-action-mean={}", enc_json(&p.id), enc_json(&p.new_action_mean)),
        En1990Mutation::ChangeReliabilityActionSd(p) => format!("change-reliability-action-sd id={} new-action-sd={}", enc_json(&p.id), enc_json(&p.new_action_sd)),
        En1990Mutation::ChangeTestQuantity(p) => format!("change-test-quantity id={} new-quantity={}", enc_json(&p.id), enc_json(&p.new_quantity)),
        En1990Mutation::ChangeTestPairs(p) => format!("change-test-pairs id={} new-pairs={}", enc_json(&p.id), enc_json(&p.new_pairs)),
        En1990Mutation::ChangeTestDistribution(p) => format!("change-test-distribution id={} new-distribution={}", enc_json(&p.id), enc_json(&p.new_distribution)),
        En1990Mutation::ChangeTestSampleSize(p) => format!("change-test-sample-size id={} new-sample-size={}", enc_json(&p.id), enc_json(&p.new_sample_size)),
        En1990Mutation::ChangeTestMean(p) => format!("change-test-mean id={} new-mean={}", enc_json(&p.id), enc_json(&p.new_mean)),
        En1990Mutation::ChangeTestStdDev(p) => format!("change-test-std-dev id={} new-std-dev={}", enc_json(&p.id), enc_json(&p.new_std_dev)),
        En1990Mutation::ChangeTestVariationKnown(p) => format!("change-test-variation-known id={} new-variation-known={}", enc_json(&p.id), enc_json(&p.new_variation_known)),
        En1990Mutation::ChangeTestConversionFactor(p) => format!("change-test-conversion-factor id={} new-conversion-factor={}", enc_json(&p.id), enc_json(&p.new_conversion_factor)),
        En1990Mutation::ChangeTestRequired(p) => format!("change-test-required id={} new-required={}", enc_json(&p.id), enc_json(&p.new_required)),
        En1990Mutation::ChangeTestTheoryVariation(p) => format!("change-test-theory-variation id={} new-theory-variation={}", enc_json(&p.id), enc_json(&p.new_theory_variation)),
        En1990Mutation::ChangePlacementX(p) => format!("change-placement-x id={} new-x={}", enc_json(&p.id), enc_json(&p.new_x)),
        En1990Mutation::ChangePlacementY(p) => format!("change-placement-y id={} new-y={}", enc_json(&p.id), enc_json(&p.new_y)),
        En1990Mutation::ChangePlacementZ(p) => format!("change-placement-z id={} new-z={}", enc_json(&p.id), enc_json(&p.new_z)),
        En1990Mutation::ChangePlacementYaw(p) => format!("change-placement-yaw id={} new-yaw={}", enc_json(&p.id), enc_json(&p.new_yaw)),
        En1990Mutation::ChangeStructureKind(p) => format!("change-structure-kind new-structure-kind={}", enc_json(&p.new_structure_kind)),
        En1990Mutation::InsertBridgeSls(p) => format!("insert-bridge-sls index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveBridgeSls(p) => format!("remove-bridge-sls index={}", enc_json(&p.index)),
        En1990Mutation::InsertFatigueSpectrum(p) => format!("insert-fatigue-spectrum index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveFatigueSpectrum(p) => format!("remove-fatigue-spectrum index={}", enc_json(&p.index)),
        En1990Mutation::InsertReliabilityAnalysis(p) => format!("insert-reliability-analysis index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveReliabilityAnalysis(p) => format!("remove-reliability-analysis index={}", enc_json(&p.index)),
        En1990Mutation::InsertTestSeries(p) => format!("insert-test-series index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveTestSeries(p) => format!("remove-test-series index={}", enc_json(&p.index)),
        En1990Mutation::InsertPlacement(p) => format!("insert-placement index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemovePlacement(p) => format!("remove-placement index={}", enc_json(&p.index)),
        En1990Mutation::ChangeMemberFrameHeight(p) => format!("change-member-frame-height id={} new-frame-height={}", enc_json(&p.id), enc_json(&p.new_frame_height)),
        En1990Mutation::ChangeMemberFrameDrift(p) => format!("change-member-frame-drift id={} new-frame-drift={}", enc_json(&p.id), enc_json(&p.new_frame_drift)),
        En1990Mutation::ChangeMemberStoreyHeight(p) => format!("change-member-storey-height id={} new-storey-height={}", enc_json(&p.id), enc_json(&p.new_storey_height)),
        En1990Mutation::ChangeMemberStoreyDrift(p) => format!("change-member-storey-drift id={} new-storey-drift={}", enc_json(&p.id), enc_json(&p.new_storey_drift)),
        En1990Mutation::ChangeBridgeTrackGauge(p) => format!("change-bridge-track-gauge id={} new-track-gauge={}", enc_json(&p.id), enc_json(&p.new_track_gauge)),
        En1990Mutation::ChangeBridgeLineSpeed(p) => format!("change-bridge-line-speed id={} new-line-speed={}", enc_json(&p.id), enc_json(&p.new_line_speed)),
        En1990Mutation::ChangeBridgeBallastDepth(p) => format!("change-bridge-ballast-depth id={} new-ballast-depth={}", enc_json(&p.id), enc_json(&p.new_ballast_depth)),
        En1990Mutation::ChangeBridgeDeckMass(p) => format!("change-bridge-deck-mass id={} new-deck-mass={}", enc_json(&p.id), enc_json(&p.new_deck_mass)),
        En1990Mutation::ChangeBridgeNaturalFrequency(p) => format!("change-bridge-natural-frequency id={} new-natural-frequency={}", enc_json(&p.id), enc_json(&p.new_natural_frequency)),
        En1990Mutation::ChangeBridgeEndRotation(p) => format!("change-bridge-end-rotation id={} new-end-rotation={}", enc_json(&p.id), enc_json(&p.new_end_rotation)),
        En1990Mutation::ChangeBridgeBasicWindSpeed(p) => format!("change-bridge-basic-wind-speed id={} new-basic-wind-speed={}", enc_json(&p.id), enc_json(&p.new_basic_wind_speed)),
        En1990Mutation::ChangeMemberMaterialStrengthK(p) => format!("change-member-material-strength-k id={} new-material-strength-k={}", enc_json(&p.id), enc_json(&p.new_material_strength_k)),
        En1990Mutation::ChangeMemberMaterialConvFactor(p) => format!("change-member-material-conv-factor id={} new-material-conversion-factor={}", enc_json(&p.id), enc_json(&p.new_material_conversion_factor)),
        En1990Mutation::ChangeMemberMatPartialFactor(p) => format!("change-member-mat-partial-factor id={} new-material-partial-factor={}", enc_json(&p.id), enc_json(&p.new_material_partial_factor)),
        En1990Mutation::ChangeMemberMatDesignStrength(p) => format!("change-member-mat-design-strength id={} new-material-design-strength={}", enc_json(&p.id), enc_json(&p.new_material_design_strength)),
        En1990Mutation::ChangeMemberNominalDimension(p) => format!("change-member-nominal-dimension id={} new-nominal-dimension={}", enc_json(&p.id), enc_json(&p.new_nominal_dimension)),
        En1990Mutation::ChangeMemberDimensionTolerance(p) => format!("change-member-dimension-tolerance id={} new-dimension-tolerance={}", enc_json(&p.id), enc_json(&p.new_dimension_tolerance)),
        En1990Mutation::ChangeMemberDesignDimension(p) => format!("change-member-design-dimension id={} new-design-dimension={}", enc_json(&p.id), enc_json(&p.new_design_dimension)),
        En1990Mutation::ChangeMemberMemberAlphaCr(p) => format!("change-member-member-alpha-cr id={} new-member-alpha-cr={}", enc_json(&p.id), enc_json(&p.new_member_alpha_cr)),
        En1990Mutation::ChangeMemberSecondOrderAnalysis(p) => format!("change-member-second-order-analysis id={} new-second-order-analysis={}", enc_json(&p.id), enc_json(&p.new_second_order_analysis)),
        En1990Mutation::ChangeBridgeDampingRatio(p) => format!("change-bridge-damping-ratio id={} new-damping-ratio={}", enc_json(&p.id), enc_json(&p.new_damping_ratio)),
        En1990Mutation::ChangeFactorOverrideFactor(p) => format!("change-factor-override-factor id={} new-factor={}", enc_json(&p.id), enc_json(&p.new_factor)),
        En1990Mutation::ChangeFactorOverrideTarget(p) => format!("change-factor-override-target id={} new-target={}", enc_json(&p.id), enc_json(&p.new_target)),
        En1990Mutation::ChangeFactorOverrideValue(p) => format!("change-factor-override-value id={} new-value={}", enc_json(&p.id), enc_json(&p.new_value)),
        En1990Mutation::ChangeFactorOverrideJustif(p) => format!("change-factor-override-justif id={} new-justification={}", enc_json(&p.id), enc_json(&p.new_justification)),
        En1990Mutation::InsertFactorOverride(p) => format!("insert-factor-override index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveFactorOverride(p) => format!("remove-factor-override index={}", enc_json(&p.index)),
        En1990Mutation::ChangeTrafficGroupGroup(p) => format!("change-traffic-group-group id={} new-group={}", enc_json(&p.id), enc_json(&p.new_group)),
        En1990Mutation::ChangeTrafficGroupVariableId(p) => format!("change-traffic-group-variable-id id={} new-variable-id={}", enc_json(&p.id), enc_json(&p.new_variable_id)),
        En1990Mutation::ChangeReliabilityMemberId(p) => format!("change-reliability-member-id id={} new-member-id={}", enc_json(&p.id), enc_json(&p.new_member_id)),
        En1990Mutation::InsertTrafficGroup(p) => format!("insert-traffic-group index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveTrafficGroup(p) => format!("remove-traffic-group index={}", enc_json(&p.index)),
    }
}

fn parse_en1990_mutation(text: &str) -> Result<En1990Mutation, String> {
    let text = text.trim();
    let (keyword, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    let args = tokenize_args(rest)?;
    let arg = |k: &str| -> Result<&str, String> { args.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str()).ok_or_else(|| format!("missing {k}")) };
    match keyword {
        "change-annex" => Ok(En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: dec_json(arg("new-annex")?)? })),
        "change-project-id" => Ok(En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: dec_json(arg("new-project-id")?)? })),
        "change-altitude-m" => Ok(En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: dec_json(arg("new-altitude-m")?)? })),
        "change-consequence-class" => Ok(En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: dec_json(arg("new-consequence-class")?)? })),
        "change-reliability-class" => Ok(En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: dec_json(arg("new-reliability-class")?)? })),
        "change-design-working-life-category" => Ok(En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: dec_json(arg("new-design-working-life-category")?)? })),
        "change-design-working-life-years" => Ok(En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: dec_json(arg("new-design-working-life-years")?)? })),
        "change-reference-period-years" => Ok(En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: dec_json(arg("new-reference-period-years")?)? })),
        "change-supervision-level" => Ok(En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: dec_json(arg("new-supervision-level")?)? })),
        "change-inspection-level" => Ok(En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: dec_json(arg("new-inspection-level")?)? })),
        "change-beta-computed" => Ok(En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: dec_json(arg("new-beta-computed")?)? })),
        "insert-permanent" => Ok(En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-permanent" => Ok(En1990Mutation::RemovePermanent(remove_permanent::RemovePermanent { index: dec_json(arg("index")?)? })),
        "insert-variable" => Ok(En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-variable" => Ok(En1990Mutation::RemoveVariable(remove_variable::RemoveVariable { index: dec_json(arg("index")?)? })),
        "insert-accidental" => Ok(En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-accidental" => Ok(En1990Mutation::RemoveAccidental(remove_accidental::RemoveAccidental { index: dec_json(arg("index")?)? })),
        "insert-seismic" => Ok(En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-seismic" => Ok(En1990Mutation::RemoveSeismic(remove_seismic::RemoveSeismic { index: dec_json(arg("index")?)? })),
        "insert-member" => Ok(En1990Mutation::InsertMember(insert_member::InsertMember { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-member" => Ok(En1990Mutation::RemoveMember(remove_member::RemoveMember { index: dec_json(arg("index")?)? })),
        "insert-effect" => Ok(En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-effect" => Ok(En1990Mutation::RemoveEffect(remove_effect::RemoveEffect { index: dec_json(arg("index")?)? })),
        "change-permanent-kind" => Ok(En1990Mutation::ChangePermanentKind(change_permanent_kind::ChangePermanentKind { id: dec_json(arg("id")?)?, new_kind: dec_json(arg("new-kind")?)? })),
        "change-permanent-gk" => Ok(En1990Mutation::ChangePermanentGk(change_permanent_gk::ChangePermanentGk { id: dec_json(arg("id")?)?, new_gk: dec_json(arg("new-gk")?)? })),
        "change-variable-category" => Ok(En1990Mutation::ChangeVariableCategory(change_variable_category::ChangeVariableCategory { id: dec_json(arg("id")?)?, new_category: dec_json(arg("new-category")?)? })),
        "change-variable-qk" => Ok(En1990Mutation::ChangeVariableQk(change_variable_qk::ChangeVariableQk { id: dec_json(arg("id")?)?, new_qk: dec_json(arg("new-qk")?)? })),
        "change-variable-group" => Ok(En1990Mutation::ChangeVariableGroup(change_variable_group::ChangeVariableGroup { id: dec_json(arg("id")?)?, new_group: dec_json(arg("new-group")?)? })),
        "change-accidental-ad" => Ok(En1990Mutation::ChangeAccidentalAd(change_accidental_ad::ChangeAccidentalAd { id: dec_json(arg("id")?)?, new_ad: dec_json(arg("new-ad")?)? })),
        "change-seismic-a-ek" => Ok(En1990Mutation::ChangeSeismicAEk(change_seismic_a_ek::ChangeSeismicAEk { id: dec_json(arg("id")?)?, new_a_ek: dec_json(arg("new-a-ek")?)? })),
        "change-seismic-importance-class" => Ok(En1990Mutation::ChangeSeismicImportanceClass(change_seismic_importance_class::ChangeSeismicImportanceClass { id: dec_json(arg("id")?)?, new_importance_class: dec_json(arg("new-importance-class")?)? })),
        "change-member-label-en" => Ok(En1990Mutation::ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn { id: dec_json(arg("id")?)?, new_label_en: dec_json(arg("new-label-en")?)? })),
        "change-member-label-de" => Ok(En1990Mutation::ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe { id: dec_json(arg("id")?)?, new_label_de: dec_json(arg("new-label-de")?)? })),
        "change-member-rd-str" => Ok(En1990Mutation::ChangeMemberRdStr(change_member_rd_str::ChangeMemberRdStr { id: dec_json(arg("id")?)?, new_rd_str: dec_json(arg("new-rd-str")?)? })),
        "change-member-rd-geo" => Ok(En1990Mutation::ChangeMemberRdGeo(change_member_rd_geo::ChangeMemberRdGeo { id: dec_json(arg("id")?)?, new_rd_geo: dec_json(arg("new-rd-geo")?)? })),
        "change-member-rd-equ-stab" => Ok(En1990Mutation::ChangeMemberRdEquStab(change_member_rd_equ_stab::ChangeMemberRdEquStab { id: dec_json(arg("id")?)?, new_rd_equ_stab: dec_json(arg("new-rd-equ-stab")?)? })),
        "change-member-rd-equ-destab" => Ok(En1990Mutation::ChangeMemberRdEquDestab(change_member_rd_equ_destab::ChangeMemberRdEquDestab { id: dec_json(arg("id")?)?, new_rd_equ_destab: dec_json(arg("new-rd-equ-destab")?)? })),
        "change-member-rd-fat" => Ok(En1990Mutation::ChangeMemberRdFat(change_member_rd_fat::ChangeMemberRdFat { id: dec_json(arg("id")?)?, new_rd_fat: dec_json(arg("new-rd-fat")?)? })),
        "change-member-span" => Ok(En1990Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { id: dec_json(arg("id")?)?, new_span: dec_json(arg("new-span")?)? })),
        "change-member-precamber" => Ok(En1990Mutation::ChangeMemberPrecamber(change_member_precamber::ChangeMemberPrecamber { id: dec_json(arg("id")?)?, new_precamber: dec_json(arg("new-precamber")?)? })),
        "change-member-deflection-w1" => Ok(En1990Mutation::ChangeMemberDeflectionW1(change_member_deflection_w1::ChangeMemberDeflectionW1 { id: dec_json(arg("id")?)?, new_deflection_w1: dec_json(arg("new-deflection-w1")?)? })),
        "change-member-deflection-w2" => Ok(En1990Mutation::ChangeMemberDeflectionW2(change_member_deflection_w2::ChangeMemberDeflectionW2 { id: dec_json(arg("id")?)?, new_deflection_w2: dec_json(arg("new-deflection-w2")?)? })),
        "change-member-deflection-w3" => Ok(En1990Mutation::ChangeMemberDeflectionW3(change_member_deflection_w3::ChangeMemberDeflectionW3 { id: dec_json(arg("id")?)?, new_deflection_w3: dec_json(arg("new-deflection-w3")?)? })),
        "change-member-defl-limit-ratio" => Ok(En1990Mutation::ChangeMemberDeflLimitRatio(change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio { id: dec_json(arg("id")?)?, new_deflection_limit_ratio: dec_json(arg("new-deflection-limit-ratio")?)? })),
        "change-member-defl-limit-ratio-w3" => Ok(En1990Mutation::ChangeMemberDeflLimitRatioW3(change_member_defl_limit_ratio_w3::ChangeMemberDeflLimitRatioW3 { id: dec_json(arg("id")?)?, new_deflection_limit_ratio_w3: dec_json(arg("new-deflection-limit-ratio-w3")?)? })),
        "change-member-sls-limit-char" => Ok(En1990Mutation::ChangeMemberSlsLimitChar(change_member_sls_limit_char::ChangeMemberSlsLimitChar { id: dec_json(arg("id")?)?, new_sls_limit_char: dec_json(arg("new-sls-limit-char")?)? })),
        "change-member-sls-limit-freq" => Ok(En1990Mutation::ChangeMemberSlsLimitFreq(change_member_sls_limit_freq::ChangeMemberSlsLimitFreq { id: dec_json(arg("id")?)?, new_sls_limit_freq: dec_json(arg("new-sls-limit-freq")?)? })),
        "change-member-sls-limit-qp" => Ok(En1990Mutation::ChangeMemberSlsLimitQp(change_member_sls_limit_qp::ChangeMemberSlsLimitQp { id: dec_json(arg("id")?)?, new_sls_limit_qp: dec_json(arg("new-sls-limit-qp")?)? })),
        "change-member-vibration-frequency" => Ok(En1990Mutation::ChangeMemberVibrationFrequency(change_member_vibration_frequency::ChangeMemberVibrationFrequency { id: dec_json(arg("id")?)?, new_vibration_frequency: dec_json(arg("new-vibration-frequency")?)? })),
        "change-member-vib-freq-min" => Ok(En1990Mutation::ChangeMemberVibFreqMin(change_member_vib_freq_min::ChangeMemberVibFreqMin { id: dec_json(arg("id")?)?, new_vibration_frequency_min: dec_json(arg("new-vibration-frequency-min")?)? })),
        "change-bridge-member-id" => Ok(En1990Mutation::ChangeBridgeMemberId(change_bridge_member_id::ChangeBridgeMemberId { id: dec_json(arg("id")?)?, new_member_id: dec_json(arg("new-member-id")?)? })),
        "change-bridge-deck-acceleration" => Ok(En1990Mutation::ChangeBridgeDeckAcceleration(change_bridge_deck_acceleration::ChangeBridgeDeckAcceleration { id: dec_json(arg("id")?)?, new_deck_acceleration: dec_json(arg("new-deck-acceleration")?)? })),
        "change-bridge-deck-accel-limit" => Ok(En1990Mutation::ChangeBridgeDeckAccelLimit(change_bridge_deck_accel_limit::ChangeBridgeDeckAccelLimit { id: dec_json(arg("id")?)?, new_deck_acceleration_limit: dec_json(arg("new-deck-acceleration-limit")?)? })),
        "change-bridge-deck-twist" => Ok(En1990Mutation::ChangeBridgeDeckTwist(change_bridge_deck_twist::ChangeBridgeDeckTwist { id: dec_json(arg("id")?)?, new_deck_twist: dec_json(arg("new-deck-twist")?)? })),
        "change-bridge-deck-twist-limit" => Ok(En1990Mutation::ChangeBridgeDeckTwistLimit(change_bridge_deck_twist_limit::ChangeBridgeDeckTwistLimit { id: dec_json(arg("id")?)?, new_deck_twist_limit: dec_json(arg("new-deck-twist-limit")?)? })),
        "change-bridge-deflection" => Ok(En1990Mutation::ChangeBridgeDeflection(change_bridge_deflection::ChangeBridgeDeflection { id: dec_json(arg("id")?)?, new_bridge_deflection: dec_json(arg("new-bridge-deflection")?)? })),
        "change-bridge-deflection-limit" => Ok(En1990Mutation::ChangeBridgeDeflectionLimit(change_bridge_deflection_limit::ChangeBridgeDeflectionLimit { id: dec_json(arg("id")?)?, new_bridge_deflection_limit: dec_json(arg("new-bridge-deflection-limit")?)? })),
        "change-effect-member-id" => Ok(En1990Mutation::ChangeEffectMemberId(change_effect_member_id::ChangeEffectMemberId { index: dec_json(arg("index")?)?, new_member_id: dec_json(arg("new-member-id")?)? })),
        "change-effect-action-id" => Ok(En1990Mutation::ChangeEffectActionId(change_effect_action_id::ChangeEffectActionId { index: dec_json(arg("index")?)?, new_action_id: dec_json(arg("new-action-id")?)? })),
        "change-effect-influence" => Ok(En1990Mutation::ChangeEffectInfluence(change_effect_influence::ChangeEffectInfluence { index: dec_json(arg("index")?)?, new_influence: dec_json(arg("new-influence")?)? })),
        "change-fatigue-member-id" => Ok(En1990Mutation::ChangeFatigueMemberId(change_fatigue_member_id::ChangeFatigueMemberId { id: dec_json(arg("id")?)?, new_member_id: dec_json(arg("new-member-id")?)? })),
        "change-fatigue-detail-category" => Ok(En1990Mutation::ChangeFatigueDetailCategory(change_fatigue_detail_category::ChangeFatigueDetailCategory { id: dec_json(arg("id")?)?, new_detail_category: dec_json(arg("new-detail-category")?)? })),
        "change-fatigue-stress-range" => Ok(En1990Mutation::ChangeFatigueStressRange(change_fatigue_stress_range::ChangeFatigueStressRange { id: dec_json(arg("id")?)?, new_stress_range: dec_json(arg("new-stress-range")?)? })),
        "change-fatigue-cycles" => Ok(En1990Mutation::ChangeFatigueCycles(change_fatigue_cycles::ChangeFatigueCycles { id: dec_json(arg("id")?)?, new_cycles: dec_json(arg("new-cycles")?)? })),
        "change-reliability-resistance-dist" => Ok(En1990Mutation::ChangeReliabilityResistanceDist(change_reliability_resistance_dist::ChangeReliabilityResistanceDist { id: dec_json(arg("id")?)?, new_resistance_distribution: dec_json(arg("new-resistance-distribution")?)? })),
        "change-reliability-resistance-mean" => Ok(En1990Mutation::ChangeReliabilityResistanceMean(change_reliability_resistance_mean::ChangeReliabilityResistanceMean { id: dec_json(arg("id")?)?, new_resistance_mean: dec_json(arg("new-resistance-mean")?)? })),
        "change-reliability-resistance-sd" => Ok(En1990Mutation::ChangeReliabilityResistanceSd(change_reliability_resistance_sd::ChangeReliabilityResistanceSd { id: dec_json(arg("id")?)?, new_resistance_sd: dec_json(arg("new-resistance-sd")?)? })),
        "change-reliability-action-dist" => Ok(En1990Mutation::ChangeReliabilityActionDist(change_reliability_action_dist::ChangeReliabilityActionDist { id: dec_json(arg("id")?)?, new_action_distribution: dec_json(arg("new-action-distribution")?)? })),
        "change-reliability-action-mean" => Ok(En1990Mutation::ChangeReliabilityActionMean(change_reliability_action_mean::ChangeReliabilityActionMean { id: dec_json(arg("id")?)?, new_action_mean: dec_json(arg("new-action-mean")?)? })),
        "change-reliability-action-sd" => Ok(En1990Mutation::ChangeReliabilityActionSd(change_reliability_action_sd::ChangeReliabilityActionSd { id: dec_json(arg("id")?)?, new_action_sd: dec_json(arg("new-action-sd")?)? })),
        "change-test-quantity" => Ok(En1990Mutation::ChangeTestQuantity(change_test_quantity::ChangeTestQuantity { id: dec_json(arg("id")?)?, new_quantity: dec_json(arg("new-quantity")?)? })),
        "change-test-pairs" => Ok(En1990Mutation::ChangeTestPairs(change_test_pairs::ChangeTestPairs { id: dec_json(arg("id")?)?, new_pairs: dec_json(arg("new-pairs")?)? })),
        "change-test-distribution" => Ok(En1990Mutation::ChangeTestDistribution(change_test_distribution::ChangeTestDistribution { id: dec_json(arg("id")?)?, new_distribution: dec_json(arg("new-distribution")?)? })),
        "change-test-sample-size" => Ok(En1990Mutation::ChangeTestSampleSize(change_test_sample_size::ChangeTestSampleSize { id: dec_json(arg("id")?)?, new_sample_size: dec_json(arg("new-sample-size")?)? })),
        "change-test-mean" => Ok(En1990Mutation::ChangeTestMean(change_test_mean::ChangeTestMean { id: dec_json(arg("id")?)?, new_mean: dec_json(arg("new-mean")?)? })),
        "change-test-std-dev" => Ok(En1990Mutation::ChangeTestStdDev(change_test_std_dev::ChangeTestStdDev { id: dec_json(arg("id")?)?, new_std_dev: dec_json(arg("new-std-dev")?)? })),
        "change-test-variation-known" => Ok(En1990Mutation::ChangeTestVariationKnown(change_test_variation_known::ChangeTestVariationKnown { id: dec_json(arg("id")?)?, new_variation_known: dec_json(arg("new-variation-known")?)? })),
        "change-test-conversion-factor" => Ok(En1990Mutation::ChangeTestConversionFactor(change_test_conversion_factor::ChangeTestConversionFactor { id: dec_json(arg("id")?)?, new_conversion_factor: dec_json(arg("new-conversion-factor")?)? })),
        "change-test-required" => Ok(En1990Mutation::ChangeTestRequired(change_test_required::ChangeTestRequired { id: dec_json(arg("id")?)?, new_required: dec_json(arg("new-required")?)? })),
        "change-test-theory-variation" => Ok(En1990Mutation::ChangeTestTheoryVariation(change_test_theory_variation::ChangeTestTheoryVariation { id: dec_json(arg("id")?)?, new_theory_variation: dec_json(arg("new-theory-variation")?)? })),
        "change-placement-x" => Ok(En1990Mutation::ChangePlacementX(change_placement_x::ChangePlacementX { id: dec_json(arg("id")?)?, new_x: dec_json(arg("new-x")?)? })),
        "change-placement-y" => Ok(En1990Mutation::ChangePlacementY(change_placement_y::ChangePlacementY { id: dec_json(arg("id")?)?, new_y: dec_json(arg("new-y")?)? })),
        "change-placement-z" => Ok(En1990Mutation::ChangePlacementZ(change_placement_z::ChangePlacementZ { id: dec_json(arg("id")?)?, new_z: dec_json(arg("new-z")?)? })),
        "change-placement-yaw" => Ok(En1990Mutation::ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw { id: dec_json(arg("id")?)?, new_yaw: dec_json(arg("new-yaw")?)? })),
        "change-structure-kind" => Ok(En1990Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: dec_json(arg("new-structure-kind")?)? })),
        "insert-bridge-sls" => Ok(En1990Mutation::InsertBridgeSls(insert_bridge_sls::InsertBridgeSls { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-bridge-sls" => Ok(En1990Mutation::RemoveBridgeSls(remove_bridge_sls::RemoveBridgeSls { index: dec_json(arg("index")?)? })),
        "insert-fatigue-spectrum" => Ok(En1990Mutation::InsertFatigueSpectrum(insert_fatigue_spectrum::InsertFatigueSpectrum { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-fatigue-spectrum" => Ok(En1990Mutation::RemoveFatigueSpectrum(remove_fatigue_spectrum::RemoveFatigueSpectrum { index: dec_json(arg("index")?)? })),
        "insert-reliability-analysis" => Ok(En1990Mutation::InsertReliabilityAnalysis(insert_reliability_analysis::InsertReliabilityAnalysis { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-reliability-analysis" => Ok(En1990Mutation::RemoveReliabilityAnalysis(remove_reliability_analysis::RemoveReliabilityAnalysis { index: dec_json(arg("index")?)? })),
        "insert-test-series" => Ok(En1990Mutation::InsertTestSeries(insert_test_series::InsertTestSeries { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-test-series" => Ok(En1990Mutation::RemoveTestSeries(remove_test_series::RemoveTestSeries { index: dec_json(arg("index")?)? })),
        "insert-placement" => Ok(En1990Mutation::InsertPlacement(insert_placement::InsertPlacement { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-placement" => Ok(En1990Mutation::RemovePlacement(remove_placement::RemovePlacement { index: dec_json(arg("index")?)? })),
        "insert-case" => Ok(En1990Mutation::InsertCase(insert_case::InsertCase { index: dec_json(arg("index")?)?, case: dec_json(arg("case")?)? })),
        "remove-case" => Ok(En1990Mutation::RemoveCase(remove_case::RemoveCase { id: dec_json(arg("id")?)? })),
        "change-case-number" => Ok(En1990Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: dec_json(arg("case-id")?)?, key: dec_json(arg("key")?)?, value: dec_json(arg("value")?)? })),
        "change-case-text" => Ok(En1990Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: dec_json(arg("case-id")?)?, key: dec_json(arg("key")?)?, value: dec_json(arg("value")?)? })),
        "change-case-label-en" => Ok(En1990Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-case-label-de" => Ok(En1990Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-case-entry-id" => Ok(En1990Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-member-frame-height" => Ok(En1990Mutation::ChangeMemberFrameHeight(change_member_frame_height::ChangeMemberFrameHeight { id: dec_json(arg("id")?)?, new_frame_height: dec_json(arg("new-frame-height")?)? })),
        "change-member-frame-drift" => Ok(En1990Mutation::ChangeMemberFrameDrift(change_member_frame_drift::ChangeMemberFrameDrift { id: dec_json(arg("id")?)?, new_frame_drift: dec_json(arg("new-frame-drift")?)? })),
        "change-member-storey-height" => Ok(En1990Mutation::ChangeMemberStoreyHeight(change_member_storey_height::ChangeMemberStoreyHeight { id: dec_json(arg("id")?)?, new_storey_height: dec_json(arg("new-storey-height")?)? })),
        "change-member-storey-drift" => Ok(En1990Mutation::ChangeMemberStoreyDrift(change_member_storey_drift::ChangeMemberStoreyDrift { id: dec_json(arg("id")?)?, new_storey_drift: dec_json(arg("new-storey-drift")?)? })),
        "change-bridge-track-gauge" => Ok(En1990Mutation::ChangeBridgeTrackGauge(change_bridge_track_gauge::ChangeBridgeTrackGauge { id: dec_json(arg("id")?)?, new_track_gauge: dec_json(arg("new-track-gauge")?)? })),
        "change-bridge-line-speed" => Ok(En1990Mutation::ChangeBridgeLineSpeed(change_bridge_line_speed::ChangeBridgeLineSpeed { id: dec_json(arg("id")?)?, new_line_speed: dec_json(arg("new-line-speed")?)? })),
        "change-bridge-ballast-depth" => Ok(En1990Mutation::ChangeBridgeBallastDepth(change_bridge_ballast_depth::ChangeBridgeBallastDepth { id: dec_json(arg("id")?)?, new_ballast_depth: dec_json(arg("new-ballast-depth")?)? })),
        "change-bridge-deck-mass" => Ok(En1990Mutation::ChangeBridgeDeckMass(change_bridge_deck_mass::ChangeBridgeDeckMass { id: dec_json(arg("id")?)?, new_deck_mass: dec_json(arg("new-deck-mass")?)? })),
        "change-bridge-natural-frequency" => Ok(En1990Mutation::ChangeBridgeNaturalFrequency(change_bridge_natural_frequency::ChangeBridgeNaturalFrequency { id: dec_json(arg("id")?)?, new_natural_frequency: dec_json(arg("new-natural-frequency")?)? })),
        "change-bridge-end-rotation" => Ok(En1990Mutation::ChangeBridgeEndRotation(change_bridge_end_rotation::ChangeBridgeEndRotation { id: dec_json(arg("id")?)?, new_end_rotation: dec_json(arg("new-end-rotation")?)? })),
        "change-bridge-basic-wind-speed" => Ok(En1990Mutation::ChangeBridgeBasicWindSpeed(change_bridge_basic_wind_speed::ChangeBridgeBasicWindSpeed { id: dec_json(arg("id")?)?, new_basic_wind_speed: dec_json(arg("new-basic-wind-speed")?)? })),
        "change-member-material-strength-k" => Ok(En1990Mutation::ChangeMemberMaterialStrengthK(change_member_material_strength_k::ChangeMemberMaterialStrengthK { id: dec_json(arg("id")?)?, new_material_strength_k: dec_json(arg("new-material-strength-k")?)? })),
        "change-member-material-conv-factor" => Ok(En1990Mutation::ChangeMemberMaterialConvFactor(change_member_material_conv_factor::ChangeMemberMaterialConvFactor { id: dec_json(arg("id")?)?, new_material_conversion_factor: dec_json(arg("new-material-conversion-factor")?)? })),
        "change-member-mat-partial-factor" => Ok(En1990Mutation::ChangeMemberMatPartialFactor(change_member_mat_partial_factor::ChangeMemberMatPartialFactor { id: dec_json(arg("id")?)?, new_material_partial_factor: dec_json(arg("new-material-partial-factor")?)? })),
        "change-member-mat-design-strength" => Ok(En1990Mutation::ChangeMemberMatDesignStrength(change_member_mat_design_strength::ChangeMemberMatDesignStrength { id: dec_json(arg("id")?)?, new_material_design_strength: dec_json(arg("new-material-design-strength")?)? })),
        "change-member-nominal-dimension" => Ok(En1990Mutation::ChangeMemberNominalDimension(change_member_nominal_dimension::ChangeMemberNominalDimension { id: dec_json(arg("id")?)?, new_nominal_dimension: dec_json(arg("new-nominal-dimension")?)? })),
        "change-member-dimension-tolerance" => Ok(En1990Mutation::ChangeMemberDimensionTolerance(change_member_dimension_tolerance::ChangeMemberDimensionTolerance { id: dec_json(arg("id")?)?, new_dimension_tolerance: dec_json(arg("new-dimension-tolerance")?)? })),
        "change-member-design-dimension" => Ok(En1990Mutation::ChangeMemberDesignDimension(change_member_design_dimension::ChangeMemberDesignDimension { id: dec_json(arg("id")?)?, new_design_dimension: dec_json(arg("new-design-dimension")?)? })),
        "change-member-member-alpha-cr" => Ok(En1990Mutation::ChangeMemberMemberAlphaCr(change_member_member_alpha_cr::ChangeMemberMemberAlphaCr { id: dec_json(arg("id")?)?, new_member_alpha_cr: dec_json(arg("new-member-alpha-cr")?)? })),
        "change-member-second-order-analysis" => Ok(En1990Mutation::ChangeMemberSecondOrderAnalysis(change_member_second_order_analysis::ChangeMemberSecondOrderAnalysis { id: dec_json(arg("id")?)?, new_second_order_analysis: dec_json(arg("new-second-order-analysis")?)? })),
        "change-bridge-damping-ratio" => Ok(En1990Mutation::ChangeBridgeDampingRatio(change_bridge_damping_ratio::ChangeBridgeDampingRatio { id: dec_json(arg("id")?)?, new_damping_ratio: dec_json(arg("new-damping-ratio")?)? })),
        "change-factor-override-factor" => Ok(En1990Mutation::ChangeFactorOverrideFactor(change_factor_override_factor::ChangeFactorOverrideFactor { id: dec_json(arg("id")?)?, new_factor: dec_json(arg("new-factor")?)? })),
        "change-factor-override-target" => Ok(En1990Mutation::ChangeFactorOverrideTarget(change_factor_override_target::ChangeFactorOverrideTarget { id: dec_json(arg("id")?)?, new_target: dec_json(arg("new-target")?)? })),
        "change-factor-override-value" => Ok(En1990Mutation::ChangeFactorOverrideValue(change_factor_override_value::ChangeFactorOverrideValue { id: dec_json(arg("id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-factor-override-justif" => Ok(En1990Mutation::ChangeFactorOverrideJustif(change_factor_override_justif::ChangeFactorOverrideJustif { id: dec_json(arg("id")?)?, new_justification: dec_json(arg("new-justification")?)? })),
        "insert-factor-override" => Ok(En1990Mutation::InsertFactorOverride(insert_factor_override::InsertFactorOverride { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-factor-override" => Ok(En1990Mutation::RemoveFactorOverride(remove_factor_override::RemoveFactorOverride { index: dec_json(arg("index")?)? })),
        "change-traffic-group-group" => Ok(En1990Mutation::ChangeTrafficGroupGroup(change_traffic_group_group::ChangeTrafficGroupGroup { id: dec_json(arg("id")?)?, new_group: dec_json(arg("new-group")?)? })),
        "change-traffic-group-variable-id" => Ok(En1990Mutation::ChangeTrafficGroupVariableId(change_traffic_group_variable_id::ChangeTrafficGroupVariableId { id: dec_json(arg("id")?)?, new_variable_id: dec_json(arg("new-variable-id")?)? })),
        "change-reliability-member-id" => Ok(En1990Mutation::ChangeReliabilityMemberId(change_reliability_member_id::ChangeReliabilityMemberId { id: dec_json(arg("id")?)?, new_member_id: dec_json(arg("new-member-id")?)? })),
        "insert-traffic-group" => Ok(En1990Mutation::InsertTrafficGroup(insert_traffic_group::InsertTrafficGroup { index: opt_index(&args)?, item: dec_json(arg("item")?)? })),
        "remove-traffic-group" => Ok(En1990Mutation::RemoveTrafficGroup(remove_traffic_group::RemoveTrafficGroup { index: dec_json(arg("index")?)? })),
        other => Err(format!("unknown keyword {other}")),
    }
}

impl protocol::OpText for En1990Mutation {
    fn print_op(&self) -> String {
        print_en1990_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_en1990_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpTextCodec

//#region 🔖️DemoCases
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1990Mutation> {
    let base = semio_s_artifact_norm_en1990_model::En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    vec![
        En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: semio_s_plugin_norm_compliance::AnnexChoice::En }),
        En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: "x".into() }),
        En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: 1200.0 }),
        En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 }),
        En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: semio_s_artifact_norm_en1990_model::ReliabilityClass::RC3 }),
        En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: 5 }),
        En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: 100.0 }),
        En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: 100.0 }),
        En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: semio_s_artifact_norm_en1990_model::SupervisionLevel::Dsl3 }),
        En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: semio_s_artifact_norm_en1990_model::InspectionLevel::Il3 }),
        En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: 4.3 }),        En1990Mutation::InsertCase(insert_case::InsertCase { index: 0, case: base.cases[0].clone() }),
        En1990Mutation::RemoveCase(remove_case::RemoveCase { id: "case-1".into() }),
        En1990Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: base.cases[0].id.clone(), key: "years".into(), value: Some(100.0) }),
        En1990Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: base.cases[0].id.clone(), key: "years".into(), value: None }),
        En1990Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: base.cases[0].id.clone(), key: "category".into(), value: Some("office".into()) }),
        En1990Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: base.cases[0].id.clone(), new_value: "Durability".into() }),
        En1990Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: base.cases[0].id.clone(), new_value: "Dauerhaftigkeit".into() }),
        En1990Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: base.cases[0].id.clone(), new_value: "E90.04".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::*;
use crate::*;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_s_artifact_norm_en1990_mut_design_basis::change_annex;
use semio_s_artifact_norm_en1990_mut_design_basis::change_project_id;
use semio_s_artifact_norm_en1990_mut_design_basis::change_altitude_m;
use semio_s_artifact_norm_en1990_mut_reliability::change_consequence_class;
use semio_s_artifact_norm_en1990_mut_reliability::change_reliability_class;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_category;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_years;
use semio_s_artifact_norm_en1990_mut_design_basis::change_reference_period_years;
use semio_s_artifact_norm_en1990_mut_reliability::change_supervision_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_inspection_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_beta_computed;
use semio_s_artifact_norm_en1990_mut_actions::insert_permanent;
use semio_s_artifact_norm_en1990_mut_actions::insert_variable;
use semio_s_artifact_norm_en1990_mut_accidental::insert_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::insert_seismic;
use semio_s_artifact_norm_en1990_mut_members::insert_member;
use semio_s_artifact_norm_en1990_mut_members::insert_effect;
use semio_s_artifact_norm_en1990_mut_actions::remove_permanent;
use semio_s_artifact_norm_en1990_mut_actions::remove_variable;
use semio_s_artifact_norm_en1990_mut_accidental::remove_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::remove_seismic;
use semio_s_artifact_norm_en1990_mut_members::remove_member;
use semio_s_artifact_norm_en1990_mut_members::remove_effect;

/// 📥️ Decodes one committed mutation JSON document into [`En1990Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1990_mutation_json(text: &str) -> Result<En1990Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
