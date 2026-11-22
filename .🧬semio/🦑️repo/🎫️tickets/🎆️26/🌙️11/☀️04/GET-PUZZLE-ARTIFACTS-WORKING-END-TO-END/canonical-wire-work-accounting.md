# Canonical Wire Work Accounting

The current Rust law is `original_wire::tests::original_tree_wire_cpu_distinguishes_denial_selected_fault_and_terminal` in `semio-framework-pack-json-tests --test tree-wire`. The retained native run recorded three passing pointer, byte, cancellation and handback laws and one failure at the exact `genuineFault` assertion. `FaultingWire` is a one-child array. The wire passes 64 output bytes to the tree during measurement, so the observed failure takes the flat `Inspect` path and reaches the source getter before the one-unit charge at the end of tree advance. A narrow output window exposes the independent child-demand fault before a turn is admitted.

## Source changes

- In the canonical tree cursor, quote a child alias using `retained_alias_demand(self.owner.borrow()?)`. The parent and child both have type `dyn ArtifactCanonicalJsonTree`. A vacant admission uses the same fixed pointer, binding and output-slot sizes for both. Projection preserves the parent's lease, so `retained_alias_work_demand(parent)` is also exactly the child's alias work quote. This preserves the existing quote owner and avoids any value crate change.
- Quote `Inspect` from the fixed 64-byte scalar staging width. The existing conditional quote invokes the fallible node accessor outside the selected turn. An admitted inspection can then call the actual accessor under work authority and report only the bytes it really initializes. This makes both node and child quotes structural. The inspection quote becomes conservative for containers and text.
- In the owned child branch, preflight `retained_alias_work_demand(parent)?.checked_add(1)` after the physical quote and before any getter. Return zero progress when this work is unavailable. Charge one unit and invoke `parent.try_project(ordinal, ...)`, then bind and publish using `retained_alias`. Advance the ordinal and install the pending child only after ownership is published. A genuine child refusal consumes the getter unit and publishes no alias.
- Select the existing flat path only when its phase and output window permit it and at least one unit remains beyond the fallback requirement. The fallback needs one unit for inspection, text or punctuation, or the getter plus full alias work at the first array ordinal. Charge one unit before the bounded flat attempt and cap its output window at 64 bytes, matching the wire's existing batch width. Keep the depth bound. If the attempt cannot complete a batch, fall through under the reserved work. This preserves the flat selected-fault law's one unit and avoids a repeated speculative probe starving the slow path at lower work grants. The owned alias check still applies whenever the flat attempt yields to owned traversal.
- Prepay one unit before a non-delegated selected tree operation with a callback or state change. Preserve the empty-output no-op before charging phases that require output. Keep paged storage and owned close operations delegated to their existing effect methods. Set tree and wire `last_turn_work` from the effect's before and after delta in the outer entry points, including errors. Remove the wire's synthetic `last_turn_work = 1` for demand errors. This also covers partial delegated work on refused turns. Apply the same measurement to wire `close_step`; wire header operations should deny zero work before moving state.

## Caller and Receipt Audit

| Caller | Impact |
| --- | --- |
| Pack wire | Uses the physical quote on both passes and caps both output windows at 64 bytes. The wire delegates the reported tree work. Read each outer effect delta once, including delegated refusal. |
| Pack original JSON receiving | Adds a 64-byte comparison buffer and offset extent to the tree quote. The selected canonical branch explicitly reports zero on `canonical.advance` failure. Read `canonical.last_turn_work()` after the result for either branch, as the successful branch already does. The producer's borrowed reply is itself settled using effect work. |
| Replication edit seal | Wraps the physical quote in its child demand and sponsors a 256-unit traversal turn. Its settlement rejects observed work greater than reported and settles only the difference, so exact child work will avoid both false mismatches and duplicate charges. Its page is 1024 bytes, and its tree window can reach 256 bytes. A 64-byte flat cap may increase turns, so verify byte equality and published digest under the existing seal law. |
| Actor diagnostics | Adds metadata to the physical quote, subtracts it from the child grant and can pass the remainder of a 4096-byte result page. A 64-byte flat cap reduces the maximum per-turn output. Verify paging and final acknowledgement. There is no fixed requirement for the cursor to fill the page in one step. |
| WGPU presented roster | Constructs an exact retained grant from the quote, so changing container and text inspection from zero to 64 changes that turn's physical admission. Its 64-byte window already matches the candidate flat bound. It receives the measured original work from `StepContext` and consumes at least one fuel unit per selected job attempt. |
| Procedural preview digest | Quotes the writer dynamically, measures actual effect work directly and hashes only the emitted bytes. Its staging window is 64 bytes. |
| Flow child-publication parameter JSON | Calls the wire wrapper and measures actual effect work. Its driving loop stops when work and retained receipts are both unchanged. A rejected alias should therefore report zero and remain retryable in a new turn. |

The remaining tree references are exports and tests. The mesh, graph, manifest, raster, Puzzle 2D and Flow tests are additional byte-equivalence callers. The public `next_demand` callers use the returned extent or apply their own admission before invoking the cursor. A copy grant below 64 at `Inspect` now denies container and text inspection as well as scalar, while scalar's existing denial guarantee is preserved. Keep the established bound separate from alias header bytes, which can be wider and already have their own quote.

`RetainedAllocationEffect::performed` records a delta in the supplied receipt before it checks the local ceiling. Use `remaining_work_units` to deny without calling `performed` when insufficient. Use the delta from the call entry, not the effect's total, because callers can already have spent work in that effect. `RetainedAllocationEffect::reborrow(grant, ceiling)` permits a focused regression to use real work ceilings without changing the companion harness. A precharged bounded flat batch needs separate funded fallback work when it cannot complete. The test can use an original tree cursor directly with a one-byte output to force owned traversal regardless of copy grant, then use the wire's existing 64-byte measurement path for the flat fault.

The byte and depth limits bound the number of flat traversal operations. An implementation of `ArtifactCanonicalJsonTree` controls the cost of each callback. Existing standard map projections can scan entries by ordinal. The paid unit remains the existing selected operation or bounded batch, so the proposed change does not attach a duration guarantee to arbitrary source code.

## Verification

- Keep the existing CPU fixture's exact selected-fault count. Extend the law to exercise the small-window owned alias path and the full-window flat path separately, with the same explicit fault. Count actual getter invocations using a thread-local `Cell` in the Rust fixture. Repeated `next_demand`, a physical shortfall, and an alias work shortfall should leave the count at zero and report no work or retained change. The exact admitted turn calls the getter once, records one work unit on refusal and can close every retained owner. A low-work effect can be borrowed via `EffectBank::driven` or a work-limited effect reborrow; the existing `crate::fx` always allows `1 << 20` work.
- Run all four companion `tree-wire` laws after the targeted fault law. Run the companion `tree` suite and check the scalar-only body-denial, exact ownership, and the nested header/body fixture. The existing wire output fixtures compare canonical bytes against `serde_json`. If the companion nested fixture rejects a projected header larger than 64 bytes, capture its baseline result before attributing the failure to the work-accounting change.
- Register the CPU law in `runNativeScript` under the `tree-wire` branch in `pack/json/🧪️tests/🔬️rust-package-commands/🟦️.ts`. The project target `@semio-tech/framework-pack-json-rs:test-tree-wire-native` currently selects the three green laws and omits the CPU law entirely. Use the existing `⏱️cpu/📐️.json` fixture and schema for the work values. The TypeScript source target currently uses Ajv and SQLite for the successful wire fixture. Add the CPU fixture validation there and keep the measured work and callback count in the native harness.

## Focused Native Fallback

From the checkout root, use the ticket's preexisting `cargo-home` registry, `cargo-wire` target and `cargo-wire-build` unit directory. The repo target should be used when the fleet is available. The offline unit graph for the fallback resolved against `🧰️framework/Cargo.toml`, including 102 units with only `tree-wire` as the root. No compilation was started. The previous test executables are absent from the ticket's recorded output directory, and Cargo may need to rebuild some dependencies from the cached registry.

```bash
wire_ticket="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated"
CARGO_HOME="$wire_ticket/cargo-home" CARGO_TARGET_DIR="$wire_ticket/cargo-wire" \
CARGO_BUILD_BUILD_DIR="$wire_ticket/cargo-wire-build" \
ZIG_GLOBAL_CACHE_DIR="$wire_ticket/zig-clipboard/global" \
ZIG_LOCAL_CACHE_DIR="$wire_ticket/zig-clipboard/local" RUSTC="$(rustup which rustc)" \
RUSTC_WRAPPER= cargo test --manifest-path '🧰️framework/Cargo.toml' \
  --offline --locked --jobs 1 -p semio-framework-pack-json-tests --test tree-wire -- \
  original_wire::tests::original_tree_wire_cpu_distinguishes_denial_selected_fault_and_terminal --exact
```

Use the same command without the arguments after Cargo's `--` for the complete companion wire target. Add `-Z unstable-options --unit-graph` before `--` to reproduce the no-build preflight.

The binary from the retained native run was unavailable for direct execution in its recorded artifact directory, so the baseline comes from `🗑️generated/clipboard-native-wire-handback.log`. The focused rebuild uses ticket-local Zig caches because the default C compiler wraps Zig and fails to link against its read-only default cache.

## Focused Results

The offline single-job target passed all four `tree-wire` laws. The baseline passed the three pointer, byte, cancellation and handback laws and failed the CPU law on its one-unit selected fault expectation. The updated CPU law executes both traversal paths. It uses a one-unit inspection turn to enter the owned child path, then measures denial at the alias-only ceiling and a selected refusal at the full child ceiling. The language-neutral values keep their original expectations. Both Bun source tests passed, including the new Ajv 2020 validation of that fixture.

Changes are confined to the canonical tree and wire cursors, the canonical receiving wrapper's failure receipt, the focused Rust and TypeScript wire tests and the exact native law registration. The native log is `🗑️generated/canonical-wire-work.log`. No wider tree, replication, actor or WGPU suite was run for this accounting change.
