# Prune Of In-Use JavaScript Dependency Installs

## Pruner location
`C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\⚡️caching\🚀️bootstrap\📦️dependencies\♻️collection\🟦️.ts`
(`planDependencyGenerationCollectionV1`, `collectDependencyGenerationsV1`, `collectUnpinnedDependencyGenerationsV1`), driven by the `cache:dependencies` phase of
`...\🏗️build\🧹️cache-gc\⌨️command\🟦️.ts`. Partition `b/d/m` = dependency store, `j/<20-hex>` = generation directories.

## Liveness rule before
A generation was retired when: not named current by a pointer, not held (hold file `hold/<key>.<pid>.json` of a live pid, written only by the installer/verifier process),
not bound by a checkout, not exclusively pinned by a reader lease, and older than the grace (superseded 1h, by newest mtime inside it). A server started from
`j/<key>/node_modules/...` by any other tool (here bun running vite) announced no hold and took no pin, so after a superseding install and one hour it was deleted under the live process.

## Liveness rule after
Additionally, every due candidate is checked against the live process table (`processTableSnapshot`, existing cross-platform ps / Get-CimInstance helper):
a generation whose directory (`/j/<key>`, separators and case normalized) is named by any live command line is kept. If the process table cannot be read,
nothing is retired (refused: "no readable process table"), consistent with the other refusals. The pure plan and the Python oracle are unchanged (the in-use set merges at the collection layer).
New export `dependencyGenerationsInUseV1`; new option `commands` for injection in laws.

## Files changed
- `...\♻️collection\🟦️.ts` (rule + docstring)
- `...\♻️collection\🧪️tests\🟦️.ts` (new laws; existing laws inject `commands:()=>[]`)

## Tests
`bun test --timeout 120000` in the collection module: 19 pass, 0 fail. New laws: injected command lines (forward and back slashes, upper case, key-prefix non-match,
unreadable table refuses, release after process gone) and a real spawned process validated against the real process table. Two older paging laws exceed bun's default 5s timeout on Windows (filesystem-bound, unrelated); they pass with a larger timeout.
Note: already-deleted installs are not restored; restart the dev server after re-running its install.
