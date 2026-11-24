# GR3: precise owner-record validity (2026-11-23 ~15:50 Mac clock)

Paths: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `S` = `B/🕸️graph/📥️inference/🧭️script-inputs/`.

## Finding 1: the premise is only half right. Rows are already per file; the whole-record verdict comes from the `closure` rows, not from Rust file rows
- Owner records already hold one row per fact: `exists`/`field` per crate `Cargo.toml` (norm-energy-plugin: 741 exists, 724 field), `file`, `lstat`, `analysis`, `global`. A moved Cargo.toml fails exactly its own row and `current()` (`B/👁️observation/🟨️.mjs:1183`) already names it (E81). Splitting these rows further would change nothing.
- What stays unnamed is the batch of `closure` rows (9 to 20 per owner): each is a question to a shared command-closure entry (`["some"|"generated"|"digest", ...]` over the walk of the command script closure). `provider.verify` (`S/🟨️.mjs`, around line 1860) verified them group by group but swallowed every exception into `failed(undefined, "closure-probe")`, which `staleWholeRowV1` renders as `whole closure observations`, `current "-"`.
- Evidence from the 6 kept coordinator read logs (`.cache/f/graph-server/read*.log`, 15 `[graph-stale]` events, all `kind whole`, `why closure-probe`, `current "-"`, none names a file):

| owner | closure rows | exists rows | events |
|---|---|---|---|
| norm-eurocode-materials-plugin (+ hub component, source take) | 14 / 9 | 762 / 753 | 3 |
| norm-eurocode-plugin | 14 | 890 | 2 |
| norm-eurocode-structural-plugin | 14 | 867 | 2 |
| norm-energy-plugin | 14 | 741 | 2 |
| block-2d-rs / block-catalog-rs / block-plugin | 20 / 16 / 10 | 582 / 253 / 680 | 1 each |
| abi-rs | 9 | 89 | 1 |
| async-timer-rs | 9 | 15 | 1 |
| realparts-of-powers-z-n | 12 | 6 | 1 |

  Size is not the discriminator: a 15-row crate goes stale for the same reason as an 890-row plugin. The shared factor is the command-closure entry (the tool's own sources, 1,500 files, edited all day by the fleet). Two exceptions explain `closure-probe`: (a) a take never derives (`runClosureTakeV1`): a stale stored entry throws `BUILD_TAKE_MISS`, which verify turned into "owner stale", although the owner's answers may be unchanged; (b) in a read parent a derivation throws the read-parent refusal (`assertNotReadParentV1`). I could not run a repro to tell which of the two produced the 14 `infer` events (open item 1).

## Finding 2: ranked invalidating paths today (kept logs, 112 `workspace is stale` lines over 7 reads)
| rank | path | lines | share | what it is |
|---|---|---|---|---|
| 1 | `units/sealed/<hash>/{🔨️modules,📦️packages}/…/🟦️.ts` (13 files, committed sealed copy of runtime sources, 349 tracked files) | 91 | 81 % | not owned by any project, so `plan` (`B/🕸️graph/🕰️generations/🧭️plan/🟨️.mjs`, the `else{envStale=true;…}` branch) marks the environment stale |
| 2 | `teaching-morph-preview-launch-kit`, `staffage-launch-kit`, `puzzle-artifact-editors-windows-kit` (removed root files) | 21 | 19 % | same branch, files removed |
| owners | the 15 `closure-probe` events of Finding 1 | 15 | – | closure entries of the command script |
Before the change: 17 stale of 107 looks in the current read ("17 stale (whole 1)" tally), every verdict unnamed.

## What changed
1. `S/🟨️.mjs` `provider.verify`: the catch no longer collapses into `closure-probe`. It names the group's first query row and a cause (`closureFailureWhyV1`): `closure-entry-stale (recipe <12 hex>, moved <kind> <path>)` for a take that met a stale entry, otherwise `closure-probe <code>: <first line> (recipe <12 hex>)`. A cancelled look (`signal.throwIfAborted()`) is rethrown instead of being reported as stale.
2. `S/🟨️.mjs` `takenClosureEntryV1`: a stale stored entry is searched for its first moved row (`firstMovedRowOfEntryV1`: own rows, then layers, budget `closureMovedRowMilliseconds` 400 ms from the limits schema, EN/DE texts), remembered per round beside `staleTakesV1` (`movedTakesV1`, bounded by the entries of one round, no new growth), and attached to `BUILD_TAKE_MISS.movedRow`. The verdict row is then that file: `[graph-stale]` shows `kind file`, the moved path, recorded and current content digest.
3. `B/👁️observation/🟨️.mjs` `projectRowsHoldV1`: the failure callback accepts a non-closure row from the provider (`row.kind!=="closure"`), so a moved file reported by the provider is not dropped.
4. `INFERENCE_LIMITS_V1.closureMovedRowMilliseconds` (schema, EN+DE), `🟨️.d.mts` exports.
Unchanged by design: the stamp memo (E35), the take memo (`staleTakesV1`, verdicts per round), the record key (content-derived, machine-independent; no row format or key change, so fleet records stay fetchable by GR2).

## Tests (repo test-file, one file at a time)
- new `B/👁️observation/🧪️tests/🎯️read-set` 2/0: Python oracle (hashlib) predicts which owners go stale; an edit of a file outside every read set leaves A, B, C valid with 0 recomputations and 0 verdict lines; editing a file read by A and C recomputes exactly A and C, B holds, each verdict names the file with oracle-equal recorded and current digest, once; a provider-reported moved file names the file instead of `closure observations`.
- new `S/🧪️tests/🧭️moved-row` 3/0: rows then layers in order, store-backed layers, budget ends the search, lost layer ends it quietly, failure text names recipe and cause.
- Existing observation tests of the touched code: see landings.md (results appended there).

## Not done / open items
1. Which of the two exceptions produced the 14 `infer` events is unknown until one read runs on this code: the next read prints `closure-entry-stale (recipe …, moved file …)` or `closure-probe <code>: …` per owner. A real before/after count needs that read; I did not start one (play chain, load). The real fix after that read is either (a) let the take ask the farm for a verdict (derive allowed) instead of declaring a stale-entry owner stale, or (b) per-query read sets for the `digest` query. Both change record semantics and belong to a follow-up that measures first.
2. `units/sealed` (81 % of `workspace is stale` lines) is a generation-level (environment owner) cause, not an owner-record cause: fix site `B/🕸️graph/🕰️generations/🧭️plan/🟨️.mjs` (`envStale` for files without an owner, ignoring a declared non-input prefix list in the roster rules `B/🕸️graph/🧭️roster/🟨️.mjs`). I did not land it: the same files also live in the membership facts of the generation, so a plan-only filter would leave the generation stale through facts; it needs the roster and the fact reader changed together and a read to verify.
3. The unit-cache key difference on `CARGO_ENCODED_RUSTFLAGS` (GR1 point 7) is not touched.
