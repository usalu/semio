# S4-PACKFIX: OS Kernel Back on the New Pack Error API

The kernel library `semio-framework-os-kernel` now compiles on native and on `wasm32-wasip2` against the new `semio-framework-pack-error` API (`PackError{Refusal(PackRefusal),TransportFailure(OwnedTransportError)}`). Its four immediate dependents also compile. Lib tests (`--tests`) were out of scope and were not run.

Before editing, I read the peer direction documents in `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/`:

- core-pack-error-context map
- thirty-seven typed-error prerequisites
- OS-326 test-only census
- store-command canonical refusal TDD

## Baseline

`cargo check -p semio-framework-os-kernel --lib --message-format=short`, run 19:52:18 to 19:52:51, exit 101, **221 errors**. That is down from the 243 in the brief, because a peer had already moved Store from 27 errors to 5.

| File | Errors |
| --- | ---: |
| `🎒️pack/🌱️value/🦀️.rs` | 158 |
| `📡️spr/📜️history/🦀️.rs` | 14 |
| `📡️spr/🔌️io/🦀️.rs` | 10 |
| `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` | 9 |
| `🏪️store/🧾️document/📥️mounted-pack/🦀️.rs` | 8 |
| `🎒️pack/🌱️value/🛫️encode/🦀️.rs` | 7 |
| `🏪️store/🦀️.rs` | 5 |
| `🎒️pack/⌨️cli/🦀️.rs` | 3 |
| `🏪️store/📜️space-history/…/🪶️sqlite/🚦️native/🦀️.rs` | 2 |
| `🧰️framework/🔨️modules/🚪️io/🦀️.rs` | 2 |
| `📦️codec/🪶️snapshot-capability/🛬️native-decoding` and `🛫️native-encoding` | 1 + 1 |
| `🎒️pack/🦀️.rs` (OS facade `content_hash`) | 1 |

By kind:

- 104 + 69 missing `PackError` variants
- 10 + 3 arity errors from the new `PackTransportContext` parameter
- 9 `SprWriter` trait-bound errors
- 9 failed `?` conversions to `ValueError` or `&str`
- 5 `RetainedPackSourceFault` and `RetainedPackSourceAllocationError` shape errors

## Key Decision: The OS Value Module Follows Its Converted Framework Twin

`🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs` (mounted as `pack::record`) is a near line-for-line twin of the OS value module. The peer had already converted it to `PackRefusal`, choosing a kind at every producer site.

- **First attempt:** keep `PackError` signatures and wrap each site in `PackError::Refusal(…)`. This reached 4 errors in that file, but every caller of a pure `*_controlled` codec then needed a made-up kind for the impossible `TransportFailure` case.
- **What I did instead:** I regenerated the OS value module from the twin. Only paths and test mounts differ: kernel `crate::os_pack::` paths, the kernel's own test mounts kept, the framework-only `🫳️preflight` mount left out. The OS `🛫️encode` lost its private `OutputError` enum and now matches the twin, minus the twin-only `DocumentSource` / `intrinsic_document`. So every kind, message split and retained-fault projection is exactly the peer's choice.
- **Why:** this follows the peer's own principle, "a pure parser cannot invent transport ownership". Pure codecs and cursors now return `PackRefusal`, and callers get an infallible `PackRefusal::into_value_error`.

The resulting public API boundary:

- **Still `PackError`:** OS facade `encode_document`, `decode_document`, `encode_record_body`, `decode_record_body`, `decode_record_body_exact` and `content_hash`, via `.map_err(PackError::from)`. In `store::pack_rt`: `encode_document`, `decode_document`, `encode_record_body`, `decode_record_body` and `decode_pack_value`.
- **Now `PackRefusal`:** the facade and `pack_rt` `*_controlled` entry points, `pack_rt::decode_wire_value[_with_options]`, and everything glob-exported from the value module (`RetainedValueCursor`, `RetainedRecordBodyCursor`, `decode_value_record_body_exact`, `schema_hash_controlled`, `PackSchemaGraph::hash_controlled`). `store` / the kernel root now re-export `PackRefusal`.

## Mapping Table

| Old form | New form | Where |
| --- | --- | --- |
| `PackError::LimitExceeded("x")` | `PackRefusal::LimitExceeded{kind,limit:"x"}`, kind taken from the twin (DepthLimit, WorkLimit, OwnershipLimit, AllocationFailed, InvariantViolated, InvalidValue) | OS value, encode |
| Combined-condition `LimitExceeded` | Split per condition with distinct kinds, as in the twin (retained value depth/item credits, owner stack, record-body physical credits, symbol length) | OS value |
| `LimitExceeded` on stack allocation failure or overgrant | `PackRefusal::RetainedAllocation{…allocated_bytes…}` (twin) | OS value |
| `RetainedMalformed{what,offset,detail}` | Same plus `kind` (InvalidValue for malformed input, InvariantViolated for owner state) | OS value |
| `RetainedMalformed{offset:fault.offset,detail:fault.code}` | `fault.into_pack_refusal(what)`, which keeps the catalog cause kind and allocation witness | OS value |
| `Malformed{…}` | Same plus `kind` (InvalidValue; UnsupportedOwner for chunked octets without a catalog; InvariantViolated for admitted-length drift) | OS value, store, spr history |
| `Truncated`, `NonCanonical`, `ValueRefusal(e)`, `TextRefusal(e)` | Same `PackRefusal` variant; `map_err(PackError::ValueRefusal)` becomes `map_err(PackRefusal::from)` | everywhere |
| `PackError::into_value_error` on controlled codecs | `PackRefusal::into_value_error` | sqlite native, native decoding/encoding |
| `Result<(), &str>` close/retire contract on `RetainedValueCursor::close_step` | `PackRefusal` (twin); mounted-pack retirement uses `PackRefusal::into_value_error` (actual kind instead of the blanket InvariantViolated) | mounted-pack |
| Retained source `&'static str` faults | `RetainedPackSourceFault` → `.reason` inside the session's `&'static str` contract; allocation errors carry `RetainedPackSourceFault::refusal/allocation(kind, reason, bytes)` with the origin kind (missing owner = InvariantViolated) | mounted-pack (`mounted_pack_rt` now re-exports `RetainedPackSourceFault`) |
| `SprWriter<S>` generic helpers | `where ProtocolError: From<S::Error>`, the real conversion bound (no fake sink) | spr history `flush_dict_delta`, `impl HistoryAppender` |
| `read_id` resolvers | `PackRefusal::Malformed{kind:InvalidValue,…}` | spr history |
| File constructors without context | Caller-owned `&PackTransportContext`, cloned into every `FilePackSource` / `FilePackSink` / `write_atomic`. `HistoryFile` keeps one context for source, sink, resume and compact, and exposes `transport_context()` for sidecar writes | spr io |

### Schema Decisions

The `Schema` variant is retired. Each former site maps as follows:

| Former `Schema` site | New form | Display |
| --- | --- | --- |
| `Schema(ValueError.to_string())` (FromValue) | `PackError::from(error)`: the real ValueError and its kind | `schema error: <msg>` |
| `Schema(SemioError.to_string())` (envelope build/unwrap) | `PackError::from(error.into_value_error())`: canonical SemioError kind (InvalidValue / UnsupportedOwner / control) | unchanged |
| `"… pack envelope mismatch"` | ValueRefusal InvalidValue. This matches `unwrap_binary_controlled`, which treats an identity mismatch as an `InvalidBinaryHeader` → InvalidValue | unchanged |
| `"symbol … missing from precomputed table"`, `"manifest not loaded"`, `"controlled manifest absent"` | InvariantViolated (twin) | unchanged |
| `"chunk owner has no controlled admission"` | UnsupportedOwner (twin) | unchanged |
| `"empty state must not contain bytes"`, base64 / scene JSON / serde_json input errors | InvalidValue | unchanged |
| UTF-8 errors | `ValueError::from(Utf8Error \| FromUtf8Error)` | unchanged |

`ValueRefusal` displays as `schema error: {e}`, so all printed text stays as it was.

### Framework IO `serializer_entry`

Text refusals go through `text_refusal`. Other refusals project with `.under("native pack decode")`. A `TransportFailure` coming out of an in-memory `S::decode_pack` cannot happen by contract, so I classified it explicitly as `InvariantViolated` with the transport display kept in the message. This is an **open contract question for the peer**: `IoError` cannot carry the transport source.

## Transport Context Ownership

- **Pack crate re-exports:** the pack crate now re-exports `PackRetryDisposition`, `PackTransportCategory`, `PackTransportContext`, `PackTransportPolicy`, `PackTransportProgress`, `PackTransportPhase`, `TransportAdmission`, `TransportContextRefusal` and `TransportContextRefusalCause`. Its public io API needs them.
- **SPR io functions:** `resume_state_for`, `HistoryFile::{create,open_append,open_read_only}`, `write_sidecar`, `recover_file`, `TailFollower::open` and `compact` all take a caller-owned `&PackTransportContext`.
- **CLI:** I added a small command-owned policy. While I was working, a Codex peer replaced it with `🎒️pack/⌨️cli/🚦️control/🦀️.rs` (`admit_command_transport` with a `CancelToken`, a progress observer and stdin `cancel`). The peer threads that context through both CLIs' `main_impl` and builds on my SPR io signatures. I kept their design and only added the two missing re-exports their import needed. Their `💾️binary/🦀️.rs` entry points still call `main_impl(&args)`, which only matters under the `native-bin` feature and is part of their in-flight work.

## Other Fix Needed for wasip2

`🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🏭️schema/🦀️.rs` used `usize::wrapping_mul(0x9e3779b97f4a7c15)`, which overflows on 32-bit. I changed it to `((address as u64).wrapping_mul(…)>>7) as usize`, which gives identical results on 64-bit targets.

## Check Log

| Time | Command | Result |
| --- | --- | --- |
| 19:52:18 to 19:52:51 | kernel `--lib` (baseline) | 221 errors |
| 20:04:21 to 20:04:43 | kernel `--lib` (first attempt: wrapping sites in `PackError`) | 59 |
| 20:14:54 to 20:15:16 | kernel `--lib` (value module regenerated from twin) | 30 |
| 20:19:58 to 20:20:21 | kernel `--lib` | 1 |
| 20:20:32 to 20:20:59 | kernel `--lib` | **0** |
| 20:21:13 to 20:21:54 | kernel `--lib --target wasm32-wasip2` | 2 (pack preflight literal) |
| 20:22:14 to 20:22:42 | kernel `--lib --target wasm32-wasip2` | **0** |
| 20:22:49 to 20:24:12 | `-p semio-framework-plugin -p semio-framework-plugin-describe -p semio-framework-os -p semio-framework-os-run --lib` | 3 (workflow-run `Schema`) |
| 20:26:07 to 20:26:32 | dependents | 1 (peer's new `control` import) |
| 20:26:59 to 20:27:18 | kernel `--lib` | **0** |
| 20:27:24 to 20:28:06 | dependents | 2 (plugin sqlite `decode_wire_value`) |
| 20:28:57 to 20:30:37 | kernel `--lib`, then dependents | **0 / 0** |
| 20:30:43 to 20:30:59 | kernel `--lib --target wasm32-wasip2` | **0** |
| 20:31:00 | kernel `--lib` | **0** (413 warnings, down from 584; none new in touched files) |

"KERNEL GREEN 20:31" was sent to `main`.

## Remaining Work and Follow-Ups

- **Lib tests:** not run. The peer's census lists 83 test-only errors. Signature changes add some more: `HistoryFile`/`compact`/`TailFollower` tests now need a context, and value-module tests now see `PackRefusal` returns.
- **Duplicate value module:** the OS value and encode modules are now identical to the framework twin apart from mounts. The coherent next step is the peer's "canonical mount consolidation": point `os_pack::value` at `pack::record` and re-home the schema-storage and unit test laws.
- **`native-bin`:** the binary entry points need the peer's context wiring.
- **Peer decision:** `serializer_entry` mapping of `TransportFailure` to InvariantViolated.

## Files Touched

**OS kernel:**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs` (my changes superseded by the peer)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📥️mounted-pack/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔌️io/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs` (my changes superseded by the peer)
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs`

**Framework pack:**

- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🏭️schema/🦀️.rs`

**Dependents:**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`

**Conversion scripts (inputs, kept):** `🧪️s4-packfix-{pair,convert,manual-value,manual-encode,port-value,port-encode,envelope-fix,errs}.py` in this ticket folder.

## Session 5 — 2026-10-05

Agent: S5-CHANNEL (successor of S4-PACKFIX + S4-BUMP). Scratch: `🗑️generated/s5-channel/`. Scope here: the kernel TEST target
(`cargo check -p semio-framework-os-kernel --lib --tests`) and the pack-error follow-ups; channel work is in `📓️s4-bump-report.md` § Session 5.

### S5.0 State at launch (00:18–00:25, landing lock HELD by COORDINATOR-ACTIVATION — no cargo, no saves under `🧰️framework/**`)

- The "83 test-only errors" figure is the peer's 10-04 19:50 census (`UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📓️current-os-326-test-only-prerequisite-census.md`),
  taken BEFORE the kernel lib was converted. Since then the Codex peer rewrote most of the listed test files (mtimes 20:36–20:55:
  pack value unit/refusals, native-decoding/-encoding tests, materialize unit, protocol-laws, intrinsic-bytes, sync unit, store unit,
  dsl unit) and moved the SPR io context to `crate::os_pack::control::CommandContext`.
- Static census on today's tree (`s5-channel/stale-packerror-variants.txt`): only **2** retired `PackError::<variant>` uses remain in
  the kernel-mounted trees, both in `🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🧪️tests/🦀️.rs:19–20`. The real error count
  is therefore unknown until the check runs — OWED at `landing: free`:
  `cargo check -p semio-framework-os-kernel --lib --tests --message-format=short` (gate v3 first).

### S5.1 First real measurements (gate v4, `CARGO_BUILD_JOBS=3`; logs `s5-channel/check-kernel-tests-{1,2}.txt`)

| Time | Command | Result |
| --- | --- | --- |
| 01:23:34–01:23:55 | `cargo check -p semio-framework-os-kernel --lib --tests` | exit 101, **1 error in `semio-framework-replication` (lib)**: `📡️replication/🎮️mutation/📦️bytes/🦀️.rs:191:53` E0308 `expected &u8, found u8` — the Codex peer's new file (git `AM`, mtime 01:21); fixed by the peer at 01:24 |
| 01:25:48–01:27:29 | same | exit 101, **kernel (lib) 1 error, kernel (lib test) the same 1 error, 871 warnings**: `💻️os/🔨️modules/🗣️dsl/🦀️.rs:219:71` E0308 `expected pack::record::EncodeOptions, found os_pack::value::EncodeOptions` |

- **The 83 test-only errors are gone.** With the lib-test target type-checked far enough to emit 871 warnings, the only error left
  is a LIB error the Codex peer introduced at 01:21:57 (new `encode_with_into` in `🗣️dsl/🦀️.rs`, its op-byte-pages wave): it
  hands the kernel twin's `os_pack::value::EncodeOptions` to the framework `pack::record::encode_record_body_into`. Borrow-check
  and later-phase errors may still hide behind it; the integration test target `sqlite_snapshot_native_admission` was not
  reached (cargo stopped at the lib), so the 2 retired `PackError::Schema` sites in `🪶️native-retirement/🧪️tests` are still to fix.
- **Why the twins disagree now.** The peer is moving the options type to the replication crate
  (`protocol::codec::PackEncodeOptions`, re-exported by the framework twin at `🎒️pack/🌱️value/🦀️.rs:3057`; that file was being
  written at 01:28 and held `crate::format::crate::format::VerificationLevel` at that moment), while the kernel twin
  `💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` (PACKFIX's 10-04 20:41 state) still declares its own `EncodeOptions`/`DecodeOptions`.
- **"Canonical mount consolidation" is NOT a mechanical follow-up any more** (correction of the PACKFIX remaining-work item):
  the twins differ in behaviour, not only in mounts. The kernel twin sorts intrinsic `DslValue::Object` keys by key bytes
  (`💻️os/…/🌱️value/🦀️.rs:568–571`, the coordinator's 20:41 fix for "emitted descriptor pack is not canonical"); the framework
  twin preserves the authored order by the peer's explicit law `schema_map_keys_are_canonical_and_intrinsic_objects_preserve_occurrences`
  (`🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs:943`, neutral fixture `🌱️value/🧫️fixtures/🔃️ordering`), and the law
  `object_keys_encode_in_canonical_key_byte_order` quoted in `📓️s5-resume.md` §1.21 no longer exists anywhere. Pointing
  `os_pack::value` at `pack::record` would silently change every kernel-encoded intrinsic object's bytes (descriptors, `.spk`
  values). That is a coordinator + peer decision (one ordering contract for both twins), not a PACKFIX edit.
- Coordinator instruction (01:3x): re-check at ~01:40 with `--target wasm32-wasip2`; if the dsl:219 error persists, convert the
  options at that one call site under the landing lock (no re-export, no shim), verify native + wasip2 + `--tests`.
  Superseded 01:39: the peer finished its pack wave itself (foundation GREEN 01:43); no call-site edit by me.

### S5.2 KERNEL TEST TARGET GREEN (landed 05:20–05:30 under the landing lock, after the 02:40–04:22 fleet cut)

Coordinator decision recorded as design §22.19: no consolidation of the value twins by this ticket (the pack peer unifies the
options type itself; the order policy is the peer's); the descriptor layer canonicalizes instead (see `📓️s4-bump-report.md` § S5.7).

| Time | Command (gate v5, `CARGO_BUILD_JOBS=3`) | Result |
| --- | --- | --- |
| 05:26:46–05:28:25 | `cargo check -p semio-framework-plugin-describe -p semio-framework-os-kernel --lib --tests` | exit 101: kernel lib 425 warnings (0 errors); integration target `sqlite_snapshot_native_admission` **3 errors** E0432 (`semio_framework_os_kernel::native_decoding` — a path the kernel never exported, also not at HEAD) |
| 05:29:30–05:30:00 | same + `--keep-going` | exit 101: **kernel lib-test 898 warnings, 0 errors**; describe lib + lib-test 2 warnings each; integration target **7 errors** E0433 (`DslValue` unresolved in the `🧬️octets` wrapper of the intrinsic-bytes law) |
| 05:30:30–05:30:32 | same | **exit 0**: kernel lib 425, kernel lib-test 898, integration target 274, describe lib/lib-test 2/2 warnings |

- The real count was **12 test-only errors, all in the integration test target** (the lib-test target had none left once the peer
  finished): 2 retired `PackError::Schema` (invisible until the imports resolved), 3 imports, 7 unresolved `DslValue`.
- Edits (5 kernel TEST files, no lib code, Edit tool):
  `🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🧪️tests/🦀️.rs` (2× `PackError::Schema(..)` →
  `PackError::from(ValueError::new(ValueRefusalKind::InvalidValue, ..))`, the mapping the peer used for the sibling fixtures);
  `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️binding/🦀️.rs`, `🧬️semio/🧪️tests/🚦️controlled/🦀️.rs`,
  `🗣️dsl/🧬️schema/🧪️tests/🛬️decoding/🦀️.rs` (import `semio_framework_value::native_decoding::*` where the module lives);
  `…/🪶️native-decoding/🧪️tests/🚪️public/🧬️octets/🦀️.rs` (`use semio_framework_value::DslValue;`).
- Targeted tests RAN (rule 48, uplift dir `target-nde-s5-channel`, `CARGO_INCREMENTAL=0`):
  `cargo test -p semio-framework-os-kernel --lib -- os_spr::channel::` → **91 passed / 0 failed** (05:54–05:55; includes
  `a_host_admits_only_a_guest_of_its_own_channel_version` and the two wire-fixture laws).
- Not run: the kernel's whole lib suite (1226 tests; needs a coordinator GO per rule 48) and the integration target
  `cargo test -p semio-framework-os-kernel --test sqlite_snapshot_native_admission` (the peer's laws; compiles now — OWED to the
  pack peer / S5-GATES, not a law of this ticket).
- `TransportFailure` → `InvariantViolated`: recommendation in `📓️s4-bump-report.md` § S5.4 (keep for this ticket; narrow
  `ArtifactPack::decode_pack_with` to `PackRefusal` in the pack owner's wave).
