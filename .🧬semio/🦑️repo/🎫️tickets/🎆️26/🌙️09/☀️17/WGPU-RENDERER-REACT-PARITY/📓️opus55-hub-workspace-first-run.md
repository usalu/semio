# Hub workspace first-run parity (wgpu twin of React `HubWorkspace`)

## What changed

1. **Shared, schema-first contract.** Stages, step structure, anchors per target (`react`/`wgpu`), localized en/de text, the seen-flag key (`ui.introduction.seen.os.hub`, value `true`) and the two workspace controls (replay, close: id, icon, variant, en/de label, order) now live in one language-neutral file next to the TS contract:
   - `🧰️framework/🔨️modules/📇️directory/🎓️first-run/🔣️.json` (data) and `🧬️.schema.json` (shape).
   - Vectors: `…/🎓️first-run/🧪️tests/🧫️fixtures/🔣️.json` + `🧬️.schema.json` (state to stage/step index, session/rows/active-space projection, seen-flag reads, auto-open lifecycle, replay step, dismiss write).
   - `…/🎓️first-run/🟦️.ts` no longer hardcodes data: it imports the JSON and derives `HUB_FIRST_RUN_*`, the text table and `hubFirstRunIntroductionV1`. New export `HUB_FIRST_RUN_CONTROLS_V1` (+ `HubFirstRunTextRowV1` type) through `📦️packages/🟦️typescript/📇️directory/🎓️first-run/🟦️.ts`.
   - Fixture barrel (`📇️directory/🧪️tests/🧫️fixtures/🟦️.ts` and the `@semio-tech/framework-os/modules/directory/testing/fixtures` entry) exports the four new documents.
2. **Rust twin** `🧱️elements/🎓️HubFirstRun/🎯️targets/🧊️wgpu/🦀️.rs` (mounted as `hub_first_run` in the hub-workspace crate): reads the same JSON via `include_str!`; stages, `hub_first_run_state_from`, stage/step-index derivation, seen-flag/auto-open/replay decisions over a `HubFirstRunStorage` door, `hub_first_run_introduction()` (an `IntroductionDefinition` carrying en+de), `hub_first_run_controls()`, `hub_first_run_stage_done` (complete-by-doing).
3. **wgpu workspace** (`🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs`): the tree now opens with the replay button then the close button, built from the contract (ids `os.hub.firstRun.replay` / `os.hub.signIn.cancel`, icons graduation-cap / x, variants ghost / outline, labels "How this works" / "Cancel sign-in", de "So funktioniert das" / "Anmeldung abbrechen"). New verb `hubReplayFirstRun`. The old `framework.hub.close` id is gone (fixture `🔗️hub-projection/🔣️.json` and the live gate `🌎️hub/…/🖥️shells/🟦️.ts` updated).
4. **One overlay, no second chrome.** The shell's introduction overlay now plays either definition: `ChromeTourState` gained `source` (`App` | `HubFirstRun`) and `entry_done`; `DocumentState::chrome_tour_definition` / `chrome_tour_seen_key` select the definition and the seen key (`os.hub`, not brand-scoped, as in React) and are used by the active-step lookup, advance, the card's step count, Escape, and veil/Skip. `reconcile_hub_first_run` (called from `chrome_tour_frame_begin`) reads the flag once per opening of the hub surface, auto-opens at the derived step when unseen and the overlay is free, ends the tour when the surface closes (without answering it), and completes a step by doing what it teaches on the false-to-true edge. Replay starts at step 0. Dismissal goes through the existing `dismiss_introduction` and write drain.
5. **AgentDelegations (slice 2, see below): implemented in code, uncompiled.**

## Findings

- React defect, FIXED in slice 2: the task steps could not be completed (Next hidden while interactions exist, no completion source). One shared rule now lives in the first-run contract.
- wgpu control labels follow the normal terminology of the React label table; the pre-existing wgpu sign-in label table (`HubSignInLabel::Cancel` = "Cancel") is left unchanged for the in-flight cancel button.
- `app` tour seen-key on the last step now uses the brand-scoped `introduction_seen_key` (previously the bare app id), matching what the read/write lanes already used.

## Tests

- TS (same vectors and contract, Ajv as third-party oracle for both shapes): new `🧬️SharedContract` describe in `🎓️HubFirstRun/🧪️tests/🧩️component/🟦️.tsx` (schema validation, stage/step-index vectors, projection vectors, seen/auto-start/replay/dismiss vectors, tour built from contract text in both locales, controls order/ids/icons/variants and rendered aria-labels in en+de). Existing laws untouched.
- Rust: `🎓️HubFirstRun/🧪️tests/🔬️wgpu-unit/🦀️.rs` (stage order vs contract and vectors, state/projection/seen/lifecycle vectors, auto-open gating, dismiss marks seen under the shared key, replay at step 0, step completes by doing, definition text in both locales, screen vs task steps, controls); `🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs` (replay then close buttons, ids/icons/variants/labels/actions; verb list); shell law `the_hub_first_run_tour_auto_opens_once_answers_under_os_hub_and_replays_from_step_zero` in `🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs`.

## Commands and results

See the end of this file (filled in after the runs).

### Status (honest): nothing verified by a green run

Nothing compiled or ran to completion. Two attempts of the hub-workspace cargo test were refused at source admission by another stream's crate (`semio-ui-wgpu-events`, active lines 4032/4017 over the 4000 boundary, being split concurrently). A later retry and the TS run were still queued behind host capacity when the coordinator froze new builds. All code is therefore UNCOMPILED; expect possible small compile fixes.

Tests the integration agent should run:

1. `repo cargo --store=shared --configuration=prototype -- test -p semio-renderer-wgpu-hub-workspace` (laws: `hub_first_run` and `hub_connection` wgpu-unit tests). Status: not run.
2. `repo cargo --store=shared --configuration=prototype -- test -p <shell-tests-chrome-6 package>` filter `the_hub_first_run_tour_auto_opens_once` (touches chrome-state, docs, session, overlays, pointer, actions, view-chrome). Also check the source-reading shell laws in the same crate (`the_introduction_read_arms_the_tour`, tour card laws). Status: not run.
3. `repo run @semio-tech/framework-renderer-react:test -- "../../../../🧱️elements/🎓️HubFirstRun/🧪️tests/🧩️component/🟦️.tsx"` (new `🧬️SharedContract` describe plus existing laws). Status: not run (queued).
4. Also typecheck TS (`@semio-tech/framework-renderer-react:typecheck`): the JSON-import typing in `🎓️first-run/🟦️.ts` is untested.
5. Live gate `🌎️hub/…/🖥️shells/🟦️.ts` now looks for `os.hub.signIn.cancel`. Status: not run.

Known compile risks: `ChromeTourState` struct literals (only one existed, updated), `StyleSpec` fields, `LocalizedLabel`/`Terminology::ALL` imports in the new tests, `IntroductionDefinition` being `Sync` for the `OnceLock`.


## Slice 2 (code only, no cargo/vitest run)

### React tour completion, one shared rule
- Contract `🎓️first-run/🔣️.json` gained `completion` (stage to rule: signIn=signedIn, space=spaceOpen, invite=inviteReady, welcome/done=never) plus schema; vectors gained `completionVectors`.
- TS: `hubFirstRunStageDoneV1` (exported through the SDK entry). `HubFirstRun` takes `inviteReady`, advances on the false-to-true edge of the active stage's rule (never when arriving on an already-finished step, so Back does not snap forward) and passes `completedInteractionIndices`. `HubWorkspace` passes `inviteReady={hub.invite !== null}`. Component test: old "stays on step when state changes" narrowed to a non-completing change; new laws for advance-by-doing, Back, and the completion vectors.
- Rust: `hub_first_run_stage_done` now reads the same rule from the JSON; new test over `completionVectors`.

### wgpu AgentDelegations
- Shared data: `📇️directory/🤖️delegations/🔣️.json` (+ `🧬️.schema.json`): route, schemas, bounds, ttl choices, control ids/icons/variants, field ids and the en/de label table (54 keys, normal terminology, extracted from the React table). Vectors: `…/🤖️delegations/🧪️tests/🧫️fixtures/📐️agent-delegation-contract-vectors.json` (+ schema): instants, ordering, create bodies and refusals, receipts, credential file/name/command, error mapping, list parsing, paths.
- Rust twin `🧱️elements/🤖️AgentDelegations/🎯️targets/🧊️wgpu/🦀️.rs` (mounted as `agent_delegations`): contract fns, `HubAgentState` reducers (begin_load/loaded/failed/minted/download_unavailable/install_unavailable/dismiss/ask/cancel/confirm revoke), `build_agent_delegations_ui` with a node budget (the panel document holds at most 128 nodes; surplus rows are summarised as `+N`). Composed LAST in `build_hub_workspace_ui` (React order: sign-in, spaces, agents), after the replay/close row. `HubWorkspaceState.agents` added; 11 verbs `hubAgent*`; helpers in HubConnection made `pub(crate)`.
- Transport: `DirectoryClient::agent_delegation_call` (route-restricted, session capability stays in the client); shell turns `run_hub_agents_reload_turn` / `run_hub_agent_create_turn` / `run_hub_agent_revoke_turn` and verb arms in `📇️directory-and-hub`; reload on OPEN_SPACE and Refresh and after create/revoke; state reset on close and sign-out.
- Host gaps made explicit, not faked: no file-save door, so Download lands in the contract's own "could not save" path (the exact file text is shown inline to copy); no MCP credential install, so the MCP block answers its `unavailable` phase and no copy control is painted.
- Not at parity: select fields are check/dot buttons (`os.hub.agent.audience.<v>`, `os.hub.agent.expiry.<secs>`); no automatic reload when sign-in completes with a space already open; control ids without a React id (`refresh`, `dismiss`, revoke buttons) follow `os.hub.agent.<name>[.<delegationId>]`.
- TS laws (AgentDelegations component test, `🧬️SharedContract`): Ajv over both shapes, TS constants equal the JSON, every JSON label present in the React label table (en and de), control id rendered, all vectors through the TS contract functions. Fixture barrels export the four new documents.

### Run list (all NOT RUN)
1. `repo cargo --store=shared --configuration=prototype -- test -p semio-renderer-wgpu-hub-workspace` (hub_first_run, agent_delegations, hub_connection laws).
2. `repo cargo --store=shared --configuration=prototype -- check -p <os directory client package>` (new `agent_delegation_call`), then shell crates (`shell-directory`, `shell-actions`, `shell-docs`, `shell-session`, `shell-view-overlays`, `shell-pointer`, `shell-view-chrome`, `shell-chrome-state`) and `test -p <shell-tests-chrome-6 package>` filter `the_hub_first_run_tour`.
3. `repo run @semio-tech/framework-renderer-react:test -- "../../../../🧱️elements/🎓️HubFirstRun/🧪️tests/🧩️component/🟦️.tsx"` and the same for `🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx` and `🔗️HubConnection` tests; plus `:typecheck` and `:hub-sign-in-spaces-check`.
4. Watch for: JSON-import typing in the two TS contracts; existing wgpu tests that enumerate the workspace tree (a new last section `os.hub.agent` and the replay/close ids) and the 128-node panel limit in shell hub-projection tests; `semio-ui-wgpu-events` source boundary of another stream may block cargo.
