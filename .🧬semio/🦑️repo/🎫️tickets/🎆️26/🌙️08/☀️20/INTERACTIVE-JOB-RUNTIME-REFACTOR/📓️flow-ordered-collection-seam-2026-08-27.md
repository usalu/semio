# Byte-Bounded Ordered Collection Construction

## Observed Boundary

Flow's retained source copier uses immutable Arc-rooted native iterators. Copying a string or dictionary value is byte-budgeted, but destination `BTreeMap::insert` still compares complete keys synchronously. A valid 16 KiB scene can contain two keys with a common prefix larger than the production 4,096-byte grant. Copy credit cannot pay for that later comparison. `range(last_key)`, `nth`, collecting all entries, and reducing valid key lengths do not solve it.

The standard library exposes no checked ordered-append operation that constructs a `BTreeMap` without comparisons, and its internal nodes must not be accessed unsafely. Therefore a strict replacement needs a new owned collection representation at the typed field boundary; an app-side builder that eventually returns a standard BTreeMap still hides final whole-collection work.

## Proposed Minimal Domain-Neutral API

The approved initial implementation uses a persistent AVL tree as the smallest fixed-fanout ordered representation. Each node stores pointer-sized Arc slots for its entry and children; each entry stores Arc slots for its string key and value. Arc-sharing is O(1), and one retained rebuild phase copies one AVL spine level. A balance phase may allocate the fixed maximum of three pointer-only nodes, independent of content size. Native rank iteration remains ordered and never reads string bytes. Serializer interfaces emit the same JSON object shape. Non-retained convenience APIs may perform a complete operation, but retained construction uses only explicit cursors:

- `OrderedMap::new()` creates an empty root with no payload traversal.
- `begin_set(key: String, value: V)` and `begin_remove(key)` move pending ownership into an `UpdateCursor` and share the old root.
- `advance(grant)` alternates one byte from each comparison operand, so a one-byte grant always progresses. It reports exact read bytes; lengths and fixed structural phases report zero bytes and one item. Once located, one bounded AVL-level mutation advances the path copy. It never performs a second full-key comparison.
- `take_result()` transfers the already-built immutable root only after completion. It cannot sort, collect, or insert.
- `iter()`, `len()`, and borrowed `get` provide ordinary views. Retained lookup has its own `advance(grant)` rather than invoking ordinary `get` inside a job.
- `begin_close` and `close_step` transfer one node/entry/value owner at a time and drain string bytes under the supplied grant. `RetirementStep::OwnedValue(V)` hands final payload ownership to the domain retirement cursor. Dropping the last shared subtree never recursively destroys it within a single close step.

Pure copies call `OrderedMap::clone` and share the exact immutable root. No ordered-source witness or append is needed. Updates still require real comparison cursors.

## Adoption and Tests

First replace the private neural Dictionary backing through its existing owned iterator/pop API and retain JSON equality against `std::collections::BTreeMap` as the external oracle. Flow GUI/layout maps also need the same typed collection, or an immutable shared-root representation; leaving either as a standard destination BTreeMap preserves the hole. Prefer Arc-sharing unchanged scene/map subtrees instead of duplicating them. No wide model change has been made by this executor.

Committed fixtures and authored Rust tests include adjacent 8 KiB equal-prefix keys, Unicode ordering, all AVL rotation and successor-removal shapes, 1/64/4,096-byte grants, cancellation during comparison, worker transfer, and terminal last-owner retirement. Independent oracles are std BTreeMap plus serde JSON in native tests, and existing third-party `fast-json-stable-stringify` with Buffer UTF-8 ordering in source fixtures. Native tests remain unrun. Any final `collect`, hidden comparison, or whole-map drop inside a retained path fails the boundedness claim.

## Explicit Ownership and Lookup Revision

The root review correctly rejected implicit recursive tree destruction. `OrderedMap`, `UpdateCursor`, `LookupCursor`, and `Retirement` now keep live roots/state in `ManuallyDrop` and require terminal emptiness before ordinary destruction. A contract violation panics without destroying the retained graph; an already-unwinding thread preserves that graph without a secondary panic. This is a strict misuse guard, not successful cleanup. Authored negative tests deliberately leak four tiny invalid ownership graphs to verify that payload destructors do not run.

`begin_set`, `begin_remove`, and `begin_lookup` preserve the caller's exact immutable root and retain an additional Arc root. Their new inputs move into the cursor. Result transfer leaves cursor-owned aliases alive until explicit close; removed-value transfer returns an Arc whose domain retirement belongs to the recipient. Close drains path nodes, successor nodes, source/result roots, entry/key/value aliases, and byte buffers. Final payload ownership is handed back as `OwnedValue(V)`, never recursively dropped by a retained step. That variant is one owner transfer; the recipient must charge its own domain retirement separately.

`LookupCursor` alternates one byte from each operand and retains comparison position, so a one-byte grant progresses and a pair of 8,193-byte equal keys accounts exactly 16,386 bytes. Its result is borrowed from its immutable retained root. Zero-item and zero-byte grants cannot mutate it. Found, missing, canceled, and final-owner lookup scenarios are in the shared fixture and authored native tests.

Every created AVL node checks `MAX_AVL_HEIGHT = 2 * usize::BITS`. Rank iteration therefore visits at most that fixed number of metadata nodes without string comparisons. A retained consumer accounts each `next` or `next_back` as one fixed metadata item and separately accounts any copied key/value bytes. Tests audit every subtree's height, balance, and size under sorted construction and 4,096 mixed upsert/removal operations.

Ordinary `get`/`contains_key`, `insert`/`remove`, `FromIterator`, serde, equality, and debug rendering are cold synchronous APIs: none authorizes interactive credit. Cold updates explicitly retire displaced roots and drain their cursor; failed serde decoding explicitly retires its partial root. No ordinary recursive tree drop is used as a successful path.

Adoption remains paused pending coordinator native validation. Framework Flow field adoption and shared fixture copy/retirement will belong to the Generator executor; neural Dictionary adoption will belong to this executor after the ownership seam is approved. The Flow retained artifact factory is still unfinished and continues to use its old monolithic preparation path; the new primitive does not by itself migrate those routes.

The canonical source target passed after this revision: `@semio-tech/framework-replication-rs:test-source`, with three ordered-operation fixtures, two long-key lookup fixtures, eight strict hostile rejections, and the third-party canonical serialization oracle. Log: `🧪️ordered-map-source-guards-2026-08-27.txt`. Eleven native laws are authored but not run by this executor. `git diff --check` passed. The authoritative launch seed now declares Flow check/source and ordered-map native/source gates; registry regeneration is recorded separately. No native compiler lease was used here.

## Coordinator Native Approval and Adoption Checkpoint

The coordinator subsequently ran the initial eleven native map laws successfully, then the expanded `value::ordered` filter: **16 passed, zero failed, 192 filtered**, 9.97 seconds compile and 1.46 seconds test. The expanded filter includes the original eleven, three new shared-ownership laws, and two OrderedSet laws authored by the Generator executor. Adoption is now approved; earlier paused/unrun descriptions above record the preceding checkpoint only.

`release_shared(self)` atomically consumes the root with `Arc::into_inner`. A shared root returns success in constant ownership work. The exact final root returns guarded `Retirement<V>` containing its entry and up to two children; it does not traverse or destroy payloads. Eight concurrent owners produce exactly seven shared releases and one final handoff. Long-key retirement reports all 16,384 key bytes under grants 1, 64, and 4,096. Empty ownership is a successful no-op.

`begin_set_shared(Arc<String>, Arc<V>)`, `begin_remove_shared(Arc<String>)`, and `begin_lookup_shared(Arc<String>)` preserve exact key/value allocations. The ordinary by-value set constructor explicitly requires bounded inline-V admission; arbitrary large inline V is not byte-free. `entry_at_rank` exposes the existing fixed-height metadata lookup without comparisons. Runtime latest-wins admission uses these shared inputs, not cloned or hash-only identities.

The coordinator also reran the source oracle successfully: map three fixtures plus two lookup cases/eight hostile rejections; shared ownership one fixture/two hostile rejections; OrderedSet one fixture/two hostile rejections. The existing third-party stable serializer validates JSON output, with Node Buffer UTF-8 ordering. These source checks are not runtime tests.

Dictionary backing has now adopted OrderedMap with strict final-owner guards, an explicit domain retirement cursor, and domain-aware cold construction/decoding. FlowWorkingScene layout and the app source-copy map/set/dictionary branches now share immutable roots rather than reinserting long keys. Live Artifact recipes remain unfinished: this infrastructure approval is not a claim that their monolithic preparation has migrated.
