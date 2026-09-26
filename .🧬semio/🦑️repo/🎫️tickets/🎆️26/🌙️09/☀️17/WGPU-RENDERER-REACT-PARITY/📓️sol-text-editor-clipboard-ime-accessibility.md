# TextEditor Clipboard, IME, and Accessibility

## Accepted host boundary

Clipboard and composition ingress reuse the exact focused TextEditor address: window id and accepted window/surface generation, retained component node, and host id. Delivery revalidates the current non-closing document, component kind and host, and retiring state before reading or writing clipboard data or mutating the editor. Pending native reads and streamed browser text carry this address and become no-ops after focus, generation, host, or lifecycle changes.

Text insertion reserves an ordered two-action batch before local mutation. It publishes `textEdit { surfaceId, text }` followed by `textSelect { surfaceId, start, end }`. Copy writes the exact selected UTF-8 text with no action. Editable cut writes then deletes; paste and committed IME text replace the selection. Readonly and stale hosts refuse mutation.

The browser and native ingress paths normalize paste and IME commit into the same accepted-host operation. Browser text streams are bounded to 32 slots and 16 KiB. Native winit IME Enabled/Preedit/Commit/Disabled events normalize to start/update/commit/cancel; only committed text mutates the document.

## Accessibility

The shared accessibility wire now carries `multiline`. Accepted TextEditor scenes publish a virtual textbox at the rendered rect with the React name (`<language> editor`, or `Editor`), current optimistic text value, focus, editable, and multiline state. The browser mirror creates a real `textarea` with `aria-multiline` and `aria-readonly`. Focus, blur, and full-value accessibility actions revalidate the same accepted host before acting. Retirement removes the virtual textbox.

## Oracles and validation

The production React TextEditor tests dispatch actual DOM paste and composition events against the hidden textarea and consume the neutral text-input fixture. Focused result: 2 passed, 684 skipped, exit 0. The focused accessibility mirror law passed: 1 passed, 15 skipped, exit 0. The root-owned Playwright text-input oracle passed all 29 cases with the canonical Playwright cache.

Native laws queued for the coordinated cargo run:

- `focused_text_editor_clipboard_composition_and_accessibility_share_the_accepted_host`
- `normalized_native_ime_commit_preserves_the_exact_unicode_payload`

## Deferred action receipts

WGPU dispatch currently has no result channel from `FrameDeferredCursor` back to EngineCanvas or Interpreter. TextEditor mutates its local echo and queues generic actions synchronously; a later dispatch error is only logged and counted. React instead keeps one edit in flight, coalesces queued edits latest-wins, awaits edit acceptance before selection, and resynchronizes refused optimistic state.

A correct WGPU continuation needs a renderer-local opaque dispatch token carried through deferred action work. It must keep the first edit in flight, retain only the latest queued edit, dispatch selection only after edit acceptance, skip it on refusal, and resynchronize or mark readonly against the exact accepted host. Descriptor-value matching after dispatch is not a sound identity and must not be used as a compatibility shim.
