# Result wpath (Windows checkout ticket paths test made fast by design)

## Cause
`git ls-files --others --exclude-standard` must readdir ~95k directories (180k tracked files); it costs about 10 s CPU and 30-38 s wall on this loaded machine (load 35-50). `git ls-files --others` does not use the untracked cache or fsmonitor, and `:(exclude)` pathspecs do not prune the walk (measured: remainder walk was as slow as the full one), so no single-process variant is fast. The old test ran the full walk plus a ticket-scoped walk per scope.

## Change
- `.../📚️library/🧪️tests/🔬️workspace-contract/🏗️shared-fixtures/🟦️.ts`: new `checkoutPathInventory(root, scopes, {parallelism?, budget?})` (+ `CheckoutPathInventory` type). Reads tracked paths once (`ls-files -z`), weights every tracked directory, splits each scope into disjoint `:(literal)` pieces (readdir of heavy directories only), bin-packs pieces over `availableParallelism()` async `git ls-files -z --others --exclude-standard -- <pieces>` processes, and unions the results. Scope `.` is walked once and ticket-scope results are derived by prefix in memory (no second walk).
- `.../🔬️workspace-contract/🎫️ticket-protection/🟦️.ts`: the live test uses one shared inventory (scope `.` when any checkout root has scope `all`, else the ticket root only). The rule is unchanged (tracked or untracked path in scope, `win32.join(root, path).length > maxPathUnits`, untracked or existing on disk); a plain-length screen (`root.length + 1 + path.length > max`, exact because `win32.join` never lengthens) runs before the normalized join, which saved ~1 s. New test "partitioned parallel inventory equals the single-process Git enumeration on a fixture repository".
- `.../🧪️tests/🧫️fixtures/🪟️windows-checkout-paths/🔣️.json`: version 2 -> 3, new language-agnostic `inventory` section (gitignore, tracked, missing, untracked, ignored, nestedRepos, checkout, expected oversize lists for scopes `all` and `ticket`); reformatted by the JSON writer.

## Verification
- `bun test './🧰️framework/.../🎫️ticket-protection/🟦️.ts'` (full file): 3 pass, 0 fail, 119 expects, 11.0 s total at load ~48 (the live test alone 6.2 s of its 15 s budget; fixture-parity test 1.3 s). Before: the live test timed out at 15 s.
- Fixture parity: temp git repo with tracked, untracked, gitignored, tracked-but-deleted and nested-repo entries; for scopes `.` and the ticket dir, and for (parallelism, budget) in (1,1000), (1,1), (3,2), (8,1) and defaults, the partitioned untracked set, the tracked set and the oversize list equal the old single-process enumeration (git is the third-party oracle) and the fixture's expected lists.
- Live parity (scratch script, deleted): partitioned vs single `git ls-files --others --exclude-standard` on the real tree: 56 vs 56 paths, 0 differences; 9.4 s vs 34.6 s. Parallelism scaling of the walk: 1 -> 30.3 s, 4 -> 13.6 s, 10 -> 9.9 s, 20 -> 11.3 s (default = core count).
- No package.json/bun.lock edits, nothing under frozen paths, no cargo, no git-modifying commands (the two `git status` runs used for timing may have refreshed the index stat cache only).

## Open
- Under extreme load (load > 50) the live test is still CPU-bound (about 6-9 s here); on an idle 10-core machine the walk should take about 2-3 s.
- No repo-wide inventory module existed that already provides tracked+untracked lists with partitioning (`repository-capture`, `source-admission`, `graph-membership` each run a single-process `ls-files --others`), so the helper lives next to the contract tests.
