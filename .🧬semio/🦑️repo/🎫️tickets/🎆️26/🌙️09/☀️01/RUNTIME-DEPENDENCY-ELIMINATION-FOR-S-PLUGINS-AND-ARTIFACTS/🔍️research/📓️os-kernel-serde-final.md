# 🏁️ os-kernel serde — `BackboneDocument` converted, `🌿️vcs`/`🏪️store` reduced, `📡️spr`/`💡️inference` re-confirmed blocked

Continues `📓️os-kernel-serde-endgame.md`. That wave named four live blockers and left all four
unmoved. This wave converted the highest-value one (`BackboneDocument`), which the compiler then
used to enumerate a second, real, previously-unknown blocker (`ArtifactRepositoryHistoryEntryAuthority<T>:
DeserializeOwned`, instantiated directly over `Checkpoint`/`Alternative`, not only over
`Edit<PresentMutation>` as the ticket's own briefing framed it) — reverted exactly the four affected
types, kept everything else. Net: real, compiler-verified reduction on two of the four modules.

## Headline — code-only production serde counts (this wave's own classifier, `🗑️generated/classify_serde5.py`, unmodified)

| module | before this wave | after this wave | delta |
|---|---|---|---|
| `🏪️store` | 41 | **31** | **-10** |
| `🌿️vcs` | 39 | **22** | **-17** |
| `📡️spr` | 9 | **9** | 0 (re-confirmed still correctly blocked) |
| `💡️inference` | 7 | **7** | 0 (re-confirmed still correctly blocked) |
| **total** | **96** | **69** | **-27 (-28%)** |

## `BackboneDocument` — CONVERTED, the wave's actual unblock

`💻️os/🖥️host/🦀️.rs`'s `BackboneDocument<P, Op>` (`#[derive(Clone, Debug, PartialEq, Serialize,
Deserialize)]` over `vcs: ArtifactVcs<P, Op>`) — the blocker every prior wave back to
`📓️directory-spr-vcs-final.md` named and declined to touch — dropped `Serialize`/`Deserialize`
outright and gained a hand-written `impl<P: ToValue, Op: ToValue> ToValue` /
`impl<P: FromValue, Op: FromValue> FromValue`. Every field already had `ToValue`/`FromValue`
(`ArtifactVcs`, `store::ArtifactCursor`, `protocol::EditMessages`/`Conflict`, `ArtifactBackboneRef`
— confirmed by direct read before editing, not assumed); no caller anywhere in the crate required
`BackboneDocument: Serialize` as a bound or called `serde_json` on one directly (every
`serde_json::to_*`/`from_*` in the file was read first). `dsl::{DslValue, FromValue, ToValue,
ValueError}` are already reachable inside `mod host` via the same `protocol::` extern-prelude path
the file already used successfully for `Mutation`/`EditMessages`/`Conflict` (traced to
`semio-framework-replication`'s `pub use crate::value::*;`, itself mounting `🌱️value/🦀️.rs`).

**A methodological note for whoever reads the predecessor's writeup**: an earlier attempt in this
same session to verify a clean baseline via `cargo check -p semio-framework-os --features
os-host-full 2>&1 | tail -80` reported a misleading `exit 0` — that exit code belongs to `tail`, not
`cargo`, and piping through it silently discards the real exit status. Re-ran unpiped
(`> logfile 2>&1`, so the captured code is `cargo`'s own) to get a trustworthy signal before editing
anything. Flagging this explicitly since this ticket has repeatedly documented self-assessments that
turned out wrong; this is a concrete, avoidable instance of the same failure class.

## Real, compiler-found blocker #2 — `ArtifactRepositoryHistoryEntryAuthority<T>: DeserializeOwned`, over `Checkpoint`/`Alternative` directly

Following `BackboneDocument`'s conversion, gated `Author`/`CompositionPin`/`Checkpoint`/
`Alternative`/`ArtifactHistoryLedger`/`ArtifactHistoryIter`/`ArtifactVcs`/`ArtifactVcsRead` all off
serde in one pass and ran `cargo check -p semio-framework-os-kernel`. It named two real,
previously-undocumented consumers:

```
🏪️store/🦀️.rs:8686: error[E0277]: the trait bound `os_vcs::Checkpoint: serde::Deserialize<'de>` is not satisfied
🏪️store/🦀️.rs:8687: error[E0277]: the trait bound `os_vcs::Alternative: serde::Deserialize<'de>` is not satisfied
```

Traced to `🏪️store/🦀️.rs:6625`: `impl<T: DeserializeOwned + Send + Sync + 'static>
ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T>` — a streaming,
`serde::Deserializer`-token-driven decode authority (`ArtifactRepositoryHistoryEntryAuthority<T>`,
`accept_token`/`take_value`/`close_step`), not a whole-value decode, so it cannot simply call
`FromValue::from_value` in its place without a real reimplementation of the token protocol. The
ticket's own briefing already named this exact type as "Deferral 2" — but framed it as blocking only
`Edit<PresentMutation>` (the animate plugin, under `SEMANTIC-MUTATIONS-OVERHAUL`). **This run proves
the same generic decoder is ALSO instantiated directly over `Checkpoint` and `Alternative`**
(`🏪️store/🦀️.rs`'s fresh-VCS decoder, field ids 4 and 5) — a materially wider blocker than the
briefing's own framing, found only because the compiler was asked, not grepped.

**Reverted exactly the four affected types** — `Checkpoint`, `Alternative` directly (the compiler's
own two named types), plus `Author` and `CompositionPin` transitively (`Checkpoint.authors:
Vec<Author>` and `Checkpoint.composition_pins: Vec<CompositionPin>` are fields of a struct that must
keep `#[derive(Serialize, Deserialize)]`, so they need it too) — with docstrings naming the real
reason, matching this ticket's own established "revert cleanly, document why" precedent. Kept the
conversion for everything the compiler did NOT name: `ArtifactHistoryLedger<T>`,
`ArtifactHistoryIter<'_, T>`, `ArtifactVcs<P, Mutation>`, `ArtifactVcsRead<'_, P, Mutation>` — all
four now genuinely serde-free in production.

## `ArtifactCursor`/`ArtifactCursorOwners` (`🏪️store`) — the second-order unblock, test-gated not deleted

`store::ArtifactCursor`'s own docstring named the exact same `BackboneDocument` reason for staying
dual. With `BackboneDocument` converted, re-checked for any OTHER consumer requiring
`ArtifactCursor`/`ArtifactCursorOwners: Serialize` (repo-wide grep, zero hits) — but found the type's
own test module uses `serde_json::from_value::<ArtifactCursor>`/`<ArtifactCursorOwners>` as an
oracle-comparison fixture (`🏪️store/🦀️.rs:~22750+`, inside `mod tests`). Per this ticket's own
established pattern (`#[cfg_attr(test, derive(Serialize, Deserialize))]`, already used ~10+ other
places), gated both types' serde to test-only rather than deleting it outright — `ArtifactCursor`'s
hand `impl serde::Serialize`/`impl Deserialize` blocks gained `#[cfg(test)]`; `ArtifactCursorOwners`'
derive line and its three field-level `#[serde(...)]` attributes moved to `#[cfg_attr(test, …)]`
mirrors. **First pass over this missed the third field** (`checkpoint_id: Option<String>` still had
an unconditional `#[serde(default, skip_serializing_if = "Option::is_none")]`, no matching
unconditional derive) — caught immediately by the next `cargo check` (`error: cannot find attribute
'serde' in this scope`), fixed, re-verified green. Recording the miss here rather than smoothing it
over: this is exactly the "silently missed one field" failure mode the ticket's own briefing warned
about, just in an attribute rather than a `use` statement this time.

## `📡️spr/🧵️channel` and `💡️inference` — re-confirmed, correctly left alone

- **`📡️spr/🧵️channel`**: consumer (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs:188`,
  `serde_json::to_vec(&result.command_ingress)`) reconfirmed genuinely native-only —
  `semio-framework-plugin-host` is depended on repo-wide only under
  `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` (grepped every consumer: `🏃️run`,
  `🌉️mcp`, `📺️renderer/…/🧊️wgpu`, `💻️os/🖥️host` itself — zero `s-plugin` manifests). Considered
  target-gating `FixedCommandPage`/`CommandPageCursor`/`CommandIngressStatus`'s derive the same way
  `ArtifactCursor` was test-gated — declined, because unlike `ArtifactCursor` these three have NO
  test-only fallback need (their only consumer is the native `plugin-host` wire, not a test oracle),
  and gating them would move neither this classifier's count nor the guest-link `cargo tree -i`
  count (see the structural finding below: `serde` stays a real, unconditional os-kernel dependency
  regardless, via `Change`). Zero-value edit; not made.
- **`💡️inference`**: re-read all three still-serde-only `InferredField` implementors directly —
  `RemodelPoseDelta` (`✏️s/🔌️plugins/📸️remodel`), `AssemblySolveResult`
  (`✏️s/🔌️plugins/🌀️procedural`), `FlattenPlane` (`✏️s/🔌️plugins/🧩️puzzle`) — all three still
  serde-only, unchanged. Ratio still 7/13. Trait bound correctly left unflipped.

## Structural finding, unchanged from the draft of this doc earlier in the session — `os-kernel`'s `Cargo.toml` still cannot drop serde, permanently

Traced `Change`'s `serde_json::to_vec(change)` (`🌿️vcs::content_addressed_checkpoint_id_core`) one
level further than any prior wave: its callers (`content_addressed_checkpoint_id`/
`…_with_pending_change`) are invoked directly from `🏪️store/🦀️.rs`'s own checkpoint-creation methods
(lines 9733, 13667, 14062, 15380) — `ArtifactStore`'s core "commit a checkpoint" path, unconditional,
guest-reachable, no target/feature gate anywhere in the chain. Even with `store`/`vcs`/`spr`/
`inference` all hypothetically at zero, `serde`/`serde_json` could never be dropped from
`os-kernel`'s `Cargo.toml` without either a proven-byte-identical-to-serde_json first-party JSON
float formatter (documented as not existing yet, same gap `🧵️canonical-edit::ScalarBytes` needs) or
an explicitly-authorized checkpoint-id hash-format break (forbidden by this ticket's own rules).
`serde` also stays a real, present dependency for every guest build regardless of any individual
type's own derive — confirmed unchanged by the closing `cargo tree -i serde` run below.

## Verification — every command run to completion this wave, verbatim tails

```
$ cargo check -p semio-framework-os-kernel --message-format=short
warning: `semio-framework-os-kernel` (lib) generated 32 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 32 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 6.92s
```
0 errors. GUARDRAIL — green. (Two earlier iterations of this same command, run after each
incremental edit, caught the two real regressions above — `ArtifactCursorOwners`'s missed third
field and, before that, `Author`/`CompositionPin`/`Checkpoint`/`Alternative`'s real blocker — both
fixed before this final green run.)

```
$ cargo check -p semio-framework-os-kernel --tests --message-format=short
```
0 errors from any file this wave touched. Exactly the same two out-of-fence, pre-existing failures
`📓️os-kernel-serde-endgame.md` already documented and this wave did not cause: `semio-framework-os-infinite`
(`#[value(flatten)]` gap, 1 error) and `semio-framework-plugin-host` (`PresenceUpdate: FromValue`
gap, 3 errors). Recorded, not chased.

```
$ cargo check -p semio-framework-os --features os-host-full --message-format=short
```
101 (nonzero) — but the SAME 3 errors as the pre-edit baseline, all in `semio-framework-plugin-host`'s
`PresenceUpdate: FromValue` (byte-identical to the `--tests` run above), **zero errors in
`💻️os/🖥️host/🦀️.rs` itself** — confirmed by running this exact command both before and after the
`BackboneDocument` edit and diffing the error sets (identical). `BackboneDocument`'s conversion is
therefore proven not to have broken the host crate, even though the host crate as a whole cannot
reach a fully clean check today (pre-existing, out-of-fence, unrelated to this wave).

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 11.34s
```
0 errors.

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (…)
│   └── semio-s-plugin-draw-fsm v0.1.0 (…)
└── semio-framework-replication v0.1.0 (…)
    ├── semio-framework-os-kernel v0.1.0 (…) (*)
    └── semio-framework-pack v0.1.0 (…)
        └── semio-framework-os-kernel v0.1.0 (…) (*)
```
Unchanged from every prior wave's own measurement — full, un-truncated output, one tree, no second
inverted instance hidden by truncation this time. Expected and correct: `serde` remains a real,
unconditional `os-kernel` dependency (`Change`'s content-hash requirement, see above), so this
number was never going to move this wave regardless of `store`/`vcs`'s own line-count reduction.

## `BackboneDocument`'s disposition — CONVERTED (the ticket's own headline question)

Fully off serde. Hand-written `ToValue`/`FromValue`, camelCase object shape mirroring the old
`#[serde(rename_all = "camelCase")]`/`skip_serializing_if`. Verified compiling clean in both
`os-kernel`'s own check and the host crate's `os-host-full` feature check (the only two places it's
reachable).

## `InferredField` implementor count — unchanged, 7 of 13

Re-verified directly this wave (not from memory): `RemodelPoseDelta`, `AssemblySolveResult`,
`FlattenPlane` still serde-only. No change attempted or warranted — each is a separate, small,
out-of-fence plugin wave.

## Does `os-kernel`'s `Cargo.toml` drop serde? No — structurally, permanently, independent of the four modules' own counts (see above).

## Files touched this wave

- `💻️os/🖥️host/🦀️.rs` — `BackboneDocument<P, Op>`: dropped `Serialize`/`Deserialize`, added hand
  `ToValue`/`FromValue`; added `use protocol::{DslValue, FromValue, ToValue, ValueError};` inside
  `mod host`.
- `🌿️vcs/🦀️.rs` — `ArtifactHistoryLedger<T>`, `ArtifactHistoryIter<'_, T>`, `ArtifactVcs<P,
  Mutation>`, `ArtifactVcsRead<'_, P, Mutation>`: serde dropped outright (production, no
  `cfg_attr(test,…)` needed — none of these were ever reached by a test-only serde oracle).
  `Author`/`CompositionPin`/`Checkpoint`/`Alternative`: attempted the same drop, compiler-enumerated
  a real, previously-undocumented blocker, reverted with docstrings explaining the true reason.
  `Change`: untouched (permanent). Shared comment block above `Change` rewritten to reflect the new,
  more precise state.
- `🏪️store/🦀️.rs` — `ArtifactCursor`'s hand `Serialize`/`Deserialize` impls and
  `ArtifactCursorOwners`'s derive (container + 3 field attributes) moved to `#[cfg(test)]`/
  `#[cfg_attr(test, …)]`, matching this ticket's own established oracle-preservation pattern.

## What remains (counts, for whoever picks this up next)

1. **`🏪️store`, 31 lines** — `pack_rt`/`InteractionState` (blocked on `📡️replication`, unchanged),
   `🧵️canonical-edit::ScalarBytes` (8 lines, permanent).
2. **`🌿️vcs`, 22 lines (18 genuine + 4 classifier false-positives — bare `Serialize`/`Deserialize`
   `VcsError` enum-variant names, not code that touches the crate)** — `Author`/`CompositionPin`/
   `Checkpoint`/`Alternative` genuinely blocked on `ArtifactRepositoryHistoryEntryAuthority<T>:
   DeserializeOwned` (`🏪️store/🦀️.rs:6625`) — a real, substantial seam: that authority is a
   streaming `serde::Deserializer`-token decoder, not a whole-value one, so unblocking it means
   reimplementing its token protocol over `FromValue`/`DslValue`, not just relaxing a bound. Same
   root cause as the ticket's own already-documented `PresentMutation: ToValue` deferral
   (`Edit<PresentMutation>` uses the identical decoder) — this wave's contribution is proving the
   SAME seam also covers `Checkpoint`/`Alternative` directly, so whoever tackles
   `ArtifactRepositoryHistoryEntryAuthority<T>` should scope for all three, not just the one the
   briefing named. `Change`: permanent (content-hash).
3. **`📡️spr`, 9 lines** — `🧵️channel`'s dual derive, reconfirmed genuinely blocked on
   `🔌️plugin/🖥️host` (native-only, real consumer); target-gating considered and correctly declined
   as zero-value.
4. **`💡️inference`, 7 lines** — unchanged, 3 plugin-side waves away.
5. **Permanent, structural**: `os-kernel`'s `Cargo.toml` cannot drop `serde`/`serde_json` — `Change`'s
   `content_addressed_checkpoint_id_core` is unconditionally guest-reachable from `store`'s own
   checkpoint-creation path. This should be treated as this ticket's own final answer on that
   question, not re-investigated by a future wave.
