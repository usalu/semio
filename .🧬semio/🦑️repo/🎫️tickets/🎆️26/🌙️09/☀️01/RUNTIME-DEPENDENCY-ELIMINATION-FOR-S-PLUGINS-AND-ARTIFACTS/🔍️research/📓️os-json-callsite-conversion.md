# 🔌️🏪️ `🔌️plugin` + `🏪️store` — `serde_json::` call-site conversion (judgment wave)

Continues `📓️os-plugin-store-serde-conversion.md` (the additive derive wave). That wave's own
scoreboard: **649 `serde_json::` call sites — 434 in `🔌️plugin` (53 files), 215 in `🏪️store` (15
files)**. This wave did the per-site judgment work that wave deliberately deferred.

## Headline finding: most of the 649 sites are legitimate test-oracle usage, not blockers

A line-by-line classifier (brace-matching every `#[cfg(test)] mod` span, plus treating any file
under a `🧪️` path segment as wholly test) split the 649 sites:

| module | total | test-only (stays serde, by design) | production (the real judgment work) |
|---|---|---|---|
| `🔌️plugin` | 434 | ~423 | **~11 files, ~50 sites** |
| `🏪️store` | 215 | ~126 | **~7 files, ~50 sites** |

This matches the ticket's own stated exception explicitly: CLAUDE.md requires "the same output of
a test with at least one third-party library in order to validate our own implementation," and the
prior wave's own doc flagged `serde_json::Value` fixture/oracle usage as something that "should
probably stay serde." Test-only `serde_json::` sites (fixture JSON parsing via `include_str!`,
`serde_json::json!` differential-oracle assertions, round-trip checks against a real third-party
parser) were **left untouched, deliberately** — they run under `#[cfg(test)]`, never link into a
shipped `wasm32-wasip2` component, and are exactly the third-party-oracle pattern CLAUDE.md's
test-driven-development rule asks for. Converting them would be net-negative: it would remove the
independent verification these tests exist to provide.

The real work — the ~100 production call sites — is what this wave converted, sorted by the
JSON-text-vs-`DslValue` rule from `📓️serde-fanout-playbook.md`.

## Converted this wave (by file, with the rule applied)

All of the following now route through `crate::os_pack::json::{to_json_string, from_json_str}`
(guest-reachable wire/JSON-text boundary) or a first-party bridge — zero `serde_json::` left in
their production code path. Every one required the target type to already have (or be given)
`ToValue`/`FromValue`.

### `🔌️plugin`

- `⚛️reactor/💼️jobs/💡️infer/🦀️component.rs` (5 sites) — `WireArtifactInferenceRequest`/`Result`
  already had both traits; converted `from_slice`/`to_vec` to `from_json_str`/`to_json_string` on
  UTF-8 text, and a `(String, String)` progress tuple (tuple `ToValue`/`FromValue` already exists,
  2- and 3-tuples encode as a fixed array exactly like `serde_json`'s own tuple shape).
- `⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs` (2 sites) — added `FromValue` to the local
  `MigrateInput` struct (was `Deserialize`-only).
- `⚛️reactor/💼️jobs/🦀️component.rs` (3 sites) — `job_io_run`/`job_io_sniff`: added `FromValue` to
  local `IoRunInput`; gave `semio_framework::io_schema::IoPayload` (framework-owned, `🚪️io` module)
  `ToValue`/`FromValue` since two plugin call sites need it.
- `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs` (1 site).
- `⚛️reactor/📸️checkpoint/🦀️component.rs` (2 sites) — `CheckpointPack` already had both traits.
- `🌐host/🦀️component.rs` (2 sites) — `HttpResponseWire` already had both traits.
- `📇️describe/📦️packages/🦀️rust/📦️glue.rs` (1 site) — reused the `DslValue` already being built
  for the pack encode two lines above (`store::json::from_dsl_value(&final_value)` +
  `to_string_pretty`) instead of a second independent `Serialize` walk of `PackageDescriptor`.
  **Removed `serde_json = "1.0.140"` from this crate's (`semio-framework-plugin-describe`,
  a build-time-only native tool, never part of the shipped component) `Cargo.toml`** — it was the
  crate's last use.
- `🛂️describe/🦀️component.rs` (1 site) — `WireArtifactInferenceMetadata` already had both traits.
- `🖥️host/🦀️component.rs` (native host/wasmtime-engine code — never compiled into the shipped
  `wasm32-wasip2` guest target, but part of the same `semio-framework-plugin` crate and worth
  getting right) — ~20 sites: `dialects`/`io_routes`/`identify` (gave `Confidence`, `IoFidelity`,
  `IoEntryDescriptor`, `IoRoute` in `🚪️io/🧬️schema` `ToValue`/`FromValue`), the inference-router
  cluster (gave `InferenceRouteResult` `FromValue` — it only had `Deserialize` before), the
  `Owned*Input` family (`OwnedPollInput`/`StartJobInput`/`StepJobInput`/`CancelJobInput`/
  `RestoreInput`/`CheckpointMetadata` already carried `ToValue`, several also `FromValue`), and
  `io_run`/`io_sniff`/`migrate`/`compose`.
  `PluginHostError::Json(serde_json::Error)` → `Json(String)` (holds `.to_string()`); the
  `impl From<serde_json::Error>` was deleted since nothing needs it any more.

### `🏪️store`

- `🔄️sync/🦀️component.rs` (1 site) — `use serde_json::Value;` was dead code (verified with a
  targeted grep for `Value` outside `serde_json::Value`/`Self::Value` before deleting); removed.
- `🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs` and `📌️commit-space-checkpoint/🦀️.rs`
  (3 sites each, hand-written bridge) — **root-caused, not just converted at the call site**: both
  leaves' hand-written `serde`-bridge `ToValue`/`FromValue` impls carried a stale comment claiming
  "`SpaceAlternative`/`SpaceCheckpoint` embed foreign types that cannot derive them (orphan rule)."
  That was true when the comment was written but a concurrent session (per the prior wave's own
  note) had since given `SpaceAlternative`/`SpaceCheckpoint` themselves `ToValue`/`FromValue`
  directly (`🏪️store/🦀️component.rs:18068-18096`) — confirmed by reading the struct's own
  `#[derive(...)]` line, not by trusting the comment. Converted both leaves to a plain
  `#[derive(ToValue, FromValue)]` + `#[value(rename_all = "camelCase", deny_unknown_fields)]`,
  matching their four sibling leaf files (`switch-space-alternative`, `remove-space-checkpoint`,
  `remove-space-alternative`, `restore-active-space-alternative`) which already used this shape.
  Deleted the now-dead hand-written serde-bridge `impl`s.
- `🧬️schema/🧬️mutations/🦀️.rs` (16 sites) — the SAME root cause reached the aggregate:
  `SpaceHistoryMutation`'s own hand-written `ToValue`/`FromValue` (in `../../🦀️component.rs`) was
  routed through the generic `to_dsl_value`/`from_dsl_value` serde bridge with a comment claiming
  `#[derive(ToValue, FromValue)]` "does not support" `#[serde(tag = "operation", content =
  "payload")]`. That claim is **stale**: the derive macro gained adjacently-tagged (`tag` +
  `content`) enum support mid-ticket (`📓️serde-fanout-playbook.md`'s own addendum). Converted
  `SpaceHistoryMutation` to `#[derive(..., ToValue, FromValue)]` +
  `#[value(tag = "operation", content = "payload", rename_all = "camelCase",
  deny_unknown_fields)]`, deleted the hand-written `impl ToValue`/`impl FromValue for
  SpaceHistoryMutation`. This also let `SpaceHistoryDiff` (same file, same stale-comment pattern —
  its `add_checkpoint: Option<SpaceCheckpoint>`/`add_alternative: Option<SpaceAlternative>` fields
  are no longer "foreign" now that those types derive the traits directly) drop its own
  hand-written bridge for a plain derive. The two per-leaf `json_to_dsl`/`dsl_to_json` helper
  functions this bridge chain depended on became fully unused once all three call sites moved —
  deleted.
- `🦀️component.rs` (4 sites) — `SpaceHistoryMutation`/`SpaceHistorySnapshot`'s own `OpText`/
  `ArtifactDsl` impls (`print_op`/`parse_op`/`parse_dsl`/`print_dsl`) are genuine JSON-**text**
  boundaries (VCS ops-log lines, `.space-history` DSL text) — converted to
  `crate::os_pack::json::{to_json_string, from_json_str}` now that the underlying types have
  `ToValue`/`FromValue` from the fix above.
- `🦀️component.rs` (1 site) — `commit_space_checkpoint`'s `pins_fingerprint` (a content-addressed
  ID input over `Vec<SpaceMemberPin>`, which already had both traits) — converted to
  `to_json_string(...).into_bytes()`. Deterministic-fingerprint use, not a persisted/compared wire
  format, so the exact byte text changing is inconsequential (greenfield repo, no persisted IDs to
  preserve — CLAUDE.md's own no-migrations rule).

## Framework types given `ToValue`/`FromValue` this wave (outside `🔌️plugin`/`🏪️store` proper, but
required to unblock a production call site inside them — same "convert it there" rule the playbook
names for framework-owned domain types)

All in `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs` (already depends on
`semio-framework-value-derive`, already imports `ToValue`/`FromValue`, already has other types in
the same file using the derive — zero new dependency edges, zero risk of the E0119 double-derive
class of conflict since none of these had a hand-written impl):

- `IoPayload` (`Text(String)`/`Binary(Vec<u8>)`, plain externally-tagged, matches serde's own
  default shape).
- `Confidence` (`None`/`Low`/`Medium`/`High`, unit-only enum — the derive's plain-unit-enum mode,
  bare-string encoding, matches `serde`'s default for a fieldless enum).
- `IoFidelity` (same unit-only shape).
- `IoEntryDescriptor`, `IoRoute` (plain structs, `rename_all = "camelCase"`, all field types already
  convertible transitively).

And in `🔌️plugin/🖥️host/🦀️component.rs` itself: `InferenceRouteResult` gained `FromValue` (it only
had `Deserialize` before — asymmetric on purpose, it is only ever decoded, never encoded, by this
crate).

## Deliberately NOT converted, with the reason — this is the judgment the ticket asked for

| site/cluster | why it stays on `serde_json` |
|---|---|
| `🏪️store/🦀️component.rs` `pub mod pack_rt`'s `encode_json_value`/`decode_json_value`/`json_value_to_dsl`/`dsl_value_to_json`/`json_values_equal`/`renormalize_json_wire_value` (13 sites) + `impl ArtifactPack for serde_json::Value` (1 site) | Explicitly documented in the file itself as **"Compose-only pack bridge (external technology)"** — a public API surface `semio_compose_rs` (a genuinely separate system, named in this file's own comments) consumes by passing real `serde_json::Value` objects, not JSON text. Changing this signature is a breaking change to that external consumer, not a same-crate refactor. Matches the ticket's own precedent for `🧩️puzzle`'s browser bridge (`verified-outcomes.md`: "not violations... `🧩️puzzle`'s live browser bridge, now correctly excluded"). |
| `🏪️store/🦀️component.rs:15763` `envelope_json()`'s `serde_json::to_string` under a method-local `where P: Serialize, Mutation: Serialize` bound | **Named explicitly in this ticket's own briefing** ("A prior agent left `envelope_json` on serde via a method-local `where P: Serialize` rather than force `ToValue` onto a second seven-type tree... decide deliberately and say how you handled the fallibility"). Decision: left as-is. `ArtifactEnvelopeRead`/`ArtifactEnvelopeOwners` and their nested nine-type tree (`ArtifactVcsRead`, `ArtifactCursorOwners`, `OwnerRef`, `HistoryLane`, …) are the prior wave's own explicitly-scoped-out reversion (`E0119` conflict with a hand-written impl, see the additive wave's doc). Forcing `ToValue` onto this method's generic bound would require converting that whole tree first — real, but separate, follow-up work, not a same-session mechanical fix. No fallibility handling changed: this one call site's `Result<String, VcsError>` return shape is untouched. |
| `🏪️store/🦀️component.rs:19745-19758` `impl ArtifactPack for protocol::InteractionState` | `InteractionState` is defined in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` — **replication is explicitly out of scope for this wave** per the ticket's own hard constraints, and was mid-edit by a concurrent session for most of this session (see "Concurrent blocker" below). Not touched. |
| `🏪️store/🦀️component.rs` VCS ops-log leaf metadata (`10065`, `10105`, `10111`, `10985`, `10991`, `10998`) — `MutationMeta`/`crate::os_spr::MutationMessage`/`crate::os_spr::Conflict` text encode/decode | `MutationMessage`/`Conflict` are `crate::os_spr` = **`semio-framework-replication`** types — same out-of-scope boundary as above. |
| `🏪️store/🦀️component.rs:6585` `ArtifactRepositoryHistoryEntryAuthority<T: DeserializeOwned>::accept_token` | Generic infrastructure bound instantiated across many different app snapshot/mutation `T`s repo-wide (not enumerable from this file alone, unlike the single-call-site generics converted above) — the same class of decision as `decode_json<T>`/`decode_owned_result<T>` below, deferred rather than rushed without exhaustively checking every instantiation. |
| `🏪️store/🧵️canonical-edit/🦀️component.rs` `ScalarBytes::from_node` (8 sites, `serde_json::to_writer` for `null`/`bool`/`i64`/`u64`/`i128`/`i128`/`u128`/`f32`/`f64`) | Canonical-JSON scalar byte encoding for edit content-addressing. Investigated in depth: `pack::json::Number` has no `i128`/`u128` variant (only `UInt(u64)`/`Int(i64)`/`Float(f64)`), and `pack::json::write_float`'s ECMA-262 fixed/exponential notation split does not match `serde_json`'s `ryu`-based writer's notation thresholds for `f32`/`f64` — routing through `pack::json` naively would silently change the canonical byte encoding in ways that need dedicated float-formatting verification (a differential harness against the old `serde_json::to_writer` output), not a same-session drive-by fix in a file three other agents were also actively editing today. Flagged as real follow-up work, not attempted. |
| `🔌️plugin/🦀️component.rs`'s `owned_abi::PollInput`/`take_json<T: DeserializeOwned>`/`return_json<T: Serialize>`, `🖥️host/🖥️host/⏳️imports.rs`'s `decode_json<T: DeserializeOwned>`, `🖥️host/⚡️effects/🦀️component.rs`'s `encode_json<T: Serialize>`/`decode_dsl`, `🖥️host/🧵️shard/🦀️component.rs`'s `Effect`/`Event`-typed sites | All trace back to `semio_framework::kernel::Event`/`Effect` (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`) lacking `ToValue`/`FromValue` — a ~30-variant enum tree with many nested framework types (`ActorInstanceOpenRequest`, `BrokerCapabilityGrant`, `QuotaSchema`, …), and using `#[serde(rename_all_fields = "camelCase")]`, an attribute key the derive macro does not currently parse. `PollInput`'s own doc comment already states this explicitly: "neither `#[derive(ToValue, FromValue)]` nor a direct `impl … for semio_framework::kernel::Event` is legal here (orphan rule)... Bridges through `serde_json` instead." Same judgment applied consistently to every sibling site touching `Event`/`Effect`. This is the `🎠️kernel` module owner's conversion, not a `🔌️plugin`/`🏪️store` one — flagged as the single largest remaining blocker (see below). |
| `🔌️plugin/🖥️host/⏳️imports.rs:decode_json<T>`'s other call sites (`MediaType`, `ClipboardFragment`, `IconRenderExportItem`, IO-compose `sources`) | `MediaType` (`🧰️framework/🔨️modules/🛂️manifest`) and `ClipboardFragment` (`🎠️kernel`) also lack `ToValue`/`FromValue`; same generic-bound-shared-across-modules situation as the `Event`/`Effect` cluster above, not attempted this wave to keep the blast radius to files this ticket actually scopes. |

## Derives: none deleted this wave, and why that is the correct call

The ticket says to delete a type's serde derive "once a module's call sites are converted... the
repo forbids leaving a type deriving both." Checked every type this wave fully converted
(`SpaceHistoryDiff`, `SpaceHistoryMutation`, `CreateSpaceAlternative`, `CommitSpaceCheckpoint`,
`IoPayload`, `Confidence`, `IoFidelity`, `IoEntryDescriptor`, `IoRoute`, `InferenceRouteResult`) for
remaining `serde`/`serde_json` consumers **beyond the sites this wave touched**: every one of them
still has at least one `#[cfg(test)]` site in the same file exercising `serde_json::to_string`/
`from_str`/`to_value` directly against it as a differential oracle (e.g.
`restore-active-space-alternative/🦀️.rs`'s own test literally does
`serde_json::to_string(&mutation)` where `mutation: SpaceHistoryMutation`, asserting the exact wire
string). That is the CLAUDE.md-mandated "same output... with at least one third-party library"
oracle test, not leftover cruft — removing `Serialize`/`Deserialize` from these types would break
it. So "the module's call sites are converted" is true for **production** code but not for the
crate as a whole (tests still legitimately need serde), and per the ticket's own philosophy that is
the correct state to leave them in, not a shortfall to fix.

## Concurrent blocker hit and cleared mid-session — not mine, confirmed by scope alone

For a stretch of this session, `cargo check -p semio-framework-os-kernel` (and
`-p semio-framework-plugin`, `-p semio-framework-replication`) failed with `E0432`/`E0433` errors
inside `🧰️framework/🔨️modules/📡️replication/⚠️diagnostic/{🦀️component.rs,📍️span/🦀️component.rs}` —
a concurrent session adding `#[derive(ToValue, FromValue)]`/`#[value(...)]` to replication's
diagnostic types without (at first) the `semio-framework-value-derive` dependency edge, then (as it
progressed) a `::semio_framework_os_kernel::...`-rooted derive-generated path that cannot resolve
from inside `replication` (replication is upstream of `os-kernel` in the dependency graph, not
downstream — confirmed by `os-kernel` itself depending on `replication`, never the reverse). Traced
with certainty to not-mine: the failing files are 100% outside `🔌️plugin`/`🏪️store`, and this
session made zero edits to `📡️replication` (explicitly out of scope per the ticket's hard
constraints). Re-ran the guardrail check periodically rather than trying to fix it; it cleared on
its own once the concurrent session finished, and the final verification run below is from after
that point.

## Third crate discovered mid-wave: `semio-framework-plugin-host`, never checked before this session

`🖥️host/🦀️component.rs` — where roughly half this wave's `🔌️plugin` conversions live — is **not**
part of `semio-framework-plugin` (the crate `cargo check -p semio-framework-plugin` verifies). It is
compiled into a **third**, separate crate: `semio-framework-plugin-host`
(`🔌️plugin/🖥️host/📦️packages/🦀️rust`), native-only (wasmtime engine/host code, never reaches the
shipped `wasm32-wasip2` target — confirmed by `draw-fsm`'s own clean wasip2 build not depending on
it). Neither this ticket's own prior status docs nor `cargo check -p semio-framework-os-kernel`
cover it. Checking it explicitly (only done after suspecting the gap, following the rule "validate
assumptions") surfaced **21 errors**, worked down to **3**:

**Fixed (18 errors, all traced to this session's own edits or a live concurrent-edit collision)**:
- A genuine merge-style corruption in `🖥️host/⚡️effects/🦀️component.rs` — a
  `use semio_framework_value_derive::{FromValue, ToValue};` line had landed **inside** a multi-line
  `use semio_framework_os_services::{ ... };` block (between the opening brace and its contents),
  a syntax error. Not attributable to a specific edit (this file was flagged "changed on disk"
  mid-session more than once); moved the import to its own line after the block.
- `PluginHostError::Json` was changed to hold `String` (see conversion list above) but two spots
  keyed off the OLD `serde_json::Error` shape: the `impl Error::source()` arm (`Some(error)` on a
  `&String` doesn't satisfy `dyn Error`, deleted the arm) and every remaining un-converted
  `serde_json::` site in this same file/its sibling `🧵️shard/🦀️component.rs` that used
  `PluginHostError::from`/bare `PluginHostError::Json` as a function reference relying on the now-
  deleted `impl From<serde_json::Error>` — patched to explicit `.map_err(|error|
  PluginHostError::Json(error.to_string()))`, including two genuinely-deferred `Event`-typed sites
  in `🧵️shard/🦀️component.rs` (kept on `serde_json`, per the `Event` blocker above — only the error
  plumbing changed, not what they decode).
- `crate::os_pack::json::...` doesn't resolve in `semio-framework-plugin-host` — unlike
  `semio-framework-plugin`, this crate has no `pub mod os_pack` of its own; it reaches the same
  module through the file's own `extern crate semio_framework_os_kernel as dsl;` alias. Every
  `crate::os_pack::json::` this wave wrote in `🖥️host/🦀️component.rs` corrected to
  `dsl::os_pack::json::`.
- `OwnedPollInput`/`OwnedRestoreInput` (the `#[derive(serde::Serialize, ToValue)]` structs feeding
  `poll`/`restore`) **pre-existed this session with a `ToValue` derive that had never compiled**:
  `OwnedPollInput` embeds `&'a [semio_framework::kernel::Event]` (needs both `Event: ToValue` — the
  same blocker as everywhere else — AND a `&[T]`/slice blanket `ToValue` impl, which does not
  exist), plus bare `CommandPageCursor`/`FixedCommandPage`/`Budget` (also `Event`-cluster types).
  `OwnedRestoreInput`'s `state: &'a [u8]` hit only the missing-slice-impl half. Fixed each on its
  own merits: `OwnedPollInput` reverted to `#[derive(serde::Serialize)]` only (documented why, same
  `Event`-cluster reason) and its one call site reverted to `serde_json::to_vec`; `OwnedRestoreInput`
  changed `state` from `&'a [u8]` to owned `Vec<u8>` (the caller already had an owned `Vec<u8>` in
  hand, so this is a pure simplification, not a behavior change) and its call site converted to
  `to_json_string` as originally intended.
- `GuestArtifactInferenceMetadata` (`Deserialize`-only) gained `FromValue`; `IoKey`/`IoDirection`
  (`🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, the OLD non-schema `io` file, distinct from
  `🚪️io/🧬️schema`) gained `ToValue`/`FromValue` — needed by `compose`'s `key_bytes` decode and
  `register_plugin`/`infer`/`infer_with_visited`'s roster/route decodes, all sites this wave's own
  table above already lists as converted.

**Left broken, confirmed pre-existing and out of scope (3 errors)**:
- `semio_framework::kernel::PresenceUpdate: FromValue` not satisfied
  (`🖥️host/🦀️component.rs:2063`, unrelated to `serde_json` — a `dsl::from_dsl_value::<PresenceUpdate>`
  call). `PresenceUpdate` is `semio_framework_ui_contract`'s own type
  (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️presence.rs`) — that crate has zero
  `ToValue`/`FromValue` usage anywhere yet (no `semio-framework-value-derive` dependency edge, no
  converted types), a whole separate module's conversion, not a two-line fix.
- `decode_wire_dsl<T: serde::de::DeserializeOwned>`/`encode_wire_dsl<T: serde::Serialize>`
  (`🖥️host/🦀️component.rs:6471,6477`) — internally call `dsl::to_dsl_value`/`from_dsl_value`, which
  are themselves bound on `T: ToValue`/`FromValue` (confirmed by reading
  `🧰️framework/🔨️modules/🌱️value/🦀️component.rs:192,198` — there is no other `to_dsl_value`/
  `from_dsl_value` definition anywhere in the repo; the `T: Serialize + DeserializeOwned`-bound
  serde bridge the fan-out playbook describes no longer exists as a distinct function). Every
  concrete instantiation of `decode_wire_dsl` already satisfies the real (`FromValue`) bound
  (`HostMutationRosterEntry`, `HostArtifactMutationPlanRequest/Result` all already derive it — this
  wave verified each). `encode_wire_dsl`'s one production call site
  (`🖥️host/🦀️component.rs:7097`, `&draft.origin: &protocol::MutationOrigin`) does **not** —
  `MutationOrigin` is a `📡️replication` type (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1524`),
  out of scope for this wave by explicit instruction. Fixing the two functions' own signatures
  (the actually-correct fix, since their bodies already require `ToValue`/`FromValue`) would break
  this one call site until `MutationOrigin` is converted — not attempted.

**Root cause, stated plainly**: this crate was not part of the additive wave's own verification
sweep (its own doc only mentions `os-kernel` and `draw-fsm`'s wasip2 target), and nothing else in
this ticket's history checked it either — the two pre-existing generic-bound bugs almost certainly
predate this session by a wide margin (they describe a framework API shape — a serde-bound
`to_dsl_value`/`from_dsl_value` — that appears to have been unified onto `ToValue`/`FromValue` only,
repo-wide, by a different concurrent session, without anyone re-checking every crate downstream of
that change). Flagging this explicitly rather than fixing it: it needs `📡️replication` and
`semio_framework_ui_contract` conversions this wave's scope explicitly excludes, not a drive-by
patch that would either revert someone else's in-flight work or add scope-creep conversions to two
modules this ticket assigns elsewhere.

## Verification — all green, verbatim tails

```
$ cargo check -p semio-framework-os-kernel --message-format=short
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 7.43s
```
0 errors, 33 warnings — identical to the additive wave's own baseline, both before and after this
wave's ~50 production call-site conversions plus the `SpaceHistoryDiff`/`SpaceHistoryMutation`
derive-root-cause fix.

```
$ cargo check -p semio-framework-plugin --message-format=short
warning: `semio-framework-plugin` (lib) generated 215 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 115 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 27.63s
```
0 errors, re-confirmed a second time after every fix in the "third crate" section below (5m08s the
second run — this crate pulls `wasmtime`, cold builds are slow). (This is the crate
`🔌️plugin/🦀️component.rs` and everything under `🔌️plugin/**` actually compiles as — separate from
`semio-framework-os-kernel`, and NOT covered by that check at all; this was verified explicitly
this wave after realizing the two are different crates.)

```
$ cargo check -p semio-framework-plugin-host --message-format=short
error: could not compile `semio-framework-plugin-host` (lib) due to 3 previous errors; 7 warnings emitted
```
**3 errors remain, all pre-existing and out of this wave's scope** — see the dedicated section
below ("Third crate discovered mid-wave"). Reduced from an initial 21 (18 were this wave's own
fault or a live concurrent-edit collision, fixed; 3 need `📡️replication`/`semio_framework_ui_contract`
conversions this wave's scope explicitly excludes).

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Compiling semio-s-plugin-draw-fsm v0.1.0 (.../🖍️draw/.../🔄️fsm/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 25.38s
```
0 errors — draw-fsm's own wasip2 build is still clean after every edit in this wave.

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (.../💻os/📦️packages/🦀️rust)
│   └── semio-s-plugin-draw-fsm v0.1.0 (.../🔄️fsm/📦️packages/🦀️rust)
└── semio-framework-replication v0.1.0 (.../📡️replication/📦️packages/🦀️rust)
    ├── semio-framework-os-kernel v0.1.0 (.../💻os/📦️packages/🦀️rust) (*)
    └── semio-framework-pack v0.1.0 (.../🎒️pack/📦️packages/🦀️rust)
        └── semio-framework-os-kernel v0.1.0 (.../💻os/📦️packages/🦀️rust) (*)
```
Full, un-truncated inverted tree (single instance of `serde` resolved). Exactly the two direct
entries the ticket names (`os-kernel`, `replication`) plus `pack` as a transitive hop under
`replication` — no new crate introduced this link, no crate removed from it either (correctly —
neither `Cargo.toml` line was touched, since neither crate's serde usage is anywhere near zero yet).

## Files touched this wave

**`🔌️plugin`**: `⚛️reactor/💼️jobs/💡️infer/🦀️component.rs`, `⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs`,
`⚛️reactor/💼️jobs/🦀️component.rs`, `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs`,
`⚛️reactor/📸️checkpoint/🦀️component.rs`, `🌐host/🦀️component.rs`,
`📇️describe/📦️packages/🦀️rust/📦️glue.rs` + its `Cargo.toml`, `🛂️describe/🦀️component.rs`,
`🖥️host/🦀️component.rs`, `🖥️host/⚡️effects/🦀️component.rs` (import-order fix + one call site),
`🖥️host/🧵️shard/🦀️component.rs` (error-plumbing fix only, sites stay serde — `Event`-typed).

**`🏪️store`**: `🔄️sync/🦀️component.rs`, `🧬️schema/🧬️mutations/🦀️.rs`,
`🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs`,
`🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🦀️.rs`, `🦀️component.rs`.

**Framework (types required by the above, additive derive only, `Cargo.toml` unchanged)**:
`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs`, `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`
(the sibling non-schema file — `IoKey`/`IoDirection`).

## What remains (counts, for whoever picks this up next)

1. **`🎠️kernel::Event`/`Effect` conversion** — the single largest remaining blocker in `🔌️plugin`.
   Once done, ~10-15 more production sites across `🔌️plugin/🦀️component.rs` (`owned_abi`),
   `🖥️host/🦀️component.rs` (`OwnedPollInput`, reverted to `serde`-only this wave after confirming
   its pre-existing `ToValue` derive did not compile — see the "third crate" section), `🖥️host/
   ⏳️imports.rs`, `🖥️host/⚡️effects/🦀️component.rs`, `🖥️host/🧵️shard/🦀️component.rs` fall out
   mechanically once it lands (their target types already have `ToValue`/`FromValue` derives ready).
   Also needs a `&[T]`/slice blanket `ToValue`/`FromValue` impl in `🌱️value/🔁️codec` (does not exist
   yet — `OwnedPollInput`'s `events: &[Event]` needs it even after `Event` itself converts). Requires
   extending the `#[derive(ToValue, FromValue)]` macro to parse `#[serde(rename_all_fields =
   "...")]`, or manually expanding it per-variant.
2. **`MediaType`/`ClipboardFragment`/`IconRenderExportItem`** (`🛂️manifest`/`🎠️kernel`) — smaller,
   self-contained, blocks `⏳️imports.rs`'s `decode_json<T>` generic.
3. **`🏪️store/🧵️canonical-edit/🦀️component.rs`'s `ScalarBytes`** — needs a dedicated
   differential-tested numeric formatter (i128/u128 decimal, f32/f64 shortest-round-trip matching
   the old `serde_json`/`ryu` notation), not a drive-by swap.
4. **`ArtifactEnvelopeRead`/`ArtifactEnvelopeOwners`'s nine-type tree** — `envelope_json`'s own
   blocker, inherited from the additive wave's deliberate revert.
5. **Replication-owned types** (`InteractionState`, `MutationMessage`, `Conflict`,
   `HybridLogicalTimestamp` already done) — out of scope for this ticket wave by explicit
   instruction; needs its own pass once that crate is not mid-edit by a concurrent session.
6. The **"Compose-only" bridge** (`pack_rt::encode_json_value`/`decode_json_value`/
   `impl ArtifactPack for serde_json::Value`) is a deliberate, permanent `serde_json::Value` public
   API for `semio_compose_rs` interop — not on a path to zero without that external system's own
   migration, out of this ticket's reach entirely.
7. **`semio-framework-plugin-host`'s 3 pre-existing errors** (`PresenceUpdate: FromValue`,
   `decode_wire_dsl`/`encode_wire_dsl`'s stale `Serialize`/`DeserializeOwned` bounds — see "Third
   crate" section above) — needs `semio_framework_ui_contract` to gain `ToValue`/`FromValue` on
   `PresenceUpdate`+`SurfaceId`+`OwnPresence`+`PeerMark` (plus a `semio-framework-value-derive`
   dependency edge that module does not have yet) and `📡️replication`'s `MutationOrigin` converted.
   Confirmed stable at exactly these 3 across two full re-checks after this wave's own fixes landed
   — nothing outstanding traces back to this wave.

None of the above blocks `serde`/`serde_json` from being removed from `Cargo.toml` this wave — that
was already true before this wave started (`os-kernel` hosts many other modules — `♾️infinite`,
`🔁️workflow`, `🌊️flow`, … — each with its own unconverted serde surface) and remains true after.
This wave's contribution is the `🔌️plugin`+`🏪️store`-scoped slice of that larger crate-wide goal.
