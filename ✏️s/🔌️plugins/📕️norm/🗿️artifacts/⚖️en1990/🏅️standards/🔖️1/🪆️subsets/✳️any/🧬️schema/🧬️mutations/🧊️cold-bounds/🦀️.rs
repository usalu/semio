//! 📐️ The Store envelope and heap quote of the EN 1990 cold replay: the domain prices its base by table rows and its mutation by its resident size, both without allocating, and the shared cold edit admits the reference worst-case heap of those sizes before it applies.
use crate::En1990Mutation;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use store::os_store::{ColdHeapQuote, ColdPreparationBounds};

const ROW_TEXT_ALLOWANCE_BYTES: usize = 256;
const MUTATION_TEXT_ALLOWANCE_BYTES: usize = 1024;

fn table<T>(rows: &[T]) -> usize {
    rows.len().saturating_mul(std::mem::size_of::<T>().saturating_add(ROW_TEXT_ALLOWANCE_BYTES))
}

/// 📏️ The resident bytes of one EN 1990 document: the root record, its project id and every table row with a fixed text allowance.
pub fn en1990_snapshot_bytes(snapshot: &En1990Snapshot) -> usize {
    [
        table(&snapshot.permanents),
        table(&snapshot.variables),
        table(&snapshot.accidentals),
        table(&snapshot.seismics),
        table(&snapshot.members),
        table(&snapshot.bridge_sls),
        table(&snapshot.effects),
        table(&snapshot.fatigue_spectra),
        table(&snapshot.reliability_analyses),
        table(&snapshot.test_series),
        table(&snapshot.placements),
        table(&snapshot.cases),
        table(&snapshot.factor_overrides),
        table(&snapshot.traffic_groups),
    ]
    .into_iter()
    .fold(std::mem::size_of::<En1990Snapshot>().saturating_add(snapshot.project_id.len()), usize::saturating_add)
}

/// 📏️ The resident bytes of one EN 1990 mutation: its enum record plus a fixed text allowance.
pub fn en1990_mutation_bytes(_mutation: &En1990Mutation) -> usize {
    std::mem::size_of::<En1990Mutation>().saturating_add(MUTATION_TEXT_ALLOWANCE_BYTES)
}

/// 📐️ The cold replay bounds of the EN 1990 document: the central Store maximum for publication, structural sizes and the reference heap quote.
pub fn en1990_cold_replay_bounds() -> ColdPreparationBounds<En1990Snapshot, En1990Mutation> {
    ColdPreparationBounds {
        maximum_bytes: store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,
        mutation_bytes: |_| store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,
        snapshot_bytes: en1990_snapshot_bytes,
        heap_quote: |snapshot, mutation| ColdHeapQuote::reference(en1990_snapshot_bytes(snapshot), en1990_mutation_bytes(mutation)),
    }
}
