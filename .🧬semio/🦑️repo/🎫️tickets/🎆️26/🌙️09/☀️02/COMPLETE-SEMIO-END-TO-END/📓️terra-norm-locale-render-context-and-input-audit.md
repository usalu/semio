# Norm Locale Render Context And Inputs Audit

**Scope:** read-only current-source audit on 2026-09-04. No build or runtime command was started.

## Verdict

**RED.** The outer plugin runtime receives a host `ViewModel`, but the static authoring contracts
discard it before every synchronous `ArtifactEditor` and `ArtifactViewer` root. Therefore none of
the 30 Norm surfaces can select a host locale or terminology in its dynamic body. The newly present
30-wrapper/90-body test is useful inventory coverage, but it uses `ViewModel::default()`; it cannot
prove explicit EN/DE selection or no-default-language admission.

**RED, independently:** every Norm Inputs body is pretty-printed JSON through a plain text component.
It does not permit real document editing. The correct first editing packet is a finite,
descriptor-owned semantic mutation command using existing `ActionArgDef` staging, not editable JSON,
a JSON Patch endpoint, or a raw `setSnapshot` text buffer.

## Current source evidence

| Boundary | Evidence | Meaning |
| --- | --- | --- |
| Host receives axes | `PluginApp::render(..., &ViewModel)` and `VcsArtifactApp::render` accept the model ([plugin:11792](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11792), [plugin:24577](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24577)). | The history body can use `view_state.locale`, but app bodies cannot. |
| Drop point | `ArtifactApp::render` / request-context render omit a host-render argument ([plugin:11306](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11306)); sync `ArtifactEditor` and `ArtifactViewer` do too ([plugin:26140](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26140), [plugin:26329](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26329)). | Both the editor and viewer adapters necessarily lose locale/terminology. |
| Adapter path | The VCS wrapper calls `A::render_with_request_context` with only owner/document/config/transient/interaction ([plugin:24605](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24605)); `EditorApp`/ `ViewerApp` forward exactly that reduced shape ([plugin:26641](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26641), [plugin:26807](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26807)). | A single framework packet can reach all 30 surfaces. |
| Existing labels | `LabelAxes`, `resolve_labels`, and exhaustive four-cell `app_labels!` already model `Locale × Terminology` ([plugin:6199](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6199), [plugin:6231](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6231)). | Reuse this; do not invent a per-plugin locale or a config-string bridge. |
| Default leak | `ViewModel` derives `Default`, and both axes deserialize with defaults ([manifest:4603](../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4603), [manifest:4637](../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4637)). The render wire accepts a nested or bare `ViewModel` ([plugin:30085](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30085)). | `{}` and `{ "viewState": {} }` can render with implicit axes; this is not no-default-language behavior. |
| Norm dynamic bodies | Shared helpers contain English body grammar, including `No checks computed.`, `catalogue`, `No checks`, and `Unknown body` ([app-surface:65](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:65)). | Descriptor/window labels have EN/DE pairs; dynamic rendered content does not. |
| Current wrapper test | The test enumerates 30 rows and 90 real body keys ([surface-test:61](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:61)), but uses `ViewModel::default()` ([surface-test:45](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:45)) and checks only typed tree presence, known-body fallback, and large-key failure ([surface-test:47](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:47)). | It is non-vacuous topology coverage only, not a localization/accessibility/user-edit proof. |

`ArtifactView` is correctly document-bound: snapshot, history, child content, durable command and
render operation identities ([plugin:8033](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8033)).
Locale or terminology must **not** be added there, to `NormConfig`, a VCS event, presence, draft,
or a shared document. Those would incorrectly serialize one user's device UI choice into
collaboration/replay state.

## Smallest clean locale packet

### Framework/plugin owner

Add a borrowed, non-serializable, non-`Default` `ArtifactRenderContext<'a>` at the SDK render
boundary. It borrows the validated host model and exposes `view_model()`, `locale()`, and
`terminology()`; it implements `LabelAxes`. Construct it once in `VcsArtifactApp::render` and
thread `&ArtifactRenderContext` through:

1. `ArtifactApp::render`, `render_with_instance_operation_owner`, and
   `render_with_request_context`;
2. both synchronous author traits, `ArtifactEditor` and `ArtifactViewer`;
3. `EditorApp`/ `ViewerApp`; and
4. both snapshot-override and cached branches of `VcsArtifactApp::render`.

The context is a render-call capability, not an `ArtifactView` property. It remains host/user/device
local and disappears after render. Framework-owned History rendering reads this same context rather
than resolving an ambient locale.

At `plugin_render_with_document`, inspect raw JSON before deserializing the permissive
`ViewModel`: every accepted bare and nested render envelope must supply valid non-null
`locale` and `terminology`. Missing, malformed, or unknown axes fail before acquiring the app
instance. Do the same for WIT/refresh render envelopes. The generic `decode_view_state` fallback
may stay for non-render control paths, but must not authorize an app render.

### Norm owner

Declare one shared `NormRenderLabels` in
`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` with `app_labels!`. It supplies all four
Native/Reuse × EN/DE cells and is resolved from `ArtifactRenderContext`. Make the shared report,
summary, catalogue, inspection, unknown-body and window-body helpers take that context, then pass it
from every editor and viewer root. Standard identifiers, clauses, calculated values and persisted
document strings remain data; only UI grammar is translated.

## Inputs: current fact and correct editing reuse

All 15 Inputs windows call `render_document_json`; EN 1990 is representative
([inputs:17](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs:17)).
That helper serializes pretty JSON and constructs only `ui::text`
([app-surface:83](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:83)); it has no
`TextEditorScene`, editable `TextWindowKit`, input schema, on-change intent, or typed mutation
binding. It is display text, not a placeholder for an editor.

`FormPanelBuilder::from_dictionary` is also **not** the right schema authority. It iterates a loose
JSON dictionary and binds every free-text field to one `ActionId`
([plugin:6007](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6007),
[plugin:6048](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6048)).
It is a reusable layout builder, not a typed property-schema compiler and cannot safely infer a Norm
mutation from JSON field names.

The existing reusable path is instead:

1. the owning mutation leaf supplies a finite typed payload and its schema; for example EN 1990's
   `ChangePermanentAction { new_g_k: f64 }` is an explicit mutation leaf
   ([leaf](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-permanent-action/🦀️.rs:8));
2. the editor declares one same-named mutation action and
   `.action_args("changePermanentAction", vec![ActionArgDef::number("newGK", ...).required()])`;
   `Editor::builder.action_args` already attaches typed argument declarations
   ([plugin:5146](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5146));
3. the app command enum maps precisely that action to the leaf payload and emits that single semantic
   mutation. No raw `setSnapshot` route is used for this UI; existing `setSnapshot` is a distinct
   whole-document DSL parser ([EN1990 command](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42),
   [set-snapshot](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️set-snapshot/🦀️.rs:31)); and
4. React already resolves localized `ActionArgDef` labels and renders staged text/number/toggle/select
   controls ([shell helpers:2031](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️.tsx:2031),
   [shell helpers:2755](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️.tsx:2755)).
   WGPU already owns staged action arguments, required-argument admission, and execution
   ([WGPU shell:1738](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1738),
   [WGPU shell:7752](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7752)).

The first honest P0 is **one EN 1990 Inputs property card for permanent action `G_k`**, backed by
`ChangePermanentAction`, with an explicit localized number label/description and bounded domain
validation in the leaf/command. It proves renderer staged argument → typed command → one event-sourced
leaf mutation → rerender. It does not claim that all 15 artefacts have generic property editors.
Subsequent cards remain descriptor-owned finite commands per artifact; a generic JSON patch or a
schema reflection layer is deliberately outside this packet.

### P0 publication prerequisite (current-source correction)

The typed command/card alone is **not** an admitted editable surface. Current EN 1990 explicitly
classifies all three advertised actions as `BatchOnlyPendingRewrite`
([editor:178-182](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:178)); the comment correctly says that
selected-check has no retained Config preparation. More generally, the framework obtains each
app-owned one-item preparation factory at mount
([plugin:19133-19139](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19133)) and its
default `ArtifactApp::build_*_store_one_item_preparation_factory` returns `None`
([plugin:11055-11080](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11055)). A
`ChangePermanentAction` UI therefore remains source-only or batch-only unless its owning EN 1990
app supplies the exact retained artifact/config publication and preparation factory, with bounded
store ownership, before it is registered as interactive.

P0 must first name that owner/factory and prove accepted command → retained preparation → one
published leaf event → rerender, including rejected/stale/cancelled turns with no publication. Do
not advertise an ActionArgDef, a parsed handler, or a batch result as actual document editing until
that retained path exists and a process/renderer proof has exercised it.

## Required acceptance and owners

| Owner | Required law |
| --- | --- |
| Framework/plugin | Bare and nested render requests with missing/invalid axes reject before app render. Explicit EN and DE contexts reach one editor and one viewer. The context neither serializes nor changes a snapshot/config/event hash. |
| Norm | Amend the 30-wrapper/90-body test to render every declared body with explicit EN and DE models, not `default`; assert selected dynamic labels and all four label-axis cells. Keep the existing independent inventory oracle. |
| Renderer/accessibility | React and WGPU preserve the localized semantic surface name and today’s read-only Inputs semantics. For P0, both stage a labelled numeric action field, expose an accessible label/description and required/error state, prevent invalid submission, and send exactly the declared command/arguments. Renderers do not own Norm literals. |
| Norm P0 mutation | Valid `G_k` changes only the declared leaf/diff and rerenders; wrong type, missing value, out-of-domain value, stale target or unauthorized action emits no mutation. A neutral JSON fixture declares valid and hostile action arguments and expected mutation/no-mutation result. |

The registered Norm source/test routes are
`bun ./✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts surface-render-source` and
`bun ./✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts surface-render-test`
([script:127](../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:127),
[script:157](../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:157)).
They were not run. A P0 implementation must register an exact typed-action runtime proof in the same
script/launch order before any claim of editable Norm Inputs.

## Nonclaims

This report does not claim a green build, renderer runtime proof, complete legal-content
localization, or actual editing of any Norm document. It records current byte-level boundaries and
the smallest ownership-correct handoff.
