# Native Timeout Cache Audit

Read-only audit, 2026-11-05. No Native launch/restart, timeout widening, environment alteration, cache mutation, recursive artifact walk, or build-stage edits.

## Recommendation

A fresh genuine invocation of the same public shared-store command is supported after terminal settlement. It is not a resume of the expired child/control. No inspected Cargo owner contract requires splitting declared stages after a timeout, and no expired invocation envelope should be reused. Root owns any next command:

```powershell
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path Cargo.toml -p semio-framework-plugin-command --lib
```

repo cargo --help executed successfully and advertises this shared-store compile ingress; it has no resume switch. Authored launcher schema supports build/check/test/etc as independent compile commands and contains no persisted failed-attempt prohibition. Internal automatic retryWhenFull is specifically image-capacity recovery, not timeout renewal. Do not wrap expired authority in a synthetic retry or fabricate parent environment.

Root reports Native57735 terminated exit1 with cargo:test timeout1183349ms after compilation (last visible crate pack-async), not a Rust test-body/source error. This audit does not claim any native test passed or that a fresh invocation will finish within its genuine deadline.

## Concrete Surviving Cache Evidence

Only lane top-level metadata, one bounded55,274-byte event ledger, one3,193-byte unit receipt, and direct object existence checks were inspected.

Lane `C:/git/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p757668` still contains b,t,m,v,lane.json. lane.json has crateKey5fcf378cceab, crates=[semio-framework-plugin-command], correct root agent identity, mtime2026-11-05T02:16:26.256Z. Live-marker directory v is empty. No helper that deletes stale markers or mutates pool state was invoked.

Lane event ledger `.../fixed/m/u/x/p757668.jsonl`:202records, all after worker6260 creation1793841366141; first1793843806620,last1793844793180. Counts:27hit,87miss,87publish,1bypass. Every observed miss has a corresponding successful publication. This is concrete reusable unit work rather than an assumption from elapsed compilation.

One exact last-publication receipt:
`.../m/u/e/27/278ac55dfde36cedcbae14b74fef2330e2e3f16ecccb7a6d991b5a92ce5d5f26/001793844793169-d6ea51.json`.
Crate semio_framework_pack_record_retained, created1793844793169,10recorded dependencies, no env-dependency rows. Three recorded outputs are present in the content-addressed object store: .d8194bytes, .rmeta564354bytes, .rlib2539670bytes. provenance:null here means measured resource provenance was not recorded; it is not a missing-output receipt. No object bytes or whole dependency tree were read.

Lane b/.rustc_info.json records successful rustc identity1.99.0-nightly9f36de775(2026-07-19),hostx86_64-pc-windows-msvc. This is compiler identity evidence, not compilation success of the requested test crate.

Current root Cargo.toml stat:129,519bytes,mtime2026-11-05T01:42:33.580Z. Cargo.lock:745,918bytes,mtime2026-11-05T02:16:44.576Z. No earlier content digest was available in this audit, so byte identity with the first attempt is not asserted. Fresh ingress must revalidate actual current sources/lock/configuration.

## Source Ownership And Reuse Rules

Repository library prefix `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`:

- Cargo launcher `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:107–120`: shared lane acquisition executes body, and finally releases headroom and lane claim; it does not delete b/t on failure. Isolated store has explicit deletion-on-settlement, but this command selected shared.
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts:39–48`: lane release removes heartbeat marker and touches metadata, preserving build/target contents.
- Pool owner `🧵️lane/🏊️pool/🟦️.ts:28–34` prefers newest free same-crate-key lane. Same exact package command is therefore eligible for warm p757668, but no guarantee: a peer may claim it, and GC/pool pressure can evict idle lanes. A new lane can still restore compatible shared units.
- Launcher source/planning memo APIs lines124–127 retain source admission and graph memo state; every fresh admitted run still performs genuine source/graph/configuration checks.
- Unit wrapper `unit-cache/📜️script.ts:72–73`: focused selected package with incremental state compiles in its lane rather than publishing itself to shared unit cache. Unchanged focused outputs can benefit from Cargo's existing lane freshness/incremental state; unfinished final focused compilation is not a completed reusable receipt.
- Same wrapper lines84–104: lookup/restore valid entries first; only status0 compilation reaches publishUnitV1. A timeout of a later unit does not roll back publications of earlier successful dependencies.
- `💾️store/🟦️.ts:50–59` lookup validates matching key, dependency digests, observed environment and output presence. Lines87–104 publish content-addressed output objects and atomically rename entry last. Partial/failed units are not successful entries.
- `🔏️key/🟦️.ts:107` keys include namespace, toolchain, normalized args, source SHA, extern digests and selected environment. Same command alone does not prove same unit inputs; changed source/config/toolchain/dependencies properly cause misses.
- Restore stamps outputs at restore time and rewrites lane-specific dep-info, avoiding stale output-time loops. Shared cache is cross-lane and has its own provenance/freshness checks.

The launcher finally may invoke owned pressure relief/GC; that is the supported policy, not permission for manual cache copying/deletion or forcing lane paths. The next invocation should let the real launcher choose lane/admit capacity and validate provenance. No hard-coded CARGO_TARGET_DIR, copied artifacts, spoofed env, custom rustc, or widened authority is needed.

## Limits

Cache reuse is supported and87successful publications are concretely present in the lane ledger; only one receipt's object existence was independently sampled. I did not validate all87receipts or predict exact future hit rate. Compilation timeout provides no failing behavioral test evidence. Splitting into build-only and test stages could change orchestration/cache focus and requires an explicit declared design; it is not mandated by the current public Cargo ingress after this terminal attempt.
