# Why ~600 owners go stale between consecutive runs

Ticket: GET-ALL-PRINT-PRODUCTS-RUNNING-END-TO-END. `BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.

## Evidence (host log `$TMPDIR/semio-repo/501/host-*.log`, persisted store `.cache/b/g/m/generations`)

- Generations 7 to 18 (12 consecutive waves of 48) carry the same `implementation`, `identity` and `env` digests (manifests in `generations/m`). So it is not an implementation fingerprint (`implementation` only gates the verified-stamp coverage, `generations/🟨️.mjs:421`, `🔭️coverage`), not lockfiles/Cargo metadata (env digest never moved), not mtime doubt (`🔎️doubt` only picks candidates; `👁️observation/🟨️.mjs:386` re-checks content and replays the analysis, an equal `resultSha256` keeps the row).
- The plan lines say it exactly: `graph: 4 stale facts (analysis 4) leave 633 projects stale` (gen 18) and `6 stale facts (analysis 4, file 1, typescriptText 1) leave 585 projects stale` (gen 19). The stale rows are the source analysis of `BUILD/🌐️remote-cache/🧶️nx/🟦️.ts` (and its test file) and 13 derived command-walk answers (`the derived result of .../📜️script.ts changed`).
- Cause of the change: another agent added a dynamic import and new named imports to that file (git diff), i.e. the analysis result (import list) changed, not only bytes.
- Why it fans out: `⚡️caching/🚀️bootstrap/📜️script.ts:454` imports that module, so it is in the walk of every command script. Probe of the gen 18 fact index (lazy `openPackedFactIndexV1`): its analysis row is held by 481 + 138 + 40 project owners directly and by 2809 derived pseudo owners (supports of the walk rows). One cargo owner holds about 24k raw rows (3878 `analysis`, 9918 `lstat`, 5599 `resolve`) next to its 2 derived walk rows.
- A command walk is keyed by its roster of every file outside the declared scheduling domains (`BUILD/📥️source/🔏️closure/🗓️scheduling/🔣️.json`, read by `🧭️script-inputs/👓️walk-views`). The remote cache was not a domain, so an import edit there changed the roster answer of every command walk.

## Fix (data, uses the existing mechanism)

- New scheduling domain `remote-cache` (prefix `BUILD/🌐️remote-cache/`, ports `🧶️nx`, `⌨️command`, `🖥️server`), mirrored in the named input `schedulingSourceExclusions` of `nx.json` and `BUILD/🔣️policy.json` (a law keeps all three equal). The `scheduling` and `native` walk views now leave it out, so an import edit or a new file there keeps the derived answer, hence every owner holding it.
- Test `BUILD/🕸️graph/📥️inference/🧭️script-inputs/👓️walk-views/🧪️tests/🟦️.ts` (new fifth test): files the remote cache gains or loses move neither view answer, a keyed file moves both, answers equal an independent digest. Run: `repo tools exec --light bun -- test ./<file>`: 5 pass. Scheduling law tests (`🗓️scheduling/🧪️tests`): 18 pass.

## Not fixed / not verified

- The roughly 660 owners that hold the analysis row of that file as a raw row, outside the derived cutoff, still go stale on a structural edit of any file in the command walk that is not a scheduling file (body-only edits are absorbed by the analysis cutoff). Why those layers are held raw (`🧭️script-inputs/🟨️.mjs:1307-1308` adds a layer raw unless a member walk is held; suspect missing sidecar members) is not proven.
- Landing the table moves every walk view answer once, so the next run re-infers the whole closure once, and Nx task keys that use `schedulingSourceExclusions` change once.
- `cargo metadata` is not called anywhere in owner inference (`rg "cargo metadata"` finds only `🧩️units/🦀️rust`); there is nothing to share per workspace. Per-owner minutes are the Rust source-set walk plus TypeScript analysis under swap with 8 workers; not profiled.
- Fixed overhead not touched: each run verifies the tree 2 to 3 times (`No verified change journal cursor is recorded for generation N`, 354k doubt rows, about 80 s) and each 48-owner wave pays a re-read and publish of the 11 MB index (about 15 s), so 13 waves add several minutes.
- `🔧️tools/🏗️bazel/🎬️scopes/🧪️tests` has 4 failures (host-capacity, process texts, closure size 470 > 450); the violations named there do not involve the remote cache.
