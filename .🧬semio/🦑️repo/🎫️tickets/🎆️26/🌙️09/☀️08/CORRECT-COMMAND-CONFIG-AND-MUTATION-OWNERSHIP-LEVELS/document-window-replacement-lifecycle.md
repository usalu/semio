# Document Window Replacement Lifecycle

## Ownership Decision

A successful replacement of the current document invalidates all concrete window transient state and all tool work captured from the previous document. Window configuration survives. Reloading identical bytes is still a new document generation: content hashes and per-partition generation alone cannot distinguish it.

Both SDK replacement routes are covered: text/pack loading through Store reset and retained candidate publication through the Store initialization owner. A replacement registry is prepared and retirement capacity checked before document publication. The old registry moves into a fixed-capacity retirement registry; the normal maintenance round robin and app close retire it under one-item grants. A failed document parse/reset/publication keeps the current registry and tool scope. No old transient root is dropped recursively at replacement.

Each captured WindowTransientSnapshot and pending typed window publication carries document generation. Begin and advance reject prior-document authority even if window ID, kind, local generation and content are identical. Context identity includes document generation. Old keyed work cannot rebase across replacement because its cancellation scope is closed; the next keyed work receives a fresh scope while old owners remain retained for cleanup.

This batch resets the exact WindowTransient registry. Existing AppTransient, draft, interaction, and presence document-relative lifetimes still require separate owner review; no claim is made that all ephemeral lanes are solved here.

## Tests and Current Evidence

The neutral retained-window-input fixture now contains a same-window/same-local-generation case in a second document generation and a two-window replacement trace. The independent TypeScript/Ajv oracle checks identity and reset expectations. Native tests exercise real typed transient publications, both concrete windows, old authority and pending publication rejection, bounded close, and cancellation scope renewal with old keyed/single work and fresh keyed work.

The generation test was changed before production fields were added. Its first Nx attempt (`window-document-generation-red-1.log`) stopped before tests at concurrent taxonomy validation, so it is not a behavioral red result. Native validation after implementation is pending in `window-document-generation-green-1.log`. Wires owns the end-to-end reload-during-drag tests.

## Changed Files

- SDK root `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.
- Window transient owner `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs`.
- Its `🧫️fixtures/🪟️retained-window-input/🔣️.json` fixture.
- Its `🧪️tests/🪟️retained-window-input/🦀️.rs` and `🟦️.ts` tests.
- Its `🧪️tests/🔁️document-replacement/🦀️.rs` cancellation lifecycle test.
- Ticket `validation/📜️script.ts`: lazy root VerifyScript import; scoped DAG and retained-window test routes.

## Implementation Placement and Audit Follow-Up

The reset implementation is authored in `plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`, included within the SDK app module so private owner fields remain private. The SDK root contains only the fields, lifecycle hooks, and existing shared cancellation mechanism changes.

The TypeScript/Ajv oracle completed successfully in the current attempt (five distinct ownership tuples plus replacement expectations). Native compilation is queued behind the Wires diagnostic using the same target directory. No native passing result is claimed yet.

The typed registry delegates payload disposal to each registered WindowTransientOwner. The existing generic `bounded_transient_store_disposer` still measures the entire DSL payload and requires that many bytes in one retirement step. That implementation needs a separate boundedness audit for large results (Jack in particular); merely moving the owner into a registry does not prove payload cleanup is bounded. This limitation is explicitly pending.

## Independent Fairness Audit Corrections

The Terra audit found first-entry starvation at all three levels: displaced document generation, window kind, and concrete partition. Added separate maintenance/close generation cursors plus rotating kind/partition cursors; a zero-item grant leaves cursors and owners untouched. Added neutral fairness expectations with independent TypeScript Map projection and Rust tests that keep the first owner blocked while later owners drain. Native verification remains pending; no behavioral RED result was observed before implementing these corrections. The generic large-payload disposer and returned snapshot lease findings remain open and are not solved by this scheduling change.

## Native Attempt And Repair

`window-document-generation-green-1.log` ended with exit 1 in the SDK test crate: the new document-generation fairness fixture omitted `std::sync::Arc` import in two factory signatures. No other compiler errors were reported. Added the import and reran the registered Bun/Nx target as `window-document-generation-green-2.log`; execution remains pending.

## Testkit Source-Authority Repair

The second native attempt reached the SDK test crate and reproduced sixteen missing mutation descriptors after source relocation into testkit. The exact metadata/payload moves and unchanged semantic identities are listed in `sdk-testkit-mutation-metadata-ownership.md`. Native lifecycle/fairness execution is still unverified.

## Native Attempts Three and Four

Attempt three stopped at the shared durable-group Send/Sync bounds before SDK tests; those source bounds are now fixed by the transient foundation owner. Attempt four compiled kernel and reached SDK, then failed during the active WindowTransientOwner bundle/read-lease migration. It also exposed four existing typed-command test initializers missing the newly required terminal_fault field; those initializers now explicitly start at None. No runtime success is claimed for either run.

## Tracked Read Test Adaptation

Both lifecycle test owners now use WindowTransientOwnerBundle with explicit preparation and owned state/mutation retirement. The shared scalar-publication testkit is deliberately limited to PublicationTransient and its one u64 mutation: compile-time assertions reject heap-owning or widened payloads. Preparation copies the exact scalar and retirement charges its actual fixed payload size; no whole DSL/binary encoding is used to estimate work. Identity tests now acquire real tracked erased reads and drain their exact store after returning them. Fairness fixture owners initialize both independent retirement and maintenance cursors. Native validation remains pending shared production owner migration.
