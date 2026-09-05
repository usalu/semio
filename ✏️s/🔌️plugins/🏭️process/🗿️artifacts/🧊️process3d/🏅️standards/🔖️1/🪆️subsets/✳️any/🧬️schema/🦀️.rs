//! 🧬️ Process3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, Pose, ProcessStep, Stock, StockQuantity, Workshop, WorkshopMachine};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;

//#region 🔖️Artifact
/// 🧬️ Full process3d artifact state across the artifact, presence and config lanes.
/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: mirrors `Process3dSnapshot`'s
/// flattened `stock_*`/composed-child field shape exactly, so `to_snapshot`/`from_snapshot` stay a
/// plain field-for-field copy.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dArtifact {
    #[state(artifact)]
    pub workshop: Workshop,
    #[state(artifact)]
    pub stock_id: String,
    #[state(artifact)]
    pub stock_label: String,
    #[state(artifact)]
    pub stock_pose: Pose,
    #[state(artifact)]
    pub stock_payload: Stock,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub stock_solid: store::ArtifactChild<SemioBrepSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub steps: store::ArtifactChild<SemioFlowSnapshot>,
    #[state(artifact)]
    pub step_payloads: Vec<ProcessStep>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub tool_solids: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    pub resolved_up_to: Option<usize>,
    #[state(presence)]
    pub selected_id: Option<String>,
    #[state(presence)]
    pub selected_face_id: Option<usize>,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub selection_method: String,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera_position_x: f64,
    #[state(config)]
    pub camera_position_y: f64,
    #[state(config)]
    pub camera_position_z: f64,
    #[state(config)]
    pub camera_target_x: f64,
    #[state(config)]
    pub camera_target_y: f64,
    #[state(config)]
    pub camera_target_z: f64,
    #[state(config)]
    pub camera_fov: f64,
    #[state(config)]
    pub sun_enabled: bool,
    #[state(config)]
    pub sun_azimuth: f64,
    #[state(config)]
    pub sun_elevation: f64,
    #[state(config)]
    pub sun_intensity: f64,
    #[state(config)]
    pub sun_color: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
    #[state(artifact)]
    pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Process3dArtifact {
    fn default() -> Self {
        let base = crate::artifacts::process3d::empty_process3d_snapshot();
        Self {
            workshop: base.workshop,
            stock_id: base.stock_id,
            stock_label: base.stock_label,
            stock_pose: base.stock_pose,
            stock_payload: base.stock_payload,
            stock_solid: base.stock_solid,
            steps: base.steps,
            step_payloads: base.step_payloads,
            tool_solids: base.tool_solids,
            resolved_up_to: None,
            selected_id: None,
            selected_face_id: None,
            active_utility_id: "select".into(),
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            camera_position_x: 3.0,
            camera_position_y: -3.0,
            camera_position_z: 2.0,
            camera_target_x: 0.0,
            camera_target_y: 0.0,
            camera_target_z: 0.0,
            camera_fov: 45.0,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
            hovered_id: None,
        }
    }
}

impl Process3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::process3d::Process3dSnapshot {
        crate::artifacts::process3d::Process3dSnapshot {
            workshop: self.workshop.clone(),
            stock_id: self.stock_id.clone(),
            stock_label: self.stock_label.clone(),
            stock_pose: self.stock_pose.clone(),
            stock_payload: self.stock_payload.clone(),
            stock_solid: self.stock_solid.clone(),
            steps: self.steps.clone(),
            step_payloads: self.step_payloads.clone(),
            tool_solids: self.tool_solids.clone(),
            resolved_up_to: self.resolved_up_to,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::process3d::Process3dSnapshot) -> Self {
        Self {
            workshop: snapshot.workshop,
            stock_id: snapshot.stock_id,
            stock_label: snapshot.stock_label,
            stock_pose: snapshot.stock_pose,
            stock_payload: snapshot.stock_payload,
            stock_solid: snapshot.stock_solid,
            steps: snapshot.steps,
            step_payloads: snapshot.step_payloads,
            tool_solids: snapshot.tool_solids,
            resolved_up_to: snapshot.resolved_up_to,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::process3d::Process3dSnapshot) {
        self.workshop = snapshot.workshop;
        self.stock_id = snapshot.stock_id;
        self.stock_label = snapshot.stock_label;
        self.stock_pose = snapshot.stock_pose;
        self.stock_solid = snapshot.stock_solid;
        self.steps = snapshot.steps;
        self.tool_solids = snapshot.tool_solids;
        self.resolved_up_to = snapshot.resolved_up_to;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.process.process3d` — twenty handcrafted schema leaves.
pub fn process3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.process.process3d",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::process3d::schema::diff::Process3dDiff;
    use crate::artifacts::process3d::schema::mutations::Process3dMutation;
    use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Process3dBuilderConstruction {
        snapshot: Process3dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Process3dBuilderConstruction {
        type Snapshot = Process3dSnapshot;
        type Mutation = Process3dMutation;
        type Diff = Process3dDiff;
        fn empty() -> Self {
            Self { snapshot: Process3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Process3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Process3dDiff as protocol::MutationDiff<Process3dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::process3d::Process3dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Process3dParts {
        pub snapshot: Option<Process3dSnapshot>,
    }

    pub struct Process3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Process3dAnalyzerAnalysis {
        type Parts = Process3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.process.process3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Process3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Process3dBuilderFacets {
        construction: Process3dBuilderConstruction,
        analysis: Process3dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Process3dComposerComposition,
    }
    builder: Process3dBuilder,
    analyzer: Process3dAnalyzer,
    composer: Process3dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ExampleFixtures
pub use crate::artifacts::process3d::dsl::{PROCESS_3D_PLATE_EXAMPLE_TEXT as PLATE_EXAMPLE_DSL, PROCESS_3D_TIMBER_EXAMPLE_TEXT as TIMBER_EXAMPLE_DSL};

pub fn default_document() -> crate::artifacts::process3d::Process3dSnapshot {
    crate::artifacts::process3d::Process3dSnapshot::parse_dsl(TIMBER_EXAMPLE_DSL).unwrap_or_default()
}

pub fn plate_document() -> crate::artifacts::process3d::Process3dSnapshot {
    crate::artifacts::process3d::Process3dSnapshot::parse_dsl(PLATE_EXAMPLE_DSL).unwrap_or_else(|_| default_document())
}
//#endregion 🔖️ExampleFixtures

//#region 🔖️Catalog
/// 🧩️ Shared capability-parameter/rule builders for every built-in domain catalog below — pulled out
/// of the four (formerly per-material-file) private copies so the identical helper exists exactly once.
fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}

fn max_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Max { quantity, parameter: parameter.into(), margin }
}

fn min_rule(quantity: StockQuantity, parameter: &str, margin: f64) -> CapabilityRule {
    CapabilityRule::Min { quantity, parameter: parameter.into(), margin }
}

/// 📦️ The built-in generic catalog — wraps `crate::artifacts::process3d::generic_machines()`, the same
/// fallback used to seed a document's default workshop, exposed here as an installable `MachineCatalog`
/// so it appears alongside domain catalogs in the workshop configurator's "installed catalogs" list.
pub struct GenericCatalog;

impl MachineCatalog for GenericCatalog {
    fn catalog_id(&self) -> &str {
        "geometry"
    }

    fn label(&self) -> &str {
        "Geometry"
    }

    fn icon_id(&self) -> &str {
        "shapes"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        crate::artifacts::process3d::generic_machines()
    }
}

/// 🔩️ Built-in metal-shop machine catalog (saws, grinding, drilling, plasma, welding). Folded in from
/// the old, standalone `semio-s-plugin-process-metal` crate.
pub struct MetalCatalog;

impl MachineCatalog for MetalCatalog {
    fn catalog_id(&self) -> &str {
        "metal"
    }

    fn label(&self) -> &str {
        "Metal"
    }

    fn icon_id(&self) -> &str {
        "wrench"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "chopSaw".into(),
                label: "Chop Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "chop".into(),
                    label: "Chop".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.355), parameter("kerf", "Kerf", 0.003), parameter("maxCutDepth", "Max Cut Depth", 0.12), parameter("maxStockWidth", "Max Stock Width", 0.23)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0), max_rule(StockQuantity::Width, "maxStockWidth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "angleGrinder".into(),
                label: "Angle Grinder".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "grindCut".into(),
                    label: "Grind Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "discDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("discDiameter", "Disc Diameter", 0.125), parameter("kerf", "Kerf", 0.0025), parameter("maxCutDepth", "Max Cut Depth", 0.038)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "pillarDrill".into(),
                label: "Pillar Drill".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "bore".into(),
                    label: "Bore".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "strokeDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.008), parameter("strokeDepth", "Stroke Depth", 0.08)],
                    rules: vec![max_rule(StockQuantity::Height, "strokeDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "magDrill".into(),
                label: "Mag Drill".into(),
                icon_id: "magnet".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "annularBore".into(),
                    label: "Annular Bore".into(),
                    icon_id: "magnet".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "cutterLength".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.017), parameter("cutterLength", "Cutter Length", 0.05), parameter("minPlateThickness", "Min Plate Thickness", 0.006)],
                    rules: vec![max_rule(StockQuantity::Height, "cutterLength", 0.0), min_rule(StockQuantity::Height, "minPlateThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "plasmaCutter".into(),
                label: "Plasma Cutter".into(),
                icon_id: "sparkles".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "plasmaCut".into(),
                    label: "Plasma Cut".into(),
                    icon_id: "sparkles".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.0015), parameter("cutLength", "Cut Length", 1.5), parameter("maxCutThickness", "Max Cut Thickness", 0.02)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "migWelder".into(),
                label: "MIG Welder".into(),
                icon_id: "combine".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "weldBead".into(),
                        label: "Weld Bead".into(),
                        icon_id: "combine".into(),
                        recipe: MeasureRecipe::CylinderAttach { radius: "beadRadius".into(), length: "beadLength".into() },
                        parameters: vec![parameter("beadRadius", "Bead Radius", 0.004), parameter("beadLength", "Bead Length", 0.05), parameter("minThickness", "Min Thickness", 0.0008)],
                        rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                    },
                    Capability {
                        id: "weldPlate".into(),
                        label: "Weld Plate".into(),
                        icon_id: "combine".into(),
                        recipe: MeasureRecipe::BoxAttach { width: "plateWidth".into(), depth: "plateDepth".into(), height: "plateThickness".into() },
                        parameters: vec![parameter("plateWidth", "Plate Width", 0.1), parameter("plateDepth", "Plate Depth", 0.1), parameter("plateThickness", "Plate Thickness", 0.008), parameter("minThickness", "Min Thickness", 0.0008)],
                        rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "studWelder".into(),
                label: "Stud Welder".into(),
                icon_id: "plug".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "stud".into(),
                    label: "Stud".into(),
                    icon_id: "plug".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "studRadius".into(), length: "studLength".into() },
                    parameters: vec![parameter("studRadius", "Stud Radius", 0.005), parameter("studLength", "Stud Length", 0.025), parameter("minThickness", "Min Thickness", 0.002)],
                    rules: vec![min_rule(StockQuantity::Height, "minThickness", 0.0)],
                }],
            },
        ]
    }
}

pub fn metal_catalog() -> MetalCatalog {
    MetalCatalog
}

/// 🪵️ Built-in wood-shop machine catalog (saws, drill press, CNC router, fasteners). Folded in from the
/// old, standalone `semio-s-plugin-process-wood` crate.
pub struct WoodCatalog;

impl MachineCatalog for WoodCatalog {
    fn catalog_id(&self) -> &str {
        "wood"
    }

    fn label(&self) -> &str {
        "Wood"
    }

    fn icon_id(&self) -> &str {
        "beam"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "circularSaw".into(),
                label: "Circular Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "crosscut".into(),
                    label: "Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.184), parameter("kerf", "Kerf", 0.002), parameter("maxCutDepth", "Max Cut Depth", 0.065)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "tableSaw".into(),
                label: "Table Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "rip".into(),
                    label: "Rip".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.315), parameter("kerf", "Kerf", 0.0032), parameter("maxCutDepth", "Max Cut Depth", 0.102), parameter("fenceWidth", "Fence Width", 0.8)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0), max_rule(StockQuantity::Width, "fenceWidth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "bandSaw".into(),
                label: "Band Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "curveCut".into(),
                    label: "Curve Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "bladeLength".into(), depth: "maxCutHeight".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.0015), parameter("bladeLength", "Blade Length", 0.5), parameter("maxCutHeight", "Max Cut Height", 0.30), parameter("throatDepth", "Throat Depth", 0.44)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutHeight", 0.0), max_rule(StockQuantity::Width, "throatDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "chainSaw".into(),
                label: "Chain Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "roughCut".into(),
                    label: "Rough Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "barLength".into(), depth: "barLength".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.008), parameter("barLength", "Bar Length", 0.45), parameter("minStockDimension", "Min Stock Dimension", 0.05)],
                    rules: vec![max_rule(StockQuantity::Height, "barLength", 0.0), min_rule(StockQuantity::MinDimension, "minStockDimension", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "drillPress".into(),
                label: "Drill Press".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "bore".into(),
                    label: "Bore".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "strokeDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.005), parameter("strokeDepth", "Stroke Depth", 0.10), parameter("throatDepth", "Throat Depth", 0.16)],
                    rules: vec![max_rule(StockQuantity::Height, "strokeDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "cncRouter".into(),
                label: "CNC Router".into(),
                icon_id: "cpu".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "pocket".into(),
                        label: "Pocket".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "pocketDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.012), parameter("pocketDepth", "Pocket Depth", 0.04), parameter("bedWidth", "Bed Width", 1.25), parameter("bedDepth", "Bed Depth", 2.5)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                    Capability {
                        id: "bore".into(),
                        label: "Bore".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "boreDepth".into() },
                        parameters: vec![parameter("bitRadius", "Bit Radius", 0.006), parameter("boreDepth", "Bore Depth", 0.04), parameter("bedWidth", "Bed Width", 1.25), parameter("bedDepth", "Bed Depth", 2.5)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "dowelJig".into(),
                label: "Doweling Jig".into(),
                icon_id: "plus".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "dowel".into(),
                    label: "Dowel".into(),
                    icon_id: "plus".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "dowelRadius".into(), length: "dowelLength".into() },
                    parameters: vec![parameter("dowelRadius", "Dowel Radius", 0.004), parameter("dowelLength", "Dowel Length", 0.04), parameter("minStockThickness", "Min Stock Thickness", 0.018)],
                    rules: vec![min_rule(StockQuantity::Height, "minStockThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "screwGun".into(),
                label: "Screw Gun".into(),
                icon_id: "wrench".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "screw".into(),
                    label: "Screw".into(),
                    icon_id: "wrench".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "screwRadius".into(), length: "screwLength".into() },
                    parameters: vec![parameter("screwRadius", "Screw Radius", 0.0025), parameter("screwLength", "Screw Length", 0.05)],
                    rules: vec![min_rule(StockQuantity::Height, "screwLength", 0.0)],
                }],
            },
        ]
    }
}

pub fn wood_catalog() -> WoodCatalog {
    WoodCatalog
}

/// 🤖️ Built-in robotic/CNC machine catalog (multi-axis mills, gantry CNC, waterjet, laser, assembler).
/// Folded in from the old, standalone `semio-s-plugin-process-robotic` crate.
pub struct RoboticCatalog;

impl MachineCatalog for RoboticCatalog {
    fn catalog_id(&self) -> &str {
        "robotic"
    }

    fn label(&self) -> &str {
        "Robotic"
    }

    fn icon_id(&self) -> &str {
        "cpu"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "sixAxisMill".into(),
                label: "6-Axis Robotic Mill".into(),
                icon_id: "cpu".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "mill".into(),
                        label: "Mill".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "millDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.02), parameter("millDepth", "Mill Depth", 0.1), parameter("reach", "Reach", 2.8)],
                        rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                    },
                    Capability {
                        id: "bore".into(),
                        label: "Bore".into(),
                        icon_id: "cpu".into(),
                        recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "boreDepth".into() },
                        parameters: vec![parameter("bitRadius", "Bit Radius", 0.01), parameter("boreDepth", "Bore Depth", 0.15), parameter("reach", "Reach", 2.8)],
                        rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "gantryCnc".into(),
                label: "5-Axis Gantry CNC".into(),
                icon_id: "grid-3x3".into(),
                catalog_id: None,
                capabilities: vec![
                    Capability {
                        id: "mill".into(),
                        label: "Mill".into(),
                        icon_id: "grid-3x3".into(),
                        recipe: MeasureRecipe::PocketCut { diameter: "endmillDiameter".into(), depth: "millDepth".into() },
                        parameters: vec![parameter("endmillDiameter", "Endmill Diameter", 0.025), parameter("millDepth", "Mill Depth", 0.2), parameter("bedWidth", "Bed Width", 3.5), parameter("bedDepth", "Bed Depth", 12.0)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                    Capability {
                        id: "saw".into(),
                        label: "Saw".into(),
                        icon_id: "grid-3x3".into(),
                        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "sawDepth".into() },
                        parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.5), parameter("kerf", "Kerf", 0.005), parameter("sawDepth", "Saw Depth", 0.2), parameter("bedWidth", "Bed Width", 3.5), parameter("bedDepth", "Bed Depth", 12.0)],
                        rules: vec![max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                    },
                ],
            },
            WorkshopMachine {
                id: "waterjet".into(),
                label: "Waterjet".into(),
                icon_id: "pen-tool".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "jetCut".into(),
                    label: "Jet Cut".into(),
                    icon_id: "pen-tool".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![
                        parameter("kerf", "Kerf", 0.001),
                        parameter("cutLength", "Cut Length", 1.5),
                        parameter("maxCutThickness", "Max Cut Thickness", 0.2),
                        parameter("bedWidth", "Bed Width", 3.0),
                        parameter("bedDepth", "Bed Depth", 1.5),
                    ],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0), max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "laserCutter".into(),
                label: "Laser Cutter".into(),
                icon_id: "scan-line".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "laserCut".into(),
                    label: "Laser Cut".into(),
                    icon_id: "scan-line".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "cutLength".into(), depth: "maxCutThickness".into() },
                    parameters: vec![
                        parameter("kerf", "Kerf", 0.0002),
                        parameter("cutLength", "Cut Length", 1.5),
                        parameter("maxCutThickness", "Max Cut Thickness", 0.025),
                        parameter("bedWidth", "Bed Width", 1.5),
                        parameter("bedDepth", "Bed Depth", 3.0),
                    ],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutThickness", 0.0), max_rule(StockQuantity::Width, "bedWidth", 0.0), max_rule(StockQuantity::Depth, "bedDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "assembler".into(),
                label: "Robotic Assembler".into(),
                icon_id: "component".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "place".into(),
                    label: "Place".into(),
                    icon_id: "component".into(),
                    recipe: MeasureRecipe::BoxAttach { width: "componentWidth".into(), depth: "componentDepth".into(), height: "componentHeight".into() },
                    parameters: vec![parameter("componentWidth", "Component Width", 0.1), parameter("componentDepth", "Component Depth", 0.1), parameter("componentHeight", "Component Height", 0.1), parameter("reach", "Reach", 2.8)],
                    rules: vec![max_rule(StockQuantity::MaxDimension, "reach", 0.0)],
                }],
            },
        ]
    }
}

pub fn robotic_catalog() -> RoboticCatalog {
    RoboticCatalog
}

/// 🧱️ Built-in concrete-shop machine catalog (saws, core drilling, anchors, grinding). Folded in from
/// the old, standalone `semio-s-plugin-process-concrete` crate.
pub struct ConcreteCatalog;

impl MachineCatalog for ConcreteCatalog {
    fn catalog_id(&self) -> &str {
        "concrete"
    }

    fn label(&self) -> &str {
        "Concrete"
    }

    fn icon_id(&self) -> &str {
        "slab"
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        vec![
            WorkshopMachine {
                id: "diamondSaw".into(),
                label: "Diamond Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "crosscut".into(),
                    label: "Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.35), parameter("kerf", "Kerf", 0.004), parameter("maxCutDepth", "Max Cut Depth", 0.125)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "wallSaw".into(),
                label: "Wall Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "wallCut".into(),
                    label: "Wall Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![parameter("bladeDiameter", "Blade Diameter", 0.8), parameter("kerf", "Kerf", 0.0045), parameter("maxCutDepth", "Max Cut Depth", 0.32)],
                    rules: vec![max_rule(StockQuantity::Height, "maxCutDepth", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "wireSaw".into(),
                label: "Wire Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "wireCut".into(),
                    label: "Wire Cut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "wireSpan".into(), depth: "maxSection".into() },
                    parameters: vec![parameter("kerf", "Kerf", 0.011), parameter("wireSpan", "Wire Span", 3.0), parameter("maxSection", "Max Section", 2.5)],
                    rules: vec![max_rule(StockQuantity::MaxDimension, "maxSection", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "coreDrill".into(),
                label: "Core Drill".into(),
                icon_id: "circle-dot".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "core".into(),
                    label: "Core".into(),
                    icon_id: "circle-dot".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "coreLength".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.051), parameter("coreLength", "Core Length", 0.45)],
                    rules: vec![max_rule(StockQuantity::Height, "coreLength", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "rotaryHammer".into(),
                label: "Rotary Hammer".into(),
                icon_id: "hammer".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "anchorHole".into(),
                    label: "Anchor Hole".into(),
                    icon_id: "hammer".into(),
                    recipe: MeasureRecipe::BoreDrill { radius: "bitRadius".into(), depth: "maxDrillDepth".into() },
                    parameters: vec![parameter("bitRadius", "Bit Radius", 0.006), parameter("maxDrillDepth", "Max Drill Depth", 0.16), parameter("minStockThickness", "Min Stock Thickness", 0.01)],
                    rules: vec![min_rule(StockQuantity::Height, "minStockThickness", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "anchorSetter".into(),
                label: "Anchor Setter".into(),
                icon_id: "plus".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "anchor".into(),
                    label: "Anchor".into(),
                    icon_id: "plus".into(),
                    recipe: MeasureRecipe::CylinderAttach { radius: "anchorRadius".into(), length: "anchorLength".into() },
                    parameters: vec![parameter("anchorRadius", "Anchor Radius", 0.008), parameter("anchorLength", "Anchor Length", 0.11), parameter("minEmbedment", "Min Embedment", 0.07)],
                    rules: vec![min_rule(StockQuantity::Height, "minEmbedment", 0.0)],
                }],
            },
            WorkshopMachine {
                id: "surfaceGrinder".into(),
                label: "Surface Grinder".into(),
                icon_id: "layers".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "grind".into(),
                    label: "Grind".into(),
                    icon_id: "layers".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "padDiameter".into(), kerf: "grindDepth".into() },
                    parameters: vec![parameter("padDiameter", "Pad Diameter", 0.25), parameter("grindDepth", "Grind Depth", 0.005), parameter("minGrindDimension", "Min Grind Dimension", 0.02)],
                    rules: vec![min_rule(StockQuantity::MinDimension, "minGrindDimension", 0.0)],
                }],
            },
        ]
    }
}

pub fn concrete_catalog() -> ConcreteCatalog {
    ConcreteCatalog
}
//#endregion 🔖️Catalog

//#region 🔖️DocumentHelpers
/// 🪪️ A pseudo-random step id — collision odds are astronomically low for a single-document timeline.
pub fn next_step_id() -> String {
    format!("step-{}", &framework_hash::hash_bytes(concat!(file!(), line!(), "step-{}").as_bytes())[..12])
}

/// ✂️➕️ Read-only operation builders for the two structural collection edits every mutating command
/// needs: inserting a step at the resolved-up-to cursor (and advancing it), and removing a step by id
/// (and pulling the cursor back if it sat past the removed step). Shared by the `🎮️commands/🪜️step` and
/// `🎮️commands/🌍️world` command modules — building `Process3dMutation`s from an immutable
/// `&Process3dSnapshot` keeps every handler free of manual mutation, since the VCS store applies them.
///
/// 🌉️ Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline record
/// (`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4) — `CreateStep`/`DeleteStep` are real
/// mutations against it now, so these builders compute a real index/cursor from `fixture.step_payloads`
/// again instead of guessing.
pub fn insert_step_mutations(fixture: &crate::artifacts::process3d::Process3dSnapshot, step: ProcessStep) -> Vec<crate::artifacts::process3d::op::Process3dMutation> {
    use crate::artifacts::process3d::op::Process3dMutation;
    use crate::artifacts::process3d::schema::mutations::{change_cursor, create_step};
    let index = fixture.resolved_up_to.unwrap_or(fixture.step_payloads.len());
    let mut operations = vec![Process3dMutation::CreateStep(create_step::CreateStep { index, step })];
    if fixture.resolved_up_to.is_some() {
        operations.push(Process3dMutation::ChangeCursor(change_cursor::ChangeCursor { new_resolved_up_to: Some(index + 1) }));
    }
    operations
}

pub fn remove_step_mutations(fixture: &crate::artifacts::process3d::Process3dSnapshot, id: &str) -> Option<Vec<crate::artifacts::process3d::op::Process3dMutation>> {
    use crate::artifacts::process3d::op::Process3dMutation;
    use crate::artifacts::process3d::schema::mutations::{change_cursor, delete_step};
    let removed_index = fixture.step_payloads.iter().position(|step| step.id == id)?;
    let mut operations = vec![Process3dMutation::DeleteStep(delete_step::DeleteStep { id: id.to_string() })];
    if let Some(cursor) = fixture.resolved_up_to {
        if cursor > removed_index {
            operations.push(Process3dMutation::ChangeCursor(change_cursor::ChangeCursor { new_resolved_up_to: Some(cursor.saturating_sub(1)) }));
        }
    }
    Some(operations)
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️ExampleFixtures
    /// 🧭️ Every `step.origin` on a document must name a machine+capability that actually exists in
    /// that document's own `workshop` — the mutations' only source of truth for legal origins.
    fn assert_origins_resolve(document: &crate::artifacts::process3d::Process3dSnapshot) {
        for step in &document.step_payloads {
            let origin = step.origin.as_ref().unwrap_or_else(|| panic!("step {:?} is missing its origin", step.id));
            let machine = document.workshop.machines.iter().find(|m| m.id == origin.machine_id).unwrap_or_else(|| panic!("step {:?} references unknown machine {:?}", step.id, origin.machine_id));
            assert!(
                machine.capabilities.iter().any(|c| c.id == origin.capability_id),
                "step {:?} references unknown capability {:?} on machine {:?}",
                step.id,
                origin.capability_id,
                machine.id
            );
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn default_document_parses_timber_example() {
        let document = default_document();
        assert!(!document.steps.child_id.is_empty());
        assert!(document.resolved_up_to.is_none());

        let printed = document.print_dsl();
        let round_tripped = <crate::artifacts::process3d::Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("timber fixture round trip");
        assert_eq!(round_tripped, document, "timber fixture must round-trip through print_dsl/parse_dsl unchanged");

        match &document.stock_payload.solid {
            crate::artifacts::process3d::WorkingSolid::Box { width, depth, height } => {
                assert!((*width - 3.0).abs() < 1e-9, "timber beam width should be 3.0m, got {width}");
                assert!((*depth - 0.2).abs() < 1e-9, "timber beam depth should be 0.2m, got {depth}");
                assert!((*height - 0.3).abs() < 1e-9, "timber beam height should be 0.3m, got {height}");
            }
            other => panic!("expected timber beam stock to be a non-degenerate Box, got {other:?}"),
        }

        let expected_ids = ["crosscut", "lap-joint-cut", "dowel-drill", "dowel-attach"];
        assert_eq!(document.step_payloads.len(), expected_ids.len(), "timber joinery timeline should have {} steps", expected_ids.len());
        for (step, expected_id) in document.step_payloads.iter().zip(expected_ids.iter()) {
            assert_eq!(&step.id, expected_id);
            assert!(step.enabled, "step {:?} should be enabled", step.id);
        }
        assert_origins_resolve(&document);
    }

    #[semio_framework_async_macros::async_test]
    async fn plate_document_parses_and_opens_mid_timeline() {
        let document = plate_document();
        assert!(!document.steps.child_id.is_empty());
        assert_eq!(document.resolved_up_to, Some(2));

        let printed = document.print_dsl();
        let round_tripped = <crate::artifacts::process3d::Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("plate fixture round trip");
        assert_eq!(round_tripped, document, "plate fixture must round-trip through print_dsl/parse_dsl unchanged");

        match &document.stock_payload.solid {
            crate::artifacts::process3d::WorkingSolid::Box { width, depth, height } => {
                assert!((*width - 1.2).abs() < 1e-9, "plate width should be 1.2m, got {width}");
                assert!((*depth - 0.8).abs() < 1e-9, "plate depth should be 0.8m, got {depth}");
                assert!((*height - 0.02).abs() < 1e-9, "plate height should be 0.02m, got {height}");
            }
            other => panic!("expected plate stock to be a non-degenerate Box, got {other:?}"),
        }

        assert_eq!(document.step_payloads.len(), 4, "drilled plate timeline should have 4 holes");
        for (index, step) in document.step_payloads.iter().enumerate() {
            assert_eq!(step.id, format!("drill-{}", index + 1));
            assert!(step.enabled, "step {:?} should be enabled", step.id);
            assert!(matches!(step.measure, crate::artifacts::process3d::ProcessMeasure::Drill { .. }), "plate step {:?} should be a Drill measure", step.id);
        }
        assert_origins_resolve(&document);
    }
    //#endregion 🔖️ExampleFixtures

    //#region 🔖️MetalCatalog
    mod metal_catalog_tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn every_machine_and_capability_id_is_unique() {
            let machines = MetalCatalog.machines();
            let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
            machine_ids.sort_unstable();
            machine_ids.dedup();
            assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in metal catalog");
            for machine in &machines {
                let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
                capability_ids.sort_unstable();
                capability_ids.dedup();
                assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn every_recipe_and_rule_parameter_resolves() {
            for machine in MetalCatalog.machines() {
                for capability in &machine.capabilities {
                    let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                    let recipe_params: Vec<&str> = match &capability.recipe {
                        MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                        MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                        MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                        MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                        MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                        MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                    };
                    for name in recipe_params {
                        assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                    }
                    for rule in &capability.rules {
                        let name = match rule {
                            CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                        };
                        assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                    }
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn machines_round_trip_json() {
            let machines = MetalCatalog.machines();
            let json = semio_framework_os_kernel::json::to_json_string(&machines);
            let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
            assert_eq!(parsed, machines);
        }

        #[semio_framework_async_macros::async_test]
        async fn catalog_has_metal_identity() {
            let catalog = MetalCatalog;
            assert_eq!(catalog.catalog_id(), "metal");
            assert_eq!(catalog.label(), "Metal");
        }
    }
    //#endregion 🔖️MetalCatalog

    //#region 🔖️WoodCatalog
    mod wood_catalog_tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn every_machine_and_capability_id_is_unique() {
            let machines = WoodCatalog.machines();
            let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
            machine_ids.sort_unstable();
            machine_ids.dedup();
            assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in wood catalog");
            for machine in &machines {
                let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
                capability_ids.sort_unstable();
                capability_ids.dedup();
                assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
            }
        }

        /// ✅️ Every recipe field and rule parameter must resolve within its own capability's parameters.
        #[semio_framework_async_macros::async_test]
        async fn every_recipe_and_rule_parameter_resolves() {
            for machine in WoodCatalog.machines() {
                for capability in &machine.capabilities {
                    let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                    let recipe_params: Vec<&str> = match &capability.recipe {
                        MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                        MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                        MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                        MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                        MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                        MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                    };
                    for name in recipe_params {
                        assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                    }
                    for rule in &capability.rules {
                        let name = match rule {
                            CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                        };
                        assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                    }
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn machines_round_trip_json() {
            let machines = WoodCatalog.machines();
            let json = semio_framework_os_kernel::json::to_json_string(&machines);
            let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
            assert_eq!(parsed, machines);
        }

        #[semio_framework_async_macros::async_test]
        async fn catalog_has_wood_identity() {
            let catalog = WoodCatalog;
            assert_eq!(catalog.catalog_id(), "wood");
            assert_eq!(catalog.label(), "Wood");
        }
    }
    //#endregion 🔖️WoodCatalog

    //#region 🔖️RoboticCatalog
    mod robotic_catalog_tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn every_machine_and_capability_id_is_unique() {
            let machines = RoboticCatalog.machines();
            let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
            machine_ids.sort_unstable();
            machine_ids.dedup();
            assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in robotic catalog");
            for machine in &machines {
                let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
                capability_ids.sort_unstable();
                capability_ids.dedup();
                assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn every_recipe_and_rule_parameter_resolves() {
            for machine in RoboticCatalog.machines() {
                for capability in &machine.capabilities {
                    let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                    let recipe_params: Vec<&str> = match &capability.recipe {
                        MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                        MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                        MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                        MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                        MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                        MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                    };
                    for name in recipe_params {
                        assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                    }
                    for rule in &capability.rules {
                        let name = match rule {
                            CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                        };
                        assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                    }
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn machines_round_trip_json() {
            let machines = RoboticCatalog.machines();
            let json = semio_framework_os_kernel::json::to_json_string(&machines);
            let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
            assert_eq!(parsed, machines);
        }

        #[semio_framework_async_macros::async_test]
        async fn catalog_has_robotic_identity() {
            let catalog = RoboticCatalog;
            assert_eq!(catalog.catalog_id(), "robotic");
            assert_eq!(catalog.label(), "Robotic");
        }
    }
    //#endregion 🔖️RoboticCatalog

    //#region 🔖️ConcreteCatalog
    mod concrete_catalog_tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn every_machine_and_capability_id_is_unique() {
            let machines = ConcreteCatalog.machines();
            let mut machine_ids: Vec<&str> = machines.iter().map(|machine| machine.id.as_str()).collect();
            machine_ids.sort_unstable();
            machine_ids.dedup();
            assert_eq!(machine_ids.len(), machines.len(), "duplicate machine id in concrete catalog");
            for machine in &machines {
                let mut capability_ids: Vec<&str> = machine.capabilities.iter().map(|capability| capability.id.as_str()).collect();
                capability_ids.sort_unstable();
                capability_ids.dedup();
                assert_eq!(capability_ids.len(), machine.capabilities.len(), "duplicate capability id on machine {}", machine.id);
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn every_recipe_and_rule_parameter_resolves() {
            for machine in ConcreteCatalog.machines() {
                for capability in &machine.capabilities {
                    let ids: Vec<&str> = capability.parameters.iter().map(|parameter| parameter.id.as_str()).collect();
                    let recipe_params: Vec<&str> = match &capability.recipe {
                        MeasureRecipe::DiscCut { diameter, kerf } => vec![diameter.as_str(), kerf.as_str()],
                        MeasureRecipe::BladeCut { kerf, length, depth } => vec![kerf.as_str(), length.as_str(), depth.as_str()],
                        MeasureRecipe::PocketCut { diameter, depth } => vec![diameter.as_str(), depth.as_str()],
                        MeasureRecipe::BoreDrill { radius, depth } => vec![radius.as_str(), depth.as_str()],
                        MeasureRecipe::CylinderAttach { radius, length } => vec![radius.as_str(), length.as_str()],
                        MeasureRecipe::BoxAttach { width, depth, height } => vec![width.as_str(), depth.as_str(), height.as_str()],
                    };
                    for name in recipe_params {
                        assert!(ids.contains(&name), "{}.{}: recipe references unknown parameter '{name}'", machine.id, capability.id);
                    }
                    for rule in &capability.rules {
                        let name = match rule {
                            CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter.as_str(),
                        };
                        assert!(ids.contains(&name), "{}.{}: rule references unknown parameter '{name}'", machine.id, capability.id);
                    }
                }
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn machines_round_trip_json() {
            let machines = ConcreteCatalog.machines();
            let json = semio_framework_os_kernel::json::to_json_string(&machines);
            let parsed: Vec<WorkshopMachine> = semio_framework_os_kernel::json::from_json_str(&json).expect("deserialize");
            assert_eq!(parsed, machines);
        }

        #[semio_framework_async_macros::async_test]
        async fn catalog_has_concrete_identity() {
            let catalog = ConcreteCatalog;
            assert_eq!(catalog.catalog_id(), "concrete");
            assert_eq!(catalog.label(), "Concrete");
        }
    }
    //#endregion 🔖️ConcreteCatalog
    //#region 🔖️DocumentHelpers
    fn timeline_fixture(cursor: Option<usize>) -> crate::artifacts::process3d::Process3dSnapshot {
        use crate::artifacts::process3d::{Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, WorkingSolid, Workshop};
        let step = |id: &str| ProcessStep {
            id: id.into(),
            label: id.into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.05, pose: Pose::default() },
        };
        let scene = ProcessWorkingScene {
            stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 0.5, height: 0.25 }, pose: Pose::default() },
            steps: vec![step("a"), step("b"), step("c"), step("d")],
        };
        crate::artifacts::process3d::process_working_scene_to_snapshot(&scene, Workshop::default(), cursor)
    }

    fn new_step() -> crate::artifacts::process3d::ProcessStep {
        use crate::artifacts::process3d::{Pose, ProcessMeasure, ProcessStep, WorkingSolid};
        ProcessStep { id: "e".into(), label: "e".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.01, depth: 0.2, height: 0.2 }, pose: Pose::default() } }
    }

    /// 🧭️ `resolved_up_to` is a COUNT of resolved steps (`replay_process` slices `step_payloads[..limit]`),
    /// so a step added at cursor `c` must land AT index `c` and push the cursor to `c + 1` — that is the
    /// only pairing under which the step the user just added is the one that becomes visible.
    #[semio_framework_async_macros::async_test]
    async fn inserting_a_step_at_the_cursor_makes_that_step_the_newly_resolved_one() {
        use crate::artifacts::process3d::op::Process3dMutation;
        let fixture = timeline_fixture(Some(2));
        let operations = insert_step_mutations(&fixture, new_step());
        match &operations[0] {
            Process3dMutation::CreateStep(create) => assert_eq!(create.index, 2, "the new step must land at the cursor, not past it"),
            other => panic!("expected CreateStep, got {other:?}"),
        }
        match &operations[1] {
            Process3dMutation::ChangeCursor(cursor) => assert_eq!(cursor.new_resolved_up_to, Some(3), "the cursor must advance past the step just added"),
            other => panic!("expected ChangeCursor, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn inserting_a_step_with_no_cursor_appends_and_leaves_the_cursor_alone() {
        use crate::artifacts::process3d::op::Process3dMutation;
        let fixture = timeline_fixture(None);
        let operations = insert_step_mutations(&fixture, new_step());
        assert_eq!(operations.len(), 1, "a fully-resolved document needs no cursor mutation");
        match &operations[0] {
            Process3dMutation::CreateStep(create) => assert_eq!(create.index, 4),
            other => panic!("expected CreateStep, got {other:?}"),
        }
    }

    /// 🧭️ Deleting the first UNRESOLVED step (index == cursor) leaves the resolved prefix untouched, so
    /// the cursor must not move; only a deletion strictly inside the prefix pulls it back.
    #[semio_framework_async_macros::async_test]
    async fn removing_a_step_only_pulls_the_cursor_back_when_the_step_was_inside_the_resolved_prefix() {
        use crate::artifacts::process3d::op::Process3dMutation;
        let fixture = timeline_fixture(Some(2));
        assert_eq!(remove_step_mutations(&fixture, "c").expect("step c exists").len(), 1, "deleting the first unresolved step must not move the cursor");
        let inside = remove_step_mutations(&fixture, "b").expect("step b exists");
        assert_eq!(inside.len(), 2);
        match &inside[1] {
            Process3dMutation::ChangeCursor(cursor) => assert_eq!(cursor.new_resolved_up_to, Some(1)),
            other => panic!("expected ChangeCursor, got {other:?}"),
        }
        assert!(remove_step_mutations(&fixture, "missing").is_none(), "an unknown id yields no operations at all");
    }
    //#endregion 🔖️DocumentHelpers
}
//#endregion 🧪️Tests
