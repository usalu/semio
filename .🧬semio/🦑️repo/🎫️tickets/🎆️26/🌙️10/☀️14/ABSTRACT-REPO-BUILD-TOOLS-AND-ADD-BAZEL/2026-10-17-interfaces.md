# Neutral Caller Integration Audit — 2026-10-17

## Scope and Result

Nx remains the main backend. This subtask qualifies the neutral normalization command grammar and read-only dashboard/editor/agent projections only. It does not qualify public Nx/Bazel execution, current SDK admission, Cargo, WATCH, RESET, protected Native custody, physical Windows/Linux, or any backend switch. No Native/cache storage code was changed.

Read the existing final report and current public Bazel qualification plan before implementation. The parent owns the reopened ticket, core interface, public runtime sequence and completion decision.

## Production Correction

`TaxonomyRegeneration.command` and `.verifyCommand`, the plan producer and the schema-owned execution guard already require `bun repo run <target>`. The deserializer still required `bun nx run <target>` in two guards. Such a serialized neutral plan could be authored but could not be read back. Both guards now require `repo`, with no compatibility fallback for Nx/Bazel command namespaces.

Added `RegenerationCommand` to the existing normalization schema before changing production. A language-neutral JSON fixture pins eight command vectors independently for each execution/verification field: neutral selection, direct Nx/Bazel leaks, wrong runtime, wrong operation, extra argument, missing target and non-string target. Ajv checks the command contract, while fast-json-stable-stringify and Node SHA256 independently derive the full regeneration and plan identities. The final test imports and runs the actual production normalizer, round-trips complete parser-only plans and never applies them or materializes their synthetic paths.

The dedicated owner is `@semio-tech/repo-lib:test-regeneration-commands`, routed through the existing package `📜️script.ts test regeneration-commands`. English/German editor entries in launch and seed issue `bun repo run @semio-tech/repo-lib:test-regeneration-commands --skip-cache`, adjacent to the existing taxonomy authority gate, group `4_gate`, orders `410.1971` and `410.1972`. A TypeScript AST/JSONC test independently verifies the owner declaration, route and both editor documents. No new script filename or runtime library was introduced.

## Caller Audit

Dashboard Rust target/group/playground execution uses `bun repo`; its authored tools preserve their own declared command vectors. Dashboard launch projection also uses `bun repo` and does not normalize unrelated serialized backend-looking words. CI workflow build/test/publish selections use `bun repo`. Current checked-in agent MCP clients use the proper non-build root `📜️script.ts dev mcp stdio` bootstrap. Those production callers needed no broad rewrite.

The existing agent fixture suite could not start because it imported the unavailable second TOML oracle `toml`, then because it read a retired fixture location. It now uses the installed independent `smol-toml` and builtin Bun TOML parsers, and reads the existing fixture from `🧪️tests/🧫️fixtures`. JSONC and Ajv oracles remain. Its existing schema-enum assertion was widened to `string[]` for Bun's typed equality overload; equality semantics are unchanged. The fixture documentation link was corrected.

## Actual Failure and Test Provenance

1. An initial relative `bun test` command without `./` was interpreted by Bun as a filename filter. One invocation crashed Bun v1.3.14 at 1.98 GB RSS, exit 133; a second reported that 7,579,864 files were searched and no test matched. These are invocation failures, not normalizer-import failures or test acceptance. Explicit `./` paths fixed the invocation. No Bun bug report was sent.
2. Before the two production guards changed, an exact-Source parser test using both Bun and TypeScript compilers produced the real RED: `taxonomy plan regenerations[0].command is invalid`, 0 pass/1 fail, 1 assertion, 1.199 s. After the correction the Source vector test passed 32 execution/verification/compiler combinations. That temporary containment in the README Source suite was removed completely.
3. The broader README Source suite at that checkpoint reported 14 pass/4 fail/709 assertions. Three failures concerned the existing current-source authority schema versus historical fixture kind/role; the fourth expected the retired `nx` package/executor registration instead of `repo`. These were not repaired or masked in this subtask. No final README or transaction-v2 test edits remain.
4. The corrected explicit-path actual transaction test imported the production normalizer and passed the 16-vector regeneration regression, 1 pass/34 assertions/752 ms. The test was subsequently moved into its dedicated permanent owner without retaining unrelated transaction-suite edits.
5. The first combined dashboard/agents run passed dashboard tests but failed to load agent tests for missing `toml`. The next passed dashboard/regeneration but failed on the retired agent fixture path. Both concrete test blockers were corrected as described above.
6. Initial agent strict TypeScript returned 2 for the preexisting typed equality overload on the schema's `string[]`. After the safe type widening, final agent strict returned 0. Strict TypeScript of the dedicated regeneration test, including its actual production-normalizer import, also returned 0. Both used `--noEmit --strict --noUncheckedIndexedAccess --skipLibCheck --target ES2022 --module ESNext --moduleResolution bundler --allowImportingTsExtensions --resolveJsonModule --esModuleInterop`.
7. Final focused integration run after editor grouping changes returned 0: **25 pass, 3 filtered out, 0 fail, 634 assertions, 7.40 s**, three files. It includes 2 regeneration/parser-registration tests, 4 dashboard launch tests and 19 read-only agent tests. Existing scratch-workspace tests were explicitly excluded because their tmpdir allocation is outside this ticket; they are not claimed passing. Earlier identical combined scope passed in 1.377 s; no performance causality is claimed.
8. Final scoped `git -c core.fsmonitor=false diff --check` returned 0. The read-only diff confirms no remaining edits to README Source or transaction-v2 tests. No modifying Git command or worktree was used.

Final runtime diagnostic frames included:

```text
[DEBUG] Neutral regeneration parser matched Ajv for 16 execution/verification vectors and independent canonical SHA256
[DEBUG] Neutral regeneration gate matches project/script ownership and both EN/DE editor documents
[DEBUG] dashboard launch commands=2 locales=en,de peerPreserved=true idempotent=true
[DEBUG] dashboard current declaration schema=true tools=496 launchConfigurations=992 BuildRouting=true
[DEBUG] Actual dashboard projection agrees with independent RFC6902 identity and leaves literal serialized backend labels/choices unchanged; AST has no compatibility normalizer
```

Final test command (from repository root; the `./` prefixes matter):

```sh
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️regeneration-commands/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🚀️launch/🟦️.ts' './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤖️agent-client-configuration/🟦️.ts' --test-name-pattern 'neutral regeneration|dashboard|parameter values|launch publication|derivation from a fixture|launcher a credential|the registry schema|the checked-in client|current ticket declarations'
```

## File Inventory at Subtask Completion

These hashes bind the observed bytes at this checkpoint only. Other collaborators may subsequently change shared files. All listed repository-relative paths are below 256 characters; longest absolute path is 128 characters.

| Path | Bytes | SHA256 |
| --- | ---: | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` | 894023 | `a6ae3afd180f70425dd6c508bd9774147cc54d8dc72daf7fe04dea9163d4ae75` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json` | 9021 | `6bbd27baa0b6d0426f3dea6dfb5b81ab6b5f838f12bfd1900fcea251d8944190` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️regeneration-commands/🟦️.ts` | 4894 | `bdbce76790b27aaed6a512d8f1c7aa29be2c914efb08213180899896c619de87` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🏗️regeneration-commands/🔣️.json` | 1738 | `0abc94fb335cb587a94191abd2971677b47e1b36dc19a72e1d9303deb64159f8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤖️agent-client-configuration/🟦️.ts` | 9487 | `3219caf4bff4d3479753e896fc49e8594ecf58b07985cf351fb5429432400263` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` | 102695 | `e380df0f7331b98d0a1bb5bf11b0a3b46c6f84787bb0558078cad71b935d4dfe` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | 153272 | `73e6186e9996ef0fa419614643f2dc903b56a2ff6d2e6f29ae05ffa5241c896e` |
| `.vscode/launch.json` | 8806156 | `1ca4d0ee0254ed86eed25365a740f3839c3be9ee7496f1c97fc2d05c3d94c714` |
| `.vscode/🧩️launch.seed.jsonc` | 6936601 | `f00ecb32361e43e8ae29ffcfbc2a4e60955dfea6dbb6b19db4209210d5ebac2e` |

The ticket remains OPEN. Public runtime and backend completion remain the parent's outstanding work. Generated raw logs for this subtask were confined to `🗑️generated/interfaces`; this Markdown preserves their outcomes and selected diagnostics.
