//! 🧱️ Indexes distinct opaque constructions and surface references in canonical encounter order.

use super::{EnvelopeSurface, Holder, ModelInference, ModelSnapshot, OriginalSurfaces, SurfaceAddress, SurfaceKind, holder_of, layers_of};
use super::original_surfaces::surface;
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Signature<'a> { kind: &'static str, type_id: &'a str, u_value: u64 }

fn signature<'a>(model: &'a ModelSnapshot, side: &EnvelopeSurface) -> Option<Signature<'a>> {
    let u_value = side.u_value?.to_bits();
    if matches!(side.kind, SurfaceKind::Window | SurfaceKind::Door | SurfaceKind::CurtainWall) { return None; }
    let holder = holder_of(model, side);
    let kind = holder.map_or("-", |holder| match holder {
        Holder::Wall(_) => "wall",
        Holder::CurtainWall(_) => "curtain-wall",
        Holder::Slab(_) => "slab",
        Holder::Roof(_) => "roof",
        Holder::Ceiling(_) => "ceiling",
        Holder::Window(_) | Holder::Door(_) => "opening",
    });
    let type_id = holder.and_then(|holder| layers_of(model, side, holder)).map_or("-", |(id, _)| id);
    Some(Signature { kind, type_id, u_value })
}

/// 📇️ The original source sides and their canonical opaque-construction addresses.
#[derive(Default)]
pub struct OriginalConstructions {
    source: Option<(usize, usize, usize)>,
    references: InferenceValues<usize>,
    sources: InferenceValues<SurfaceAddress>,
    current: usize,
    count: usize,
    comparison: usize,
    pending_source: Option<SurfaceAddress>,
    pending_reference: Option<usize>,
    scanned: usize,
    schematic: bool,
}

impl OriginalConstructions {
    /// 🛠️ Advances one original signature comparison, row publication or schematic check.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, sides: &OriginalSurfaces, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML construction planning was cancelled").into()); }
        let source = (std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize, std::ptr::from_ref(sides) as usize);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(source); }
            return Ok(false);
        }
        if self.source != Some(source) { return Err(ValueError::invariant("gbXML construction source changed").into()); }
        let extent = sides.extent();
        if !self.sources.admit_source_step(cx)? || !self.sources.initialize_step(extent, cx)? { return Ok(false); }
        if !self.references.admit_source_step(cx)? || !self.references.initialize_step(extent, cx)? { return Ok(false); }
        if self.current == extent {
            if self.schematic || self.scanned == extent { return Ok(true); }
            if cx.charge_work_unit() {
                self.schematic = sides.get(self.scanned).and_then(|row| sides.ordinal(row.surface))
                    .is_some_and(|ordinal| self.references.get(ordinal).is_none());
                self.scanned += 1;
            }
            return Ok(false);
        }
        if self.pending_source.is_some() {
            if self.sources.publish_step(self.count, &mut self.pending_source, cx)? {
                self.pending_reference = Some(self.count);
                self.count += 1;
            }
            return Ok(false);
        }
        if self.pending_reference.is_some() {
            if self.references.publish_step(self.current, &mut self.pending_reference, cx)? {
                self.current += 1;
                self.comparison = 0;
            }
            return Ok(false);
        }
        if !cx.charge_work_unit() { return Ok(false); }
        let address = sides.source_at(self.current).ok_or_else(|| ValueError::invariant("gbXML construction source ordinal disappeared"))?;
        let side = surface(model, inferred, address).ok_or_else(|| ValueError::invariant("gbXML construction side disappeared"))?;
        let Some(candidate) = signature(model, side) else {
            self.current += 1;
            self.comparison = 0;
            return Ok(false);
        };
        if self.comparison == self.count {
            self.pending_source = Some(address);
        } else {
            let same = self.sources.get(self.comparison).and_then(|source| surface(model, inferred, *source))
                .and_then(|source| signature(model, source)).is_some_and(|issued| issued == candidate);
            if same { self.pending_reference = Some(self.comparison); } else { self.comparison += 1; }
        }
        Ok(false)
    }

    /// 🏢️ The original opaque construction count including an actually referenced schematic.
    pub fn len(&self) -> usize { self.count + usize::from(self.schematic) }

    pub(super) fn opaque_len(&self) -> usize { self.count }

    pub(super) fn has_schematic(&self) -> bool { self.schematic }

    /// 🧮️ Whether a retained construction has been planned.
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// 🔑️ Reads the original authored side that supplied this distinct construction.
    pub fn source(&self, index: usize) -> Option<SurfaceAddress> { self.sources.get(index).copied() }

    /// 📐️ Reads a surface's canonical construction address, including the fallback when used.
    pub fn reference(&self, side: usize) -> Option<usize> { self.references.get(side).copied().or(self.schematic.then_some(self.count)) }

    /// ♻️ Quotes the selected original named construction owner.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.references.terminal_is_empty() { self.references.retirement_demands(body) } else if !self.sources.terminal_is_empty() {
            self.sources.retirement_demands(body)
        } else { Ok(Default::default()) }
    }

    /// 🧺️ Whether the original sources and references have retired.
    pub fn terminal_is_empty(&self) -> bool { self.references.terminal_is_empty() && self.sources.terminal_is_empty() }

    /// 🔚️ Releases one original construction row or source loan.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.references.terminal_is_empty() { self.references.close_step(cx)?; } else if !self.sources.terminal_is_empty() {
            self.sources.close_step(cx)?;
        }
        Ok(self.terminal_is_empty())
    }
}
