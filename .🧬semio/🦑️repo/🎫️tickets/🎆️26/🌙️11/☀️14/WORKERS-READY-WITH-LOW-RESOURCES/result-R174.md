# R174: nx:authorizers ENOENT on the cache policy (wall of g1-m07 and g1-m10)

## Cause
`🏗️build/👁️observation/📂️source-names/🟨️.mjs` (new 2026-11-17 09:00, not R166) read `⚡️caching/🔣️policy.json` at module load with `readFileSync(new URL(rules.generatedDirectories.policy, import.meta.url))`; the path was a string in its schema JSON, so the compiler's literal-path scan (`dataReadLiteralV1`) never saw it and the bundle did not serve it. In the single-file bundle `import.meta.url` is `.cache/b/t/m/nx/.stage-X/`, so `../../../` resolved to `.cache/b/t/⚡️caching/🔣️policy.json`.

## Reproduction (red)
Scratch only: `scratchpad/R174/repro.test.ts` (mocks the two control checks, captures the real closure of `🔏️authorizers/📜️script.ts`, calls the real `compileNxAuthorizerSnapshotV1` with the pinned bun 1.4.2), copy the bundle four directories deep, run it: ENOENT at `46679:43678`, same offset as the workers. Bun 1.3.14 mis-decodes emoji specifiers, use the pinned bun.

## Fix
`source-names/🟨️.mjs` imports `../../../⚡️caching/🔣️policy.json` with `{type:"json"}`; the `policy` path field is removed from `source-names/🧬️schema/🔣️.json`.

## Laws (new dir `🔏️authorizers/🧪️tests/📦️bundle-load`, 3 tests)
1. scanner unit vectors; 2. static: every read call in the closure whose path is built from `import.meta.url|dir|dirname` and is not a literal sibling or `physicalSourceOriginV1` must be in the audited list `🧫️audited.json` (7 entries, each with a reason); 3. dynamic: compile the real bundle, run it from a far temp directory, no ENOENT and the expected "invocation required" refusal. Red before (tests 2 and 3), 3/3 green after.

## Other location-relative reads found
Seven, all audited as not failing the load (literal-served, lazy, or caught): caching/📥️inference, bootstrap/📜️script.ts, 🧊️memo, 📍️coordinates, 🧳️portable, 🛰️plan, library/🟨️.mjs. None changed.

## Landing
02:13:30 to 02:13:41 UTC, lock owner claude-R174, released, hold 11 s. EXCEPTION to the 02:05-03:30 freeze granted by the coordinator (every worker fails on this wall). Script `🧰️scripts/R174/land.sh` (restore by backup copy). Under the lock: bundle-load 3 pass 0 fail, source-names 8 pass 2 fail (the 2 are baseline red, cwd .gitignore). After: entry-law 3/0; closure-law 3 pass 1 fail (computed import sites in 9 unrelated modules, not from this change).

## Workers after landing
Not yet at `ready: true` at 02:18 UTC; see the appended record in landings.md.
