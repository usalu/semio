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
