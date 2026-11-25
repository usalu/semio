# Warden unit budgets for the XLSX wasm builds

## Policy (where it lives)
- Authored in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json`, `Policy.unitCeiling` (class `16g`: library 1536 MB, final 4096 MB, serial backend, plus the closed `carving` list).
- Classification (`🏔️ceiling/🟦️.ts`): a unit is `final` only when its rustc command line carries a linked crate type (`cdylib`, `bin`, `dylib`, `staticlib`, or `--test`); everything else is `library`. A unit on the carving list may reach `max(budgetMegabytes, kind budget)`. The warden (`🛡️warden`) reads this at classification time and cancels over-ceiling units; "not on the carving list: carve it or declare it" is the unlisted wording.
- Law of the list: entry budget >= measured, <= measured * 1.1 (pinned by the unit-ceiling test), below 60 % of host memory; the list may only shrink, so every entry names the carve it waits for (`action`).

## Remedy per unit
| Unit | Measured | Declared budget | Action | Why |
|---|---|---|---|---|
| semio_hub_stdio (wasm32-wasip2) | 7677 (20:0x rebuild; first 7095) | 8100 | split-app-fleet | `[lib]` rlib, not a link unit. The wasm component cdylib is the separate `*-component` crate, so classifying it as a library is correct. Reclassifying would hand an rlib the final allowance and the serial-backend contract, which only a cdylib link gets. Honest cause: the aggregate pulls the whole app fleet into one codegen unit. |
| semio_hub_stdio_office | 7939 (20:0x rebuild; first 7458) | 8400 | split-app-fleet | same, office extension aggregate |
| semio_hub_space_component | 5055 | 5300 | serial-backend | genuine cdylib final link, like the other hub components on the list |
| semio_s_artifact_stdio_pdf_model | 3330 (was 2973) | 3500 (was 3200) | split-families | measurement drifted; entry updated, still waits for the family split |
| semio_s_artifact_bim_model_snapshot | 2772 | 2900 | split-families | library over 1536; snapshot model family split pending |

Global ceilings unchanged. All budgets are below 9 GB (largest 8400 MB), so one heavy job at a time through `claude-resource-guard.sh` fits a 16 GB host; stdio and stdio_office must never compile concurrently (guard keeps one heavy job).

Carve follow-ups: split the hub aggregates along the app fleet (per-app crates behind a thin aggregate), thin the space component closure, split the pdf and bim snapshot models by family. Each removal is forced by the verdict ("resolved: remove it from the carving list").

## Files changed
- `🎟️admission/🧬️schema/🔣️.json`: pdf_model entry updated, 4 entries added.
- `🎟️admission/🧪️tests/🧢️unit-ceiling/🔣️.json`: 5 corpus command lines (oracle `🐍️.py` computes the expected ceilings independently).
- `🎟️admission/🧪️tests/🧢️unit-ceiling/🟦️.ts`: pins for the five entries, a < 9000 MB bound for every entry, and the "hub aggregate lib stays library, carved" assertion.

## Tests
- bun test unit-ceiling: 11 pass, 0 fail (includes the Python oracle comparison).
- bun test warden tests: 49 pass, 0 fail.
- beyond-host and charged-peak: 2 pass each. The admission aggregate test file needs `SEMIO_TEST_ARTIFACT_DIR` (use `repo test-file`); its failure under plain bun is environmental, unrelated.

## Does the running warden apply it?
Yes. The warden's source digest includes the admission schema and ceiling module (`hostWardenCodeDigestV1`); it checks every 60 s and restarts itself on change. Verified: running warden (pid 24458, enforce) reports code `cc0126ece274b97a`, equal to the digest computed from the current files. No restart needed. If a stale warden is ever suspected: wait one `codeCheckMilliseconds` (60 s) for self-exit, then the next `repo` call re-ensures it; do not kill it by hand.

## Update: 20:0x rebuild measurements
stdio 7677 MB (cancelled at 7500) and stdio_office 7939 MB (reported at 7900) while both compiled concurrently under compressed-memory pressure. Budgets now 8100 and 8400 (5.5 % and 5.8 % above, within the 10 % rule, below 9 GB). Corpus gained two commands (aggregate above the old budget stays a library; a `rlib,cdylib` office unit is final); tests re-pinned. unit-ceiling 11 pass, warden 49 pass. Running warden (pid 46710) reports code `fd44613e37b07089`, equal to the on-disk digest: it has picked the policy up.

## Does compression inflate the measured peaks? (follow-up, nothing changed)
- The warden already measures on macOS the kernel `phys_footprint` of the process tree (`🔁️loop` footprint port), not `ps` RSS. phys_footprint charges a process for its dirty pages whether resident or compressed/swapped, so memory pressure does not inflate or deflate it: a unit holding 7.7 GB reads about 7.7 GB uncompressed or compressed. Concurrent builds therefore do not raise one unit's reading by themselves; the remaining overshoot is real, as the linker/driver in the tree is added to the unit.
- The carving `measuredMegabytes` come from build telemetry `peakRssBytes` (`ru_maxrss`), a different metric. Under compression RSS can read lower than the footprint (compressed pages leave RSS), so budgets seeded from RSS can sit below what the warden measures. Follow-up: have telemetry record the same footprint metric (peak phys_footprint of the unit tree) so measured and enforced numbers agree; until then enforce with the 5 to 6 percent headroom used here. Also the unit tree includes the linker, which the budget silently covers.

## Update: stdio component and stdio again
- semio_hub_stdio_component (cdylib, final): measured 7817, budget 8300, action serial-backend (like space).
- semio_hub_stdio: measured 8329, budget 8800 (5.7 %; the 10 % law would allow 9162, the under-9 GB pin caps it).
- semio_hub_stdio_office_component: deliberately no entry. An entry's `measuredMegabytes` is a fact the tests pin and the 10 % law is relative to it; a figure derived from the sibling would be an invented measurement. Until measured it stays under the 4096 MB final ceiling (corpus case pins this), so the first run will be reported or cancelled; give the measurement and the entry is added (expected near 8.4 GB, so also keep it sequential).
- Tests: unit-ceiling 11 pass, warden 49 pass. Warden code digest on disk equals the running warden's (37bf6df804e93f51): applied.

## Update: 21:05 rebuild (cap reached)
stdio_component 8792 and stdio 8953 measured; budgets set to 8990 each (the most the under-9 GB rule allows; the requested 9,700 and 9,800 MB break the one-heavy-job-on-16-GB rule and were refused). Margin is 0.4 % and 2 %, so any further growth cancels again. Peaks rose 8 to 13 % between rebuilds, so the units are not reproducible; the fix is the carve (split-app-fleet for the aggregates, and thin the stdio component closure), or an explicit owner decision to lift the 9 GB rule. office_component stays unbudgeted until measured.

## Update: narrow 10 GB exception for hub units (coordinator decision)
- semio_hub_stdio 8953 -> budget 9450 (5.5 %); semio_hub_stdio_component 8792 -> budget 9300 (5.8 %). Actions stay split-app-fleet and serial-backend (carve follow-ups). Rationale: they build only under the single-slot resource guard and the warden's compression danger line still suspends them.
- Test pin: every carving budget <= 10000 MB for `semio_hub_*` crates (hub aggregate and component units), <= 9000 MB for all others (the existing 60 % of host cap, 9830 MB, still applies to all).
- The unit-ceiling test passes (11).

## Recommendation: profile settings versus link memory (not changed)
- I could not find a checked-in `[profile.wasm-prototype]` with lto="thin", codegen-units=16, opt-level "s": the root `Cargo.toml` has no profile section and the profile is materialised by the repo tool, so those values are the coordinator's claim, unverified. The repo's own prototype fixture (`native-build/🧪️tests/🦀️cargo/🧫️fixtures`) uses opt-level 0, incremental, lto=false, codegen-units=256; release uses lto thin and codegen-units 1.
- What is verified in `.cargo/config.toml`: every target carries `-Z threads=0` (parallel frontend on all cores), and wasm32-wasip2 links with lld.
- Likely drivers of 8 to 9 GB: (1) thin LTO over the whole app-fleet closure keeps all bitcode of the aggregate resident at the cdylib link, and a peak that rises 8 to 13 % between runs points to scheduler-dependent parallelism (`-Z threads=0`, many codegen units) rather than a fixed need; (2) the aggregate rlibs (hub_stdio, hub_stdio_office) hold the entire fleet in one crate; (3) opt-level "s" optimises and size-shrinks all of it.
- Experiments to try, one heavy job at a time under the guard: lto=false or "off" for the wasm-prototype profile on the two components; `-Z threads=4` instead of 0 for the wasm target; codegen-units 16 to 64 (more units, smaller each); opt-level 1 for the aggregates. Compare phys_footprint peaks from the warden log. The lasting fix remains the carve of the aggregates.

## Warden
The change is in the admission schema, which is in the warden digest: self-restart within 60 s (checked below in the reply).
