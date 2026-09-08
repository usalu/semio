# Trinity Jack and Rewriting State-Ownership Audit

## Scope and method

This is a read-only audit of `✏️s/🔌️plugins/🔱️trinity`, with the framework OS shell inspected only to identify the existing global locale owner. It covers Jack and Rewriting editor commands, config, presence, transient state, public schemas, and their consumers. It does not recommend changing the graph-domain snapshot camera without a separate graph-domain decision.

The governing ownership rule used here is:

| Lifetime and audience | Owner |
| --- | --- |
| One shell's UI language | Framework OS shell preferences (`uiLocale`) |
| User-local durable preference for one app window | That concrete window's config |
| User-local short-lived UI/result/request state | That concrete window's transient state |
| Live state deliberately shown to collaborators | Presence, with an explicit producer and renderer consumer |
| Graph content and reproducible graph layout | The Trinity document snapshot and document mutations |

## Confirmed facts that constrain the fix

1. The existing OS owner is complete: `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🦀️.rs` reduces `ShellCommand::SetUiLocale` into `ShellState.ui_locale`; its TypeScript twin maps it to capability `os.setLocale` and writes `uiLocale`. `ShellHost` sends that command from Settings, while the UI package persists the shell key `ui.chrome.locale` and initializes locale before the first render.
2. `JackSnapshot` and `RewritingSnapshot` do **not** persist editor config. `JackArtifact::to_snapshot()` only copies graph fields (`schema`, `name`, manifest, graph camera/content/root); `RewritingArtifact::to_snapshot()` only copies the rule fields. These snapshot boundaries are correct and must remain the discriminator between graph data and UI state.
3. The public aggregate schemas nevertheless leak UI fields. `JackArtifact`/`JackDiff` enumerate config and presence fields alongside artifact fields in `.../🔌️jack/.../🧬️schema/🦀️.rs` and `.../🔺️diff/🦀️.rs`; Rewriting does the same. Their JSON Schema, GraphQL, Proto, TypeScript, text diff, and mutation-diff fixtures mirror this. The app-level config descriptor already exists separately at `✏️editor/🎚️config/🧬️schema/🦀️.rs`, so this is redundant public aggregate-contract exposure, not snapshot persistence.
4. Both editors install `NoTransient` even though they contain short-lived local UI state. Jack and Rewriting also declare a presence type but do not override `ArtifactEditor::ephemeral`; render paths use `ConfigView`, not `PresenceView`.

## P0: duplicated state has contradictory ownership

### Jack

`JackConfig` owns `active_fixture_id`, `jack_query`, `camera`, and `lod_mode_by_window` in `.../✏️editor/🎚️config/🦀️.rs`. `JackPresence` declares the same four values as shareable state in `.../✏️editor/👥️presence/🦀️.rs`.

The live app uses Config only:

- graph rendering reads `cfg.camera` and `cfg.lod_mode_by_window` in `.../🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs`;
- query editor rendering reads `cfg.jack_query` and `cfg.editor_selection` in `.../🪟️windows/📝️editor/🦀️.rs`;
- results rendering reads `cfg.jack_result_json` in `.../🪟️windows/📊️results/🦀️.rs`;
- the catalogue reads `cfg.active_fixture_id` in `.../📌️panels/📚️catalogue/🦀️.rs`.

There is no `ephemeral()` override in `TrinityJackPlayApp`; no command emits a `JackPresenceMutation`; no renderer reads a `JackPresence`. The four-field presence schema and its mutation collection are therefore dead duplicate surface, despite advertising collaboration semantics.

### Rewriting

`RewritingConfig` and `RewritingPresence` duplicate `before_pane_camera` and `lod_mode_by_window`. The Before window reads only `cfg.before_pane_camera`; render and window-measure helpers read only `cfg.lod_mode_by_window`. As with Jack, there is no `ephemeral()` producer and no render consumer for `RewritingPresence`.

### Decision

Choose **local window ownership**, not presence, for these view values. Camera and LOD are each user's view of a surface; synchronizing them would make collaborators fight each other's viewport and performance mode. Delete the unused presence duplicates and move the retained values to the owning window configurations:

| Current value | Correct owner | Lifetime |
| --- | --- | --- |
| Jack `camera` | Edit/Graph window config | persisted local-only |
| Jack `lod_mode_by_window[graph]` | Edit/Graph window config | persisted local-only |
| Rewriting `before_pane_camera` | Edit/Before window config | persisted local-only |
| Rewriting `lod_mode_by_window[...]` | each concrete Rewriting graph window config | persisted local-only |

Use named per-window fields rather than a map keyed by arbitrary `window_id`. The declared fixed window set is known at compile time, so an unbounded map accepts invalid/nonexistent window IDs and hides each field's actual owner.

Do not remove `JackSnapshot.camera` or the camera embedded in a Rewriting before-fixture as part of this work. Those are graph/document seed or layout data. The bug is the live viewport copy and its duplicate presence declaration, not the camera type or every camera in the snapshot.

## P0: locale is a shell preference, not a Trinity mutation

Both editors implement the complete app-local locale stack:

- `locale: String` in each Config and default `"en-US"`;
- `SetLocale` config mutation and `set-locale` wire/schema leaves;
- `SetLocale` command enum case and `commands/set-locale` emitter;
- app render resolves labels from `cfg.snapshot.locale` / `config.locale`;
- context-menu German selection checks the same app-local value;
- aggregate artifact and diff schemas expose locale as config;
- contract fixtures and config unit tests assert the locale default/mutation.

Jack locations include `.../🔌️jack/.../✏️editor/🎚️config`, `.../🧬️mutations/🗣️set-locale`, `.../🎮️commands/🗣️set-locale`, and `.../✏️editor/🦀️.rs`. Rewriting has the corresponding paths under `.../♻️rewriting`.

Delete these app-local locale surfaces. Route Settings to the already-existing OS command `os.setLocale`, and make the shell locale available to plugin rendering and context-menu construction as a typed host read model. `ArtifactEditor::render` and `context_menu` currently receive only document/config state in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`; this missing explicit host input is why Trinity used app config as a substitute. The new input must feed both label resolution and the `is_de` context-menu branch. It should be passed through `render_with_request_context` rather than recovered from any artifact or config store.

This change must retain per-shell scope: several Framework OS shells can coexist, so “OS-wide” means the shell preferences owner, not a process-global singleton.

## P1: Jack's remaining config is mixed across three lifetimes

| Current Jack member and command/mutation | Evidence | Decision |
| --- | --- | --- |
| `active_fixture_id` / `SetActiveFixture` / `setActiveExample` | The command resets the document and then records a catalogue selection. Only the catalogue panel consumes the id. Presence duplicates it but is unused. | Delete it as document-app state. The reset effect is the authoritative outcome. If product needs the last highlighted catalogue row, own it as local transient for the catalogue panel; never in shared presence or document config. |
| `jack_query` / `SetQuery`, `textEdit`, `formatDocument` | Editor window is the sole display consumer. `runQuery` and formatting use it as the editable source. | Move to Edit/Editor window config: persisted local-only, scoped by document and user. Keep a typed query-edit command/mutation at that window leaf. |
| `editor_selection` / `SetEditorSelection`, `textSelect` | Only the Jack TextEditor renderer consumes it. | Move to Edit/Editor window transient. A caret/range is local live UI state and must not produce durable config events. |
| `jack_result_json` / `SetResult`, `runQuery`, `loadExampleQuery` | Only Results window consumes it. It is a derivation of a query execution, not document content. | Move to Edit/Results window transient. The run command may emit document mutations plus a local transient result; it must not serialize a stale result as config. |
| `revision` / `SetRevision`, `requestCompletions` | One producer increments it; its only consumers are the two dispatch paths which pass the old value back to that producer. There is no renderer, LSP request, or completion consumer. | Delete the field, config mutation, retained Config job route, command, and contract vectors. Reintroduce only with a real cancellable LSP operation and a local editor transient request/result model. |
| `reorganize_epoch` / `SetReorganizeEpoch`, `reorganize` | The only use is to calculate and persist `epoch + 1`. It is never rendered or otherwise consumed. | Delete the epoch and its config mutation. Jack reorganize already returns actual graph mutations, which are the complete event-sourced outcome. |
| `editor_engagement_input`, `graph_engagement_input`, `results_engagement_input` and their three command/mutation families | Each has exactly one leaf producer; repository search found no render or behavior consumer outside its producer, command routing, schema, and fixtures. | Delete all three as dead state and delete their retained Config job publication contracts. If the framework later supplies genuine engagement input, it owns that local interaction state. |

The three dead engagement inputs are currently treated as Config in all of these places: `JackConfig`, its 3 mutations, command enum/dispatch, `JACK_RETAINED_CONFIG_TOOL_IDS`, publication contracts, first-step proof list, schema leaves, aggregate schema/diff leaves, contract fixtures, and generated language mirrors. Remove the entire vertical slice instead of leaving orphan command registration.

## P1: Rewriting contains two more non-state commands

| Current member/command | Evidence | Decision |
| --- | --- | --- |
| `before_pane_camera` / `SetBeforePaneCamera`, `setViewport` | The Before window alone reads it; the initial seed reads the embedded before-fixture camera exactly once. | Keep the seed behavior, but store live updates in the Edit/Before window's persisted local config. |
| `lod_mode_by_window` / `SetLodMode` | Render/window-measure reads it for the fixed Before, After, LHS, RHS, and Jack windows. Presence duplicates it unused. | Split into window-owned local preferences. |
| `reorganize_epoch` / `SetReorganizeEpoch`, `reorganize` | Rewriting `reorganize` only emits `SetReorganizeEpoch(epoch + 1)`; it does not mutate the rule or render any observed state. | Delete this no-op action, field, mutation, retained config surface, and tests rather than migrating it. A future arrangement feature must emit actual `RewriteRuleMutation` layout events. |

## P1: aggregate schema and generated-contract cleanup

After an owner move, remove every moved UI leaf from `JackArtifact`/`JackDiff` and `RewritingArtifact`/`RewritingDiff` rather than changing only the Rust struct. The affected contract families are:

- Rust, TypeScript, GraphQL, JSON Schema, Proto, and text-diff leaves under each `🧬️schema` and `🔺️diff`;
- config schema mirrors and `replace-config` mutation schemas;
- config mutation-contract fixtures/unit tests and aggregate mutation-diff fixtures;
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` entries for the deleted `app.trinity.*.mutation.set-locale` leaves;
- command enum routing, retained-job publication contracts, and first-step proof declarations.

The artifact-schema descriptor needs to describe artifact/snapshot/diff/mutation data only. `app_schema_descriptor()` remains the one owner of actual app configuration. The subsequent per-window config and transient descriptors must be registered at their own leaves, so a schema consumer sees one owner per value.

## Implementation order

1. Add typed shell UI context to the ArtifactEditor render and context-menu path, supplied from the existing shell `uiLocale`; add an OS-level integration test with a Jack and Rewriting editor.
2. Remove locale from both Trinity apps and all their command/config/mutation/aggregate-schema/test/catalogue surfaces; resolve labels and contextual menu language from that host context.
3. Delete unused Jack and Rewriting presence state and mutations. Remove all moved config fields from aggregate/diff public contracts; regenerate checked mirrors.
4. Introduce concrete per-window config records for camera/LOD/query and local transient records for selection/results. Convert each command to emit to the appropriate lane. Keep document mutations in the graph/rule lanes only.
5. Delete write-only request/engagement scaffolding (`revision`, both reorganize epochs, three Jack engagement fields, and Rewriting's no-op reorganize action).

## Required regression coverage

1. **OS locale propagation:** changing `os.setLocale` once updates Jack and Rewriting labels/context-menu language; neither app config schema nor command codec contains `locale` or `set-locale`.
2. **Two users/two windows:** distinct local Graph/Before cameras and LOD values survive for their owner windows without changing a collaborator's view or the Trinity document snapshot bytes.
3. **Transient isolation:** Jack text selection and query result are visible only in the local Editor/Results windows, produce no config/document event, and do not survive a fresh transient session.
4. **Document invariance:** viewport, LOD, selection, result, locale, and removed engagement updates cannot alter a `JackSnapshot` or `RewritingSnapshot`; graph/rule mutations still do.
5. **Dead-surface removal:** command/action and schema catalog enumeration no longer exposes `requestCompletions`, the three engagement-input actions, `setLocale`, or Rewriting's no-op `reorganize`.
6. Retain existing cross-language Rust/TypeScript schema and reducer parity checks, then add at least one language-neutral contract fixture for each retained per-window config and transient mutation.

## Risks to avoid

- Do not solve locale by moving `locale` to a shared app presence record. Locale is a shell preference and needs immediate cross-app propagation.
- Do not make camera/LOD shared just because currently unused presence records exist. Their intended local view semantics and fixed window ownership are clear from their render sites.
- Do not treat `JackArtifact`/`RewritingArtifact` aggregate schema fields as proof that UI data persists in the document. `to_snapshot()` proves otherwise; fix the aggregate contract without deleting graph snapshot fields.
- Do not keep empty compatibility variants or migration adapters. This greenfield repository can remove the obsolete schemas and command names in one change.
