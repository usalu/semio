# Sol Independent Audit: Phase 8 Trinity Rewrite Retained Load Cohort — 2026-08-23

## Verdict

**REJECT — source-only cohort.** The Rewrite caller is cut over to the accepted Jack retained-load authority and the structural census decreases exactly once, but the public page-admission adapter ordinary-drops the exact rejected fixed page. The permanent check named as the rejected-page handback discriminator requires that discard and its mutation is not a faithful ownership regression. No production source was edited by this audit.

## Independent evidence

| Requirement | Result | Evidence |
| --- | --- | --- |
| Raw placeholder and old store route | PASS | Trinity Rewrite has zero `reject_whole_buffer_artifact_envelope_ingress`, zero `envelope_json: Option<String>`, zero `RefCell<TrinityGraphStore>`, and no former dispatch/projection/envelope methods. The live owner is `VcsArtifactApp<EditorApp<TrinityJackPlayApp>>`; no second domain decoder/store implementation was added. |
| Fixed retained lifecycle | PASS | `beginEnvelopeLoad` validates 64 pages/256 KiB, reconstructs operation+generation, and delegates to the fixed ingress registry. Page size is an inline 4 KiB array. Seal, one `maintenance_step(1, 4096)` opportunity, poll, generation-specific ACK/cancel, and `close_step(1, 4096)` all delegate to the accepted Jack/framework authority. No production `loop`, `while let`, `run_to_completion`, or growable page input occurs in the Rewrite bridge. |
| Exact rejected-page ownership | **FAIL** | At Rewrite source line 763, `admitEnvelopePage` converts `Err((fault, _page))` into `JsValue`. `_page` is the exact `ArtifactEnvelopeDecodePage` returned untouched by the shared app, but the public adapter neither returns it nor retains it for retry/close; it is ordinary-dropped at the end of the error closure. The borrowed `Uint8Array` remains with JavaScript, but that does not make the newly constructed admitted page owner observable or terminally retained as claimed. |
| Permanent mutations | **FAIL** | The predicate at `📜️script.ts:1644` requires the literal `_page` discard. The named handback mutation at line 3266 replaces it with `.map_err(js_fault)`, which changes error conversion shape and does not model a well-formed page-owner loss. It therefore proves only literal syntax, not exact handback. The other baseline plus twelve mutations distinguish whole-buffer resurrection, dynamic input, generation erasure, fixed-array loss, seal/poll/ACK/cancel/close regressions and missing shared fixtures. |
| Fixed caps, progress, cancellation and terminal close | PASS by shared-source composition | The 64-page/256-KiB cap fixture covers exact caps, zero and both `+1` boundaries. Accepted Jack/framework fixtures cover first/duplicate ACK, stale/cancelled initialization, fixed-registry saturation owner return, one real page per interrupted-close grant, candidate retirement and terminal emptiness. These source fixtures were inspected; Rust tests were not built or run. |
| Structural census | PASS | The production Rust scan is exactly **13** occurrences: one shared fail-closed definition plus twelve callers. Rewrite is zero. The surviving callers are Dag, Flow, FEM 2d/3d, Procedural 2d/3d, CAD, Puzzle 5d/3d, Shooting, Process 3d and Raster. No second caller cohort was removed. |
| Deterministic ledgers | PASS | `p8yu-trinity-rewrite-ledger-a/b.json` are byte-identical. Both SHA-256 values are `f12335f43c5f7e2fc790aa11282cf2f2525062ce76cbe71e8571a1aac6ecb5ce`. |

## Gates

- Scoped `rustfmt --edition 2021 --check --config skip_children=true` on the Rewrite component: exit 0.
- `bun 📜️script.ts verify interactivity tool-jobs --self-test --format json`: exit 0, `self-tests=299 clean`. The ownership false positive above means this count is not acceptance evidence for rejected-page handback.
- Full tool-job ledger: expected global RED, 0 bounded of 884 remaining commands and 18 failure classes; no additional Rewrite-named class because the predicate accepts the `_page` discard.
- Broad interactivity DENY: exit 1 only for the concurrent Phase 1 DB replay-history seven-to-six census finding; no Rewrite finding.
- Scoped working, staged and combined-HEAD diff checks: exit 0. Whole working-tree check: exit 0. Whole staged and combined-HEAD checks retain only two unrelated findings: the prior Phase 3 raster audit blank EOF and `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` trailing whitespace.
- Cargo, Nx, native, Wasm, browser, runtime and network execution were not run.

## Exact repair packet

1. Remove `map_err(|(fault, _page)| ...)` from the Rewrite public adapter. Either reject before constructing/copying a page while the exact raw owner remains wholly outside the authority, or expose/retain the returned `ArtifactEnvelopeDecodePage` in a generation-tagged bounded rejection result with retry/take/one-page close.
2. Add a semantic fixture that fills or stales the exact ingress, verifies the rejected page bytes/identity remain retrievable, retries or closes it, and reaches terminal empty without ordinary drop. Include generation ABA and page-cap `+1`.
3. Replace the current line-3266 mutation with a well-formed mutation that changes the real return/retention path to a discard while preserving all other syntax. Require that mutation to fail for the intended exact-owner predicate.
4. Rerun scoped rustfmt/parser, all 299 self-tests, exact 13-symbol census, deterministic ledgers, broad DENY, and scoped/whole working/staged/HEAD diff checks.

## Residual status

This rejection is limited to the Rewrite page-rejection edge and its verifier. The shared Jack retained-load architecture remains accepted and was not reopened. Phase 8 remains RED at 0/884 with 18 global classes; compile/runtime behavior and the twelve remaining raw callers remain outside this audit.
