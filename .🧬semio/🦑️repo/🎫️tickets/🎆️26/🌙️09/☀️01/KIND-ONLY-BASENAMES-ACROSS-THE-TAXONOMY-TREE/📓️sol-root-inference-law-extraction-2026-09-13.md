# Root Inference Law Extraction

## Scope and result

This slice extracts the root repository inference discovery and law implementation into twelve anonymous TypeScript leaves under semantic discovery and schema-inference directories. The root command module now imports the aggregate inference law, the shared no-follow file walker, and the source-coordinate helper directly. It no longer declares any extracted inference implementation or compatibility facade.

The extraction registers fourteen semantic contexts, a portable schema/fixture/test contract, the exact Bun and Nx routes, and the seed and derived launch entry. The twelve-owner graph has 23 internal import edges, is acyclic, and has no owner-to-root import. The aggregate performs one family discovery capture and passes that captured family set to each law.

## Anonymous owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🚶️file-walk/🟦️.ts`
   - `PolicySourceWalkIssue`
   - `PolicySourceWalk`
   - `policyWalkRelFileSources`
   - `policyWalkRelFiles`
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📍️source-coordinate/🟦️.ts`
   - `policyLineOfIndex`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/🧱️contract/🟦️.ts`
   - `POLICY_INFERENCES_FACET`
   - `POLICY_DERIVED_MARKER`
   - `PolicyInferenceSourceIssue`
   - `PolicyInferenceFamilySource`
   - `PolicyInferenceDiscovery`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/🔍️family-discovery/🟦️.ts`
   - `PolicyInferenceSlugListing`
   - `policyListInferenceDirs`
   - `policyFindAllInferencesDirs`
   - `policyArtifactRootOfInferencesDir`
   - `policyDiscoverInferenceFamilies`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🚧️source-admission/🟦️.ts`
   - `policyInferenceSourceBreaches`
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🧩️family-root-completeness/🟦️.ts`
   - `policyInferenceFamilyRootCompletenessBreaches`
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🍃️slug-leaf-presence/🟦️.ts`
   - `policyInferenceSlugLeafPresenceBreaches`
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/⚙️derivation-presence/🟦️.ts`
   - `policyInferenceImplPresenceBreaches`
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/😀️emoji-uniqueness/🟦️.ts`
   - `policyInferenceEmojiUniquenessBreaches`
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🧶️assembly-coverage/🟦️.ts`
    - `policyInferenceNormalizeToken`
    - `policyInferenceAssemblyCoverageBreaches`
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/💾️state-separation/🟦️.ts`
    - `policyDerivedMarkerLeakBreaches`
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts`
    - `policyInferenceFamilyBreaches`

## Semantic contexts

The taxonomy registers:

- `repo-discovery-file-walk`
- `repo-discovery-source-coordinate`
- `repo-schema-inference`
- `repo-schema-inference-contract`
- `repo-schema-inference-family-discovery`
- `repo-schema-inference-laws`
- `repo-schema-inference-source-admission-law`
- `repo-schema-inference-family-root-completeness-law`
- `repo-schema-inference-slug-leaf-presence-law`
- `repo-schema-inference-derivation-presence-law`
- `repo-schema-inference-emoji-uniqueness-law`
- `repo-schema-inference-assembly-coverage-law`
- `repo-schema-inference-state-separation-law`
- `repo-schema-inference-law-aggregate`

Every implementation leaf is the anonymous `🟦️.ts` basename. The new portable artifacts use anonymous `🔣️.json` and `🟦️.ts` leaves.

## Consumer and source-data closure

`📜️script.ts` directly imports `policyWalkRelFiles`, `policyLineOfIndex`, and `policyInferenceFamilyBreaches` from their real owners. The aggregate imports the discovery result and seven laws. The laws import only the contract, source-admission helper, file-walk helper, source coordinate, and existing lower-level source-access or mutation-identity owners they use. No extracted owner imports the root command module.

The portable contract retains the actual Trinity flat-position inference implementation and its unit-test source as source data:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs`

The repository root compiler test compiles the current root through Bun and esbuild after the rebind. The focused source contract typechecks all twelve owners and checks the real root imports and removed declarations.

## Law behavior

- Missing optional inference facets remain absent without a breach.
- Present families require their root Rust and TypeScript leaves; missing, empty, and placeholder-only leaves breach completeness or implementation presence.
- Rust `pub fn` and TypeScript `InferredField<` implementations are admitted by the portable controls.
- Emoji uniqueness is scoped to one inference family. Missing emoji is medium severity and variation-selector drift remains low severity.
- Assembly coverage normalizes casing and separator differences across kebab, snake, camel, Pascal, and scalar spellings.
- `#[derived]` below snapshot state breaches separation, while the marker in inference implementation is admitted.
- Source walking does not follow a final or ancestor symlink. Missing optional roots remain empty, while unreadable, linked, or otherwise unavailable admitted sources remain typed evidence and become `inference-migration/source-unreadable` findings instead of a false-clean result.

## Test-driven evidence

The initial portable red phase produced 1 pass and 6 failures because the owners, root rebind, taxonomy contexts, and command route did not exist. The intermediate owner phase produced 5 passes and 2 failures until the readonly source type and route were completed.

Final focused evidence:

- Ordinary no-environment package route, after formatting: `env -u SEMIO_TEST_ARTIFACT_DIR bun …/📜️script.ts test root-inference-law-source` passed 8 tests and 122 assertions in 10.77 seconds. Its native hostile fixture observed a chmod-000 directory and a linked ancestor through typed source evidence and removed its temporary directory.
- Actual registered Nx route before formatting: `bun nx run @semio-tech/repo-lib:test-root-inference-law-source` passed 8 tests and 122 assertions. The selected target took 5.2 seconds; outer project-graph setup took about 66 seconds and Nx skipped the cache.
- The only changes after that Nx pass were Prettier formatting of the new owner, schema, fixture, and focused-test leaves. A post-format direct pass and `bunx prettier --check` over those leaves establish the final formatted behavior and style. A post-format Nx attempt failed before target execution because an Nx JavaScript plugin worker exited during concurrent graph activity; it is infrastructure non-evidence and was not retried solely for formatting.
- Root import: `bun -e 'await import("./📜️script.ts")'` completed and printed `[DEBUG] root import ok`.
- Workspace taxonomy load completed with 541 contexts at that checkpoint.
- Root compiler: the current `⚙️root-script-compiler` test passed 6 tests and 93 assertions, including Bun and esbuild compilation and its eager vocabulary/Rust/Protobuf semantics.
- A read-only live aggregate returned 1,277 current diagnostics in 20.87 seconds: 356 assembly, 380 emoji, 59 family-root-completeness, 243 implementation-presence, and 239 slug-presence. It returned no source-unreadable or state-leak findings. These counts describe existing migration debt; they are not expected counts or suppressions.
- Independent Terra acceptance passed the ordinary package route with 8 tests and 122 assertions in 5.99 seconds and confirmed twelve owners, fourteen contexts, no root facade or back edge, meaningful source-admission/assembly/state controls, and correct registration. Its retained delta is in `📓️terra-root-inference-law-pre-extraction-audit-2026-09-13.md`.

## Native limit

An actual Nx Trinity native attempt remained in project-graph construction for more than six minutes during unrelated concurrent tasks and never reached Cargo; the owned process was interrupted without a shared reset or broad process kill. A second selected package-router attempt used `CARGO_NET_OFFLINE=true`, `--locked`, and the exact libtest filter `flat_position_bfs_walks_from_root`. It remained in Nextest's `cargo test --no-run` warm-build phase behind concurrent Cargo work for 5 minutes 49 seconds and never began an assertion. The owned process group was terminated precisely after the PTY interrupt left its two children orphaned. This is queue-bound non-evidence; the permanent source-data control still confirms the concrete Trinity implementation and unit-test leaves, but this run does not establish their native runtime behavior.

## Exact attribution

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🚶️file-walk/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📍️source-coordinate/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/🧱️contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/🔍️family-discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🚧️source-admission/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🧩️family-root-completeness/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🍃️slug-leaf-presence/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/⚙️derivation-presence/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/😀️emoji-uniqueness/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/🧶️assembly-coverage/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/💾️state-separation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-inference-law-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-inference-law-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-root-inference-law-extraction-2026-09-13.md`

Updated:

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

No Git lifecycle, AGENTS, live cleanup, taxonomy apply, scaffold publication, or runtime dependency change belongs to this slice.
