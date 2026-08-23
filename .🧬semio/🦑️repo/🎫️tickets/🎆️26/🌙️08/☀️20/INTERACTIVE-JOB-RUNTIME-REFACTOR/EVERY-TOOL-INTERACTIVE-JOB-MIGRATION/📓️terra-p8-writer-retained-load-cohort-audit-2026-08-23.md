# Terra P8 Writer Retained-Load Cohort Audit — 2026-08-23

## Verdict

**ACCEPT — Writer retained-load source cohort only.** This accepts the current source ownership route, not runtime behavior, global application close, the full operation migration, or Phase 8.

## Independent findings

1. The live Writer Wasm bridge accepts only a `Uint8Array` page. It validates `source.length()` against the fixed 4,096-byte page before copying into the fixed array, validates requested page/byte credits before opening the operation, and has no document `&str` input, whole-buffer constructor, `ArtifactStore::new`, synchronous drain loop, or `Drop` close path.
2. `VcsArtifactApp` preflights its three fixed operation registries and `OwnedSchemaDecodeCredits` before admitting ingress. Page admission validates generation, handle, close state, and exact page credits; stale/full failures return the page owner. Sealing transfers the exact ingress owner to the decoder and restores it to the same registry slot if decoder admission is saturated.
3. The live progression is one-owner and one-maintenance-turn: app ingress → retained schema decoder/completed record → `WriterStoreInitializationAuthority` → checked next generation → one `mem::replace` store swap → retained displaced-store cursor → exact acknowledgement. The public Wasm poll advances `maintenance_step(1, 4096)` and the operation once; it does not run either worker to completion inline.
4. The Writer initializer validates identity and edit uniqueness, clones initial snapshot fields one at a time, seeds causal history lane-by-lane, locates applied/redo edits incrementally, applies/hashes one mutation at a time, and performs atomic `ArtifactStore::from_initialized_runtime_with_owners`. The domain snapshot/mutation/store owner catalog is moved into that constructor in the same statement.
5. Cancellation, stale authority/generation, missing candidate, false terminal, closed response, and saturated paths retain the initializer, candidate, completed record, or ingress owner for a cursorized terminal path. `ActiveArtifactStoreReplacement::recover_job` re-installs a nonterminal or false-terminal job through `retain_initializer_for_close`; it does not drop it. A terminal candidate is only removable after its disposer reports terminal empty.
6. `close_step` orders envelope ingress, decoder jobs, returned field decoders, completed records, and store-replacement jobs before later app-owned lanes. Each of these uses a fixed cursor and one-item/page grant. Replacement acknowledgement removes only a matching operation/generation that is terminal empty; a duplicate acknowledgement returns `false` without removing another generation.

## Semantic fixture review

The source contains meaningful assertions for submit/decode/initializer/swap/displaced-store retirement/first-and-duplicate ACK, partial-ingress cancellation without publication, exact page saturation and interrupted page close, initializer cancellation and stale generation, checked next generation, and incremental candidate disposal. The verifier additionally mutates the Wasm page type, removes acknowledgement, and replaces retained false-terminal recovery with `drop(job)`; each mutation is rejected by the self-test. These Rust fixtures were inspected, not compiled or executed.

## Executed gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on store, plugin, Writer editor, Writer Wasm, and Writer mutation-binary sources | PASS |
| Same rustfmt command without `skip_children` | Concurrent non-cohort RED: recursive plugin-module discovery reports formatting drift in reactor/host submodules, not in the five cohort files |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: 146 self-tests clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected global RED: 18 failure classes, 0 admitted / 884 remaining; no Writer retained-route failure |
| Independent direct-placeholder census (`reject_whole_buffer_artifact_envelope_ingress`, excluding shared store) | PASS for the claimed decrement: 16 direct callers remain; Writer Wasm has 0 placeholder/direct-constructor hits |
| `bun ./📜️script.ts verify interactivity --format json` | Global RED: one concurrent DB-testkit `WorkerPool::new` thread-pool finding; no Writer finding. The recorded test-only blocking bridge remains structurally exempt. |
| Whole and scoped working/staged/HEAD `git diff --check` | PASS |

## Residuals outside this cohort

- No Cargo build, Rust test execution, Wasm/browser run, network, or timing/saturation runtime probe was run by audit scope.
- The full verifier remains RED at 0/884 registrations with 18 global failure classes; this cohort does not activate a tool operation.
- Whole-app terminal close remains incomplete (`close_terminal_is_empty()` is still false), and other app/runtime close owners remain outside the Writer-specific cursor route.
- The 16 remaining direct envelope placeholder callers, global ArtifactEnvelope/store structural failures, full typed operation migration, and Phase 8 remain unaccepted.
