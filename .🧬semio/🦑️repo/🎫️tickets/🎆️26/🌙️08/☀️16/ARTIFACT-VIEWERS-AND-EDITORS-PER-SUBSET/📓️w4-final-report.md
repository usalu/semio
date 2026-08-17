# W4-FINAL — SDK dispatch fix + full gate suite + browser E2E attempt

Lane: W4-FINAL, last lane before ticket close. Read `📓️w4-tests-report.md`, `📋️contract-freeze.md`,
`📋️master-plan.md` §Verification first, per the brief.

## Job 1 — SDK dispatch bug fix

**PASS.** Fixed `VcsArtifactApp::dispatch_typed_command_inner`'s kind-discipline guard in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (~line 11634). The guard used to
resolve `verb`'s `ActionKind` via `self.registry.get_command(&verb)` only — `AppActionRegistry`'s
`app_commands`/`mode_commands` maps — which never indexes plain declared actions (`.view_action(...)`,
`.mutation(...)`, `.shell_action(...)`), those live in the registry's separate `actions` map, reached
via `self.registry.get(&verb)`. A `View`-kind action declared with `.view_action(...)` that emitted an
operation was therefore silently not rejected, even though the `ArtifactApp` contract requires it.

Fix: the guard now checks `self.registry.get(&verb)` **before** falling back to
`self.registry.get_command(&verb)` — mirroring the identical two-step lookup this same file already
uses for kind resolution elsewhere (`dispatch_action`, lines ~10779 and ~10814). No test was weakened;
`view_action_emitting_ops_is_rejected` now runs its full body and passes for the real reason (the
op-emitting `View` action is now actually rejected with the fault), rather than passing vacuously.

Verified: `component::plugin_runtime::plugin_builder_contract_tests::view_action_emitting_ops_is_rejected`
— `ok` in isolation and in the full serial run.

**Re-check of the other 6 (attribution)**: re-ran the full suite after the fix; all 6 reproduced
unchanged, confirming the prior lane's attribution was correct — none is this ticket's fallout:
1–3. `component::app::artifact_definition_contract_tests::*` — a different identity system
   (`ArtifactIdentity`/`ArtifactDefinition` resource+localization grammar), unrelated to
   `AppRole`/`surface_app_id` or the dispatch kind-guard.
4. `component::app::testkit::testkit_tests::assert_two_instances_converge_on_disjoint_edits` — a
   VCS/store-layer "invalid edit reference" bug.
5. `component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`
   — flaky cross-test global child-factory registry pollution under parallel execution.
6. `component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads`
   — a history/merge-conflict validation seeding bug.

None of the 6 touch `dispatch_typed_command_inner`, `AppActionRegistry`, or `ActionKind` resolution.
Left failing, as instructed.

## Job 2 — full final gate suite

Full output: `🧪️w4-final-gates.txt` (narrative + all raw command output). Raw per-gate logs also saved
standalone: `🧪️w4-final-plugin-check.txt`, `🧪️w4-final-registrycheck.txt`, `🧪️w4-final-verifygate.txt`,
`🧪️w4-final-livepolicy.txt` (+ its script `🧪️w4-final-live-policy-check.ts`),
`🧪️w4-final-repolibtest.txt`, `🧪️w4-final-workspace-check.txt`, `🧪️w4-final-stdio-check.txt`.

| # | Gate | Result | Detail |
|---|---|---|---|
| 1 | `cargo test -p semio-framework-plugin --lib` | **PASS** | 213/7 -> **214 passed, 6 failed** (all 6 pre-existing, see Job 1) |
| 2 | `cargo check -p semio-framework-plugin --all-targets --keep-going` | **PASS** | 0 errors |
| 3 | `bun ./📜️script.ts check` (via `nx run @semio-tech/plugin-registry:check`) | **PASS** | registry catalog + `.vscode/launch.json` both fresh; only non-fatal informational findings |
| 4 | `bun ./📜️script.ts verify gate` | **BLOCKED** (pre-existing) | dies at dependency-cruiser step: 827 violations (650 err/177 warn), exit 138 — identical to W3-A's own documented run of this same gate; zero violations reference this ticket's files |
| 5 | `bun nx run @semio-tech/repo-lib:test` | **PASS** | 170 pass, 18 fail — exact same 18 names as the documented baseline, zero new failures |
| 6 | Live-filesystem policy run (4 required functions, called directly against the live tree) | **PASS** | `policySubsetSurfaceCompletenessBreaches` = 0, `policyViewerPurityBreaches` = 0, `policyContributedSurfaceTargetBreaches` = 0, `policyOsConfigShapeBreaches` = 0 |
| 7 | Structural facts (`find`) | **PASS** | `🎛️apps` dirs = 0; `👁️viewer` dirs = 143; `✏️editor` dirs = 143; `SCAFFOLD`-marker files under `✏️s/🔌️plugins` = 0 (repo-wide: 3, all benign — the scaffolder's own source defining the marker string, plus one unrelated 2026-03-10 cache file) |
| 8 | `cargo check --workspace --all-targets --keep-going` | **FAIL, as expected** | 753 error lines total. Top failing crates: `semio-s-plugin-stdio` (164 lib + 556 test — live peer `FULL-STDIO-…`, 1520 uncommitted files, `absorb`/`apply` signature migration to `Result<_, MutationApplyError>` not yet propagated), `semio-compose-rs` (10, downstream of the same stdio bug), `semio-framework-ui` (89 test-only, foreign/already-committed, unrelated to any named peer), `semio-framework-os-kernel-db` (67, live peer `MUTATION-OUTCOMES`/os-kernel, 3 uncommitted files, `protocol::ConflictRule`/`MergeStrategyKind` renamed mid-flight), `semio-framework-plugin-host` (3, live peer, same `AppFrame::Error.report` fallout W3-A already flagged), `semio-framework-os-infinite` (13, live peer, same os-kernel scope). Zero errors touch this ticket's files. |

Gate 4 and Gate 8's failures are both pre-existing/foreign and were independently re-verified at
ticket-close time (not just carried over from earlier lane reports) — see `🧪️w4-final-gates.txt` for
full git-status/git-log attribution evidence per crate.

## Job 3 — browser end-to-end

**BLOCKED**, attempted for real (not skipped, not faked). Full evidence in
`🧪️w4-final-job3-blocked.txt`.

Booted `cad-react-dev` (`.claude/launch.json`, port 6020, equivalent to `.vscode/launch.json`'s
`🛠️dev📐️cad⚛️react`) via the preview browser tooling. The React shell's Vite dev server started fine.
The plugin wasm build pipeline correctly targeted `cad, stdio`; `semio-framework-plugin` (wasm32)
compiled cleanly (confirming this lane's own dispatch-guard fix does not block the wasm build), but
`semio-s-plugin-stdio` (wasm32) failed with the same 164 errors as the host-target `cargo check`
(`error[E0053]: method 'absorb' has an incompatible type for trait`, expecting
`Result<_, MutationApplyError>`). Critically, the CAD artifact's own schema
(`✏️s/🔌️plugins/🗄️stdio/.../🗿️artifacts/🧿️semio/.../✳️cad/🧬️schema/🦀️component.rs`) is glued into the
stdio crate and hits the identical error, so the cad example cannot be opened at all — the dev
tooling's own error: `error: plugin build failed: stdio`. The browser flow (open example -> editor ->
"Open with… -> Viewer" -> read-only chrome -> mutation fault -> set default viewer -> reload ->
resolver check) could not proceed past plugin load.

Attribution: `✏️s/🔌️plugins/🗄️stdio` has 1520 files with live uncommitted `git status` changes right
now, matching this ticket's own `📌️important.md` and the brief's named peer ticket
`26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`. Not this ticket's work; zero
errors reference `🔌️plugin/🦀️component.rs` or any file this ticket touched. No `[DEBUG]` logging was
added by this lane — the `[DEBUG]` lines captured are the dev tool's own pre-existing instrumentation.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — production fix:
  `dispatch_typed_command_inner`'s kind-discipline guard now also checks `self.registry.get(&verb)`
  (declared plain actions) before falling back to `self.registry.get_command(&verb)` (app/mode
  commands), mirroring the existing two-step lookup pattern already used elsewhere in this same file.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w4-final-gates.txt` — full narrated gate-suite output (created).
- `.../🧪️w4-final-plugin-check.txt`, `🧪️w4-final-registrycheck.txt`, `🧪️w4-final-verifygate.txt`, `🧪️w4-final-livepolicy.txt`, `🧪️w4-final-live-policy-check.ts`, `🧪️w4-final-repolibtest.txt`, `🧪️w4-final-workspace-check.txt`, `🧪️w4-final-stdio-check.txt` — raw per-gate command output (created).
- `.../🧪️w4-final-job3-blocked.txt` — Job 3 blocked evidence (created).
- `.../📓️w4-final-report.md` — this report (created).

No modifying git commands were run. `ticket_close` was not called (per the brief — last lane, but
closing itself is out of this lane's instructions; leaving `📌️important.md` and ticket status as-is
for whoever finalizes the ticket).
