use std::mem::size_of;
use semio_framework_value::{paged::PagedUtf8, retained_clone::{RetainedClone, RetainedFieldCursor}};
use semio_framework_value_persistent::list::PagedList;
use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::records::{Puzzle2dEdge, Puzzle2dNode};
use semio_s_artifact_puzzle_2d_snapshot::Puzzle2dSnapshot;
use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;

const fn layout<T: RetainedClone>() -> [u64; 3] {
    [size_of::<T>() as u64, size_of::<RetainedFieldCursor<T>>() as u64, size_of::<T::Cursor>() as u64]
}

#[no_mangle]
pub static LAW_LAYOUTS: [[u64; 3]; 10] = [
    layout::<Puzzle2dSnapshot>(),
    layout::<Puzzle2dNode>(),
    layout::<Puzzle2dEdge>(),
    layout::<PagedList<Puzzle2dNode, { usize::MAX }>>(),
    layout::<PagedList<Puzzle2dEdge, { usize::MAX }>>(),
    layout::<PagedList<String, { usize::MAX }>>(),
    layout::<PagedUtf8<{ usize::MAX }>>(),
    layout::<Option<PagedUtf8<{ usize::MAX }>>>(),
    layout::<String>(),
    [size_of::<Puzzle2dMutation>() as u64, 0, 0],
];
