# Norm Document Contract Ownership

EN 1990 `qK` and DIN 18599 `climate` are composed child identities. Their native artifact, snapshot and diff owners already use Store child handles, but non-Rust facets still described inline arrays, list wrappers or strings. Several diff facets also kept a whole-artifact replacement field absent from the actual mutation vocabulary.

The three JSON Schema, TypeScript, GraphQL and Proto facets per artifact now use the shared Store `ArtifactChild` owner. Duplicate TypeScript `ArtifactDialect`, `ArtifactRef` and `ArtifactChildHandle` declarations were removed. Diffs preserve native nullable optional-field semantics and reject whole-artifact replacements; the existing Rust schemas are unchanged. The TypeScript full/sparse parsers validate one shared domain field vocabulary, and snapshots reuse the artifact parser.

The language-neutral fixture covers both documents, invalid child identities and process-local child fields, unsigned integer bounds, enum choices and misplaced editor fields. The new root/Nx `norm-document-contract` command is registered in launch.json. It reuses the shared document-contract testkit and independently compares production TypeScript parsers with Ajv, including the committed native mutation fixture corpus.

`norm-document-contract-red-3.log` is the clean expected baseline: Ajv rejected a valid EN 1990 child handle because the old artifact schema required an array. Earlier red-1/red-2 attempts failed in command setup/import resolution and are not feature regression evidence. `norm-document-contract-green-1.log` is GREEN, Nx exit 0 after 3.5s: EN 1990 matched 20 native snapshot fixtures and six committed diffs; DIN 18599 matched 26 snapshots and twelve diffs, plus independent rejection vectors. This is runtime evidence for the TypeScript/Ajv contract boundary, not a new native Rust execution result.

The field-parity refresh and focused TypeScript static checking remain pending. Source review also corrected DIN 18599 Proto acronym names to `internal_gains_w_m2` and `reference_q_p_kwh`, preserving the native JSON field spellings.

## Completed Contract Verification

`norm-document-contract-green-3.log` is GREEN including strict TypeScript checking of all six changed production facets and the oracle entry point. The check uses `skipLibCheck` for dependency declarations; all selected source files are checked. Runtime fixture/Ajv checks remain green. Nx exited 0 after 12.4s. The new test now uses standard `fileURLToPath`/`dirname` rather than a Bun-only `ImportMeta.dir` typing.

`artifact-field-parity-norm-8.log` completed in 29.5s and reports 36 remaining mismatching representations, with no EN 1990 or DIN 18599 rows. Reporting success is not a green repository-wide parity gate.

## Concurrent Fixture Taxonomy Change

After GREEN3, the independent fixture organization work moved native mutation inputs from each mutation's tests directory to the standard/subset `🧫️fixtures/🧬️mutations` owner. GREEN4 correctly failed its corpus count (zero discovered instead of 20/6), while contract vectors still ran; this was not a URL-import failure. The Norm oracle now addresses the verified canonical fixture directory directly. GREEN5 is pending. No old-path fallback or duplicate fixtures were introduced.

## Final Current Validation Receipt

`norm-document-contract-green-5.log` completed successfully through Nx in 8.0 seconds after the concurrent fixture relocation. Production parsers plus Ajv validated EN1990: 20 snapshots and 6 diffs; DIN18599: 26 snapshots and 12 diffs. Strict TypeScript passed. The parity report completed with 36 remaining findings across other families and no EN1990 or DIN18599 findings; reporting success does not constitute a clean global enforcement gate.
