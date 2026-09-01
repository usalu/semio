# `ArtifactApp::Snapshot` Bound — Second Serde Root Cause

Companion to `📓️serde-replacement-surface.md` and `📓️serde-fanout-playbook.md` (the pilot's
`Mutation`/`MutationDiff` fix). This session moved the OTHER framework-wide serde-forcing bound —
`ArtifactApp::Snapshot`/`Config`/`Draft`/`Presence`/`Transient` and their many local restatements
in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — from
`Serialize + DeserializeOwned` to `ToValue + FromValue`.

**Headline: the bound itself is now fully serde-free everywhere in `component.rs` (53/53 sites
migrated, 0 remaining).** The crate as a whole does NOT yet compile — blocked one level down, in
`🏪️store` (os-kernel), which is out of this session's assigned surface and under active concurrent
edit by the pilot. See "Blast radius" below.

## (a) Occurrence inventory — `component.rs`, before this session

Full-file grep for `Serialize + DeserializeOwned` / `Serialize + serde::de::DeserializeOwned`
(the two spellings actually present — one via `use serde::{Deserialize, Serialize}` bare names,
one via `use serde::de::DeserializeOwned` bare + explicit `serde::de::DeserializeOwned` mixed in
some blocks) found **53 lines**, all of them restatements of the same defect the ticket named:

| site | count | example |
|---|---|---|
| the 3 `ArtifactApp`-shaped trait definitions (`Snapshot`/`Config`/`Draft`/`Presence`/`Transient` associated types) | 14 | `type Snapshot: Clone + PartialEq + Serialize + DeserializeOwned + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static;` at (then) lines 11069/25777/26120 and siblings |
| `document_codec_bare`/`document_codec_bare_async`/`native_codecs` generic-fn bounds on `Snapshot, Mutation` | 8 | lines ~3240–3358 |
| `bounded_config_store_owners`/`bounded_document_store_owners`/related generic-fn bounds on `C, M` / `P, Mutation` | 20 | lines ~13283–13474 |
| same family, a second cluster | 8 | lines ~18132–18332 |
| `SnapshotBuilder`/`ArtifactBuilder` impl bounds | 3 | lines ~27521–27703 |

Zero occurrences were anything OTHER than this one defect — no unrelated `Serialize +
DeserializeOwned` pair existed in the file for a different purpose. All 53 were mechanically
replaced with `protocol::ToValue + protocol::FromValue` (the file's own established
fully-qualified spelling — already used at `encode_contributed_wire`/`decode_contributed_wire`
near line 4527 for `CompositeMutationKind`, so this session followed the existing local
convention rather than introducing bare-name `use` imports that could collide with
`semio_framework_value_derive`'s derive-macro names in the same namespace). Verified after:
`grep -c "Serialize + DeserializeOwned" component.rs` → **0**.

Two now-dead `use serde::de::DeserializeOwned;` imports (the ones that existed ONLY to feed these
53 sites) were removed (former lines 267 and 27299).

## (b) `#[derive(Serialize, Deserialize)]` sites needing `ToValue`/`FromValue` — inventory and disposition

Every concrete type in the crate's MAIN (non-`#[cfg(test)]`) build path that instantiates
`Snapshot`/`Config`/`Draft`/`Presence`/`Transient`/`Mutation`-family associated types needed the
new bound satisfied. Found by tracing `ArtifactApp`'s default associated types (`NoConfig` /
`NoDraft = NoConfig` / `NoPresence` / `NoTransient`, used by nearly every real `EditorApp`/
`ViewerApp` impl in the fleet) and the one non-test concrete `Mutation` impl in this crate,
`InteractionConfigMutation` (`local_interaction`, wired into `VcsArtifactApp`'s hover mechanism).

**Derived (`#[derive(..., ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]` added alongside the existing serde derive, framework code kept serde per the playbook's "framework is exempt from the ban" rule):**

- `NoConfig`, `NoConfigMutation` (empty enum), `NoPresence`, `NoPresenceMutation` (empty enum),
  `NoTransient`, `NoTransientMutation` (empty enum) — `component.rs` ~9490–9700.
- `$snapshot` in the test-only `fixture_channel!` macro (`declarations::fixture`, `#[cfg(test)]`)
  — `$diff` already had it from a prior wave; `$snapshot` did not, now does.

**Hand-written (the macro doesn't support the shape — see "Not supported" list in the fan-out
playbook):**

- `InteractionConfigMutation` (`component.rs`) — externally-tagged single-variant enum
  (`{"setState": …}`, serde's own default with no `tag` attribute present); the derive only
  supports internally-tagged or (new, this session) plain-unit enums.
- `SetInteractionState` (`🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs`) — `#[serde(transparent)]`,
  explicitly unsupported; 1-line passthrough to `self.state`.
- `SelectionMode`, `DomainSelection`, `DomainHover`, `InteractionState`
  (`📡️replication/📡️wire/🦀️.rs`) — the transitive chain `SetInteractionState` composes, previously
  flagged in that file's own comment as "NOT converted this wave… tracked in the fan-out
  playbook's 'not yet converted' list." These are hand-written, not derived, for a structural
  reason independent of shape: `semio-framework-replication` (crate name `protocol`) physically
  OWNS `DslValue`/`ToValue`/`FromValue`, and the derive macro's generated code is hardcoded to
  `::semio_framework_os_kernel::…` paths — `protocol` cannot depend on `os-kernel` (the dependency
  arrow runs the other way), so the derive macro is structurally unusable from inside this crate.
  Same pattern as the pilot's own `CausalAddDiff`/`CausalAddOp` hand-written impls in
  `🔗️causal/🦀️.rs` — copied that exact style (fully-qualified `crate::value::ToValue::to_value(…)`
  calls, never bare `.to_value()`, per playbook trap #1).

Total: 6 derived (+1 test-only) + 6 hand-written = **13 type conversions**.

## (c) Genuine JSON-boundary code — inventory and deferral rationale

Found via `grep -n "Serialize\|Deserialize"` residual scan after (a)/(b): 4 distinct generic
helper groups still bound on bare `Serialize`/`DeserializeOwned`, all in `component.rs`, all
deliberately left untouched this session:

| function | location | why deferred |
|---|---|---|
| `owned_abi::take_json<T: DeserializeOwned>` / `return_json<T: Serialize>` | ~line 165/171 | Only ever instantiated with `PollInput`/`StartJobInput`/`StepJobInput`/`CancelJobInput`/`RestoreInput`/`JobStep` (wasm-ABI wire types local to `owned_abi`), which embed `semio_framework::kernel::{Event, CommandPageCursor, FixedCommandPage, Budget}` — ~30 `#[derive(Serialize, Deserialize)]` types in a **wholly separate framework module**, `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`, not this file, not os-kernel. Converting these five structs is easy; converting what they transitively require is a different module's job. |
| `try_serialize<T: serde::Serialize>` | ~line 12990 | Called with ad-hoc tuples (`&(&str, &str, Option<&str>, …)`) and a `receipt`/`ui_scope` type — tuple `ToValue`/`FromValue` only exists for 2-tuples in `🌱️value/🔁️codec` today (a concurrent addition found mid-session), not the 3–4-tuples this call site needs. |
| `encode_wire_serialized<T: Serialize>` / `decode_wire_serialized<T: DeserializeOwned>` / `decode_wire_serialized_or` | ~line 28149–28176 | ~15+ distinct concrete `T`s across the file (`Fault`, `protocol::DispatchReport`, `ManifestActionInvocation`, `ManifestCommandInvocation`, `ContextMenuWireRequest`, `protocol::MutationOrigin`, …), most of which are serde-only today. Already routes through the OLD `to_dsl_value`/`from_dsl_value` serde-bridge (not raw `serde_json`), so it is at least on the sanctioned interim path (playbook trap #6), just not yet on the terminal one. |
| `ui_refresh_section<T: Serialize>` | ~line 30033 | Same shape as `encode_wire_serialized` — payload types vary per call site, not migrated this session. |

None of these are reachable from the `Snapshot`/`Mutation` bound chain this session's job covers
— they're a parallel, independently-scoped serde surface within the same file. Left exactly as
found (still serde, framework code is exempt from the ban by CLAUDE.md's own carve-out), not
converted, not silently ignored — flagged here as real, scoped follow-up work.

## Derive macro changes — `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs`

**1. New mode: plain unit enums.** `SelectionMode`-shaped enums (`#[serde(rename_all=…)]`, no
`tag`, every variant a bare unit) were previously **impossible** to derive — the macro
hard-errored unless `#[value(tag = "…")]` was present, and adding a fake `tag` would have changed
the wire shape away from serde's own default (`"single"`, not `{"tag":"single"}`). Added a new
`Data::Enum` match arm, gated on `container.tag.is_none() && data.variants.iter().all(|v|
matches!(v.fields, Fields::Unit))`, in both `expand_to_value` and `expand_from_value`, producing
a bare `DslValue::String(wire_variant)` / parsing one back. Vacuously covers 0-variant enums too
(`.all()` on an empty iterator is `true`).

**2. Real bug found and fixed by the standalone harness: `match self {}` on an empty enum does
not type-check on stable Rust.** `NoConfigMutation`/`NoPresenceMutation`/`NoTransientMutation`
(all real, pervasively-instantiated types — every plugin's default unused-lane mutation type) are
`enum Foo {}` — zero variants. The naive codegen `match self { #(#arms),* }` (0 arms, `self: &Self`)
is rejected by rustc as non-exhaustive (`E0004: non-exhaustive patterns: type &Foo is non-empty` —
references to uninhabited types are NOT treated as uninhabited by exhaustiveness checking on
stable). The codebase's own existing hand-written idiom for this
(`NoConfigMutation`'s pre-existing `impl protocol::Mutation` in `component.rs`) already used
`match *self {}` (dereferenced) for exactly this reason — the derive macro didn't. Fixed both enum
branches of `expand_to_value` (the new plain-unit branch and the pre-existing tagged branch,
same latent defect, same fix) to `match *self { #(#arms),* }`. **Without this fix, my own core
task's key deliverable (`NoConfigMutation` et al. deriving `ToValue`) would not compile** — caught
only because of the standalone differential harness, not by inspection.

**3.** Doc comment at the top of the file updated to describe the new plain-unit-enum mode instead
of overstating internally-tagged as "the ONLY enum representation."

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`:
`semio-framework-value-derive` promoted from `[dev-dependencies]` to `[dependencies]` — the
6 `No*`/`No*Mutation` derives above are compiled unconditionally (not test-gated), so the derive
crate can no longer be a dev-only dependency. Path re-verified with `ls -d` (memory note: a wrong
`../` count broke `cargo metadata` repo-wide once already this ticket).

## Files touched this session

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — 53 bound sites, 6 derives, 1
  hand-written `InteractionConfigMutation` impl pair, 2 dead-import removals, 1 test-macro derive
  addition.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` — dependency
  promotion.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs` —
  hand-written `SetInteractionState` impl pair, removed now-unused derive-macro import.
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` — hand-written impl pairs for `SelectionMode`,
  `DomainSelection`, `DomainHover`, `InteractionState` (additive; existing `serde` derives
  untouched).
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs` — plain-unit-enum derive mode,
  `match *self` fix (2 sites), doc update.

## Verification

### `cargo check -p semio-framework-value-derive --message-format=short` — PASSED

```
    Checking syn v2.0.117
    Checking semio-framework-value-derive v0.1.0 (…/🌱️value/✨️derive/📦️packages/🦀️rust)
…/🦀️component.rs:152:49: warning: unnecessary qualification
…/🦀️component.rs:202:9: warning: unnecessary qualification
warning: `semio-framework-value-derive` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 1m 16s
```

The 2 warnings are pre-existing (documented as "cosmetic, not fixed" in the fan-out playbook's
own verification section) and unrelated to this session's edits.

### `cargo check -p semio-framework-plugin --message-format=short` — 864 errors, ALL traced to one upstream chokepoint (not this session's edits)

Ran to completion. Zero errors reference the 53 migrated bound sites, the 6 derived types, or the
hand-written `InteractionConfigMutation`/`SetInteractionState` chain directly — every single
error is `E0277`/`E0599` at a call site into `store::ArtifactStore<P, Mutation>`'s (or
`MemberStoreOwners`'s) own methods (`advance_apply_one`, `begin_apply_one`,
`content_revision_now`, `snapshot_read_leases_terminal_is_empty`,
`maintenance_retirements_step`, …), because THEIR owning `impl` blocks in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` still require `P: Serialize +
DeserializeOwned` directly (73 lines / ~36 restatements, same defect one level down — the ticket's
own "follow the derived impls outward" instruction correctly predicted this). The single largest
is `impl<P, Mutation> ArtifactStore<P, Mutation> where P: Clone + Serialize + DeserializeOwned +
ArtifactPack + Send + 'static, Mutation: … { … }` at line ~13538 — one `impl` block housing most
of `ArtifactStore`'s methods, so it alone accounts for the large majority of the 864.

**Deliberately not fixed.** `🏪️store/🦀️component.rs` is physically part of `semio-framework-os-kernel`
— explicitly the pilot's territory per this ticket's coordination note ("it owns os-kernel…do not
'fix' os-kernel yourself"). Confirmed under ACTIVE, VERY RECENT concurrent edit (file mtime 23
minutes old at the time this was checked): `SpaceHistoryDiff`/`SpaceHistoryMutation` in that same
file already carry hand-written `ToValue`/`FromValue` impls, and `use
semio_framework_value_derive::{FromValue, ToValue};` is already imported at its top — someone is
mid-conversion there right now. Bulk-editing the same 73 lines concurrently risked a real conflict
with in-flight work, so this was left alone and is reported here instead of attempted.
**This is the single highest-leverage remaining task on the whole ticket** — fixing that one
`impl<P, Mutation> ArtifactStore<P, Mutation>` block (and its ~35 sibling restatements) would
almost certainly clear the large majority of `semio-framework-plugin`'s 864 errors in one shot,
by the same mechanical `Serialize + DeserializeOwned` → `ToValue + FromValue` substitution used
throughout this session.

### `cargo check --workspace --keep-going --message-format=short 2>&1 | grep -cE ' error(\[|:)'` — workspace blast radius

**997 errors** (full run completed). 864 of these are `semio-framework-plugin` itself (see above);
the remaining ~133 are downstream crates that depend on `semio-framework-plugin` and therefore
cannot build without it — none of them are new independent breakage, they are the same root
cause (`🏪️store`'s `ArtifactStore`/`MemberStoreOwners` bound) propagating one hop further. Notably,
`semio-framework-os-kernel` and `semio-framework-replication` themselves compiled with **zero**
errors in this run — confirming the `🏪️store` defect is a latent `impl`-block bound that only
surfaces as an error once a DOWNSTREAM crate's generic `P`/`Mutation` no longer unconditionally
carries `Serialize`/`DeserializeOwned` (exactly what this session's 53-site migration did), not a
pre-existing break in os-kernel's own internal usage.

The shared machine was intermittently saturated during this session — one `cargo check -p
semio-framework-value-derive` invocation stalled at 0% CPU for ~50 minutes on a build-directory
lock before a fresh invocation succeeded in 76s once contention eased, consistent with this
ticket's documented environment risk. The workspace-wide run above completed in one pass once
retried.

### Round-trip oracle — standalone crate, outside the repo (dodges the shared `target/` lock)

Built at `/private/tmp/…/scratchpad/verify-value-derive/` (3 crates: `runtime` — a byte-for-byte
copy of `DslValue`/`ToValue`/`FromValue`/`ValueError` + every hand-written base-case impl,
including `Box<T>` and `BTreeMap<String, T>` which a first draft omitted and caught immediately by
compile failure; `derive` — a byte-for-byte copy of the real, edited
`🌱️value/✨️derive/🦀️component.rs`; `tests` — `serde`/`serde_json`-derived twin types mirroring
the exact real edits: a plain unit enum, an empty struct, an empty enum, an
`Option`+`default`+`skip_serializing_if` struct, a composite `BTreeMap<String, T>`-of-composite
struct, and an externally-tagged single-variant enum). `cargo test --manifest-path
tests/Cargo.toml`:

```
running 7 tests
test empty_enum_from_value_always_errors_never_panics ... ok
test empty_struct_matches_serde_empty_object ... ok
test externally_tagged_single_variant_matches_serde_default_shape ... ok
test option_default_skip_matches_serde_present_and_absent ... ok
test composite_btreemap_of_composite_matches_serde ... ok
test plain_unit_enum_matches_serde_bare_string ... ok
test randomized_state_round_trips_against_serde_lcg_seeded ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

This harness caught two real bugs before they reached the shared workspace: (1) the missing
`Box`/`BTreeMap` base-case impls in the copy (a test-setup bug, not a real-code bug, but it forced
completeness-checking the base cases this session relies on), and (2) the `match self {}` vs
`match *self {}` exhaustiveness defect described above (a REAL bug in the derive macro, fixed in
both the standalone copy and the actual repo file before this doc was written).

### `cargo check -p semio-framework-replication --lib --message-format=short` — PASSED

```
    Checking semio-framework-replication v0.1.0 (…/📡️replication/📦️packages/🦀️rust)
…/📡️wire/🦀️.rs:149:51: warning: unnecessary qualification
…/🔗️causal/🦀️.rs:142:8: warning: method `push` is never used
warning: `semio-framework-replication` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 2m 23s
```

Confirms the hand-written `SelectionMode`/`DomainSelection`/`DomainHover`/`InteractionState`
`ToValue`/`FromValue` impls in `📡️replication/📡️wire/🦀️.rs` are correct by an actual compile of
the real crate (not just the standalone harness's structurally-identical twins) — zero errors,
2 pre-existing unrelated warnings.

### `cargo test -p semio-framework-replication --lib` — 226 passed, 1 pre-existing failure (unrelated)

```
test result: FAILED. 226 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s
```

The 1 failure is `causal::tests::causal_add_fixture_has_exact_required_descriptor`, a
`payloadSchema` string-content mismatch (`"🛂️schema.json"` vs `"../🛂️schema/🔣️.json"`) — byte-for-
byte the SAME pre-existing, documented, unrelated taxonomy-fixture failure the fan-out playbook
already flagged after the pilot's own run (225 passed then; 226 now, one more test collected,
same single unrelated failure). Confirms this session's edits introduced zero regressions in the
crate whose types (`SelectionMode`/`DomainSelection`/`DomainHover`/`InteractionState`) were
directly hand-edited.

### Derive macro — extended further by concurrent work after this session's own edit, still compiles clean

While this doc was being written, a concurrent agent extended
`🌱️value/✨️derive/🦀️component.rs` further: adjacently-tagged (`tag` + `content`) enum support,
and a full externally-tagged (serde's true default, no `tag` attribute, at least one data-carrying
variant) enum mode, built as new match arms alongside this session's plain-unit-enum branch (which
is untouched and still the first, most-specific arm). **Not authored or verified in depth by this
session** — flagged here for the record, not claimed as this session's own work. Re-ran `cargo
check -p semio-framework-value-derive --message-format=short` after that concurrent change landed:
still clean, same 2 pre-existing cosmetic warnings, confirming the two efforts composed correctly.

## What remains — explicit list

1. **`🏪️store/🦀️component.rs`'s `ArtifactStore`/`MemberStoreOwners`-family `impl` bounds** (73
   lines) — the actual blocker for `semio-framework-plugin` to compile. Pilot's territory,
   confirmed mid-flight; not attempted here. Highest-leverage remaining task.
2. **Category (c)** — `take_json`/`return_json`, `try_serialize`, `encode_wire_serialized`/
   `decode_wire_serialized`/`decode_wire_serialized_or`, `ui_refresh_section` in `component.rs`.
   Each needs either a framework module outside this session's scope (`🎠️kernel`) converted first,
   or a widened tuple `ToValue`/`FromValue` base case in `🌱️value/🔁️codec`.
3. **`cargo check -p semio-framework-replication --lib`** unrun — see above.
4. **`declaration_fixture_mutations`/`declarations::fixture` test-only chain** — `$mutation`
   (`Std1AnyMutation` et al.) in the `fixture_channel!` macro is defined in a separate
   `#[cfg(test)]` fixture file (`🧪️tests/📄️declaration-channels/🦀️.rs`) not traced this session;
   likely needs the same treatment, lower priority since it only gates `cargo test`, not
   `cargo check`.
5. **`🧪️tests/📄️declaration-channels/🧪️tests/🦀️.rs`'s `assert_codecs`** helper — still bounds on
   `Serialize + DeserializeOwned` (the one file-wide grep hit outside `component.rs` under this
   crate), but it's a test ORACLE that deliberately exercises serde codec behavior directly, not a
   restatement of the `Snapshot` defect — left as-is, out of scope by design, not an oversight.
