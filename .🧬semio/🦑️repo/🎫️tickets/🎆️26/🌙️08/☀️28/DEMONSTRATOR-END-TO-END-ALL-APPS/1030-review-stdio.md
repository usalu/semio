# 1030 review: stdio standards + DSL/record-pack (read-only, no builds)

Scope read: framework pack decode (`record-body-cursor`, `receiving`, `document`), `dsl` scalar-enum derive, `dsl-record` pair binding, `s.stdio.semio` base (`SemioRecordPack`, `semio_snapshot_record_pack!`, `semio_member_owner!`), the sqlite native migration (value, brep samples), generation3d `phased-job`, the Python/TS `record-document` codec. Most severe first. Nothing here was compiled or run.

## 1. HIGH (conf 0.75) sqlite binary path lost its pre-construction admission census and encode accounting; owner: resid (semiodsl)
- `.../🧿️semio/.../🪆️subsets/*/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs` (`binary_into(.., _limits)` in all 17 subsets, e.g. value, brep) and `.../✉️base/🦀️.rs:143-176`.
- The removed `admit_binary`/`semantic::binary` counted entities/rows against `SqliteDatabaseLimits` BEFORE typed construction. The replacement ignores `limits` (`_limits`) and only enforces `PackDecodeOptions::default()` limits plus `NativeDecodeControl`. Hostile SQLite blob: row/entity ceilings that the relational census enforced no longer apply to the binary encoding; only pack `max_total_alloc` does.
- Encode side (`✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛫️native/🦀️.rs:62` `record_binary`): `writer.entities(1)` charges one entity for the whole snapshot, and `snapshot.record_binary()` allocates the complete document `Vec` outside `SqliteSnapshotControl` (uncharged, uncancellable) before `writer.bytes` copies it a second time. A multi-MiB snapshot is never priced.
- Fix: thread `limits` into `from_record_binary` (census from `RecordValue` row count before `__dsl_from_record_controlled`), charge `record_binary` length to the writer control before allocation (or stream via the controlled writer).

## 2. MEDIUM (conf 0.7) error kind laundering drops cancellation/limit kinds; owner: resid (semiodsl)
- `✉️base/🦀️.rs:167-176` (`record_binary` and `from_record_binary` in `semio_snapshot_record_pack!`): `.map_err(|e| ValueError::new(ValueRefusalKind::InvalidValue, e.to_string()))`.
- `PackRefusal::into_value_error` (`🎒️pack/🪶️refusal/🦀️.rs:46`) keeps `kind()`. Scenario: a caller cancels (or `max_total_alloc` trips -> OwnershipLimit) during `decode_document_controlled`; the sqlite caller sees `InvalidValue` "corrupt data" instead of Cancelled/limit, so cancellation is treated as a permanent corrupt row (wrong retirement/retry path).
- Fix: use `error.into_value_error()` (and `PackError` equivalent) in both closures.

## 3. MEDIUM (conf 0.55) close turns report item progress the grant never funded; owner: fwfix
- `🎒️pack/🌱️value/🛬️decode/📦️document/🦀️.rs:29` (`partial_close`) and `:157` (final `Complete(copied_items:1)`).
- `partial_close` maps a sub-owner `Complete` to `copied_items:1` regardless of `grant.maximum_items`. The admission gate (`retained_clone_grant_admits(grant, demand)`) is demand-driven; the `RetirementDemand{depth:1}` terminal quote carries no item requirement, so a grant with `maximum_items == 0` can be admitted and then reports 1 item, exceeding its own ceiling (the exact fabrication the new receiving test `..close_never_reports_progress_without_an_item_grant` forbids, but that test only drives the Body cursor path, not the Document driver). If the turn accounting asserts progress <= grant this is a panic; if not it is a silent over-charge.
- Same pattern in `📜️record-body-cursor/🦀️.rs:~533` (SymbolsLogical -> ValuePhysical) is guarded by `maximum_items == 0` and is fine; only the Document driver lacks the guard.
- Fix: add `if grant.maximum_items==0 {return Ok(Progress(default))}` before the `partial_close`/terminal branches (as the `ids` branch already does), add a Document-driver twin of the starved-close test.

## 4. MEDIUM (conf 0.5) export step may do zero work in a turn; owner: gen
- generation3d `🩹️..../⏱️phased-job/🦀️.rs:284`: `while ... && !cx.should_yield()`. The old code did `fuel.max(1)` (at least one export per call); the job steps keep that guarantee via `fuel_budget` (`.max(1)`), the export step does not. If a turn starts with the context already yielding (previous step spent the last fuel but `Flow::Next` loop in `Pipeline::step` re-enters before the yield check, or a caller grants fuel 0) the step returns `Flow::Again` having exported nothing: zero-progress turn, livelock when the caller re-grants identical fuel.
- Also `spend(cx, if matches!(step, Working) { budget } else { 1 })` charges a finished multi-unit job step only 1 unit, undercounting real work (fairness, not correctness).
- `catch_unwind(AssertUnwindSafe(|| step(work, cx)))` now captures a `&mut StepContext`; a kernel panic can leave fuel accounting half-updated and the context is then reused for the fault path.

## 5. LOW (conf 0.85) decode oracle accepts what Rust rejects; owner: playbook
- `🧿️semio/🚪️io/💾️binary/🎒️record-document/🐍️.py:104-113` (`varint`) and `🟦️.ts:199` (`varint()` = `Number(varintBig())`): the 10th byte payload is not limited to `<= 1`; Rust `read_varint_u64` (`📡️replication/⚙️codec/🦀️.rs:127-146`) rejects it. A stream with a >64-bit varint decodes in Python/TS but is refused by Rust, so the "cross-implementation oracle" does not prove refusal parity. TS `Number()` additionally truncates ids/lengths above 2^53 silently (only `count()` is bounds-checked).
- No cap on total inflated bytes across segments (only 64 MiB per segment, `MAX_SEGMENT_RAW`); Rust enforces `max_total_alloc`. Many segments => unbounded memory in the oracle. Test-only tooling, hence low.
- Controlled Rust decoder rejects trailing document-body bytes (`🚪️document-codec/🦀️.rs:~101`), the uncontrolled `decode_document` (line ~83) does not: the `.pack.semio` open path accepts trailing garbage that the sqlite path refuses. Asymmetric; fix by moving the `reader.remaining()!=0` check into the shared tail.

## 6. LOW (conf 0.9) leftovers / policy; owner: policy
- `🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🦀️.rs` (last test) `fn debug_size_probe(){eprintln!("[DEBUG] sizes ...")}` is a committed probe that is not a test; AGENTS.md requires removal.
- 542 files drop `#![allow(long_running_const_eval)]` (deny-by-default lint). Verified green only for gltf/dwg/pdf/docx (stdiofix); epw, zip, step, mp3, mp4, bcf, jpg, pptx, wav, tiff, bmp and the fem/hub crates were not re-checked after removal. Any const-eval-heavy schema table there fails to compile.
- `record_binary` ignores caller `PackEncodeOptions` (always default), while `encode_pack_with` honors them; Python adapters for graph/drawing/kit need `omit_nulls=True`, so sqlite blobs and `.pack.semio` for the same snapshot can differ byte-wise (both decode; only digest-based cross-checks would notice).

## Checked, no defect found
- Scalar-enum derive (`✨️derive/🪢️receiving-and-projection:22-31`) ordinals equal `DslScalar` ordinals (`🚦️emitters:104-116`, declaration order in both); empty/non-unit enums fall through to the old path.
- `(T,T)` pair binding: shape `Tuple(_, Some(2))`, `from_value` requires exactly 2 items, controlled decode retires `first` when `second` fails.
- Record-body cursor close: every funded phase transition now charges one item; `maximum_items==0` returns zero-progress only (no fabrication); the zero-progress `Pending` at `ValueLogical`/`SymbolsLogical` fires only when a release allocation is pending, which the receiving layer gates through `source_close_demands`.
- `RetainedRecordReceiving::admit` sizes the symbol table by `min(length, max_symbols)`, so allocation is bounded by input length.
- Orphan-impl moves (docx strict `OpText/OpBinary`, pptx-strict, flow diff codec): each trait impl now lives in exactly one crate (the mutations/diff crate that owns the type); I found no remaining second impl, so selection is unchanged.
