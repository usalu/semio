#!/usr/bin/env python3
"""🪜 H14 14c (c): kernel-db `db_index` — an append-only owner appends runs without listing the document's runs and folds
four full runs of one level into one run of the next (bounded by one read operation's credit), so a document's run count
grows with log4 of its entries per level instead of linearly and the 4096-item run listing is never exhausted.
Idempotent, region-guarded; `--dry-run` reports only. usage: python3 h14-index-levels.py [--dry-run]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems, states = [], []


def edit(old, new, label, count=1):
    global text
    if new in text and old not in text:
        states.append("done")
        return
    found = text.count(old)
    if found != count:
        problems.append(f"{label}: expected {count}, found {found}")
        states.append("problem")
        return
    text = text.replace(old, new)
    states.append("replace")


edit("""/// @emoji 🧺️ The entries one run holds at most — what an owner batching its own entries fills.
pub const RUN_ENTRIES_MAX: usize = MAX_RUN_ENTRIES as usize;
""", """/// @emoji 🧺️ The entries one run holds at most — what an owner batching its own entries fills.
pub const RUN_ENTRIES_MAX: usize = MAX_RUN_ENTRIES as usize;

/// @emoji 🪜️ The highest level an append-only owner's runs fold to ([`IndexHandle::append_owned_sorted_run`]): the newest
/// [`LEVEL_FOLD_RUNS`] full runs of level `L` fold into one run of level `L + 1`, so a level-`L` run holds up to
/// `MAX_RUN_ENTRIES · 4^L` entries and a kind of `N` entries spans about `3 · RUN_LEVEL_MAX + N / (MAX_RUN_ENTRIES · 4^RUN_LEVEL_MAX)` runs.
const RUN_LEVEL_MAX: u32 = 3;

/// @emoji 🪜️ How many full runs of one level fold into one run of the next.
const LEVEL_FOLD_RUNS: usize = 4;

/// @emoji 🛡️ Ceiling on the entries of any run a reader admits: a level-[`RUN_LEVEL_MAX`] run's.
const LEVEL_RUN_ENTRIES_MAX: u64 = MAX_RUN_ENTRIES << (2 * RUN_LEVEL_MAX);

/// @emoji 🛡️ Ceiling on the bytes a fold reads (its input runs together): one read operation's credit, so the folded run —
/// never larger than its inputs — is always read in one operation; a fold that would exceed it is skipped and its runs
/// stay as they are.
const LEVEL_FOLD_BYTES_MAX: usize = db_storage::DB_IO_MAX_READ_BYTES as usize;
""", "limits")

edit("""/// @emoji 🔢️ A `run_id`'s layout, high to low: `[format:4][kind:4][sequence:50][entries-1:6]`.
/// `db_storage::IndexStorage` addresses runs by a single flat `u64` per document; the high byte
/// namespaces the runs of one kind (and this crate's run-id format) so ten kinds share one document's
/// storage without colliding, and the low bits carry the run's entry count, so a listing alone tells
/// the merge policy every run's size — no run is read just to learn how many entries it holds.
const RUN_ID_FORMAT: u64 = 1;
const RUN_NAMESPACE_SHIFT: u32 = 56;
const RUN_ENTRY_BITS: u32 = 6;
const RUN_ENTRY_MASK: u64 = (1u64 << RUN_ENTRY_BITS) - 1;
const SEQUENCE_BITS: u32 = RUN_NAMESPACE_SHIFT - RUN_ENTRY_BITS;
const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;
const _: () = assert!(MAX_RUN_ENTRIES == 1 << RUN_ENTRY_BITS);
""", """/// @emoji 🔢️ A `run_id`'s layout, high to low: `[format:4][kind:4][sequence:48][level:2][entries-1:6]`.
/// `db_storage::IndexStorage` addresses runs by a single flat `u64` per document; the high byte
/// namespaces the runs of one kind (and this crate's run-id format) so ten kinds share one document's
/// storage without colliding, and the low bits carry the run's fold level and, for a level-0 run, its
/// entry count, so a listing alone tells the merge policy every run's size — no run is read just to
/// learn how many entries it holds. A folded run keeps its oldest input's sequence and sorts right after
/// it, so ascending ids stay oldest-first.
const RUN_ID_FORMAT: u64 = 2;
const RUN_NAMESPACE_SHIFT: u32 = 56;
const RUN_ENTRY_BITS: u32 = 6;
const RUN_ENTRY_MASK: u64 = (1u64 << RUN_ENTRY_BITS) - 1;
const RUN_LEVEL_BITS: u32 = 2;
const RUN_LEVEL_MASK: u64 = (1u64 << RUN_LEVEL_BITS) - 1;
const RUN_SEQUENCE_SHIFT: u32 = RUN_ENTRY_BITS + RUN_LEVEL_BITS;
const SEQUENCE_BITS: u32 = RUN_NAMESPACE_SHIFT - RUN_SEQUENCE_SHIFT;
const SEQUENCE_MASK: u64 = (1u64 << SEQUENCE_BITS) - 1;
const _: () = assert!(MAX_RUN_ENTRIES == 1 << RUN_ENTRY_BITS);
const _: () = assert!(RUN_LEVEL_MAX as u64 <= RUN_LEVEL_MASK);
""", "layout")

edit("""/// @emoji 🧮️ Packs `kind`, `sequence` and the run's `entries` count into one `run_id`. Errors
/// `LimitExceeded` if `sequence` doesn't fit its 50 bits (2^50 runs of one kind for one document) or
/// `entries` is outside `1..=MAX_RUN_ENTRIES` (an empty run is never written).
fn make_run_id(kind: IndexKind, sequence: u64, entries: usize) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 50-bit per-kind namespace"));
    }
    if entries == 0 || entries as u64 > MAX_RUN_ENTRIES {
        return Err(DbError::LimitExceeded("db_index run entry count"));
    }
    Ok((u64::from(run_namespace(kind)) << RUN_NAMESPACE_SHIFT) | (sequence << RUN_ENTRY_BITS) | (entries as u64 - 1))
}
""", """/// @emoji 🧮️ Packs `kind`, `sequence`, the run's fold `level` and its `entries` count into one `run_id`. Errors
/// `LimitExceeded` if `sequence` doesn't fit its 48 bits (2^48 runs of one kind for one document), `level` exceeds
/// [`RUN_LEVEL_MAX`], or `entries` is outside `1..=` the level's capacity (an empty run is never written).
fn make_run_id(kind: IndexKind, sequence: u64, level: u32, entries: usize) -> Result<u64, DbError> {
    if sequence > SEQUENCE_MASK {
        return Err(DbError::LimitExceeded("db_index run sequence exceeds the 48-bit per-kind namespace"));
    }
    if level > RUN_LEVEL_MAX {
        return Err(DbError::LimitExceeded("db_index run level"));
    }
    if entries == 0 || entries as u64 > level_capacity(level) {
        return Err(DbError::LimitExceeded("db_index run entry count"));
    }
    let count = if level == 0 { entries as u64 - 1 } else { RUN_ENTRY_MASK };
    Ok((u64::from(run_namespace(kind)) << RUN_NAMESPACE_SHIFT) | (sequence << RUN_SEQUENCE_SHIFT) | (u64::from(level) << RUN_ENTRY_BITS) | count)
}

/// @emoji 📐️ The entries a run of `level` holds at most.
fn level_capacity(level: u32) -> u64 {
    MAX_RUN_ENTRIES << (2 * level)
}
""", "make_run_id")

edit("""fn sequence_of_run_id(run_id: u64) -> u64 {
    (run_id >> RUN_ENTRY_BITS) & SEQUENCE_MASK
}

fn entries_of_run_id(run_id: u64) -> u64 {
    (run_id & RUN_ENTRY_MASK) + 1
}
""", """fn sequence_of_run_id(run_id: u64) -> u64 {
    (run_id >> RUN_SEQUENCE_SHIFT) & SEQUENCE_MASK
}

fn level_of_run_id(run_id: u64) -> u32 {
    ((run_id >> RUN_ENTRY_BITS) & RUN_LEVEL_MASK) as u32
}

/// @emoji 📏️ A level-0 run's exact entry count; a folded run's capacity (its level's).
fn entries_of_run_id(run_id: u64) -> u64 {
    match level_of_run_id(run_id) {
        0 => (run_id & RUN_ENTRY_MASK) + 1,
        level => level_capacity(level),
    }
}
""", "run id readers")

edit("""    let entry_count = reader.varint()?;
    check_len(entry_count, MAX_RUN_ENTRIES, "db_index::entries")?;
    Ok(RunHeader { entry_count })""", """    let entry_count = reader.varint()?;
    check_len(entry_count, LEVEL_RUN_ENTRIES_MAX, "db_index::entries")?;
    Ok(RunHeader { entry_count })""", "header ceiling")

edit("""/// @emoji 🔀️ The ascending merge of two adjacent runs, the newer winning on an equal key and
/// tombstones dropped only when nothing older remains beneath.
fn merge_run_views(older: &RunView, newer: &RunView, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<Vec<RunPick>, DbError> {
    let (mut old_index, mut new_index) = (0usize, 0usize);
    let mut picks = Vec::with_capacity(older.entries.len() + newer.entries.len());
    while old_index < older.entries.len() || new_index < newer.entries.len() {
        control.grant()?;
        let order = if old_index == older.entries.len() {
            std::cmp::Ordering::Greater
        } else if new_index == newer.entries.len() {
            std::cmp::Ordering::Less
        } else {
            run_range_cmp(&older.pages, older.entries[old_index].key, &newer.pages, newer.entries[new_index].key)?
        };
        let pick = match order {
            std::cmp::Ordering::Less => {
                old_index += 1;
                RunPick { view: 0, entry: old_index - 1 }
            }
            std::cmp::Ordering::Greater => {
                new_index += 1;
                RunPick { view: 1, entry: new_index - 1 }
            }
            std::cmp::Ordering::Equal => {
                old_index += 1;
                new_index += 1;
                RunPick { view: 1, entry: new_index - 1 }
            }
        };
        let view = if pick.view == 0 { older } else { newer };
        if !(drop_tombstones && view.entries[pick.entry].value.is_none()) {
            picks.push(pick);
        }
    }
    Ok(picks)
}
""", """/// @emoji 🔀️ The ascending merge of adjacent runs (`views` oldest first), the newest winning on an equal
/// key and tombstones dropped only when nothing older remains beneath.
fn merge_run_views(views: &[&RunView], drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<Vec<RunPick>, DbError> {
    let mut heads = vec![0usize; views.len()];
    let mut picks = Vec::with_capacity(views.iter().map(|view| view.entries.len()).sum());
    loop {
        control.grant()?;
        let mut winner: Option<usize> = None;
        for (view, head) in heads.iter().enumerate() {
            if *head == views[view].entries.len() {
                continue;
            }
            winner = match winner {
                None => Some(view),
                Some(best) => match run_range_cmp(&views[view].pages, views[view].entries[*head].key, &views[best].pages, views[best].entries[heads[best]].key)? {
                    std::cmp::Ordering::Greater => Some(best),
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Some(view),
                },
            };
        }
        let Some(winner) = winner else { break };
        let pick = RunPick { view: winner, entry: heads[winner] };
        for view in 0..views.len() {
            if view != winner && heads[view] < views[view].entries.len() && run_range_cmp(&views[view].pages, views[view].entries[heads[view]].key, &views[winner].pages, views[winner].entries[pick.entry].key)? == std::cmp::Ordering::Equal {
                heads[view] += 1;
            }
        }
        heads[winner] += 1;
        if !(drop_tombstones && views[winner].entries[pick.entry].value.is_none()) {
            picks.push(pick);
        }
    }
    Ok(picks)
}
""", "k-way merge")

edit("""async fn encode_run_from_views(kind: IndexKind, views: [&RunView; 2], picks: &[RunPick], control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(picks.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;""", """async fn encode_run_from_views(kind: IndexKind, views: &[&RunView], picks: &[RunPick], control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(picks.len() as u64, LEVEL_RUN_ENTRIES_MAX, "db_index::entries")?;""", "encode from views")

edit("""        let encoded = match merge_run_views(&older, &newer, drop_tombstones, control) {
            Ok(picks) if picks.is_empty() => Ok(None),
            Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(older_id), picks.len()) {
                Ok(merged_id) => encode_run_from_views(self.kind, [&older, &newer], &picks, control).await.map(|pages| Some((merged_id, pages))),""", """        let encoded = match merge_run_views(&[&older, &newer], drop_tombstones, control) {
            Ok(picks) if picks.is_empty() => Ok(None),
            Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(older_id), 0, picks.len()) {
                Ok(merged_id) => encode_run_from_views(self.kind, &[&older, &newer], &picks, control).await.map(|pages| Some((merged_id, pages))),""", "merge_adjacent")

edit("""        let next = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        let run_id = match make_run_id(self.kind, next, count) {""", """        let next = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        let run_id = match make_run_id(self.kind, next, 0, count) {""", "append_run")

edit("""            match make_run_id(self.kind, sequence_of_run_id(run_id), picks.len()) {
                Ok(rewritten) => encode_run_from_views(self.kind, [&view, &view], &picks, control).await.map(|pages| Some((rewritten, pages))),""", """            match make_run_id(self.kind, sequence_of_run_id(run_id), level_of_run_id(run_id), picks.len()) {
                Ok(rewritten) => encode_run_from_views(self.kind, &[&view], &picks, control).await.map(|pages| Some((rewritten, pages))),""", "drop tombstones")

edit("""    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    /// A run whose entry count differs from the one its id declares is corrupt too.
    pub async fn verify(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let ids = self.kind_run_ids(control).await?;
        for id in ids {
            let view = self.view_run(id, control).await?;
            let declared = entries_of_run_id(id);
            let held = view.entries.len() as u64;
            view.close()?;
            if declared != held {
                return Err(DbError::Corrupt(format!("index run {id:#x} declares {declared} entries and holds {held}")));
            }
        }
        Ok(())
    }""", """    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
    /// surfacing the first `DbError::Corrupt` found rather than any value — `db_cli verify`'s hook.
    /// A level-0 run whose entry count differs from the one its id declares, or a folded run holding
    /// more than its level's capacity, is corrupt too.
    pub async fn verify(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let ids = self.kind_run_ids(control).await?;
        for id in ids {
            let view = self.view_run(id, control).await?;
            let declared = entries_of_run_id(id);
            let held = view.entries.len() as u64;
            view.close()?;
            if (level_of_run_id(id) == 0 && declared != held) || held > declared {
                return Err(DbError::Corrupt(format!("index run {id:#x} declares {declared} entries and holds {held}")));
            }
        }
        Ok(())
    }

    /// @emoji 🗂️ Lists this kind's runs once for an append-only owner ([`OwnedRuns`]).
    pub async fn owned_runs(&self, control: &mut IndexCursorControl) -> Result<OwnedRuns, DbError> {
        let ids = self.kind_run_ids(control).await?;
        let next_sequence = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        Ok(OwnedRuns { ids, next_sequence, fold_ceiling: RUN_LEVEL_MAX })
    }

    /// @emoji 📥️ Appends strictly ascending, unique `(key, value)` puts as `runs`' newest run without listing the
    /// document's runs, then folds while the newest [`LEVEL_FOLD_RUNS`] runs are full runs of one level below
    /// `runs`' fold ceiling — at most [`RUN_LEVEL_MAX`] folds of at most [`LEVEL_FOLD_BYTES_MAX`] each, so an append
    /// costs the same however many runs the kind holds. `runs` names exactly what storage holds after every
    /// successful append; after a failed one the owner lists again ([`Self::owned_runs`]).
    pub async fn append_owned_sorted_run(&self, runs: &mut OwnedRuns, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        let pages = encode_sorted_run_pages(self.kind, entries, control).await?;
        let run_id = match make_run_id(self.kind, runs.next_sequence, 0, entries.len()) {
            Ok(run_id) => run_id,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        self.storage.write_run(&self.document, run_id, pages).await?;
        runs.ids.push(run_id);
        runs.next_sequence += 1;
        while runs.ids.len() >= LEVEL_FOLD_RUNS {
            let group: [u64; LEVEL_FOLD_RUNS] = runs.ids[runs.ids.len() - LEVEL_FOLD_RUNS..].try_into().map_err(|_| DbError::Internal("db_index fold group".to_string()))?;
            let level = level_of_run_id(group[0]);
            if level >= runs.fold_ceiling || group.iter().any(|id| level_of_run_id(*id) != level || entries_of_run_id(*id) != level_capacity(level)) {
                return Ok(());
            }
            control.grant()?;
            let Some(folded) = self.fold_runs(&group, level + 1, control).await? else {
                runs.fold_ceiling = level;
                return Ok(());
            };
            runs.ids.truncate(runs.ids.len() - LEVEL_FOLD_RUNS);
            runs.ids.push(folded);
        }
        Ok(())
    }

    /// @emoji 🪜️ Folds `group` (adjacent full runs, oldest first) into one run of `level` under the oldest's
    /// sequence, written before its inputs are deleted — a crash between leaves copies whose values agree and the
    /// folded copy sorts right after its oldest input. `None`, with nothing written, when the inputs exceed
    /// [`LEVEL_FOLD_BYTES_MAX`].
    async fn fold_runs(&self, group: &[u64], level: u32, control: &mut IndexCursorControl) -> Result<Option<u64>, DbError> {
        let mut views: Vec<RunView> = Vec::with_capacity(group.len());
        for id in group {
            match self.view_run(*id, control).await {
                Ok(view) => views.push(view),
                Err(error) => {
                    for view in views {
                        view.close()?;
                    }
                    return Err(error);
                }
            }
        }
        let encoded = if views.iter().map(|view| view.pages.len()).sum::<usize>() > LEVEL_FOLD_BYTES_MAX {
            Ok(None)
        } else {
            let inputs: Vec<&RunView> = views.iter().collect();
            match merge_run_views(&inputs, false, control) {
                Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(group[0]), level, picks.len()) {
                    Ok(folded_id) => encode_run_from_views(self.kind, &inputs, &picks, control).await.map(|pages| Some((folded_id, pages))),
                    Err(error) => Err(error),
                },
                Err(error) => Err(error),
            }
        };
        for view in views {
            view.close()?;
        }
        let Some((folded_id, pages)) = encoded? else { return Ok(None) };
        self.storage.write_run(&self.document, folded_id, pages).await?;
        for id in group {
            self.storage.delete_run(&self.document, *id).await?;
        }
        Ok(Some(folded_id))
    }""", "verify + owned API")

edit("""//#region 🔖️RecordLocation
/// @emoji 📍️ A pointer into a document's WAL""", """//#region 🔖️OwnedRuns
/// @emoji 🗂️ The runs of one kind an append-only owner writes, ascending, as it knows them: listed once when the
/// owner mounts ([`IndexHandle::owned_runs`]) and kept current by its own appends and folds, so an append never
/// lists the document's runs again. `fold_ceiling` is the level this owner stopped folding at because a fold
/// would exceed one read operation's credit (entries of that size never fold higher).
#[derive(Debug)]
pub struct OwnedRuns {
    ids: Vec<u64>,
    next_sequence: u64,
    fold_ceiling: u32,
}

impl OwnedRuns {
    /// @emoji 🔢️ How many runs the kind holds.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}
//#endregion 🔖️OwnedRuns

//#region 🔖️RecordLocation
/// @emoji 📍️ A pointer into a document's WAL""", "OwnedRuns type")

edit("""    async fn record_run(&self, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (seq, location) in entries {
            encoded.push((seq.to_be_bytes(), encode_location(*location).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }""", """    async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (seq, location) in entries {
            encoded.push((seq.to_be_bytes(), encode_location(*location).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }""", "seq-location owned")

edit(f"""    /// @emoji 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`).
    pub async fn record_run(&self, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {{
        self.0.record_run(entries).await
    }}""", f"""    /// @emoji 📥️ Records ascending `(command_seq, location)` pairs as ONE run (at most `MAX_RUN_ENTRIES`) of `runs`, without
    /// listing ([`IndexHandle::append_owned_sorted_run`]).
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(u64, RecordLocation)]) -> Result<(), DbError> {{
        self.0.record_owned_run(runs, entries).await
    }}

    /// @emoji 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {{
        self.0.owned_runs().await
    }}""", "command + inverse wrappers", count=2)

edit("""    /// @emoji 📥️ Records `(actor, actor_seq, command_seq)` triples as ONE run (at most
    /// `MAX_RUN_ENTRIES`), keyed and ordered as `record` keys them.
    pub async fn record_run(&self, entries: &[(ActorId, u64, u64)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (actor, actor_seq, command_seq) in entries {
            encoded.push((actor_seq_key(actor, *actor_seq).await?, command_seq.to_le_bytes()));
        }
        encoded.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }""", """    /// @emoji 📥️ Records `(actor, actor_seq, command_seq)` triples as ONE run (at most
    /// `MAX_RUN_ENTRIES`) of `runs`, keyed and ordered as `record` keys them, without listing.
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, entries: &[(ActorId, u64, u64)]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(entries.len());
        for (actor, actor_seq, command_seq) in entries {
            encoded.push((actor_seq_key(actor, *actor_seq).await?, command_seq.to_le_bytes()));
        }
        encoded.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    /// @emoji 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }""", "actor-seq owned")

edit("""    /// @emoji 📥️ Records ascending frontiers (by `commit_seq`) as ONE run (at most `MAX_RUN_ENTRIES`).
    pub async fn record_run(&self, frontiers: &[Frontier]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(frontiers.len());
        for frontier in frontiers {
            encoded.push((frontier.commit_seq.to_be_bytes(), encode_frontier(frontier).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.put_sorted_run(&slices, &mut control).await
    }""", """    /// @emoji 📥️ Records ascending frontiers (by `commit_seq`) as ONE run (at most `MAX_RUN_ENTRIES`) of `runs`, without listing.
    pub async fn record_owned_run(&self, runs: &mut OwnedRuns, frontiers: &[Frontier]) -> Result<(), DbError> {
        let mut encoded = Vec::with_capacity(frontiers.len());
        for frontier in frontiers {
            encoded.push((frontier.commit_seq.to_be_bytes(), encode_frontier(frontier).await));
        }
        let slices: Vec<(&[u8], &[u8])> = encoded.iter().map(|(key, value)| (&key[..], &value[..])).collect();
        let mut control = self.handle.operation_control(65_536)?;
        self.handle.append_owned_sorted_run(runs, &slices, &mut control).await
    }

    /// @emoji 🗂️ This kind's runs, listed once for its append-only owner.
    pub async fn owned_runs(&self) -> Result<OwnedRuns, DbError> {
        let mut control = self.handle.operation_control(8_192)?;
        self.handle.owned_runs(&mut control).await
    }""", "frontier owned")

edit("""    /// @emoji 📥️ Durably appends one run of already strictly ascending, unique `(key, value)` puts —
    /// the bulk path of an owner that generates its own keys (the artifact engine's command, inverse,
    /// actor-seq and frontier entries): the run is encoded straight from the caller's bytes into one
    /// write, with no per-entry byte owner. Errors `InvalidArgument` on unordered or duplicate keys.
    pub async fn put_sorted_run(&self, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        let pages = encode_sorted_run_pages(self.kind, entries, control).await?;
        self.append_run(pages, entries.len(), control).await
    }

""", "", "remove put_sorted_run")

edit("""/// @emoji ⚖️ When an append (`IndexHandle::put_batch`/`put_sorted_run`) folds runs together.""", """/// @emoji ⚖️ When an append (`IndexHandle::put_batch`) folds runs together.""", "merge policy doc")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    open(PATH, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
