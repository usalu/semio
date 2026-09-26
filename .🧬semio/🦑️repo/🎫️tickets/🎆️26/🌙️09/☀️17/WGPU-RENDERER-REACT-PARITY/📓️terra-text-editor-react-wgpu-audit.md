+# TextEditor React/WGPU Parity Audit

## Scope and method

This is a source-only, read-only audit on 2026-09-26. It compares the React `TextEditorHost` with the accepted-scene WGPU path, native `winit` ingress, browser-worker ingress, the shared accessibility projection, and authored text-editor command bridges. No test command or browser probe was run by this audit.

The keyboard navigation, primary-pointer blur, and action-wire correction landed while this audit was in progress. Their current source is recorded below as source-reviewed; this audit does not claim its test results. The remaining findings are separate from that completed source slice.

## Priority and ownership

| Priority | User-visible gap | Recommended owner / boundary |
| --- | --- | --- |
| Resolved in current source; receipt test still required | WGPU previously sent incompatible `textEdit`/`textSelect` argument names and reverse action order. It now writes the React fields and order from a pre-mutation reservation. | Keep a runtime precedence/receipt law at the current TextEditor input owner; do not fold asynchronous refusal/coalescing into this synchronous slice. |
| P1 | TextEditor wheel motion changes only the private WGPU host camera; React publishes `setCamera`, so the guest cannot persist or observe the scroll. | WGPU EngineCanvas text-editor scene interaction. This is a small, independent next slice. |
| P1 | Clipboard commands and committed text input do not route to focused TextEditor; copy/cut/paste are missing and browser/native IME commits go to generic input handling. | WGPU Interpreter focus/input boundary plus native ingress. Keep clipboard and IME as two implementations over one guarded committed-text helper. |
| P2 | A WGPU TextEditor is announced only as the focusable `application` Surface, not as React’s labelled read-only multiline textbox. The shared projection lacks multiline/readonly state. | Shared accessibility contract, browser mirror, and accepted scene projection. It needs a schema-first virtual scene child, not a chrome row. |

## Action contract: corrected in current source; asynchronous receipt remains open

React’s one latest-wins outbox delivers an edit before its matching selection:

```text
textEdit   { surfaceId, text }
textSelect { surfaceId, start, end }
```

The `textSelect` call occurs only after the edit action settles without a refusal. The direct evidence is:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx:227-253`
- React contract test `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:6552-6585`

The pre-correction WGPU helper emitted:

```text
textSelect { surfaceId, selectionJson }
textEdit   { surfaceId, document }
```

in that order. The current helper has replaced it with an all-or-nothing reservation followed by:

```text
textEdit   { surfaceId, text }
textSelect { surfaceId, start, end }
```

in that order:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5872-5900`

This is not merely a local representation difference. An authored bridge parses `textEdit` from `text`/ `value` and `textSelect` from `start`/ `end`; it does not read either WGPU field:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:221-224`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352-364`

That bridge persists `textEdit` as WindowConfig and publishes `textSelect` as WindowTransient, so the ordering is material:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:383-389`

### Remaining acceptance law

The direct WGPU law now asserts the fields/order, and the neutral `rendererKeys` fixture is consumed by both React and WGPU (`🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:244-320`; `🧰️framework/🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json`). It does not prove the production `AppRuntime` precedence. Its required companion regression is a focused accepted TextEditor receiving bare pressed Space through `AppRuntime::handle_key`, yielding exactly edit then select and leaving generic `space_pressed` unchanged; unfocused pressed/released Space must remain a generic hold with no editor action. A retained Select or open context menu must retain its existing Space priority.

A synchronous native batch cannot reproduce React’s awaited refusal and latest-wins queue by itself. It must nevertheless match action names, fields, and order; action refusal/coalescing is a separate transport concern.

## Scrolling: guest camera publication is absent

React handles every TextEditor wheel event by scrolling the session, rendering, and dispatching:

```text
setCamera { surfaceId, camera: JSON.parse(session.cameraJson()) }
```

See `🧱️elements/✏️TextEditor/🟦️.tsx:620-628`.

WGPU’s `text_editor_wheel_into` only mutates `EditorHost::wheel_scroll_screen` and returns a boolean. Its Scene intent caller discards any chance to publish an action:

- `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5830-5842`
- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2329-2339`

The Paint2d scene already shows the correct WGPU action-reservation pattern for `setCamera` at `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5814-5824`.

### Executable slice

Change TextEditor wheel handling to accept the input action queue, reserve one bounded `setCamera` descriptor before mutation, and publish `{ surfaceId, camera: { x, y, zoom } }` after the host scroll. The direct EngineCanvas law should start from a non-clamped camera, wheel once, and assert exactly one `setCamera` descriptor with the host’s post-scroll camera. The React contract test should use its existing mocked session to assert the matching `onAction` call after `wheel`.

Do not assert a made-up action for a delta that leaves a clamped camera unchanged until React behavior for that exact case is fixed as the oracle.

## Clipboard: TextEditor currently has no ownership route

### React contract

The hidden textarea owns browser clipboard events:

- paste replaces the exact selection and emits edit then selection;
- copy writes the exact selection;
- cut writes the selection even in read-only mode, and removes it only when mutable.

See `🧱️elements/✏️TextEditor/🟦️.tsx:719-754`. The context menu also delegates `cut`/`copy`/`paste` to the textarea through `document.execCommand` at `🧱️elements/✏️TextEditor/🟦️.tsx:600-680`.

### WGPU gap

`text_editor_apply_key_into` accepts printable input, Ctrl/Cmd+A, Backspace, and Delete; it deliberately has no Ctrl/Cmd+C/X/V handling:

- `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5903-5939`

The runtime’s ordinary paste and text input path offers content to focused Ink first, then enqueues a generic text operation. It never offers the content to focused TextEditor:

- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:417-429`

Generic retained UI clipboard does exist, including asynchronous platform reads, but it dispatches `UiEvent::Paste` to the retained document; it is not a TextEditor scene route:

- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1107-1111`
- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1395-1435`

`EditorHost` already provides exactly the local model needed for this: `selection_text` and `replace_selection`:

- `🧰️framework/🔨️modules/✍️editor/🦀️.rs:613-631`

### Executable slice

Introduce a focused TextEditor clipboard route that uses the same live-address validation as `apply_focused_text_editor_key`:

`window_id + presented generation + node + host_id + TextEditor kind + !close pending + !retiring`.

- Copy: write nonempty `selection_text()` through the existing platform clipboard adapter.
- Cut: write the same text, then replace the selection with empty text and publish the React action pair.
- Paste: replace the selection, then publish the React action pair.
- A late asynchronous read must revalidate the exact focus address before mutation, mirroring the existing Ink clipboard address discipline rather than trusting only a window id.

The React component enters read-only after a guest action refusal; WGPU currently has no equivalent local read-only state for TextEditor. Preserve copy in that condition and do not claim cut parity until the refusal/read-only contract is explicitly carried.

The neutral text-input fixture already has `a-paste-replaces-the-selection`. It is shared with Rust `EditorHost` laws and a Playwright textarea oracle, but the current WGPU renderer test consumes only `rendererKeys`, not its `sequences`. Add a scene-input fixture row or an explicit shared event table for copy/cut/paste action expectations. Test React with a real `paste`/`copy`/`cut` event on the textarea, WGPU with a focused accepted TextEditor, and stale delivery after close, new generation, and host replacement.

## IME: browser transport exists but committed text misses TextEditor; native ingress is absent

React ignores composing keydowns and inserts only committed composition data in `onCompositionEnd`:

- `🧱️elements/✏️TextEditor/🟦️.tsx:719-726`
- `🧱️elements/✏️TextEditor/🟦️.tsx:755-756`

The browser transport has the full IME vocabulary. It maps start/cancel and maps a bounded final `ime-commit` payload to `DispatchEvent::Ime(Commit)`:

- `🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts:118-123`
- `🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs:238-239`
- `🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:515-575`

The common native dispatch then sends that commit to a generic text operation rather than the focused TextEditor, and discards update/start/cancel:

- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:471-476`

Native `WinitApp::normalize` has no `WindowEvent::Ime` arm at all; it accepts cursor, mouse, touch, keyboard, and modifier events only:

- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:773-820`

### Executable slice

Implement a committed-text route before any preedit UI:

1. validate the accepted focused TextEditor address;
2. insert a bounded committed string using `replace_selection`/the action-pair helper;
3. ignore update/start/cancel for TextEditor, matching React’s no-canvas-preedit behavior;
4. normalize native `WindowEvent::Ime` commits into the same `DispatchEvent::Ime(Commit)` path.

The fixture’s existing `a-composition-commits-at-the-caret` case establishes text and selection output and is already replayed against a Chromium textarea. It does not prove real composition event plumbing because its Playwright harness uses `keyboard.insertText`. Add React `compositionEnd`, browser-worker IME-commit, and native Winit-normalization tests. Each must prove no text action after close, presented-generation change, or replacement of the TextEditor host before delivery.

## Accessibility: Surface application is not React’s editor textbox

React renders a real hidden `<textarea>` with its native multiline textbox semantics, `aria-readonly`, and a label of `"<language> editor"` or `"Editor"`:

- `🧱️elements/✏️TextEditor/🟦️.tsx:906-910`

Those labels are hard-coded in the React host; no `useLabel` or locale dictionary is involved. A WGPU fix should therefore use that same current text rather than create an unrelated localization key.

WGPU projects every `Component::Surface` as a focusable `application`:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:86-96`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:112-136`

`build_accessibility_dump` walks only that retained UI tree. It contains no text-editor virtual child:

- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:3761-3778`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:95-145`

The projection contract and browser mirror already carry role, label, focus, value, and editable state, but no `multiline` or `readonly` field. The mirror creates an `<input>` for any projected textbox, so it cannot express the textarea contract without a schema change:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:158-220`
- `🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:1-145`

### Correct boundary

Add a typed scene accessibility child only after the shared projection has `multiline` and `readonly`. For an accepted mounted TextEditor it should be a focused/actionable `textbox` child carrying:

```text
label:    language ? `${language} editor` : "Editor"
value:    current accepted editor buffer
multiline: true
readonly:  current explicit scene/read-only contract
rect:      accepted scene rect
```

The browser mirror should create a `<textarea>` for `multiline: true`, set `aria-multiline` and `aria-readonly`, and deliver its focused edits to the same accepted TextEditor input route. Do not model it as a chrome control and do not reuse generic Surface `application` semantics as a textbox.

A schema-first fixture should require native role/name/read-only/multiline/value semantics and an accepted-generation stale focus/value rejection. Extend `🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx` with the browser-mirror DOM oracle and the Rust accepted-projection law. The existing mirror has transport support for `accessibility-value`, but its fixed payload limit means a full-document accessibility edit must be bounded or streamed explicitly; do not silently truncate text.

## Focus and retirement status

The current focused TextEditor address is correctly bound to window id, presented generation, node, and host id, and it rejects close-pending or retiring hosts before a key operation:

- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1533-1601`

It is cleared during close and scene-host retirement and is rebased only once the successor frame is accepted:

- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2025-2129`
- `🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1164-1185`

Reuse that guard for clipboard and IME. The current pointer entry resolves the published target then blurs a TextEditor only for a primary press outside that target, retaining focus for an in-editor press:

- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:17148-17153`

## Completion boundary

No claim of TextEditor parity is justified: clipboard, IME, accessible textbox projection, wheel `setCamera`, and asynchronous refusal/coalescing remain. Wheel camera publication is the next independent compact behavior slice. Clipboard, IME, and accessibility each require their own stale-focus regressions rather than being folded into keyboard navigation.
