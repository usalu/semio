# Fix: `semio-framework-os-mcp` 🏠️workspace/🦀️component.rs — three-family compile blocker

File owned and edited: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs`.
No other file was edited.

## Family A — `ProbeSnapshot`/`ProbeMutation`/`ProbeDiff` need `ToValue`/`FromValue`

Hand-written for all three (not derived), matching the precedent in
`📓️fix-os-kernel-store-mutations.md` (`SpaceHistoryDiff`/`SpaceHistoryMutation`): each wraps a
foreign `serde_json::Value`, one of the documented derive gaps, so each gets:

```rust
impl store::ToValue for ProbeSnapshot { // (and ProbeDiff, ProbeMutation)
    fn to_value(&self) -> store::DslValue { store::to_dsl_value(self).expect("... infallibly") }
}
impl store::FromValue for ProbeSnapshot {
    fn from_value(value: store::DslValue) -> Result<Self, store::ValueError> {
        store::from_dsl_value(value).map_err(store::ValueError::new)
    }
}
```

**Extra, not in the original brief**: fixing the `ToValue`/`FromValue` bound surfaced a second,
previously-masked error on the same `impl store::Mutation<ProbeSnapshot> for ProbeMutation` block:
`E0046 missing DESCRIPTORS, descriptor`. `store::Mutation<P>` requires
`const DESCRIPTORS: &'static [MutationLeafDescriptor]` and `fn descriptor(&self)`.
`SpaceHistoryMutation`'s precedent gets these for free from `#[derive(dsl::Mutations)]`, which
`ProbeMutation` does not use (that derive requires each variant to wrap a real `MutationLeaf`
payload; `SetValue`'s foreign `serde_json::Value` field is not one). Followed the OTHER
repo-wide precedent for a hand-written `Mutation` impl with one real operation
(`InteractionConfigMutation` in `🔌️plugin/🦀️component.rs:9806`, `const DESCRIPTORS = &[...]`,
`descriptor(&self) { &Self::DESCRIPTORS[0] }`) and hand-built one
`store::MutationLeafDescriptor` (`PROBE_SET_VALUE_DESCRIPTOR`) whose 14 fields satisfy
`store::validate_mutation_leaf_descriptor`'s contract (verified by hand against
`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`'s validation functions — `owner` contains
`/🧬️mutations/`, `semantic_kind`/`aggregate_variant` match the variant, `text_opcode`/`binary_tag`
are `None` since `ProbeMutation`'s `OpText`/`OpBinary` impls carry no opcode prefix).

## Family B — `store::Fault` doesn't implement `Display` (12 sites)

All 12 `Self::not_wired("<what>", fault)` call sites (lines were originally ~614-671, now
~681-738 after Family A's added impls) reformatted to surface `Fault`'s real fields, matching the
established idiom at `🏃️run/🦀️component.rs:389` (`format!("{}: {}", fault.code.0, fault.message)`,
same `store::Fault` type, same product):

```rust
Self::not_wired("encoding AppCommand", format!("{}: {}", fault.code.0, fault.message))
```

No `Display` impl was added for the foreign `store::Fault` type.

## Family C — `Event::InstanceOpen` has no field named `instance` (2 sites)

Real definition (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:894`) carries `request:
ActorInstanceOpenRequest { activation_generation, instance_id, request_sequence }`, not a bare
`instance: PluginInstanceId(..)`. Matched against the one call site elsewhere in the repo already
on the new shape — `🔌️plugin/⚛️reactor/🦀️component.rs:2320` (construction) and
`🔌️plugin/🖥️host/🦀️component.rs:2684` (destructuring match arm, confirms field names). Both sites
in this file now build:

```rust
request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: <u32 identity>, request_sequence: 1 },
```

`instance_id` is the numeric identity already in scope at each site (`instance: u32` param in
`ensure_instance`; `u32::from(plugin_ordinal)` in `activate_plugin_instance`, which has no other
numeric instance id). `activation_generation`/`request_sequence` are `1` — first open, and
`store`'s own `valid_request`/`ActorInstanceOpenRequest::is_valid` require both nonzero.

Per the brief: the two `E0599`s on `applied_edit_ids`/`snapshot_pack` were consequences of Family
A and vanished once the bounds were satisfied — not touched directly.

## Follow-on breakage in the same file, same root cause class

Fixing Family A made `ArtifactStore<ProbeSnapshot, ProbeMutation>` satisfy `store::SpaceMember`'s
bounds for the first time, which changed what several existing call sites resolved to:

- `probe_store.applied_edit_ids()` and `probe_store.snapshot()` are **plain sync methods**
  (`🏪️store/🦀️component.rs:13828`, `:13994`) — 5 call sites previously written as `.await`
  (`E0277 ... is not a future`) had the `.await` removed; one `resolve_ready(probe_store.
  applied_edit_ids())` had the `resolve_ready` wrapper removed (that helper requires a real
  `Future`).
- `probe_store.undo()`/`.redo()` are real `store::SpaceMember` trait methods (object-safe async
  trait, `🏪️store/🦀️component.rs:17212-17213`, blanket-impl'd for `ArtifactStore<P, Mutation>` at
  `:17340` once `P`/`Mutation` satisfy `ToValue + FromValue + ...`) — not inherent, so the trait
  needed importing: added `use store::SpaceMember as _;` near this file's other imports.
- `apply_probe_mutation` computed `applied_edit_ids` (now a borrow of `probe_store`) and then
  moved `probe_store` into `self.open_probes...insert(..)` before the borrow's last use — a real
  borrow-checker error once `applied_edit_ids` stopped being an owned `Vec` from an `.await`.
  Reordered to compute `head_edit_id`/`cursor` as owned values before the move (mirrors
  `ensure_probe_artifact`'s existing ordering just above it).

## Audit invariants (`📜️script.ts` `toolJobPagedIngressExact`, ~4685-4740)

Re-checked after all edits — all still hold:

```
grep -c "pending_exchanges\|PendingResponsePage\|RejectedCommandBuildRegistry<1>\|CommandBatchDriver\|close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)\|terminal_is_empty\|persistent_command_completion_port_ready" 🏠️workspace/🦀️component.rs
```
All markers present (unchanged — no edit touched that region); `response: Option<Result<store::AppFrame, Fault>>` absent (grep returns nothing), confirmed by inspection — none of the edits added that shape.

## Verification — real command output

### Full clean build, watched in the foreground, BEFORE the isolated target dir was swept

```
export CARGO_TARGET_DIR=".../AI-MCP-END-TO-END/🗑️generated/target"
export RUSTC_WRAPPER=""
cargo build -p semio-framework-os-mcp --bin semio-os-mcp --message-format short
```

Tail of the real run (own crate compiled, zero errors):

```
   Compiling semio-framework-os-mcp v0.1.0 (.../🌉️mcp/📦️packages/🦀️rust)
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🚚️transport/🦀️component.rs:258:30: warning: unnecessary qualification
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🗿️artifact/🦀️component.rs:29:91: warning: unused import: `Tool`
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../💡️inference/🦀️component.rs:29:118: warning: unused import: `Tool`
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🖥️ui/🦀️component.rs:28:27: warning: unnecessary qualification
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🦀️component.rs:26:9: warning: ambiguous glob re-exports: the name `JobStatus` in the type namespace is first re-exported here
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🚚️transport/🦀️component.rs:176:8: warning: method `since` is never used
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🧵️bridge/🦀️component.rs:976:19: warning: method `kind` is never used
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🧵️bridge/🦀️component.rs:1836:8: warning: method `take_terminal_job` is never used
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🧵️bridge/🦀️component.rs:2423:19: warning: method `take_terminal_broadcast_job` is never used
🧰️.../🌉️mcp/📦️packages/🦀️rust/../../🏠️workspace/🦀️component.rs:587:8: warning: method `command_closes_terminal_is_empty` is never used
warning: `semio-framework-os-mcp` (lib) generated 10 warnings (run `cargo fix --lib -p semio-framework-os-mcp` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 27.23s
```

Immediately re-confirmed with a `grep -E ": error"` on the same command — **zero matches**, real
shell exit path observed, not assumed. This was a genuine, complete, from-clean build of
`semio-framework-os-mcp` including its `[[bin]] semio-os-mcp` target, all warnings pre-existing
or in files this ticket did not touch (10 warnings, 4 auto-fixable, none new from these edits
except the expected "never used" ones from methods this file already had unused).

### Cache wiped mid-session (not by this session), rebuild blocked by UNRELATED peer churn

Between that build and the next verification step, the isolated
`.../AI-MCP-END-TO-END/🗑️generated/target` directory (and its parent `🗑️generated/`) vanished —
not deleted by this session (no `rm`/destructive command was run). A from-scratch rebuild was
attempted 3x in the foreground; each attempt gets through the full dependency graph and then fails
in `semio-framework-os-kernel`, in a file this ticket does not own:

```
🧰️.../🔨️modules/🧩️extension/🦀️component.rs:16:9: error[E0433]: cannot find module or crate `zip` in this scope
... (10 more, same file, same crate)
error: could not compile `semio-framework-os-kernel` (lib) due to 12 previous errors; 31 warnings emitted
```

Root cause (confirmed, not guessed): `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`
declares `zip` only under `[target.'cfg(all(target_arch = "wasm32", not(target_env =
"p2")))'.dependencies]` (wasm32-only), while `🧩️extension/🦀️component.rs` uses
`zip::result::ZipError` unconditionally — breaks on this native target regardless of Family
A/B/C. `git status` shows this exact `Cargo.toml` as `MM` (modified, uncommitted) — a live,
in-progress peer edit, unrelated to `🌉️mcp`, first observed transiently as a different
crate-resolution error (`semio_framework_io_base64` not found) two retries earlier that resolved
itself between attempts. The `zip` error has been stable across 2 consecutive retries (not
shrinking) as of this report.

**Not fixed, on purpose**: this is a shared `Cargo.toml`/`🧩️extension` file this ticket does not
own, mid-edit by someone else — out of scope per the ticket's own instruction ("nothing outside
this file should need to change").

### `cargo test -p semio-framework-os-mcp --lib` — NOT run

Blocked transitively by the same unresolved peer `zip` breakage in `semio-framework-os-kernel`
before `semio-framework-os-mcp` (or its tests) can even build. Not attempted since it cannot
produce a different result than the build above while that blocker stands. **No claim of a
passing or failing test suite is made — none was observed.**

## Summary

- Family A: fixed, plus the previously-masked `DESCRIPTORS`/`descriptor` requirement it exposed.
- Family B: fixed, all 12 sites.
- Family C: fixed, both sites, matched against the real repo-wide precedent.
- Follow-on `.await`/borrow-order/trait-import fixes in the same owned file: fixed.
- `semio-framework-os-mcp` (including its own `📇️registry`/`🗿️artifact`/`💡️inference`/`🖥️ui`/
  `💬️prompts` facets) was seen to build clean, zero errors, real terminal output, once, before an
  external cache wipe. A fresh rebuild cannot currently re-verify this because an unrelated,
  unowned, uncommitted peer edit (`zip` crate target-cfg in the shared `os` product `Cargo.toml`)
  blocks `semio-framework-os-kernel` first. `cargo test` has not been run or observed to pass.
