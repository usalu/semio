//! 🧬️ En1994 snapshot schema — complete composite structure subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};
pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 🏗️ Kind of structure: a building or a bridge (selects the EN 1994-1-1 or EN 1994-2 rules).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum StructureKind {
    Building,
    Bridge,
}

/// ♻ Steel fatigue detail category (EN 1993-1-9 / EN 1994-2 6.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FatigueDetail {
    StudWelded,
    ShearConnector,
    Reinforcement,
    FlangeButtWeld,
}

/// 🔥 Standard fire resistance rating R (EN 1994-1-2); `Unrated` disables the fire checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FireRating {
    Unrated,
    R30,
    R60,
    R90,
    R120,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub structure_kind: StructureKind,
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
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[state(artifact)]
    pub fire_rating: FireRating,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub insulation_thickness_m: f64,
    #[state(artifact)]
    pub fatigue_detail: FatigueDetail,
    #[state(artifact)]
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    #[dsl(table)]
    #[state(artifact)]
    pub studs: Vec<Stud>,
    #[dsl(table)]
    #[state(artifact)]
    pub cases: Vec<CaseRow>,
}

impl En1994Snapshot {
    /// 🌱 The showcase building document of the given annex; the annex is a normative choice and never defaulted.
    pub fn example(annex: AnnexChoice) -> Self {
        let beam = CompositeBeam::default_example();
        let mut beam = beam;
        beam.id = "beam-B1".into();
        let mut col = CompositeColumn::default_example();
        col.id = "col-C1".into();
        let mut slab = CompositeSlab::default_example();
        slab.id = "slab-S1".into();
        Self {
            annex,
            reliability_class: ReliabilityClass::RC2,
            structure_kind: StructureKind::Building,
            steel_f_y_pa: 355e6,
            beams: vec![beam],
            columns: vec![col],
            slabs: vec![slab],
            fire_rating: FireRating::R60,
            insulation_thickness_m: 0.025,
            fatigue_detail: FatigueDetail::StudWelded,
            placements: vec![],
            studs: Stud::typical_row("beam-B1", 4.0, 0.15, 52),
            cases: vec![],
        }
    }
}

impl En1994Snapshot {
    /// 🎪️ The full showcase: one document that populates every entity collection and runs every check family of the spec trace, decoded from the committed fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (generated by the ticket script `🧰️scripts/showcase`).
    pub fn full_showcase() -> Self {
        semio_framework_pack_json::from_json_str(include_str!("../../🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed showcase fixture decodes")
    }

    /// 🏗️ Unpropped construction-stage beam (LTB length governs).
    pub fn unpropped_building() -> Self {
        let mut s = Self::example(AnnexChoice::De);
        s.beams[0].construction = ConstructionMethod::Unpropped;
        s.beams[0].ltb_length_m = 5.0;
        s.beams[0].support = SupportCondition::SimplySupported;
        s
    }

    /// 🪛 Custom non-catalogue steel plate girder (tw/tf/geometry leaves govern).
    pub fn custom_plate_building() -> Self {
        let mut s = Self::example(AnnexChoice::De);
        s.beams[0].steel = crate::SteelSection::custom_plate();
        s
    }

    /// 🌉️ Bridge girder with FLM3 fatigue actions (EN 1994-2).
    pub fn bridge_girder() -> Self {
        let mut s = Self::example(AnnexChoice::De);
        s.structure_kind = StructureKind::Bridge;
        s.annex = AnnexChoice::De;
        s.fatigue_detail = FatigueDetail::StudWelded;
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
        let mut s = Self::example(AnnexChoice::De);
        s.fire_rating = FireRating::R120;
        s.insulation_thickness_m = 0.010;
        s
    }
}

//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1994_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1994_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1994_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1994_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// 🧱 Rolled or welded steel section geometry for a composite member (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
    #[dsl(unit = "m")]
    pub r_m: f64,
}

/// 〰 Profile of the steel sheeting of a composite slab (EN 1994-1-1 9.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SheetingProfile {
    Trapezoidal,
    ReEntrant,
}

/// 🧱 Profiled steel sheeting under a composite slab (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ProfiledSheeting {
    pub profile: SheetingProfile,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub rib_width_m: f64,
    #[dsl(unit = "m")]
    pub thickness_m: f64,
    pub ribs_parallel_to_beam: bool,
}

/// 🏷️ Construction stage in which an action acts: before composite action, with composite action, or at every stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ActionStage {
    Construction,
    Composite,
    Any,
}

/// 🏋 Kind of an action (EN 1990 Table A1.1): selects the ψ factors and the load case colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ActionKind {
    Permanent,
    Imposed,
    Snow,
    Wind,
    Construction,
    Fatigue,
}

/// 📋 Characteristic action / load case of a composite beam under EN 1990 / EN 1991 (SI); slab strips use [`SlabAction`].
///
/// Area intensity `q_area_pa` [Pa] is the single source of truth for distributed loads
/// (line load = q_area × tributary width). When `q_area_pa ≈ 0`, optional point force `f_k_n`
/// supplies the sole characteristic concentrated load (EN 1990 combination of effects).
/// Fatigue uses Δσ_k / Δτ_k or FLM3 when `kind=fatigue`. Column N/M use [`ColumnAction`].
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicAction {
    pub id: String,
    /// 📌️ permanent | imposed | snow | wind | construction | fatigue
    pub kind: ActionKind,
    /// 🚦️ self_steel | wet_concrete | finishes | A..H | flm3 | …
    pub category: String,
    /// 🪜️ construction | composite
    pub stage: ActionStage,
    /// 🗺️ Characteristic area load q_k [Pa = N/m²] (× member tributary width → line).
    #[dsl(unit = "Pa")]
    pub q_area_pa: f64,
    /// ☝️ Sole characteristic concentrated force [N] when `q_area_pa ≈ 0` (never both).
    #[dsl(unit = "N")]
    pub f_k_n: f64,
    #[dsl(unit = "Pa")]
    pub delta_sigma_k_pa: f64,
    #[dsl(unit = "Pa")]
    pub delta_tau_k_pa: f64,
    /// 📍️ Position of a concentrated force along the member span [m] from its start (0 = mid-span).
    #[dsl(unit = "m")]
    pub x_m: f64,
}

/// 🧱️ Characteristic action of one composite slab strip (EN 1990 combination of effects): the fields of [`CharacteristicAction`] plus the position `y_m` of a point force across the strip width.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SlabAction {
    pub id: String,
    /// 📌️ permanent | imposed | snow | wind | construction | fatigue
    pub kind: ActionKind,
    /// 🚦️ self_steel | wet_concrete | finishes | A..H | flm3 | …
    pub category: String,
    /// 🪜️ construction | composite
    pub stage: ActionStage,
    /// 🗺️ Characteristic area load q_k [Pa = N/m²] (× member tributary width → line).
    #[dsl(unit = "Pa")]
    pub q_area_pa: f64,
    /// ☝️ Sole characteristic concentrated force [N] when `q_area_pa ≈ 0` (never both).
    #[dsl(unit = "N")]
    pub f_k_n: f64,
    #[dsl(unit = "Pa")]
    pub delta_sigma_k_pa: f64,
    #[dsl(unit = "Pa")]
    pub delta_tau_k_pa: f64,
    /// 📍️ Position of a concentrated force along the member span [m] from its start (0 = mid-span).
    #[dsl(unit = "m")]
    pub x_m: f64,
    /// 📍️ Position of a slab point force across the strip width [m] (0 = strip axis).
    #[dsl(unit = "m")]
    pub y_m: f64,
}

/// 🏛️ Column characteristic N/M from structural analysis (EN 1990 action effects, SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColumnAction {
    pub id: String,
    /// 📜️ permanent | imposed | snow | wind | construction
    pub kind: ActionKind,
    pub category: String,
    /// 👣️ construction | composite
    pub stage: ActionStage,
    #[dsl(unit = "N")]
    pub n_k_n: f64,
    /// 🪢️ Characteristic bending moment [N·m].
    pub m_k_nm: f64,
}

/// 🪜 Construction method of a composite beam: with or without temporary props during casting (EN 1994-1-1 9.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ConstructionMethod {
    Propped,
    Unpropped,
}

/// 🪝 Support condition of the designed span of a composite beam or slab (EN 1994-1-1 5.4.1.2 Fig. 5.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SupportCondition {
    SimplySupported,
    Continuous2Span,
    EndSpan,
    InteriorSpan,
    Cantilever,
}

/// 🧵 Production route of the reinforcing steel, selecting the fire strength reduction (EN 1992-1-2 Table 3.2a).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ReinforcementProduct {
    HotRolled,
    ColdWorked,
}

/// 🧱 Composite beam subject under EN 1994-1-1 / EN 1994-2 (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeBeam {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    /// 🪝️ simply_supported | continuous_2_span
    pub support: SupportCondition,
    pub construction: ConstructionMethod,
    pub steel: SteelSection,
    #[dsl(unit = "m")]
    pub slab_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub concrete_f_ck_pa: f64,
    #[dsl(unit = "Pa")]
    pub concrete_e_cm_pa: f64,
    pub sheeting: ProfiledSheeting,
    pub transverse_as_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub ltb_length_m: f64,
    pub as_hogging_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub bar_spacing_m: f64,
    #[dsl(unit = "m")]
    pub wk_limit_m: f64,
    pub n_cycles: f64,
    #[dsl(unit = "m")]
    pub adjacent_span_m: f64,
    #[dsl(unit = "m")]
    pub patch_bearing_length_m: f64,
    pub bridge_lanes: u32,
    pub cracked_length_fraction: f64,
    #[dsl(unit = "m")]
    pub stud_transverse_spacing_m: f64,
    #[dsl(unit = "m")]
    pub stud_edge_distance_m: f64,
    #[dsl(unit = "m2/m")]
    pub transverse_as_top_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub end_plate_width_m: f64,
    #[dsl(unit = "m")]
    pub end_plate_height_m: f64,
    #[dsl(unit = "m")]
    pub end_zone_length_m: f64,
    /// 🕸️ hot_rolled | cold_worked hogging reinforcement (EN 1992-1-2 Table 3.2a strength reduction in fire).
    pub reinforcement_product: ReinforcementProduct,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

/// 📉 European buckling curve a, b, c or d of a composite column (EN 1994-1-1 6.7.3.5, EN 1993-1-1 Table 6.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum BucklingCurve {
    A,
    B,
    C,
    D,
}

/// 🏛 Cross-section type of a composite column (EN 1994-1-1 6.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ColumnKind {
    Encased,
    ConcreteFilled,
    PartiallyEncased,
}

/// 🧱 Composite column subject under EN 1994-1-1 §6.7 (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeColumn {
    pub id: String,
    /// 🔩️ encased | concrete_filled | partially_encased
    pub kind: ColumnKind,
    #[dsl(unit = "m")]
    pub length_m: f64,
    /// 📤️ Outer diameter (CHS) or side length (SHS) for local buckling / confinement [m].
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
    pub buckling_curve: BucklingCurve,
    #[dsl(unit = "N")]
    pub introduced_force_n: f64,
    #[dsl(unit = "m")]
    pub section_height_m: f64,
    #[dsl(unit = "m")]
    pub section_width_m: f64,
    #[dsl(unit = "m")]
    pub section_tw_m: f64,
    #[dsl(unit = "m")]
    pub section_tf_m: f64,
    #[dsl(unit = "m")]
    pub rebar_cover_m: f64,
    /// 🕸️ hot_rolled | cold_worked reinforcing steel (EN 1992-1-2 Table 3.2a strength reduction in fire).
    pub reinforcement_product: ReinforcementProduct,
    #[dsl(table)]
    pub actions: Vec<ColumnAction>,
}

/// 🧱 Composite slab with profiled sheeting under EN 1994-1-1 §9 (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeSlab {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    /// 🤝️ simply_supported | continuous_2_span
    pub support: SupportCondition,
    pub sheeting: ProfiledSheeting,
    #[dsl(unit = "m")]
    pub concrete_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub f_ck_pa: f64,
    pub m_factor: f64,
    pub k_factor: f64,
    pub as_m2_per_m: f64,
    #[dsl(table)]
    pub actions: Vec<SlabAction>,
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

/// 🔩 Headed shear stud of a composite beam (EN 1994-1-1 6.6): one row per stud with its position along the beam and its dimensions.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Stud {
    pub id: String,
    /// 🔗 Id of the composite beam the stud belongs to.
    pub beam_id: String,
    /// ↔️ Position x [m] of the stud along the beam axis, measured from the start of the beam (the first support of its span layout).
    pub x_m: f64,
    /// ⌀ Shank diameter d [m] (EN 1994-1-1 6.6.3.1).
    pub diameter_m: f64,
    /// ↕️ Overall height h_sc [m] after welding.
    pub height_m: f64,
    /// 💪 Specified ultimate tensile strength f_u [Pa], at most 500 MPa.
    pub f_u_pa: f64,
}

impl SteelSection {
    /// 🧱 HEB 300 default example for insert mutations.
    pub fn heb300() -> Self {
        Self { designation: "HEB300".into(), height_m: 0.300, width_m: 0.300, tw_m: 0.011, tf_m: 0.019, a_m2: 0.0149, w_pl_y_m3: 1.867e-3, i_y_m4: 2.517e-4, a_v_m2: 0.0045, r_m: 0.027 }
    }

    /// 📚 Normalizes rolled-section designations (`HEB 320` → `HEB320`).
    pub fn normalize_designation(raw: &str) -> String {
        raw.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase()
    }

    /// 📚 Catalogue geometry for common HEB sections (EN 10365 / ArcelorMittal tables, SI). The fillet radius `r_m` of every row (27 mm for HEB 300 to 700) is recalled from EN 10365 and unverified against the print.
    pub fn from_catalogue(designation: &str) -> Option<Self> {
        match Self::normalize_designation(designation).as_str() {
            "HEB300" => Some(Self::heb300()),
            "HEB320" => Some(Self {
                designation: "HEB320".into(), height_m: 0.320, width_m: 0.300, tw_m: 0.0115, tf_m: 0.0205,
                a_m2: 0.0161, w_pl_y_m3: 2.129e-3, i_y_m4: 3.082e-4, a_v_m2: 0.0049,
                r_m: 0.027,
            }),
            "HEB340" => Some(Self {
                designation: "HEB340".into(), height_m: 0.340, width_m: 0.300, tw_m: 0.0120, tf_m: 0.0215,
                a_m2: 0.0171, w_pl_y_m3: 2.402e-3, i_y_m4: 3.666e-4, a_v_m2: 0.0052,
                r_m: 0.027,
            }),
            "HEB360" => Some(Self {
                designation: "HEB360".into(), height_m: 0.360, width_m: 0.300, tw_m: 0.0125, tf_m: 0.0225,
                a_m2: 0.0180, w_pl_y_m3: 2.682e-3, i_y_m4: 4.319e-4, a_v_m2: 0.0055,
                r_m: 0.027,
            }),
            "HEB400" => Some(Self {
                designation: "HEB400".into(), height_m: 0.400, width_m: 0.300, tw_m: 0.0135, tf_m: 0.0240,
                a_m2: 0.0198, w_pl_y_m3: 3.231e-3, i_y_m4: 5.768e-4, a_v_m2: 0.0061,
                r_m: 0.027,
            }),
            "HEB450" => Some(Self {
                designation: "HEB450".into(), height_m: 0.450, width_m: 0.300, tw_m: 0.0140, tf_m: 0.0260,
                a_m2: 0.0218, w_pl_y_m3: 3.898e-3, i_y_m4: 7.986e-4, a_v_m2: 0.0068,
                r_m: 0.027,
            }),
            "HEB500" => Some(Self {
                designation: "HEB500".into(), height_m: 0.500, width_m: 0.300, tw_m: 0.0145, tf_m: 0.0280,
                a_m2: 0.0239, w_pl_y_m3: 4.715e-3, i_y_m4: 1.072e-3, a_v_m2: 0.0075,
                r_m: 0.027,
            }),
            "HEB550" => Some(Self {
                designation: "HEB550".into(), height_m: 0.550, width_m: 0.300, tw_m: 0.0150, tf_m: 0.0290,
                a_m2: 0.0254, w_pl_y_m3: 5.453e-3, i_y_m4: 1.366e-3, a_v_m2: 0.0081,
                r_m: 0.027,
            }),
            "HEB600" => Some(Self {
                designation: "HEB600".into(), height_m: 0.600, width_m: 0.300, tw_m: 0.0155, tf_m: 0.0300,
                a_m2: 0.0270, w_pl_y_m3: 6.285e-3, i_y_m4: 1.710e-3, a_v_m2: 0.0087,
                r_m: 0.027,
            }),
            "HEB650" => Some(Self {
                designation: "HEB650".into(), height_m: 0.650, width_m: 0.300, tw_m: 0.0160, tf_m: 0.0310,
                a_m2: 0.0286, w_pl_y_m3: 7.199e-3, i_y_m4: 2.112e-3, a_v_m2: 0.0094,
                r_m: 0.027,
            }),
            "HEB700" => Some(Self {
                designation: "HEB700".into(), height_m: 0.700, width_m: 0.300, tw_m: 0.0170, tf_m: 0.0320,
                a_m2: 0.0306, w_pl_y_m3: 8.217e-3, i_y_m4: 2.569e-3, a_v_m2: 0.0102,
                r_m: 0.027,
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
            a_m2: 0.0124, w_pl_y_m3: 1.95e-3, i_y_m4: 3.8e-4, a_v_m2: 0.0040, r_m: 0.0,
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

    /// 🔘️ Root radius r [m]: the stored `r_m` leaf, otherwise implied by the area (the fillets add (4 − π) r² to the three plates, 0 for a welded plate section).
    pub fn root_radius_m(&self) -> f64 {
        if self.r_m > 0.0 {
            return self.r_m;
        }
        let fillets = self.a_m2 - 2.0 * self.width_m * self.tf_m - self.h_w_m() * self.tw_m;
        let limit = (0.5 * (self.width_m - self.tw_m)).min(0.5 * self.h_w_m()).max(0.0);
        if fillets <= 0.0 { 0.0 } else { (fillets / (4.0 - std::f64::consts::PI)).sqrt().min(limit) }
    }

    /// 📐️ Shear area from plate geometry (EN 1993-1-1 §6.2.6 for rolled I).
    pub fn shear_area_from_plates_m2(&self) -> f64 {
        (self.height_m * self.tw_m).max(self.a_v_m2)
    }
}

impl ProfiledSheeting {
    pub fn trapezoidal() -> Self {
        Self { profile: SheetingProfile::Trapezoidal, height_m: 0.055, rib_width_m: 0.100, thickness_m: 0.0009, ribs_parallel_to_beam: false }
    }

    /// 📐️ Steel area of sheeting per metre width A_p [m²/m]: thickness times the developed length per metre (1.2 m/m, the same constant as the checks' `SHEETING_DEVELOPED_LENGTH_FACTOR`).
    pub fn a_p_m2_per_m(&self) -> f64 {
        self.thickness_m * 1.2
    }
}

impl Stud {
    /// 🔩 A 19 × 95 mm stud of 450 MPa at position `x_m` of beam `beam_id` (the example layout and the stud an insert starts from; a visible starting point the dev edits, not a normative value).
    pub fn typical(id: &str, beam_id: &str, x_m: f64) -> Self {
        Self { id: id.into(), beam_id: beam_id.into(), x_m, diameter_m: 0.019, height_m: 0.095, f_u_pa: 450e6 }
    }

    /// 🔩 `count` typical studs of beam `beam_id`, `pitch_m` apart and centred on `centre_m` (`<beam>-stud-<n>`).
    pub fn typical_row(beam_id: &str, centre_m: f64, pitch_m: f64, count: u32) -> Vec<Self> {
        (0..count).map(|n| Self::typical(&format!("{beam_id}-stud-{n}"), beam_id, centre_m + (f64::from(n) - (f64::from(count) - 1.0) / 2.0) * pitch_m)).collect()
    }
}

impl CharacteristicAction {
    /// 📐️ Permanent area load (line = q_area × tributary width in `action_internals`).
    pub fn permanent_area(id: &str, stage: ActionStage, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: ActionKind::Permanent, category: category.into(), stage,
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0,
        }
    }
    pub fn imposed_area(id: &str, stage: ActionStage, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: ActionKind::Imposed, category: category.into(), stage,
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0,
        }
    }
    /// 📍 Sole characteristic concentrated force when area load is absent.
    pub fn point_force(id: &str, stage: ActionStage, kind: ActionKind, category: &str, f_k_n: f64) -> Self {
        Self {
            id: id.into(), kind, category: category.into(), stage,
            q_area_pa: 0.0, f_k_n, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0,
        }
    }
    pub fn fatigue_flm3(id: &str) -> Self {
        Self {
            id: id.into(), kind: ActionKind::Fatigue, category: "flm3".into(), stage: ActionStage::Composite,
            q_area_pa: 0.0, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0,
        }
    }
}

impl SlabAction {
    /// 📐️ Permanent area load of the strip.
    pub fn permanent_area(id: &str, stage: ActionStage, category: &str, q_area_pa: f64) -> Self {
        Self::from(&CharacteristicAction::permanent_area(id, stage, category, q_area_pa))
    }
    pub fn imposed_area(id: &str, stage: ActionStage, category: &str, q_area_pa: f64) -> Self {
        Self::from(&CharacteristicAction::imposed_area(id, stage, category, q_area_pa))
    }
}

impl From<&CharacteristicAction> for SlabAction {
    /// 🔁️ The strip action with the same loads and the point force on the strip axis.
    fn from(a: &CharacteristicAction) -> Self {
        Self { id: a.id.clone(), kind: a.kind.clone(), category: a.category.clone(), stage: a.stage.clone(), q_area_pa: a.q_area_pa, f_k_n: a.f_k_n, delta_sigma_k_pa: a.delta_sigma_k_pa, delta_tau_k_pa: a.delta_tau_k_pa, x_m: a.x_m, y_m: 0.0 }
    }
}

impl From<&SlabAction> for CharacteristicAction {
    /// 🔁️ The statics view of a strip action: the beam action row without the transverse position.
    fn from(a: &SlabAction) -> Self {
        Self { id: a.id.clone(), kind: a.kind.clone(), category: a.category.clone(), stage: a.stage.clone(), q_area_pa: a.q_area_pa, f_k_n: a.f_k_n, delta_sigma_k_pa: a.delta_sigma_k_pa, delta_tau_k_pa: a.delta_tau_k_pa, x_m: a.x_m }
    }
}

impl CompositeSlab {
    /// 🧮️ The actions of the strip as beam action rows for the shared EN 1990 statics.
    pub fn statics_actions(&self) -> Vec<CharacteristicAction> {
        self.actions.iter().map(CharacteristicAction::from).collect()
    }
}

impl ColumnAction {
    /// 🏛️ Column characteristic N_k / M_k from structural analysis.
    pub fn forces(id: &str, stage: ActionStage, kind: ActionKind, category: &str, n_k_n: f64, m_k_nm: f64) -> Self {
        Self {
            id: id.into(), kind, category: category.into(), stage,
            n_k_n, m_k_nm,
        }
    }
}

/// 📋 Default construction- and composite-stage actions of a floor beam.
fn default_beam_actions(steel_a_m2: f64, spacing_m: f64, slab_thickness_m: f64, sheeting_h: f64) -> Vec<CharacteristicAction> {
    let g_steel_line = steel_a_m2 * 7850.0 * 9.81;
    let h_conc = (slab_thickness_m - sheeting_h).max(0.04);
    let g_steel_area = g_steel_line / spacing_m.max(1e-6);
    let g_wet_area = 25e3 * h_conc;
    vec![
        CharacteristicAction::permanent_area("G-steel", ActionStage::Construction, "self_steel", g_steel_area),
        CharacteristicAction::permanent_area("G-wet", ActionStage::Construction, "wet_concrete", g_wet_area),
        CharacteristicAction {
            id: "Q-constr".into(), kind: ActionKind::Construction, category: "construction_load".into(), stage: ActionStage::Construction,
            q_area_pa: 1.0e3, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0, x_m: 0.0,
        },
        CharacteristicAction::permanent_area("G-steel-comp", ActionStage::Composite, "self_steel", g_steel_area),
        CharacteristicAction::permanent_area("G-slab", ActionStage::Composite, "slab", g_wet_area * 0.9),
        CharacteristicAction::permanent_area("G-finishes", ActionStage::Composite, "finishes", 1.0e3),
        CharacteristicAction::imposed_area("Q-office", ActionStage::Composite, "B", 2.0e3),
    ]
}

impl CompositeBeam {
    /// 🧭️ The span continues over a support: two-span, end or interior span of a continuous beam.
    pub fn is_continuous(&self) -> bool {
        matches!(self.support, SupportCondition::Continuous2Span | SupportCondition::EndSpan | SupportCondition::InteriorSpan)
    }

    /// 🧭️ A hogging moment arises over a support: every continuous span and the cantilever.
    pub fn has_hogging(&self) -> bool {
        self.is_continuous() || self.support == SupportCondition::Cantilever
    }

    /// 📏️ Length of the neighbouring span; `adjacent_span_m` of 0 means a span equal to this one.
    pub fn neighbour_span_m(&self) -> f64 {
        if self.adjacent_span_m > 1e-9 { self.adjacent_span_m } else { self.span_m }
    }

    pub fn default_example() -> Self {
        let steel = SteelSection::heb300();
        let sheeting = ProfiledSheeting::trapezoidal();
        let spacing = 3.0;
        let slab_t = 0.140;
        let actions = default_beam_actions(steel.a_m2, spacing, slab_t, sheeting.height_m);
        Self {
            id: "beam-new".into(), span_m: 8.0, spacing_m: spacing, support: SupportCondition::SimplySupported,
            construction: ConstructionMethod::Propped,
            steel, slab_thickness_m: slab_t, concrete_f_ck_pa: 30e6, concrete_e_cm_pa: 33e9,
            sheeting,
            transverse_as_m2_per_m: 2.5e-3, ltb_length_m: 4.0,
            as_hogging_m2_per_m: 5.0e-4, bar_spacing_m: 0.150, wk_limit_m: 0.0003,
            n_cycles: 2.0e6, adjacent_span_m: 0.0, patch_bearing_length_m: 0.1, bridge_lanes: 0, cracked_length_fraction: 0.15, stud_transverse_spacing_m: 0.0, stud_edge_distance_m: 0.0, transverse_as_top_m2_per_m: 0.0, end_plate_width_m: 0.0, end_plate_height_m: 0.0, end_zone_length_m: 0.0, reinforcement_product: ReinforcementProduct::HotRolled, actions,
        }
    }
}

impl CompositeColumn {
    pub fn default_example() -> Self {
        Self {
            id: "col-new".into(), kind: ColumnKind::ConcreteFilled, length_m: 3.5,
            outer_size_m: 0.300, wall_thickness_m: 0.010,
            steel_a_m2: 0.0084, steel_f_y_pa: 355e6, concrete_a_m2: 0.072, concrete_f_ck_pa: 30e6,
            reinforcement_as_m2: 0.0012, reinforcement_f_yk_pa: 500e6, i_m4: 2.1e-4, buckling_curve: BucklingCurve::A,
            introduced_force_n: 0.0, section_height_m: 0.0, section_width_m: 0.0, section_tw_m: 0.0, section_tf_m: 0.0, rebar_cover_m: 0.0, reinforcement_product: ReinforcementProduct::HotRolled,
            actions: vec![
                ColumnAction::forces("G-col", ActionStage::Composite, ActionKind::Permanent, "self", 1200e3, 20e3),
                ColumnAction::forces("Q-col", ActionStage::Composite, ActionKind::Imposed, "B", 400e3, 15e3),
            ],
        }
    }
}

impl CompositeSlab {
    pub fn default_example() -> Self {
        Self {
            id: "slab-new".into(), span_m: 3.0, support: SupportCondition::SimplySupported,
            sheeting: ProfiledSheeting::trapezoidal(),
            concrete_thickness_m: 0.140, f_ck_pa: 30e6, m_factor: 180.0, k_factor: 0.05, as_m2_per_m: 2.5e-4,
            actions: vec![
                SlabAction::permanent_area("G-slab", ActionStage::Composite, "self", 3.5e3),
                SlabAction::imposed_area("Q-slab", ActionStage::Composite, "B", 2.0e3),
            ],
        }
    }
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1994", "EN 1994", EN1994_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1994_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm.en1994", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1994_DOCUMENT_SCHEMA: &str = "semio.norm.en1994/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;

impl StructureKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Building => "building",
            Self::Bridge => "bridge",
        }
    }
}

impl std::fmt::Display for StructureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ConstructionMethod {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Propped => "propped",
            Self::Unpropped => "unpropped",
        }
    }
}

impl std::fmt::Display for ConstructionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BucklingCurve {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
            Self::C => "c",
            Self::D => "d",
        }
    }
}

impl std::fmt::Display for BucklingCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ActionStage {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Construction => "construction",
            Self::Composite => "composite",
            Self::Any => "any",
        }
    }
}

impl std::fmt::Display for ActionStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FatigueDetail {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StudWelded => "stud_welded",
            Self::ShearConnector => "shear_connector",
            Self::Reinforcement => "reinforcement",
            Self::FlangeButtWeld => "flange_butt_weld",
        }
    }
}

impl std::fmt::Display for FatigueDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FireRating {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unrated => "none",
            Self::R30 => "r30",
            Self::R60 => "r60",
            Self::R90 => "r90",
            Self::R120 => "r120",
        }
    }

    /// 🔥 The rating itself, `None` for an unrated document (the fire checks and plates are skipped).
    pub const fn rated(self) -> Option<Self> {
        match self {
            Self::Unrated => None,
            rating => Some(rating),
        }
    }
}

impl std::fmt::Display for FireRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SupportCondition {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SimplySupported => "simply_supported",
            Self::Continuous2Span => "continuous_2_span",
            Self::EndSpan => "end_span",
            Self::InteriorSpan => "interior_span",
            Self::Cantilever => "cantilever",
        }
    }
}

impl std::fmt::Display for SupportCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SheetingProfile {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trapezoidal => "trapezoidal",
            Self::ReEntrant => "re_entrant",
        }
    }
}

impl std::fmt::Display for SheetingProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ReinforcementProduct {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HotRolled => "hot_rolled",
            Self::ColdWorked => "cold_worked",
        }
    }
}

impl std::fmt::Display for ReinforcementProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ColumnKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Encased => "encased",
            Self::ConcreteFilled => "concrete_filled",
            Self::PartiallyEncased => "partially_encased",
        }
    }
}

impl std::fmt::Display for ColumnKind {
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
            Self::Snow => "snow",
            Self::Wind => "wind",
            Self::Construction => "construction",
            Self::Fatigue => "fatigue",
        }
    }
}

impl std::fmt::Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
