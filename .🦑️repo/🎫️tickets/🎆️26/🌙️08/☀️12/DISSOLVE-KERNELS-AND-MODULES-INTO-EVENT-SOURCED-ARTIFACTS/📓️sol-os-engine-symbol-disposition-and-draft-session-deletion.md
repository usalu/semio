# OS Engine Symbol Disposition and Draft-Session Deletion

## Stable audit basis

- Repository `HEAD`: `07873f842a5a99ac2f69c1648c21f36ebf260bdb`.
- Engine component: `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs`.
- Engine SHA-256 before any packet: `32b7ecd98ddbb9858e91a7060bd72b299c7fb1dcb3245ac92e9c709042eb7237`.
- The component and its immediate OS kernel glue were clean throughout the read-only audit.

## Live symbol families

The component is a mixed semantic umbrella.

- `EngineKey`, `EngineHandle`, `EngineFault`, `Engine`, and `EngineCache` implement the OS host-owned content-addressed guest-compute boundary. They have live plugin-host/WIT/runtime consumers. Their consumers are currently dirty, so this family remains quarantined.
- `EngineHandles` is a public host-to-app transport bag with 54 direct Rust consumers across plugin apps and OS dispatch. It survives but requires later specific ownership and manifest treatment.
- `EngineRep` has one live implementation in the stdio Semio BREP topology component. Under the one-consumer rule it must eventually become private to that component, but stdio glue and the BREP owner are in an active external wave.
- `DynEngine` and `CacheEntry` are correctly private implementation of the cache family.
- `EngineHost` has no live Rust implementation or call site. It is mentioned by a dev-policy scanner and future-injection comments. Those are not production consumers, but editing the policy wording is a separate owner concern; this trait remains quarantined rather than being mixed into the draft-session lease.
- `DraftBaseHash`, `DraftEngineSessionStats`, and `DraftEngineSession` have zero production callers. Six tests exercise only their own mechanism. One Surface Paint comment uses the type name as an analogy but does not consume it.

## Binding decision

An older W1 ticket established `DraftEngineSession` as a sanctioned possible mechanism. The active amendment supersedes speculative retention: zero production consumers are deleted, and tests or future requests do not count. Therefore the draft-session family and its mechanism-only tests must be removed.

The only external cleanup is the stale analogy in `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs`, current SHA-256 `74b9544a145819e73bbc92516fba3072335c7ef903906de56987498bc4888e52`. The underlying comment will continue to state the real invariant directly: paint scratch buffers are droppable without losing committed state.

This deletion does not touch plugin host, plugin dispatch, WIT, stdio, OS glue, Cargo manifests, registries, generated outputs, or the remaining engine families.

## Dirty-frontier quarantine

The repository currently has 6,606 dirty paths, dominated by a separate viewer/editor artifact wave plus plugin/glTF work. Global census, generation, formatting, and registrar edits remain paused. The draft-session packet is safe only because its two exact writable source paths are clean and outside every active owner.
