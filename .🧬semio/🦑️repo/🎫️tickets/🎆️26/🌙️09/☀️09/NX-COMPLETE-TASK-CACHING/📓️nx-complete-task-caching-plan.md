# Nx Complete Task Caching

Associated goal: `🎯️aioptimizedrepo` (`AI-optimized Repo`).

## Objective

Every deterministic Nx step (build, test, lint, generate, schema, verify, stdio, cpp-build/test, wasm, package) must cache. Continuous live processes (dev, serve, watch, start) stay uncached. Mutating setup/deps/clean/publish/format-write stay uncached. Cache hits only when inputs are unchanged.

`nx.json` must not gain `targetDefaults` — native Nx defaults override plugin metadata. Policy lives in `⚡️caching/🔣️policy.json` and `🟨️.mjs`.

## Evidence from current tree

Workspace `test`/`build`/`lint`/`generate` stay uncached because of a triple lock:

1. Authored `cache: false` in root `📋️project.json`.
2. `targetPolicy` spreads authored target after `cache: true`, so false wins.
3. `projectWithDefaults` workspace-root override: `target.cache ?? false`.

Additional defects:

- `matchesCommand` treats `format-check` as uncached via the `format-` prefix.
- `rootCommandTargets` seeds every router command with `cache: false`.
- `test-exhaustive` is hard-coded uncached.
- `generate` has no positive cache rule.
- 300 deterministic-looking targets across 32 projects declare `cache: false`.
- The caching module disables cache on all 13 of its own targets.

Keep uncached by design: continuous servers, setup/deps/install, clean/purge/publish, write-baseline, playground `prepare`/`activate`/`serve`, generator `checkTarget` freshness guards, `repo:generator-inputs` discovery.

## Implementation

1. Make `targetPolicy` authoritative: continuous first, then uncached (exact `format`, prefix for other families), then cacheable families with `cache: true` after spread, then default `cache !== false`.
2. Remove the workspace-root / repo-test-domain deny override.
3. Stop hardcoding `cache: false` in `rootCommandTargets`.
4. Enable cache on authored workspace and caching-module deterministic targets.
5. Enable cache on remaining deterministic `📋️project.json` targets (generators, checks, tests) except freshness guards and mutating names.
6. Extend language-agnostic cache-contract fixtures and assert Nx `isCacheableTask`.
7. Runtime-verify a representative cache hit/miss.

## Non-goals

- Remote Nx cloud cache.
- Caching continuous Vite/dev servers.
- Caching `prepare` playground gates until they own durable outputs.
