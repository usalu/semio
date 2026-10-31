# TypeScript Program Reuse Lifetime Audit

This is a read-only Source and authored-metadata audit. No compiler, new full graph, cache mutation, policy change or reuse-store instrumentation was run. The reuse store remains a candidate only; no concrete stale-answer or unintended unused-retention defect was established, and this audit does not attribute the historical13416MB total to it.

## Installation and process ownership

The framework-wide installation census found one production caller of useTypeScriptProgramReuseV1: build/invocation/graph/memo/derive/warm/main/TS line19. The other matches are its defining function and type declaration. The cold graph parent and project-farm service do not install a reuse store. activeTypeScriptProgramReuseV1 starts undefined and is read only by script-inputs typescriptProgramV1 when it delegates to declaredTypeScriptProjectsV1. Consequently the reuse store is not intrinsically enabled in the cold graph/farm lane that previously overran. A separately running warm deriver could hold its own heap; no earlier per-process sample was retained to prove its presence or share.

The installer creates one store and one question-memory Map per detached warm deriver process. It retains both across requests. The process validates its captured implementation before serving, rejects a second live endpoint, serializes accepted requests through one promise queue and exits when idle or when implementation stamps change. The authored warm policy is600000ms idle,15000ms checks and2147483648maximum resident bytes. Those are existing limits, not changes.

runWarmDeriverMainV1 does not uninstall the module-global active store on return, but its actual production CLI immediately exits with the returned status. No successful in-process production caller re-entering this exported main was found, so this is not established as a live retention defect.

## Per-question begin/end

warmProgramDeriverV1 uses row.path as the question key. Before a repeated question, it checks every previous observation except typescriptText rows of that previous program's own sources. Only a successful guard arms oldProgram reuse. It removes the previous question-memory entry before computation, calls store.begin(armed), and always calls store.end() in finally. If base fails, no new successful question-memory entry is installed; the next question cannot arm from that removed entry.

A successful answer records the new support plus previous guard rows that remain required, and stores the source-name Set returned by end. The server's serialized request queue prevents independent requests from interleaving begin/end. No non-test production direct caller of warmProgramDeriverV1 outside that serialized warm main was found.

## What stays reachable

The reuse store deliberately owns two process-lifetime Maps:

- programs: one latest Program per absolute configuration path. keep replaces that configuration's previous Program. begin/end do not remove configurations absent from a later question.
- parsed: one settings value and file Map per absolute configuration path. A changed serialized compiler-options value replaces that configuration's file Map. Its key includes fileName, languageVersion, impliedNodeFormat and jsDocParsingMode; a known entry is reused only when text matches and shouldCreateNewSourceFile is false.

end prunes unused parsed keys only for configurations touched by sourceFile during that question. Configurations untouched by the question retain their previous parsed entries and Program. Programs themselves also retain their source files. A previously used configuration can therefore remain resident even while the next question uses another configuration. This is explicitly the documented warm optimization: the production memo schema states that the warm deriver keeps every program it built and stops at its resident threshold. There is no separate per-config eviction policy in current Source, and this audit provides no evidence justifying one.

sources, used and reused are reset at begin and detached/reset again at end. They do not accumulate across questions. releaseInferenceMemoryV1 runs before and after each warm request and clears observation/block/source-analysis/project-image caches, but does not clear the reuse store or the question-memory Map. That separation matches the purpose of retaining warm programs.

The installed store can keep at most one directly mapped Program per configuration, not every historical Program under that key. Whether TypeScript internally retains older structures through a latest Program was not measured. Configurations queried over time can grow the Maps until the existing process lifetime/resident guard ends the warm process. No real heap-size claim is made from Map shape alone.

## Bounded authored configuration census

The current census of1407 canonical authored project documents, excluding tests, node_modules, cache/generated trees and symlink traversal, found four explicit metadata.semio.closure.typescript target declarations resolving to four unique configuration paths:

| Owner | Target | Configuration |
| --- | --- | --- |
| architecture course TypeScript package | build | tsconfig.site.json |
| architecture course TypeScript package | typecheck | tsconfig.json |
| staffage site TypeScript package | build | tsconfig.site.json |
| staffage site TypeScript package | typecheck | tsconfig.json |

Each site config extends its package's main config; their include rosters are local builder/entry modules. Main configs include local scripts, builders and tests, with transitive framework imports and declared ambient package types. No TypeScript compiler was used to enumerate their complete files. Four is the number of explicit declarations in this authored census, not an observed count of live store entries or a bound on possible generated/derived requests.

The raw bounded inventories are generated/typescript-reuse-authored-config-census.json and generated/typescript-reuse-installation-census.json. They contain metadata coordinates and installer lines, not copied Source files.

## Current controlled graph observation

The existing schema gate remains owned by the schema agent (issuer362816); the last read-only log observation reached1838/1842input closures around05:01UTC and had no terminal receipt. This audit started no competing graph run and did not change its Source checkpoint. Root owns the independent actual RSS/PSS lineage sampler.

A later read observed the same schema gate verifying722/1843closures with no close receipt. This is a new verification pass observation; no cause for the changed total or terminal outcome is inferred.
