pub use semio_framework_value_paged_list::list::PagedList;
pub use semio_framework_value_control_contract::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
#[path = "@REPO@/🧰️framework/🔨️modules/🌱️value/📶️sort/🦀️.rs"]
pub mod sort;
pub use sort::{HeapSortCursor, HeapSortWork};

pub const SPARSE_INDEX_SPACE: usize = usize::MAX;
pub const SPARSE_PAGE_BYTES: usize = 4096;

pub fn numerical_grant_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

pub fn close_paged_owner_step<T, const N: usize>(owner: &mut PagedList<T, N>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let progress = owner.release_empty_page(maximum_bytes).map_err(|_| ())?;
    if progress.progressed {
        return Ok(Some((0, progress.released_allocation_bytes)));
    }
    if owner.terminal_is_empty() {
        Ok(None)
    } else {
        Err(())
    }
}

pub fn paged_from_values<T, I: IntoIterator<Item = T>>(values: I) -> PagedList<T, SPARSE_INDEX_SPACE> where I::IntoIter: ExactSizeIterator {
    let values = values.into_iter();
    let target = values.len();
    let mut owner = PagedList::default();
    while owner.capacity() < target {
        owner.reserve_capacity_one(target, SPARSE_PAGE_BYTES).expect("cold CSR page allocation");
    }
    for value in values {
        owner.push_reserved(value).unwrap_or_else(|_| unreachable!("cold CSR reserved exact slot"));
    }
    owner
}

#[derive(Clone)]
pub struct Csr {
    pub n: usize,
    pub indptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub indices: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub vals: PagedList<f64, SPARSE_INDEX_SPACE>,
}

impl Csr {
    /// 🧊️ Moves cold contiguous inputs into fixed physical pages.
    pub fn from_owned_parts(n: usize, indptr: Vec<u32>, indices: Vec<u32>, vals: Vec<f64>) -> Self {
        Self { n, indptr: paged_from_values(indptr), indices: paged_from_values(indices), vals: paged_from_values(vals) }
    }

    /// 📚️ Transfers retained paged arrays without a contiguous intermediate owner.
    pub fn from_paged_parts(n: usize, indptr: PagedList<u32, SPARSE_INDEX_SPACE>, indices: PagedList<u32, SPARSE_INDEX_SPACE>, vals: PagedList<f64, SPARSE_INDEX_SPACE>) -> Self {
        Self { n, indptr, indices, vals }
    }

    pub fn indptr(&self, index: usize) -> Option<u32> {
        self.indptr.get(index).copied()
    }

    pub fn index(&self, index: usize) -> Option<u32> {
        self.indices.get(index).copied()
    }

    pub fn value(&self, index: usize) -> Option<f64> {
        self.vals.get(index).copied()
    }

    /// 📦️ Queries the actual retained physical backing without traversing matrix values.
    pub fn allocated_bytes(&self) -> usize {
        self.indptr.allocated_bytes() + self.indices.allocated_bytes() + self.vals.allocated_bytes()
    }

    /// 🧮️ Returns the exact initialized compressed-entry count.
    pub fn entries_len(&self) -> usize {
        self.vals.len()
    }

    /// 🧱️ Reports row-pointer, column-index, and scalar physical backing independently.
    pub fn physical_backing_bytes(&self) -> [usize; 3] {
        [self.indptr.allocated_bytes(), self.indices.allocated_bytes(), self.vals.allocated_bytes()]
    }

    pub fn ownership_is_valid(&self) -> bool {
        self.n.checked_add(1).is_some_and(|rows| self.indptr.len() == rows) && self.indices.len() == self.vals.len() && self.indptr(self.n) == u32::try_from(self.entries_len()).ok()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.indptr.terminal_is_empty() && self.indices.terminal_is_empty() && self.vals.terminal_is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_paged_owner_step(&mut self.vals, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.indices, &mut self.indptr] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }

}

#[path = "@SRC@/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧮️triplet-csr-build/🦀️.rs"]
mod triplet_csr_build;
pub use triplet_csr_build::*;
