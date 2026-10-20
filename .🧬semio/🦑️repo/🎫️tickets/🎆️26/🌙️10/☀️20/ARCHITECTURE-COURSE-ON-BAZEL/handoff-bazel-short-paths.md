# Handoff: Short Bazel Source Paths And A Per-OS Path Budget

For the peer fleet (ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN). Staged, not applied. `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL`, tools in `T/x-paths-tools/`, report in `T/result-x-paths.md`. `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `BS` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel`.

## Problem

Source labels were `s/<repo path>`, so every path Bazel resolves for a declared source was `<output base>/execroot/_main/s/<repo path>`. For the course closure that is 260 (macOS), 259 (Linux native), 250 (devcontainer) and 262 (Windows) UTF-16 units against the 256 rule; test actions add a runfiles tree (`…/bin/t_<16>.runfiles/_main/s/<repo path>`, 331 on macOS) that the first audit missed. The platform profile only checked a constant (`trackedPathUnits = 256`) and refused Windows unless LongPathsEnabled; the readiness check did the same unconditionally; macOS used the POSIX limit 4095 instead of PATH_MAX 1024.

## Change

1. Alias farm. A host-state action (task, test, launcher) names sources `a/<alias>/<name>`; `alias` = first 10 hex digits of SHA-256 of the slash-separated repo directory. `<ws>/a/<alias>` is a directory symlink (junction on Windows) to the checkout directory. Labels are independent of the checkout location and OS, so action keys stay portable. The real path stays in the checkout (the runner never reads `a/`) and in `<ws>/sources.json` (alias to directory). Not in task manifests, because manifests are action inputs.
2. Hermetic actions keep `s/<repo path>` (the hermetic runner executes inside that layout and its tests assert it). The generator keeps two filegroup namespaces (`a_<hash>` and `d_<hash>` group names); only hermetic actions use the second.
3. Measured closure. The generator writes `<ws>/paths.json` (longest alias label and longest hermetic label in UTF-16 units and UTF-8 bytes). `requireBazelPlatformProfileV1` reads it and `bazelPlatformProfileV1` computes every path family Bazel creates beneath the output base (source, runfiles-source, snapshot, runfiles-snapshot, sandbox-source) from schema templates.
4. Per-OS limits from the schema (`PathBudget`): repository rule 256 units everywhere, plus macOS PATH_MAX 1024 and Linux 4096 (usable = max - 1, in UTF-8 bytes), Windows MAX_PATH 260 (259 units) or 32767 when long paths are enabled. The profile refuses only when the real closure does not fit, naming the family and how many characters shorter the checkout must be. Windows no longer needs LongPathsEnabled for the aliased closure.
5. Readiness (`BS`): the unconditional "enable LongPathsEnabled machine-wide" refusal and the `longPaths` fact are removed.

## Files

New, copied from `T/x-paths-tools/staged/` (short names, because the repo paths exceed 256 units inside the ticket folder): `BZ/🧪️tests/📥️paths/{🟦️.ts,🔣️.json,🧬️schema.json}`.

Anchored edits by `apply.py` (45 edits, listed by `apply.py --check`): `BZ/📜️script.ts` (alias functions, group views, farm, generator, measurement reader, profile call sites), `BZ/🌐️platform/🟦️.ts` (budget model), `BZ/🧬️schema/🔣️.json` (alias rule, `PathBudget`, `PathClosure`, `PathMeasurement`, `PathBudgetReport`, `PlatformInput.closure`; removes `PlatformRules.limits|trackedPathUnits|execroot`), `BZ/🧪️tests/🟦️.ts` (registers the new test), `BZ/🧪️tests/📥️platform/{🟦️.ts,🔣️.json}` (closure in inputs, expected outcomes recomputed, the constant-based MAX_PATH test removed), `BS/🩺️readiness/🟦️.ts`, `BS/🧬️schema/🔣️.json`, `BS/🩺️readiness/🧪️tests/🧫️fixtures/🔣️.json`.

## Tests (test-first)

`BZ/🧪️tests/📥️paths/🟦️.ts`, 11 tests plus 1 real-Bazel test (skipped without `SEMIO_TEST_BAZEL_BINARY`). Language-agnostic fixture `🔣️.json` (25 host rows, aliases, labels) with closed schema `🧬️schema.json`, checked by Ajv. Oracles: Python hashlib for aliases and labels, an independent Python evaluation of the budget, iconv-lite for UTF-16 and UTF-8 lengths, the repo taxonomy for the 256 rule. A generator integration test runs `generateBazelWorkspaceV1` on a graph with one hermetic and one host-state task and checks that every label in the BUILD resolves through the farm. Red before the change: `Export named 'bazelPathBudgetV1' not found` (`T/🗑️generated/x-paths/red.txt`). Green: `T/🗑️generated/x-paths/check.log`.

`check.sh [workdir]` builds an unpatched and a patched overlay of the build library (copies plus symlinks, never the checkout), runs the new and 16 regression test files on both, compares the failing names of the aggregator (22 pre-existing overlay-only failures, identical) and type-checks the touched files (0 messages). Status 0.

## Apply order

1. After the peer integrate rounds finish and the coordinator lifts the freeze: `python3 T/x-paths-tools/apply.py --root <checkout> --check`. Expect 0 failures and 45 pending (anchors were verified against the live tree at 16:2x). A moved anchor is reported as `anchor-missing`, never guessed.
2. `python3 T/x-paths-tools/apply.py --root <checkout> --live`. Without `--live` it refuses to write into a git checkout. It is idempotent (second run: all applied).
3. Run `check.sh`, then `bun test` the explicit file `BZ/🧪️tests/📥️paths/🟦️.ts` in the checkout, then one `bun repo --tool=bazel run @teaching/architecture-course:test` and the real-Bazel test (`SEMIO_TEST_BAZEL_BINARY=<pinned bazel>`).
4. Independent of the other staged patches (no overlapping anchors with x-release, x-keys, x-ingress, x-site, x-dev, x-inputs, x-proctor tools). Apply after the peer's own `generateBazelWorkspaceV1` changes; rerun `--check` after each integration.

## Not verified (needs the coordinator or a CI run)

- Real Bazel acceptance of `a/<alias>/<name>` labels where `a/<alias>` is a directory symlink. It is the same mechanism as the existing `s` link (one more path component); the gated test and the generator test cover it, but the brief forbids executors to run Bazel.
- Windows junction farm (about 2,900 junctions for the course), `readlink` comparison against `\\?\` forms (a mismatch only recreates the junction).
- Hermetic actions keep the repository layout, so a deep hermetic closure is refused by the budget (sandbox-source family) instead of being shortened. No project declares `hermetic` today.
- The macOS budget is dominated by `runfiles-snapshot` (231 of 256, slack 25), not by sources. Optional later levers: shorter server hash (16 to 8), `files/<32 hex>` to 16 hex.
- Git on Windows still needs `core.longpaths` for the checkout itself (deepest tracked file 226 units plus the root); that is a per-user git setting, not a machine-wide change.
