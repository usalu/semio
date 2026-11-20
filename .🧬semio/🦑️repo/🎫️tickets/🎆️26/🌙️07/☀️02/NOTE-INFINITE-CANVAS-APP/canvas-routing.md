# Canvas interaction routing

The Note artifact editor publishes an Ink Canvas scene for the editable Canvas and a separate scene for the Navigator. The request-context renderer sets the `blocks` interaction domain, which the React and WGPU hosts use for canvas selection and hover. The base renderer previously omitted this domain even when the scene was marked interactive.

Both render paths now construct the scene through the same function and use the interaction domain and granularity declared in the editor manifest. The Navigator stays read-only. A JSON fixture and native editor test exercise both windows, decode their actual packed scenes, and compare the wire schema and interaction routes against the fixture.

The React host now clears the `pointer` hover channel when the pointer leaves the editable canvas. The mounted regression sends an actual pointer-leave event after hovering a block. Separate cases cover moving to empty canvas space and leaving a read-only scene. The shared JSON fixture defines the valid interaction domain and malformed cases checked by Ajv in React and by the typed decoder in the WGPU test.

`repo tools exec git -- diff --check` reported no whitespace errors for the editor changes. The pinned Rust compiler parsed the modified renderer and its regression test with `-Z parse-crate-root-only`. A Bun check confirmed the fixture's document schema against the actual TypeScript `parseInkScene` host parser and its view mode against the Note editor's TypeScript body key. The native test commands were canceled before admission so the shared machine could build the Note dev guest first.
