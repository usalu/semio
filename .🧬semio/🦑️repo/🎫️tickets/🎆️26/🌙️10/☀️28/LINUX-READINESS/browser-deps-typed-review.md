# Typed Bootstrap and Browser Test Review

Read-only Source review, 2026-10-29. No edits, tests, native/lease/process operations were performed.

## Verdict

The reviewed type fixes preserve the existing behavioral comparisons and add no production runtime dependency. No new concrete blocker was found in these edits or the browser leaf's authored freshness closure.

## Semver Oracle

Native bootstrap toolchain test now loads existing semver using node:module createRequire and a local narrow port type `{gte(actual:string,required:string):boolean}`. This still calls the actual third-party semver.gte for every neutral version row and compares its boolean result against the authored expectation; shell version_at_least is separately executed and compared against the same expectation. No expected-value substitution or weakened oracle was introduced.

semver7.8.5 already appears in root and repo library package devDependencies, not dependencies. The local type avoids adding an external ambient @types package or exporting third-party API types; no new package/runtime dependency is required. The test's existing declared domain is stable semantic versions; this typing fix does not claim new prerelease support.

## JSONC Structural Guard

Bun.JSONC.parse result must be nonnull object, mounts must be array of strings, workspaceFolder must be string, and onCreateCommand must be either string or string[]. Both shapes are valid for the existing projection assertions. The actual current .devcontainer onCreateCommand is argv array sudo/chown/owner/two cache paths, so the corrected guard accepts the real authored shape.

The full Bun JSONC result still compares deeply to JSON.parse(source). Existing Source projection checks remain: schema-valid fixture endpoint, every mount target basename differs from Source endpoint basename, and onCreateCommand does not contain the workspace Source endpoint. Guard broadening from string-only to string|string[] removes a false rejection; it does not bypass these existing checks.

The browser law's `expect<unknown>(calls).toEqual(row.expected)` changes matcher generic compatibility only. Runtime deep equality still compares every recorded command, argv and selected environment object against neutral expected output; independent Ajv const and own-validator const comparisons remain immediately adjacent. Own-validator [] success/error-length checks remain correct.

## Browser Leaf Freshness

Current `📚️library/⚡️caching/📋️project.json` test-browser-dependencies retains source execution, sourceInputObservation freshness and the actual changed test file as runtimeSourceEntries. Inputs include system/**/* (therefore typed test/schema/policy/fixture/helper), native dependency branch, and Bash bootstrap. Native/Bash files read textually remain explicit resources. No forwardAllArgs, outputs or arbitrary new dependency edge appeared.

The typed bootstrap semver/JSONC edits reside in native bootstrap's toolchain test, separate from the browser leaf; their combined execution is supporting evidence, not a reason to expand browser leaf to own unrelated bootstrap tests. Current portable contribution remains capabilities:{} and all three supported platforms, and the caching test aggregate includes its explicit leaf as previously reviewed.

## Evidence Boundary

Coordinator reports actual combined31tests275assertions0failures and strict173files0errors following owner fixes, replacing the earlier88674 six-error result. These are supplied terminal results, not rerun here. Fullsemantic6279 was reported normally queued and is not assumed green. Type-check/focused-test success does not establish full setup, real Linux package provisioning or browser acceptance.
