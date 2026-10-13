# Nx Build Modes And Cache Ownership

## Implementation

The inference entry delegates compiler configuration projection to `library/🔌️nx-plugin/🎚️mode/🟨️.mjs` and source/cache input ownership to `library/🔌️nx-plugin/📥️inputs/🟨️.mjs`. Both are part of the plugin revision identity. The mode projection reads the neutral process JSON schema, gives inferred tasks explicit prototype/release configurations, and carries `SEMIO_BUILD_MODE` through options before execution. Defaults select prototype; fixed release and live preparation names retain their encoded intent. These workflow names remain live-session vocabulary, not accepted compiler modes.

The bootstrap resolves `run`, `run-many`, and `affected`, including colon configurations and `-c`. It admits prototype/release, rejects contradictory environment/explicit/fixed modes, and projects the chosen mode into the child environment before Nx hashes tasks. Explicit compiler mode/profile flags after `--` are refused for compiler targets. Test-specific arguments keep their owned meaning. The Nx patch propagates each initiating task's resolved default configuration through dependencies and refuses contradictory fixed-mode or unknown configurations.

Cargo, component, Python, .NET, site and browser-WASM outputs now use mode-specific declared paths. Browser-WASM and browser-support tasks retain uncached publication of their leased active package/module aliases; compiler warm stores still serve repeat builds. Canonical component module roots live below `dist/<mode>/🔌️plugin-modules`. Generic Demonstrator build uses a generic prepare target; mode-aware preparation aliases keep its entire prerequisite graph in the selected mode. React distribution exposes generic both-mode build aliases and fixed release aliases with canonical output ownership.

Generic script command inputs use the existing exact command-source closure and module parser, then resolve external imports through the admitted Bun lock graph. Nx hashes explicit package-location nodes, including patches and transitive dependencies, instead of blanket root package/lock bytes. Internal workspace packages remain source-graph dependencies. Commands without a discovered script retain manifest/lock fallback inputs. Cargo leaves likewise no longer hash unrelated root JavaScript package registration. The Cargo named-input fallback now hashes its own sources and root compiler policy/lock instead of every workspace manifest. Prototype profile override variables enter the cache policy. Owned command routing honors Nx's declared cache policy and no longer always bypasses task caches.

Nx collection ignore publication uses an atomic temporary-file rename with an authority recheck and cleanup. Admission snapshots use unique UUID temporary names. A failed nonwatching graph attempt whose diagnostic specifically says dependency resolution authority changed receives at most three retries; each re-admits current owners and disables stale graph reuse while retaining the original cancellation, deadline, and process ownership envelope. Arbitrary failed compiler/task commands are not retried.

## Tests Executed

- Initial added Cargo inference assertion failed because the old build target lacked an explicit prototype default. After mode projection, the test passed.
- `bun nx run repo:test-build-mode --output-style=stream` completed successfully with runtime `[DEBUG]` output. The portable JSON fixture includes eleven default, explicit, inherited, fixed, invalid and conflicting selections. Installed Nx verifies both configuration dependency graphs and mode-specific output interpolation.
- Enhanced installed Nx tests verify that a fixed default release task propagates release into a generic leaf and that fixed-mode/unknown-mode contradictions reject before tasks execute. The test deliberately selects `.nx/installation` because root `node_modules/nx` is a separate older, unpatched installation.
- Exact command dependency scope is checked against esbuild's independent runtime import graph and the admitted location-sensitive Bun model. The fixture explicitly names the yargs dependency loaded through computed `createRequire`; esbuild cannot discover that dynamic call. The direct focused test passed with `[DEBUG]` runtime output.
- The scoped integrated Nx test completed successfully (`nx-build-mode-ready.log`). Earlier integration attempts caught and repaired missing injected `owned`/`SCRIPT_BASENAME` context, a shebang pushed below an import by a concurrent edit, and internal workspace imports incorrectly classified as external packages.
- A real invocation rejected a changed dependency-resolution authority while concurrent agents edited source. Bounded bootstrap recovery was added afterward; runtime retry behavior and concurrent cache/mode evidence are being recorded below.

The full cache-contract suite and all native/web compiler families have not been run by this workstream. Their separate workstreams own those runtime checks. Windows/Linux behavior is authored cross-platform but was not executed by this workstream.

## Files Owned Or Edited

- `nx.json`
- `library/🟨️.mjs`
- `library/🔌️nx-plugin/🎚️mode/🟨️.mjs` (new)
- `library/🔌️nx-plugin/📥️inputs/🟨️.mjs` (new)
- `library/⚡️caching/🚀️bootstrap/📜️script.ts`
- `library/⚡️caching/🚀️bootstrap/🎚️mode/🟦️.ts` (new)
- `library/⚡️caching/🩹️patches/nx@23.2.0.patch`
- `library/⚡️caching/🔣️policy.json`
- `library/⚡️caching/📜️script.ts`
- `library/⚡️caching/📋️project.json`
- `library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts` (new)
- `library/⚡️caching/🧫️fixtures/nx-contract/🔣️.json`
- `library/⚡️caching/🧫️fixtures/bun-dependencies/🔣️.json`
- `library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts`
- Play and Demonstrator root `📋️project.json`
- Framework styling Python/.NET `📋️project.json`
- Framework plugin TypeScript `📋️project.json`

Here `library` expands to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. No modifying Git commands, worktrees, or AGENTS edits were used. Generated command output is confined to the active ticket's generated directory.
