# Actor Codec Domain Taxonomy Read-Only Audit

2026-10-14. Concurrent source audit; no source edits or registered tests, no initializer execution.

- Found stale catalog owner `framework.actor.shard-client.component-codec` at JSON line 6238 and Markdown line 266. Paths already point to Actor codec. Reported to root; root is repairing it.
- After root repair, repository-wide hidden-file text scan (excluding tickets, node_modules, .git and generated outputs) found no old `ShardCodec`, `shardCodec`, `SHARD_CODEC`, old moved directory/URI, or `actorComponentCodecSchema`; only the above catalog key remained at scan time.
- Defining implementation has zero imports and no specific-product type dependency. Public request/success/answer/export shapes use built-in bytes, strings, tuples and Promises.
- Every relative static import/export in defining codec, moved test and public codec facade resolves to an existing file. Facade keeps `./codec`; schema export is `actorCodecSchema`.
- Operation projection schema and test reference now use `ActorCodecOperationAnswerProjection`. Schema and fixture schema URI use `/framework/actor/codec`; fixture scope uses `actor-codec`.
- Actor owner-boundary schema/corpus rosters include the new test path; public-facet schema/corpus includes canonical schema export. Actor task script references the moved test in typecheck and test commands. Taxonomy runtime lists include new defining path.

Limits: no typecheck, registered test, framework collector, or runtime verification ran. Initial broad Git status output was too large and truncated and is not audit evidence. This targeted textual audit does not certify entire transitive dependency closure, exact public API enumeration execution, or schema-document-index generation. Files can change concurrently after these hashes.

## Exact Source SHA-256

- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🟦️.ts`: `4d2692380bfad7a71db82a9923d8091ca643997a82c441bbbd7b3f500b1cf63f`
- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🧬️schema/🔣️.json`: `db14ccbc1dcba4dd94fca2a21ce6f7b1b8c7735e32ad2fd091533fb605a49abf`
- `🧰️framework/🔨️modules/🎭️actor/🧬️codec/🧪️tests/🟦️.ts`: `2a8f64c9987266bd04773a74e4db340168c3ce61e86b3316b59dd017039a2118`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts`: `315c6c9ce009a2b04ff76794503d63e79c2cbe615be91038ee011ca8477d7576`
