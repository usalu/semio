"""db_index: read runs in place (byte ranges over their own pages) so loading, searching and merging a run costs its
pages and never a page per entry; merges never grow a run past MAX_RUN_ENTRIES; latest-style queries walk runs
newest-first instead of materializing every entry."""
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs"
s = open(p, encoding="utf-8").read()

def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)

# 1. RunPageReader::skip
rep('''    fn array<const N: usize>(&mut self) -> Result<[u8; N], DbError> {''', '''    fn skip(&mut self, len: usize) -> Result<RunRange, DbError> {
        let end = self.position.checked_add(len).ok_or(DbError::LimitExceeded("index run field cursor"))?;
        if end > self.limit {
            return Err(DbError::Corrupt("index run field exceeds retained body".to_string()));
        }
        let range = RunRange { start: self.position, len };
        self.position = end;
        Ok(range)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], DbError> {''')

# 2. decode_run_pages / merge_run_entries become test-only codec oracles
rep('''async fn decode_run_pages_inner(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''', '''#[cfg(test)]
async fn decode_run_pages_inner(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''')
rep('''async fn decode_run_pages(mut pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''', '''#[cfg(test)]
async fn decode_run_pages(mut pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''')
rep('''async fn merge_run_entries(mut older: RunEntries, mut newer: RunEntries, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''', '''#[cfg(test)]
async fn merge_run_entries(mut older: RunEntries, mut newer: RunEntries, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {''')

# 3. RunView region, inserted before the Merge region
views = '''//#region 🔖️RunView
/// @emoji 📏️ One byte range inside a run's retained pages.
#[derive(Clone, Copy, Debug)]
struct RunRange {
    start: usize,
    len: usize,
}

/// @emoji 👁️ One entry of a run read in place: its key and, for a put, its value, as ranges of the
/// run's own pages.
#[derive(Clone, Copy, Debug)]
struct RunViewEntry {
    key: RunRange,
    value: Option<RunRange>,
}

/// @emoji 👁️ One run read in place. Loading, searching and merging a run costs exactly its retained
/// pages — never a page per entry — so a run of `MAX_RUN_ENTRIES` small entries stays inside one
/// operation's I/O credit, and an index never outgrows that credit as its document grows.
struct RunView {
    pages: db_storage::DbIoPages,
    entries: Box<[RunViewEntry]>,
}

impl RunView {
    fn close(mut self) -> Result<(), DbError> {
        let _ = self.pages.close_step()?;
        drop(self);
        Ok(())
    }

    fn key_cmp(&self, index: usize, key: &IndexBytes) -> Result<std::cmp::Ordering, DbError> {
        run_range_cmp(&self.pages, self.entries[index].key, &key.pages, RunRange { start: 0, len: key.len() })
    }

    fn key_starts_with(&self, index: usize, prefix: &IndexBytes) -> Result<bool, DbError> {
        let key = self.entries[index].key;
        if prefix.len() > key.len {
            return Ok(false);
        }
        Ok(run_range_cmp(&self.pages, RunRange { start: key.start, len: prefix.len() }, &prefix.pages, RunRange { start: 0, len: prefix.len() })? == std::cmp::Ordering::Equal)
    }

    /// @emoji 🔎️ The entry holding exactly `key`, by binary search over the run's ascending keys.
    fn find(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<usize>, DbError> {
        let (mut low, mut high) = (0usize, self.entries.len());
        while low < high {
            control.grant()?;
            let middle = low + (high - low) / 2;
            match self.key_cmp(middle, key)? {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => return Ok(Some(middle)),
            }
        }
        Ok(None)
    }

    /// @emoji 📤️ Copies one of this run's ranges out as caller-owned bytes, charged to the run's own
    /// read operation.
    async fn materialize(&self, range: RunRange, control: &mut IndexCursorControl) -> Result<IndexBytes, DbError> {
        let mut writer = db_storage::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), range.len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        let mut reader = RunPageReader { pages: &self.pages, position: range.start, limit: range.start + range.len };
        while reader.position < reader.limit {
            control.grant()?;
            let fragment = reader.fragment()?;
            let written = writer.write_fragment(fragment)?;
            reader.position += written;
        }
        writer.seal_retained().await.map(|pages| IndexBytes { pages }).map_err(db_storage::DbIoPageWriterRejected::into_error)
    }
}

/// @emoji ⚖️ Lexicographic order of two ranges, each over its own pages.
fn run_range_cmp(left: &db_storage::DbIoPages, left_range: RunRange, right: &db_storage::DbIoPages, right_range: RunRange) -> Result<std::cmp::Ordering, DbError> {
    let mut left_reader = RunPageReader { pages: left, position: left_range.start, limit: left_range.start + left_range.len };
    let mut right_reader = RunPageReader { pages: right, position: right_range.start, limit: right_range.start + right_range.len };
    loop {
        match (left_reader.position < left_reader.limit, right_reader.position < right_reader.limit) {
            (false, false) => return Ok(std::cmp::Ordering::Equal),
            (false, true) => return Ok(std::cmp::Ordering::Less),
            (true, false) => return Ok(std::cmp::Ordering::Greater),
            (true, true) => {}
        }
        let left_fragment = left_reader.fragment()?;
        let right_fragment = right_reader.fragment()?;
        let count = left_fragment.len().min(right_fragment.len());
        match left_fragment[..count].cmp(&right_fragment[..count]) {
            std::cmp::Ordering::Equal => {
                left_reader.position += count;
                right_reader.position += count;
            }
            order => return Ok(order),
        }
    }
}

/// @emoji 👁️ Verifies one run's checksum and structure and reads its entries in place.
async fn view_run_pages(mut pages: db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
    match view_run_entries(&pages, expected_kind, control).await {
        Ok(entries) => Ok(RunView { pages, entries }),
        Err(error) => {
            let _ = pages.close_step()?;
            drop(pages);
            Err(error)
        }
    }
}

async fn view_run_entries(pages: &db_storage::DbIoPages, expected_kind: IndexKind, control: &mut IndexCursorControl) -> Result<Box<[RunViewEntry]>, DbError> {
    control.grant()?;
    if pages.len() < 4 {
        return Err(DbError::Corrupt("index run is shorter than its checksum trailer".to_string()));
    }
    let body_len = pages.len() - 4;
    let mut checksum = pack::codec::Crc32cCursor::new();
    let mut remaining = body_len;
    for fragment in pages.fragments() {
        control.grant()?;
        let count = remaining.min(fragment.len());
        checksum.update_page(&fragment[..count]);
        remaining -= count;
        if remaining == 0 {
            break;
        }
    }
    let mut trailer = RunPageReader::new(pages, pages.len());
    trailer.position = body_len;
    if checksum.finish() != u32::from_le_bytes(trailer.array::<4>()?) {
        return Err(DbError::Corrupt("index run checksum mismatch".to_string()));
    }
    let mut reader = RunPageReader::new(pages, body_len);
    let header = read_run_header(&mut reader, expected_kind, control).await?;
    let mut entries: Vec<RunViewEntry> = Vec::with_capacity(header.entry_count as usize);
    for _ in 0..header.entry_count {
        control.grant()?;
        let key_len = reader.varint()?;
        check_len(key_len, MAX_KEY_LEN, "db_index::key")?;
        let key = reader.skip(key_len as usize)?;
        if let Some(previous) = entries.last() {
            if run_range_cmp(pages, previous.key, pages, key)? != std::cmp::Ordering::Less {
                return Err(DbError::Corrupt("index run entries are not strictly ascending by key".to_string()));
            }
        }
        let value = match reader.byte()? {
            0 => None,
            1 => {
                let value_len = reader.varint()?;
                check_len(value_len, MAX_VALUE_LEN, "db_index::value")?;
                Some(reader.skip(value_len as usize)?)
            }
            other => return Err(DbError::Corrupt(format!("index run entry has unknown value tag {other}"))),
        };
        entries.push(RunViewEntry { key, value });
    }
    if reader.position != body_len {
        return Err(DbError::Corrupt("index run has trailing bytes before checksum".to_string()));
    }
    Ok(entries.into_boxed_slice())
}

/// @emoji 📌️ One output entry of a merge: which input view, which of its entries.
#[derive(Clone, Copy)]
struct RunPick {
    view: usize,
    entry: usize,
}

/// @emoji 🔀️ The ascending merge of two adjacent runs, the newer winning on an equal key and
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

async fn run_write_range(writer: &mut db_storage::DbIoPageWriter, checksum: &mut pack::codec::Crc32cCursor, pages: &db_storage::DbIoPages, range: RunRange, control: &mut IndexCursorControl) -> Result<(), DbError> {
    let mut reader = RunPageReader { pages, position: range.start, limit: range.start + range.len };
    while reader.position < reader.limit {
        control.grant()?;
        let fragment = reader.fragment()?;
        run_write(writer, checksum, fragment).await?;
        reader.position += fragment.len();
    }
    Ok(())
}

/// @emoji ✍️ Encodes picked entries of in-place runs as one run, copying key and value bytes straight
/// from their source pages — the same wire as `encode_run_pages`.
async fn encode_run_from_views(kind: IndexKind, views: [&RunView; 2], picks: &[RunPick], control: &mut IndexCursorControl) -> Result<db_storage::DbIoPages, DbError> {
    check_len(picks.len() as u64, MAX_RUN_ENTRIES, "db_index::entries")?;
    let mut encoded_len = RUN_MAGIC.len() + 2 + varint_len(picks.len() as u64) + 4;
    for pick in picks {
        control.grant()?;
        let entry = views[pick.view].entries[pick.entry];
        encoded_len = encoded_len.checked_add(varint_len(entry.key.len as u64) + entry.key.len + 1).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        if let Some(value) = entry.value {
            encoded_len = encoded_len.checked_add(varint_len(value.len as u64) + value.len).ok_or(DbError::LimitExceeded("db_index encoded run bytes"))?;
        }
    }
    let mut writer = db_storage::DbIoPageWriter::try_reserve(encoded_len.div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
    let mut checksum = pack::codec::Crc32cCursor::new();
    run_write(&mut writer, &mut checksum, &RUN_MAGIC).await?;
    run_write(&mut writer, &mut checksum, &[RUN_VERSION, kind.tag()]).await?;
    let mut varint = [0u8; 10];
    run_write(&mut writer, &mut checksum, encode_varint(picks.len() as u64, &mut varint)).await?;
    for pick in picks {
        let view = views[pick.view];
        let entry = view.entries[pick.entry];
        run_write(&mut writer, &mut checksum, encode_varint(entry.key.len as u64, &mut varint)).await?;
        run_write_range(&mut writer, &mut checksum, &view.pages, entry.key, control).await?;
        match entry.value {
            None => run_write(&mut writer, &mut checksum, &[0]).await?,
            Some(value) => {
                run_write(&mut writer, &mut checksum, &[1]).await?;
                run_write(&mut writer, &mut checksum, encode_varint(value.len as u64, &mut varint)).await?;
                run_write_range(&mut writer, &mut checksum, &view.pages, value, control).await?;
            }
        }
    }
    run_write_trailer(&mut writer, &checksum.finish().to_le_bytes()).await?;
    writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)
}
//#endregion 🔖️RunView

//#region 🔖️Merge
'''
rep('''//#region 🔖️Merge
''', views)

# 4. merge policy doc
rep('''/// @emoji ⚖️ When `IndexHandle::put_batch` should automatically fold old runs together. This
/// crate's own choice (the contract fixes the LSM-lite shape, not the trigger threshold): after
/// every write, while a kind's live run count exceeds `max_runs_before_merge`, the two OLDEST runs
/// are merged into one (see `IndexHandle::maybe_auto_merge`) — a bounded, incremental amount of
/// merge work per write rather than a large stop-the-world compaction.''', '''/// @emoji ⚖️ When `IndexHandle::put_batch` should automatically fold runs together. This crate's own
/// choice (the contract fixes the LSM-lite shape, not the trigger threshold): after every write,
/// while a kind has more than `max_runs_before_merge` runs, the oldest adjacent pair among its newest
/// `max_runs_before_merge + 1` runs whose entries fit one run (`MAX_RUN_ENTRIES`) is merged into one
/// (see `IndexHandle::maybe_auto_merge`). A run never grows past `MAX_RUN_ENTRIES`, so every run —
/// and every merge of two — stays inside one operation's I/O credit however large its document grows;
/// full runs simply accumulate behind the newest window.''')

old_load = s[s.index("    async fn load_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {"):s.index("    /// @emoji ✍️ Durably appends `entries` as one new, newest retained run,")]
new_load = '''    async fn view_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
        let pages = self.storage.read_run(&self.document, run_id).await?;
        view_run_pages(pages, self.kind, control).await
    }

    async fn run_entry_count(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<u64, DbError> {
        let mut pages = self.storage.read_run(&self.document, run_id).await?;
        let count = peek_entry_count(&pages, self.kind, control).await;
        control.grant()?;
        let _ = pages.close_step()?;
        drop(pages);
        count
    }

    /// @emoji 🔀️ Folds two adjacent runs (`older` directly beneath `newer`) into `older`'s run id and
    /// deletes `newer`'s: written before deleted, so a crash between the two leaves both runs, and the
    /// newer copy of every key still wins. The caller only picks a pair whose entries fit one run.
    async fn merge_adjacent(&self, older_id: u64, newer_id: u64, drop_tombstones: bool, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let older = self.view_run(older_id, control).await?;
        let newer = match self.view_run(newer_id, control).await {
            Ok(newer) => newer,
            Err(error) => {
                older.close()?;
                return Err(error);
            }
        };
        let encoded = match merge_run_views(&older, &newer, drop_tombstones, control) {
            Ok(picks) if picks.is_empty() => Ok(None),
            Ok(picks) => encode_run_from_views(self.kind, [&older, &newer], &picks, control).await.map(Some),
            Err(error) => Err(error),
        };
        older.close()?;
        newer.close()?;
        match encoded? {
            Some(pages) => self.storage.write_run(&self.document, older_id, pages).await?,
            None => self.storage.delete_run(&self.document, older_id).await?,
        }
        self.storage.delete_run(&self.document, newer_id).await
    }

    /// @emoji 🥇️ The greatest live entry whose key starts with `prefix` and is at most `upper`, walking
    /// runs newest-first and keeping only the best candidate plus the tombstoned keys above it —
    /// never every entry of the kind. The newest occurrence of a key decides whether it is live.
    pub async fn last_live_in_range(&self, prefix: &IndexBytes, upper: Option<&IndexBytes>, control: &mut IndexCursorControl) -> Result<Option<(IndexBytes, IndexBytes)>, DbError> {
        const DEAD_KEYS_MAX: usize = MAX_RUN_ENTRIES as usize;
        let mut ids = self.kind_run_ids(control).await?;
        let mut best: Option<(IndexBytes, IndexBytes)> = None;
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..ids.len()).rev() {
            let view = match self.view_run(ids.as_slice()[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in (0..view.entries.len()).rev() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(if view.key_cmp(index, prefix)? == std::cmp::Ordering::Less { RangeStep::Below } else { RangeStep::Skip });
                    }
                    if let Some(upper) = upper {
                        if view.key_cmp(index, upper)? == std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    if let Some((best_key, _)) = best.as_ref() {
                        if view.key_cmp(index, best_key)? != std::cmp::Ordering::Greater {
                            return Ok(RangeStep::Below);
                        }
                    }
                    for dead_key in &dead {
                        if view.key_cmp(index, dead_key)? == std::cmp::Ordering::Equal {
                            return Ok(RangeStep::Skip);
                        }
                    }
                    let entry = view.entries[index];
                    match entry.value {
                        None => {
                            if dead.len() == DEAD_KEYS_MAX {
                                return Err(DbError::LimitExceeded("index range scan tombstone window"));
                            }
                            dead.push(view.materialize(entry.key, control).await?);
                            Ok(RangeStep::Skip)
                        }
                        Some(value) => {
                            let key = view.materialize(entry.key, control).await?;
                            let value = view.materialize(value, control).await?;
                            if let Some((previous_key, previous_value)) = best.replace((key, value)) {
                                close_index_bytes(previous_key, control).await?;
                                close_index_bytes(previous_value, control).await?;
                            }
                            Ok(RangeStep::Below)
                        }
                    }
                }
                .await;
                match step {
                    Ok(RangeStep::Skip) => {}
                    Ok(RangeStep::Below) => break,
                    Err(error) => {
                        failure = Some(error);
                        view.close()?;
                        break 'runs;
                    }
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        control.grant()?;
        let _ = ids.close_step();
        drop(ids);
        if let Some(error) = failure {
            if let Some((key, value)) = best {
                close_index_bytes(key, control).await?;
                close_index_bytes(value, control).await?;
            }
            return Err(error);
        }
        Ok(best)
    }

'''
s = s.replace(old_load, new_load)

# get
old_get = s[s.index("    /// @emoji 🔎️ Resolves `key` by scanning runs newest-to-oldest and returning the first match —"):s.index("    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`,")]
new_get = '''    /// @emoji 🔎️ Resolves `key` by searching runs newest-to-oldest in place and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<IndexBytes>, DbError> {
        let mut ids = self.kind_run_ids(control).await?;
        let mut result = Ok(None);
        for position in (0..ids.len()).rev() {
            control.grant()?;
            let view = match self.view_run(ids.as_slice()[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    result = Err(error);
                    break;
                }
            };
            let found = match view.find(key, control) {
                Ok(Some(index)) => match view.entries[index].value {
                    Some(value) => Some(view.materialize(value, control).await.map(Some)),
                    None => Some(Ok(None)),
                },
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            };
            view.close()?;
            if let Some(found) = found {
                result = found;
                break;
            }
        }
        control.grant()?;
        let _ = ids.close_step();
        drop(ids);
        result
    }

'''
s = s.replace(old_get, new_get)

# scan_prefix
old_scan = s[s.index("    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`,"):s.index("    /// @emoji 🌀️ `MergePolicy`'s enforcement:")]
new_scan = '''    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`, ascending by
    /// key — runs searched in place newest-first, each key decided by its newest occurrence, only the
    /// live matches materialized (at most `MAX_RUN_ENTRIES` of them).
    pub async fn scan_prefix(&self, prefix: &IndexBytes, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let mut run_ids = self.kind_run_ids(control).await?;
        let mut output = RunEntries::new();
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..run_ids.len()).rev() {
            let view = match self.view_run(run_ids.as_slice()[position], control).await {
                Ok(view) => view,
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            };
            for index in 0..view.entries.len() {
                let step = async {
                    control.grant()?;
                    if !view.key_starts_with(index, prefix)? {
                        return Ok::<_, DbError>(());
                    }
                    for seen in (0..output.len()).filter_map(|slot| output.get(slot)).map(|entry| &entry.key).chain(dead.iter()) {
                        if view.key_cmp(index, seen)? == std::cmp::Ordering::Equal {
                            return Ok(());
                        }
                    }
                    let entry = view.entries[index];
                    let key = view.materialize(entry.key, control).await?;
                    match entry.value {
                        None => {
                            if dead.len() == MAX_RUN_ENTRIES as usize {
                                close_index_bytes(key, control).await?;
                                return Err(DbError::LimitExceeded("index scan tombstone window"));
                            }
                            dead.push(key);
                        }
                        Some(value) => {
                            let value = view.materialize(value, control).await?;
                            if let Err(entry) = output.push(RunEntry { key, value: RunValue::Put(value) }) {
                                close_run_entry(entry, control).await?;
                                return Err(DbError::LimitExceeded("index scan result owner"));
                            }
                        }
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = step {
                    failure = Some(error);
                    view.close()?;
                    break 'runs;
                }
            }
            view.close()?;
        }
        for key in dead {
            close_index_bytes(key, control).await?;
        }
        control.grant()?;
        let _ = run_ids.close_step();
        drop(run_ids);
        if let Some(error) = failure {
            while output.close_step()? {}
            return Err(error);
        }
        for pass in 0..output.len() {
            for index in 0..output.len().saturating_sub(pass + 1) {
                output.sort_step(index, index + 1, control)?;
            }
        }
        Ok(output)
    }

'''
s = s.replace(old_scan, new_scan)

# maybe_auto_merge + compact
old_merge = s[s.index("    /// @emoji 🌀️ `MergePolicy`'s enforcement:"):s.index("    /// @emoji 📊️ Current shape of this kind's runs")]
new_merge = '''    /// @emoji 🌀️ `MergePolicy`'s enforcement: while this kind has more runs than
    /// `policy.max_runs_before_merge`, merges the oldest adjacent pair among the newest
    /// `max_runs_before_merge + 1` runs whose entries fit one run. Tombstones are dropped only when the
    /// pair holds the kind's oldest run, since nothing older can still need shadowing.
    async fn maybe_auto_merge(&self, control: &mut IndexCursorControl) -> Result<(), DbError> {
        loop {
            let mut run_ids = self.kind_run_ids(control).await?;
            let count = run_ids.len();
            if count <= self.policy.max_runs_before_merge {
                control.grant()?;
                let _ = run_ids.close_step();
                drop(run_ids);
                return Ok(());
            }
            let window = count - (self.policy.max_runs_before_merge + 1);
            let ids: Vec<u64> = run_ids.as_slice()[window..].to_vec();
            control.grant()?;
            let _ = run_ids.close_step();
            drop(run_ids);
            let mut sizes = Vec::with_capacity(ids.len());
            for id in &ids {
                sizes.push(self.run_entry_count(*id, control).await?);
            }
            let Some(pair) = (0..ids.len() - 1).find(|pair| sizes[*pair] + sizes[*pair + 1] <= MAX_RUN_ENTRIES) else { return Ok(()) };
            self.merge_adjacent(ids[pair], ids[pair + 1], window + pair == 0, control).await?;
        }
    }

    /// @emoji 🧹️ Folds this kind's runs into the fewest runs `MAX_RUN_ENTRIES` allows: from the oldest,
    /// every adjacent pair that fits one run is merged (tombstones dropped wherever the pair holds the
    /// oldest run, and from a lone oldest run). Returns the post-compaction `stats()`.
    pub async fn compact(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut position = 0usize;
        loop {
            let mut run_ids = self.kind_run_ids(control).await?;
            let pair = (position + 1 < run_ids.len()).then(|| (run_ids.as_slice()[position], run_ids.as_slice()[position + 1]));
            let oldest = run_ids.as_slice().first().copied();
            control.grant()?;
            let _ = run_ids.close_step();
            drop(run_ids);
            let Some((older, newer)) = pair else {
                if let Some(oldest) = oldest {
                    self.drop_run_tombstones(oldest, control).await?;
                }
                break;
            };
            if self.run_entry_count(older, control).await? + self.run_entry_count(newer, control).await? <= MAX_RUN_ENTRIES {
                self.merge_adjacent(older, newer, position == 0, control).await?;
            } else {
                if position == 0 {
                    self.drop_run_tombstones(older, control).await?;
                }
                position += 1;
            }
        }
        self.stats(control).await
    }

    /// @emoji 🧹️ Rewrites the kind's oldest run without its tombstones (nothing older remains for
    /// them to shadow), or deletes it when nothing else is left in it.
    async fn drop_run_tombstones(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let view = self.view_run(run_id, control).await?;
        if view.entries.iter().all(|entry| entry.value.is_some()) {
            return view.close();
        }
        let picks: Vec<RunPick> = (0..view.entries.len()).filter(|index| view.entries[*index].value.is_some()).map(|entry| RunPick { view: 0, entry }).collect();
        let encoded = if picks.is_empty() { Ok(None) } else { encode_run_from_views(self.kind, [&view, &view], &picks, control).await.map(Some) };
        view.close()?;
        match encoded? {
            Some(pages) => self.storage.write_run(&self.document, run_id, pages).await,
            None => self.storage.delete_run(&self.document, run_id).await,
        }
    }

'''
s = s.replace(old_merge, new_merge)

# verify via views
old_verify = '''        let mut ids = self.kind_run_ids(control).await?;
        for index in 0..ids.len() {
            let mut entries = self.load_run(ids.as_slice()[index], control).await?;
            control.grant()?;
            let _ = entries.close_step()?;
            drop(entries);
        }'''
new_verify = '''        let mut ids = self.kind_run_ids(control).await?;
        for index in 0..ids.len() {
            self.view_run(ids.as_slice()[index], control).await?.close()?;
        }'''
rep(old_verify, new_verify)

rep('''//#region 🔖️IndexHandle''', '''/// @emoji 🧭️ What one entry of a newest-first range walk means for the rest of its run.
enum RangeStep {
    Skip,
    Below,
}

//#region 🔖️IndexHandle''')

open(p, "w", encoding="utf-8").write(s)
print("ok")
