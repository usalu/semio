#!/usr/bin/env python3
"""♻ H14 set "retire-pages" (kernel `🏪️store`, guest-linked → first post-chain train): LW1's red law
`production_snapshot_read_lease_survives_multiturn_copy_and_bounded_cancellation` ("returned large snapshot retirement remains
bounded") is not a livelock but O(bytes) turns — a `Vec<T>` retired one ELEMENT per step even when `T` has no drop glue, so the
fixture's 2 MiB `Vec<u8>` payload needed ~6.3 M pump turns (child box, one byte, pop) at the pump's one item per turn: a large
byte buffer of a returned snapshot stayed resident for millions of maintenance turns. Root fix: a collection of a type without
drop glue releases a page per step (`maximum_bytes / size_of::<T>()` elements, like `String`'s bytes). Laws: the neutral
fixture gains a byte buffer and a word list (schema 9 → 11 cases); a 2 MiB byte buffer retires in ≤ 36 one-item steps of 64 KiB.
LW1's second red (`retained_paged_list_copy_matches_vec_serde_and_closes_page_by_page`, `close_turns > 1`) is the law's error: a
cursor whose output was taken holds only its source binding and closes in exactly one step (the copied list's page-by-page
retirement is the same law's `retirement_turns > 1`) → `close_turns == 1`. Idempotent, region-guarded; `--dry-run` reports."""
import json
import sys

R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
SRC = f"{R}/♻️retirement/🦀️.rs"
TESTS = f"{R}/♻️retirement/🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = f"{R}/♻️retirement/🧫️fixtures/🔣️.json"
SCHEMA = f"{R}/♻️retirement/🧬️schema/🔣️.json"
PAGED = f"{R}/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (SRC, TESTS, FIXTURE, SCHEMA, PAGED)}
problems, states = [], []


def edit(path, old, new, label):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit(SRC, """struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }""", """/// ♻️ A vector retires element by element, each through its own retirement, unless its elements have no drop glue: then
/// nothing but their memory is released and a step releases a page of them at once (`maximum_bytes / size_of::<T>()`), so a
/// large byte buffer or number array costs its size over the grant in steps instead of three steps per element (a returned
/// 2 MiB `Vec<u8>` took ~6.3 M one-item pump turns). A grant narrower than one element retires that element on its own.
struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if !std::mem::needs_drop::<T>() && !self.0.is_empty() {
            let width = size_of::<T>();
            if width == 0 {
                self.0.clear();
                return RetirementStep::Bytes(0);
            }
            let count = (maximum_bytes / width).min(self.0.len());
            if count > 0 {
                let next = self.0.len() - count;
                self.0.truncate(next);
                return RetirementStep::Bytes(count * width);
            }
        }
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }""", "collection pages")

edit(TESTS, """    assert_eq!(fixture["cases"].as_array().unwrap().len(), 9);""", """    assert_eq!(fixture["cases"].as_array().unwrap().len(), 11);""", "case count")

edit(TESTS, """                "value" => owned_retirement(crate::os_pack::json::from_json_str::<crate::DslValue>(&row["value"].to_string()).unwrap()),""", """                "value" => owned_retirement(crate::os_pack::json::from_json_str::<crate::DslValue>(&row["value"].to_string()).unwrap()),
                "bytes" => owned_retirement(serde_json::from_value::<Vec<u8>>(row["value"].clone()).unwrap()),
                "words" => owned_retirement(serde_json::from_value::<Vec<u32>>(row["value"].clone()).unwrap()),""", "fixture kinds")

edit(TESTS, """#[test]
fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {""", """/// ♻️ A collection without drop glue retires a page per step: a 2 MiB byte buffer under one item and 64 KiB per step releases
/// exactly its bytes in at most 36 steps (32 pages, the push, the pop, the root), a grant narrower than one element still
/// completes, and a list of strings still retires string by string.
#[test]
fn a_byte_buffer_retires_page_by_page_and_owned_elements_one_by_one() {
    let mut steps = 0usize;
    let mut released = 0usize;
    let mut retirement = owned_retirement(vec![7u8; 2 * 1024 * 1024]);
    loop {
        steps += 1;
        assert!(steps <= 36, "a 2 MiB buffer needs at most 36 steps of 64 KiB");
        match retirement.close_step(1, 64 * 1024).unwrap() {
            SnapshotRetirementStep::Pending { released_bytes, .. } => released += released_bytes,
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Blocked => panic!("an owned buffer never blocks"),
        }
    }
    assert_eq!(released, 2 * 1024 * 1024);
    assert!(retirement.terminal_is_empty());
    assert_eq!(drain(owned_retirement(vec![1u32, 2, 3]), 1, 4), 12);
    assert_eq!(drain(owned_retirement(vec![1u32, 2, 3]), 1, 3), 12, "a grant narrower than one element retires it element by element");
    assert_eq!(drain(owned_retirement(vec!["ab".to_string(), "c".to_string()]), 1, 1), 3);
}

#[test]
fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {""", "page law")

edit(FIXTURE, """      "bytes": 22
    }
  ],
  "budgets": [""", """      "bytes": 22
    },
    {
      "id": "byte-buffer",
      "kind": "bytes",
      "value": [""" + ", ".join(str(index % 256) for index in range(3072)) + """],
      "bytes": 3072
    },
    {
      "id": "word-list",
      "kind": "words",
      "value": [1, 65536, 4294967295],
      "bytes": 12
    }
  ],
  "budgets": [""", "fixture cases")

edit(SCHEMA, """          "minItems": 9,
          "maxItems": 9,""", """          "minItems": 11,
          "maxItems": 11,""", "schema count")

edit(SCHEMA, """                  "stringMap",
                  "value"
                ]""", """                  "stringMap",
                  "value",
                  "bytes",
                  "words"
                ]""", "schema kinds")

edit(SCHEMA, """                  {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  }
                ]""", """                  {
                    "type": "array",
                    "items": {
                      "type": "string"
                    }
                  },
                  {
                    "type": "array",
                    "items": {
                      "type": "integer",
                      "minimum": 0,
                      "maximum": 4294967295
                    }
                  }
                ]""", "schema integer arrays")

edit(PAGED, """    assert!(close_turns > 1);
    let mut retirement = crate::os_store::retirement::owned_retirement(copied);""", """    assert_eq!(close_turns, 1, "a spent cursor whose output was taken holds only its source binding and closes in one step");
    let mut retirement = crate::os_store::retirement::owned_retirement(copied);""", "paged close law")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
