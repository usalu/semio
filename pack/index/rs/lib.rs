//! 📦 `pack_index` — the `FieldIndex` segment (`KIND_FIELD_INDEX` = `0x08`): maps a field-id
//! path from the document record root to the `ByteRange` of that field's encoded value inside
//! the Document segment(s), enabling sub-document random access without a full decode. This
//! crate is a standalone segment codec — root-eager (the whole sorted table is loaded on
//! `open`), leaf lookups are a binary search. It does not hook into `pack_value::encode_document`
//! / `decode_document`; a future caller wires a `FieldIndexBuilder` in around those calls.

use pack_core::{ByteRange, PackError, read_varint_u64, write_varint_u64};

//#region 🔖FieldIndex
/// @emoji 🧭 A sequence of field ids from the record root down to a leaf value, e.g. `[3, 0, 12]`
/// for "field 3, then its field 0, then its field 12". The empty path denotes the record root.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldPath(pub Vec<u16>);

/// @emoji 📌 One row of the `FieldIndex`: a path paired with the byte range of its encoded value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldIndexEntry {
    pub path: FieldPath,
    pub range: ByteRange,
}

/// @emoji 🏗️ Accumulates `(path, range)` entries during encoding, then serializes them into a
/// `FieldIndex` segment payload sorted by path (lexicographic on the field-id sequence — the
/// same ordering `FieldPath`'s derived `Ord` gives), so a reader can binary-search on `open`.
pub struct FieldIndexBuilder {
    entries: Vec<FieldIndexEntry>,
}

impl FieldIndexBuilder {
    /// @emoji 🌱 Starts an empty builder.
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// @emoji ➕ Records one field's location. Later calls with the same `path` shadow earlier
    /// ones once sorted, since `build` does not deduplicate by insertion order — callers should
    /// record each path at most once.
    pub fn record(&mut self, path: FieldPath, range: ByteRange) {
        self.entries.push(FieldIndexEntry { path, range });
    }

    /// @emoji 📤 Serializes the accumulated entries into a `FieldIndex` segment payload:
    /// `count: varint`, then `count × (path_len: varint, path field ids: varint*, offset: varint,
    /// len: varint)`, sorted ascending by path so `FieldIndexReader::lookup` can binary search.
    pub fn build(mut self) -> Vec<u8> {
        self.entries.sort_by(|a, b| a.path.cmp(&b.path));
        let mut buf = Vec::new();
        write_varint_u64(&mut buf, self.entries.len() as u64);
        for entry in &self.entries {
            write_varint_u64(&mut buf, entry.path.0.len() as u64);
            for &field_id in &entry.path.0 {
                write_varint_u64(&mut buf, field_id as u64);
            }
            write_varint_u64(&mut buf, entry.range.offset);
            write_varint_u64(&mut buf, entry.range.len);
        }
        buf
    }
}

impl Default for FieldIndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// @emoji 🔍 Borrows a raw `FieldIndex` segment payload and offers exact-path lookup. `open`
/// eagerly parses every entry into an owned, sorted `Vec` (the "root-eager, small header" shape
/// the contract calls for) so `lookup` is a plain binary search with no further allocation.
pub struct FieldIndexReader<'a> {
    payload: &'a [u8],
    entries: Vec<FieldIndexEntry>,
}

impl<'a> FieldIndexReader<'a> {
    /// @emoji 📂 Parses `payload` fully, validating every varint and length so a truncated or
    /// corrupted payload returns `Err` rather than panicking or reading out of bounds.
    pub fn open(payload: &'a [u8]) -> Result<Self, PackError> {
        let mut pos = 0usize;
        let count = read_varint_u64(payload, &mut pos)?;
        let mut entries = Vec::with_capacity((count as usize).min(1 << 20));
        for _ in 0..count {
            let path_len = read_varint_u64(payload, &mut pos)?;
            let mut field_ids = Vec::with_capacity((path_len as usize).min(1 << 20));
            for _ in 0..path_len {
                let field_id = read_varint_u64(payload, &mut pos)?;
                if field_id > u16::MAX as u64 {
                    return Err(PackError::Malformed {
                        what: "field_index_path_id",
                        offset: pos as u64,
                        detail: format!("field id {field_id} exceeds u16 range"),
                    });
                }
                field_ids.push(field_id as u16);
            }
            let offset = read_varint_u64(payload, &mut pos)?;
            let len = read_varint_u64(payload, &mut pos)?;
            entries.push(FieldIndexEntry { path: FieldPath(field_ids), range: ByteRange { offset, len } });
        }
        Ok(Self { payload, entries })
    }

    /// @emoji 🧾 The raw payload this reader was opened from.
    pub fn payload(&self) -> &'a [u8] {
        self.payload
    }

    /// @emoji 🎯 Exact-path lookup via binary search over the sorted entries; `Ok(None)` when no
    /// entry matches `path`, never a panic.
    pub fn lookup(&self, path: &FieldPath) -> Result<Option<ByteRange>, PackError> {
        match self.entries.binary_search_by(|entry| entry.path.cmp(path)) {
            Ok(index) => Ok(Some(self.entries[index].range)),
            Err(_) => Ok(None),
        }
    }
}
//#endregion 🔖FieldIndex

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<FieldIndexEntry> {
        vec![
            FieldIndexEntry { path: FieldPath(vec![]), range: ByteRange { offset: 0, len: 100 } },
            FieldIndexEntry { path: FieldPath(vec![1]), range: ByteRange { offset: 4, len: 10 } },
            FieldIndexEntry { path: FieldPath(vec![3]), range: ByteRange { offset: 20, len: 5 } },
            FieldIndexEntry { path: FieldPath(vec![3, 0]), range: ByteRange { offset: 21, len: 2 } },
            FieldIndexEntry { path: FieldPath(vec![3, 0, 12]), range: ByteRange { offset: 22, len: 1 } },
            FieldIndexEntry { path: FieldPath(vec![3, 1]), range: ByteRange { offset: 23, len: 2 } },
            FieldIndexEntry { path: FieldPath(vec![50000]), range: ByteRange { offset: 30, len: 8 } },
        ]
    }

    #[test]
    fn build_serialize_reopen_lookup_returns_correct_ranges() {
        let mut builder = FieldIndexBuilder::new();
        for entry in sample_entries() {
            builder.record(entry.path, entry.range);
        }
        let payload = builder.build();

        let reader = FieldIndexReader::open(&payload).unwrap();
        for entry in sample_entries() {
            let found = reader.lookup(&entry.path).unwrap();
            assert_eq!(found, Some(entry.range), "path {:?} should resolve", entry.path);
        }
    }

    #[test]
    fn lookup_nonexistent_path_returns_none() {
        let mut builder = FieldIndexBuilder::new();
        builder.record(FieldPath(vec![1]), ByteRange { offset: 4, len: 10 });
        builder.record(FieldPath(vec![3, 0]), ByteRange { offset: 21, len: 2 });
        let payload = builder.build();
        let reader = FieldIndexReader::open(&payload).unwrap();

        assert_eq!(reader.lookup(&FieldPath(vec![2])).unwrap(), None);
        assert_eq!(reader.lookup(&FieldPath(vec![1, 0])).unwrap(), None);
        assert_eq!(reader.lookup(&FieldPath(vec![3])).unwrap(), None);
        assert_eq!(reader.lookup(&FieldPath(vec![])).unwrap(), None);
    }

    #[test]
    fn build_output_is_sorted_by_path_regardless_of_insertion_order() {
        let mut builder = FieldIndexBuilder::new();
        builder.record(FieldPath(vec![3, 1]), ByteRange { offset: 23, len: 2 });
        builder.record(FieldPath(vec![1]), ByteRange { offset: 4, len: 10 });
        builder.record(FieldPath(vec![]), ByteRange { offset: 0, len: 100 });
        builder.record(FieldPath(vec![3]), ByteRange { offset: 20, len: 5 });
        let payload = builder.build();

        let mut pos = 0usize;
        let count = read_varint_u64(&payload, &mut pos).unwrap();
        assert_eq!(count, 4);
        let mut previous: Option<FieldPath> = None;
        for _ in 0..count {
            let path_len = read_varint_u64(&payload, &mut pos).unwrap();
            let mut ids = Vec::new();
            for _ in 0..path_len {
                ids.push(read_varint_u64(&payload, &mut pos).unwrap() as u16);
            }
            let _offset = read_varint_u64(&payload, &mut pos).unwrap();
            let _len = read_varint_u64(&payload, &mut pos).unwrap();
            let path = FieldPath(ids);
            if let Some(prev) = &previous {
                assert!(prev < &path, "entries must be strictly increasing by path");
            }
            previous = Some(path);
        }
    }

    #[test]
    fn empty_index_round_trips() {
        let builder = FieldIndexBuilder::new();
        let payload = builder.build();
        let reader = FieldIndexReader::open(&payload).unwrap();
        assert_eq!(reader.lookup(&FieldPath(vec![0])).unwrap(), None);
    }

    #[test]
    fn open_empty_payload_errors_not_panics() {
        let result = FieldIndexReader::open(&[]);
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }

    #[test]
    fn open_truncated_count_only_errors_not_panics() {
        let mut builder = FieldIndexBuilder::new();
        builder.record(FieldPath(vec![1, 2]), ByteRange { offset: 4, len: 10 });
        let payload = builder.build();
        // Truncate to just the count varint (claims 1 entry, but no bytes follow).
        let truncated = &payload[..1];
        let result = FieldIndexReader::open(truncated);
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }

    #[test]
    fn open_truncated_mid_entry_errors_not_panics() {
        let mut builder = FieldIndexBuilder::new();
        builder.record(FieldPath(vec![1, 2, 3]), ByteRange { offset: 4, len: 10 });
        let payload = builder.build();
        for cut in 1..payload.len() {
            let truncated = &payload[..cut];
            let result = FieldIndexReader::open(truncated);
            assert!(result.is_err(), "truncation at byte {cut} should error, not panic");
        }
    }

    #[test]
    fn open_corrupted_count_claims_more_entries_than_bytes_support() {
        // count = 0xFFFFFFFF (way more entries than the remaining bytes could hold).
        let mut payload = Vec::new();
        write_varint_u64(&mut payload, u32::MAX as u64);
        let result = FieldIndexReader::open(&payload);
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }

    #[test]
    fn open_field_id_exceeding_u16_range_is_malformed() {
        let mut payload = Vec::new();
        write_varint_u64(&mut payload, 1); // count = 1
        write_varint_u64(&mut payload, 1); // path_len = 1
        write_varint_u64(&mut payload, u32::MAX as u64); // field id way over u16::MAX
        write_varint_u64(&mut payload, 0); // offset
        write_varint_u64(&mut payload, 0); // len
        let result = FieldIndexReader::open(&payload);
        assert!(matches!(result, Err(PackError::Malformed { .. })));
    }

    #[test]
    fn deep_and_varied_depth_paths_all_resolve() {
        let mut builder = FieldIndexBuilder::new();
        let deep_path = FieldPath((0..40).map(|i| i as u16).collect());
        builder.record(deep_path.clone(), ByteRange { offset: 1000, len: 4 });
        builder.record(FieldPath(vec![0]), ByteRange { offset: 1, len: 1 });
        builder.record(FieldPath(vec![0, 0]), ByteRange { offset: 2, len: 1 });
        builder.record(FieldPath(vec![0, 0, 0]), ByteRange { offset: 3, len: 1 });
        let payload = builder.build();
        let reader = FieldIndexReader::open(&payload).unwrap();
        assert_eq!(reader.lookup(&deep_path).unwrap(), Some(ByteRange { offset: 1000, len: 4 }));
        assert_eq!(reader.lookup(&FieldPath(vec![0, 0])).unwrap(), Some(ByteRange { offset: 2, len: 1 }));
    }
}
//#endregion 🧪Tests
