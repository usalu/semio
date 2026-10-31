# Catalog Public Exports Review

## Correction and Current Verdict

The earlier audit incorrectly generalized filesystem/precision discovery results to fresh-checkout Source admission. It missed `.gitignore:97` (`**/🤖️generated/`), which ignored all three authored package façades under their former directory. Explicit runtime entries and permissive filesystem discovery do not make ignored authored Source available in a new checkout. The prior no-blocker conclusion about that location is withdrawn.

Root moved the three authored façades to `✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/📚️catalog`, preserving their implementation bytes and directory depth. Current static review finds no remaining blocker in this relocation. Independent read-only `repo tools exec --light git -- check-ignore --no-index -v -- <three current paths>` returned exit1 with no output: none of the three current paths is ignored. This proves ignore admission, not that uncommitted authored additions have already been committed; no modifying Git action was performed.

## Public Contract and Relative Imports

The existing package public keys remain `./generated`, `./generated/plugins` and `./generated/playgrounds`; only their export target coordinates change to `./📚️catalog/...`. The neutral fixture uses the same targets while its implementation references remain the owning catalog-level `🤖️generated` files. The root façade still uses three parents to the owning Native catalog JSON; nested plugin/playground façades use four parents to their generated implementations. All paths exist and named value/type reexports match their producer. Identity is forwarded directly, with no copied generated payload or compatibility alias.

The actual generator output remains catalog-level `🤖️generated`, separate from authored package `📚️catalog`. No producer output owns or overwrites the new façade directory. The runtime export test checks formal package declaration, actual runtime resolution, independent TypeScript resolution, value kind and identity against the owning implementation. Root reports causal old-coordinate neutral failures and final42266 green pending settlement; this audit does not invent its terminal result or rerun tests.

## Closure, Discovery and Lifetime

Current test-catalog-exports runtimeSourceEntries explicitly declare the test plus all three new 📚️catalog façade paths with workspaceRoot prefixes; its data resource is the actual package.json. Catalog-wide inputs cover fixture/schema/public package files, and owning generate remains its dependency. Local target chain publication→exports→generate is unchanged and acyclic in the inspected declarations.

Current canonical generatedDirectories policy excludes the distinct 🗑️generated, not 📚️catalog. The schema capsule matcher only excludes exact .p/.d eight-lowercase-hex components. Package/source discovery, precision explicit entries and import closure therefore preserve the new authored directory. Bootstrap manifest-only exclusions for 🤖️generated no longer intersect it either. Clean exact build/ticket classifiers do not classify 📚️catalog as generated output. These facts supplement, rather than replace, the actual Git admission result.

The routed child still uses issued15-second control from the received invocation and passes its signal to the owned command; no new Original is minted. The three new absolute path lengths are84,95,99UTF16 units; the longest is129 with the existing46-unit root budget, within256. No AGENTS edits, tests, compiler or Native jobs were performed by this review.
