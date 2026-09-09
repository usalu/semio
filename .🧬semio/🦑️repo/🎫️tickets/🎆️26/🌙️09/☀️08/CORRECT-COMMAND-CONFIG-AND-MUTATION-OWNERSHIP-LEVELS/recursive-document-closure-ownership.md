# Recursive Document Closure Ownership

The Store owns complete owned-document closure admission. A root and its descendants must agree with each decoded member's typed ChildRestoreProjection, exact OwnerRef and full ArtifactRef. Global member admission is limited to 1,024 descendants; each parent still has its independent 64-reference projection limit. Child payloads and forward links are outside this validator.

The cursor uses fixed index/queue metadata and a caller-owned immutable candidate source. It is generation-fenced and yields for cancellation, fuel and deadlines. It never owns or drops the candidate member payloads. A completed topology check must be paired with the SDK's exact live generation and atomic candidate publication; this contract alone does not save/load a full document.

Language-neutral vectors cover unordered Kit→Object→Mesh/Value closure, missing grandchildren, dialect/owner/slot substitutions, duplicate members/edges, unreachable cycles, and separate global/per-parent limits. A Graphlib graph oracle and Ajv shape oracle will validate the same cases independently.

Additional open ownership debt: LayoutDrawingChild embeds content in the parent's document and its custom export paths read that content. Its import/export must be wired to actual member stores and child read views when the SDK archive path is available. Shared durable-group currently contains GIS-specific parent/drawing/value recovery schema constants; the general decision mechanism and the GIS binding must be separated without weakening current atomic publication.

## Implementation And Verification

Store now exports OwnedDocumentClosureSource and a fixed-scalar OwnedDocumentClosure cursor. An open-addressed fixed index rejects duplicate identities; a breadth-first queue walks only declared, exactly owned descendants. Missing members, extra/unreachable members, identity/dialect/owner substitution and cycles cannot complete. Source generations, operation generations, cancellation and expiry are checked on every continuation. ChildRestoreProjection exposes one borrowed admitted row through get(index). The SDK still must connect its immutable candidate source and commit authority.

The independent Ajv/Graphlib oracle and strict TypeScript passed. The first actual red run failed on the intentionally missing implementation module; an earlier incorrectly copied ticket route ran the Map oracle instead and is not closure evidence. The first implementation run exposed the zero-fuel test expectation for a globally oversized input; its structural refusal takes no work and the assertion now records that behavior. Native tests are authored and queued, with no pass claimed.

OwnerRef now lives in the shared child-owner schema with Rust, TypeScript, JSON Schema, GraphQL and Proto facets. Store reexports the native type. Closure test vectors independently exercise owner record parsing; the closure projection JSON Schema references this shared owner schema.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🔗️.graphql`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🛰️.proto`

Store root mounts/reexports the cursor and moved OwnerRef; it adds ChildRestoreProjection::get. Root script/project, both launch registries and ticket validation project/script register closure oracle/native commands.


## Continuation Member Count Fence

Two neutral vectors now change candidate member count after the root step without advancing its generation. The TypeScript cursor initially continued (`owned-document-closure-count-red-1.log`) instead of rejecting the stale source. It now captures and fences the initial member count like the Rust cursor. The same vectors feed a native assertion and an independent Ajv constant-schema oracle. Bun/Nx oracle and strict TypeScript passed (`owned-document-closure-count-green-1.log`, exit 0); the native continuation assertions remain queued with the closure suite.

## Native Owner Stamp Rejection Follow-up

Native closure run 1 executed two tests: the cancellation/generation/deadline law passed, while the neutral graph suite failed at the OwnerRef foreign-field vector (`owned-document-closure-native-1.log`, session 28322; one passed, one failed). OwnerRef's native FromValue derive omitted `deny_unknown_fields`, allowing the foreign locale field. The shared owner schema now denies unknown fields, matching TypeScript and JSON Schema. The failing assertion includes its input for diagnosis. Native run 2 is queued as session 45057 (`owned-document-closure-native-2.log`); passing runtime closure behavior is not yet claimed.
