# Complete Official Example Corpus Differential

Pinned [buildingSMART/IFC4.x-specification-models at ef22440ec501551ccc9689309560bbc7e26aa9d7](https://github.com/buildingSMART/IFC4.x-specification-models/tree/ef22440ec501551ccc9689309560bbc7e26aa9d7). All 46 IFC source files declare IFC4X3_ADD2. Original bytes total 1,781,339; the complete corpus contains 8,172 entities. Git blob identities, byte counts and SHA-256 checks were independently verified on download. Original headers were preserved.

## Executed Comparison

The owned parser, central no-op diff admission, writer and re-parser round-trip all 46 files exactly at the retained graph boundary. Semantic population evaluation reports no diagnostics for 45 models. It rejects one schema-domain error in the remaining model.

IfcOpenShell **0.9.0.post1**, with `express_rules=True`, independently parsed and evaluated all 46 original files. All entity counts agree. Forty-five have no diagnostics. Both implementations reject **entity #52 in basin-advanced-brep.ifc**, whose IfcPerson.Identification contains an integer instead of IfcIdentifier. The official fixture is retained unchanged, and its source error is an expected regression result. No independent oracle subprocess exceeded its 20-second per-model bound.

The source repository describes examples as informative and potentially outside particular model-view subsets. Thus these checks establish the stated syntax/editing/semantic comparisons, not certification of every MVD, informal proposition or geometric representation. The [source README](https://github.com/buildingSMART/IFC4.x-specification-models/tree/ef22440ec501551ccc9689309560bbc7e26aa9d7) records buildingSMART ownership and release/license terms; retained fixture documentation attributes the originals.

## Retained Regression Infrastructure

Production tests now have all 46 original files in IFC4x3 `🧪️fixtures/official`, a closed language-neutral `schema.json`, exact provenance and independent oracle results in `cases.json`, and attribution README. `🧪️tests/official-corpus.test.ts` checks the closed metadata contract, both hashes, byte counts, entity counts, central admission, round-trip identity and expected semantic outcomes. It is registered with the existing package suite router. Test-first execution failed on the missing corpus manifest before the original assets and independently checked baseline were added.

The retained corpus regression suite executed successfully: **47 tests passed, zero failed, 462 assertions**, in approximately 12 seconds.

A fixture-local `.gitattributes` disables IFC line-ending conversion, preserving the original source hashes on native Windows/macOS/Linux and devcontainer checkouts. The repository's global LF rule would otherwise normalize source CRLF bytes; its existing other-fixture exemption uses a different folder name.

Research inputs remain `official-corpus.py`, `official-corpus.ts`, `official-corpus-oracle.py` and `official-corpus-retain.py`. Downloaded research copies and verbose JSON/log results remain only beneath ticket `🗑️generated/corpus`; those outputs are temporary and must be removed when the whole shared ticket can close. Native corpus tests are requested from the native validation worker and remain subject to the authentic compiler-ingress limitation.

## Other Integration Checks

The canonical native program was regenerated after the normative operator-precedence correction. Its full-program SHA-256 remains `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`; retained official expressions are unaffected by those newly tested generic precedence cases. Rustfmt parsed/formatted the canonical data.

Strict checking of the public IFC4x3 TypeScript facade still reaches one unrelated missing framework package: `@semio-tech/framework-interaction` from framework local-interaction wire code. This ticket leaves the concurrent build refactor to its owners. IFC metadata now accurately distinguishes written/executed semantic capabilities from unverified native execution and unestablished complete MVD conformance.
