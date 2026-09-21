//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::os_pack::json::Value;
use dsl::DslValue;
use framework_schema::ArtifactSchema;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Artifact
/// 🧬️ Shared wires artifact content.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: crate::WiresContentChild,
    #[state(artifact)]
    pub meta: DslValue,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self { wires_fixture: crate::empty_wires_fixture(), content: crate::wires_content_child_with_owner(Vec::new(), Vec::new()), meta: DslValue::Null }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::WiresSnapshot {
        crate::WiresSnapshot { wires_fixture: self.wires_fixture.clone(), content: self.content.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds the shared artifact from its document snapshot.
    pub fn from_snapshot(snapshot: crate::WiresSnapshot) -> Self {
        Self { wires_fixture: snapshot.wires_fixture, content: snapshot.content, meta: snapshot.meta }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.content = snapshot.content;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ Hand-rolled `ArtifactBuilder` — the generic `semio_framework_plugin::app::SnapshotBuilder<S, M>`
/// (ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`, `📓️w4-sequence-report.md`'s
/// zero-boilerplate replacement) does NOT fit here: its `ArtifactBuilder` impl requires `S: Default`,
/// and `WiresSnapshot` deliberately has none — `content` is a composed `ArtifactChild` that needs a
/// freshly-minted, content-addressed handle (`empty_wires_snapshot()`'s
/// `wires_content_child_with_owner`), not a blanket zero value. This is the same class of
/// "the generic doesn't fit, keep the hand-rolled type" finding as `📓️w4-sequence-report.md`
/// `## recipeGaps` #1 (there for `ArtifactInferrer`, here for `ArtifactBuilder`). No current caller
/// exercises `ArtifactBuilder` for this subset (confirmed: zero references outside this module,
/// `derive_artifact_facets!`'s deleted generated wrapper, and the deleted `io_registry`) — kept as
/// real, correctly-typed SDK equipment matching the fan-out's established `Construction` convention,
/// not dead API (mirrors how `SurfaceDeclaration.mutation_roster` is kept unread, per debt tracked in
/// `📓️w1-c-report.md` openQuestion 3).
pub mod derived_construction {
    use crate::schema::diff::WiresDiff;
    use crate::schema::mutations::WiresMutation;
    use crate::schema::snapshot::WiresSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug)]
    pub struct WiresBuilderConstruction {
        snapshot: WiresSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for WiresBuilderConstruction {
        type Snapshot = WiresSnapshot;
        type Mutation = WiresMutation;
        type Diff = WiresDiff;
        fn empty() -> Self {
            Self { snapshot: crate::empty_wires_snapshot(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::MutationDiff::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot)?;
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
//#endregion 🏗️Construction

//#region 🔖️DocumentHelpers
/// 🧬️ Pure helpers over `DslValue`-shaped documents — dissolved from the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): every fn here is generic over document shape
/// (never `WiresSnapshot`, never an app type) so it has no home more specific than the artifact schema.
/// Reads that DO take `&WiresSnapshot` (`find_board_node`/`find_board_edge`/`find_relationship`) live
/// in `💡️inferences/` instead — see that file's `🔖️LookupHelpers` region.
pub fn array_mut<'a>(fixture: &'a mut DslValue, key: &str) -> &'a mut Vec<DslValue> {
    if !matches!(fixture, DslValue::Object(_)) {
        *fixture = DslValue::Object(vec![]);
    }
    let DslValue::Object(entries) = fixture else {
        unreachable!("fixture coerced to object above");
    };
    if let Some(idx) = entries.iter().position(|(entry_key, _)| entry_key == key) {
        let value = &mut entries[idx].1;
        if !matches!(value, DslValue::Array(_)) {
            *value = DslValue::Array(vec![]);
        }
        match value {
            DslValue::Array(items) => items,
            _ => unreachable!("array coerced above"),
        }
    } else {
        entries.push((key.to_string(), DslValue::Array(vec![])));
        match &mut entries.last_mut().expect("just pushed").1 {
            DslValue::Array(items) => items,
            _ => unreachable!("just pushed array"),
        }
    }
}

pub fn entity_id<'a>(entity: &'a DslValue, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` (and similar numeric-id fields) read as a
/// whole `u64` regardless of whether the source JSON number is an integer or a float literal. Fixtures
/// round-tripped through the `.wires` DSL text arrive as exact JSON integers (`Number(1)`, see
/// `IdentityDsl`/`RelationshipDsl`'s plain `u64` fields), so this fallback stays for documents built or
/// patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where nothing enforces the
/// integer representation.
pub fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}

pub fn dsl_to_json(value: &DslValue) -> Value {
    dsl::os_pack::json::from_dsl_value(value)
}

pub fn fixture_json_string(fixture: &DslValue) -> String {
    dsl::os_pack::json::to_json_string(fixture)
}

pub fn fixture_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

pub fn fixture_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture.get("nodes").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn fixture_edges(fixture: &DslValue) -> &[DslValue] {
    fixture.get("edges").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires.get("identities").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires.get("relationships").and_then(|value| value.as_array()).unwrap_or(&[])
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
pub fn node_position(node: &DslValue) -> (f64, f64) {
    (node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, parsed from `crate::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`.
/// The committed asset IS the example — the only content `setActiveExample`, the `.example` manifest
/// registration and every metabolism test ever see. It used to be a stub envelope (an empty board
/// plus one "Demo" node) that a hand-built in-code graph silently stood in for whenever the parse
/// yielded fewer than seven nodes, so the play pane, which loads the asset itself, rendered an empty
/// canvas while every unit test saw the seven-node graph. The fallback is gone and the asset carries
/// the real graph (regenerated with this crate's own `ArtifactDsl::print_dsl`).
pub fn metabolism_wires_example_snapshot() -> protocol::MutationApplyResult<crate::WiresSnapshot> {
    <crate::WiresSnapshot as store::ArtifactDsl>::parse_dsl(crate::document_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT)
        .map_err(|error| protocol::MutationApplyError::new("example.unparsable", format!("the committed metabolism example must parse: {error:?}")))
}

//#endregion 🔖️ExampleFixture
