//! 🗂️ Analytical identity indexes borrow ranked source keys and retain only original-funded row locators.
use super::{Fem3dSnapshot, Fem3dSnapshotValue};
use semio_framework_dsl_record::receiving::{NativeSnapshotDecodeOwner, work};
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::{cmp::Ordering, mem::size_of};

/// 🏷️ Canonical analytical namespaces retain independent source identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FemIdentityKind {
    Node,
    Element,
    Material,
    Section,
    Support,
    LoadCase,
}
impl FemIdentityKind {
    fn index(self) -> u8 {
        match self {
            Self::Node => 0,
            Self::Element => 1,
            Self::Material => 2,
            Self::Section => 3,
            Self::Support => 4,
            Self::LoadCase => 5,
        }
    }
    fn from_index(index: u8) -> Result<Self, ValueError> {
        match index {
            0 => Ok(Self::Node),
            1 => Ok(Self::Element),
            2 => Ok(Self::Material),
            3 => Ok(Self::Section),
            4 => Ok(Self::Support),
            5 => Ok(Self::LoadCase),
            _ => Err(invalid()),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Element => "element",
            Self::Material => "material",
            Self::Section => "section",
            Self::Support => "support",
            Self::LoadCase => "loadCase",
        }
    }
}
/// 👓️ Immutable authored and actual owners lend constant-time ranked identities until the index retires.
pub trait FemIdentitySource {
    fn identity_count(&self, kind: FemIdentityKind) -> usize;
    fn identity(&self, kind: FemIdentityKind, row: usize) -> Option<&str>;
    fn load_count(&self, case: usize) -> usize;
    fn load_identity(&self, case: usize, row: usize) -> Option<&str>;
    fn identity_source(&self) -> usize
    where
        Self: Sized,
    {
        self as *const Self as usize
    }
}
macro_rules! source{
 ($($source:ty),+)=>{$(impl FemIdentitySource for $source{
  fn identity_count(&self,kind:FemIdentityKind)->usize{match kind{FemIdentityKind::Node=>self.nodes.len(),FemIdentityKind::Element=>self.elements.len(),FemIdentityKind::Material=>self.materials.len(),FemIdentityKind::Section=>self.sections.len(),FemIdentityKind::Support=>self.supports.len(),FemIdentityKind::LoadCase=>self.load_cases.len()}}
  fn identity(&self,kind:FemIdentityKind,row:usize)->Option<&str>{match kind{FemIdentityKind::Node=>self.nodes.get(row).map(|v|v.id.as_str()),FemIdentityKind::Element=>self.elements.get(row).map(crate::standards::v1::subsets::any::schema::structure::element_id),FemIdentityKind::Material=>self.materials.get(row).map(|v|v.id.as_str()),FemIdentityKind::Section=>self.sections.get(row).map(|v|v.id.as_str()),FemIdentityKind::Support=>self.supports.get(row).map(|v|v.id.as_str()),FemIdentityKind::LoadCase=>self.load_cases.get(row).map(|v|v.id.as_str())}}
  fn load_count(&self,case:usize)->usize{self.load_cases.get(case).map_or(0,|v|v.loads.len())}
  fn load_identity(&self,case:usize,row:usize)->Option<&str>{self.load_cases.get(case)?.loads.get(row).map(crate::standards::v1::subsets::any::schema::structure::load_id)}
 })+};
}
source!(Fem3dSnapshot, Fem3dSnapshotValue);
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "FEM analytical identity is repeated, absent, or outside its original namespace")
}
fn step(owner: &mut NativeSnapshotDecodeOwner<'_, '_>, bytes: usize) -> Result<(), ValueError> {
    work(owner, bytes, 1)?;
    owner.native().step()
}
const KINDS: [FemIdentityKind; 6] = [FemIdentityKind::Node, FemIdentityKind::Element, FemIdentityKind::Material, FemIdentityKind::Section, FemIdentityKind::Support, FemIdentityKind::LoadCase];
const LOAD: u8 = 6;
type Row = (u8, usize, usize);
/// 🧳️ One sorted locator vector retains no borrowed text aliases and closes through the original snapshot frame.
#[derive(Default, RetireOwned)]
pub struct FemIdentityIndex {
    rows: Vec<Row>,
    source: usize,
    counts: [usize; 6],
}
impl FemIdentityIndex {
    /// 🎟️ Admits coarse cardinalities before scanning, then funds backing, sorting, duplicate checks and lookup metadata.
    pub fn receive<S: FemIdentitySource>(source: &S, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        let counts = KINDS.map(|kind| source.identity_count(kind));
        let mut total = counts.iter().try_fold(0usize, |sum, count| sum.checked_add(*count)).ok_or_else(invalid)?;
        if total > 64000 {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "FEM analytical identity cardinality exceeds native receiving admission"));
        }
        owner.scoped_stage(|owner| owner.receive_nested::<Self, Self>(|slot, original| {
            original.native().begin_stage(0)?;
            step(original, size_of::<Self>())?;
            *slot = Some(Self { rows: Vec::new(), source: source.identity_source(), counts });
            for case in 0..counts[FemIdentityKind::LoadCase.index() as usize] {
                step(original, 2 * size_of::<usize>())?;
                total = total.checked_add(source.load_count(case)).ok_or_else(invalid)?;
                if total > 64000 {
                    return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "FEM analytical identity cardinality exceeds native receiving admission"));
                }
            }
            let frame = slot.as_mut().unwrap();
            frame.rows = original.receive::<Vec<Row>, Vec<Row>>(|rows, native, body| {
                *rows = Some(Vec::new());
                body.allocate_vec_into(native, total, rows.as_mut().unwrap(), 1)?;
                Ok(rows.take().unwrap())
            })?;
            for kind in KINDS {
                for row in 0..source.identity_count(kind) {
                    let id = source.identity(kind, row).ok_or_else(invalid)?;
                    step(original, size_of::<Row>())?;
                    protocol::wire::validate_scoped_identity_segments(&[kind.name(), id]).map_err(|_| invalid())?;
                    frame.rows.push((kind.index(), row, 0));
                }
            }
            for case in 0..counts[FemIdentityKind::LoadCase.index() as usize] {
                let parent = source.identity(FemIdentityKind::LoadCase, case).ok_or_else(invalid)?;
                for row in 0..source.load_count(case) {
                    step(original, size_of::<Row>())?;
                    let id = source.load_identity(case, row).ok_or_else(invalid)?;
                    protocol::wire::validate_scoped_identity_segments(&["load", parent, id]).map_err(|_| invalid())?;
                    frame.rows.push((LOAD, case, row));
                }
            }
            sort(&mut frame.rows, source, original)?;
            for pair in frame.rows.windows(2) {
                if compare(pair[0], pair[1], source, original)? == Ordering::Equal {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "FEM analytical identity repeats in its namespace"));
                }
            }
            step(original, size_of::<Self>())?;
            Ok(slot.take().unwrap())
        }))
    }
    /// 🔍️ Returns the exact original rank using cancellable binary search over the validated namespace.
    pub fn lookup<S: FemIdentitySource>(&self, source: &S, kind: FemIdentityKind, id: &str, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<usize, ValueError> {
        if self.source != source.identity_source() || KINDS.map(|k| source.identity_count(k)) != self.counts {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "FEM identity lookup requires its unchanged original source"));
        }
        owner.scoped_stage(|owner| {
        owner.native().begin_stage(0)?;
        let (mut lo, mut hi) = (0, self.rows.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let (k, row, _) = self.rows[mid];
            let order = if k != kind.index() {
                step(owner, 2 * size_of::<Row>())?;
                k.cmp(&kind.index())
            } else {
                let key = source.identity(kind, row).ok_or_else(invalid)?;
                compare_text(key, id, owner)?
            };
            match order {
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return Ok(row),
            }
        }
        Err(ValueError::literal(
            ValueRefusalKind::InvalidValue,
            match kind {
                FemIdentityKind::Node => "FEM member or support references an absent node",
                FemIdentityKind::Material => "FEM member references an absent material",
                FemIdentityKind::Section => "FEM member references an absent section",
                _ => "FEM analytical reference points to an absent identity",
            },
        ))
        })
    }
}
fn compare_text(a: &str, b: &str, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Ordering, ValueError> {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    for offset in (0..a.len().min(b.len())).step_by(256) {
        let end = (offset + 256).min(a.len().min(b.len()));
        step(owner, 2 * (end - offset))?;
        let order = a[offset..end].cmp(&b[offset..end]);
        if !order.is_eq() { return Ok(order); }
    }
    step(owner, 2 * size_of::<usize>())?;
    Ok(a.len().cmp(&b.len()))
}
fn compare<S: FemIdentitySource>(a: Row, b: Row, source: &S, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Ordering, ValueError> {
    if a.0 != b.0 {
        step(owner, 2 * size_of::<Row>())?;
        return Ok(a.0.cmp(&b.0));
    }
    if a.0 == LOAD {
        let x = source.identity(FemIdentityKind::LoadCase, a.1).ok_or_else(invalid)?;
        let y = source.identity(FemIdentityKind::LoadCase, b.1).ok_or_else(invalid)?;
        let order = compare_text(x, y, owner)?;
        if !order.is_eq() { return Ok(order); }
        let x = source.load_identity(a.1, a.2).ok_or_else(invalid)?;
        let y = source.load_identity(b.1, b.2).ok_or_else(invalid)?;
        return compare_text(x, y, owner);
    }
    let kind = FemIdentityKind::from_index(a.0)?;
    let x = source.identity(kind, a.1).ok_or_else(invalid)?;
    let y = source.identity(kind, b.1).ok_or_else(invalid)?;
    compare_text(x, y, owner)
}
fn sort<S: FemIdentitySource>(rows: &mut [Row], source: &S, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError> {
    fn sift<S: FemIdentitySource>(rows: &mut [Row], mut root: usize, end: usize, source: &S, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError> {
        loop {
            let left = root * 2 + 1;
            if left >= end {
                return Ok(());
            }
            let mut child = left;
            if left + 1 < end && compare(rows[left], rows[left + 1], source, owner)?.is_lt() {
                child = left + 1;
            }
            if !compare(rows[root], rows[child], source, owner)?.is_lt() {
                return Ok(());
            }
            step(owner, 2 * size_of::<Row>())?;
            rows.swap(root, child);
            root = child;
        }
    }
    let length = rows.len();
    for root in (0..length / 2).rev() {
        sift(rows, root, length, source, owner)?;
    }
    for end in (1..rows.len()).rev() {
        step(owner, 2 * size_of::<Row>())?;
        rows.swap(0, end);
        sift(rows, 0, end, source, owner)?;
    }
    Ok(())
}
#[cfg(test)]
#[path = "identity-index-tests.rs"]
mod tests;
