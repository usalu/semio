# 📓️ terra-ts-os-report — `ts-os` verification

## 🎯️ Scope
Owned writable paths only: `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/**` and this ticket folder.
No edits were made — this packet is a **verification-only** run against the current baseline
(`💻️os/…/🟦️typescript` **206 passed / 1 failed** per `📌️important.md` "LATEST coordinator-verified
baselines"). No changes regressed it, so nothing needed repair.

## 🔎️ Pre-flight
- `git status --porcelain -- 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/` → **empty**
- `git diff --stat` same path → **empty**
- `git log --date=iso --oneline -5` same path → no commits touch this path recently (dates in commit
  subjects are fake per binding rules; not used for attribution)
- Confirmed `🧪️vitest.config.ts` already carries the W4 double-count fix: `include: []`,
  `includeSource: ["../../🟦️component.ts", "../../🟦️backbone-worker.ts", "../../🟦️effect-backbone.ts"]`
  — in-source suites only, no duplicate collection.
- Did **not** touch `💻️os/🟦️component.ts`'s `AppChannelCodec`/`AppChannelClient` region (it is outside
  my owned path regardless — that file lives at the package root, not under `📦️packages/🟦️typescript/`).

## ✅️ Measured — ran the CONSUMER command, not a narrower one
Command (exactly what `nx run @semio-tech/framework-os:test` runs):
```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
bun ./📜️script.ts test --reporter=verbose
```
Full output saved to ticket-adjacent scratchpad (not repo-tracked, per instructions temp files go in
ticket folder — reproduced here since the raw log is disposable):
`/private/tmp/claude-501/…/scratchpad/terra-ts-os-run2.txt`.

**Result:**
```
Test Files  1 failed | 2 passed (3)
     Tests  1 failed | 206 passed (207)
```
**Exit code: 1** (measured directly from the command itself, not through a pipe — `cmd > file 2>&1; echo $?`,
never `cmd | tail; echo $?`).

This is an **exact match** to the ticket's latest coordinator-verified baseline `206 / 1`. Named failure,
confirmed with `--reporter=verbose`:

> `matches the Rust plan_workflow across shared fixtures decoded via wasm` (in `../../🟦️component.ts`)

Error text:
```
Error: Cannot find module '…/💻️os/🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js'
imported from …/💻️os/🟦️component.ts
```
Confirmed by direct filesystem check that `🖥️host/📦️packages/🦀️rust/pkg/` **does not exist at all** —
consistent with the documented, already-routed-out-of-band root cause (`RUSTFLAGS` overrides
`.cargo/config.toml`'s wasm32 `getrandom_backend` cfg, so `pkg/semio_framework_os.js` cannot build).
Not re-labelled "pre-existing" without re-measuring — it was re-measured, by name, this run, and it is
the same single named failure as the baseline. No second, distinct failure appeared (the older W4-era
baseline table's `decodes the Rust-generated binary wire fixtures byte-identically` failure in
`🟦️backbone-worker.ts` is **not present** in this run — that test now passes; only the one wasm-pkg
failure remains, matching the "LATEST" table, not the older "W4" table).

All 206 other tests passed, spanning `../../🟦️component.ts` and `../../🟦️backbone-worker.ts`
(`../../🟦️effect-backbone.ts` contributed the 0-test third file entry seen in "2 passed" test-file count
— it has no `import.meta.vitest` blocks yet). Verbose listing confirms passing suites by name including
`backbone-worker offline resilience` (reconnect-backoff-resets-after-sustained-health family, matching
binding rule 16), `identity config facet`, `directory lane`, `@semio-tech/framework-os AppChannelCodec`,
`@semio-tech/framework-os AppChannelClient`, `workflow`.

## 🛠️ Changed
**Nothing.** No source edits were made — the suite already matches its verified baseline exactly and
nothing in this session's churn regressed it. No `lease-request` needed.

## 📣️ For siblings / coordinator
- `ts-os` baseline **holds**: 206 / 1, same named failure, exit 1, reproduced directly (no `tail` pipe).
- The 1 failure remains the routed-out-of-band wasm-pkg build issue — still unfixed, still not this
  ticket's to fix, confirmed still reproducing for the same reason (missing `pkg/` dir).
- `AppChannelCodec`/`AppChannelClient` region in `💻️os/🟦️component.ts` untouched; all its tests
  (`@semio-tech/framework-os AppChannelCodec`, `@semio-tech/framework-os AppChannelClient`) still pass,
  so nothing here currently depends on the `exchange` seam a sibling is removing from that file — no
  blocking dependency to report.
