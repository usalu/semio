pub use semio_framework_value_paged_list::list::PagedList;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dof { Tx, Ty, Tz, Rx, Ry, Rz }

impl Dof {
    pub const ALL: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
    pub fn index(self) -> usize {
        match self { Dof::Tx => 0, Dof::Ty => 1, Dof::Tz => 2, Dof::Rx => 3, Dof::Ry => 4, Dof::Rz => 5 }
    }
}

pub const ASSEMBLY_TRIPLET_INDEX_SPACE: usize = usize::MAX;
pub type AssemblyOrder<T> = PagedList<T, ASSEMBLY_TRIPLET_INDEX_SPACE>;

#[path = "@SRC@/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🧭️dof-map/🦀️.rs"]
mod dof_map;
pub use dof_map::*;
