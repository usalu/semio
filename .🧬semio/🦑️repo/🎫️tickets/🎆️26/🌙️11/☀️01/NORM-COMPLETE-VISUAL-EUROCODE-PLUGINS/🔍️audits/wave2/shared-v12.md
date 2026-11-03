# Shared v12 (wave 2): partial, stopped by the en1994 freeze

Nothing compiled. `freeze.txt` still names build-en1994 (graph-plan since 03:24, runner log shows no clear), so after the coordinator's message I stopped editing shared crates. "- ready for rebuild: shared v12" is NOT appended to integrate/latest.md yet; append it once the freeze clears and tasks 2-4 are applied (or as is for task 1).
Note: the task 1 edits below were made before the freeze message reached me; they touch framework and the norm app-surface (en1994 depends on both), so they may restart its graph-plan.

## 1. Done: Ctrl+A through the framework `selectAll` verb
Framework (`🧰️framework`):
- `ShellKeyOutcome::Verb { action, args: BTreeMap<String,String> }` + `ShellKeyOutcome::verb(..)` (`🔨️modules/🛂️manifest/🧱️app-surface/📱️shell-hooks`, rs + proto `ShellVerb` + ts twin); wire test extended.
- wgpu shell keyboard (`...⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs`): `Verb` dispatches the action with the args as an `ActionDescriptor` (same as a palette entry).
- `selectAll` verb (`🔌️plugin/🧵️instance/🖼️render/📥️staging/🧭️interaction/🦀️.rs`): optional text arg `granularities` (comma list). Absent/undeclared: unchanged (active granularity). Declared: the topology nodes of all listed kinds, in topology order, selected together (selection granularity = first node's). Other apps unaffected. Test `select_all_takes_an_optional_granularity_scope_and_ignores_undeclared_ids` (single-granularity fixture only: fallback and declared scope; a multi-kind fixture is not written).
Norm (`📇️registry/🧬️contract/🖥️app-surface`):
- `host-routes`: in-process path removed (`publish_selectable_scene`, `select_all_targets`, `Selectable.entities`, toolbar call). `select_all_outcome` now returns `ShellKeyOutcome::verb("selectAll", select_all_args())` (`Ignored` without a domain). New `select_all_granularities` (the 9 entity kinds, no check/loadCase), `select_all_args`, `visible_rows`.
- Outliner filter: `domain_topology(.., visible)` drops filtered entities and rows (checks stay); `norm_interaction_topology` reads `visible_rows(domain)`. Side effect: a filter also prunes hidden entities from an existing selection.
- Touch menu Select all passes the same args (about 60 bytes).
- Tests: topology filter test, touch menu with scope, scope names all kinds, Ctrl+A verb, filter publication, no-domain Ignored; old chord/touch tests replaced.

## 2. Deferred: `setSelectedCheckIndex` lane
Finding: the `ReplayShellCommand{interactionSelect}` effect is an inline interaction verb; the typed ladder peels it from `emit.effects` (`take_inline_interaction_verbs`) and folds it with no publication-lane check, so the job does NOT refuse it today. Lane checks cover only store lanes (`typed-operation-unit` line ~215: `interaction_writes` need `ArtifactToolPublicationLane::Interaction`). To declare it honestly, apply after the freeze: add `Interaction` to `setSelectedCheckIndex` in `NORM_PUBLICATION_CONTRACTS` and `NORM_EDITOR_PUBLICATION_CONTRACTS` (`🧵️retained-commands`), and update the fixture `🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json` (publicationContracts[0], routes[0].emittedLanes and publicationLanes: `["window-config","interaction"]`) and `expected_lanes[0]` in `🔬️retained-disposition-oracle`. Test: the existing live-acceptance check-row-click test already exercises the effect; add an assertion that the contract lists `interaction`.

## 3. Deferred: "Show all checks" toggle
Plan: `ChecksViewState` gains `show_all: bool` (wire form too); `owners` narrowing is skipped when set; new shortcut `ShortcutAction::ToggleAllChecks` (scope Viewport/Global, chord `Alt+A`, EN "Show all checks" / DE "Alle Nachweise anzeigen") routed as `HostRoute::Local`; a checkbox/toggle row in the checks window with an a11y node (role `switch`, `checked`); state is persisted-local view state (WindowConfig). Test: with a pick, narrowed rows; after toggle, full list; toggling back narrows. Touches `🪄️selection-mirror`, `✅️checks`, `focus-order`, ten editors only if the state is wired through `render_checks` (it is not: `LiveView` owns it), so no artifact edits.

## 4. Triage (requests.md)
- 585 `Outcome.flags`: needs the xi rows consumed by compliance checks of each artifact; shared part is the `Outcome.flags` field in `norm_compliance` (twins rs/ts/proto/graphql) plus the report row rendering; not applied (shared crate, frozen). Stays `[open]` with this plan.
- 1527 `beams[].steel.rM`: en1994 schema leaf (artifact-agent scope: field adder/codegen), nothing shared remains; stays `[open]` for the en1994 agent.
