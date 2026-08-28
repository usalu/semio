# Prepared Host Consumer Boundary

## Current Evidence

The paired Surface owner now publishes prepared scene records atomically, and managed React effect readers have executed replacement/unmount tests. This does not change the live Interpreter yet.

Interpreter `surfacePropsToComponentSceneNode` still constructs a whole Uint8Array, decodes the complete ScenePack, and casts a dynamically assembled object into the former host input. Fourteen hosts use the resolver switch; VirtualFileSystem is a separate fifteenth branch. The old comment declaring those hosts outside a previous packet is not an exemption for this task.

The typed catalogue already captures all fifteen native scene root schemas. Fourteen of those roots contain nested JSON strings; DiffView contains text fields without JSON. NodeGraph additionally contains typed sequences and records. TextEditor is a concrete example of a second whole boundary: it reconstructs a generic pack from the complete scene, synchronously calls `syncFromScenePack`, parses tokens/diagnostics/completions/rename/newline-gate JSON, and uses whole buffer strings for textarea and edit events. Passing a reconstructed object into this host would hide the remaining work, not complete retained admission.

## Next Coherent Integration

Prepared-host consumers must use the exact managed record/text readers, with effect cleanup unregistering scheduled work and returning readers to the subscription-owned close queue. A raw prepared owner must not be passed into render, and read refusal must retain the old rendered result rather than substitute an empty scene.

Nested JSON requires a retained typed projection over the captured text field. A source scan found no reusable TypeScript retained JSON decoder in framework modules. The native GLB token cursor is domain-specific; the Store canonical cursor is an encoder, not a decoder. Any new projection must preserve JSON escaping, duplicate-key semantics, full number tokens, and byte-accounted scanning without a whole `JSON.parse` call. Raw numeric spans can be retained until a schema-owned numeric conversion gate; this cannot silently narrow the accepted domain or claim complete numeric host support prematurely.

The downstream wasm sessions also need page/field admission where their existing `syncFromScenePack` accepts one entire contiguous payload. That interface cannot be certified by moving the same synchronous call into a React effect. Existing CodeMirror/textarea or canvas platform operations need a named bounded admission boundary and actual maximum-domain timing evidence, not a wrapper timer. These remaining joins are implementation work, not permission to remove supported host behavior.

## Preservation

No cleanup, deletion, normalization rename, compiler launch, commit, or ticket closure was performed in this packet. Active ticket evidence remains preserved. Native finite geometry/default parity is still separate, queued through the sole compiler owner when source is ready.

## Retained Nested JSON Design Boundary

The next parser must consume a privately captured prepared text-field byte view, not arbitrary user-supplied callbacks or a complete reconstructed string. Existing prepared `beginText` emits bounded decoded text chunks but loses direct byte-range positioning for later random field reads. A narrow typed text-byte range reader over the existing exact ScenePack root can preserve source ownership, UTF-8 spans and slice offsets without reconstructing the whole field.

The JSON result should remain a flat persistent numeric-index tree of immutable token/span records. Arrays and objects use explicit linked parse frames, not recursive JavaScript calls. Strings retain raw source spans and decode escaping incrementally; duplicate object keys must preserve JSON.parse's last-value/first-insertion-order behavior, including escaped-equivalent keys. Long keys cannot be inserted through a whole-string comparison or hashing callback. Numeric tokens likewise retain their exact span until a schema-owned numeric admission rule; a whole arbitrary-length Number(string) call is not a bounded conversion certificate. JSON grammar accepts only finite lexical syntax, but JSON.parse may round overflowed decimal literals to Infinity; native serde_json and individual host geometry policies differ, so this edge needs an explicit neutral policy fixture rather than an implicit global finite-number ban.

This is design evidence only. No JSON parser or live host adapter is claimed implemented here. The exact native input-retirement token handshake took priority while the producer peer completed that cross-owner seam.
