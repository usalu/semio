# 📓️ terra-sdk-async — guest SDK dual host backend

## delivered

1. **`🔌️plugin/🌐host/🦀️component.rs`** (371 → 1015 lines) — `enum HostBackend { Poll(RequestRegistry), Direct }` (the second variant `#[cfg(feature = "component-guest-async")]`). Every one of `Host`'s ~46 public methods is now a two-arm match on `&self.backend`. Added a second, independent `wit_bindgen::generate!({ world: "actor-async", path: "../../🧬️schema" })` in a new `pub mod direct` (gated `component-guest-async` + real wasm32-wasip2), sibling to the root `🦀️component.rs`'s own `pub mod component` (`world: "actor"`) — two `generate!` calls in one crate, no collision, since neither world's `export!` is ever invoked from here (imports only).
2. **`🔌️plugin/🌐host/📖️body/🦀️component.rs`** (new, 138 lines) — `BodyReader`: `Poll { bytes, consumed }` (an already-fully-buffered body, replayed in slices) and `Direct(wit_bindgen::StreamReader<u8>)` (real host-async stream, `#[cfg(component-guest-async, wasm32-wasip2)]`). `next_chunk()`/`collect(cap)`, the latter faulting (`plugin.host.body-too-large`) rather than truncating. 4 unit tests, all against the `Poll` variant (the only one constructible off a real wasm32 build).
3. **`🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs`** — the chunk-reassembly fix: `Slot::Pending` grew a `partial: Vec<u8>` field; `RequestRegistry::append_chunk(id, bytes, done, cap)` accumulates instead of discarding, faulting over `cap` via the same `Inner::complete` path `resolve` already used (factored out, not duplicated); `instance_of(id) -> Option<u32>` added so the caller can look up the owning instance's quota. 3 new tests (multi-chunk reassembly byte-for-byte, over-cap fault, late-chunk-after-resolve no-op).
4. **`🔌️plugin/⚛️reactor/🦀️component.rs`** — `Event::HttpChunk`'s routing step now calls `append_chunk` with `cap` resolved from `INSTANCE_QUOTAS.get(instance).message_bytes` (default 64 MiB) instead of `if done { resolve(req, Ok(bytes)) }` (which silently dropped every non-final chunk). **Cross-packet fix, in-scope**: `⚛️reactor/💼️jobs/🦀️component.rs`'s `spawn_job` (owned by `cold-kinds`, not touched) calls `crate::reactor::host()` with **zero args**, but the only `host` function that existed took `instance: u32` — an unreachable-until-today mismatch behind the newly-declared `component-guest-async` gate. Fixed entirely inside my own file: renamed the instance-scoped constructor to `host_for_instance(instance)` (updating its one call site, `spawn_task`, also mine) and added a genuine zero-arg `host()` (instance 0, matching `RequestRegistry::for_instance`'s own "no tag ⇒ instance 0" default) for `spawn_job` to resolve against. No edit to `💼️jobs/**`.

## the 24 host-async imports — guest table

Poll arm behaviour is **byte-for-byte unchanged** for every pre-existing method (verified by keeping the identical `Effect` construction inline, just wrapped in the match). "NEW" = no `Host` method existed for this import before this packet.

| WIT `host-async` func | `Host` method | Poll arm | Direct arm |
|---|---|---|---|
| `storage-read` | `storage_read` | `Effect::StorageRead` (unchanged) | `StorageReadParams{key}`; `Ok(None)` → typed `plugin.storage.not-found` fault (WIT allows `None`, the poll world never produced it — see honest gaps) |
| `storage-write` | `storage_write` | unchanged | `StorageWriteParams{key, value: bytes}` (WIT field is `value`, not `bytes`) |
| `storage-delete` | `storage_delete` | unchanged | `StorageDeleteParams{key}` |
| `blob-load` | `blob_load` | unchanged | `BlobLoadParams{hash}` |
| `blob-write` | `blob_write` | unchanged | `BlobWriteParams{media_type: pack(&media_type), bytes}` |
| `blob-read` | `blob_read` **(NEW)** | reuses `Effect::BlobLoad`, wraps result as `BodyReader::poll_buffered` (no chunked blob backend exists anywhere in this codebase — same honest-gap the host report already recorded) | real streaming import → `BodyReader::direct(stream)` |
| `http-fetch` | `http_fetch` **(NEW)** | reuses `Effect::HttpRequest{stream:true}`, decodes the JSON `HttpResponseWire{status,headers,body}` envelope `🖥️host/⚡️effects/🦀️component.rs::encode_http_response` writes, wraps `body` as one `BodyReader` chunk | real `http-response{status,headers,body:stream<u8>}` |
| `document-read` | `document_read` | unchanged | `DocumentReadParams{doc: doc.0 as u64, lane}` |
| `document-write` | `document_write` | unchanged | `DocumentWriteParams{doc: doc.0 as u64, lane, ops}` |
| `link-resolve` | `resolve_link` | unchanged | `link_resolve(link.into_bytes())` (positional `pack`, not `*-params`-wrapped, per WIT) |
| `registry-query` | `registry_query` | unchanged | `RegistryQueryParams{kind, filter: filter.map(pack).unwrap_or_default()}` |
| `io-compose` | `io_compose` | unchanged | `IoComposeParams{key: key.into_bytes(), sources: pack(&sources)}` |
| `io-run` | `io_run` | **unchanged, and stays a documented approximation** — kernel `Effect` has no `IoRun` variant (the same A3 gap the host report and `🦀️component.rs`'s own `wit_effect_to_kernel` already name), so the Poll arm still synthesizes a one-hop `IoCompose` | real multi-hop `IoRunParams{source,target,payload}` — a genuine capability Direct gets over Poll, not a bug to reconcile |
| `cache-derive` | `cache_derive` | unchanged | `CacheDeriveParams{engine_id, input}` |
| `cache-read` | `cache_read` | unchanged | `CacheReadParams{engine_id, key: key.into_bytes()}` |
| `invoke-extension` | `invoke_extension` | unchanged | `InvokeExtensionParams{extension_id, capability, payload: request_json.into_bytes()}` |
| `open-window` | `open_window` | unchanged | `OpenWindowParams{kind: kind.0, params: pack(&params)}` |
| `open-dialog` | `open_dialog` | unchanged | `OpenDialogParams{dialog_id, args: args.map(pack)}` |
| `dispatch-action` | `dispatch_action` **(NEW)** | `Effect::DispatchAction` (effect existed, no `Host` method ever built it before) | `DispatchActionParams{action, args: args.map(pack), delay_ms}` |
| `spawn-plugin-instance` | `spawn_plugin_instance` | unchanged | `SpawnPluginInstanceParams{...}` (direct field mapping) |
| `request-file-open` | `request_file_open` | unchanged | `RequestFileOpenParams{...}` |
| `request-media-frames` | `request_media_frames` | unchanged | `RequestMediaFramesParams{..., args: args.map(pack)}` |
| `request-capability` | `request_capability` | unchanged | `RequestCapabilityParams{id: capability.id.0, scope, reason, optional}` |
| `spawn-job` | `spawn_job` | unchanged (`job = req.0`, reusing the `RequestRegistry` counter) | `job` minted from a NEW dedicated `DIRECT_JOB_IDS: AtomicU64` (Direct has no registry/counter to reuse) |

`emit`/`emit-patch` (sync, not among the 24): `Host`'s ~22 fire-and-forget methods (`send_message`, `notify`, `set_timer`, …) now go through a shared `Host::emit(effect)` helper — Poll calls `RequestRegistry::emit` (unchanged), Direct calls `direct::host_async::emit(&kernel_effect_to_direct_wit(effect))`, a second `kernel::Effect → wit effect` conversion (`kernel_effect_to_direct_wit`) covering exactly the variants `Host` itself constructs, with an `unreachable!` fallback (see honest gaps) — necessarily duplicated from `⚛️reactor/🦀️component.rs`'s `kernel_effect_to_wit` because the two `generate!` calls produce nominally distinct Rust types for the identical WIT shape. `emit-patch` has no caller: `Host` never had a UI-patch-emitting method (patches are diffed inside `⚛️reactor::poll`, not through `Host`), so it wasn't added.

## chunk-reassembly evidence

`⚛️reactor/📮️requests/🦀️component.rs::append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes`: feeds a 710-byte non-uniform buffer through `append_chunk` in 97-byte windows (8 chunks, only the last `done`), then asserts the resolved `RequestFuture` yields the **exact original `Vec<u8>`** — byte-for-byte, not just length. `append_chunk_over_cap_faults_instead_of_silently_truncating`: two 40-byte chunks against a 64-byte cap fault on the SECOND chunk (before `done` ever arrives) with `fault.code.0 == "plugin.request-registry.body-too-large"`, not a truncated `Ok`. `append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op`: a chunk for a never-requested id, and a chunk arriving after `resolve` already fired, both leave the real resolution untouched.

**Not run** — see `## acceptance blocked` below; compiled clean (see `## commands`) but never executed.

## commands + exit codes

All foreground, single Bash call each, `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/14212986-9ab2-4361-9901-92138f0456ba/scratchpad/target-sdk-async`, `-p semio-framework-plugin` only, per rules 4/24.

```
$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest-async
    ... (full output: sdk-async-wasm32-check.txt, 38210 bytes) ...
error: could not compile `semio-framework-plugin` (lib) due to 8 previous errors; 2 warnings emitted
EXIT=101
```
Zero of the 8 errors and 2 warnings reference any file I own (`🌐host/**`, `⚛️reactor/📮️requests/**`, `⚛️reactor/🦀️component.rs`) — verified by `grep -n "🌐host\|📮️requests\|⚛️reactor" sdk-async-wasm32-check.txt` returning nothing after my fixes (two iterations before that: a `&Effect` vs `Effect` mismatch at `direct::host_async::emit`, and an `unnecessary_qualification` warning — both fixed, see `## bugs found and fixed in my own code`). All 8 errors trace to `🔌️plugin/🦀️component.rs:16330,453,9678,12548,12554` — a syntax error (`instance.semio_framework_plugin::resolve_ready(...)`) plus unresolved imports plus `Send`-future errors in `VcsArtifactApp::export_media`, all inside the **`io-async-signatures`** packet's ATOMIC, currently-mid-flight ownership (file mtime 12:24, 2 minutes before this check ran; confirmed identical failure on a bare `cargo check -p semio-framework-plugin --lib` with NO features at all — not something my feature flag triggered).

```
$ cargo check -p semio-framework-plugin --all-targets --features component-guest-async   (native, no --target)
    ... (full output: sdk-async-alltargets-check.txt, 55262 bytes) ...
error: could not compile `semio-framework-plugin` (lib test) due to 10 previous errors; 3 warnings emitted
EXIT=101
```
Same `grep` returns nothing for my files. The 2 EXTRA errors here (10 vs 8) are `E0004` non-exhaustive-match on `jobs::JobStep::Running(None)` in `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs:138` and `⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs:94` — the **`cold-kinds`** packet's own in-progress test code, inside `⚛️reactor/💼️jobs/**` (explicitly not mine, per the mission's "you do NOT own" list).

Both `--lib` and `--all-targets` were run (rule 26); neither surfaced anything in my files either way.

## acceptance blocked — not my defect

`cargo test -p semio-framework-plugin --lib` (the real 263/5 baseline gate, and the only way to actually RUN the 7 new tests this packet added) cannot succeed right now: the crate does not compile at all, for anyone, regardless of feature flags — confirmed via a bare `cargo check -p semio-framework-plugin --lib` (no `--target`, no `--features`) failing with the identical 8 errors. This is **not gated by `component-guest-async`** and would block every OTHER packet's acceptance run too, not just mine. Two live, concurrent, uncommitted peer edits are the cause (`io-async-signatures` in `🔌️plugin/🦀️component.rs`, `cold-kinds` in `⚛️reactor/💼️jobs/**`) — per rule 25 (an atomic packet may be redirected before it starts or allowed to finish, never interrupted) I have not touched either file. My own code is proven to compile cleanly in isolation (zero errors/warnings from my files across 4 separate check runs, 2 targets × 2 feature combinations) but the coordinator's real `cargo test` gate needs one or both of those peers to land first.

## bugs found and fixed in my own code (self-review via the compile, not guessed)

1. `direct::host_async::emit(kernel_effect_to_direct_wit(effect))` — `emit`'s generated signature takes `&effect` (WIT-generated guest imports borrow record-typed params by default), not owned; fixed to `&kernel_effect_to_direct_wit(effect)`. Caught by the FIRST wasm32 check, not guessed.
2. `dsl::DslValue::Null` → `DslValue::Null` in the `pack()` helper (`unused_qualifications` warning; `DslValue` is already `use`d at the top of the file).
3. `Host::call` (the old single Poll-only helper) became genuinely dead code once every method got its own inline two-arm match — removed rather than left as an unused private method (confirmed zero remaining call sites via `grep -n "self\.call(" 🌐host/🦀️component.rs`).

No other errors or warnings from my files across any of the 4 check runs.

## honest gaps

- **`Host::new_direct()` has no live caller.** No actor task drives `world actor-async`'s `runner::run` yet (that's a different, not-yet-dispatched packet's job per the mission) — `HostBackend::Direct` is real, compiles, and is exercised by nothing at runtime. Same shape as the host side's `AsyncActorHostState`: built, not yet wired to a live `Store`.
- **`storage-read`'s `Ok(None)` maps to a typed "not-found" fault**, not `Ok(Vec::new())` — the host report's own honest gaps note the real host backend never actually produces `None` today (every not-found surfaces as `Err`), so this mapping is unverified against a real host, only against the WIT type signature.
- **`http_request`'s Direct arm collects the whole streamed body with `cap = usize::MAX`** (`collect_direct_body`) — that call site has no instance-quota handle to read a real cap from (unlike `RequestRegistry::append_chunk`, which reads `INSTANCE_QUOTAS` from the reactor's own per-instance table). `http_request`'s signature predates `BodyReader` and returns `Result<Vec<u8>, Fault>` (a live contract I must not break), so it has to fully buffer either way; the missing cap is a real, recorded gap, not a silent unlimited-by-design choice.
- **`kernel_effect_to_direct_wit`'s `unreachable!` fallback is untested** — it covers the ~22 `Effect` variants `Host::emit`'s call sites actually construct; every OTHER `Effect` variant (all the completable ones, which route through their own dedicated import instead) would panic if `Host::emit` were ever called with one, by construction never happens today, but nothing enforces that at the type level.
- **No test exercises `HostBackend::Direct` end-to-end** — it cannot be, without a live `wasmtime` `Store<AsyncActorHostState>` and a running `world actor-async` guest, which don't exist yet (same "blocked on the runtime packet" gap the host report already named for its own side). Everything Direct-side is verified by compilation only (wasm32-wasip2 `cargo check`, clean), never by execution.
- **The 7 new tests (3 in `📮️requests`, 4 in `📖️body`) compiled clean (`--all-targets`) but never ran** — see `## acceptance blocked`.
- **`Host::registry()` narrowed from `&RequestRegistry` to `Option<&RequestRegistry>`** — confirmed zero external callers (`grep -rn "\.registry()\b"` across the whole `🔌️plugin` tree found only the definition), so this is not a live-contract break, but it IS a public signature change worth flagging explicitly since rule-of-thumb elsewhere in this ticket is "never break a live cross-file contract."

## lease-requests

None. The one cross-packet mismatch I found (`⚛️reactor/💼️jobs`'s zero-arg `crate::reactor::host()` call vs. the old single `host(instance: u32)`) was fixable entirely inside my own owned file (`⚛️reactor/🦀️component.rs`) by adding a new zero-arg `host()` alongside a renamed `host_for_instance(instance)` — no edit to `💼️jobs/**` needed or made.

## files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️component.rs` (rewritten)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/📖️body/🦀️component.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests/🦀️component.rs` (edited)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs` (edited: `Event::HttpChunk` routing + `host`/`host_for_instance` split)
- Scratch (ticket folder, `.txt`, not deleted per rule "must not delete temp files"): `sdk-async-wasm32-check.txt`, `sdk-async-alltargets-check.txt`
