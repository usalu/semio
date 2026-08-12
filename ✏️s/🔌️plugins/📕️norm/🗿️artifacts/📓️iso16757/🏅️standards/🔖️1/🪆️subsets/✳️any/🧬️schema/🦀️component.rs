//! 🧬️ Iso16757 artifact schema — every field of the artifact with its state class.


use std::collections::BTreeMap;

use schema::ArtifactSchema;
use crate::artifacts::iso16757::CatalogueValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Iso16757 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Artifact {
    #[state(persistent)] pub catalogue: crate::artifacts::iso16757::part_1::Catalogue,
    #[state(persistent)] pub dictionary: crate::artifacts::iso16757::part_4::Dictionary,
    #[state(persistent)] pub geometry: crate::artifacts::iso16757::part_2::GeometryCatalogue,
    #[state(persistent)] pub selection: crate::artifacts::iso16757::part_1::SelectionRequest,
    #[state(persistent)] pub part_number_rule: crate::artifacts::iso16757::part_5::PartNumberRule,
    #[state(persistent)] pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(persistent)] pub script_limits: crate::artifacts::iso16757::part_5::ScriptLimits,
    #[state(persistent)] pub exchange_process: crate::artifacts::iso16757::part_5::ExchangeProcess,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Iso16757Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::iso16757::Iso16757Snapshot {
        crate::artifacts::iso16757::Iso16757Snapshot {
            catalogue: self.catalogue.clone(),
            dictionary: self.dictionary.clone(),
            geometry: self.geometry.clone(),
            selection: self.selection.clone(),
            part_number_rule: self.part_number_rule.clone(),
            part_number_inputs: self.part_number_inputs.clone(),
            script_limits: self.script_limits.clone(),
            exchange_process: self.exchange_process.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::iso16757::Iso16757Snapshot) -> Self {
        Self {
            catalogue: snapshot.catalogue,
            dictionary: snapshot.dictionary,
            geometry: snapshot.geometry,
            selection: snapshot.selection,
            part_number_rule: snapshot.part_number_rule,
            part_number_inputs: snapshot.part_number_inputs,
            script_limits: snapshot.script_limits,
            exchange_process: snapshot.exchange_process,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::iso16757::Iso16757Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.iso16757` — twenty handcrafted schema leaves.
pub fn iso16757_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.iso16757",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757BuilderConstruction {
        snapshot: Iso16757Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Iso16757BuilderConstruction {
        type Snapshot = Iso16757Snapshot;
        type Mutation = Iso16757Mutation;
        type Diff = Iso16757Diff;
        fn empty() -> Self { Self { snapshot: Iso16757Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::iso16757::Iso16757Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757Parts {
        pub snapshot: Option<Iso16757Snapshot>,
    }

    pub struct Iso16757AnalyzerAnalysis;

    impl ArtifactAnalysis for Iso16757AnalyzerAnalysis {
        type Parts = Iso16757Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Iso16757Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Iso16757BuilderFacets {
        construction: derived_construction::Iso16757BuilderConstruction,
        analysis: derived_analysis::Iso16757AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Iso16757ComposerComposition,
    }
    builder: Iso16757Builder,
    analyzer: Iso16757Analyzer,
    composer: Iso16757Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure ISO 16757 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`. `part_1`, `part_2`, `part_4` and `part_5` are pure
/// function libraries over the artifact's own document types (`Catalogue`, `GeometryNode`,
/// `Dictionary`, …), never over the whole `Iso16757Snapshot`; the snapshot-level composition
/// (`evaluate`) lives in `💡️inferences`, and the JSON serializers live in `🚪️io`.
use crate::document::{ClauseId, NormError};
use std::collections::{HashMap, HashSet};

// #region Part1
pub mod part_1 {
    use super::*;
    use crate::artifacts::iso16757::part_1::*;

    pub fn evaluate_constraint(value: &CatalogueValue, constraint: &SelectionConstraint) -> bool {
        match (&constraint.operator, value, &constraint.value) {
            (ConstraintOperator::Equal, a, b) => a == b,
            (ConstraintOperator::NotEqual, a, b) => a != b,
            (ConstraintOperator::LessThan, CatalogueValue::Decimal { value: a }, CatalogueValue::Decimal { value: b }) => a < b,
            (ConstraintOperator::GreaterThan, CatalogueValue::Decimal { value: a }, CatalogueValue::Decimal { value: b }) => a > b,
            (ConstraintOperator::InRange, CatalogueValue::Decimal { value: v }, CatalogueValue::Range { min, max, .. }) => *v >= *min && *v <= *max,
            _ => false,
        }
    }

    pub fn select_products(catalogue: &Catalogue, request: &SelectionRequest) -> SelectionResult {
        let mut matches = Vec::new();
        let mut explanations = Vec::new();
        for index in &catalogue.product_indexes {
            let Some(product) = catalogue.products.iter().find(|p| p.id == index.product_id) else {
                continue;
            };
            let Some(series) = catalogue.product_series.iter().find(|s| s.id == product.series_id) else {
                continue;
            };
            if series.class_id != request.class_id {
                continue;
            }
            if let Some(series_id) = &request.series_id {
                if product.series_id != *series_id {
                    continue;
                }
            }
            let variant = index.variant_id.as_ref().and_then(|vid| product.variants.iter().find(|v| &v.id == vid));
            let properties: Vec<&PropertyValue> = product.static_properties.iter().chain(variant.into_iter().flat_map(|v| &v.property_values)).collect();
            let mut passes = true;
            for constraint in &request.constraints {
                let Some(pv) = properties.iter().find(|pv| pv.definition_id == constraint.property_id) else {
                    passes = false;
                    explanations.push(format!("missing property {}", constraint.property_id));
                    break;
                };
                if !evaluate_constraint(&pv.value, constraint) {
                    passes = false;
                    explanations.push(format!("constraint failed on {}", constraint.property_id));
                    break;
                }
            }
            if passes {
                matches.push(index.clone());
            }
        }
        let ambiguity = matches.len() > 1;
        SelectionResult { matches, ambiguity, explanations }
    }

    pub fn detect_composition_cycle(catalogue: &Catalogue, root_product_id: &str) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![root_product_id.to_string()];
        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                return true;
            }
            if let Some(parts) = catalogue.compositions.get(&current) {
                for part in parts {
                    stack.push(part.component_product_id.clone());
                }
            }
        }
        false
    }

    pub fn resolve_bim_embedding(catalogue: &Catalogue, index_id: &str, parameters: HashMap<String, CatalogueValue>) -> Result<BimEmbedding, NormError> {
        let index = catalogue.product_indexes.iter().find(|i| i.id == index_id).ok_or_else(|| NormError::InvalidValue { field: "index_id".into(), reason: "unknown product index".into() })?;
        let product = catalogue.products.iter().find(|p| p.id == index.product_id).ok_or_else(|| NormError::InvalidValue { field: "product_id".into(), reason: "unknown product".into() })?;
        let variant = index.variant_id.as_ref().and_then(|vid| product.variants.iter().find(|v| &v.id == vid)).ok_or_else(|| NormError::IncompleteInput { field: "variant_id".into() })?;
        for (param_id, value) in &parameters {
            let domain = product.parameter_domains.iter().find(|d| &d.parameter_id == param_id);
            if let Some(domain) = domain {
                if !domain.allowed_values.is_empty() && !domain.allowed_values.contains(value) {
                    return Err(NormError::InvalidValue { field: param_id.clone(), reason: "not in allowed domain".into() });
                }
            }
        }
        let mut resolved_properties = product.static_properties.clone();
        resolved_properties.extend(variant.property_values.clone());
        Ok(BimEmbedding {
            selected_index_id: index_id.to_string(),
            frozen_parameters: parameters,
            resolved_article_number: variant.article_number.clone(),
            resolved_geometry_id: variant.geometry_id.clone().or_else(|| catalogue.product_series.iter().find(|s| s.id == product.series_id).and_then(|s| s.geometry_id.clone())),
            resolved_properties,
            catalogue_provenance: catalogue.id.clone(),
            dictionary_provenance: catalogue.dictionary.clone(),
        })
    }

    pub fn validate_catalogue_structure(catalogue: &Catalogue) -> Vec<String> {
        let mut issues = Vec::new();
        if catalogue.products.is_empty() {
            issues.push("catalogue has no products".into());
        }
        for product in &catalogue.products {
            if catalogue.product_series.iter().all(|s| s.id != product.series_id) {
                issues.push(format!("product {} references unknown series {}", product.id, product.series_id));
            }
            if detect_composition_cycle(catalogue, &product.id) {
                issues.push(format!("composition cycle at product {}", product.id));
            }
        }
        for def in &catalogue.property_definitions {
            if def.id.is_empty() {
                issues.push("empty property definition id".into());
            }
        }
        issues
    }
}
// #endregion Part1

// #region Part2
pub mod part_2 {
    use super::*;
    use crate::artifacts::iso16757::part_2::*;

    pub fn substitute_parameters(node: &GeometryNode, values: &HashMap<String, f64>) -> GeometryNode {
        match node {
            GeometryNode::Primitive { kind, parameters } => {
                let mut resolved = std::collections::BTreeMap::new();
                for (key, value) in parameters {
                    resolved.insert(key.clone(), *values.get(key).unwrap_or(value));
                }
                GeometryNode::Primitive { kind: kind.clone(), parameters: resolved }
            }
            GeometryNode::Transform { translation, rotation_deg, child } => GeometryNode::Transform { translation: *translation, rotation_deg: *rotation_deg, child: Box::new(substitute_parameters(child, values)) },
            GeometryNode::Boolean { operator, children } => GeometryNode::Boolean { operator: *operator, children: children.iter().map(|c| substitute_parameters(c, values)).collect() },
            GeometryNode::Reference { geometry_id } => GeometryNode::Reference { geometry_id: geometry_id.clone() },
        }
    }

    pub fn evaluate_bounding_box(node: &GeometryNode, catalogue: &GeometryCatalogue) -> Result<BoundingBox, NormError> {
        match node {
            GeometryNode::Primitive { kind, parameters } => match kind.as_str() {
                "box" => {
                    let w = *parameters.get("width").ok_or_else(|| NormError::IncompleteInput { field: "width".into() })?;
                    let h = *parameters.get("height").ok_or_else(|| NormError::IncompleteInput { field: "height".into() })?;
                    let d = *parameters.get("depth").ok_or_else(|| NormError::IncompleteInput { field: "depth".into() })?;
                    Ok(BoundingBox::from_size(w, h, d))
                }
                "cylinder" => {
                    let r = *parameters.get("radius").ok_or_else(|| NormError::IncompleteInput { field: "radius".into() })?;
                    let h = *parameters.get("height").ok_or_else(|| NormError::IncompleteInput { field: "height".into() })?;
                    Ok(BoundingBox::from_size(2.0 * r, h, 2.0 * r))
                }
                "sphere" => {
                    let r = *parameters.get("radius").ok_or_else(|| NormError::IncompleteInput { field: "radius".into() })?;
                    Ok(BoundingBox::from_size(2.0 * r, 2.0 * r, 2.0 * r))
                }
                _ => Err(NormError::OutOfScope { clause: ClauseId::new("ISO 16757", "2", "7.1") }),
            },
            GeometryNode::Transform { translation, child, .. } => {
                let mut bbox = evaluate_bounding_box(child, catalogue)?;
                bbox.min[0] += translation[0];
                bbox.min[1] += translation[1];
                bbox.min[2] += translation[2];
                bbox.max[0] += translation[0];
                bbox.max[1] += translation[1];
                bbox.max[2] += translation[2];
                Ok(bbox)
            }
            GeometryNode::Boolean { children, .. } => {
                let mut iter = children.iter();
                let first = iter.next().ok_or_else(|| NormError::IncompleteInput { field: "boolean_children".into() })?;
                let mut acc = evaluate_bounding_box(first, catalogue)?;
                for child in iter {
                    let next = evaluate_bounding_box(child, catalogue)?;
                    acc.min[0] = acc.min[0].min(next.min[0]);
                    acc.min[1] = acc.min[1].min(next.min[1]);
                    acc.min[2] = acc.min[2].min(next.min[2]);
                    acc.max[0] = acc.max[0].max(next.max[0]);
                    acc.max[1] = acc.max[1].max(next.max[1]);
                    acc.max[2] = acc.max[2].max(next.max[2]);
                }
                Ok(acc)
            }
            GeometryNode::Reference { geometry_id } => {
                catalogue.objects.get(geometry_id).and_then(|obj| obj.shape.as_ref()).map_or_else(|| Err(NormError::InvalidValue { field: "geometry_id".into(), reason: "unresolved reference".into() }), |shape| evaluate_bounding_box(shape, catalogue))
            }
        }
    }

    pub fn validate_geometry_graph(object: &GeometryObject, catalogue: &GeometryCatalogue, visited: &mut HashSet<String>) -> Vec<String> {
        let mut issues = Vec::new();
        if let Some(GeometryNode::Reference { geometry_id }) = &object.shape {
            if geometry_id == &object.id {
                issues.push(format!("self-reference in geometry {}", object.id));
            }
            if !visited.insert(geometry_id.clone()) {
                issues.push(format!("cycle in geometry reference {}", geometry_id));
            } else if let Some(referenced) = catalogue.objects.get(geometry_id) {
                issues.extend(validate_geometry_graph(referenced, catalogue, visited));
            }
            visited.remove(geometry_id);
        }
        for binding in object.parameter_bindings.values() {
            if binding.is_empty() {
                issues.push(format!("empty parameter binding on {}", object.id));
            }
        }
        issues
    }

    pub fn project_step_entity(_object: &GeometryObject, bbox: BoundingBox) -> String {
        format!("#1=IFCCARTESIANPOINT(({:.3},{:.3},{:.3}));\n#2=IFCBOUNDINGBOX(#1,{:.3},{:.3},{:.3});", bbox.min[0], bbox.min[1], bbox.min[2], bbox.max[0] - bbox.min[0], bbox.max[1] - bbox.min[1], bbox.max[2] - bbox.min[2])
    }
}
// #endregion Part2

// #region Part4
pub mod part_4 {
    use super::*;
    use crate::artifacts::iso16757::part_4::*;

    pub fn subtype_closure(dictionary: &Dictionary, subject_id: &str) -> HashSet<String> {
        let mut closure = HashSet::from([subject_id.to_string()]);
        let mut changed = true;
        while changed {
            changed = false;
            for rel in &dictionary.relationships {
                if rel.kind == RelationshipKind::IsSubtypeOf && closure.contains(&rel.source_id) && closure.insert(rel.target_id.clone()) {
                    changed = true;
                }
            }
        }
        closure
    }

    pub fn detect_subtype_cycle(dictionary: &Dictionary) -> bool {
        for subject in &dictionary.subjects {
            let mut visited = HashSet::new();
            let mut stack = vec![subject.id.clone()];
            while let Some(current) = stack.pop() {
                if !visited.insert(current.clone()) {
                    return true;
                }
                for rel in dictionary.relationships.iter().filter(|r| r.kind == RelationshipKind::IsSubtypeOf && r.source_id == current) {
                    stack.push(rel.target_id.clone());
                }
            }
        }
        false
    }

    pub fn filter_controlled_values(list: &ControlledValueList, subject_id: &str, dictionary: &Dictionary) -> Vec<String> {
        if list.context_subject_ids.is_empty() {
            return list.values.clone();
        }
        let closure = subtype_closure(dictionary, subject_id);
        if list.context_subject_ids.iter().any(|ctx| closure.contains(ctx)) {
            list.values.clone()
        } else {
            Vec::new()
        }
    }

    pub fn resolve_property<'a>(dictionary: &'a Dictionary, property_id: &str) -> Option<&'a DictionaryProperty> {
        dictionary.properties.iter().find(|p| p.id == property_id)
    }

    pub fn validate_dictionary(dictionary: &Dictionary) -> Vec<String> {
        let mut issues = Vec::new();
        if detect_subtype_cycle(dictionary) {
            issues.push("subtype cycle detected".into());
        }
        for rel in &dictionary.relationships {
            if (rel.kind == RelationshipKind::HasPart || rel.kind == RelationshipKind::HasBlock) && !rel.cardinality.satisfies(1) && rel.cardinality.min > 0 {
                issues.push(format!("relationship {} requires cardinality review", rel.id));
            }
            let source_exists = dictionary.subjects.iter().any(|s| s.id == rel.source_id);
            let target_exists = dictionary.subjects.iter().any(|s| s.id == rel.target_id);
            if !source_exists || !target_exists {
                issues.push(format!("relationship {} has dangling endpoints", rel.id));
            }
        }
        issues
    }

    pub fn to_iso12006_mappings(dictionary: &Dictionary) -> Vec<Iso12006Mapping> {
        dictionary.subjects.iter().map(|subject| Iso12006Mapping { dictionary_object_id: subject.id.clone(), iso12006_uri: format!("iso12006://subject/{}", subject.id), object_kind: format!("{:?}", subject.kind) }).collect()
    }
}
// #endregion Part4

// #region Part5
pub mod part_5 {
    use super::*;
    use crate::artifacts::iso16757::part_5::*;
    use std::time::{Duration, Instant};

    /// 🧮️ Sandboxed calculation runtime (constrained numeric expressions).
    pub trait ScriptRuntime {
        fn execute(&self, source: &str, inputs: &HashMap<String, f64>, limits: ScriptLimits) -> Result<ScriptResult, ScriptError>;
    }

    /// 🔢️ Default deterministic script runtime.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultScriptRuntime;

    impl ScriptRuntime for DefaultScriptRuntime {
        fn execute(&self, source: &str, inputs: &HashMap<String, f64>, limits: ScriptLimits) -> Result<ScriptResult, ScriptError> {
            let started = Instant::now();
            let trimmed = source.trim();
            if trimmed.contains("import") || trimmed.contains("require") || trimmed.contains("fetch") || trimmed.contains("fs") {
                return Err(ScriptError::InvalidExpression("forbidden construct".into()));
            }
            let result = eval_expression(trimmed, inputs, limits.max_steps, limits.max_recursion, 0, &started, limits.timeout_ms)?;
            Ok(ScriptResult { value: result, diagnostics: Vec::new() })
        }
    }

    fn eval_expression(expr: &str, inputs: &HashMap<String, f64>, steps: u32, max_recursion: u32, depth: u32, started: &Instant, timeout_ms: u64) -> Result<f64, ScriptError> {
        if started.elapsed() > Duration::from_millis(timeout_ms) {
            return Err(ScriptError::Timeout(timeout_ms));
        }
        if depth > max_recursion {
            return Err(ScriptError::RecursionLimit(max_recursion));
        }
        if steps == 0 {
            return Err(ScriptError::StepLimit(0));
        }
        let expr = expr.trim();
        if let Ok(value) = expr.parse::<f64>() {
            return Ok(value);
        }
        if let Some(value) = inputs.get(expr) {
            return Ok(*value);
        }
        if let Some(inner) = expr.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            return eval_expression(inner, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms);
        }
        for operation in ["+", "-", "*", "/"] {
            if let Some((left, right)) = split_binary(expr, operation) {
                let l = eval_expression(left, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms)?;
                let r = eval_expression(right, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms)?;
                return Ok(match operation {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => {
                        if r.abs() < f64::EPSILON {
                            return Err(ScriptError::InvalidExpression("division by zero".into()));
                        }
                        l / r
                    }
                    _ => unreachable!(),
                });
            }
        }
        Err(ScriptError::InvalidExpression(expr.into()))
    }

    fn split_binary<'a>(expr: &'a str, operation: &str) -> Option<(&'a str, &'a str)> {
        let mut depth = 0i32;
        for (idx, ch) in expr.char_indices().rev() {
            if ch == ')' {
                depth += 1;
            } else if ch == '(' {
                depth -= 1;
            } else if depth == 0 && expr[idx..].starts_with(operation) {
                let left = expr[..idx].trim();
                let right = expr[idx + operation.len()..].trim();
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, right));
                }
            }
        }
        None
    }

    fn catalogue_value_as_str(value: &CatalogueValue) -> Option<String> {
        match value {
            CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } | CatalogueValue::Identifier { value } => Some(value.clone()),
            CatalogueValue::Decimal { value } => Some(value.to_string()),
            CatalogueValue::Integer { value } => Some(value.to_string()),
            _ => None,
        }
    }

    pub fn calculate_part_number(rule: &PartNumberRule, inputs: &std::collections::BTreeMap<String, CatalogueValue>, runtime: &dyn ScriptRuntime) -> Result<String, NormError> {
        match rule {
            PartNumberRule::Literal { value } => Ok(value.clone()),
            PartNumberRule::Table { rows, output_column } => {
                for row in rows {
                    let mut matches = true;
                    for (key, expected) in row.iter().filter(|(k, _)| *k != output_column) {
                        let actual = inputs.get(key).and_then(catalogue_value_as_str);
                        if actual.as_deref() != Some(expected.as_str()) {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        return row.get(output_column).cloned().ok_or_else(|| NormError::IncompleteInput { field: output_column.clone() });
                    }
                }
                Err(NormError::InvalidValue { field: "part_number_table".into(), reason: "no matching row".into() })
            }
            PartNumberRule::Script { source, .. } => {
                let numeric: HashMap<String, f64> = inputs
                    .iter()
                    .filter_map(|(k, v)| match v {
                        CatalogueValue::Decimal { value } => Some((k.clone(), *value)),
                        CatalogueValue::Integer { value } => Some((k.clone(), *value as f64)),
                        _ => None,
                    })
                    .collect();
                let result = runtime.execute(source, &numeric, ScriptLimits::default()).map_err(|e| NormError::InvalidValue { field: "part_number_script".into(), reason: e.to_string() })?;
                Ok(format!("{:.0}", result.value))
            }
        }
    }

    pub fn build_ifc_catalogue(catalogue: &crate::artifacts::iso16757::part_1::Catalogue) -> IfcCatalogue {
        let metadata = IfcCatalogueNode {
            entity_type: "IfcBuildingServicesCatalogue".into(),
            global_id: catalogue.id.0.clone(),
            name: catalogue.metadata.names.preferred.text.clone(),
            attributes: HashMap::from([("dictionaryId".into(), catalogue.dictionary.id.clone()), ("dictionaryVersion".into(), catalogue.dictionary.version.clone())]),
            children: Vec::new(),
        };
        let product_classes = catalogue
            .product_classes
            .iter()
            .map(|class| IfcCatalogueNode { entity_type: "IfcProductClass".into(), global_id: class.id.clone(), name: class.names.preferred.text.clone(), attributes: HashMap::from([("groupId".into(), class.group_id.clone())]), children: Vec::new() })
            .collect();
        let products = catalogue
            .products
            .iter()
            .map(|product| IfcCatalogueNode {
                entity_type: "IfcProduct".into(),
                global_id: product.id.clone(),
                name: product.names.preferred.text.clone(),
                attributes: HashMap::from([("seriesId".into(), product.series_id.clone())]),
                children: Vec::new(),
            })
            .collect();
        IfcCatalogue { schema: "IFC4".into(), metadata, product_classes, products, unknown_entities: Vec::new() }
    }

    pub fn export_ifc_step(catalogue: &IfcCatalogue) -> String {
        let mut lines = vec![format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('ISO 16757-5'),'2;1');\nFILE_NAME('{}.ifc','',(''),(''),'norm_iso_16757','','');\nENDSEC;\nDATA;", catalogue.metadata.global_id)];
        let mut id = 1u32;
        let write_node = |node: &IfcCatalogueNode, id: &mut u32| -> String {
            let current = *id;
            *id += 1;
            format!("#{}={}('{}','{}','{}',({}));", current, node.entity_type, node.global_id, node.name, node.name, node.attributes.values().map(|v| format!("'{v}'")).collect::<Vec<_>>().join(","))
        };
        lines.push(write_node(&catalogue.metadata, &mut id));
        for class in &catalogue.product_classes {
            lines.push(write_node(class, &mut id));
        }
        for product in &catalogue.products {
            lines.push(write_node(product, &mut id));
        }
        lines.push("ENDSEC;\nEND-ISO-10303-21;".into());
        lines.join("\n")
    }

    pub fn validate_exchange(catalogue: &crate::artifacts::iso16757::part_1::Catalogue, ifc: &IfcCatalogue) -> Vec<String> {
        let mut issues = Vec::new();
        if ifc.schema.is_empty() {
            issues.push("missing IFC schema declaration".into());
        }
        if ifc.products.len() != catalogue.products.len() {
            issues.push(format!("IFC product count {} != catalogue {}", ifc.products.len(), catalogue.products.len()));
        }
        issues
    }
}
// #endregion Part5

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;
    use crate::artifacts::iso16757::Iso16757Snapshot;
    use crate::artifacts::iso16757::standards::v1::subsets::any::schema::component::part_5::ScriptRuntime;
    use std::collections::BTreeMap;

    #[test]
    fn reference_fixture_selects_one_product() {
        let doc = Iso16757Snapshot::default();
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert_eq!(selection.matches.len(), 1);
        assert!(!selection.ambiguity);
    }

    #[test]
    fn geometry_bbox_volume_for_box_primitive() {
        let doc = Iso16757Snapshot::default();
        let geom = doc.geometry.objects.get("geom.valve.50").expect("geometry");
        let bbox = part_2::evaluate_bounding_box(geom.shape.as_ref().expect("shape"), &doc.geometry).expect("bbox");
        assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
    }

    #[test]
    fn dictionary_controlled_values_filter_by_subject() {
        let doc = Iso16757Snapshot::default();
        let list = doc.dictionary.controlled_lists.first().expect("list");
        let allowed = part_4::filter_controlled_values(list, "subject.valve", &doc.dictionary);
        assert_eq!(allowed, vec!["50", "80", "100"]);
    }

    #[test]
    fn part_number_script_is_deterministic() {
        let runtime = part_5::DefaultScriptRuntime;
        let rule = crate::artifacts::iso16757::part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() };
        let inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]);
        let part_no = part_5::calculate_part_number(&rule, &inputs, &runtime).expect("part number");
        assert_eq!(part_no, "550");
    }

    #[test]
    fn ifc_step_export_contains_data_section() {
        let doc = Iso16757Snapshot::default();
        let ifc = part_5::build_ifc_catalogue(&doc.catalogue);
        let step = part_5::export_ifc_step(&ifc);
        assert!(step.contains("ENDSEC"));
        assert!(step.contains("IFCPRODUCT") || step.contains("IfcProduct"));
    }

    #[test]
    fn composition_cycle_detected() {
        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.compositions.insert("product.a".into(), vec![crate::artifacts::iso16757::part_1::CompositionRelationship { component_product_id: "product.b".into(), quantity: 1 }]);
        doc.catalogue.compositions.insert("product.b".into(), vec![crate::artifacts::iso16757::part_1::CompositionRelationship { component_product_id: "product.a".into(), quantity: 1 }]);
        assert!(part_1::detect_composition_cycle(&doc.catalogue, "product.a"));
    }

    #[test]
    fn script_rejects_forbidden_import() {
        let runtime = part_5::DefaultScriptRuntime;
        let err = runtime.execute("import fs", &HashMap::new(), crate::artifacts::iso16757::part_5::ScriptLimits::default()).unwrap_err();
        assert!(matches!(err, crate::artifacts::iso16757::part_5::ScriptError::InvalidExpression(_)));
    }

    #[test]
    fn evaluate_constraint_operators() {
        let dec = |v: f64| CatalogueValue::Decimal { value: v };
        let mk = |op, value| crate::artifacts::iso16757::part_1::SelectionConstraint { property_id: "p".into(), operator: op, value };
        assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::NotEqual, dec(6.0))));
        assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::NotEqual, dec(5.0))));
        assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::LessThan, dec(6.0))));
        assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::LessThan, dec(4.0))));
        assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::GreaterThan, dec(4.0))));
        assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::GreaterThan, dec(6.0))));
        let range = CatalogueValue::Range { min: 1.0, max: 10.0, unit: None };
        assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::InRange, range.clone())));
        assert!(!part_1::evaluate_constraint(&dec(50.0), &mk(crate::artifacts::iso16757::part_1::ConstraintOperator::InRange, range)));
    }

    #[test]
    fn evaluate_constraint_type_mismatch_returns_false() {
        let constraint = crate::artifacts::iso16757::part_1::SelectionConstraint { property_id: "p".into(), operator: crate::artifacts::iso16757::part_1::ConstraintOperator::LessThan, value: CatalogueValue::Text { value: "x".into() } };
        assert!(!part_1::evaluate_constraint(&CatalogueValue::Decimal { value: 1.0 }, &constraint));
    }

    #[test]
    fn select_products_filters_by_series_id() {
        let mut doc = Iso16757Snapshot::default();
        let other_series = crate::artifacts::iso16757::part_1::ProductSeries { id: "series.other".into(), class_id: "class.valve".into(), names: doc.catalogue.product_series[0].names.clone(), shared_property_values: BTreeMap::new(), geometry_id: None };
        let other_product = crate::artifacts::iso16757::part_1::Product {
            id: "product.other".into(),
            series_id: "series.other".into(),
            names: other_series.names.clone(),
            parameter_domains: Vec::new(),
            variants: vec![crate::artifacts::iso16757::part_1::ProductVariant {
                id: "variant.other".into(),
                parameter_values: BTreeMap::new(),
                property_values: vec![crate::artifacts::iso16757::part_1::PropertyValue { definition_id: "prop.dn".into(), value: CatalogueValue::Decimal { value: 50.0 }, function_id: None }],
                article_number: None,
                geometry_id: None,
            }],
            static_properties: Vec::new(),
        };
        doc.catalogue.product_series.push(other_series);
        doc.catalogue.products.push(other_product);
        doc.catalogue.product_indexes.push(crate::artifacts::iso16757::part_1::ProductIndex { id: "index.other".into(), product_id: "product.other".into(), variant_id: Some("variant.other".into()), search_tags: Vec::new() });
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert_eq!(selection.matches.len(), 1);
        assert_eq!(selection.matches[0].id, "index.cv50");
    }

    #[test]
    fn select_products_records_missing_property_and_constraint_failures() {
        let mut doc = Iso16757Snapshot::default();
        doc.selection.constraints.push(crate::artifacts::iso16757::part_1::SelectionConstraint { property_id: "prop.missing".into(), operator: crate::artifacts::iso16757::part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 1.0 } });
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert!(selection.matches.is_empty());
        assert!(selection.explanations.iter().any(|e| e.contains("missing property")));

        doc.selection.constraints.clear();
        doc.selection.constraints.push(crate::artifacts::iso16757::part_1::SelectionConstraint { property_id: "prop.dn".into(), operator: crate::artifacts::iso16757::part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 999.0 } });
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert!(selection.matches.is_empty());
        assert!(selection.explanations.iter().any(|e| e.contains("constraint failed")));
    }

    #[test]
    fn select_products_flags_ambiguity_with_multiple_matches() {
        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.product_indexes.push(crate::artifacts::iso16757::part_1::ProductIndex { id: "index.cv50.dup".into(), product_id: "product.cv".into(), variant_id: Some("variant.50".into()), search_tags: Vec::new() });
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert_eq!(selection.matches.len(), 2);
        assert!(selection.ambiguity);
    }

    #[test]
    fn resolve_bim_embedding_error_paths() {
        let doc = Iso16757Snapshot::default();
        let unknown_index = part_1::resolve_bim_embedding(&doc.catalogue, "index.unknown", HashMap::new());
        assert!(matches!(unknown_index, Err(NormError::InvalidValue { field, .. }) if field == "index_id"));

        let mut catalogue_no_product = doc.catalogue.clone();
        catalogue_no_product.product_indexes[0].product_id = "product.unknown".into();
        let unknown_product = part_1::resolve_bim_embedding(&catalogue_no_product, "index.cv50", HashMap::new());
        assert!(matches!(unknown_product, Err(NormError::InvalidValue { field, .. }) if field == "product_id"));

        let mut catalogue_no_variant = doc.catalogue.clone();
        catalogue_no_variant.product_indexes[0].variant_id = None;
        let missing_variant = part_1::resolve_bim_embedding(&catalogue_no_variant, "index.cv50", HashMap::new());
        assert!(matches!(missing_variant, Err(NormError::IncompleteInput { field }) if field == "variant_id"));

        let out_of_domain = part_1::resolve_bim_embedding(&doc.catalogue, "index.cv50", HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 12345.0 })]));
        assert!(matches!(out_of_domain, Err(NormError::InvalidValue { field, .. }) if field == "dn"));
    }

    #[test]
    fn resolve_bim_embedding_falls_back_to_series_geometry() {
        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.products[0].variants[0].geometry_id = None;
        let embedding = part_1::resolve_bim_embedding(&doc.catalogue, "index.cv50", HashMap::new()).expect("embedding");
        assert_eq!(embedding.resolved_geometry_id, doc.catalogue.product_series[0].geometry_id);
    }

    #[test]
    fn validate_catalogue_structure_flags_issues() {
        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.products.clear();
        assert!(part_1::validate_catalogue_structure(&doc.catalogue).iter().any(|i| i.contains("no products")));

        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.products[0].series_id = "series.unknown".into();
        let issues = part_1::validate_catalogue_structure(&doc.catalogue);
        assert!(issues.iter().any(|i| i.contains("references unknown series")));

        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.property_definitions[0].id = String::new();
        let issues = part_1::validate_catalogue_structure(&doc.catalogue);
        assert!(issues.iter().any(|i| i.contains("empty property definition id")));

        let mut doc = Iso16757Snapshot::default();
        doc.catalogue.compositions.insert("product.cv".into(), vec![crate::artifacts::iso16757::part_1::CompositionRelationship { component_product_id: "product.cv".into(), quantity: 1 }]);
        let issues = part_1::validate_catalogue_structure(&doc.catalogue);
        assert!(issues.iter().any(|i| i.contains("composition cycle")));
    }

    #[test]
    fn substitute_parameters_recurses_through_node_kinds() {
        let primitive = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0)]) };
        let transform = crate::artifacts::iso16757::part_2::GeometryNode::Transform { translation: [1.0, 0.0, 0.0], rotation_deg: [0.0, 0.0, 0.0], child: Box::new(primitive.clone()) };
        let boolean = crate::artifacts::iso16757::part_2::GeometryNode::Boolean { operator: crate::artifacts::iso16757::part_2::BooleanOperator::Union, children: vec![primitive.clone(), transform.clone()] };
        let reference = crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "geom.x".into() };

        let values = HashMap::from([("width".into(), 2.0)]);
        match part_2::substitute_parameters(&primitive, &values) {
            crate::artifacts::iso16757::part_2::GeometryNode::Primitive { parameters, .. } => assert_eq!(parameters["width"], 2.0),
            _ => panic!("expected primitive"),
        }
        match part_2::substitute_parameters(&transform, &values) {
            crate::artifacts::iso16757::part_2::GeometryNode::Transform { child, .. } => match *child {
                crate::artifacts::iso16757::part_2::GeometryNode::Primitive { parameters, .. } => assert_eq!(parameters["width"], 2.0),
                _ => panic!("expected primitive child"),
            },
            _ => panic!("expected transform"),
        }
        match part_2::substitute_parameters(&boolean, &values) {
            crate::artifacts::iso16757::part_2::GeometryNode::Boolean { children, .. } => assert_eq!(children.len(), 2),
            _ => panic!("expected boolean"),
        }
        match part_2::substitute_parameters(&reference, &values) {
            crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id } => assert_eq!(geometry_id, "geom.x"),
            _ => panic!("expected reference"),
        }
    }

    #[test]
    fn evaluate_bounding_box_error_paths() {
        let catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue::default();
        let missing_width = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::new() };
        assert!(matches!(part_2::evaluate_bounding_box(&missing_width, &catalogue), Err(NormError::IncompleteInput { field }) if field == "width"));

        let unknown_kind = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "cone".into(), parameters: BTreeMap::new() };
        assert!(matches!(part_2::evaluate_bounding_box(&unknown_kind, &catalogue), Err(NormError::OutOfScope { .. })));

        let empty_boolean = crate::artifacts::iso16757::part_2::GeometryNode::Boolean { operator: crate::artifacts::iso16757::part_2::BooleanOperator::Union, children: Vec::new() };
        assert!(matches!(part_2::evaluate_bounding_box(&empty_boolean, &catalogue), Err(NormError::IncompleteInput { field }) if field == "boolean_children"));

        let unresolved_ref = crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "missing".into() };
        assert!(matches!(part_2::evaluate_bounding_box(&unresolved_ref, &catalogue), Err(NormError::InvalidValue { field, .. }) if field == "geometry_id"));
    }

    #[test]
    fn evaluate_bounding_box_cylinder_sphere_boolean_transform() {
        let catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue::default();
        let cylinder = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "cylinder".into(), parameters: BTreeMap::from([("radius".into(), 1.0), ("height".into(), 2.0)]) };
        let bbox = part_2::evaluate_bounding_box(&cylinder, &catalogue).expect("cylinder bbox");
        assert_eq!(bbox.max, [2.0, 2.0, 2.0]);

        let sphere = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "sphere".into(), parameters: BTreeMap::from([("radius".into(), 1.5)]) };
        let bbox = part_2::evaluate_bounding_box(&sphere, &catalogue).expect("sphere bbox");
        assert_eq!(bbox.max, [3.0, 3.0, 3.0]);

        let a = crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) };
        let b = crate::artifacts::iso16757::part_2::GeometryNode::Transform {
            translation: [5.0, 5.0, 5.0],
            rotation_deg: [0.0, 0.0, 0.0],
            child: Box::new(crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) }),
        };
        let boolean = crate::artifacts::iso16757::part_2::GeometryNode::Boolean { operator: crate::artifacts::iso16757::part_2::BooleanOperator::Union, children: vec![a, b] };
        let bbox = part_2::evaluate_bounding_box(&boolean, &catalogue).expect("boolean bbox");
        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [6.0, 6.0, 6.0]);

        let mut objects = BTreeMap::new();
        objects.insert(
            "geom.ref".to_string(),
            crate::artifacts::iso16757::part_2::GeometryObject {
                id: "geom.ref".into(),
                shape: Some(crate::artifacts::iso16757::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) }),
                symbolic: None,
                spaces: Vec::new(),
                surfaces: Vec::new(),
                ports: Vec::new(),
                parameter_bindings: BTreeMap::new(),
            },
        );
        let ref_catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
        let reference = crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "geom.ref".into() };
        let bbox = part_2::evaluate_bounding_box(&reference, &ref_catalogue).expect("resolved reference bbox");
        assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn validate_geometry_graph_self_reference_and_cycle() {
        let mut objects = BTreeMap::new();
        let self_ref = crate::artifacts::iso16757::part_2::GeometryObject {
            id: "geom.self".into(),
            shape: Some(crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "geom.self".into() }),
            symbolic: None,
            spaces: Vec::new(),
            surfaces: Vec::new(),
            ports: Vec::new(),
            parameter_bindings: BTreeMap::new(),
        };
        objects.insert("geom.self".to_string(), self_ref.clone());
        let catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
        let mut visited = HashSet::new();
        let issues = part_2::validate_geometry_graph(&self_ref, &catalogue, &mut visited);
        assert!(issues.iter().any(|i| i.contains("self-reference")));
        assert!(visited.is_empty());

        let mut objects = BTreeMap::new();
        let a = crate::artifacts::iso16757::part_2::GeometryObject {
            id: "geom.a".into(),
            shape: Some(crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "geom.b".into() }),
            symbolic: None,
            spaces: Vec::new(),
            surfaces: Vec::new(),
            ports: Vec::new(),
            parameter_bindings: BTreeMap::new(),
        };
        let b = crate::artifacts::iso16757::part_2::GeometryObject {
            id: "geom.b".into(),
            shape: Some(crate::artifacts::iso16757::part_2::GeometryNode::Reference { geometry_id: "geom.a".into() }),
            symbolic: None,
            spaces: Vec::new(),
            surfaces: Vec::new(),
            ports: Vec::new(),
            parameter_bindings: BTreeMap::new(),
        };
        objects.insert("geom.a".to_string(), a.clone());
        objects.insert("geom.b".to_string(), b);
        let catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
        let mut visited = HashSet::new();
        let issues = part_2::validate_geometry_graph(&a, &catalogue, &mut visited);
        assert!(issues.iter().any(|i| i.contains("cycle in geometry reference")));
    }

    #[test]
    fn validate_geometry_graph_empty_parameter_binding() {
        let object = crate::artifacts::iso16757::part_2::GeometryObject { id: "geom.bind".into(), shape: None, symbolic: None, spaces: Vec::new(), surfaces: Vec::new(), ports: Vec::new(), parameter_bindings: BTreeMap::from([("width".into(), String::new())]) };
        let catalogue = crate::artifacts::iso16757::part_2::GeometryCatalogue::default();
        let mut visited = HashSet::new();
        let issues = part_2::validate_geometry_graph(&object, &catalogue, &mut visited);
        assert!(issues.iter().any(|i| i.contains("empty parameter binding")));
    }

    #[test]
    fn subtype_closure_is_transitive() {
        let dictionary = crate::artifacts::iso16757::part_4::Dictionary {
            reference: crate::artifacts::iso16757::DictionaryRef { id: "d".into(), version: "1".into() },
            subjects: Vec::new(),
            relationships: vec![
                crate::artifacts::iso16757::part_4::Relationship { id: "r1".into(), kind: crate::artifacts::iso16757::part_4::RelationshipKind::IsSubtypeOf, source_id: "a".into(), target_id: "b".into(), cardinality: crate::artifacts::iso16757::Cardinality::optional() },
                crate::artifacts::iso16757::part_4::Relationship { id: "r2".into(), kind: crate::artifacts::iso16757::part_4::RelationshipKind::IsSubtypeOf, source_id: "b".into(), target_id: "c".into(), cardinality: crate::artifacts::iso16757::Cardinality::optional() },
            ],
            properties: Vec::new(),
            controlled_lists: Vec::new(),
            meta_subjects: Vec::new(),
        };
        let closure = part_4::subtype_closure(&dictionary, "a");
        assert!(closure.contains("a") && closure.contains("b") && closure.contains("c"));
    }

    #[test]
    fn detect_subtype_cycle_true() {
        let subject = |id: &str| crate::artifacts::iso16757::part_4::Subject {
            id: id.into(),
            kind: crate::artifacts::iso16757::part_4::SubjectKind::ProductClass,
            names: crate::artifacts::iso16757::Names { preferred: crate::artifacts::iso16757::LocalizedText { locale: "en".into(), text: id.into() }, short_name: None, alternatives: Vec::new() },
            definition: crate::artifacts::iso16757::LocalizedText { locale: "en".into(), text: String::new() },
            parent_id: None,
        };
        let dictionary = crate::artifacts::iso16757::part_4::Dictionary {
            reference: crate::artifacts::iso16757::DictionaryRef { id: "d".into(), version: "1".into() },
            subjects: vec![subject("a"), subject("b")],
            relationships: vec![
                crate::artifacts::iso16757::part_4::Relationship { id: "r1".into(), kind: crate::artifacts::iso16757::part_4::RelationshipKind::IsSubtypeOf, source_id: "a".into(), target_id: "b".into(), cardinality: crate::artifacts::iso16757::Cardinality::optional() },
                crate::artifacts::iso16757::part_4::Relationship { id: "r2".into(), kind: crate::artifacts::iso16757::part_4::RelationshipKind::IsSubtypeOf, source_id: "b".into(), target_id: "a".into(), cardinality: crate::artifacts::iso16757::Cardinality::optional() },
            ],
            properties: Vec::new(),
            controlled_lists: Vec::new(),
            meta_subjects: Vec::new(),
        };
        assert!(part_4::detect_subtype_cycle(&dictionary));
    }

    #[test]
    fn resolve_property_found_and_missing() {
        let doc = Iso16757Snapshot::default();
        assert!(part_4::resolve_property(&doc.dictionary, "prop.dn").is_some());
        assert!(part_4::resolve_property(&doc.dictionary, "prop.unknown").is_none());
    }

    #[test]
    fn validate_dictionary_flags_dangling_and_cardinality_review() {
        let mut doc = Iso16757Snapshot::default();
        doc.dictionary.relationships.push(crate::artifacts::iso16757::part_4::Relationship {
            id: "r.dangling".into(),
            kind: crate::artifacts::iso16757::part_4::RelationshipKind::IsDependentOn,
            source_id: "subject.valve".into(),
            target_id: "subject.unknown".into(),
            cardinality: crate::artifacts::iso16757::Cardinality::optional(),
        });
        doc.dictionary.relationships.push(crate::artifacts::iso16757::part_4::Relationship {
            id: "r.cardinality".into(),
            kind: crate::artifacts::iso16757::part_4::RelationshipKind::HasPart,
            source_id: "subject.valve".into(),
            target_id: "subject.valve".into(),
            cardinality: crate::artifacts::iso16757::Cardinality { min: 2, max: Some(3) },
        });
        let issues = part_4::validate_dictionary(&doc.dictionary);
        assert!(issues.iter().any(|i| i.contains("dangling endpoints")));
        assert!(issues.iter().any(|i| i.contains("cardinality review")));
    }

    #[test]
    fn filter_controlled_values_context_rules() {
        let doc = Iso16757Snapshot::default();
        let mut empty_context_list = doc.dictionary.controlled_lists[0].clone();
        empty_context_list.context_subject_ids.clear();
        assert_eq!(part_4::filter_controlled_values(&empty_context_list, "anything", &doc.dictionary), empty_context_list.values);

        let mut unrelated_list = doc.dictionary.controlled_lists[0].clone();
        unrelated_list.context_subject_ids = vec!["subject.other".into()];
        assert!(part_4::filter_controlled_values(&unrelated_list, "subject.valve", &doc.dictionary).is_empty());
    }

    #[test]
    fn to_iso12006_mappings_basic() {
        let doc = Iso16757Snapshot::default();
        let mappings = part_4::to_iso12006_mappings(&doc.dictionary);
        assert_eq!(mappings.len(), doc.dictionary.subjects.len());
        assert_eq!(mappings[0].iso12006_uri, "iso12006://subject/subject.valve");
        assert_eq!(mappings[0].object_kind, "ProductClass");
    }

    #[test]
    fn calculate_part_number_table_rule_paths() {
        let runtime = part_5::DefaultScriptRuntime;
        let rows = vec![BTreeMap::from([("dn".to_string(), "50".to_string()), ("code".to_string(), "CV50".to_string())])];
        let rule = crate::artifacts::iso16757::part_5::PartNumberRule::Table { rows, output_column: "code".into() };
        let inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]);
        assert_eq!(part_5::calculate_part_number(&rule, &inputs, &runtime).expect("match"), "CV50");

        let no_match_inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 999.0 })]);
        let err = part_5::calculate_part_number(&rule, &no_match_inputs, &runtime).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "part_number_table"));

        let missing_output_rule = crate::artifacts::iso16757::part_5::PartNumberRule::Table { rows: vec![BTreeMap::from([("dn".to_string(), "50".to_string())])], output_column: "code".into() };
        let err = part_5::calculate_part_number(&missing_output_rule, &inputs, &runtime).unwrap_err();
        assert!(matches!(err, NormError::IncompleteInput { field } if field == "code"));

        let literal_rule = crate::artifacts::iso16757::part_5::PartNumberRule::Literal { value: "LIT-1".into() };
        assert_eq!(part_5::calculate_part_number(&literal_rule, &inputs, &runtime).expect("literal"), "LIT-1");
    }

    #[test]
    fn script_runtime_timeout() {
        let runtime = part_5::DefaultScriptRuntime;
        let limits = crate::artifacts::iso16757::part_5::ScriptLimits { max_steps: 100, max_recursion: 10, timeout_ms: 0 };
        let err = runtime.execute("1 + 1", &HashMap::new(), limits).unwrap_err();
        assert!(matches!(err, crate::artifacts::iso16757::part_5::ScriptError::Timeout(0)));
    }

    #[test]
    fn script_runtime_limit_errors() {
        let runtime = part_5::DefaultScriptRuntime;
        let recursion_limits = crate::artifacts::iso16757::part_5::ScriptLimits { max_steps: 1000, max_recursion: 2, timeout_ms: 5_000 };
        let err = runtime.execute("(((1)))", &HashMap::new(), recursion_limits).unwrap_err();
        assert!(matches!(err, crate::artifacts::iso16757::part_5::ScriptError::RecursionLimit(2)));

        let step_limits = crate::artifacts::iso16757::part_5::ScriptLimits { max_steps: 1, max_recursion: 100, timeout_ms: 5_000 };
        let err = runtime.execute("1+1", &HashMap::new(), step_limits).unwrap_err();
        assert!(matches!(err, crate::artifacts::iso16757::part_5::ScriptError::StepLimit(0)));

        let err = runtime.execute("unknownVar", &HashMap::new(), crate::artifacts::iso16757::part_5::ScriptLimits::default()).unwrap_err();
        assert!(matches!(err, crate::artifacts::iso16757::part_5::ScriptError::InvalidExpression(ref e) if e == "unknownVar"));
    }

    #[test]
    fn script_runtime_arithmetic_operators() {
        let runtime = part_5::DefaultScriptRuntime;
        let inputs = HashMap::new();
        let result = runtime.execute("(10 - 4) / 2 * 3", &inputs, crate::artifacts::iso16757::part_5::ScriptLimits::default()).expect("arithmetic");
        assert!((result.value - 9.0).abs() < 1e-9);
    }
}
//#endregion 🧪️ComplianceHelpersTests

