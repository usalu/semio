# Root Artifact Schema Law Extraction Acceptance Audit

Date: 2026-09-13  
Status: **not accepted yet**. The owner split is structurally sound, but independent focused execution found two source-admission/test-control defects that need repair.

## Current Owned Tree

The portable ownership fixture declares exactly eleven anonymous implementation leaves across thirteen contextual kinds:

1. discovery source access
2. artifact owner discovery
3. declared export identity
4. facet leaf loading
5. facet completeness law
6. field parity law
7. state parity law
8. diff coverage law
9. type-name parity law
10. ownership field parity law
11. aggregate law

The tree is rooted at:

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact

The live root now directly imports only the artifact aggregate and ownership-field-parity law; the field-parity oracle imports owner discovery, aggregate and ownership parity directly. No root compatibility export remains.

## Structural Evidence

I independently ran the focused package command:

    env SEMIO_TEST_ARTIFACT_DIR=<ticket-private> bun ./📜️script.ts test root-artifact-schema-law-source

It executed nine tests and 2,127 expectations before two focused controls failed. The following individual assertions passed against current sources:

- JSON fixture/schema strict Ajv admission, eleven exact owners, thirteen contexts and anonymous primary leaves.
- Installed TypeScript syntactic and semantic checks for all eleven owners.
- All moved declarations absent from root and real command/oracle consumers bound to actual owners.
- Static owner graph acyclic and free of root back imports.
- Live discovery agrees with independent fast-glob inventory; laws still return diagnostics without a frozen total; the ownership field oracle has no missing TypeScript declaration records.
- Bun package command, Nx target, package script, launch seed and derived launch route are registered.

The Nx target is @semio-tech/repo-lib:test-root-artifact-schema-law-source. Its project inputs include the root source, shared taxonomy, source-access owner, full artifact owner subtree, accepted field-discovery subtree, fixture/schema/test and field-parity oracle. Both launch files contain one matching launch record. I did not run Nx because the current direct control is red.

## Source Admission

Current source access has the needed boundary at:

/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts

policySourceAncestry checks every resolved ancestor using lstat. policySourceDirectory and policySourceText propagate missing, unreadable, linked and wrong-kind state. The legacy safe wrappers now return empty only for missing input and throw for the other states.

I built a ticket-private real filesystem tree with a linked ancestor. Current behavior was:

    policySourceDirectory(linked-parent/child) -> symlink
    policySourceText(linked-parent/child/source.ts) -> symlink, empty text
    policyDiscoverArtifactSchemaOwners with a linked ✏️s ancestor -> []

This independently verifies the current ancestor no-follow behavior. It does not use Nx, Cargo or external infrastructure.

## Findings Requiring Repair

### 1. The portable source-state controls are invalid after ancestor admission

The source-state test at:

/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts:125

supplies an operation stub whose lstat returns the leaf state even for /repo. With ancestry checks, a normal leaf now fails at its root ancestor as not-file before its intended final lstat state. The unreadable completeness control at line 167 likewise omits the directory chain and never reaches its unreadable leaf.

Repair the test controls only: make /repo and all declared ancestors directories, keep the intended final missing, unreadable, linked and wrong-kind state, and add an explicit ancestor-link row. Do not relax policySourceAncestry.

### 2. Owner discovery still falsely cleans unreadable discovery roots

I separately invoked current owner discovery and the aggregate with injected operations where /repo, /repo/✏️s and /repo/🧰️framework are directories but /repo/✏️s/🔌️plugins raises EACCES. Current output was:

    owners: []
    breachKinds: []

The directories helper in artifact owner discovery converts every non-directory state into an empty entry list. An unreadable scan root therefore produces zero owners and zero aggregate findings. This violates the source-evidence contract.

Repair the discovery boundary so missing roots retain their intentional-absence behavior while unreadable, linked and wrong-kind discovery roots/subtrees become source-unreadable evidence carried into the aggregate. Add a portable operation-injection control for the exact EACCES root case and preserve the actual ancestor-symlink control.

## Limits

No successful current direct, registered Nx, native Rust, full root verify, or remote graph claim is made while these focused controls are red. The live 192-owner/2,000-finding result remains a current semantic diagnostic, not a fixture snapshot and not a green conformance assertion. This audit made no product or Git change.

## Post-Repair Independent Evidence

Both reported defects are repaired in current source.

I reran the focused ordinary package route with a ticket-private artifact root:

    bun ./📜️script.ts test root-artifact-schema-law-source

Result: **11 passed, 0 failed, 2,146 assertions, 24.10 seconds**.

The route now validates full injected ancestry, missing/unreadable/non-file/final-link states, unreadable schema evidence, discovery admission failures, a private native unreadable source and a linked-parent refusal. It also repeats strict fixture/context/declaration checks, installed TypeScript diagnostics, no-root-import/cycle checks, live fast-glob parity without frozen counts, field-oracle behavior, and package/Nx/seed/generated-launch registration.

I independently reran two narrower probes after that route:

- A real ticket-private linked-parent tree returns symlink for both directory and text admission. Artifact discovery returns an empty owner list plus one symlink issue for ✏️s/🔌️plugins.
- Injected EACCES at ✏️s/🔌️plugins returns an empty owner list plus one unreadable issue, and the aggregate returns artifact-schema/source-unreadable. It no longer produces a false-clean empty result.

The discovery owner now returns typed owners and issues evidence. The aggregate renders each non-missing discovery state as a source-unreadable breach; missing remains intentional absence. This is the required separation and does not relax the source boundary.

## Remaining Evidence

The focused direct acceptance and independent probes are green. Sol is separately running the current field-parity, root-compiler and isolated registered Nx routes. I will add those exact registered results after they are available, without duplicating them.

## Final Registered Evidence

Sol’s final isolated routes complete the registered closure:

- @semio-tech/repo-lib:test-root-artifact-schema-law-source with cache skipped passed 11/11 and 2,146 assertions. Bun took 17.19 seconds and the Nx target 17.8 seconds.
- workspace:artifact-field-parity-test with cache skipped passed in 22.2 seconds against the live 192 owners.
- The direct field-parity route passed; root compiler passed 6/6 and 86 assertions; root import closure passed.

The live aggregate remains 192 owners, no discovery issues and 2,000 diagnostics: 74 diff coverage, 377 completeness, 1,363 field parity, 2 normative leaf, 83 state parity and 101 type-name parity. These are retained current law findings, not frozen fixture counts and not conformance-green claims.

**Acceptance:** accepted for the bounded artifact-schema extraction. The owner split, no-follow and unreadable admission, source-data consumers, direct route, current registered Nx route and direct root consumers are evidenced. No native Rust law was necessary for this TypeScript source/law extraction.
