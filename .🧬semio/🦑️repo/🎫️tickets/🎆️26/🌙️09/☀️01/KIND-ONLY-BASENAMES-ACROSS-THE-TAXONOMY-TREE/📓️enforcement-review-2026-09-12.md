# Shared Enforcement Review During Implementation

The first shared implementation draft adds basename and inverted-package-target classifiers to repo discovery and integrates them into normalization. The coordinator reviewed the draft during active implementation and sent these items to its Sol owner for resolution and tests:

- A blanket `entry.name.startsWith('.')` traversal exclusion can hide authored tooling sources. The census must derive exclusions from declared opaque/generated/cache/exempt ownership rather than dot-prefix alone.
- Requiring only `fileKind.role === 'source'` needs explicit coverage analysis for test adapters, benchmarks, stylesheet code, and other authored implementations. Ordinary data collections remain distinct from source code.
- The walker must tolerate observed file disappearance during concurrent moves without silently declaring the source tree valid. Tracking directory-entry kinds avoids redundant `lstat` calls; cancellation/progress must remain available for the full filesystem walk.
- External filename admission must verify exact scope and authority. Cargo's conventional `src/main.rs` and `src/lib.rs` entries are configurable and cannot remain marked unconfigurable; the real `build.rs` restriction remains a narrow tool constraint.
- A focused live census is necessary for iterative proof. The existing full taxonomy verifier performs an extensive inventory, migration plan, and structured-reference plan before returning any findings.
- Package-boundary findings must remain separate from leaf-name findings. Source code can have anonymous filenames while still being owned by a language package instead of its domain.

These were review items on an unfinished draft, not claims about the final implementation. The final independent audit must check their disposition against tests and the live code.

## Concurrent Integration Diagnostics

The first package-purity run failed because concurrent test-taxonomy work introduced `testOraclesDirName` without its semantic directory registration. The Sol enforcement lane added the missing exact `oracles` kind while preserving the other work.

A subsequent discovery probe reached WGPU generator-contract validation and reported that browser module paths and input patterns were no longer in their required ordering after relocation. The package-topology lane owns reordering those current path arrays. Neither failed probe produced a package-purity count.

The next full-taxonomy load exposed a digest mismatch in `readme-license-owner-authority/🔣️.json`. A read-only comparison against `HEAD` confirmed that its source/owner evidence paths changed with this task's UI React package move, while its authority pin retained the prior digest. These changes had become staged through concurrent work and therefore did not appear in a plain unstaged diff. This mismatch belongs to the current relocation work; it is not established as an unrelated or pre-existing defect. The package-topology lane owns updating the reviewed live catalog pins and their dependent contracts consistently.

## Remaining Configuration Authority Review

The existing `fixedFilenameContracts["vitest-config"]` labels `**/vitest.config.ts` as unconfigurable while its own verification is `vitest --config`. That deserves an independent exact-contract review: an explicit config-path option appears to make this a configurable entry, so a default discovery filename alone does not establish a mandatory naming exception. The root `vitest.config.ts` currently appears in the source census while copies at TypeScript package roots are exempt under that lexical scope. A future configuration lane should validate the native tool resolver and then either give the exception genuine authority or relocate configuration source through existing explicit router config paths. This is separate from Cargo source defaults already repaired.

## Launch Authority Integration

A fresh search of both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` found no `kind-only-basename` or `verify-taxonomy-implementation` entries, despite the initial implementation report recording three inspected launch registrations. The workspace has concurrent launch regeneration. The enforcement lane is now registering its commands in the live source authority and verifying the derived launch result; the Go lane was warned to use the same authority. Existing unrelated launch flags and removed entries must be preserved. The failed leading-grapheme launch assertion expecting `--skip-nx-cache` is not permission to restore that flag broadly.

## Remaining Language-Specific Generator Topology

The normalizer still has a specialized `packageLocation` branch for `generator/🦀️<name>` directories: its source comment calls each a standalone Rust package root by construction, bypassing the ordinary domain/`packages/<ecosystem>` structure. This is a remaining implementation-specific folder rule that the Rust generator/oracle source wave must inspect. Renaming `src/main.rs` alone would not make that folder tree implementation-neutral. The appropriate long-term shape is a semantic generator owner with anonymous implementation leaves and an ordinary Rust packaging boundary; migrate the manifests, consumer commands, and fixtures before removing the special-case recognizer. Read the live tree because an external test-taxonomy task is concurrently reorganizing generator/oracle categories.

The full normalizer inventory can be scoped to an active semantic root, avoiding the huge historical ticket inventory; the next directory-coverage audit should use scoped inventory with progress rather than restarting the earlier unbounded whole-repository planner. Source basename and package-body results alone do not prove semantic directory ownership.

## Additional Implementation Extension Coverage Probe

The current implementation leaf policy includes all schema source-role file kinds plus CSS and HTML. Inspected source roles cover assembly, C/C++ and native headers, Cypher, C#, Go, JavaScript, PowerShell, Python, Rust, shell, SQL, Swift, and TypeScript including declaration chains. WIT is classified as an implementation-neutral schema contract.

A bounded rg files probe, including hidden admitted files while respecting Git ignores and excluding opaque/history/build roots, found no files in the tested unregistered shader/source families: WGSL, GLSL, HLSL, vertex/fragment/Metal shaders, Java/Kotlin, Lua/Ruby/Elixir/Elm, Vue/Svelte, F#/R/Dart, Objective-C, and additional bash/zsh/fish/PowerShell-module suffixes. This narrows an extension-coverage concern for the active admitted tree; it is not an exhaustive unknown-extension or ignored persistent-output census.
