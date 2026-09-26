# WP-P8 — Plugin Handler Correctness (Session 12)

Slice P8 · session 12 · 2026-09-26. A verb whose handler lies about its declared effect, ignores its arguments or does
nothing breaks users, collaborators and agents alike. Seeds: D1 §6. Scope excludes T12's in-flight items (trinity
clearSelection, note verb args, stdio open-target patch set). ABI freeze until W2's `--packages all`. Captures
`wp-p8/generated/` (expendable); binaries `CARGO_TARGET_DIR=wp-p8/target`.

Status legend: **measured** = ran here, capture named; **source** = read from source only; **written, not run**.

**Captures:** `wp-p8/generated/` and `wp-p8/target/` (gitignored) were swept when the ticket folder was re-materialized at
12:50; captures named `generated/…` below are gone (their numbers stay as measured then). Durable captures since:
`.🧬semio/🌐hub/s12-p8-logs/` (clone runs, `census-committed.json` regenerated 15:1x: 46 agent-facing non-migrated in the
committed descriptors — they change only with W2's `describe`); the landing script logs to `.🧬semio/🌐hub/s12-p8-land/`.

## Status

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | architect `runAnalysis` declared View, emits artifact mutation | **fixed at the root, law green**: decision = product intent is "store an analysis record in the program" (the `CreateAnalysisRecord` leaf, D1's description) → declared `Mutation`. It was ALSO dead: all 8 analysis/exchange verbs were `BatchOnlyPendingRewrite` (`interactive-job.not-ui-safe`) → migrated onto a second retained factory `ArchitectExchangeCommandJobFactory` (public-invocation wire budget; lanes Config / Artifact+Config / Artifact+WindowConfig / HostOnly) + a config one-item preparation factory; undeclared `importProgramRequest` declared (file picker). Law found 20 findings before (`test-architect-law-before-2`): silent no-ops (patchRegisterItem, applyTemplate, setAdjacencyField, addRegisterItem on unknown register/template, removeRegisterItem, importProgram/importRegistersCsv on unreadable input), undeclared read args (setAdjacencyKind elementAId/elementBId/cycle, setAdjacencyField entityId/field/value, removeElement elementId, addElement name), unknown analysis/report kinds silently mapped to gap/executive summary → every one refuses by name now | law **0 findings over 22 verbs** (`test-architect-law-3`), architect lib 2092/2093 before the final pin fix (`test-architect-lib-2`), `check-architect-1` EXIT 0 |
| 2 | reasoning `addRelationship` hardwires `node-1 → node-2` | **fixed at the root, law green**: declared args `sourceId`/`targetId`/`kind`; handler uses the named nodes, else the first two selected graph nodes (retained + handle routes pass the selection); refuses missing/unknown/identical endpoints by name; relationship row only when both nodes carry an identity (no hardcoded identity 1→2); free edge/node ids (`addNode` no longer collides after deletes). Found on the way by the law: `deleteSelection` answered an empty success on an empty selection → refuses `wires.delete-selection-empty`. Decision: product intent = relate two chosen nodes (catalogue rows pass only `kind`, so the selection is the UI path) | `check-reasoning-1` EXIT 0; lib **195/195** (`test-reasoning-lib-2`); law + 4 command laws (`test-reasoning-law-3`) |
| 3 | flow `duplicateWidget` no-op handler → **flow split-brain** | **root-fixed as patch set `patches/p8-flow.py` (dry-run clean, 23 files, 65 hunks, reproduces the clone bit-for-bit)**. duplicateWidget itself works on the shell lane (retained child work); D1's "no-op" came from the unreachable pure `handle`. The law's leftover refusals led to the real defect, measured with a clone scratch law: flow had TWO scenes — the parent's genesis working-scene owner (what every renderer, `interaction_topology`, evaluation and the 9 parent-lane verbs read) and the composed content child (what `addWidget`/`moveMediaNode`/`nodeGraphEdit`/`spotlightCommit`/`duplicateWidget` write, and the only one persisted). After `addWidget` the main window did NOT render the new widget and `removeWidget` of it was refused; any parent-lane verb re-pointed the content coordinate at a new content-addressed child no store held (`NO-CHILD-STORE`), dropping every child edit and refusing every later child verb "before any capacity was measured". Fix: one composed read (`flow_composed_snapshot`) at every read entry, one child publication (`flow_scene_publication` / `flow_content_edit` / `host_scene_edit`, replacing the parent diff `host_operations`); the 9 verbs move to the `Child` lane; unknown targets / no-op edits refused by name (incl. duplicateWidget's missing source, which answered an empty success); duplicateWidget no longer needs a window transient. New law `the_content_child_is_the_one_scene_every_verb_and_window_reads` | clone: flow lib **257 passed / 0 failed** (`clone-flow-test-8`), tree baseline 252 / 5 failed (`tree-flow-test-baseline`: 4 tests still asserted silent no-ops my pre-freeze refusals replaced + the law placeholder — all fixed by the patch); flow law 0 findings / 28 verbs; `semio-s-plugin-flow` check EXIT 0 |
| 4 | writer `open-document` peer `[DEBUG]` `eprintln!` | **fixed**: 4 production `[DEBUG]` prints removed (open-document, main-window tokens ×2 — one printed the whole token JSON on every render — and the language-token inference). Census of other production `[DEBUG]` prints (not removed, peers may be debugging): trinity rewriting world ×2, space open-space ×3 + navigate-vfs ×1, procedural gen3d ×2, fem ×4 | `check-writer-1` EXIT 0 |
| 5 | gen3d viewer `setActiveExample` destructive but config-only | **fixed**: destructive flag dropped (it shows an example in this viewer's config and discards nothing; the audit lexicon asks this question of mutations only). The law's `destructiveWithoutDiscard` rule now catches the class for every surface | source edit (procedural check pending with the next procedural build) |
| 6 | gumball/transform verbs: one classification rule (lowpoly, puzzle3d/5d, fem, world3d) | **landed, fixture law green**: rule in the manifest (`framework_fixed_audience`: `transformBegin`/`transformEnd` = Input, `setTransformGumballFlag` = Chrome), enforced by `try_build_definition` for every app; offenders fixed: lowpoly brackets (were Agent), fem3d brackets (were Chrome), puzzle3d + puzzle5d toggle (were Agent); lowpoly delta descriptions corrected (they commit on their own outside a drag) | `check-manifest-1`, `check-plugin-3`, `test-manifest-gumball-1` (1/1), `check-gumball-1` (EXIT 0) |
| 7 | law (a): View/Config verb never emits an artifact mutation — every plugin + extension | **harness landed + rolled out to 3 surfaces** (`artifact_app_laws::probe_declared_verbs` / `declared_verb_findings` / `assert_declared_verbs_honour_their_declarations`): every declared verb probed on a fresh registered app booted with the surface's own example (`setActiveExample` replayed), from every window kind presenting it, staged + two values per perturbable arg; verdict fixture **28 cases** green. Surfaces: reasoning 0/8, architect 0/22, flow 6/28 open. Rollout to the rest is FROZEN (preamble rule 20) → prepared as a codemod | `test-plugin-verdicts-3` 3/3 |
| 8 | law (b): no declared mutating verb has a no-op handler | **in the same harness** (`silentMutation`, `destructiveWithoutDiscard`) | as row 7 |
| 9 | law (c): handlers consume every declared argument (no hardcoded ids) | **in the same harness** (`ignoredArgument`, staged third point for refusals, `othersSpecified`); a companion class it surfaced: handlers that READ undeclared args (agents cannot pass them) — declared where fixed (architect 7 args, reasoning 3, flow 18) | as row 7 |
| 10 | law (d): one gumball/transform classification rule | **landed** (see row 6); fixture `🛂️manifest/🧫️fixtures/🖐️gumball-verb-audience.json` (11 cases) | `test-manifest-gumball-1` |
| 11 | measured table: laws, offenders, fixed, per plugin | **partial** (§ Measured: reasoning, architect, flow, gumball + 6 surfaces of the 09:3x tree run); the full fleet (136 surfaces / 66 crates) runs AFTER landing, on the tree, as one sequence: `python3 .tmp-ticket/wp-p8/p8-surfaces.py .🧬semio/🌐hub/s12-p8-logs/surfaces.json && python3 .tmp-ticket/wp-p8/p8-harness-gen.py .🧬semio/🌐hub/s12-p8-logs/surfaces.json` (harness already regenerated against the tree, 15:5x) → `nice -n 15 cargo build` in `wp-p8/probe-harness` → `P8_DUMP=1 …/p8-probe-harness > .🧬semio/🌐hub/s12-p8-logs/fleet.jsonl` (the clone build was SIGTERMed at its final crate under swap pressure) | below |
| 13 | Coordinator add-on (B) + key deliverable: agent lane ≡ shell lane | **law landed; root fix = patch `p8-agent-lane.py`** (preview builds and runs the SAME retained job; lanes an agent transaction cannot carry are refused by name). G10's guest faults after it: cad preview → runs the job; flow `legacy-dispatch` → runs the job; sequence `content-child-dialect-required` → the job sees the child; what remains is ONE host-side gap routed to G10: the transaction carries parent ops only, so every child-group verb (flow ×10, sequence ×12) is `interactive-job.agent-lane-uncarried` until the gateway commits owned children. **Earlier state** (`agentLaneDiverges`, `declared_verb_agent_divergences`, each surface pins its list): the MCP prepare phase (`preview_addressed_action`) runs `A::handle`, the shell runs the retained tool-job work; for retained routes they differ (flow refuses in `handle`, cad needs an operation-bound view, sequence the job's child context) and effects/children/window lanes never reach the agent at all (every `LoadDocument` example switch is a no-op for agents). Measured: reasoning [setActiveExample], architect [setActiveExample] (selectRegister declared Chrome instead), flow: every child-group verb. Root fix designed (§ Agent lane) — SDK frozen → patch set | `test-reasoning-agent-2`, `test-architect-agent-2` |
| 14 | Coordinator add-on (A): dead BatchOnly commands | **census 46 → 5, all 5 routed with reason**: architect 8 migrated (tree); cad 5 (`p8-cad.py`), space studio 24 (`p8-space-studio.py`), space home `renameSpace` (`p8-space-home.py`: its "catalog + filesystem" blocker was stale — the handler is a dialog/hub relay like its HostOnly siblings) migrated as patches. Left, routed (§ Routed): space home `bindSpaceFile`/`importSpace`/`deleteVirtualFileSystemNode` (backbone+catalog IO inside `handle`, errors swallowed, empty success — needs an IO-owning job, not a reclassification), animate `exportVideoFromDeck` (host video capability), procedural `listFlowExtensions` (declared without a classification). Law: the probe's `unreachable` finding (`interactive-job.not-ui-safe`) is the census law per surface | `generated/census-committed.json`, `patches/*.py --dry-run` |
| 15 | law (e, new): no verb orphans a composed child | **HELD set `p8-orphan.py`** (not in the default landing): `composedChildOrphaned` — after every settled probe the law lists each child the parent names that no store holds and did not before. Measured in the clone it is TRUE beyond flow: reasoning wires `addNode`/`addRelationship` orphan `content/wires-content-*` (the parent re-mints its content-addressed child on every edit; wires never reads the child, so the live composed child — what exports, archives, hub members and agents read — is stale). Source: 12 plugins mint content-addressed children from parent edits (dag, writer, sequence, animate, playbook, imperative, trinity jack, raster, note, reasoning, cad, flow). Landing it now would turn the reasoning law red → it lands with the class fix (§ Routed) | clone `clone-agent-surface-laws` (reasoning: 2 orphan findings); `p8-agent-lane`+`p8-law`+`p8-orphan` reproduce the clone bit-for-bit, twin 28 cases after `law`, 29 after `orphan` |
| 16 | third-party oracle of the verdict fixture | **written, measured in the clone, patch `p8-law.py` part C**: the fixture's `why` named an AJV twin that never existed (my miss). Now `🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` (beside the Rust verdict test) states every rule as ONE JSON Schema over the probe, AJV evaluates it (`$data` compares the two perturbed outcomes); runs in the SDK `test` script and in `p8-land.py --test`; `why` names the real path + the orphan rule | clone `bun …/🟦️.ts` → `cases=29`; red-check: mutating an orphan, an ignored-argument and an agent case each fails with its case name |
| 17 | landing window = ONE command | `python3 .tmp-ticket/wp-p8/p8-land.py --write --test` (plain run = dry run of all sets). Order: `p8-agent-lane` (4 files: SDK, retained-command, action-bus + the architect law's agent pin, which the SDK change moves: `exportProgram`/`exportRegistersCsv`/`importProgramRequest` are now refused by name — measured in the clone) → `p8-law` (4: boot args, AJV twin, fixture `why`, SDK test script) → `p8-flow` (23) → `p8-cad` (4) → `p8-space-studio` (8) → `p8-space-home` (3); `p8-orphan` HELD (`--only orphan`). Preflight dry-runs every set; each set is backed up (`.🧬semio/🌐hub/s12-p8-land/`), written and gated by `cargo check -p` of the crates it touches (after agent-lane: semio-framework, SDK, flow/cad/sequence/architect natively); a failing gate restores only that set and stops; `--test` then runs the reasoning + architect + flow + cad + space laws, the verdict law and the AJV twin. Clone validation: flow lib 257/257, cad lib+tests check EXIT 0, space home + space plugin lib+tests check EXIT 0, architect law needs the new pin (measured). Not yet run: reasoning law without the orphan rule (build-quiet since rule 24) — the window's `--test` runs it | `p8-land.py`, `s12-p8-logs/clone-*` |
| 12 | W2 request `wp-w1/requests/p8.txt` | **filed** (gumball, reasoning, architect); flow/writer/procedural pending the landing window | `wp-w1/requests/p8.txt` |

## Measured (per plugin, laws run here)

| plugin / surface | verbs probed | findings before | after | fixed at the root | agent-lane divergences (pinned) | captures |
|---|---:|---:|---:|---|---|---|
| reasoning wires editor | 8 | 3 (addRelationship hardwired ids — not visible to the law until its args were declared; deleteSelection silent + destructive-without-discard) | **0** | declared sourceId/targetId/kind; named endpoints else selection; refusals by name; free ids | setActiveExample | `test-reasoning-probe-1`, `test-reasoning-lib-2` (195/195), `test-reasoning-agent-2` |
| architect program editor | 22 (21 before) | 21 → 20 (per-window) | **0** | runAnalysis Mutation; 8 BatchOnly migrated + importProgramRequest declared; 11 silent/ignored → refusals by name; 7 undeclared read args declared; selectRegister Chrome | setActiveExample | `test-architect-law-before-2`, `test-architect-law-3`, `test-architect-agent-2` |
| flow editor | 28 | 14 | **0** (clone, patch set) | 9 refusals by name, 18 args declared on 9 verbs (tree); ONE scene = the composed content child for every read and write, 9 verbs onto the Child lane, 5 more refusals by name (patch `p8-flow.py`) | addWidget, removeWidget, duplicateWidget, disconnect, connectMediaPorts, moveMediaNode, reorganize, patchFlowWidgets, renameFlowWidget, setActiveExample (child groups: `agent-lane-uncarried` → G10 carrier), evaluate (host-only job, `preview-unsupported`), focusSelection (main-window camera, agent address names no window) | `test-flow-law-1/2`, `clone-flow-test-3..8` |
| gumball rule (lowpoly, fem3d, puzzle3d, puzzle5d) | 16 declarations | 4 offenders | **0** | rule enforced at definition build | — | `test-manifest-gumball-1` |

## Routed (owner decisions, with evidence)

| to | item | evidence |
|---|---|---|
| G10 (MCP gateway) | the agent transaction carries parent ops only: after `p8-agent-lane` every child-group verb (flow ×10, sequence ×12) and every effect-bearing verb is refused `interactive-job.agent-lane-uncarried` by name instead of committing nothing; the gateway must commit owned children + host effects to reach parity; `createStudio` needs the gateway to stamp the session identity | flow law pin (row 3), `test-reasoning-agent-2`, `test-architect-agent-2` |
| cad owner | `applyTransformation` + OBJ/STL/STEP object import are stubs since the composable migration (no pane-model child seam); solid exports fall back to spatial JSON; `addObject` documented no-op — `p8-cad` makes them refuse by name | `p8-cad.py` doc |
| space owner | home `bindSpaceFile` (no-op on wasm32), `importSpace`, `deleteVirtualFileSystemNode`: IO in `handle`, errors swallowed, empty success; studio `compiledDagEngagementSubmit` documented empty stub | source |
| animate owner | `exportVideoFromDeck` BatchOnly: needs the host video capability | `census-committed.json` |
| procedural owner | `listFlowExtensions` declared without an interactive-job classification | `census-committed.json` |
| coordinator (design decision) + owners of dag, writer, sequence, animate, playbook, imperative, trinity, raster, note, reasoning, cad | **content-addressed children re-minted by parent edits** (the class behind flow's split-brain): every parent edit names a new `…-content-<hash>` child that no store holds, so the live composed child is stale for exporters, archives, hub members and agents, and each edit would mint a new hub artifact id. Fix per surface = flow's pattern (the child is the one scene at a stable coordinate; edits publish on the child lane — `p8-flow.py`; sequence already reads its child) or one SDK rule (derivable children follow their coordinate and the unnamed one is retired). The held law `p8-orphan.py` lands with that fix | clone: reasoning wires 2 findings; source: 12 plugins |
| stdio owner | 52 `full-app-catalog` example editors do not compile (E0407 ×29 each; not shipped) | log 09:1x |

## Log

- 00:58 started; read AGENTS.md, preambles 12/11, `wp-d1.md`, `wp-t12.md` (in-flight: trinity clearSelection, note verb
  args, stdio open-target patch set — not touched).
- 01:0x–01:20 design: the mounted dispatch (`handle_action` → `settle_registered_typed_operation`) is the only faithful
  observation point — retained (Migrated) verbs never run `ArtifactApp::handle` live (flow `duplicateWidget`: pure
  `handle` is a no-op, the retained step work implements it), and the framework's own kind guard only fires at
  publication. One generic probe per declared verb on a fresh registered app loaded with the app's own boot example.
- 01:2x framework law landed in `artifact_app_laws` (`🔖️DeclaredVerbLaws`): probe (staged defaults + two distinct values
  per perturbable declared arg), outcome = unreachable / refused / settled {lanes, document/config change, LoadDocument,
  user-path write, host effects, fingerprint incl. the invoking window's render}; verdict rules a/b/c + destructive +
  unreachable + unbridged. Fixture `🔌️plugin/🧫️fixtures/⚖️declared-verb-verdicts.json`. check EXIT 0; verdict tests 3/3.
- 01:3x coordinator rule 18 (`nice -n 15` on every cargo) adopted. Coordinator add-on (A) BatchOnly census + migration,
  (B) G10 agent-lane guest faults: queued behind the seeds; the probe's `unreachable` class IS the (A) law.
- 01:3x gumball rule (d) landed in the manifest + builder validation; 4 offenders fixed; checks EXIT 0.
- 03:4x resumed after the usage cut. Reasoning: law first run red (deleteSelection silent + destructive-without-discard),
  then `interactive-job.app-owned-output` misread as infrastructure — it is the retained job's code for the handler's own
  fault page, now classified `refused`. Probe law extended: surface example args (`🧫️fixtures/⚖️declared-verb-examples.json`,
  overlaid on staged defaults) + `othersSpecified` (equal refusals only count when every sibling arg had a value);
  law returns the probes so a surface pins what its examples made act. Verdict fixture 19 cases.
- 04:0x–05:00 architect: red-first law run (21 findings over 21 verbs, then 20 with per-window probing), fixes as in row 1,
  law green (0/22). Probe law revised: verbs are probed from EVERY window kind that presents them (an app-level
  window-config verb only acts from its own window — architect `selectRegister`); equal refusals count as ignoring an
  argument only when the staged invocation was refused identically too (a body both perturbations fail to parse
  the same way was still read). Verdict fixture 23 cases, green. One test command overran 600 s (fleet load 40–80)
  and the harness moved it to the background; waited on its capture in one blocking call, exit 0.
- 05:0x–05:4x agent lane: G10's cad/flow/sequence faults traced to ONE cause (preview runs `A::handle`, shell runs the
  retained work); coordinator informed and approved "same code for agents and humans" (constraints: no ABI change, compile-
  atomic, live via W2's restage). Law: `agentLaneDiverges` + pinned `declared_verb_agent_divergences` per surface. Probe now
  replays the surface's own `setActiveExample` per fixture (an example that lands as ops, children or LoadDocument boots
  the same way the shell boots it); child envelopes are part of the observed document; Input gestures are exempt from
  staged bridging. Flow law red-first 14 → 6.
- 05:41 my SDK edit (`🔌️plugin/🦀️.rs`, probe boot replay) restarted W2's bootstrap closure builds (coordinator). 05:5x
  **preamble rule 20: HARD guest freeze** — no edits to the SDK, guest-linked framework crates or `✏️s/🔌️plugins/**` until
  W2's `--packages all` is announced. Everything from here is a dry-run-clean patch set under `wp-p8/`.
  Left on disk from before the freeze, to be removed first in the landing window: the flow law file's temporary
  `p8_scratch_flow_probe_dump` test (a `[DEBUG]` dump, test-only).
- 08:4x resumed; tree edits ended 05:46:57 (before rule 20) — coordinator told; nothing edited in the tree since.
  Everything guest-side now as patch sets under `wp-p8/patches/` (engine `p8_patch.py`: anchored hunks, all-or-nothing,
  `--dry-run` / `--write`).
- 09:0x patch `p8-agent-lane.py` (dry run clean, 3 files, 7 hunks): (A) action-bus `ToolPayload::into_inner`; (B)
  `ArtifactRetainedCommandJob::preview_emit` (preflight + work, nothing published); (C) `capture_typed_command_roots` — the
  ONE capture both lanes use; (D) `preview_addressed_action` builds the SAME job (`A::build_tool_job`) and runs it, refuses
  lanes an agent transaction cannot carry (children, host effects, window config, extension calls, tasks, selection
  writes) with `interactive-job.agent-lane-uncarried` instead of committing nothing. Written, not compiled yet (clone below).
- 09:1x patch `p8-cad.py` (dry run clean, 4 files, 20 hunks): cad's 5 BatchOnly verbs onto its retained factory (lanes:
  applyTransformation Artifact, importCadFile Config, 3 exports HostOnly; importCadFile gets the contributions wire budget);
  the two cad stubs refuse by name (`cad.apply-transformation-unavailable`, `cad.import-object-unavailable`,
  `cad.import-unreadable`) instead of settling silently; descriptions say so. **Routed to the cad owner**: the stubs
  themselves (applyTransformation, object import, `collect_pane_solids` ⇒ solid exports always fall back to spatial JSON,
  addObject documented no-op) need the composed pane-model child seam (`Emit::child_emits` exists since C1).
- Fleet harness `wp-p8/probe-harness` (ticket-local standalone crate, path deps, `P8_DUMP=1` dumps failing probes):
  first builds failed only on stdio: **finding (stdio owner, frozen)** — all 52 stdio "example factory" editors put
  their `ArtifactEditor` items (`bounded_first_step_tool_proofs!`, `register_tool_job_factories`, `build_tool_job`, …)
  inside `impl ArtifactOwnedToolJobFactory`, so stdio's `full-app-catalog` feature does not compile (E0407 ×29 per
  editor). Not shipped (the component ships 7 text/data crates only), so not W2's failure; library-only dead code.
  Harness now probes the shipped surfaces only (138 surfaces / 66 crates).
- Scratch clone `.🧬semio/🌐hub/s12-p8-clone` (rsync of Cargo.toml/lock, .cargo, ✏️s, 🌎️hub, 🧰️framework, 🧫️fixtures,
  🧪️tests minus target/node_modules/dist) to apply and COMPILE the patch sets without touching the tree.
- 09:2x clone moved to the scratchpad (inside `.🧬semio` the MutationLeaf derive found the REAL repo root via
  `📋️project.json` and refused every leaf; root-level files copied into the clone). **Patched SDK compiles in the clone**:
  `p8-agent-lane.py` + `p8-cad.py` applied, `cargo check -p semio-framework-plugin --features artifact-app-testing --lib
  --tests` Finished, 0 errors, no warning in the patched code (`s12-p8-logs/clone-check-plugin-2.txt`).
- 09:3x fleet run on the tree (6 surfaces before a crash): writer 13 findings over 17 verbs (agent divergences
  lintDocument, setActiveExample, openDocument), mathematical 4/7, wfc2d 2/21. Crash = wfc grid2d boot: its
  `setActiveExample` has a closed choice without default → empty id refused → `p8-law.py` (boot with the first option,
  the navbar's first example). Harness now runs every surface in its own 512 MiB thread and reports a panic as a row.
- 09:4x patch `p8-space-studio.py` (dry run clean, 8 files, 36 hunks): all 24 studio BatchOnly verbs onto the bounded
  retained factory with the lanes each handler emits; the 6 selection verbs read the retained interaction state through
  their existing selection bodies (now `pub(crate)`); the language-neutral catalogue fixture + its oracle law move to 40/40
  bounded+migrated, 11 HostOnly. `compiledDagEngagementSubmit` is a documented empty stub (Input gesture) — routed.
- 09:5x harness rebuilt against the PATCHED clone (agent-lane + cad + space + law) — measures the post-landing fleet.
- 10:0x–11:3x flow, measured in the clone with a scratch law (removed): boot → `addWidget` settles `[Child, Ui, Terminal]`
  but the parent scene stays `[slider, add, preview]` and the rendered main window (decoded `NodeGraphScene`) lacks
  `note_2`; `removeWidget note_2` → "found no widget"; `removeWidget slider` settles `[Artifact, …]` and leaves
  `NO-CHILD-STORE`; the next `addWidget` is refused before any capacity. Root: the composable-artifact migration moved the
  scene into the content child but left every reader and 9 writers on the parent's genesis owner (`to_host_snapshot`,
  "staleness gap"), which only a parsed/genesis document carries — a reloaded document rendered empty too. Fix in the
  clone (row 3), then `wp-p8/p8-hunks.py` turns the clone edits into anchored hunks: `patches/p8-flow.py` dry-run clean,
  verified to reproduce the clone file-for-file (`P8_ROOT` copy). Found on the way: the declared-verb law depended on
  global extension registration (other tests installing the first-party operators changed connect/evaluate outcomes) →
  the flow law installs them itself; agent-lane divergence list is now deterministic and pinned.
- 11:3x space home: `renameSpace` migrated in the clone (`p8-space-home.py`, 3 files, 8 hunks, dry-run clean) — the fixture
  blocker named a catalog + filesystem rename that the handler no longer does. Orphan law (row 15) added to the probe and the
  verdict fixture in the clone; verdict tests green there; `p8-agent-lane` + `p8-law` reproduce the clone bit-for-bit.
- 15:0x resumed after the app restart (all processes died; clone + patches intact). All 6 sets dry-run clean on the tree.
  Landing script `p8-land.py` written and exercised on a scratch root (row 17). Fleet harness rebuild restarted detached
  against the clone (the 09:43 binary predates flow/space-home/orphan) → measured table when it links.
- 15:2x AJV twin of the verdict law written and red-checked in the clone; `p8-law` part C creates it and hooks it into the
  SDK `test` script; `p8-agent-lane` + `p8-law` reproduce all 7 clone files bit-for-bit; `p8-land.py --test` runs it.
- 15:3x the fleet harness rebuild (136 surfaces, clone) got SIGTERM from outside at the final crate (swap 11.8/13.3 GB, load
  120+); not restarted during W2's publish. Decision: the per-plugin fleet table is measured AFTER landing, on the tree
  (`p8-harness-gen.py` with the default deps root = tree → `cargo build` in `wp-p8/probe-harness` → run the binary): that
  measures what actually ships instead of the clone. Until then the measured table covers reasoning, architect, flow
  (clone), gumball, plus the 6 surfaces of the 09:3x tree run. Landing gates now also run the reasoning + architect
  declared-verb laws after `agent-lane` (their pinned agent divergences are what the SDK change can move).
- 15:4x (build-quiet from rule 24 on; only running test binaries finished) clone laws with the agent-lane patch: architect
  pin moves to [setActiveExample, exportProgram, exportRegistersCsv, importProgramRequest] (exports/picker are now honest
  refusals) → folded into `p8-agent-lane` part E; reasoning law red on 2 `composedChildOrphaned` findings (wires) → the
  orphan rule is split out as the HELD set `p8-orphan.py`; the content-addressed-child class is routed (§ Routed).
  Space home + space plugin and cad lib+tests compile in the clone (one retry after the shared build-dir lost
  `semio-framework-value-derive` mid-run to a peer prune).
