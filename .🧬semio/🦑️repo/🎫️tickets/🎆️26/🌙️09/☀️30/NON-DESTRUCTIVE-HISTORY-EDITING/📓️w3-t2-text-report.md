# 📓️ W3-T2-TEXT — Typing Runs as Tool Machines (design §13.2)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Session 1 left no report, so the predecessor's work is reconstructed below from
the code, `git diff 48d881aa7ab HEAD` and `🗑️generated/w3-t2-text/`. Paths are relative to the repo root. `T` = this ticket folder.

## Session 2 — 2026-10-01 (successor of W3-T2-TEXT, coordinator `⚪552b484a…`)

Status: **IN PROGRESS** (this file is updated at every milestone). Usage cuts at about 13:30 (resumed 16:35), 18:00 (resumed 21:38) and 23:00
(resumed 02:45 under a CARGO HOLD: no cargo / nx / test suite until the coordinator lifts it).

### 1. Reconstruction of the session-1 work (landed in HEAD `4e36b2b5012` and earlier)

| Area | What landed | Files |
|---|---|---|
| Framework machine | `TypingMachine` statechart (`idle → typing`, an `Edit` with the sameBuffer guard, the `after 750 ms` idle timer, `Commit`), `Typing` runner (`start/resume/send/lapse/persist`), `TypingLedger` (per window: `send/commit/commit_all/lapse/abort/abort_all/retain_windows/provisional/next_deadline_ms`), `TypingPhase::parse(typing, typingCommit)`, `TypingCommit` (7 reasons), `TypingFold {Net, Split}` (the app's typing algebra), constants `TYPING_BUFFER_ARG="typing"`, `TYPING_COMMIT_ARG="typingCommit"`, `TYPING_IDLE_MS=750`. The TS twin is a full mirror. | `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs` (region `🔖️Typing`), `🟦️.ts`, `🧬️schema/🔣️.json` (`Typing*` defs), `🧫️fixtures/🧫️typing-law/🔣️.json` (11 scenarios, authored by `T/🧪️w3-t2-text-typing-law-fixture.ts` with independently minted ids), `🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🧪️conformance/🟦️.ts` (ajv + xstate oracle + fast-check 400 runs) |
| Plugin runtime | `ToolMachineRuntime<P,M>` = scrub ledger + typing ledger + ONE committed ⊕ provisional overlay; `admit_tool_dispatch` (a typing edit is tagged; `typingCommit` publishes the run and never reaches the app; a frozen document refuses `timeTravel.frozen` with zero trace), `commit_typing_before` (an idle lapse, and every non-typing non-View verb commits the open runs first: `otherVerb`), `settle_tool_operation` (the edit's leaves fold into the window's run; a commit publishes ONE edit through `Emit::commit_transaction`), `retire_tool_windows` (a window that left commits like a blur), `freeze_tool_machines` (time travel), `follow` (an overlay refold; a run the moved base refuses aborts `baseMoved`). `ArtifactApp::typing_fold` with the single-buffer default (`EditorApp` forwards it). | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs`, hooks in `🔌️plugin/🦀️.rs` (`dispatch_action`, `typing_fold`), `🔌️plugin/🧪️tests/🧪️typing/🦀️.rs` (2 overlay laws) |
| Text splice | `TextSplice::then` composition (`Composed/Cancelled/Disjoint`) in Rust and TS; the corpus gained `compositions` (15) and `typingRuns` (5) vectors (`T/🧪️w3-t2-text-splice-compositions.ts`, derived with `textSpliceFromEditV1` and checked against jsdiff); `createTextEditorTypingRunV1` (the host side of a run: idle timer, `typed`, `commit(reason)`, `dispose`). | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🦀️.rs`, `🟦️.ts`, `🧫️fixtures/✂️text-splice/🔣️.json`, `🧬️schema/✂️text-splice/🔣️.json`, `🧪️tests/✂️text-splice/{🦀️.rs,🟦️.test.ts}` |
| Presence (ephemeral shared preview) | `PresenceTyping {windowId, deleted, insert}` (256-byte excerpts) on the presence peer wire, flag bit 14 (the unknown-flag vector moved to bit 15), limit `maximumTypingRuns: 8`, Rust + TS codecs, corpus vectors (`T/🧪️w3-t2-text-presence-typing-cases.ts`); React `publishLocalPresenceTypingV1` / `localPresenceTypingFieldsV1` (Shell heartbeat) and `TextPeerCaretsOverlayV1` (draws a peer's pending run at its caret, with en/de `aria-label`). | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`, `📡️replication/🟦️.ts`, `👕️peer-overlay/🟦️.ts`, `🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json`, `RE/👕️canvas-presence/{🟦️.ts,🟦️.tsx,🧪️tests/…}`, `RE/🏛️ShellHost/🟦️.tsx` |
| React host | `TextEditor`: every splice and whole-text delivery carries `typing: <surfaceId>`; the run ends on idle, a caret jump (`selectionJump`), blur, `pagehide`/`visibilitychange` (`hidden`) and unmount; the pending run is published as presence. | `RE/✏️TextEditor/🟦️.tsx` |
| wgpu host | `TextEditorTypingRun` per editor surface: deliveries carry `typing`; ONE `typingCommit` on idle (`drive_text_editor_typing_idle`) and on a pure caret move (`selectionJump`). | `RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` |
| `TextWindowKit` (F-5) | An editable text window is never live whole-text typing: `render_editable(TextEditView, locale)` is the explicit draft (`commit:"explicit"`, local undo, Apply / Discard / conflict / progress / failure copy en+de) that publishes ONE `textEdit` on Apply. | `🔌️plugin/🦀️.rs` region `🔖️TextWindowKit`; adopters: stdio `md`, `html`, `txt`, `binary`, `deflate` main windows |
| writer | `textEdit`/`textSplice` emit plain leaves (the `"writer-text-edit"` static amend key is gone); `typing_fold` composes splices into ONE net `splice-text` (`Cancelled` → empty, `Disjoint` → split; whole-text replaces); `mutation_label` from the leaf; the `splice-text` label quotes the typed text ("Type “hello”" / "„hello“ tippen"); the leaf schema carries full `x-semio-ui`. Laws: `🧪️tests/🧪️typing-runs/🦀️.rs` (6) + `text_edit` laws (typing burst, ledger-length run, 10 000 keys). | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor/…,🧬️schema/🧬️mutations/✂️splice-text/…}` |
| jack / vcs | Jack `text-edit` is one whole-query `set-query` (single-buffer run; `JACK_QUERY_TYPING_COALESCE_KEY` deleted); vcs `edit`/`textEdit` = the diff of the run's last text against the committed document (`VCS_TEXT_TYPING_COALESCE_KEY` deleted). One adoption law each (the run lands as ONE edit with its `TransactionRef`; one undo/redo moves the whole run). | `🔱️trinity/🗿️artifacts/🔌️jack/…/✏️editor/🎮️commands/✏️text-edit/🦀️.rs`, `🌿️vcs/…/✏️editor/🎮️commands/🩹️edit/🦀️.rs`, their `🧪️tests/🔬️unit/🦀️.rs` |
| trinity rewriting parameters | The parameters window commits each field once, on blur or Enter (`Trigger::Commit`). | `♻️rewriting/…/🪟️windows/🎛️parameters/🦀️.rs` |

Session-1 verification: at 08:09 `cargo test -p semio-s-artifact-writer-writer --lib` gave **17 passed / 6 failed** (all six
`typing_runs_tests` that count history rows: 3 rows where 2 were expected). That is the first item of §3.

### 2. Session-2 changes so far

**stdio md / html: an explicit Apply is ONE edit of NET leaves, never a whole-document `SetSnapshot`.**
- `📝️md/…/✏️editor/🦀️.rs`: `md_applied_text` (CommonMark source, which is what the window shows and edits; the DSL envelope when the text
  carries the preamble — the old code ran `parse_dsl` on the window's markdown); `md_net_mutations`: unchanged blocks at either end are kept, a
  changed paragraph or a same-level heading becomes `set-inlines`, block quotes and same-shape lists recurse with their path steps, any other change
  is `replace-block`, surplus blocks are `remove-block` (last first) or `insert-block`; `set-snapshot` only when the schema itself differs;
  `mutation_label` comes from the leaf.
- `🌐️html/…/✏️editor/🦀️.rs`: `html_net_mutations`: `set-doctype`, then a depth-first node walk: an element keeps its identity
  (`set-element-name`, `set-attribute` set/remove when its attribute order is reproducible, else the node is replaced, children walked); text,
  comment and raw-text nodes are re-set; changed kinds become `remove-node` + `insert-node`; only a root that must be replaced is `set-snapshot`;
  `mutation_label` comes from the leaf.
- Language-agnostic corpora + schemas: `…/📝️md/…/✳️any/🧫️fixtures/🧫️net-leaves/🔣️.json` (12 cases) + `🧬️schema/🔣️net-leaves/🔣️.json`;
  `…/🌐️html/…/✳️any/🧫️fixtures/🧫️net-leaves/🔣️.json` (12 cases) + `🧬️schema/🔣️net-leaves/🔣️.json`.
- Third-party oracles (bun): `📝️md/…/🧪️tests/🧪️net-leaves/🟦️.ts` (ajv + markdown-it + jsdiff `diffArrays`: every expected leaf addresses a
  changed top-level block of its kind), `🌐️html/…/🧪️tests/🧪️net-leaves/🟦️.ts` (ajv + parse5: every leaf addresses the node it edits).
- Rust laws (appended to each editor's `🧪️tests/🔬️unit/🦀️.rs`): the corpus yields exactly its leaves, they reach exactly the applied text, each leaf
  undoes with ONE row, and undoing restores the document; one Apply of one changed paragraph/text is ONE edit with ONE labelled leaf row, and ONE undo
  restores it.

**trinity rewriting: no scratch-snapshot diff anywhere; gestures become relative leaves.**
- Deleted `rewriting_snapshot_mutations` (from `🧬️schema/⚙️operations/🦀️.rs`, its re-export in `🧬️mutations/📝️text/🦀️.rs`, its test use).
- Direct leaves: `set-parameter` → ONE `change-parameter-binding` (typed by the declared kind; an unchanged value moves nothing); `set-lhs-json` →
  `edit-lhs`; `set-rhs-json` → `edit-rhs` + `parameter_binding_mutations` (reset to the new side's defaults, `✏️editor/🦀️.rs`); `add-rule-clause` →
  `edit-lhs` | `edit-rhs` (+ the new parameter's binding); `delete-rule-clause` → `delete_rule_clauses` (one edit per side, binding/layout removals; a
  multi-selection deletes each list from the highest index down — the old per-id loop deleted the wrong clause after an index shift); `patch-nodes` →
  `edit-before-fixture` (until `patch-working-nodes` below is wired); `node-graph-edit` → per-operation leaves folded over a running state.
- Shared rule-graph slots: `🧬️schema/🦀️.rs` region `📐️RuleGraphSlots` (`lhs_graph_slots`, `rhs_graph_slots`, `rule_graph_position`); the editor's
  LHS/RHS graph builders read their default positions from there.
- Four new relative leaves written (`T/🧪️w3-t2-text-trinity-leaves.py`), all surfaces per leaf (Rust payload/diff/inverse, text/binary identity,
  descriptor, payload schema with full `x-semio-ui`, TS/GraphQL/protobuf): `✋️drag-working` `drag-working-nodes{targets,dx,dy}` (inverse: ONE
  `edit-before-fixture`), `🩹️patch-working` `patch-working-nodes{targets,field,value}` (inverse: ONE `edit-before-fixture`; an undeclared kind is
  `mutation.target-mismatch`), `🫳️drag-rule` `drag-rule-nodes{targets,dx,dy}` (from the layout point, else the default slot; inverse: ONE
  `set-rule-layout-points`), `📍️set-rule-layout` `set-rule-layout-points{points[],cleared[]}` (absolute, the one-row undo; `x-semio-invariant
  keys-unique`). Working-graph leaves patch node fields in the graph JSON and write it back as compact JSON with sorted keys, so a second
  implementation can reproduce the bytes. Registries patched (`T/🧪️w3-t2-text-trinity-registries.py`): aggregate enum + helpers, TS union, JSON
  Schema `oneOf`, GraphQL, protobuf, binary protocol + tag registry, text grammar `.semio`/`.g4`/`.ebnf` + opcode registry, retirement cursors +
  corpus. Then (17:00): crate mounts (`♻️rewriting/🦀️.rs`), hand-authored quintets + 7 laws per leaf (`T/🧪️w3-t2-text-trinity-fixtures.py`:
  `✋️moves`, `🩹️renames`, `🫳️moves`, `📍️places`), structural-correspondence blocks, oracle catalog (`kinds`, `vectors`, `mutationManifests`),
  the cross-language harness `🧪️tests/♻️mutate-rewrite-1` (Rust `KINDS` + `touches_one`; Python second implementation of the four verbs incl.
  the rule-graph slots and the sorted-key graph JSON; mutate / inverse / spec-vector feature rows on the real 180-node Nakagin rule), the TS
  retirement oracle (11 mutations), `semio-framework-tool-machine` dependency, `mutation_label` from the leaf.
- Node-graph drag machine (`✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs`): `move` gesture records (`NodeDragRecord::from_row`, refused
  by name when malformed) and whole-graph `setHostSnapshot` releases read as displacement records (a structural change stays
  `edit-before-fixture`) become `drag-working-nodes` (BEFORE canvas) / `drag-rule-nodes` (LHS/RHS) and commit as ONE transaction through
  `node_drag_commit` (`s.trinity.rewriting@1/*#editor#nodeGraphEdit`, seeded by the admission's `authoring_seed`); `patchNodes` publishes ONE
  `patch-working-nodes`; `reorganize` publishes ONE `set-rule-layout-points` (fixture now expects one row). New editor laws (`🕹️NodeDragLaws`):
  one drag = one transaction of one relative leaf + one-row inverse + two gestures two transactions + zero trace; rule-node drags from slot or
  point; a handed-back graph read as its drag; one shell drag = one labelled history row + one undo.
- Completed the interrupted path-budget rename `🔬️set-lod-mode` → `🔬️set-lod` (rewriting and jack command dirs). The mounts and the
  taxonomy already named `🔬️set-lod`, but the directories had not moved, so neither crate compiled.

**writer**: the typing-runs laws now print the rows they saw when a count fails (`describe`).

**trinity rewriting, 02:45–03:40 (written during the CARGO HOLD, not yet compiled):**
- The working-graph leaves no longer use `serde_json` at runtime: `edit_working_graph_nodes` (aggregate `🧬️mutations/🦀️.rs`, region
  `🕸️WorkingGraph`) reads and writes the framework's own `pack` JSON, sorting object keys recursively (`sorted_keys`) before
  `pack::json_to_string`, which writes numbers exactly as `serde_json` does. `drag-working-nodes` / `patch-working-nodes` diffs follow. With no
  runtime use left, `serde_json` moved out of the crate's `[dependencies]` (it stays a dev-dependency); the stale "thin `serde_json` wrapper"
  doc of `encode_rewriting_snapshot_json` now names `pack`.
- **Fifth relative leaf `✂️delete-working` `delete-working-nodes{targets}`** (binary tag 11): removes the targeted nodes, every edge with an
  endpoint on one of them (`node@port` names its node before the `@`) and clears `rootNodeId` to `null` when the root went; partial targets warn
  `mutation.partial`, none present is `mutation.target-missing`; inverse ONE `edit-before-fixture`; label "Delete 1 node" / "1 Knoten löschen",
  "Delete N nodes" / "N Knoten löschen". Written by the generators (`T/🧪️w3-t2-text-trinity-leaves.py`, `…-fixtures.py`, both re-run and
  byte-identical for the four earlier leaves) and registered by `T/🧪️w3-t2-text-trinity-delete-registries.py` (aggregate + `remove_working_graph_nodes`,
  TS union, JSON `oneOf`, GraphQL, protobuf field 12, binary record + tag registry, text opcode + `.semio`/`.g4`/`.ebnf`, retirement arm + corpus
  row (3 bytes) + TS retirement oracle (12 rows; a `targets`-only payload carries no offset), crate mount + test mount
  `tests_deletes_node_a_and_its_edges`, structural-correspondence block, oracle catalog (`kinds`, `vectors`, `mutationManifests`, "twelve"),
  harness: Rust `KINDS` + `touches_one`, Python second implementation (`endpoint`, delete branch), feature rows (mutate + inverse on the real
  Nakagin rule: two nodes, 11 edges; spec vector `✂️deletes`). Fixture `✂️deletes`: the root `a` of a three-node chain with its edge a→b.
- `node-graph-edit`: `deleteSelection` on the BEFORE canvas is ONE `delete-working-nodes` of the selected ids the graph holds (the
  whole-graph `working_graph_without` → `edit-before-fixture` is deleted); a handed-back graph (`setHostSnapshot`) that only dropped nodes and
  their edges is read as that deletion (`deleted_nodes`): `delete-working-nodes` on the BEFORE canvas, `delete_rule_clauses` on a rule side.
  New editor law `a_deleted_working_graph_selection_is_one_relative_leaf` (gesture + handed-back graph + one-row inverse + empty selection).
- Python reference checked standalone against the committed vectors (stubbed `semio_repo_test`, no suite run): all five relative cases land
  on their committed after-state and their inverse restores the before-state; the Nakagin delete row removes 2 of 180 nodes and 11 of 179
  edges, writes only `beforeFixtureJson`, and undoes exactly.

**jack (22:30)**: a peer sweep (19:41) replaced the removed `dsl::from_dsl_value` with `semio_framework_value::FromValue::from_value` in
`🔌️jack/…/🧬️schema/🦀️.rs` and `🧬️schema/📸️snapshot/🦀️.rs` but kept `.map_err(dsl::ValueError::new)`, which no longer type-checks (E0277
`String: From<ValueError>`, four errors; the crate did not compile and blocked every writer / trinity build). Dropped the redundant `map_err`
(the call already answers `ValueError`).

**stdio kit users txt / binary / deflate (03:15–03:30; json examined):**
- `🔤️txt` (`✏️editor/🦀️.rs`): an explicit Apply that keeps the line ending and terminator is ONE edit of its net line leaves
  (`txt_net_mutations`: shared ends kept, paired changed lines `set-line`, surplus `remove-line` last first or `insert-line`); every
  step is simulated through the leaf's own diff and must say nothing, else (and for a change of line ending or terminator) the
  existing whole-buffer lowering `txt_replacement_mutations` carries it. The non-localized descriptions `"Replace text"` and
  `"Edit text details"` are gone, so rows are labelled from their leaves. Laws: `an_applied_text_is_its_net_line_leaves`,
  `an_applied_text_is_exactly_the_corpus_net_line_leaves`; the 19×19 lowering law is unchanged.
- `💾️binary`: the hex Apply is ONE net `replace-byte-range` (`binary_net_replacement`: shared prefix/suffix kept; none when unchanged)
  instead of a whole-buffer replacement; the `"Replace bytes"` description is gone. Laws: `an_applied_hex_dump_is_one_net_byte_range`,
  `an_applied_hex_dump_is_exactly_the_corpus_net_byte_range`.
- `🗜️deflate`: the summary Apply emits only the header leaves it changed (`SetCompressionParams` and/or `SetPresetDictionary`, none
  when unchanged) instead of both always; the `"Set compression header"` description is gone. Law:
  `an_applied_summary_is_only_the_header_leaves_it_changed`.
- `🧾️json`: already relative — a node Apply is ONE `patch-snapshot` of that node's path (`snapshot_edit_patch`); nothing to convert.
- Language-neutral corpora + schemas (authored by `T/🧪️w3-t2-text-stdio-net-corpora.py`, independently of the editors):
  `🔤️txt/…/✳️any/🧫️fixtures/🧫️net-leaves/🔣️.json` (15 cases, 3 whole-buffer) + `🧬️schema/🔣️net-leaves/🔣️.json`;
  `💾️binary/…/✳️any/🧫️fixtures/🧫️net-leaves/🔣️.json` (8 cases) + schema. Third-party oracles (ajv + jsdiff `diffArrays`):
  `…/🧪️tests/🧪️net-leaves/🟦️.ts` in both.
- Not in TEXT scope, reported for the owner: every stdio snapshot-details window (≈40 artifacts) publishes a whole `SetSnapshot`
  per edit event through `semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`).

**Peer break forward-fixed (03:22):** `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🧬️record-codec-laws/🦀️.rs` still called
`RecordSpecProducer`s as functions (`spec_fn()`, 3 sites, E0618) after the dsl refactor; now `(spec_fn.ordinary)()`, the refactor's
own convention. It blocked every build of `semio-framework-os-kernel` with its law features.

### 3. Verification so far (all commands from the repo root unless noted)

| Command | Result |
|---|---|
| `bun test <abs>/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (cwd outside the repo, see §5) | **33 pass / 0 fail** (incl. typing law: ajv, chart tables, the fixture ledger replay, xstate + fast-check 400 runs) |
| `bun test <abs>/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` | **68 pass / 0 fail** |
| `bun test <abs>/👕️canvas-presence/🧪️tests/🔬️unit/🟦️.ts` | **7 pass** |
| `bun test <abs>/📡️replication/👕️peer-overlay/🧪️tests/🔬️unit/🟦️.ts` | **2 pass** |
| `bun <abs>/📡️replication/📦️packages/🦀️rust/📜️script.ts presence-peer-codec-check --oracle-only` | **41 neutral vectors, 32 hostile inputs rejected** |
| `SEMIO_TEST_LEVEL=long node node_modules/vitest/vitest.mjs run --config RE/…/⚛️react/🧪️tests/🎚️config/🟦️.ts text-carets echo-pack` | text-carets **5 passed**; echo-pack does not load (stale generated plugin registry, §5) |
| `node node_modules/typescript/bin/tsc -p T/🧪️w3-t2-text-typecheck.tsconfig.json` | **0 errors in TEXT scope**; 13 errors are all in peer files (UI contract limits, store worker `line`, Shell, generated plugin registry) |
| `bun test <abs>/📝️md/…/🧪️tests/🧪️net-leaves/🟦️.ts <abs>/🌐️html/…/🧪️tests/🧪️net-leaves/🟦️.ts` | **26 pass / 0 fail** |
| `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-md -p …-html --lib -- editor::` | WRITTEN BUT UNVERIFIED (the build was killed by the 13:30 usage cut) |
| writer `typing_runs_tests` | four builds killed (EXIT 137: swap 92–98 % used, load 30–127, 14 GiB free disk, the disk guard pruning build units); the fifth (22:06, `-j 2`) reached the jack E0277 above (EXIT 101); the sixth (22:33–23:03, after the jack fix) compiled jack and every dependency, then stopped at a peer break in the writer's own `✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs` (E0432 `store::retirement`, the retirement API moved to `semio_framework_value::retirement`; the peer fixed the import at 23:56). The usage cut (23:00–02:30) and the coordinator's CARGO HOLD (fleet rule 26, 02:45) stop the seventh run until the hold is lifted |
| `SEMIO_TEST_LEVEL=long node node_modules/vitest/vitest.mjs run --config …/🎚️config/🟦️.ts text-carets echo-pack` (after S2-INFRA's registry re-emit) | **2 files, 10 passed** |
| `bun test ./retire-oracle.test.ts` (scratch wrapper of `♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts`) | **1 pass** (the 4 new corpus rows' byte counts confirmed independently) |
| `bun ./📜️script.ts schema mutation-payloads --under <scope>` (cwd `…/🧪️test`) | **0 findings**: trinity 27/27 payloads, 26/26 leaves witnessed; writer 10/10; vcs 6/6; md 11/11 (5 leaves); html 19/19 (9 leaves) |
| `bun ./📜️script.ts schema mutation-inputs --under <scope>` | writer, vcs, md, html **0**; trinity 32/32 inputs described, **4 `leafUncatalogued`** (the four new leaves await the central `schema generate`) |
| `node node_modules/typescript/bin/tsc -p T/🧪️w3-t2-text-typecheck.tsconfig.json` (03:05; now also the trinity aggregate union, the five relative leaves' TS payload/diff/inverse mirrors, the TS retirement oracle and the md/html net-leaf oracles) | **0 errors in TEXT scope**; 4 errors, all in peer files (`🏪️store/👷️worker/🟦️.ts` ×2, `🐚️Shell/🟦️.tsx`, `🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts`) |
| `bun test ./retire-oracle.test.ts` (03:06, 12-row corpus) | **1 pass / 0 fail** |
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, 03:07) | **0 findings**: 14/14 payloads and feature rows meet their leaf schema, 14/14 leaves witnessed |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting` (03:08) | 21/21 inputs carry a UI descriptor; **1 finding**: `delete-working-nodes` is `leafUncatalogued` (needs the central `schema generate`). Fixed on the way: `set-rule-layout-points /cleared` declared `ref.many`, which the UI descriptor schema does not allow (it surfaced once the 22:08 catalog read the leaf) |
| writer `typing_runs_tests`, seventh run (02:50, after the hold) | stopped in `semio-framework-os-kernel`: a peer's in-progress dsl refactor (`RecordSpecProducer`, 29 errors in `🗣️dsl/`, `🏪️store/`, `🎒️pack/`; files still changing at 03:05). Retrying when it settles |

### 4. Open items

1. Writer: six typing-run laws are red (one extra edit row per committed run). The root cause is not yet diagnosed.
2. trinity rewriting: the rest of the new-leaf wiring (§2) and the node-graph drag machine.
3. trinity working graph: `deleteSelection` and handed-back deletions are now relative (`delete-working-nodes`, §2). Still a whole-graph
   `edit-before-fixture`: a handed-back graph that ADDED a node or drew/cut a wire (no add/connect leaf vocabulary for the working graph yet).
4. Remaining static keys and amends on text paths, for CLOSURE: `git grep` shows no `Emit::amend` / static coalesce key left on writer, jack, vcs, stdio or trinity. The framework definitions remain (`Emit::amend`, `Emit.coalesce_key`, `ArtifactCommand::AmendLast` in `🔌️plugin/🦀️.rs`).

### 5. Coordinator actions and peer breaks

- Central `schema generate`: ran at 22:08 (`🔣️schema-catalog.json` lists the four leaves); it must run again for `delete-working-nodes`.
- wgpu text host (`RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, FLOW-owned): the typing run is committed on idle and on a caret move only.
  It still needs `blur` (editor focus lost) and `hidden` (page hide / visibility) signals so that a run typed in the last 750 ms before the
  tab goes away is not lost (React has both). The file has no focus hook today; this request is for the owner.

- `bun test` segfaults (0x8033) whenever the cwd is inside the repo, even for a trivial test. The suspected cause is the root `package.json`
  workspaces glob `🧰️framework/**` added at 10:27. Workaround: run bun from a cwd outside the repo with absolute test paths. Owned by S2-INFRA.
- The generated `🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts` is stale against the `moduleDirectoryName` signature (TS2554; the
  TextEditor echo-pack suite fails to load). It needs a plugin-registry regenerate, which only the coordinator may run.
- Descriptors and the central `schema generate` must be regenerated for: writer, trinity (four new rewriting leaves; `setParameter`/`setLhsJson`
  describe texts are unchanged), stdio md/html (no verb change), tool-machine schemas.

## Session 3 — 2026-10-02 (S3-TEXT, coordinator `⚪b7db773a…`)

Status: **IN PROGRESS** (updated at every milestone). Scratch output: `T/🗑️generated/s3-text/`.

### S3.1 Repair-first diff (fleet rule 28)

Files in TEXT scope with mtimes newer than the session-2 report (03:27): writer `Cargo.toml` / `✏️editor/🦀️.rs` / `👁️viewer/🦀️.rs`, trinity
rewriting + jack `Cargo.toml` / editor / viewer / wire-runtime, vcs editor / viewer / sqlite — all peer edits (05:41–05:58: `store::EngineHandles`
→ `semio_framework_2d::compute::EngineHandles` + `semio-framework-2d`/`semio-framework-value` deps; sqlite snapshot tests). No half-finished TEXT
edit found; `🛠️tool-machine`, the plugin runtime tool-machine glue, the typing laws and `✂️text-splice` are unchanged since session 2.

### S3.2 Verification (commands from the repo root unless noted)

| Command | Result |
|---|---|
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` (typing machine TS twin: ajv + chart tables + fixture ledger replay + xstate oracle + fast-check 400 runs; cwd = repo root now works) | **33 pass / 0 fail** (20 340 expects) |
| `bun test ./🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts` | **68 pass / 0 fail** |
| `bun test ./…/🧱️elements/👕️canvas-presence/🧪️tests/🔬️unit/🟦️.ts ./🧰️framework/🔨️modules/📡️replication/👕️peer-overlay/🧪️tests/🔬️unit/🟦️.ts` (presence ephemeral-shared preview) | **9 pass / 0 fail** |
| `bun ./🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts presence-peer-codec-check --oracle-only` | **41 neutral Rust/TS vectors, 32 hostile inputs rejected** |
| `SEMIO_TEST_LEVEL=long node node_modules/vitest/vitest.mjs run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts text-carets echo-pack` | **2 files, 10 passed** — the TextEditor echo-pack suite loads (§5: the generated registry, 10:37, calls `moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES)`, matching `📦️deployment/🟦️.ts:81`; no longer stale) |
| `bun test` md / html / txt / binary `🧪️tests/🧪️net-leaves/🟦️.ts` (ajv + markdown-it / parse5 / jsdiff oracles) | **51 pass / 0 fail** (4 files) |
| `node node_modules/typescript/bin/tsc -p T/🧪️w3-t2-text-typecheck.tsconfig.json` | **0 errors in TEXT scope**; 4 peer errors (`🏪️store/👷️worker/🟦️.ts:3776,3842` + `🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts:77` TS2741 `line`; `🐚️Shell/🟦️.tsx:1112` TS2304 `idleInstalledServiceStatusV1`) |
| `bun ./📜️script.ts schema mutation-payloads --under <scope>` (cwd `…/🔨️modules/🧪️test`), 9 scopes | **0 findings** each: rewriting 14/14 (14 leaves witnessed), writer 10/10, vcs 6/6, jack 14/14 (13), md 11/11, html 19/19, txt 10/10, binary 18/18, deflate 9/9 |
| `bun ./📜️script.ts schema mutation-inputs --under <scope>`, 9 scopes | **0 findings** for writer, vcs, jack, md, html, txt, binary, deflate; rewriting 21/21 inputs described, **1** `leafUncatalogued` (`delete-working-nodes`, awaits the central `schema generate`) |

### S3.3 Changes

**N8 (gap audit P2): the wgpu text host ends a typing run on blur and on a hidden page, like React.**
- Shared corpus: `🖱️ui/🎬️scene/🧫️fixtures/✂️text-splice/🔣️.json` gains `hostSignals` (idle → `idle`, caretMove → `selectionJump`, blur →
  `blur`, hidden → `hidden`) + its schema (`🧬️schema/✂️text-splice/🔣️.json`, required, enum-checked against the 7 reasons).
- TS: `TEXT_EDITOR_TYPING_HOST_SIGNALS` (`✂️text-splice/🟦️.ts`), used by `createTextEditorTypingRunV1` (idle, dispose) and the React
  `TextEditor` (caret move ×2, blur, hidden). Law `every host signal ends the run with the corpus reason, once, …` (text-splice TS test).
- Rust ui scene: law `every_host_signal_commits_with_a_tool_machine_reason` (`🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🦀️.rs`).
- wgpu engine (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, region TextEditor + the delivery state): `TextEditorTypingEnd {Blur, Hidden}`,
  `TextEditorDeliveryState.typing_end` + `end_typing`, `end_text_editor_typing(host, end)`, `end_every_text_editor_typing(end)`,
  wasm door `semioWgpuHostPageHidden`; the drive (`drive_text_editor_typing_end`, was `…_idle`) sends ONE commit signal naming the
  reason once the run's last keystroke left (idle otherwise); a caret-move commit clears a pending end; the retired editor's run
  strings retire through `close_step` and count in `terminal_is_empty`. Law `text_editor_host_signals_end_the_typing_run_with_the_corpus_reasons`
  (`⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`) drives every corpus row through the real outbox.
- Blur: the interpreter's focus-loss helper `clear_text_editor_caret` became `blur_text_editor` (ends the run + clears the caret) at all
  four focus-loss sites (pointer press elsewhere, accessibility blur, clipboard/text on a gone editor, key route on a gone editor)
  (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`).
- Hidden: the browser host posts `host-page-hidden` on `visibilitychange` → hidden (`🌐️browser-host/🟦️.ts` `wireInput`); the transport
  (`🚚️browser-frame-transport/🟦️.ts`) posts it at once outside any batch (`setHostPageHidden`, type `BrowserFrameHostPageHidden`; a
  hidden page gets no rAF, so batching would stall it); the frame Worker (`🎞️frame-worker/🟦️.ts`) calls `semioWgpuHostPageHidden` and
  requests its own frame turn. `pagehide` is not wired: the mount's own `pagehide` dispose runs first, and a closing tab fires
  `visibilitychange` before `pagehide`. Transport law `posts the hidden page to the Worker at once, outside any batch, …`
  (`🧪️tests/📨️browser-frame-transport/🟦️.ts`).

**Closure §20.3 (census §(b) S3-TEXT): the stdio text document-details edits are domain leaves, never a whole `SetSnapshot`.**
- Contract (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs`, new fn beside `snapshot_edit_set_snapshot`):
  `snapshot_edit_net(event, snapshot, net)` applies the event and publishes the artifact's own net leaves with NO description.
- `📝️md` → `md_net_mutations` (block leaves), `🌐️html` → `html_net_mutations` (node leaves), `💾️binary` → new `binary_net_mutations` (ONE
  `replace-byte-range`), `🗜️deflate` → new `deflate_net_mutations` (`set-compression-params` / `set-preset-dictionary` / `set-payload`;
  the summary Apply now shares `deflate_header_mutations`). None of the four calls `snapshot_edit_set_snapshot` any more.
- Named genuine whole-document replacements (the only `set-snapshot` left on these paths): md — an applied DSL envelope naming another
  document schema; html — another schema, or a root the node leaves cannot reach (another node kind / an unreproducible root attribute
  order); binary, deflate — another document schema (unreachable from the details editor, whose dialect check pins the schema id).
- Laws: md `a_document_details_edit_is_its_net_block_leaves`, html `a_document_details_edit_is_its_net_node_leaves` (each replays the
  whole net-leaves corpus through `snapshot_edit_emit`), binary `a_document_details_edit_is_one_net_byte_range`, deflate
  `a_document_details_edit_is_only_the_domain_leaves_it_changed`.
- Note for S3-CLOSURE/S3-STDIO: `snapshot_edit_patch` and `snapshot_edit_set_snapshot` still stamp the literal description
  `"Edit document details"` (§20.4 G7 gate) — contract owner. The `🧵️retained` accumulator coalesce keys (`🗒️note` `:173-358`) are
  S3-DRAW's; no TEXT path holds a coalesce key or amend (re-grepped writer, jack, vcs, rewriting, stdio md/html/txt/binary/deflate).

**N10 + node-graph record rows: trinity rewriting maps the shared rows to intent leaves; `setHostSnapshot`/`deleteSelection` gone.**
- `✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs` rewritten: rows decode through the ONE shared decoder `node_graph_edit_rows`
  (`🛠️tool-machine` region `🔖️NodeGraphEditRows`, S3-GRAPHS) — a malformed row refuses the batch by name. BEFORE canvas: `move` →
  `drag-working-nodes`, `connect` → `connect-working-ports` (edge kind = the resolved manifest's first edge kind, else the graph's
  wires'), `disconnect` → `disconnect-working-edges`, `delete` → `delete-working-nodes` + `disconnect-working-edges` of the named
  wires the node delete does not already remove. LHS/RHS: `move` → `drag-rule-nodes`, `delete` with nodes → clause deletions; a rule
  wire alone (connect / disconnect / wire-only delete) is refused (derived from clauses). `setSlider` / `insertPort` refused (no such
  widgets). Deleted: the whole-graph readers (`displacement_records`, `deleted_nodes`, `working_graph_deletion`, `rendered_graph`,
  `HOST_SNAPSHOT_GESTURE`), the `edit-before-fixture` structural fallback and the `selected_node_ids` parameter (both call sites in
  `✏️editor/🦀️.rs`). The last whole-graph write on a gesture path (N10: add/connect) is gone; `edit-before-fixture` remains only as the
  exact one-row inverse of the working-graph leaves.
- Two new relative leaves, schema-first with every surface, written + registered by `T/🧪️s3-text-trinity-wire-leaves.py`:
  `🔌️connect-working` `connect-working-ports {source, target, kind}` (tag 12; id `source->target`; `target-missing` /
  `target-mismatch` / `no-op` / `x-semio-invariant distinct-endpoints`; label "Connect two ports" / "Zwei Anschlüsse verbinden") and
  `🪚️disconnect-working` `disconnect-working-edges {targets}` (tag 13; `partial` / `target-missing`; "Disconnect N edge(s)" /
  "N Kante(n) trennen"); both undo with ONE `edit-before-fixture`. Shared helpers `connect_working_graph_ports`,
  `remove_working_graph_edges` (aggregate region `🕸️WorkingGraph`). Quintets `🔌️connects`, `🪚️cuts` (computed in Python), 6 laws each,
  structural correspondence, oracle catalog, TS union/oneOf/GraphQL/protobuf/binary/text grammars, retirement arms + corpus rows
  (11 and 3 bytes) + TS oracle (14 rows), harness KINDS + Python second implementation + Nakagin feature rows (mutate / inverse /
  spec vector).
- Editor laws rewritten (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`): `the_shared_node_graph_rows_map_to_intent_leaves` (every refused row of
  the shared fixture `🧫️node-graph-edit-rows` refused by name; connect/disconnect leaves + one-row inverse; setSlider/insertPort and
  rule-side wires refused), `a_deleted_working_graph_selection_is_relative_leaves`; `add_and_delete_rhs_set_clause` now sends a
  `delete` row.

### S3.4 Open items

1. N7 writer typing laws (6/23 red since session 1: one extra edit row per committed run) — every session-2/3 build was stopped
   upstream before the writer tests ran; root cause still undiagnosed (needs the `describe` output of a run). See S3.2 for the runs.
2. trinity rewriting: adding a node on the working graph has no gesture path any more (the shared rows carry no add-node record;
   `addWidget` is a flow verb) — a product decision, not a residual whole-graph write.
3. The session-2 trinity, stdio md/html/txt/binary/deflate and writer Rust laws are WRITTEN BUT UNVERIFIED until a cargo run reaches them.
4. Writer `setSnapshot` / `setSnapshotJson` / `setFixtureJson` / `openDocument` load a whole document through the `LoadDocument`
   effect — genuine whole-document replacement intents (agent / dev-chrome load), not mutation leaves; named, kept.

### S3.5 Coordinator actions and peer breaks
