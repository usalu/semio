//! 🎚️ Projects the original private RGBA raster into paid, bounded native plane reads.
use super::plane_identity::{PlaneFacts, PlaneIdentity, PlaneRole, PlaneShape, PngPlanes};
use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
    paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES},
    retained_clone::retained_clone_grant_admits,
};

#[derive(Clone, Copy)]
pub struct RgbaLoan<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [u8],
}

impl RgbaLoan<'_> {
    fn admits(self, maximum_pixels: usize) -> bool {
        let count = u64::from(self.width) * u64::from(self.height);
        count != 0 && count <= maximum_pixels as u64 && count.checked_mul(4) == Some(self.pixels.len() as u64)
    }
}

struct Plane<'a> {
    pixels: &'a [u8],
    role: PlaneRole,
    count: usize,
}

impl ByteReadSource for Plane<'_> {
    fn byte_len(&self) -> usize { self.count }

    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        let count = self.count.saturating_sub(offset).min(output.len()).min(PAGED_BYTES_CHUNK_BYTES);
        for (index, out) in (offset..offset + count).zip(output) {
            *out = self.byte_at(index).expect("validated raster plane extent");
        }
        count
    }

    fn byte_at(&self, index: usize) -> Option<u8> {
        if index >= self.count { return None; }
        let rgba = match self.role {
            PlaneRole::Color => (index / 3) * 4 + index % 3,
            PlaneRole::Alpha => index * 4 + 3,
        };
        self.pixels.get(rgba).copied()
    }
}

pub struct PlaneProjection<'a> {
    shape: PlaneShape,
    color: Plane<'a>,
    alpha: Plane<'a>,
}

impl<'a> PlaneProjection<'a> {
    fn from_raster(shape: PlaneShape, loan: RgbaLoan<'a>) -> Self {
        let plane = |role| Plane { role, pixels: loan.pixels, count: shape.len(role) };
        Self { shape, color: plane(PlaneRole::Color), alpha: plane(PlaneRole::Alpha) }
    }

    pub fn planes(&self) -> PngPlanes<'_> {
        PngPlanes::new(self.shape, &self.color, &self.alpha).expect("exact projected plane lengths")
    }
}

enum Phase {
    Alpha(usize),
    Hash(PlaneIdentity),
    Ready(PlaneFacts),
    Revoked,
}

pub struct RawPngPlacementSamples {
    source: usize,
    length: usize,
    width: u32,
    height: u32,
    revision: [u8; 32],
    phase: Phase,
}

fn fault(kind: ValueRefusalKind) -> ValueError {
    ValueError::literal(kind, "PDF PNG placement lost its admitted raster or canonical revision")
}

impl RawPngPlacementSamples {
    pub fn birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Self>>(), depth: 1, ..Default::default() }
    }

    /// 🎟️ Admits the original decoded raster and captures the full canonical revision.
    pub fn admit(
        output: &mut Option<Self>, raster: RgbaLoan<'_>, revision: [u8; 32], maximum_pixels: usize,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::birth_demand();
        if output.is_some() || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        if !raster.admits(maximum_pixels) { return Err(fault(ValueRefusalKind::InvalidValue)); }
        *output = Some(Self {
            source: raster.pixels.as_ptr() as usize, length: raster.pixels.len(),
            width: raster.width, height: raster.height, revision, phase: Phase::Alpha(0),
        });
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn check(&mut self, raster: RgbaLoan<'_>, live: &[u8; 32], canceled: bool) -> Result<(), ValueError> {
        if canceled || &self.revision != live { self.phase = Phase::Revoked; }
        if matches!(self.phase, Phase::Revoked) { return Err(fault(ValueRefusalKind::Canceled)); }
        if raster.pixels.as_ptr() as usize != self.source || raster.pixels.len() != self.length
            || raster.width != self.width || raster.height != self.height
        {
            return Err(fault(ValueRefusalKind::OwnershipLimit));
        }
        Ok(())
    }

    /// ⏭️ Scans opacity or hashes at most one fixed page of the original straight samples.
    pub fn advance(
        &mut self, raster: RgbaLoan<'_>, live: &[u8; 32], canceled: bool,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.check(raster, live, canceled)?;
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Default::default()); }
        let work = usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX);
        let window = work.min(PAGED_BYTES_CHUNK_BYTES);
        if window == 0 { return Ok(Default::default()); }
        let copied = match &mut self.phase {
            Phase::Alpha(at) => {
                if grant.maximum_copy_bytes < size_of::<Phase>() { return Ok(Default::default()); }
                let count = window.min(raster.pixels.len() / 4 - *at);
                if !effect.performed(count as u64)? { return Ok(Default::default()); }
                let end = *at + count;
                let alpha = raster.pixels[*at * 4..end * 4].chunks_exact(4).fold(false, |any, pixel| any | (pixel[3] != 255));
                *at = end;
                if alpha || end * 4 == raster.pixels.len() {
                    let shape = PlaneShape::new(raster.width, raster.height, 3, alpha)
                        .ok_or_else(|| fault(ValueRefusalKind::InvalidValue))?;
                    self.phase = Phase::Hash(PlaneIdentity::new(shape));
                }
                size_of::<Phase>()
            }
            Phase::Hash(identity) => {
                let Some((role, offset)) = identity.position() else {
                    if grant.maximum_copy_bytes < size_of::<Phase>() || !effect.performed(1)? {
                        return Ok(Default::default());
                    }
                    let Phase::Hash(identity) = std::mem::replace(&mut self.phase, Phase::Revoked) else { unreachable!() };
                    self.phase = Phase::Ready(identity.finish().expect("terminal plane digest"));
                    return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Phase>(), ..Default::default() });
                };
                let shape = identity.shape();
                let projection = PlaneProjection::from_raster(shape, raster);
                let count = window.min(grant.maximum_copy_bytes).min(shape.len(role) - offset);
                if count == 0 || !effect.performed(count as u64)? { return Ok(Default::default()); }
                let mut scratch = [0; PAGED_BYTES_CHUNK_BYTES];
                if identity.read_one(projection.planes(), &mut scratch, count) != Some(count) {
                    self.phase = Phase::Revoked;
                    return Err(fault(ValueRefusalKind::InvariantViolated));
                }
                count
            }
            Phase::Ready(_) | Phase::Revoked => return Ok(Default::default()),
        };
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() })
    }

    /// 🧾 Borrows both plane projections after digest completion under the same live revision.
    pub fn sealed<'a>(
        &mut self, raster: RgbaLoan<'a>, live: &[u8; 32], canceled: bool,
    ) -> Result<Option<(PlaneFacts, PlaneProjection<'a>)>, ValueError> {
        self.check(raster, live, canceled)?;
        match self.phase {
            Phase::Ready(facts) => Ok(Some((facts, PlaneProjection::from_raster(facts.shape, raster)))),
            _ => Ok(None),
        }
    }
}
