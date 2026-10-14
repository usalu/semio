//! 🧬️ En1994 snapshot schema — complete composite structure subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub structure_kind: String,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub steel_f_y_pa: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub beams: Vec<CompositeBeam>,
    #[dsl(table)]
    #[state(artifact)]
    pub columns: Vec<CompositeColumn>,
    #[dsl(table)]
    #[state(artifact)]
    pub slabs: Vec<CompositeSlab>,
    #[state(artifact)]
    pub fire_rating: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub insulation_thickness_m: f64,
    #[state(artifact)]
    pub fatigue_detail: String,
}

impl Default for En1994Snapshot {
    fn default() -> Self {
        let beam = CompositeBeam::default_placeholder();
        let mut beam = beam;
        beam.id = "beam-B1".into();
        let mut col = CompositeColumn::default_placeholder();
        col.id = "col-C1".into();
        let mut slab = CompositeSlab::default_placeholder();
        slab.id = "slab-S1".into();
        Self {
            annex: AnnexChoice::De,
            structure_kind: "building".into(),
            steel_f_y_pa: 355e6,
            beams: vec![beam],
            columns: vec![col],
            slabs: vec![slab],
            fire_rating: "r60".into(),
            insulation_thickness_m: 0.025,
            fatigue_detail: "stud_welded".into(),
        }
    }
}

impl En1994Snapshot {
    /// 🏗️ Unpropped construction-stage beam (LTB length governs).
    pub fn unpropped_building() -> Self {
        let mut s = Self::default();
        s.beams[0].construction = "unpropped".into();
        s.beams[0].ltb_length_m = 5.0;
        s.beams[0].support = "simply_supported".into();
        s
    }

    /// 🪛 Custom non-catalogue steel plate girder (tw/tf/geometry leaves govern).
    pub fn custom_plate_building() -> Self {
        let mut s = Self::default();
        s.beams[0].steel = crate::SteelSection::custom_plate();
        s
    }

    /// 🌉️ Bridge girder with FLM3 fatigue actions (EN 1994-2).
    pub fn bridge_girder() -> Self {
        let mut s = Self::default();
        s.structure_kind = "bridge".into();
        s.annex = AnnexChoice::De;
        s.fatigue_detail = "stud_welded".into();
        s.beams[0].n_cycles = 2.0e6;
        s.beams[0].span_m = 25.0;
        let mut fat = crate::CharacteristicAction::fatigue_flm3("FLM3");
        fat.delta_sigma_k_pa = 65e6;
        fat.delta_tau_k_pa = 40e6;
        s.beams[0].actions.push(fat);
        s
    }

    /// 🔥️ Elevated fire rating demanding thicker insulation.
    pub fn fire_demanding() -> Self {
        let mut s = Self::default();
        s.fire_rating = "r120".into();
        s.insulation_thickness_m = 0.010;
        s
    }
}

//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface;

/// 🧱 Rolled or welded steel section geometry for a composite member (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelSection {
    pub designation: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub width_m: f64,
    #[dsl(unit = "m")]
    pub tw_m: f64,
    #[dsl(unit = "m")]
    pub tf_m: f64,
    #[dsl(unit = "m2")]
    pub a_m2: f64,
    #[dsl(unit = "m3")]
    pub w_pl_y_m3: f64,
    #[dsl(unit = "m4")]
    pub i_y_m4: f64,
    #[dsl(unit = "m2")]
    pub a_v_m2: f64,
}

/// 🧱 Profiled steel sheeting under a composite slab (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ProfiledSheeting {
    pub profile: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub rib_width_m: f64,
    #[dsl(unit = "m")]
    pub thickness_m: f64,
    pub ribs_parallel_to_beam: bool,
}

/// 🧱 Headed stud shear connectors on a composite beam (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HeadedStuds {
    #[dsl(unit = "m")]
    pub diameter_m: f64,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "Pa")]
    pub f_u_pa: f64,
    pub count_per_rib: u32,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    pub total_count: u32,
}

/// 📋 Characteristic action / load case under EN 1990 / EN 1991 (SI).
///
/// Area intensity `q_area_pa` [Pa] is the single source of truth for distributed loads
/// (line load = q_area × tributary width). When `q_area_pa ≈ 0`, optional point force `f_k_n`
/// supplies the sole characteristic concentrated load (EN 1990 combination of effects).
/// Fatigue uses Δσ_k / Δτ_k or FLM3 when `kind=fatigue`. Column N/M use [`ColumnAction`].
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicAction {
    pub id: String,
    /// permanent | imposed | snow | wind | construction | fatigue
    pub kind: String,
    /// self_steel | wet_concrete | finishes | A..H | flm3 | …
    pub category: String,
    /// construction | composite
    pub stage: String,
    /// Characteristic area load q_k [Pa = N/m²] (× member tributary width → line).
    #[dsl(unit = "Pa")]
    pub q_area_pa: f64,
    /// Sole characteristic concentrated force [N] when `q_area_pa ≈ 0` (never both).
    #[dsl(unit = "N")]
    pub f_k_n: f64,
    #[dsl(unit = "Pa")]
    pub delta_sigma_k_pa: f64,
    #[dsl(unit = "Pa")]
    pub delta_tau_k_pa: f64,
}

/// 🏛️ Column characteristic N/M from structural analysis (EN 1990 action effects, SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColumnAction {
    pub id: String,
    /// permanent | imposed | snow | wind | construction
    pub kind: String,
    pub category: String,
    /// construction | composite
    pub stage: String,
    #[dsl(unit = "N")]
    pub n_k_n: f64,
    /// Characteristic bending moment [N·m].
    pub m_k_nm: f64,
}

/// 🧱 Composite beam subject under EN 1994-1-1 / EN 1994-2 (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeBeam {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    /// simply_supported | continuous_2_span
    pub support: String,
    pub construction: String,
    pub steel: SteelSection,
    #[dsl(unit = "m")]
    pub slab_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub concrete_f_ck_pa: f64,
    #[dsl(unit = "Pa")]
    pub concrete_e_cm_pa: f64,
    pub sheeting: ProfiledSheeting,
    pub studs: HeadedStuds,
    pub transverse_as_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub ltb_length_m: f64,
    pub as_hogging_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub bar_spacing_m: f64,
    #[dsl(unit = "m")]
    pub wk_limit_m: f64,
    pub n_cycles: f64,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

/// 🧱 Composite column subject under EN 1994-1-1 §6.7 (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeColumn {
    pub id: String,
    /// encased | concrete_filled | partially_encased
    pub kind: String,
    #[dsl(unit = "m")]
    pub length_m: f64,
    /// Outer diameter (CHS) or side length (SHS) for local buckling / confinement [m].
    #[dsl(unit = "m")]
    pub outer_size_m: f64,
    #[dsl(unit = "m")]
    pub wall_thickness_m: f64,
    #[dsl(unit = "m2")]
    pub steel_a_m2: f64,
    #[dsl(unit = "Pa")]
    pub steel_f_y_pa: f64,
    #[dsl(unit = "m2")]
    pub concrete_a_m2: f64,
    #[dsl(unit = "Pa")]
    pub concrete_f_ck_pa: f64,
    #[dsl(unit = "m2")]
    pub reinforcement_as_m2: f64,
    #[dsl(unit = "Pa")]
    pub reinforcement_f_yk_pa: f64,
    #[dsl(unit = "m4")]
    pub i_m4: f64,
    pub buckling_curve: String,
    #[dsl(table)]
    pub actions: Vec<ColumnAction>,
}

/// 🧱 Composite slab with profiled sheeting under EN 1994-1-1 §9 (SI).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeSlab {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    /// simply_supported | continuous_2_span
    pub support: String,
    pub sheeting: ProfiledSheeting,
    #[dsl(unit = "m")]
    pub concrete_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub f_ck_pa: f64,
    pub m_factor: f64,
    pub k_factor: f64,
    pub as_m2_per_m: f64,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

impl SteelSection {
    /// 🧱 HEB 300 placeholder for insert mutations.
    pub fn heb300() -> Self {
        Self { designation: "HEB300".into(), height_m: 0.300, width_m: 0.300, tw_m: 0.011, tf_m: 0.019, a_m2: 0.0149, w_pl_y_m3: 1.867e-3, i_y_m4: 2.517e-4, a_v_m2: 0.0045 }
    }

    /// 📚 Normalizes rolled-section designations (`HEB 320` → `HEB320`).
    pub fn normalize_designation(raw: &str) -> String {
        raw.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase()
    }

    /// 📚 Catalogue geometry for common HEB sections (EN 10365 / ArcelorMittal tables, SI).
    pub fn from_catalogue(designation: &str) -> Option<Self> {
        match Self::normalize_designation(designation).as_str() {
            "HEB300" => Some(Self::heb300()),
            "HEB320" => Some(Self {
                designation: "HEB320".into(), height_m: 0.320, width_m: 0.300, tw_m: 0.0115, tf_m: 0.0205,
                a_m2: 0.0161, w_pl_y_m3: 2.129e-3, i_y_m4: 3.082e-4, a_v_m2: 0.0049,
            }),
            "HEB340" => Some(Self {
                designation: "HEB340".into(), height_m: 0.340, width_m: 0.300, tw_m: 0.0120, tf_m: 0.0215,
                a_m2: 0.0171, w_pl_y_m3: 2.402e-3, i_y_m4: 3.666e-4, a_v_m2: 0.0052,
            }),
            "HEB360" => Some(Self {
                designation: "HEB360".into(), height_m: 0.360, width_m: 0.300, tw_m: 0.0125, tf_m: 0.0225,
                a_m2: 0.0180, w_pl_y_m3: 2.682e-3, i_y_m4: 4.319e-4, a_v_m2: 0.0055,
            }),
            "HEB400" => Some(Self {
                designation: "HEB400".into(), height_m: 0.400, width_m: 0.300, tw_m: 0.0135, tf_m: 0.0240,
                a_m2: 0.0198, w_pl_y_m3: 3.231e-3, i_y_m4: 5.768e-4, a_v_m2: 0.0061,
            }),
            "HEB450" => Some(Self {
                designation: "HEB450".into(), height_m: 0.450, width_m: 0.300, tw_m: 0.0140, tf_m: 0.0260,
                a_m2: 0.0218, w_pl_y_m3: 3.898e-3, i_y_m4: 7.986e-4, a_v_m2: 0.0068,
            }),
            "HEB500" => Some(Self {
                designation: "HEB500".into(), height_m: 0.500, width_m: 0.300, tw_m: 0.0145, tf_m: 0.0280,
                a_m2: 0.0239, w_pl_y_m3: 4.715e-3, i_y_m4: 1.072e-3, a_v_m2: 0.0075,
            }),
            "HEB550" => Some(Self {
                designation: "HEB550".into(), height_m: 0.550, width_m: 0.300, tw_m: 0.0150, tf_m: 0.0290,
                a_m2: 0.0254, w_pl_y_m3: 5.453e-3, i_y_m4: 1.366e-3, a_v_m2: 0.0081,
            }),
            "HEB600" => Some(Self {
                designation: "HEB600".into(), height_m: 0.600, width_m: 0.300, tw_m: 0.0155, tf_m: 0.0300,
                a_m2: 0.0270, w_pl_y_m3: 6.285e-3, i_y_m4: 1.710e-3, a_v_m2: 0.0087,
            }),
            "HEB650" => Some(Self {
                designation: "HEB650".into(), height_m: 0.650, width_m: 0.300, tw_m: 0.0160, tf_m: 0.0310,
                a_m2: 0.0286, w_pl_y_m3: 7.199e-3, i_y_m4: 2.112e-3, a_v_m2: 0.0094,
            }),
            "HEB700" => Some(Self {
                designation: "HEB700".into(), height_m: 0.700, width_m: 0.300, tw_m: 0.0170, tf_m: 0.0320,
                a_m2: 0.0306, w_pl_y_m3: 8.217e-3, i_y_m4: 2.569e-3, a_v_m2: 0.0102,
            }),
            _ => None,
        }
    }

    /// 📚 Resolves catalogue properties when the designation is known; otherwise keeps stored geometry.
    pub fn resolve(&self) -> Self {
        Self::from_catalogue(&self.designation).unwrap_or_else(|| self.clone())
    }

    /// 🪛 Non-catalogue welded plate girder (geometry leaves drive checks).
    pub fn custom_plate() -> Self {
        Self {
            designation: "CUSTOM-PLATE".into(),
            height_m: 0.450, width_m: 0.220, tw_m: 0.010, tf_m: 0.018,
            a_m2: 0.0124, w_pl_y_m3: 1.95e-3, i_y_m4: 3.8e-4, a_v_m2: 0.0040,
        }
    }

    /// 📚 Heavier HEB options for OneOf remedies (wire values = designation leaf).
    pub fn heavier_heb_options(current: &str) -> Vec<String> {
        let order = ["HEB300", "HEB320", "HEB340", "HEB360", "HEB400", "HEB450", "HEB500", "HEB550", "HEB600", "HEB650", "HEB700"];
        let cur = Self::normalize_designation(current);
        let idx = order.iter().position(|d| *d == cur).unwrap_or(0);
        order[idx.saturating_add(1)..].iter().map(|s| (*s).to_string()).collect()
    }

    /// 📐️ Web clear height h_w [m].
    pub fn h_w_m(&self) -> f64 {
        (self.height_m - 2.0 * self.tf_m).max(0.0)
    }

    /// 📐️ Shear area from plate geometry (EN 1993-1-1 §6.2.6 for rolled I).
    pub fn shear_area_from_plates_m2(&self) -> f64 {
        (self.height_m * self.tw_m).max(self.a_v_m2)
    }
}

impl ProfiledSheeting {
    pub fn trapezoidal() -> Self {
        Self { profile: "trapezoidal".into(), height_m: 0.055, rib_width_m: 0.035, thickness_m: 0.0009, ribs_parallel_to_beam: false }
    }

    /// 📐️ Steel area of sheeting per metre width A_p [m²/m] from thickness and developed profile factor ~1.2.
    pub fn a_p_m2_per_m(&self) -> f64 {
        self.thickness_m * 1.2
    }
}

impl HeadedStuds {
    pub fn typical() -> Self {
        Self { diameter_m: 0.019, height_m: 0.075, f_u_pa: 450e6, count_per_rib: 1, spacing_m: 0.150, total_count: 72 }
    }
}

impl CharacteristicAction {
    /// 📐️ Permanent area load (line = q_area × tributary width in `action_internals`).
    pub fn permanent_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: "permanent".into(), category: category.into(), stage: stage.into(),
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    pub fn imposed_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: "imposed".into(), category: category.into(), stage: stage.into(),
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    /// 📍 Sole characteristic concentrated force when area load is absent.
    pub fn point_force(id: &str, stage: &str, kind: &str, category: &str, f_k_n: f64) -> Self {
        Self {
            id: id.into(), kind: kind.into(), category: category.into(), stage: stage.into(),
            q_area_pa: 0.0, f_k_n, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    pub fn fatigue_flm3(id: &str) -> Self {
        Self {
            id: id.into(), kind: "fatigue".into(), category: "flm3".into(), stage: "composite".into(),
            q_area_pa: 0.0, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
}

impl ColumnAction {
    /// 🏛️ Column characteristic N_k / M_k from structural analysis.
    pub fn forces(id: &str, stage: &str, kind: &str, category: &str, n_k_n: f64, m_k_nm: f64) -> Self {
        Self {
            id: id.into(), kind: kind.into(), category: category.into(), stage: stage.into(),
            n_k_n, m_k_nm,
        }
    }
}

impl CompositeBeam {
    pub fn default_placeholder() -> Self {
        let steel = SteelSection::heb300();
        let sheeting = ProfiledSheeting::trapezoidal();
        let spacing = 3.0;
        let slab_t = 0.140;
        let actions = default_beam_actions(steel.a_m2, spacing, slab_t, sheeting.height_m);
        Self {
            id: "beam-new".into(), span_m: 8.0, spacing_m: spacing, support: "simply_supported".into(),
            construction: "propped".into(),
            steel, slab_thickness_m: slab_t, concrete_f_ck_pa: 30e6, concrete_e_cm_pa: 33e9,
            sheeting, studs: HeadedStuds::typical(),
            transverse_as_m2_per_m: 2.5e-3, ltb_length_m: 4.0,
            as_hogging_m2_per_m: 5.0e-4, bar_spacing_m: 0.150, wk_limit_m: 0.0003,
            n_cycles: 2.0e6, actions,
        }
    }
}

impl CompositeColumn {
    pub fn default_placeholder() -> Self {
        Self {
            id: "col-new".into(), kind: "concrete_filled".into(), length_m: 3.5,
            outer_size_m: 0.300, wall_thickness_m: 0.010,
            steel_a_m2: 0.0084, steel_f_y_pa: 355e6, concrete_a_m2: 0.072, concrete_f_ck_pa: 30e6,
            reinforcement_as_m2: 0.0012, reinforcement_f_yk_pa: 500e6, i_m4: 2.1e-4, buckling_curve: "a".into(),
            actions: vec![
                ColumnAction::forces("G-col", "composite", "permanent", "self", 1200e3, 20e3),
                ColumnAction::forces("Q-col", "composite", "imposed", "B", 400e3, 15e3),
            ],
        }
    }
}

impl CompositeSlab {
    pub fn default_placeholder() -> Self {
        Self {
            id: "slab-new".into(), span_m: 3.0, support: "simply_supported".into(),
            sheeting: ProfiledSheeting::trapezoidal(),
            concrete_thickness_m: 0.140, f_ck_pa: 30e6, m_factor: 180.0, k_factor: 0.05, as_m2_per_m: 2.5e-4,
            actions: vec![
                CharacteristicAction::permanent_area("G-slab", "composite", "self", 3.5e3),
                CharacteristicAction::imposed_area("Q-slab", "composite", "B", 2.0e3),
            ],
        }
    }
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1994", "EN 1994", EN1994_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1994_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1994", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1994_DOCUMENT_SCHEMA: &str = "semio.norm.en1994/v1";
