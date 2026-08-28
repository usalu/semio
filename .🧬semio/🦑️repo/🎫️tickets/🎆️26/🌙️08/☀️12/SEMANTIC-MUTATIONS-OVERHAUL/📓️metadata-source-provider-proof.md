# Metadata Source-Provider Proof

## Initial Slice

`inspectMutationMetadataSource` is a repository-owned, non-operative discovery proof. It requires one unique direct consumer declaration origin and consumer module context, then independently resolves the derive and `mutation_leaf(contract = ...)` route through the bounded Cargo binding resolver. It checks the frozen lower, OS facade, and derive manifest/package/library/proc-macro identities; a facade path must publicly re-export the lower trait.

The existing metadata token facts now retain a `use …::*` alias fact so a later proof can follow a proven public glob route rather than guessing a terminal name. No second Rust parser, module graph, Cargo/TOML parser, inventory, policy activation, or production Rust mutation was added.

## Local Scope Checkpoint

The local `crate`, `self`, and nested-inline `super` routes are now proven through the one reused module graph and metadata-fact parser. The proof first resolves a local scope token, then only accepts a direct Cargo binding or a token-proven alias in that resulting scope. It does not invent a second parser or graph. Candidate routes are fail-closed: a known competing route that cannot establish the frozen identity now rejects the declaration, while duplicate routes to the same canonical provider converge.

The schema fixture adds a root `self::facade` positive, a nested inline `super::facade` positive, a direct-versus-facade ambiguous local binding negative, a shadowed facade-versus-derive alias negative, and a conditional nested `super` negative. The existing crate-scoped regression is retained. The compiler oracle now compiles both `self` and nested `super` contract spellings alongside the proc-macro/facade/lower chain.

Executed and retained under `🧪️metadata-source-provider/🧪️self-super`:

`SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️metadata-source-provider/🧪️self-super' bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test -t 'proves independent canonical derive and lower contract routes from one consumer declaration'`

Passed: one focused Bun group, 24 assertions, 301 filtered, including four standalone `rustc` compilation steps. The registered Nx route also passed through the required wrapper:

`SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️metadata-source-provider/🧪️self-super-nx' bun ./📜️script.ts nx exec --projects=workspace -- bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test metadata-source-provider`

It passed with one focused group, 24 assertions, and 301 filtered tests. `git diff --check` also passed after the local-scope change.

## Deliberately Outstanding

This is not FND28 acceptance. Still open: a real STDIO consumer proof; inherited dependency inputs; multi-hop grouped public re-export chains and convergent same-origin routes; local inline scope cases beyond the completed `crate`/`self`/`super` routes; competing glob, unknown, cyclic, and unsupported-import diagnostics; wrong manifest/library/proc-macro identity; absent, malformed, and conditional attribute coverage; direct manual implementation coverage; wrong/ambiguous consumer context and child-source authority; and the complete unsafe-locator, no-follow, exclusion, and cancellation matrix. Global metadata policy remains inactive.

## Current STDIO and Inherited Binding Slice

The neutral schema now carries an inherited-workspace dependency vector. The resolver keeps Cargo's selected normal dependency authority, then compares its derived Rust extern name with the actual Rust alias; it does not infer an alias from a package basename. This permits the current kernel `protocol` spelling only when the bounded Cargo binding proves that its provider library name is `protocol`.

The real STDIO proof reads only an explicit, no-symlink, compose-excluded source inventory: the consumer manifest/root, TXT and glTF roots and leaves, kernel facade/root, DSL component, SPR component, and SPR command component. It derives each public unconditional leaf origin from metadata facts rather than hardcoding a Rust declaration name. Both prove the exact frozen derive, façade, and lower identities through the existing module graph.

Macro and type namespaces remain distinct while following public re-exports: a canonical lower trait path cannot invalidate the derive-macro proof, and a canonical proc-macro path cannot invalidate the trait proof. Unknown or noncanonical routes remain rejection evidence. The target uses the long profile because the compiler-backed neutral oracle and real source proofs exceed the ordinary focused budget.

Executed and retained under `🧪️metadata-source-provider/🧪️stdio-registered`:

`SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️metadata-source-provider/🧪️stdio-registered' bun ./📜️script.ts nx exec --projects=workspace -- bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test metadata-source-provider long`

Passed: two focused groups, 41 assertions, 302 filtered tests, zero failures. This is a bounded slice only, not FND28 acceptance. Still open: complete competing/glob/unknown/cyclic diagnostics across a complete source inventory; the full conditional, malformed, child-source, unsafe-locator/no-follow, and cancellation matrix; exact manual-implementation variants; and root-owned global policy activation.

The registered target was also executed through the workspace wrapper with `SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️metadata-source-provider/🧪️stdio-registered-target-final'` and `nx run @semio-tech/repo-lib:test-metadata-source-provider`: two groups, 41 assertions, 302 filtered, zero failures. Nx emitted its generic flaky-task advisory after success; this run had no retry or test failure.
