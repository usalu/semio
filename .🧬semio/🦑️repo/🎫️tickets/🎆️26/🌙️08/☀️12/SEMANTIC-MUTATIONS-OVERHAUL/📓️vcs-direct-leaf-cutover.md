# VCS Direct Leaf Cutover

## Baseline

- Scope: VCS 1 Any schema mutations only; IO mutation ownership remains a later batch.
- Six semantic kinds: rename-vcs, change-counter, change-notes, change-status, add-tag, remove-tag.
- Observed: six nested `🦠️mutation/🦀️component.rs` owners and zero direct Rust owners.
- Root currently holds the aggregate, a hand-maintained roster, protocol-dispatch helpers, and behavior tests.
- Existing leaf fixtures pin before, payload, after, diff, and outcomes for all six kinds.

## Intended Invariants

- Each semantic kind owns a direct Rust behavior file, descriptor, payload schema, wire schema, TypeScript, GraphQL, and protobuf.
- The root is transparent; generic protocol-dispatch helpers and cross-mutation store laws are schema operations.
- Schema mutation codecs remain in the existing IO layer, outside this batch; no replacement codec or compatibility path is introduced.
- Every fixture must validate against its direct payload/wire schema through Ajv, with malformed payloads rejected.

## Verification

- Six direct owners, six descriptors, six payload schemas, six wire schemas, and complete TypeScript/GraphQL/protobuf surfaces are present.
- Existence-checked structural query: 0/17.
- Ajv/internal descriptor validation: 12 agreements across six valid and six deliberately invalid descriptors.
- Ajv wire/payload validation: six committed fixtures accepted, six malformed payloads rejected.
- Nightly Rust AST: 20/20 owner, aggregate, diff, inverse, and operations files parsed.
- Rustfmt and scoped `git diff --check`: clean.
- TypeScript aggregate import and six-kind roster: clean.
- Runtime compilation remains pending the serialized compiler gate.
- The plugin's pre-existing `[DEBUG] vcs ts ok` message is outside this mutation scope; no temporary debug probe was added by this cutover.
