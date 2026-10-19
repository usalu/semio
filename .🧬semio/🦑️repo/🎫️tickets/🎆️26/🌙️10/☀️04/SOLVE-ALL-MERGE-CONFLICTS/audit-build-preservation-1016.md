# Independent Build Preservation Audit — 2026-10-16

Scope: the exact 44 existing source/asset paths listed in `preserve-build-stash-1016.md`. Read-only source/Git audit; only this report was written. No compiler, build, runtime, or tests ran. No source, index, stash, checkout, or ticket metadata changed. Repo MCP is unavailable in this session; parent owns ticket lifecycle.

## References

Verified HEAD `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, latest stash `ec7b3361858df86599da79e5b6dde70ccbbc3625`, stash parent/base `f856dff081373e131ebf72618120cb73e303a912` with `git rev-parse`.

## Observations

Compared every path against base, HEAD, latest stash and working source, normalizing CRLF. Every inserted/replacement stash block is present verbatim except three combined owner blocks: build entry, cache dispatcher, Cargo replan source contract. Manual HEAD-to-working diff inspection confirms the build preflight requirements moved into the dissolved child-capability owner; the build entry dispatches preflight before opening lease/cache partitions and retains parent-control validation. The child owner preserves HEAD bounded watch descendant collection and excludes preflight from Nx bootstrap declarations. Cache dispatcher retains authored routes while supplying HEAD `phaseControl` to both compiler and test child calls. The replan source contract preserves dashboard plus launch/seed assertions, as already documented in the preservation checkpoint.

A separate base-to-HEAD insertion/replacement line scan across all 44 paths found only one nontrivial HEAD-authored line absent verbatim: the child-capability main-command expression, extended solely with `preflight` in the excluded command list. The HEAD-owned remainder of that expression and watch processing remain. No unexplained HEAD loss was found by this finite scan.

CMake presets, Windows bootstrap asset, CMake owner/test/feature/schema/fixture and all preflight source/schema/policy/fixtures match latest stash exactly after newline normalization. Consequently Ninja/MSVC activation and test-toolchain source remain exactly authored, without attempted compilation repair. Bootstrap preserves stash envelope extraction, original policy/capability identity checks, inherited cancellation handoff, exit-code handling and environment restoration while retaining HEAD resource-control/native network custody. Cargo preparation owner/custody and original-control contracts match stash; Cargo shared owner retains HEAD physical source selection while adding controlled program/observation operations and selected preparation. The graph inference changes inspected against HEAD are documentation/type declarations; tool-neutral Nx/Bazel implementation remains.

All 16 JSON files in this 44-path list parse. `git diff --check HEAD -- <44 paths>` returned 0. Maximum absolute touched source path length is 133 characters. No tracked AGENTS.md change appeared in the read-only HEAD diff. The list introduces no newly named permanent script file other than existing convention `📜️script.ts`; the `.ps1` is the pre-existing native bootstrap asset and is stash-exact. No generated output or temporary script file was retained.

## Deviations and Boundary

No actionable preservation deviation found in this finite build lane. Whole-repository preservation, runtime correctness and compile/test success are not established. Root package/lock/Cargo/launch integration, Teaching course rename/declarations, and Domain owners remain with their assigned lanes; their unsettled files were not altered. Broader library dependency/catalog/discovery/native-artifact preservation lies outside this exact 44-path checkpoint.
