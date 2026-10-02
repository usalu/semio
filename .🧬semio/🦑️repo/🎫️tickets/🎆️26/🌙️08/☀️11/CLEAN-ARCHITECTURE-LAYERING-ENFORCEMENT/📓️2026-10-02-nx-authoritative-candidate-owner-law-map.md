# Nx Authoritative Candidate Owner Law Map

Read-only primary repository sources; no plugin/test/compiler execution.

## Existing Exact Owners

Repo test domain `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` owns:

- `🧫️fixtures/🚷️discovery-boundaries/🔣️.json`, `🧬️schema/🚷️discovery-boundaries/🔣️.json`, `🧪️tests/🚷️discovery-boundaries/🟦️.ts`. The two laws validate closed schema and compare actual materialized no-follow discovery against independent fast-glob, including loop/case/feature links and generated/opaque trees. Package script18–21 registers test discovery-boundaries,30s and owned artifacts.
- `🧫️fixtures/📐️test-layout` and corresponding schema/test-layout owner; test at282–292 calls createNodes callback with corpus-derived feature path and real workspace root.
- `🕸️dependencies/🧫️fixtures/🦀️inputs/🔣️.json` plus same owner `📐️schema/🔣️.json`, test `🕸️dependencies/🧪️tests/🦀️inputs/🟦️.ts`. Materializes actual feature and calls callback [feature] at27. Proves subject/transitive/oracle/native host input closure, actual esbuild command dependency parity, preparation outputs and cache invalidation. At105–109 a resident Node plugin must reject malformed changed helper, then accept restored helper: preserve fresh authority and module revision semantics.

## Current Callback Semantics

Plugin createNodesV2 declares **/*.feature at309. testCaseProjects at183–202 receives configFiles but ignores it, performing full discoverCaseDirs. That is a source-proven performance candidate, not measured latency attribution. A canonical replacement must validate each current supplied feature candidate physically and apply identical taxonomy exclusions/canonical owner/case rules before computing projects. Candidates are not authority merely because Nx supplied them: linked leaves/ancestors, absent files, wrong feature basename, reserved/generated/opaque paths and delivery nesting remain rejected. Added/removed files must affect the next callback; no persistent list without exact freshness proof.

Retain case project naming/hash, target phases/levels, native package closure, generator dependencies, declared command sources and output/cache policy. Preserve deterministic candidate order and dedup. If callback configFiles is a full matched list in installed Nx, use it directly; if caller supplies incremental facts, define and test that exact contract rather than fall back silently to stale membership. Current direct tests often supply one concrete feature and should receive only its admitted project.

## Concrete Harness Contradiction

Test-layout:282–292 constructs synthetic feature paths from source corpus but calls the actual callback against repoRootFromHere without materializing the feature. Its assertion only asks discovered.length>0. Under current ignore-configFiles behavior, unrelated real projects can satisfy accepted=true. This is not a sound candidate oracle. Materialize those features in a fixture workspace with actual taxonomy/policy/router authority, or test a pure owned candidate admission selector and separately prove callback output identity with actual files. Do not preserve unrelated-project returns to keep that accidental test green.

## Minimal Closed Extension

Extend discovery-boundaries fixture/schema with supplied candidate list and expected exact projects/case directories. Include wrong, duplicate, absent, linked and excluded candidates; compare against the independent physical glob oracle filtered to supplied current candidates. In one resident callback test physically add a case, call with refreshed candidate list, remove original case, call again, and assert exact membership changes. Keep helper mutation and taxonomy exclusion mutation laws fresh. Add a tree with many irrelevant admitted directories and an observable owned filesystem port/counter to prove callback work scales with supplied candidates while preserving no-follow ancestor checks. A deadline or reduced global census is not proof of correct optimization.

No route is run and no pass is inferred by this report.
