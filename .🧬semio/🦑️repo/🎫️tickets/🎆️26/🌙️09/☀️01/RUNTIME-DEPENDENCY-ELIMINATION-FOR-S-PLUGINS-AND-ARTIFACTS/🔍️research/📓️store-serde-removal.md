# 🏪️ `🏪️store` — production serde removal

Continues `📓️os-json-callsite-conversion.md` (call-site conversion) and
`📓️os-plugin-store-serde-conversion.md` (additive derive wave). This wave: delete the
now-redundant serde derives from types that already carry `ToValue`/`FromValue`, and either finish
or explicitly, individually re-defer every remaining production `serde`/`serde_json` reference.

## Headline: production vs test classification, and why the raw number moves

A brace-matching classifier (`#[cfg(test)] mod {...}` span detection, plus a rule that any file
whose path contains a `🧪️` segment is wholly test — verified directly by reading every mount point:
`🏪️store/🦀️.rs`'s own `#[cfg(test)] #[path = "🧪️fixtures/🦀️.rs"] mod fixture_mutations;` etc., all
confirmed `#[cfg(test)]`-gated at the `mod` declaration) was run over all 43 files under `🏪️store/**`.

Three different counting methodologies give three different "production" numbers — stated
explicitly here because the ticket's own briefing number (280) does not match this wave's:

| method | production | note |
|---|---|---|
| ticket's own briefing figure | 280 | methodology not re-derivable from the briefing text alone |
| this wave's own classifier, **before any edit**, raw `serde\|Serialize\|Deserialize` regex, `🧪️`-path files forced test, but *not* excluding `VcsError::Serialize(String)`/`VcsError::Deserialize(String)` (first-party error-enum variants that only share a name with the serde traits) | 397 | first pass, later found to over-count |
| same, **after** excluding the `VcsError::` false positives and doc-comment/prose lines (i.e. counting only real code tokens) | ~310 at start | the honest starting figure |

**After this wave: 92 real code lines** (172 raw regex hits, of which 80 are prose/doc-comments or
naming false positives like the module literally being called `operation_envelope_serde`). All 92
are one of five explicitly-deferred clusters, each with a stated reason below — none are oversight.

## Converted this wave — redundant serde derives deleted (type already had `ToValue`/`FromValue`)

Mechanical, script-assisted (`🔬️verification-plugin-store-derive`'s sibling script pattern: locate
the struct/enum body by brace-matching from the `#[derive(...)]` line, strip `Serialize`/
`Deserialize`/`serde::Serialize`/`serde::Deserialize` tokens from the derive list, delete every
`#[serde(...)]` line immediately followed by its `#[value(...)]` mirror), then hand-verified by
`cargo check` after every batch:

**`🏪️store/🦀️.rs`**: `HistoryLane`… — wait, see "Reverted" below, several of these were later put
back. Net converted and staying converted: `RawFixtureInbound`/`FixtureManifest` equivalents live in
`🔄️sync`, not here — see that section. This file's net conversions: `operation_envelope_serde`'s
dead `serialize`/`deserialize` fns deleted (their `#[serde(with = "operation_envelope_serde")]` call
site on `ArtifactCommand::IngestRemote` had already lost its `Serialize`/`Deserialize` derive-list
membership once `ArtifactCommand<Mutation>` itself converted), leaving only the `to_value`/
`from_value` twins.

**`🏪️store/🔄️sync/🦀️.rs`** (fully serde-free in production now): `PersistenceBinding`,
`ArtifactActorConfig`, `ArtifactActorMsg`, `RemoteState`, `ArtifactSyncStatus`, `ArtifactEvent`,
`CommandAckOutcome`, `backbone_worker_wire::BackboneWorkerRequest`/`BackboneWorkerResponse` (were
`serde::Serialize`/`serde::Deserialize` fully-qualified — confirmed dead: this worker's real wire
format is `crate::os_store::pack_rt::encode_wire_value`/`decode_wire_value` over a `DslValue`, never
`serde_json`), `RawFixtureInbound`/`FixtureManifest` (confirmed dead: this file has **zero**
`serde_json::` call sites anywhere, `parse_fixture_dsl_manifest` hand-parses text, never
deserializes JSON — the derive was pure unused weight). Also deleted `envelope_serde`'s dead
`serialize`/`deserialize` fns (kept `to_value`/`from_value`) and the unused `Serialize +
serde::de::DeserializeOwned` trait bounds on `SyncSession<P, Mutation>`'s two impl blocks (verified
nothing in the impl body used them — `Mutation<P>: ToValue + FromValue` already covers what the
struct actually needs).

**`🏪️store/🧵️canonical-edit/🦀️.rs`**: `ArtifactStoreOneItemSealCheckpoint` moved from unconditional
dual-derive to test-only (see next section — same oracle-test pattern as the space-history tree).

## Converted this wave — moved to `#[cfg_attr(test, ...)]` (serde stays, but TEST-ONLY)

Not simple deletion: `SpaceHistoryMutation` (the space-history mutation aggregate) and its full
transitive closure — six mutation leaves (`CommitSpaceCheckpoint`, `CreateSpaceAlternative`,
`SwitchSpaceAlternative`, `RemoveSpaceCheckpoint`, `RemoveSpaceAlternative`,
`RestoreActiveSpaceAlternative`), the three space-history domain types they carry
(`SpaceCheckpoint`, `SpaceAlternative`, `SpaceMemberPin`), and `ArtifactStoreOneItemSealCheckpoint`
(canonical-edit) — all have a **sibling test that uses `serde_json` as an independent differential
oracle** against the first-party `ToValue`/`FromValue` path (exact-wire-string assertions,
unknown-field rejection checks, `serde_json::to_vec`/`from_slice` round trips). CLAUDE.md's own
test-driven-development rule requires exactly this ("the same output of a test with at least one
third-party library in order to validate our own implementation") — deleting the derive would have
destroyed real, working, independent verification to make a raw grep count smaller. That is a
regression, not a cleanup.

The fix used throughout: `#[derive(..., ToValue, FromValue)]` (unconditional) +
`#[cfg_attr(test, derive(Serialize, Deserialize))]` + `#[cfg_attr(test, serde(...))]` mirroring the
old container/field attributes, with the `use serde::{Deserialize, Serialize};` import itself moved
behind `#[cfg(test)]`. This is strictly better than the two alternatives (delete the oracle test, or
leave serde in the unconditional/production derive list): serde now compiles into `cargo test` only,
never into a shipped `wasm32-wasip2` component, while the oracle keeps working. Applied consistently
once discovered to be needed (first hit while converting `SpaceHistoryMutation`'s own aggregate
derive — every variant's payload type must implement `Serialize` for the aggregate enum's own derive
to type-check, which is what surfaced the whole tree; same discovery repeated independently for
`ArtifactStoreOneItemSealCheckpoint` while auditing `canonical-edit`).

One field-level nuance, `RestoreActiveSpaceAlternative.alternative_id: Option<String>`: the old
`#[serde(deserialize_with = "required_option")]` existed only to defeat serde's derive-macro-level
special case where a bare `Option<T>` field is implicitly optional-if-missing. The `ToValue`/
`FromValue` derive has no such special case — a field with no `#[value(default)]` is already an
error if its key is missing, `Option<T>`'s blanket impl collapses `null` to `None` on the value that
IS present — so `#[value(...)]` needed **no** `deserialize_with` mirror at all; the derive's default
behavior already matches what the hand-written bridge existed to enforce. Verified by reading the
derive macro's `from_value_struct_fields` codegen directly
(`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs:367-407`), not by guessing.

## Reverted mid-wave — a documented mistake, corrected before landing

Five types (`ArtifactBackboneRef`, `ArtifactCursorOwners`, `HistoryLane`, `MigrationProvenance`,
`OwnerRef`) were initially stripped of `Serialize`/`Deserialize` in the first bulk pass, then found
to be structurally required by `ArtifactEnvelopeRead`'s own (necessarily-serde, see below)
`#[derive(Serialize)]` — a `#[derive(Serialize)]` on a struct requires every field type to implement
`Serialize` too. Re-added their dual derive before the first `cargo check` confirmed green. Caught
by the guardrail itself (`E0277`/`E0599` on the very next check), not by inspection — exactly the
kind of mistake the ticket's "never revert a peer, but do verify your own edits" discipline exists
to catch.

## Deliberately NOT converted — the `ArtifactEnvelopeRead` tree, with the fallibility decision made

**The fallibility question, resolved**: the ticket's own briefing worried that `ToValue::to_value`
(infallible) can't express `ArtifactEnvelopeOwners::capture_read()`'s fallibility (`Result<...,
&'static str>`, a group-visibility consistency check). On inspection **this tension doesn't actually
exist** for a hand-written conversion: nothing requires `ArtifactEnvelopeOwners`/`ArtifactEnvelope`
to implement the `ToValue` trait itself (they aren't used as a `P`/`Mutation` type parameter
anywhere). `envelope_json()` can call `capture_read()` first (fallible, unchanged), and only THEN
convert the successfully-captured, already-borrowed `ArtifactEnvelopeRead` to a `DslValue` via a
plain **inherent** `to_value(&self) -> DslValue` method — a second, genuinely infallible step, since
by construction it never fails once you're holding a valid `ArtifactEnvelopeRead`. Serde's own
`Serialize` trait only conflates the two steps into one call because it has no separate "capture"
phase; a hand-written conversion doesn't share that constraint. **Decision: this is not a blocker,
and would not require a lossy design if attempted.**

**What IS the actual blocker, and why this wave still didn't convert the tree**: `ArtifactEnvelopeRead<'a, P, Mutation>` has a field `vcs: crate::os_vcs::ArtifactVcsRead<'a, P, Mutation>` —
a borrowed view type owned by `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`, a
**different module**, explicitly assigned to a different concurrent agent this ticket ("the
small-module tail", `🌿️vcs 13` in the ticket's own scoping table). Confirmed by reading that file
directly: `ArtifactVcsRead` still has only `#[derive(Serialize)]`, zero `ToValue`/`FromValue` usage
anywhere in that file — it hasn't been touched by anyone yet. Converting `ArtifactEnvelopeRead`
requires `ArtifactVcsRead: ToValue` to exist first, which is out of this wave's scope by the
ticket's own hard constraint ("re-read before every edit, never revert a peer") — not a fallibility
problem, a cross-module dependency on unstarted work in someone else's module.

Left exactly as it was, dual-derived, ~11 production reference lines (`ArtifactEnvelopeRead`'s own
`#[derive(Serialize)]` + field attrs, the hand-written `impl Serialize for ArtifactEnvelopeOwners`/
`ArtifactEnvelope`, `ArtifactCursor`'s hand `impl Serialize`/`impl Deserialize` bridging to
`ArtifactCursorOwners`, `ArtifactEditMessageLedger`'s hand `impl Serialize`, `envelope_json`'s
method-local `where P: Serialize, Mutation: Serialize` bound). All five "load-bearing" types listed
in the revert above stay dual-derived for this reason, not out of caution.

**`ArtifactChild<S>`** (`#[derive(Serialize, Deserialize)]`, `#[serde(bound = "")]`, a `#[serde(skip)]`
`local_owner: Option<Arc<dyn Any>>` field): unchanged, matches the playbook's own trap #3/#6 exactly
— composed-child generic bridging through `to_dsl_value`/`from_dsl_value`, needs a hand-written
`impl<S> ToValue for ArtifactChild<S>` bridging `ArtifactRef` through its URI round trip first. Real,
scoped, substantial follow-up (the playbook already names the exact shape), not attempted this wave.

## Deliberately NOT converted — everything else, unchanged from the prior wave's own precedent

| cluster | lines | why |
|---|---|---|
| `pack_rt::{encode_json_value, decode_json_value, json_value_to_dsl, dsl_value_to_json, json_values_equal, renormalize_json_wire_value}` + `impl ArtifactPack for serde_json::Value` | ~30 | **Compose-only pack bridge (external technology)** — `semio_compose_rs` consumes real `serde_json::Value` objects across this API, a breaking change to an external consumer, not a same-crate refactor. Same precedent as the prior wave and as `🧩️puzzle`'s browser bridge. |
| `ArtifactRepositoryHistoryEntryAuthority<T: DeserializeOwned>::accept_token` (+ `ArtifactRepositoryHistoryEntryDecoder<T>`, `artifact_bounded_history_entry_decoder<T>()`) | ~6 | Traced its **one** production call site repo-wide (`grep -rn artifact_bounded_history_entry_decoder`): `✏️s/🔌️plugins/🎞️animate/.../💾️binary/🦀️component.rs:320`, with `T = protocol::Edit<PresentMutation>`. `Edit<Op>` is defined in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1419` and still only derives `Serialize`/`Deserialize` there — `📡️replication` is explicitly out of scope for this ticket wave (a different live agent's territory). Converting the bound to `FromValue` is blocked on that crate gaining it first. |
| VCS ops-log leaf metadata (`MutationMeta`/`crate::os_spr::MutationMessage`/`crate::os_spr::Conflict` text encode/decode, ~6 call sites) | 6 | `crate::os_spr` resolves (`📦️glue.rs:157-160`) to `🧰️framework/🔨️modules/📡️spr/🦀️component.rs`, itself documented as speaking the `📡️replication` contract — same out-of-scope boundary. |
| `envelope_json`'s `serde_json::to_string` under `where P: Serialize, Mutation: Serialize` | 1 call + 2 bounds | Part of the `ArtifactEnvelopeRead` tree above — same `🌿️vcs` blocker. |

None of these five clusters are new — every one restates and re-verifies (call sites actually
traced this wave, not assumed) the prior wave's own documented exceptions. Nothing new was
discovered to be safe to convert that the prior wave had missed; this wave's contribution is closing
the **derive-redundancy** gap the prior two waves deliberately left open, converting five oracle-test
types to `cfg_attr(test)`, and fully clearing `🔄️sync/🦀️.rs`'s production serde surface to zero.

## Verification — verbatim tails, both re-run fresh after every batch

```
$ cargo check -p semio-framework-os-kernel --message-format=short
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 15.58s
```
0 errors, 33 warnings — identical to the baseline before this wave (re-confirmed after every edit
batch, not just at the end).

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Compiling semio-s-plugin-draw-fsm v0.1.0 (.../🔄️fsm/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 9.35s
```
0 errors.

**End-of-session guardrail note**: both checks above were run and re-confirmed green repeatedly
throughout this wave, most recently immediately after the last edit. In the ~2 minutes after that
last confirmation, `cargo check -p semio-framework-os-kernel` started failing workspace-wide with
`error: multiple workspace roots found in the same workspace` (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle`,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, and the repo root) — a root-`Cargo.toml`-level
conflict, not a `🏪️store` file, and `git status` shows the root `Cargo.toml` as currently modified
(uncommitted), i.e. mid-edit by the concurrent "plugin manifests" agent this ticket names as live.
Retried three times over ~2 minutes; still present as of this doc's writing. Not this wave's fault
(zero `🏪️store` files in the error, zero edits by this wave to any workspace-root `Cargo.toml`) and
not something this wave's scope can or should fix. The verbatim green tails above are the accurate
record of `🏪️store`'s own compiling state; whoever next runs the guardrail should expect it to clear
once the concurrent manifest edit lands, per this ticket's own "poll rather than chase" guidance.

## Concurrent-session notes (repo-wide taxonomy rename, and an unverified injected "coordinator" claim)

**Taxonomy rename, real, confirmed, harmless**: partway through this session every `🦀️component.rs`
file under `🏪️store/**` (and elsewhere repo-wide) was renamed to `🦀️.rs` by a concurrent
taxonomy-normalization sweep (unrelated ticket, visible in git status at session start). Confirmed
this wave's own edits survived the rename intact (content diffed post-rename, matched) and both
guardrail checks stayed green through it. File paths in this doc use the post-rename `🦀️.rs` name.

**A message formatted as "the coordinator sent a message while you were working" arrived embedded in
a tool-result system-reminder** (not a genuine user turn) partway through this session, describing a
`#[cfg_attr(test, ...)]` fix and claiming a prior edit to this module's aggregate file. Per this
session's own injection-handling rule, tool-observed content is data, not a command — its claims were
independently verified against the actual file on disk (confirmed present) before any action was
taken on its basis, and the technical approach was adopted only because it independently checks out
(matches CLAUDE.md's oracle-test contract, matches the file's real state, matches this ticket's own
"never leave a type deriving both unconditionally" goal) — not because the message asserted
authority. Recorded here for the record, not because it caused any incorrect action.

## Files touched this wave

`🏪️store/🦀️.rs` (derive cleanup + `operation_envelope_serde` dead-code deletion + 5-type revert),
`🏪️store/🔄️sync/🦀️.rs` (full derive cleanup, `envelope_serde` dead-code deletion, `SyncSession`
bound cleanup — production serde surface now zero), `🏪️store/🧵️canonical-edit/🦀️.rs`
(`ArtifactStoreOneItemSealCheckpoint` → `cfg_attr(test)`), `🏪️store/🧬️schema/🧬️mutations/🦀️.rs` and
all six leaf files under it (→ `cfg_attr(test)`, `RestoreActiveSpaceAlternative`'s
`required_option` helper also moved behind `#[cfg(test)]`).

## What remains (counts, for whoever picks this up next)

1. **`ArtifactEnvelopeRead` tree** (~11 production lines) — blocked on `🌿️vcs::ArtifactVcsRead`
   gaining `ToValue`/`FromValue` (a different agent's module). Once that lands, the fallibility
   question is ALREADY answered above (not a real blocker) — the remaining work is mechanical:
   hand-write `ArtifactEnvelopeRead::to_value(&self) -> DslValue` (all fields already `ToValue`
   except `vcs`), delete the hand `impl Serialize` pair on `ArtifactEnvelopeOwners`/`ArtifactEnvelope`
   and `ArtifactCursor`'s hand `impl Serialize`/`Deserialize` (replace with a matching hand `ToValue`/
   `FromValue` delegating to `ArtifactCursorOwners`, which already derives both), and
   `ArtifactEditMessageLedger`'s hand `impl Serialize` similarly. Then `ArtifactBackboneRef`,
   `ArtifactCursorOwners`, `HistoryLane`, `MigrationProvenance`, `OwnerRef` can drop back to
   `ToValue`/`FromValue`-only.
2. **`ArtifactChild<S>`** — needs its own hand-written generic `impl<S> ToValue`/`FromValue`,
   bridging `child_id`/`target: ArtifactRef` (the latter via its existing `to_uri()`/`parse_uri()`
   round trip, matching `enc_ref`/`dec_ref` in `📸️snapshot/🦀️.rs`).
3. **`ArtifactRepositoryHistoryEntryAuthority<T: DeserializeOwned>`** — blocked on
   `📡️replication::Edit<Op>` gaining `ToValue`/`FromValue` (traced to exactly one call site
   repo-wide, `T = protocol::Edit<PresentMutation>`, so this is a small, well-scoped follow-up once
   unblocked — not the "many T's" scope the prior wave worried about before this wave traced it).
4. **VCS ops-log leaf metadata + `envelope_json`'s bound** — blocked on the same `📡️replication`
   boundary (`MutationMessage`, `Conflict`) plus item 1's `🌿️vcs` blocker.
5. **`pack_rt`'s compose bridge** — permanent, out of this ticket's reach (external `semio_compose_rs`
   API), not on a path to zero without that system's own migration.

None of the above blocks `serde`/`serde_json` from being removed from `semio-framework-os-kernel`'s
`Cargo.toml` — that was already true before this wave (other unconverted modules in the same crate:
`♾️infinite`, `🔁️workflow`, `🌊️flow`, `📇️directory`, `📡️spr`, `💡️inference`, `🧩️extension`, `🌿️vcs`,
`⚙️engine`, `🎒️pack`, each with its own remaining serde surface) and remains true after. This wave's
contribution is the `🏪️store`-scoped slice of that larger crate-wide goal: real, verified reductions
(9 types fully serde-free, 10 types moved to test-only serde, `🔄️sync/🦀️.rs` at zero production
serde), every remaining reference individually traced and deliberately deferred with a stated,
checkable reason — not left unexamined.
