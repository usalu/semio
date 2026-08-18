# terra-W0-params report

Packet W0 — params-record refactor (the SSOT mechanism) for the plugin ABI WIT.

## delivered

26 records carry `req: request-id` (verified by grep, matches the packet's count). 23 got a
`*-params` extraction; 3 were judged already-a-single-named-type and left unwrapped (see below).

| # | `*-effect`/`*-event` record | new `*-params` record | fields moved into params |
|---|---|---|---|
| 1 | `blob-load-effect` | `blob-load-params` | `hash: string` |
| 2 | `blob-write-effect` | `blob-write-params` | `media-type: pack, bytes: pack` |
| 3 | `http-request-effect` | `http-params` (named per the packet's own worked example, not `http-request-params`) | `method, url, headers, body, streaming` |
| 4 | `document-read-effect` | `document-read-params` | `doc: u64, lane: string` |
| 5 | `document-write-effect` | `document-write-params` | `doc: u64, lane: string, ops: pack` |
| 6 | `registry-query-effect` | `registry-query-params` | `kind: string, filter: pack` |
| 7 | `io-compose-effect` | `io-compose-params` | `key: pack, sources: pack` |
| 8 | `io-run-effect` | `io-run-params` | `source: string, target: string, payload: pack` |
| 9 | `cache-derive-effect` | `cache-derive-params` | `engine-id: string, input: pack` |
| 10 | `cache-read-effect` | `cache-read-params` | `engine-id: string, key: pack` |
| 11 | `open-window-effect` | `open-window-params` | `kind: string, params: pack` (inner field literally named `params`, unchanged — now nests under the outer `open-window-effect.params` field; a quirk, not a conflict) |
| 12 | `dispatch-action-effect` | `dispatch-action-params` | `action: string, args: option<pack>, delay-ms: u64` |
| 13 | `invoke-extension-effect` | `invoke-extension-params` | `extension-id, capability, payload` |
| 14 | `spawn-plugin-instance-effect` | `spawn-plugin-instance-params` | `plugin-id, app-id, os-instance-id, label, document-json` |
| 15 | `open-dialog-effect` | `open-dialog-params` | `dialog-id: string, args: option<pack>` |
| 16 | `request-file-open-effect` | `request-file-open-params` | `accept, read-as, import-action, multiple` |
| 17 | `request-media-frames-effect` | `request-media-frames-params` | `accept, frame-action, done-action, fallback-action, sample-stride, max-frames, max-long-edge-px, fps-hint, payload, args` |
| 18 | `storage-read-effect` | `storage-read-params` | `key: string` |
| 19 | `storage-write-effect` | `storage-write-params` | `key: string, value: pack` |
| 20 | `storage-delete-effect` | `storage-delete-params` | `key: string` |
| 21 | `request-capability-effect` | `request-capability-params` | `id: capability-id, scope, reason, optional` |
| 22 | `http-chunk-event` | `http-chunk-params` | `bytes: pack, done: bool` |
| 23 | `request-event` | `request-params` | `origin: message-endpoint, capability: string, payload: pack` |

**NOT wrapped** (payload is already a single named type — wrapping would add a pointless second
layer around one already-nominal field):

| record | sole non-`req` field | why left alone |
|---|---|---|
| `link-resolve-effect` | `link: pack` | `pack` is itself the wire-value named type (`types.wit`'s alias); a `link-resolve-params { link: pack }` wrapper would duplicate nothing and add a layer for no benefit. |
| `respond-effect` | `outcome: respond-result` | `respond-result` is already a defined `variant`; same reasoning. |
| `completed-event` | `outcome: completion-result` | `completion-result` is already a defined `variant`; same reasoning. |

Note on scope: `blob-load-effect`/`storage-read-effect`/`storage-delete-effect` also end up with
exactly one field in their `-params` record (`hash`/`key`), but that field's type is a *primitive*
(`string`), not a pre-existing named type — so per the literal criterion above ("already a single
**named type**") they were still wrapped, for uniformity with every other completable effect. If
the registrar prefers single-primitive-field payloads to stay unwrapped too, that's a one-line
follow-up (drop the `-params` record, put the field straight on the effect) — flagging the
distinction here rather than silently picking one reading.

`io-run-effect` was wrapped in the WIT (`io-run-params`) for consistency even though
`semio_framework::kernel::Effect` has **no `IoRun` variant yet** (blocked on A3 — see
`wit_effect_to_kernel`'s `E::IoRun(_inner) => return Err(...)` arm, pre-existing, untouched). The
host-side test `io_run_effect_is_a_reported_error_not_a_silent_mismap` was updated to construct
the now-nested `IoRunEffect { req, params: IoRunParams { .. } }` shape so it still compiles.

## line ranges edited per file

### `🧬️schema/📜️component.wit` (822 → 933 lines)
Only the `effects` interface (originally lines 193–500) and three records inside the `events`
interface (`completed-event`, `http-chunk-event`, `request-event`, originally lines 588–626) were
touched. Every `*-params` record was inserted immediately before its `*-effect`/`*-event` record;
no interface was added, removed, or reordered; the `variant effect { .. }`/`variant event { .. }`
lists and `world actor` were not touched (they reference the effect/event record names, which did
not change). Editing was done via many small, exact-match `Edit` calls rather than one line-range
replace, so "line ranges" below are the ORIGINAL file's line numbers for each touched record group:
- 206–214 (blob-load/blob-write)
- 216–228 (http-request, `streaming` doc-comment kept verbatim)
- 230–244 (document-read/document-write/link-resolve)
- 246–306 (registry-query/io-compose/io-run/cache-derive/cache-read/open-window/close-window(unwrapped, no req)/dispatch-action/invoke-extension)
- 341–358 (spawn-plugin-instance/open-plugin-instance(unwrapped, no req)/open-dialog)
- 368–387 (request-file-open/request-media-frames)
- 414–443 (respond-result/respond-effect(unwrapped)/storage-read/storage-write/storage-delete/request-capability)
- 588–591 (completed-event, unwrapped) and 592–596 (http-chunk-event)
- 621–626 (request-event)

### `⚛️reactor/🦀️component.rs` (469 lines, unchanged count)
kernel→WIT direction (`kernel_effect_to_wit`) and WIT→kernel direction (`wit_event_to_kernel`),
both inside the `wit_bridge` module (`#[cfg(component-guest…)]`-gated):
- line 291 — `W::HttpChunk` now reads `payload.params.bytes`/`payload.params.done`
- line 297 — `W::Request` now reads `payload.params.origin/capability/payload`
- line 391 — `Effect::OpenWindow` now builds `OpenWindowParams { kind, params }` nested under the outer `params` field
- lines 402–437 — every wrapped effect's WIT constructor now builds `<X>Params { .. }` and sets the outer record's `params` field instead of the flat fields (`RequestFileOpen`, `RequestMediaFrames`, `SpawnPluginInstance`, `OpenDialog`, `DispatchAction`, `BlobWrite`, `BlobLoad`, `HttpRequest`, `DocumentRead`, `DocumentWrite`, `RegistryQuery`, `IoCompose`, `CacheDerive`, `CacheRead`, `StorageRead`, `StorageWrite`, `StorageDelete`, `RequestCapability`)
- `LinkResolve`/`Respond` arms left untouched (unwrapped records)

### `🖥️host/🦀️component.rs` (4409 lines, unchanged count) — surgical, two functions only
- `wit_effect_to_kernel` (fn starts line 1083): lines 1088, 1089–1093, 1094–1098, 1099–1103,
  1106–1111, 1128, 1130, 1133, 1134–1149, 1159–1162 — every `inner.<field>` read updated to
  `inner.params.<field>` for the 21 wrapped effect variants; `E::LinkResolve`/`E::Respond` (line
  1097, and the `Respond` arm ~1155-1158) left untouched.
- `kernel_event_to_wit` (fn starts line 1188): lines 1211, 1220 — `HttpChunkEvent`/`RequestEvent`
  construction now nests the payload under `params: wit_events::HttpChunkParams { .. }` /
  `wit_events::RequestParams { .. }`.
- line 1306 (test `io_run_effect_is_a_reported_error_not_a_silent_mismap`): updated the
  `IoRunEffect { .. }` literal to the new nested `params: IoRunParams { .. }` shape so the test
  crate still compiles.

No other region of this 4409-line file was touched.

## commands + exit codes

All four run with `CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-w0`, foreground, single turn each.
Exit codes below are the real `cargo` process exit code (re-verified with a second run capturing
`$?` directly, not through a `tail` pipe, after the first run's full output was already inspected).

```
$ cargo check -p semio-framework-plugin --lib
    Finished `dev` profile [unoptimized] target(s) in 1m 19s
exit=0
```

```
$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
    Finished `dev` profile [unoptimized] target(s) in 1m 06s
exit=0
```
This is the build that actually parses the new WIT as a real component (wit-parser 0.220.1 /
wit-bindgen 0.36.0 pinned toolchain) — it passed clean, first try, no cascade-of-`exports`-errors
symptom the packet warned about.

```
$ cargo check -p semio-framework-plugin-host --all-targets
    Finished `dev` profile [unoptimized] target(s) in 52.47s
exit=0
```

```
$ cargo test -p semio-framework-plugin-host --lib
test result: ok. 86 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.35s
exit=0
```

Also ran (not in the mandatory list, but the packet's baseline table names it):
```
$ cargo test -p semio-framework-plugin --lib
test result: FAILED. 241 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
exit=101
```
See "baseline vs after test counts" below for attribution.

## reserved-word sweep evidence

```
$ grep -n "async func\|future<\|stream<" 📜️component.wit
none found
```
Every new identifier introduced by this packet — the 23 new `<x>-params` record names and the
`params` field name added to each wrapping effect/event — checked by hand against the WIT keyword
list (`record`, `variant`, `enum`, `flags`, `resource`, `func`, `interface`, `world`, `import`,
`export`, `use`, `package`, `type`, `list`, `option`, `result`, `tuple`, `bool`, `string`, `char`,
`u8..u64`, `s8..s64`, `f32`, `f64`, `static`, `constructor`, `own`, `borrow`, `with`, `from`,
`async`, `future`, `stream`, `true`, `false`): none collide. `params` in particular is not
reserved (WIT has no special meaning for it). Confirmed `streaming` (the deliberate `stream`
rename from a prior packet) was preserved verbatim inside the new `http-params` record, not
"fixed back". Also grepped every new record name against the pre-edit file for name collisions
before writing them (`select:` pattern over all 23 new names) — zero collisions.

Brace-balance check on the final WIT file (`{`/`}` depth walk) ends at depth 0 — no truncated or
misnested block from the many surgical edits.

## baseline vs after test counts

- `cargo test -p semio-framework-plugin-host --lib`: baseline "75 passed / 0 failed" →
  **86 passed / 0 failed / 1 ignored**. No regression; the extra passing tests are pre-existing
  fleet growth since the baseline was measured (this packet added no new tests here).
- `cargo test -p semio-framework-plugin --lib`: baseline "4 pre-existing failures" →
  **6 failures observed** (241 passed). All 6 panic locations are OUTSIDE the files/regions this
  packet touched:
  - `component::app::artifact_definition_contract_tests::*` (3 failures) — panic inside the main
    plugin crate's own `🦀️component.rs` (`../../🦀️component.rs:4209/4225/4242`), about
    "canonical resource identity grammar" / artifact-definition validation — nothing to do with
    effect/event wire shapes.
  - `component::builder::plugin_builder_dependency_tests::host_media_conflicts_reject_the_whole_candidate_before_execution`
    — panic in `🏗️builder/🦀️component.rs:771`, about aggregate rollback side effects.
  - `component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`
    and `...merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` — panics
    in the main crate's `🦀️component.rs:17627`/`17393`, about child-factory registration
    conflicts and channel-command merge/conflict identity — again nothing on the effect/event
    wire path.
  None of the 6 reference `wit_effects`/`wit_events`/`*-params`/`Effect::`/`Event::` conversion
  code. This matches the repo's known "concurrent cargo workspace churn" pattern (peer sessions
  live in the same tree) rather than a regression from this packet — but I did not revert my diff
  to prove it by elimination (that would require touching git), so treat the +2-failures delta as
  unattributed-but-circumstantially-unrelated rather than proven-innocent.

## other observation (not mine, not touched)

`git diff HEAD` on `⚛️reactor/🦀️component.rs` and `🖥️host/🦀️component.rs` shows extra hunks I
did not author: the `#[cfg(...)]` gates on the `Effect/Event/.. ` and `HashMap` imports near the
top of the reactor file, and `kernel.submit(&env(..))` / `kernel.complete(actor, &ok_turn(), ..)`
reference-taking changes around `runtime_metrics_publisher_tests` in the host file (lines
~1481–1600). Both were already present in the working tree the first time I `Read` each file,
before I made any edit — confirmed by comparing my first-Read output against these diff hunks.
Called out here per the "live tree, other sessions/auto-commit" rule; not a lease-request since I
never touched that content and it is not in my owned regions.

## lease-requests

None.

## honest gaps

- The single-primitive-field-vs-single-named-type-field line (see the note under "delivered")
  is a judgment call, not something the packet spelled out unambiguously. I applied the literal
  reading ("already a **named** type" ≠ "already a single field") and wrapped
  `blob-load`/`storage-read`/`storage-delete`. If sol/the registrar wanted ALL single-field
  payloads left bare regardless of field type, that's a small follow-up, not a redesign.
- `io-run-effect` was wrapped in WIT even though the Rust `kernel::Effect` has no `IoRun` variant
  yet (pre-existing gap, blocked on A3, not this packet's job to fix) — the WIT shape is ready for
  whenever that lands.
- I did not run the full workspace or touch any file outside the three owned paths. The `+2`
  test-failure delta on `semio-framework-plugin --lib` is reported above with full attribution
  evidence (panic locations all outside my diff) but not proven via a revert-and-recompare, per
  the no-git-modifying-commands rule.
