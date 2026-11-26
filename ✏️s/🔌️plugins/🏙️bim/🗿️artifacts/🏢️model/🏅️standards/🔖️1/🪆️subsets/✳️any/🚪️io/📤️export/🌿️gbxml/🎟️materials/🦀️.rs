//! 🧱️ Publishes each distinct material thickness and the original construction-layer references.

use super::{ModelInference, ModelSnapshot, OriginalConstructions, holder_of, layers_of};
use super::original_surfaces::surface;
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer;

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub struct MaterialSource { pub construction: usize, pub layer: usize }

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct Range { first: usize, count: usize }

pub(super) fn layers<'a>(model: &'a ModelSnapshot, inferred: &ModelInference, input: &OriginalConstructions, construction: usize) -> &'a [Layer] {
    input.source(construction).and_then(|source| surface(model, inferred, source))
        .and_then(|side| holder_of(model, side).and_then(|holder| layers_of(model, side, holder)))
        .map_or(&[], |(_, layers)| layers)
}

fn key(layer: &Layer) -> (&str, i64) {
    (layer.material.as_str(), (layer.thickness * 1e6).round() as i64)
}

/// 📇️ The original layer ranges, shared material ordinals and lexically ordered material view.
#[derive(Default)]
pub struct OriginalMaterials {
    source: Option<(usize, usize, usize)>,
    ranges: InferenceValues<Range>,
    references: InferenceValues<usize>,
    sources: InferenceValues<MaterialSource>,
    order: InferenceValues<usize>,
    extent: usize,
    count: usize,
    count_cursor: usize,
    construction: usize,
    layer: usize,
    compare: usize,
    pending_range: Option<Range>,
    pending_source: Option<MaterialSource>,
    pending_reference: Option<usize>,
    sorted: usize,
    scan: usize,
    after: Option<usize>,
    selected: Option<usize>,
    pending_order: Option<usize>,
}

impl OriginalMaterials {
    /// 🛠️ Advances one original range, material comparison, reference or sorted ordinal.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, input: &OriginalConstructions, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML material planning was cancelled").into()); }
        let source = (std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize, std::ptr::from_ref(input) as usize);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(source); }
            return Ok(false);
        }
        if self.source != Some(source) { return Err(ValueError::invariant("gbXML material source changed").into()); }
        let constructions = input.len();
        if !self.ranges.admit_source_step(cx)? || !self.ranges.initialize_step(constructions, cx)? { return Ok(false); }
        if self.count_cursor < constructions {
            if self.pending_range.is_some() {
                if self.ranges.publish_step(self.count_cursor, &mut self.pending_range, cx)? { self.count_cursor += 1; }
            } else if cx.charge_work_unit() {
                let count = layers(model, inferred, input, self.count_cursor).len();
                self.pending_range = Some(Range { first: self.extent, count });
                self.extent = self.extent.checked_add(count).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "gbXML material ordinal overflow"))?;
            }
            return Ok(false);
        }
        if !self.sources.admit_source_step(cx)? || !self.sources.initialize_step(self.extent, cx)? { return Ok(false); }
        if !self.references.admit_source_step(cx)? || !self.references.initialize_step(self.extent, cx)? { return Ok(false); }
        if self.construction == constructions {
            return self.sort(model, inferred, input, cx);
        }
        let range = self.ranges.get(self.construction).ok_or_else(|| ValueError::invariant("gbXML current layer range disappeared"))?;
        if self.pending_source.is_some() {
            if self.sources.publish_step(self.count, &mut self.pending_source, cx)? {
                self.pending_reference = Some(self.count);
                self.count += 1;
            }
            return Ok(false);
        }
        if self.pending_reference.is_some() {
            if self.references.publish_step(range.first + self.layer, &mut self.pending_reference, cx)? {
                self.layer += 1;
                self.compare = 0;
            }
            return Ok(false);
        }
        if !cx.charge_work_unit() { return Ok(false); }
        if self.layer == range.count {
            self.construction += 1;
            self.layer = 0;
            return Ok(false);
        }
        let current = layers(model, inferred, input, self.construction).get(self.layer)
            .ok_or_else(|| ValueError::invariant("gbXML construction layer disappeared"))?;
        let candidate = key(current);
        if !model.materials.contains_key(candidate.0) {
            self.layer += 1;
            self.compare = 0;
        } else if self.compare == self.count {
            self.pending_source = Some(MaterialSource { construction: self.construction, layer: self.layer });
        } else if self.layer_at(model, inferred, input, self.compare).map(key) == Some(candidate) {
            self.pending_reference = Some(self.compare);
        } else {
            self.compare += 1;
        }
        Ok(false)
    }

    fn sort(&mut self, model: &ModelSnapshot, inferred: &ModelInference, input: &OriginalConstructions, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.order.admit_source_step(cx)? || !self.order.initialize_step(self.count, cx)? { return Ok(false); }
        if self.sorted == self.count { return Ok(true); }
        if let Some(rank) = self.pending_order {
            if self.order.publish_step(self.sorted, &mut self.pending_order, cx)? {
                self.sorted += 1;
                self.scan = 0;
                self.after = Some(rank);
                self.selected = None;
            }
        } else if cx.charge_work_unit() {
            if self.scan == self.count {
                self.pending_order = Some(self.selected.ok_or_else(|| ValueError::invariant("gbXML material sort exhausted early"))?);
            } else {
                let rank = self.scan;
                self.scan += 1;
                let ready = |index| self.layer_at(model, inferred, input, index).map(key);
                let material = ready(rank).ok_or_else(|| ValueError::invariant("gbXML material sort source disappeared"))?;
                if self.after.and_then(ready).is_none_or(|after| material > after)
                    && self.selected.and_then(ready).is_none_or(|selected| material < selected)
                {
                    self.selected = Some(rank);
                }
            }
        }
        Ok(false)
    }

    /// 🗂️ The number of unique used materials.
    pub fn len(&self) -> usize { self.count }

    /// 🧮️ Whether material planning published a used material.
    pub fn is_empty(&self) -> bool { self.count == 0 }

    /// 📖️ The original layer that supplied a material's properties at first encounter.
    pub fn layer_at<'a>(&self, model: &'a ModelSnapshot, inferred: &ModelInference, input: &OriginalConstructions, index: usize) -> Option<&'a Layer> {
        if self.source != Some((std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize, std::ptr::from_ref(input) as usize)) { return None; }
        let position = self.sources.get(index)?;
        layers(model, inferred, input, position.construction).get(position.layer)
    }

    /// 🔢️ Returns a material index from the sorted material order.
    pub fn sorted_at(&self, index: usize) -> Option<usize> { self.order.get(index).copied() }

    /// 🔗️ The material index referenced by a construction's original layer ordinal.
    pub fn reference(&self, construction: usize, layer: usize) -> Option<usize> {
        let range = self.ranges.get(construction)?;
        if layer >= range.count { return None; }
        self.references.get(range.first + layer).copied()
    }

    /// ♻️ Quotes retirement of the current paid material owner.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.order.terminal_is_empty() { self.order.retirement_demands(body) }
        else if !self.references.terminal_is_empty() { self.references.retirement_demands(body) }
        else if !self.sources.terminal_is_empty() { self.sources.retirement_demands(body) }
        else if !self.ranges.terminal_is_empty() { self.ranges.retirement_demands(body) }
        else { Ok(Default::default()) }
    }

    /// 🧺️ Whether the four indexed material owners have retired.
    pub fn terminal_is_empty(&self) -> bool {
        self.order.terminal_is_empty() && self.references.terminal_is_empty() && self.sources.terminal_is_empty() && self.ranges.terminal_is_empty()
    }

    /// 🔚️ Retires one material source, reference or sorted row.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.order.terminal_is_empty() { self.order.close_step(cx)?; }
        else if !self.references.terminal_is_empty() { self.references.close_step(cx)?; }
        else if !self.sources.terminal_is_empty() { self.sources.close_step(cx)?; }
        else if !self.ranges.terminal_is_empty() { self.ranges.close_step(cx)?; }
        Ok(self.terminal_is_empty())
    }
}
