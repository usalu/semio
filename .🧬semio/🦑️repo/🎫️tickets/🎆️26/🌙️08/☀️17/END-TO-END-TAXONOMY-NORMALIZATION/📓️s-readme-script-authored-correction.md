# README Script Authored Correction Authority

## Read-only finding

The repo-library TypeScript README still contains guidance for the retired ASCII root script alias. No README, frozen catalog, historical plan, or AGENTS file was edited during this check.

Historical source: [the frozen owner catalog's `/cases/28/sourcePath` field](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️readme-license-owner-authority/🔣️.json#/cases/28/sourcePath).

Authorized leaf destination: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📃️readme/📝️.md`.

The current source exactly matches its frozen preimage: regular file, mode `0644`, 2,395 bytes, SHA-256 `42106d6ca4234c9a6aec8805bb98e1f77f593b278b189e606fed70a4a3f7a71b`. The unchanged 40-leaf catalog SHA-256 is `051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4`.

Exact source lines, UTF-16 offsets, byte offsets, context preimages, and proposed postimage hashes are retained in `🧪️root-script-alias/📇️readme-authored-correction/🔣️.json`. Offsets are zero-based, half-open. The report's line numbers are one-based.

## Exact correction spans

Every proposed replacement in this table changes only `script.ts` to `📜️script.ts`.

| Source line | UTF-8 byte span | Role |
| --- | --- | --- |
| 5 | 101–110 | Bundle-router filename heading |
| 7 | 171–180 | Bundle/workspace router filename rule |
| 25 | 1006–1015 | Explicit workspace-root `/script.ts` |
| 25 | 1096–1105 | Explicit “see root” filename |
| 31 | 1750–1759 | Policy filename heading |
| 33 | 1879–1888 | Bundle policy filename |
| 34 | 1971–1980 | Folder policy filename |
| 39 | 2088–2097 | Policy command placeholder |
| 40 | 2141–2150 | Lint command argument placeholder |
| 43 | 2373–2382 | Nx command placeholder |

The two line-25 spans are the direct root-alias guidance. Applying only those two to the frozen source would produce 2,409 bytes, SHA-256 `fa65ea3e015f223fe6cd8b12f6fb3869a820ae2a08336e858f32287e6ecc42b2`.

Applying all ten exact filename spans would produce 2,465 bytes, SHA-256 `6e41b7894232a49ba2432ddfbda7e2669cc27b06b8fab7a058fc8952af2bf3f9`. These are proposed-output calculations, not assertions that edits were applied or that the whole README is current.

Do not replace the suffix in `./src/bundle-script.ts`; that is a different imported leaf, not the retired alias. Existing `**/📜️script.ts` and canonical execution examples stay unchanged. The old `repo/lib/js/bin/lint.ts` entrypoint on line 40 and older API/bootstrap guidance require their own verified command/path correction; changing the argument filename alone does not validate that command.

## Transaction integration boundary

The existing `readme-license-owner-leaves-v1` authority owns the move and frozen source bytes. It does not own arbitrary authored prose changes. Its Markdown relative-reference adapter resolves actual Markdown path references, while `exactOwnedReferenceTokens` currently implements only the declared Go and Rust README-reader forms. Inline code and prose naming the removed root alias are not a currently registered content rewrite. Generic incoming-reference closure cannot infer an alias move that never exists in this README transaction.

Therefore the existing frozen leaf catalog and its 62 reference bindings must remain intact. A new, separately declared exact authored-document correction authority is needed before this content edit can enter a valid plan. That authority should bind:

1. This exact source owner and authorized destination, activated only by its README owner move.
2. The existing frozen source hash, mode, and size; not a replacement “current” preimage.
3. The selected exact spans, old values, complete source-line contexts, canonical replacement values, and expected resulting hash/size.
4. A distinct rationale and fail-closed treatment of missing/repeated/overlapping spans, changed source bytes, an unregistered owner, or an unexpected destination.

The normalizer can then emit those declared tokens as existing `exact-owner-reference` edits targeting the README's own source identity. The current `buildReferenceEdits` path records the final destination, original file preimage, structured location, old/new values, edited result hash/size, and owning move. Existing edit backup, projected generator preview, rollback, and apply preimage handling can carry the move and content correction in the same transaction. This requires a schema/parser/token-production change with tests; it cannot be achieved by hand-editing a retained plan or injecting unregistered edits after planning.

The focused tests should prove the language-neutral span application against an independent parser/reference implementation, unchanged 40-leaf/62-binding authority, exact source-scope inclusion, occupied/drift rejection, move-plus-edit rollback/retry/commit, and an empty second plan. No production edit is authorized by this report alone, and no golden should be refreshed merely to accept a changed README.

## Release

This audit and its proposed-span evidence are released to the coordinator. The source README and catalog are unchanged. Fresh CAD/Draw captures remain the separate priority lane; this report is not their apply-readiness result.

## Implemented exact correction authority

The coordinator authorized the bounded implementation after the read-only audit above. The original 40-leaf catalog, its 62 reference bindings, and the README source preimage remain unchanged.

The exact owner projection now has a separate `authoredDocumentCorrections.repo-library-script-filename-v1` schema declaration. It binds all ten filename spans above, source and destination owner identities, the original preimage, the exact resulting hash/size, the existing canonical `root-script` filename contract, and activation only by that owner's README move. The declaration is not a replacement golden or a generic basename rewrite.

A shared closed parser and pure authority resolver reject unregistered owners, altered source bytes/mode, wrong destinations, omitted/repeated/overlapping spans, UTF-8 boundary errors, changed line contexts, and postimage drift. The resolver produces UTF-16 offsets only after verifying the byte-bound source. Inactive/unrelated owners produce no edits. The existing normalizer reference-edit machinery owns the final-path edits, original preimage, result hashes, projected preview input, edit backup, rollback, and retry.

The runtime test found and closed two additional integration edges:

- Generic incoming discovery initially proposed an eleventh edit changing the new schema's historical `sourcePath`. The existing frozen-coordinate mechanism now recognizes only that exact schema pointer after comparing the complete loaded correction row and matching it to the frozen owner/preimage. It is not a blanket schema or Markdown exemption.
- Markdown relative rebasing initially reassigned the exact authored tokens to a nonexistent owner-local ASCII script path. Exact-owner-reference tokens now bypass that relative-link adapter; all ten edits attach to the actual README move.

## Test-first and runtime evidence

Language-neutral vectors are retained in `🧪️readme-authored-correction/🔣️.json`; the focused implementation tests are `🧪️readme-authored-correction.test.ts`.

The initial pure gate was **0 pass, 2 fail, 5 assertions, 107 ms**: the schema field and resolver did not exist. After implementation, the pure gate was **2 pass, 0 fail, 48 assertions, 678 ms**. Ajv independently validates the declaration, and MarkdownIt independently finds the inline/fenced code contexts and produces the exact same 2,465-byte postimage and SHA-256.

The isolated lifecycle first exposed the extra schema-coordinate edit, then the missing move attachment. After both fixes it passed **1 pass, 0 fail, 20 assertions, 3.30 s**. Its observed runtime sequence was move plus ten edits, injected failure after edits, exact source-byte rollback, same-ticket retry, commit, corrected-content/hash/mode check, and an empty replan scoped to the canonical leaf. The observed successful fixture is retained at `🧪️readme-authored-D2jdvQ`.

The final complete packet, with temporary diagnostics removed:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-authored-correction.test.ts'
4 pass, 0 fail, 76 assertions, 4.07 s
```

The recorded invocation used an explicit `./` relative test path, not an absolute path. That historical command is retained as a safety deviation from the coordinator's absolute-path requirement; it is not relabeled after the fact. All subsequent test invocations in this lane use absolute paths. The focused file selection and reported assertions are unchanged.

The two adjacent exact-owner registration/count tests also passed: **2 pass, 0 fail, 17 assertions, 233 ms**. The final packet covers occupied destination and source drift without writes. All fixture trees and failed-run evidence are retained inside the ticket. The tiny fixture intentionally omits unrelated registry and graph producers, remapping only those input selectors to inert exact fixture paths; no production generator was run. Actual owner-batch planning still uses the unchanged live generator authority.

## Implementation release

Production changes are limited to the exact-owner schema declaration, discovery types/parser/resolver, the exact-owner token producer and its argument, the exact Markdown adapter boundary, and the narrowly bound schema source-coordinate recognition. No source README, original catalog, historical plan, AGENTS file, or production leaf was moved or edited. No runtime dependency or permanent script was added. No test process remains from this packet.

## Follow-up incoming-evidence gate

The four-case runtime packet above does not by itself authorize a live README apply. A fifth test now includes the outer language-neutral vector in the fixture's complete incoming-reference set. It reproduced an unwanted eleventh edit to the vector's historical `/authority/sourcePath`: **0 pass, 1 fail, 2 assertions, 2.80 s**. The vector bytes remain SHA-256 `35db5a0ae52f6e4779453782708ee65efbda63e40439bc1124e022f78cd183b0`, size 7,519.

The coordinator authorized preparation of a bounded digest-and-typed-pointer evidence registration, shared with the CAD/Draw frozen catalog lane. Proposed row and negative cases are retained in `🧪️readme-frozen-coordinate-authority/🔣️.json`. Actual taxonomy/discovery shape edits are held while the coordinator's unrelated production transaction runs. This fifth test is intentionally red until that exact frozen-coordinate authority is implemented; there is no blanket fixture exemption.

The earlier proposed-span audit is not being reclassified as immutable authority. Its duplicated source literal was replaced with an explicit `historicalSourceCoordinate` reference to the unchanged owner catalog, its digest, and `/cases/28/sourcePath`; its source preimage, ten spans, and proposed postimages were preserved. The authored Markdown source identity above uses the same explicit historical link. No evidence was compressed or removed to conceal the unresolved vector boundary.

The pending packet now has nine tests. The shared registry declaration test was run with the absolute file path and failed as expected because `frozenCoordinateEvidenceContracts` has not landed: **0 pass, 1 fail, 1 assertion, 462 ms**. Additional prepared cases require registered digest drift to abort before writes, a digest-rebound unowned source-coordinate field to block without rewriting the bound document, and an unregistered neighboring document to remain an ordinary rewritten consumer. Those new runtime negatives have not yet been run; the active production transaction's schema hold remains in force.

The shared pure resolver can already be tested with an explicitly supplied proposal, without schema wiring or filesystem discovery. That absolute-path focused test passed **1 pass, 0 fail, 6 assertions, 512 ms**. Its two declared source/destination value spans exactly match `jsonc-parser` offsets and values. An unregistered neighbor returns no frozen authority; changed document bytes, schema version, and a missing pointer each fail closed. This pure result does not replace the pending full incoming-reference and transaction gate.

## Registered incoming-evidence release

After the coordinator's unrelated transaction committed, the shared frozen-coordinate registry was wired and the exact README vector row was inserted. Fresh `loadTaxonomy` plus `validateTaxonomy` returned an empty violation list. The row preserves the vector's unchanged SHA-256 and owns only `/authority/sourcePath` and `/authority/destinationPath` with explicit source/destination roles.

The full nine-case packet was run with an absolute test path:

```text
bun test '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-authored-correction.test.ts'
9 pass, 0 fail, 109 assertions, 31.92 s
```

This includes both isolated move/edit rollback→same-ticket retry→commit→empty-replan lifecycles, the registered historical vector in the complete incoming set, exact schema/Ajv and JSON-span/parser parity, digest drift rejection, unowned source-coordinate rejection without rewriting the registered document, and ordinary rewriting of an unregistered neighboring document. The previously red fifth and sixth tests are now green; the earlier red results above remain historical evidence. Fixture evidence is retained, no production move/generator ran, and no process remains from this packet.
