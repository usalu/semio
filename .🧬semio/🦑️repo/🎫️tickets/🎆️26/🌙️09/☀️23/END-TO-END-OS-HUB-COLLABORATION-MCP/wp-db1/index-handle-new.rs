impl<'a, S: IndexStorage> IndexHandle<'a, S> {
    /// @emoji 🚀️ Opens a handle with the default `MergePolicy`.
    pub async fn new(storage: &'a S, document: ArtifactId, kind: IndexKind) -> Self {
        Self::with_policy(storage, document, kind, MergePolicy::default()).await
    }

    /// @emoji 🚀️ Opens a handle with an explicit `MergePolicy` (e.g. a tighter threshold for a
    /// hot, frequently-scanned kind, or a looser one for a write-heavy, rarely-read kind).
    pub async fn with_policy(storage: &'a S, document: ArtifactId, kind: IndexKind, policy: MergePolicy) -> Self {
        Self { storage, document, kind, policy, cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)) }
    }

    pub fn operation_control(&self, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::new(self.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), fuel)
    }

    /// 🧵️ Mounts a parent job's exact cancellation/deadline authority for retained index work.
    pub fn retained_operation_control(&self, cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<IndexCursorControl, DbError> {
        IndexCursorControl::retained(cancelled, deadline, fuel)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    /// @emoji 📋️ This handle's live run ids, ascending (oldest sequence first) — one storage listing;
    /// every id of another kind or run-id format for the same document is filtered out.
    async fn kind_run_ids(&self, control: &mut IndexCursorControl) -> Result<Vec<u64>, DbError> {
        let mut source = self.storage.list_runs(&self.document).await?;
        let namespace = run_namespace(self.kind);
        let mut output = Vec::new();
        for id in source.as_slice() {
            control.grant()?;
            if namespace_of_run_id(*id) == namespace {
                output.push(*id);
            }
        }
        while source.close_step() {}
        Ok(output)
    }

    async fn view_run(&self, run_id: u64, control: &mut IndexCursorControl) -> Result<RunView, DbError> {
        let pages = self.storage.read_run(&self.document, run_id).await?;
        view_run_pages(pages, self.kind, control).await
    }

    /// @emoji 🔀️ Folds two adjacent runs (`older` directly beneath `newer`) into one run under
    /// `older`'s sequence and deletes both inputs: written before deleted, so a crash in between
    /// leaves copies whose values agree, and the newer copy of every key still wins. The caller only
    /// picks a pair whose entries fit one run.
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
            Ok(picks) => match make_run_id(self.kind, sequence_of_run_id(older_id), picks.len()) {
                Ok(merged_id) => encode_run_from_views(self.kind, [&older, &newer], &picks, control).await.map(|pages| Some((merged_id, pages))),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };
        older.close()?;
        newer.close()?;
        if let Some((merged_id, pages)) = encoded? {
            self.storage.write_run(&self.document, merged_id, pages).await?;
            if merged_id != older_id {
                self.storage.delete_run(&self.document, older_id).await?;
            }
        } else {
            self.storage.delete_run(&self.document, older_id).await?;
        }
        self.storage.delete_run(&self.document, newer_id).await
    }

    /// @emoji 🥇️ The greatest live entry whose key starts with `prefix` and is at most `upper`, walking
    /// runs newest-first and keeping only the best candidate plus the tombstoned keys above it —
    /// never every entry of the kind. The newest occurrence of a key decides whether it is live.
    pub async fn last_live_in_range(&self, prefix: &IndexBytes, upper: Option<&IndexBytes>, control: &mut IndexCursorControl) -> Result<Option<(IndexBytes, IndexBytes)>, DbError> {
        const DEAD_KEYS_MAX: usize = MAX_RUN_ENTRIES as usize;
        let ids = self.kind_run_ids(control).await?;
        let mut best: Option<(IndexBytes, IndexBytes)> = None;
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..ids.len()).rev() {
            let view = match self.view_run(ids[position], control).await {
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
        if let Some(error) = failure {
            if let Some((key, value)) = best {
                close_index_bytes(key, control).await?;
                close_index_bytes(value, control).await?;
            }
            return Err(error);
        }
        Ok(best)
    }

    /// @emoji ✍️ Durably appends `entries` as one new, newest retained run,
    /// then applies `MergePolicy`. A no-op (no run written) if `entries` is empty.
    pub async fn put_batch(&self, mut entries: RunEntries, control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        for pass in 0..entries.len() {
            for index in 0..entries.len().saturating_sub(pass + 1) {
                entries.sort_step(index, index + 1, control)?;
            }
        }
        let mut unique = RunEntries::new();
        for index in 0..entries.len() {
            let entry = entries.take(index).ok_or_else(|| DbError::Internal("index sorted entry owner lost".to_string()))?;
            if unique.get(unique.len().saturating_sub(1)).is_some_and(|previous| index_bytes_cmp(&previous.key, &entry.key) == std::cmp::Ordering::Equal) {
                close_run_entry(unique.pop().ok_or_else(|| DbError::Internal("index duplicate owner lost".to_string()))?, control).await?;
            }
            if let Err(entry) = unique.push(entry) {
                close_run_entry(entry, control).await?;
                return Err(DbError::LimitExceeded("index unique fixed entry owner"));
            }
        }
        let count = unique.len();
        let pages = encode_run_pages(self.kind, &unique, control).await?;
        control.grant()?;
        let _ = unique.close_step()?;
        drop(unique);
        self.append_run(pages, count, control).await
    }

    /// @emoji 📥️ Durably appends one run of already strictly ascending, unique `(key, value)` puts —
    /// the bulk path of an owner that generates its own keys (the artifact engine's command, inverse,
    /// actor-seq and frontier entries): the run is encoded straight from the caller's bytes into one
    /// write, with no per-entry byte owner. Errors `InvalidArgument` on unordered or duplicate keys.
    pub async fn put_sorted_run(&self, entries: &[(&[u8], &[u8])], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if entries.is_empty() {
            return Ok(());
        }
        let pages = encode_sorted_run_pages(self.kind, entries, control)?;
        self.append_run(pages, entries.len(), control).await
    }

    /// @emoji ➕️ Writes `pages` (holding `count` entries) as the kind's newest run from ONE listing
    /// of its runs, then lets `MergePolicy` fold at most one adjacent pair — sizes come from the run
    /// ids, so an append never reads a run it does not merge.
    async fn append_run(&self, pages: db_storage::DbIoPages, count: usize, control: &mut IndexCursorControl) -> Result<(), DbError> {
        let mut ids = match self.kind_run_ids(control).await {
            Ok(ids) => ids,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        let next = ids.last().map_or(0, |id| sequence_of_run_id(*id) + 1);
        let run_id = match make_run_id(self.kind, next, count) {
            Ok(run_id) => run_id,
            Err(error) => {
                close_run_pages(pages)?;
                return Err(error);
            }
        };
        self.storage.write_run(&self.document, run_id, pages).await?;
        ids.push(run_id);
        self.merge_one_within_policy(&ids, control).await
    }

    /// @emoji 🔎️ Resolves `key` by searching runs newest-to-oldest in place and returning the first match —
    /// `Ok(None)` if the first match is a tombstone, or if no run has ever held `key`.
    pub async fn get(&self, key: &IndexBytes, control: &mut IndexCursorControl) -> Result<Option<IndexBytes>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        let mut result = Ok(None);
        for position in (0..ids.len()).rev() {
            control.grant()?;
            let view = match self.view_run(ids[position], control).await {
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
        result
    }

    /// @emoji 🔝️ The kind's newest run read in place, or `None` when the kind holds no run — the
    /// one read an append-only owner needs to learn how far its entries reach.
    async fn newest_run(&self, control: &mut IndexCursorControl) -> Result<Option<RunView>, DbError> {
        let ids = self.kind_run_ids(control).await?;
        match ids.last() {
            Some(newest) => self.view_run(*newest, control).await.map(Some),
            None => Ok(None),
        }
    }

    /// @emoji 📜️ Every live (non-tombstoned) `(key, value)` whose key starts with `prefix`, ascending by
    /// key — runs searched in place newest-first, each key decided by its newest occurrence, only the
    /// live matches materialized (at most `MAX_RUN_ENTRIES` of them).
    pub async fn scan_prefix(&self, prefix: &IndexBytes, control: &mut IndexCursorControl) -> Result<RunEntries, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut output = RunEntries::new();
        let mut dead: Vec<IndexBytes> = Vec::new();
        let mut failure = None;
        'runs: for position in (0..run_ids.len()).rev() {
            let view = match self.view_run(run_ids[position], control).await {
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

    /// @emoji 🌀️ `MergePolicy`'s enforcement after one append, bounded to ONE merge: while this kind
    /// has more runs than `policy.max_runs_before_merge`, the oldest adjacent pair among its newest
    /// `max_runs_before_merge + 1` runs whose entries fit one run is merged. Sizes come from the run
    /// ids. Tombstones are dropped only when the pair holds the kind's oldest run, since nothing older
    /// can still need shadowing. Full runs never pair, so an owner that appends full runs never merges.
    async fn merge_one_within_policy(&self, ids: &[u64], control: &mut IndexCursorControl) -> Result<(), DbError> {
        if ids.len() <= self.policy.max_runs_before_merge {
            return Ok(());
        }
        let window = ids.len() - (self.policy.max_runs_before_merge + 1);
        let candidates = &ids[window..];
        let Some(pair) = (0..candidates.len() - 1).find(|pair| entries_of_run_id(candidates[*pair]) + entries_of_run_id(candidates[*pair + 1]) <= MAX_RUN_ENTRIES) else { return Ok(()) };
        control.grant()?;
        self.merge_adjacent(candidates[pair], candidates[pair + 1], window + pair == 0, control).await
    }

    /// @emoji 🧹️ Folds this kind's runs into the fewest runs `MAX_RUN_ENTRIES` allows: from the oldest,
    /// every adjacent pair that fits one run is merged (tombstones dropped wherever the pair holds the
    /// oldest run, and from a lone oldest run). Returns the post-compaction `stats()`.
    pub async fn compact(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let mut position = 0usize;
        loop {
            let run_ids = self.kind_run_ids(control).await?;
            let pair = (position + 1 < run_ids.len()).then(|| (run_ids[position], run_ids[position + 1]));
            let oldest = run_ids.first().copied();
            let Some((older, newer)) = pair else {
                if let Some(oldest) = oldest {
                    self.drop_run_tombstones(oldest, control).await?;
                }
                break;
            };
            if entries_of_run_id(older) + entries_of_run_id(newer) <= MAX_RUN_ENTRIES {
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
        let encoded = if picks.is_empty() {
            Ok(None)
        } else {
            match make_run_id(self.kind, sequence_of_run_id(run_id), picks.len()) {
                Ok(rewritten) => encode_run_from_views(self.kind, [&view, &view], &picks, control).await.map(|pages| Some((rewritten, pages))),
                Err(error) => Err(error),
            }
        };
        view.close()?;
        match encoded? {
            Some((rewritten, pages)) => {
                self.storage.write_run(&self.document, rewritten, pages).await?;
                if rewritten != run_id {
                    self.storage.delete_run(&self.document, run_id).await?;
                }
                Ok(())
            }
            None => self.storage.delete_run(&self.document, run_id).await,
        }
    }

    /// @emoji 📊️ Current shape of this kind's runs — see `IndexStats`'s doc for what `entry_count`
    /// does and doesn't count. Run and entry counts come from the listing; bytes from each run.
    pub async fn stats(&self, control: &mut IndexCursorControl) -> Result<IndexStats, DbError> {
        let run_ids = self.kind_run_ids(control).await?;
        let mut total_bytes = 0u64;
        for run_id in &run_ids {
            control.grant()?;
            let bytes = self.storage.read_run(&self.document, *run_id).await?;
            total_bytes += bytes.len() as u64;
            close_run_pages(bytes)?;
        }
        Ok(IndexStats { run_count: run_ids.len(), entry_count: run_ids.iter().map(|id| entries_of_run_id(*id)).sum(), total_bytes })
    }

    /// @emoji ✅️ Fully decodes (checksum + structural validation) every live run for this kind,
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
    }
}
