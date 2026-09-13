# HTML Before/After Fixture Pre-Audit

Date: 2026-09-13  
Status: **read-only preparation; no fixture move or runtime execution claimed.**

## Current Authority

The 16 target leaves are eight exact before/after pairs beneath `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧫️fixtures`:

- `📜set-doctype-applied`
- `➕insert-node-applied`
- `➖remove-node-applied`
- `🏷️set-element-name-applied`
- `🔖set-attribute-applied`
- `✍️set-text-applied`
- `💬set-comment-applied`
- `⌨️set-raw-text-applied`

Each currently contains `⬅️before.html` and `➡️after.html`. The only active path-and-byte authority for those 16 leaves is the owning subset's `🔮️oracles/🔣️.json`, in its `fixtureManifests` records. Each record names both relative paths, their `expected-before-html`/`expected-after-html` roles, media type, byte count, SHA-256, third-party generator provenance, and target coordinates. The native oracle unit instead embeds the separate real page `🧫️fixtures/🏚️zukunft-bau-entwerfen-mit-bestand/🌐️.html`; it provides no direct control for these 16 bytes.

## Current Consumer and Route

`loadOracleRegistry` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:955` admits the subset contribution. The exported `fixtureManifestProblems` and `verifyFixture` at lines 3624 and 3694 validate provenance and re-hash every declared file. `validateAllContracts` calls both at lines 6771-6779, turning a missing or digest-mismatched file into a testing/fixture breach. The test-domain `fixtures` command also calls `verifyFixture` at `📜️script.ts:1112`.

The narrow existing package route is the `TestScript` bundle in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts:14`, which includes `🧪️tests/🧬️mutation-fixtures/🟦️.ts` alongside platform, schema, layout, and fixture-resolution controls. It is a broad test-domain route, so it should not be rerun merely for a future 16-leaf move. A dedicated portable manifest control should invoke the existing validator and reader over only these eight records.

## Taxonomy Boundary

The established before/after directory contexts are `📸️snapshot/⬅️before` and `📸️snapshot/➡️after`: taxonomy contract `mutation-fixture-bundle-v1` classifies them as `comparison-before` and `comparison-after`. That contract requires JSON leaves plus mutation, outcome, and diff nodes, so it cannot simply be asserted over these raw HTML source leaves. The new fixture shape needs an explicit HTML source-fixture context that preserves the existing generic `fixtures` owner, admits anonymous HTML leaves, and distinguishes the two sides without pretending that they are mutation-runtime snapshots.

Acceptance should require all 16 source paths to migrate as data selected by the unchanged fixture manifests; their role, digest, bytes, and relative relationship must survive. It should run the manifest-reader/hash control and an installed third-party HTML parser control separately. Native oracle tests for the unrelated real page and the full mutation simulation cannot substitute for either result.

## Post-Move Context Closure Review

The member registry can legally serve as the root of an exact descendant contract: `projectionDirectoryKinds` combines global, owner-local, and projected directory kinds in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:4229`, and the descendant validator accepts any of those roots at line 4338. There is no existing descendant contract rooted at `stdio-html-mutation-fixture`.

The current `mutation-fixture-bundle-v1` is not reusable. Its fixed twelve-node grammar includes mutation, outcome, diff, and JSON leaves (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:4379-4382`), which would misrepresent the raw HTML source pair.

A precise `stdio-html-source-fixture-pair-v1` contract can instead root at `stdio-html-mutation-fixture` and require exactly these six realized nodes: the root directory, `📸️snapshot`, both `⬅️before` and `➡️after` directories, and `🌐️.html` below each side. It has no alternatives and a derived longest-suffix reserve of 42 UTF-8 bytes. This binds the directory grammar without conflating it with mutation-runtime data.

At the time of this review, the member registry holds the eight physical names but five noncanonical emoji spellings resolve to no semantic member because `semanticDirectoryKindId` canonicalizes an extended-pictographic input to VS16 before comparing it to raw member names (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1970-2000`). The affected live names are `➖remove-node-applied`, `🔖set-attribute-applied`, `💬set-comment-applied`, `➕insert-node-applied`, and `📜set-doctype-applied`; the other three already resolve. The coordinator chose the narrow, policy-consistent repair: canonicalize those five physical parent names and rebind their manifest/control paths, rather than broaden local-member identity matching. That is pending fresh verification.

`fixtureManifestProblems` only validates field shape and `verifyFixture` joins the declared relative path then hashes it (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:3624-3698`). Neither establishes this descendant grammar. The acceptance control must therefore prove all eight canonical parents resolve to the member kind, an unknown fixture parent does not, each parent realizes the six-node source-pair contract, and each manifest role selects its matching side before the existing digest and independent HTML-parser checks run.

## Current Independent Static Closure

After the coordinator's canonical-name and primary-file-rendering repair, `loadCatalogTaxonomy()` succeeds. I independently enumerated the Stdio fixture root and found exactly the eight member names declared by `stdio-html-mutation-fixture`; each resolves to that kind below `fixtures`. A random `🧭️unknown-fixture`, the unrelated real-page fixture, and a valid Stdio name below the wrong `tests` parent all resolve to `null`.

`stdio-html-source-pair-v1` now has the intended root, six required nodes, no alternatives, and a 42-byte reserve. Its file nodes use the registered `html` kind. `semanticDescendantNodeRelativePath` now uses the schema-ordered primary filename renderer at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:2772-2778`; the focused source test checks `🌐️.html`, `🔣️.json`, and rejection of an unknown kind.

`🌐️html-source-pair` is a separate, exact `html-source-pair-control` member used by the test-domain fixture at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🌐️html-source-pair`. It does not resolve to `stdio-html-mutation-fixture`. That fixture explicitly fixes the Stdio member kind and descendant contract, and the source test rejects an unregistered parent. It is therefore a test-control identity, not a broad admission of production HTML fixture parents.

The Stdio manifest binds each record's `expected-before-html` and `expected-after-html` roles to the matching canonical snapshot side. The source control checks exact realized paths, no symlinks, two manifest files, existing `verifyFixture` digests, byte counts, SHA-256 values, and parse5 parse/serialize idempotence. Root's direct 10/203 and 16/16 parse5/immutable-byte evidence, plus its in-progress registered fixture-verification route, remain distinct from this static observation.

## Acceptance

**Accepted for the bounded 16-leaf move.** My independent static closure confirms exact eight-member admission, six-node source-pair grammar, `html` primary leaf selection, role-to-side manifest binding, and test-control separation. The coordinator's current runtime evidence is the scoped registered `@semio-tech/repo-test-domain:test-fixture-verify --args='--artifact s.stdio.html --standard 5 --subset any'` result: exit 0, eight fixtures, zero file problems, cache skipped, 45.7-second critical task / 46.7-second Nx run (223.49 seconds including graph setup). It is distinct from the direct full mutation-fixtures result, 10/203 in 9.02 seconds, and the 16/16 immutable-byte/parse5 baseline over 1,497 bytes.

The renderer repair uses `canonicalPrimaryFilenameForKind`; the coordinator's focused impact probe reports ten existing file nodes unchanged and only the two new HTML contract leaves newly rendered. The accepted evidence does not claim a mutation simulation, a browser execution, or native HTML oracle coverage for these sixteen source leaves. The relevant parser evidence is the focused parse5 control. No generic fixture or HTML exemption was found.

## Final Nx Input Closure

I independently parsed both current project manifests. Each has an identical `htmlSourcePairs` named input containing exactly 17 existing source inputs: the Stdio oracle manifest plus every one of the 16 canonical HTML leaves. The test-domain package manifest binds it to its four package test targets (`test`, `test-quick`, `test-long`, and `test-exhaustive`); the test-domain project manifest binds it to `test-fixture-verify`. This is the exact producer/consumer closure for the manifest reader and fixture verifier, with no Stdio/root project attribution.

The coordinator's installed Nx `createNodesV2` probe reported five target projections and the same 17 inputs in 2.36 seconds, retaining the original commands and working directories. The portable control covers both consumers. The final direct control is 11/210 in 2.89 seconds. These results extend the preceding acceptance without claiming another registered execution run.

## Normalization reader repair delta

The HTML pair exposed a separate generic reader defect: normalization rendered a descendant file through the one-extension-only canonical filename helper, while the schema declares both .html and .htm. The current reader uses canonicalPrimaryFilenameForKind, so it follows the schema-ordered authoring leaf without changing the one-extension helper.

I re-read the actual control. It scopes inventory to the fixture root, filters only the eight declared owner prefixes, and requires the exact sixteen file paths. It then requires each leaf to retain file kind html, identity normalizedPath, and no violations. This prevents the earlier overbroad sibling population while retaining the full declared set inside every owner.

The portable Nx closure now distinguishes fixture bytes from reader code: both consumers keep the exact 17 htmlSourcePairs inputs; the four package test targets additionally include normalization, discovery, and taxonomy, while the fixture-verification target needs no reader-code input. The control verifies each declared source input against every target that reads it.

Coordinator evidence for the repaired current sources is direct 12/271 in 26.79 seconds and package-body-policy 121/530 in 68.16 seconds. The registered reader route is still pending at this audit point, so this delta is not a second registered acceptance claim.

## Reader Nx progression

The first current package-policy Nx run reached 120 of 121 tests and 530 assertions in 133.54 seconds (2 minutes 15 seconds Nx). All three HTML cases passed. Its sole nonzero result was an unrelated native Go bootstrap case exceeding the former implicit five-second per-case budget at 5.84 seconds; its assertions were not relaxed. The coordinator assigned native-oracle cases an explicit 30-second bound, retained five seconds for TypeScript cases, and set the total route budget to 240 seconds from the observed direct duration. The scoped HTML registered route is still running, so this remains progress evidence rather than a final registered result.

## Registered normalization-reader acceptance

The exact registered long route is now green: @semio-tech/repo-test:test-long with -t HTML completed with four passing tests, 256 assertions, and 325 filtered tests in 86.74 seconds; Nx took 1 minute 28 seconds with cache skipped. The installed Nx graph independently confirms four package test targets, three reader inputs, the 17-pair input set, and cacheability.

The short route's 30-second outer cap was reached in an unrelated test-platform module-scope contribution scan before any HTML assertion. It is not HTML reader evidence and did not result in a route, assertion, or budget change. The preceding fixture verifier and this long-route result together accept the bounded HTML source-pair and normalization-reader closure, with the parser/browser/native limits already stated.

## Final package-policy result

The coordinator's final current `package-body-policy` Nx route is green: 121 tests and 530 assertions in 127.51 seconds of test work and 2 minutes 8 seconds through Nx, cache skipped. It postdates the explicit 30-second native-oracle bound and 240-second route bound; the three HTML controls pass within that route. This is whole-policy corroboration, separate from the focused fixture verifier and HTML long route above. No browser execution or mutation simulation is implied.
