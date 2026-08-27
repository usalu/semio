# Metadata Source-Provider Proof Contract

## Frozen Ownership Boundary

FND-METADATA-SOURCE-PROVIDER-28 joins the existing wrapped declaration origin, Rust metadata facts, module-context graph and selected Cargo provider binding. The wrapped payload's physical source is a consumer source, normally a direct mutation leaf. It must **not** equal the metadata provider's library root. Derive provider and contract provider are separate resolutions.

The exact approved package identities are:

| Role | Provider manifest | Package | Explicit library | Macro |
| --- | --- | --- | --- | --- |
| Lower contract | `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-replication` | `protocol` | false |
| OS facade | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-kernel` | `semio_framework_os_kernel` | false |
| Metadata derive | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-kernel-dsl-derive` | `dsl_derive` | true |

Each currently declares `📦️glue.rs` as its explicit library source. These are repository-relative designated provider identities, never guessed from a dependency key or terminal type name. Exact direct or inherited Cargo bindings must agree with every identity field. The OS facade's actual public export route must resolve to the designated lower contract or derive provider; its package name alone does not bless arbitrary symbols.

## Proof API And Resolution

Add one repository-owned `inspectMutationMetadataSource` proof API in discovery, with the exact wrapped origin triple, repository root, and existing file/source/module-context evidence. Reuse `inspectRustModuleGraph`, `inspectRustMutationMetadataFacts`, `inspectRustModuleGraphFacts`, `inspectRustStructure`, and `resolveCargoProviderBinding`. No second filesystem inventory, module graph, TOML parser, hardcoded payload type name, or conventional-root authority is permitted. The caller's owned inventory supplies source text; provider filesystem reads remain bounded by FND25. Provide a cancellation callback for bounded traversal loops.

1. Require one unconditional direct canonical declaration at the origin. Child-facet declarations remain unapproved until their compiler-source authority is extended. FND21's wrapper path remains the source of the origin, not a newly guessed declaration.
2. Resolve the consumer manifest/library context of that declaration using the existing graph. A source mounted through ambiguous consumer contexts is not automatically approved. Do not equate the consumer leaf with a provider root.
3. Resolve the declaration's metadata derive and absolute `mutation_leaf(contract = ...)` path independently. Derive must reach the designated `MutationLeaf` macro; contract's `MutationLeaf` must reach the designated lower trait. For an in-provider self alias, verify the consumer itself is the exact designated package rather than looking for a fabricated dependency edge.
4. Follow token-proven extern aliases, local imports and public reexports in their actual Rust module scope. Private imports can be used inside their own scope; external reexport hops must be public. Support grouped/renamed imports, explicit `crate`/`self`/`super` paths, and actual public glob export chains without terminal-name guessing. Multiple routes to the identical origin may converge; differing or conditional origins reject. Resolve through the existing module contexts and source-local scope facts.
5. Reject a manual implementation of the genuine metadata trait for the wrapped declaration, including imported trait aliases. Inspect all impl trait paths rather than relying solely on FND12's terminal-name-filtered manual list. Source policy complements, and does not pretend to replace, compiler ownership/provenance checks.
6. Unknown, conditional, cyclic, shadowed, unsupported external or unsafe evidence fails closed with an explicit diagnostic. Unsupported import syntax cannot silently count as a resolved provider. No real excluded directory is touched, even in negative tests.

The result records acceptance/diagnostics, exact consumer context and resolved derive/contract identities. It is not an opt-in compatibility path: global structural-policy activation is part of the already-required mandatory trait/aggregate transaction. This packet must be tested before activation, and its acceptance must not be reported as global enforcement.

## Write Set And Verification

The executor owns one narrow discovery proof region and necessary reuse-oriented fact extensions, explicit TypeScript facade exports, dedicated neutral schema/fixtures/tests, and existing test-router/launch entries needed to execute those tests. Root's current structural-policy/reachability region, Rust sources, traits, registry and production mutations are excluded from this packet.

Use schema-first compiler-backed fixtures for genuine lower and OS-facade routes, direct/renamed/inherited dependency bindings, root extern aliases, explicit local imports, grouped and glob reexports, convergent same-origin routes, and semantic payload aliases. Negatives include fake same-name packages, wrong manifest/library/proc-macro identity, absent or fake derive, absent/malformed/conditional contract attributes, manual direct/aliased impls, shadowed aliases, competing globs, wrong consumer context, child-source authority, and every relevant no-follow/exclusion boundary. Retain a real STDIO consumer proof in addition to miniature examples. Run only Bun/Nx and standalone compiler oracles; root alone runs Cargo.
