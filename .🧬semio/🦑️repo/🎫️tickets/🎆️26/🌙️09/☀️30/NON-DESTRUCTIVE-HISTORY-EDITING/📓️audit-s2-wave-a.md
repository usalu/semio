# 📓️ W3-R audit, wave A: S2-W2B, S2-TAX, S2-CODES, S2-INFRA (session 2)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Reviewer: W3-R (read-only; no cargo/nx/test/build/server was run; no file edited except this one).
Binding references: `📋️design.md` §7, §10, §11, §16–§18; `📌️important/📝️.md`; `/Users/ueli/Documents/semio/AGENTS.md`.

## 0. Method and caveats

- **HEAD is not the 11:16 snapshot any more.** `git log` shows HEAD = `25bb77059d6` (14:31 auto-commit); the 11:16 snapshot is `4e36b2b5012`. All session-2 diffs below are `git diff 4e36b2b5012 -- <path>` (read-only), then read in the working tree. Peer edits are mixed into several files (Interpreter, Stepper, taxonomy, `📜️script.ts`, discovery); only work attributable to the audited WP is judged.
- Independent static checks I ran (Python over `git ls-files`, no repo code executed):
  - **i18n:** the 10 new `ui.timeTravel.refusal.*` keys plus `review.blocked`, in both React bundles, equal `TIME_TRAVEL_LABELS` (`⏪️time-travel/🟦️.ts`) byte for byte. Every en/de text of `TIME_TRAVEL_LABELS` also occurs verbatim in the Rust `⏪️time-travel/🦀️.rs`. `HISTORY_REFUSAL_LABEL_KEYS` order equals the band corpus `refusals` order (20 = 20).
  - **Outcome vocabulary:** Rust `OUTCOME_CODES` equals TS `OUTCOME_CODES` equals fixture `🧫️outcome-code/🔣️.json` (9 codes, same levels). The nine code labels plus the apply family are byte-equal between the React bundle (`ui.mutation.code.*`, `normal`) and Rust `history_code_text`.
  - **Outcome documents:** all 3,115 committed `🎯️outcome` documents use only vocabulary codes, each at its fixed level; 0 breaches.
  - **Retired spelling:** no `"warn"` outcome level remains in any fixture, decoder, TS or Python twin. The only hits are `⏱️trace` record levels, a different vocabulary.
  - **Literals:** no non-vocabulary `mutation.*` literal remains outside the intentional `rejected` list of the fixture (plus `mutation.json` and the GraphQL resolver ids).
  - **Sealed evidence:** the 51 sealed documents (frozen coordinate contracts, markdown contracts, package projection catalogs, owned-file catalogs) all match their registered sha256.
  - **Taxonomy:** `members-of-fixtures` lost 19 names. All 21 directories carrying those names are proven by a sibling `🧬️schema/🧬️mutations/<leaf>` whose `🔣️.json` descriptor names itself as owner. The new `sourceModulePaths` entries are in UTF-8 byte order and unique.
  - **Owned commands:** `workspaceCommand` rows: 109, 0 invalid, 0 duplicate.
- **Not verifiable under the hold:** any compile or test result claimed by the reports. I did not re-run the gate, tests, cargo or taxonomy validation. Every "passes" below is the executor's claim.

---

## 1. S2-W2B (React shell)

### Findings

**B-1 · major · number-control keyboard law (design §18) is only partly wired on staged vector axes**
- `🛠️ShellHelpers/🟦️.tsx:4672` `if (event.key !== "PageUp" && event.key !== "PageDown") return;`
- `🛠️ShellHelpers/🟦️.tsx:4653` `values[index] = Math.min(control.max ?? Number.POSITIVE_INFINITY, Math.max(control.min ?? …, next));`
- `🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx:95` `…).filter((row) => row.key === "pageUp" || row.key === "pageDown");`
- §18 binds slider, dial, stepper and **vector axes** to one law: Home/End = hard min/max, arrows = one step then snap, typed values rounded half away from zero.
- The shared corpus `🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls` carries `home` (3), `end` (3), `increment` (14), `decrement` (4) rows plus `typed` and `limits` sections, including `number-field-home-without-bound` and `number-field-end-to-bound`.
- `StagedVectorField` implements 2 of the 6 key kinds. Home/End are native caret keys. A typed value crossing a bound is silently clamped, whereas `Stepper` and `Interpreter.InputView` refuse it visibly (`uiNumberCrossedBound`, `aria-invalid`) and keep the exact stored value on retyping (`uiNumberTypedValue`). The test asserts only the 14 page-key rows.
- The same gap sits in files S2-W2B did not write (owner S2-W1E/W2-D): `🗣️Interpreter/🟦️.tsx:1460` handles only PageUp/PageDown, and `🪜️Stepper/🟦️.tsx:289` has no Home/End.
- **Fix:**
  - Route `Home`, `End`, `ArrowUp`/`ArrowDown` (±Shift = large) of each staged axis through `uiNumberKeyValue`.
  - Commit typed text through `uiNumberTypedValue`, then `uiNumberCrossedBound` (refuse and show the bound's refusal, as `Stepper` does).
  - Extend the staged-arg test to every `numberControls.keys` row (not just page keys) and to the `typed` and `limits` sections for a one-axis vector.
  - Cross-file: the same rows for `InputView` and `Stepper`.

**B-2 · major (coverage) · the History-panel reveal and the focus effect have no component-level test**
- `🏛️ShellHost/🟦️.tsx:11868–11898` (`revealHistoryPanelRef`, the session effect `timeTravelFocusCancelRef`, mobile branch, `SET_PANEL_VISIBLE`/`SET_MOBILE_PANEL_VISIBLE`).
- No test references `revealHistoryPanelRef`. Only the pure `timeTravelTransitionV1` (corpus rows), the DOM finders and `scheduleTimeTravelFocusV1` are tested; the report §S2.4 itself lists the live probe as pending.
- The three claims "reveals on the edge into a session", "mobile opens the merged panel on History" and "an agent-begun session never steals typing" are therefore only proven for the helpers, not for the wiring (dock lookup, anchor, mobile branch).
- **Fix:**
  - Extract the effect body into a hook in ShellHelpers (`useTimeTravelReveal({dock, mobile, session, dispatch, root})`).
  - Test it with the real dock reducer: desktop anchor opens and selects History; `mobile` opens the merged panel; no dispatch when History is absent; the focus cancel survives progress patches and is cancelled by close.
  - Add a third-party oracle for the transition table (an xstate model checked against the corpus, as the lifecycle-law does), since the table is currently verified only by its own corpus.

**B-3 · minor · the `Stepper` still emits a dangling `aria-labelledby` (the §S2.2 accessibility fix is half done)**
- `🪜️Stepper/🟦️.tsx:319` `aria-labelledby={labelledBy ?? (ariaLabel === undefined ? labelElementId : undefined)}`
- `🪜️Stepper/🟦️.tsx:356` `if (showLabel && id) { … <Label … labelElementId={labelElementId}>`
- The label element exists only with `showLabel`. An interpreted stepper with no `accessibility.label`, or any `Stepper` without `showLabel`/`labelledBy`/`aria-label`, still points at a missing id and has no name.
- **Fix:** `aria-labelledby={labelledBy ?? (showLabel && ariaLabel === undefined ? labelElementId : undefined)}`, and add a Stepper law "no `labelledby` naming a missing id; a nameless stepper is reported".

**B-4 · minor · phone-fit of `LocalFolderReconnectBand` is untested**
- `🏛️ShellHost/📎️local-folders/🟦️.tsx` (`max-w-[90vw]`, `min-h-medium px-tiny`); the local-folders suite never asserts them (`grep 90vw` in its tests: 0). The "fits a phone" law covers only the time-travel band, and only by class names.
- **Fix:** extend the law to the reconnect band. Prefer a layout-level assertion (computed width ≤ 90 % of a 375 px container in jsdom is impossible, so keep the class law but state it).

**B-5 · minor · docstring hygiene**
- Non-unique leading emoji (AGENTS: "unique and fitting"): the new docstrings `⏪️time-travel/🟦️.tsx:170,208,239` all open with `🎯️` (a fourth at `:77`); `:174,:178` both open with `🧭️` (a third at `:49`); `🛠️ShellHelpers/🟦️.tsx:3257,3264` both open with `📌️` (8 in the file).
- Misplaced docstring: `🧪️tests/🧩️component/🟦️.tsx:438` `/** ✏️ The Rust-shaped body while a draft is open … */` sits on `const GENERATION = 7;` instead of `draftBody`.
- Orphan stacked docstring (pre-existing): `🛠️ShellHelpers/🟦️.tsx:2264` `/** 🗂️ The panel-tab ids whose bodies an actor may render …` directly above the `HISTORY_REFUSAL_LABEL_KEYS` docstring, which S2-W2B edited.
- **Fix:** give each its own emoji, move the test docstring to `draftBody`, delete the orphan.

**B-6 · minor · one user-reachable runtime fault has no localized notice**
- `🔌️plugin/⏪️time-travel/🦀️.rs:1516` `FaultCode::new("timeTravel.member-gone")` ("the composed member a history edit targets is gone"). The code is in neither `TIME_TRAVEL_CODE_LABELS` nor `HISTORY_REFUSAL_LABEL_KEYS`, so both shells show the generic fault text.
- A closed composed child (flow) during a session is a normal user path; the other three non-vocabulary codes (`member-owners`, `snapshot-close`, `unknown-action`) are internal invariants.
- **Fix:** add `timeTravel.member-gone` to the vocabulary (en/de, band corpus row, React bundle, wgpu), or turn it into a `refusal` of a defined code.

**B-7 · minor (a11y) · the finalize prompt can open without taking focus**
- `⏪️time-travel/🟦️.tsx:239–250`: `scheduleTimeTravelFocusV1` returns early on `timeTravelFocusIsHeldV1(document.activeElement)` for every target, including `"dialog"`. If an agent-driven finalize arrives while an editable field outside the History panel holds focus, the modal prompt opens with focus left behind it.
- **Fix:** apply the held rule only to `editor` and `band`; a modal prompt always takes focus (the person's focus returns on close). Add the row to the transitions or focus laws.

**B-8 · minor · transition corpus rows are partial wire values**
- `🧫️time-travel-band/🔣️.json` `transitions[*].from/to` omit `blocking` and `acceptedCount`. They validate only because the kernel schema requires just `sessionId, generation, stage` (`🎠️kernel/🧬️schema/🔣️history-patch/🔣️.json`), while design §10 states `blocking: boolean` unconditionally.
- **Fix:** either require `blocking` in the schema and the rows (the Rust side always serializes it) or amend §10.

### Verified OK (S2-W2B)
- All 10 plugin-level refusal texts and the changed blocked copy are byte-equal en/de across React, the vocabulary and the Rust crate (see §0). The band corpus's 20 refusals, the new faulted-replay case and the `transitions` array are consumed by React and also by the wgpu law `the_shared_band_transitions_hold_on_wgpu`.
- The deleted `time-travel-focus` corpus (created and removed by S2-W2C) leaves no dangling reference.
- Sync-attach identity: `syncAttachDocumentIdV1` and its law match the ShellHost call site. The checkpoint gate, the close key and `useCheckpointOnCloseV1` are used by ShellHost, and `focusedHistoryV1` reads the live store, so a close fired from a stale closure still sees the closing program's session. No `[DEBUG]` leftovers, no comments inside definitions, no legacy shims.
- The shared corpus `🧫️local-folder-bindings` is validated against the new schema `🧬️schema/🔣️local-folder-bindings` (+ the two leaf payload schemas) with an unknown-mutation negative.

---

## 2. S2-TAX (taxonomy)

### Findings

**T-1 · major (governance) · a sealed golden and the "previous contracts unchanged" pin were re-sealed to fit a change**
- `🔣️taxonomy.json` `frozenCoordinateEvidenceContracts.cad-draw-projection-vectors-v1.sha256`: `9264c9de…` → `8bfd3766…`.
- `🧫️fixtures/📐️cad-draw-path-projection/🔣️.json`: exactly 43 `…/liveBindings/*/live` strings changed (verified by structural diff); every sealed coordinate is untouched.
- `🧫️fixtures/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json`: `originalContracts` `38 / 50623c9c…` → `40 / e78713c8…`.
- The sealed coordinates (`sourceRoot`, `destinationRoot`, `mappings[*].sourcePath|destinationPath`) exclude `liveBindings`, yet the whole-document sha256 includes them. Every rename wave therefore forces either a codemod hit on sealed evidence (4 hits this ticket) or a re-seal.
- The new guard `historical-json-source-encoding` "every live sealed document still matches its registered digest" cannot tell an intentional re-seal from drift, because the executor edits document and registration together.
- The test `one exact encoded historical source is registered without changing the previous JSON contracts` (`🕰️historical-json-source-encoding/🟦️.ts:72–75`) is now a snapshot of the current state, not a "previous contracts unchanged" proof.
- **Fix:**
  - Move `liveBindings` into its own unsealed fixture (e.g. `🧫️fixtures/📐️cad-draw-live-bindings`, schema-first).
  - Restore the golden to the original sealed bytes and `sha256 9264c9de…`.
  - Make the energy pin cover the contracts present at its date plus an explicit allow-list of later ids, instead of re-hashing everything.

**T-2 · major (rule: "MUST get everything working") · S2-TAX leaves red tests in files it edited**
- Report §S2.5: `📍️draw-destination-observation` 0/1; `🏺️historical-package-owner-identity` 25/1 (purity row 29); `☂️frozen-coordinate-wildcard-coverage` 4/1; `❄️frozen-markdown-coordinates` 34/2; `💥️nested-cargo-collision-authority` 25/1; `🔬️workspace-contract` CAD+Draw package-move 6/12.
- S2-TAX edited `🧫️fixtures/📍️draw-destination-observation/🔣️.json` (authority paths) and extended `🧫️fixtures/🖍️draw-source-scenario/🔣️.json` (+280 lines) but left the observation law red. Its pin `catalogSha256 1410a74c…` is the catalog as of the old revision `cd96692e52f` (09-19); the catalog has had other digests since (9264c9de, 1205dd20, 8bfd3766).
- The observation freezes an 11-file Draw destination that no longer exists (live Draw is the 2-node bundle), so the law compares a dead shape to the live tree and cannot go green.
- Most causes are pre-existing or peer (plugin-registry closure, collision authority); the Draw-observation red is a stale observation of a deleted shape.
- **Fix:** delete the dead observation law and fixture, or re-observe the 2-node destination with correct digests. Re-point the remaining reds to named owners in the ticket status; none may stay anonymous "pre-existing".

**T-3 · minor · the Draw command-bundle rule was loosened, and the overlay shadows the production contract id**
- `🔍️discovery/🟦️.ts:4250–4266`: `draw-editor-command-bundle-v1` now accepts any number of declaration-only nested Rust packages.
- `🧪️tests/🔬️workspace-contract/🟦️.ts:4337–4338` overlays `"draw-editor-command-bundle-v1": DRAW_SOURCE_SCENARIO.commandBundle.descendantContract` on the production taxonomy, so the package-move laws run against a taxonomy production never loads.
- Live Draw is 2 nodes; the 16-node form exists only as a synthetic scenario, so the engine keeps nested-package support alive only for a fixture (AGENTS: no legacy support).
- **Fix:** if no live consumer exists, delete nested-package support and the scenario. If kept, register the scenario under its own contract id (`draw-editor-command-bundle-scenario-v1`) so production ids are never overridden in tests.

**T-4 · minor · the vector proof admits any `<leaf>-<anything>` directory**
- `🧹️normalization/🟦️.ts:3625` (`provenMutationLeafOwners`): `if (vector !== folded && !vector.startsWith(`${folded}-`)) continue; … proven.add(directory)`.
- Any directory named `<proven leaf>-<suffix>` under `🧫️fixtures` resolves as `members-of-fixtures` with no structural check, unlike scenario children, which must hold `🦠️mutation|📸️snapshot|🔺️diff`.
- The two new leaf-identity rows are positives only; there is no negative row for an orphan `<leaf>-x` directory.
- **Fix:** require the vector's own bundle shape (or the registered `-applied` family) and add a `none`-form negative vector to `🧫️mutation-leaf-identity`.

### Verified OK (S2-TAX)
- Structural identity: `members-of-fixtures` removal is sound (19 names, 21 directories, all proven; see §0). `members-of-members-of-fixtures` exists. The two new rows reference existing descriptors and bundles. Ajv is the descriptor oracle.
- Sealed evidence: the 3 restored documents match their seals (51/51 digests); the energy pin's count (40) equals 41 contracts minus the historical one.
- Draw scenario fixture validates against its schema (0 Ajv-equivalent errors, checked with Python jsonschema). Draft-07 schema, no external runtime dependency.
- `[DEBUG]`: none. Ticket scripts `🧪️s2-tax-*.ts` are ticket-local (allowed). Launch rows for `test-mutation-leaf-identity` / `test-mutation-wire-witness` now exist in `.vscode/launch.json` (the §6.2 coordinator action is closed).

---

## 3. S2-CODES (outcome-code vocabulary)

### Findings

**C-1 · major (coverage) · the gate's planted-violation proofs exist only as ticket-local scripts**
- `📜️script.ts:20925` `policyMutationMessageCodeBreaches`, with the new `POLICY_OUTCOME_FEATURE_MESSAGE_RE`, `POLICY_OUTCOME_RUST_LEVEL_ALIAS_RE` and the `warn` token matching.
- The only callers outside `📜️script.ts` are ticket files (`🧪️s2-codes-level-alias-negatives.ts`, `🧪️w3-codes-gate-negatives.ts`, `🧪️s2-codes-outcome-law-bundle.ts`, `🧪️w3-codes-outcome-law.ts`, `🧪️w2-r-energy-outcome-law.ts`); no repo `🧪️tests` entry references it.
- AGENTS requires a language-agnostic test per feature; a regex gate whose six alias positions and thirteen code positions are proven only by scratch scripts can regress silently.
- **Fix:** add `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️outcome-law-gate/` with a schema-first fixture (planted files, expected `file:line` positions, canonical controls) and the vocabulary fixture as the oracle. Validate the fixture with Ajv; port the 13 + 6 positions.

**C-2 · minor · `MutationOutcome::refuse` degrades a bad code to `Fatal` at runtime**
- `📡️replication/🎮️mutation/🦀️.rs:1218–1221` `outcome_code_level(code).unwrap_or(crate::diagnostic::Severity::Fatal)`.
- A runtime-coded guard with a code outside the vocabulary becomes a `Fatal` message that persistence then refuses as a whole (`validate_persisted_message`), so a leaf bug surfaces far from its cause.
- **Fix:** `debug_assert!(outcome_code_level(code).is_some())` plus a unit law that every `refuse` call site's literal is in the table (or make the code argument a vocabulary type).

**C-3 · minor · the "one spelling" rule stops at the wire**
- `📡️replication/🎮️mutation/🦀️.rs:1117` `MutationMessage::warn`, `:1268` chainable `.warn(`; the gate maps `"warn"` → `"warning"` (`📜️script.ts` Rust builder/chain scans) and `type PolicyOutcomeLevel` (`:20752`) omits `"warn"` while `:20980` casts `m[1] as PolicyOutcomeLevel`.
- AGENTS: "many inconsistencies you MUST refactor". The builder is `warn`, the wire is `warning`, and the gate carries a cast that contradicts its own type.
- **Fix:** rename the builders to `warning` repo-wide (mechanical; the gate's inventory lists every call site), or at least type the scan token as `PolicyOutcomeLevel | "warn"`.

**C-4 · minor · generators cannot regenerate the committed layout fixtures**
- Report §S2.8: `🧪️w3-t-layout-author-vectors.py` slugs predate the §14 short-slug rename, so a dry run would rewrite 79 files. The committed layout `🎯️outcome` documents and readers were changed by codemod, and the generator template was edited to match.
- **Fix:** re-point the generator's slug map (S2-LAYOUT) and prove `dry run = 0` before the ticket closes.

**C-5 · minor · the checked-adapter result is discarded by the editors**
- `🧬️generation2d/…/✏️editor/🦀️.rs:449` `let _ = apply_generation2d_mutation(&mut overlay, &leaf);` and generation3d `✏️editor/🦀️.rs:764,769`.
- The preview overlay silently drops a refused leaf: exactly the "refusal dropped" behaviour the adapter was made to expose. Not S2-CODES's edit, but its new `Err(Vec<MutationMessage>)` is thrown away at the only production callers.
- **Fix:** surface the first Error/Fatal message in the preview (design §16.4/G3 UX) or name the intent (`overlay_apply_ignoring_refusals`).

### Verified OK (S2-CODES)
- One vocabulary: Rust = TS = fixture = `expected_mutation_message_level` (delegates to `protocol::outcome_code_level`). Labels are byte-equal across React and Rust (§0).
- Level `warning` everywhere (0 residual `"warn"` outcome spellings; 3,115 outcome documents consistent). The 17 layout readers use `protocol::outcome_code_level`.
- Checked adapter: `apply_generation2d_mutation` returns the diff's own messages on Error/Fatal and joins the apply-time `Fatal`. The delta is retired on both failure paths and the unit rejection test plus `checked_apply_propagates_the_vocabulary_outcome_unchanged` cover the failure paths. The 2d and 3d adapters have the same shape.
- No `[DEBUG]`; no comments inside definitions; docstrings carry emojis.

---

## 4. S2-INFRA (peer-break fixes so far)

### Findings

**I-1 · minor (design) · the renamed owned-command rows use verbs no dispatcher serves**
- The router is the only reader of `metadata.semio.workspaceCommand`, and `dispatchOwnedScriptRoute` is called only with `["verify", …]` (root `📜️script.ts:7164`, `:7978`, `:8208`, `:8212`), `["stdio", …]` and `["flow", …]`.
- INFRA's rows `["generate","<scope>-graph"]` ×9, `["check","<scope>-graph-wire"]` ×9, `["check","gis-inference-discovery"]`, `["check","component-deployment-contract"]`, `["check","cargo-workspaces-*"]` ×5 and `["prepare","cargo-workspaces-manifest"]` are valid but unreachable (45 `generate|check|prepare|members` rows exist repo-wide). The rename only silenced the validator, which still throws repo-wide on any bad row (cross-project coupling).
- The 23 rows under `metadata.workspaceCommand` have no reader at all (INFRA flagged the question).
- **Fix:** decide the grammar: either add the missing `generate`/`check`/`prepare` dispatch at the root script, or drop `metadata.semio.workspaceCommand` from rows no dispatcher can reach (the nx targets already run via `bun nx run`). Add a law "every declared route has a dispatcher verb".

**I-2 · minor · the `async run()` repair stops at syntax**
- INFRA made three `WasmScript.run` async (`🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`, jack LSP, `🖥️host/📦️packages/🦀️rust/📜️script.ts`) and proved only that no file fails to parse.
- In the jack LSP file it fixed, `🧠️lsp/📦️packages/🦀️rust/📜️script.ts:26–28` `run(segments: string[]): void { … runRepositoryCargoTests([…]) }` still calls the async `runRepositoryCargoTests` (`📚️library/🟦️.ts:1419`, `export async function … Promise<void>`) without `await`.
- The same floating call occurs in about 21 Rust package scripts (`flow/🧩️extensions/*`, `process/🧩️extensions/*`, `cad/🧩️extensions/*`, `imperative/🧩️extensions/*`, `sourcing/🧩️extensions/*`, `🔨️modules/📜️imperative`, `jack/🐚️shell`) and about 18 Go package scripts (`runRepositoryTestCommand("go", …)`).
- The repo already tolerates a floating `runVitest` (a law pins it), so the runtime effect (unhandled rejection instead of a routed failure; no router progress/cancellation) is unverified, but the pattern is the same peer migration INFRA claimed to repair.
- **Fix:** make those `run` methods `async` and `await`. Add a policy rule or type-level check "no un-awaited call to an async repo runner inside `BundleScript.run`" so a syntax-only parse is not the only guard.

**I-3 · minor · the guest-check `workspace` field is not schema-first and not cross-platform safe**
- `🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:62` `!workspace.split("/").every((part) => part && part !== "." && part !== "..") || workspace.split("/").at(-1) !== "Cargo.toml"`.
- No JSON schema backs `🔁️rebuild/🔣️.json` (the report calls the fix "schema-first"); the validator is hand-coded, and no third-party oracle (Ajv) checks the rows.
- A Windows path (`..\x\Cargo.toml`, `C:x/Cargo.toml`) passes the segment test (AGENTS: native windows).
- **Fix:** add `🔁️rebuild/🧬️schema/🔣️.json` (workspace pattern excludes `\` and `:`), validate rows with Ajv in the existing test, and reuse the schema in `readGuestFrameworkChecks`.

### Verified OK (S2-INFRA)
- Owned-command renames: 109 rows, 0 invalid, 0 duplicate; no non-router reader or doc names the old words; nx target names (and so `launch.json` rows) are unchanged. The committed `🧱️owned-script-routes` "real contributions" law would have failed on the old rows.
- Generated registry: the current `🤖️generated/🧩️plugins/🟦️.ts` follows `emitTypeScript` in `📽️projection/🟦️.ts` (header, `directoryName` field, `COMPONENT_MODULE_DIRECTORIES`, `PROGRAM_TARGETS`, ordering), and its 69 `directoryName` values equal `🔌️plugins.json`. INFRA's hand emit went through the template's own emitter (not a hand edit); the coordinator's regeneration at 02:49 has since superseded it. The browser artifact `🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js` bundles this module verbatim and stays stale until `generate-frame-worker` runs (coordinator).
- Retirement-macro imports: the 3 modules (`workflow`, `collection`, `space`) import `semio_framework_value::{retirement::…, artifact_retire_* }`; the macros are `#[macro_export]` in `🌱️value/♻️retirement` with `$crate::retirement::…`; no `store::` retirement-macro user remains. The workflow file also carries a peer's `MoveNodes`/`SetNodePositions` arms (not INFRA's).
- Guest-check `workspace`: all 4 rows have an existing workspace manifest; `guestFrameworkCheckArgs` emits `--manifest-path`; the test covers the missing, `../` and absolute refusals.
- `DslValue::Bytes`: the 9 sites are correct in their owners' styles (copy with `try_reserve_exact`, digest `b"bytes"`+len+bytes, census `capacity()`, bounded retire, hash tag 6, JSON byte array). A scan for matches mentioning `DslValue::Array|Object|String` without a `Bytes` arm or a catch-all finds 3 candidates, all of which have catch-alls (`_ =>`) or are tests. **Compile status is unverified:** the 6-plugin-crate check was queued and then hit the hold.
- `sourceModulePaths` additions (I11): byte-ordered, unique, count 151.
- No `[DEBUG]` leftovers in INFRA files. (A pre-existing `[DEBUG]` token is used as a functional marker in `📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts:52,56,60`; out of scope, but it breaks the "[DEBUG] is temporary" convention.)

---

## 5. Cross-cutting notes

- **(c) generated files:** no hand-edited generated file was found among the four WPs. The only gitignored generated files touched (registry `🧩️plugins/🟦️.ts`) were produced by the template emitter. `🔣️taxonomy.json` is authored (Edit-tool edits).
- **(e) peer intent:** S2-TAX restoring 3 sealed documents against codemod rewrites and S2-INFRA I7 reverting 1,521 collateral reference rewrites both fight a REPO-PATH-BUDGET rewrite, but only where the target was never renamed. Both are consistent with the evidence rules; no objection.
- **Commit hygiene:** AGENTS "docstrings start with a unique emoji": the repo gate (`verify docstrings emoji-first`) checks only emoji-first, so the uniqueness rule in B-5 is enforced by nothing.

## 6. Per-WP verdict

| WP | Verdict | Blocking items |
|---|---|---|
| **S2-W2B** | **Accept with changes.** The wire, strings (byte-equal en/de), transitions, R2-4/R2-6 extractions and a11y of the band/prompt are sound and well tested at helper level. | B-1 (§18 keys/typed law on staged vector axes) and B-2 (ShellHost reveal/focus wiring untested) before the ticket closes. |
| **S2-TAX** | **Accept with changes.** Structural identity and registry shrink are correct and verified; seals all match. | T-1 (re-seal and re-pin; split `liveBindings` out) and T-2 (stale red observation law and ownerless reds). |
| **S2-CODES** | **Accept with changes.** Vocabulary single source, level spelling and checked adapters are consistent across Rust/TS/fixtures/i18n. | C-1 (commit the gate proofs as a repo test). C-2…C-5 are follow-ups. |
| **S2-INFRA (listed fixes)** | **Accept.** Every listed fix is correct against the code on disk. The compile check of the `Bytes` arms is still unverified. | None blocking. I-1 and I-2 are design and cleanup follow-ups; I-3 completes the schema-first claim. |
