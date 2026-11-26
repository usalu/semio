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

## Update: -Z threads=1 rebuild
stdio_component 9306 -> budget 9820 (hub exception; every entry must also stay below the 60 % cap share of 16 GB = 9830 MB, so 10000 is not reachable); bim_model_snapshot 2957 -> 3100; fem_2d_model (wasm32-wasip2, library) 1549 only 13 MB above the 1536 library ceiling: added as a carving entry with budget 1640 (action split-families), the honest form since the list is the only way to declare an overshoot. semio_hub_stdio stayed under 9450 with threads=1, unchanged. unit-ceiling test passes.

## Update: IFC catalog entity shards
The carving list matches exact `<crate>|<target>` (no pattern), and an entry's measured peak is a pinned fact, so a family entry would invent measurements for unmeasured shards. Per-shard entries (wasm32-wasip2, library, split-families): entities_00 3097 -> 3270, _04 3095 -> 3270, _05 3048 -> 3220, _06 1694 -> 1790. Other shards (_01 to _03, _07 and up) get an entry once measured; corpus pins _01 as unlisted. A pattern entry would need a schema, ceiling code and oracle change plus a per-family measurement rule; follow-up if the shard count keeps growing.

## Update: office component measured; the configuration that fits
semio_hub_stdio_office_component final link measured 4231 MB (log range 4103 to 4231): entry added, budget 4470 (5.7 %), serial-backend. Its first-in-class unbudgeted state is over.
Configuration that fits: `lto = "off"` plus `-Z threads=1` for the wasm-prototype builds. With it the stdio component link produced no warden event (earlier runs with thin LTO and threads=0 reached 8792, 9306 and 9879 MB in the log; its entry stays at 9306 / 9820 as the conservative ceiling and should be re-measured and lowered on the next clean lto-off run, since entries may only shrink once they fit). The profile experiment of the earlier recommendation is thereby confirmed: LTO and frontend parallelism drove the link memory.

## Update: all IFC shards from the warden log in one pass
Mined the whole warden log (a superset of the last 2 hours) for every `semio_*ifc*` wasm32-wasip2 unit reported or cancelled above its ceiling or budget, took each max peak, budget = max * 1.055 rounded up to 10 MB, action split-families. 24 entries added (ifc2x3 catalog entities 00 to 06 and types 00; ifc4 catalog entities 00 to 07 and types 00; ifc4x3 add2 entities 01, 02, 03, 07, 08, 09 and types 00), and 3 raised because they drifted above their budget: ifc4x3 add2 entities_04 3271 -> 3460, _05 3282 -> 3470, _06 1888 -> 2000. Notable: ifc4 entities_06 2908, ifc4 entities_01 2546, ifc2x3 entities_02 2854. The peaks mix runs with different settings (some before lto off and threads=1), so they are upper bounds; a clean re-measure may shrink them. Shard 99 pinned unlisted in the corpus. unit-ceiling test passes.

## Update: re-scan of every listed unit against the log maximum
ifc2x3 entities_02 peaked at 3242 -> budget 3430; entities_03 at 3107 -> 3280 (was set from 1880). Re-scanned the whole log for the max `peaked at N MB, above` line (reported and cancelled) of every carving entry; 19 entries were below their max and were raised (budget = max * 1.055, rounded up to 10): pdf_model 3525/3720, binary_io_registry 3167/3350, dwg_snapshot_objects (wasm32-wasip2) 2457/2600, semio_cad_member 1830/1940, semio_kit_member 1788/1890, bim_model_snapshot 3016/3190, ifc2x3 entities 00 1841/1950, 01 1796/1900, ifc4 entities 00 to 07 (1647 to 3125 -> 1740 to 3300) and ifc4 types_00 1780/1880.
Not raised: semio_hub_stdio (log max 9743) and semio_hub_stdio_component (log max 9879). Those maxima come from the thin-LTO, threads=0 runs; raising to max + 5.5 % would exceed the 9830 MB cap share (10280 and 10430), and with lto off plus threads=1 neither unit produced an event. Entries stay 9450 and 9820. If either is cancelled again under lto off, the cap itself needs an owner decision.
The log is cumulative, so a max can come from an older configuration; budgets are upper bounds and may be lowered after a clean measurement.

## Update: carving families for generated shards (schema-first rule)
Problem: per-shard peaks of generated IFC shards vary run to run (ifc2x3 entities_00 went 1841 -> 2868 MB, _03 1880 -> 3440 MB), so entries set from one reading are overrun and an unlisted shard is cancelled at 1536 MB.
Rule implemented: `Policy.unitCeiling.families`, a closed list of homogeneous generated shard families: `{family, pattern (a regular expression over the whole crate name), target, measuredMegabytes, budgetMegabytes, action, source}`. `measuredMegabytes` is the class measurement: the largest peak observed over any shard of the family, `source` names the warden log lines it comes from; `budgetMegabytes` is the class measurement plus the variance margin (5.5 %, never above 10 %, below the 9830 MB cap share). A unit with its own carving entry uses it, else the family it matches (`carvingOfUnitV1`, also used by the verdict, the cold-budget warning and the beyond-host test, and mirrored in both Python oracles). Any shard, measured or not, gets the family budget, so a new or unlucky shard is no longer cancelled at 1536 MB.
Families: ifc2x3 (entities and types) 3440 -> 3630 MB; ifc4 3125 -> 3300 MB; ifc4x3-add2 3282 -> 3470 MB. The 28 per-shard ifc entries were removed (the list shrinks; a test forbids an entry that a family already covers). The coordinator's two readings are covered: ifc2x3 entities_03 3440 and entities_00 2868 are both below 3630.
Tests: unit-ceiling (family bounds, no entry inside a family, corpus cases for a family member, an unmatched name, a wrong target and the ifc2x3 variance) against the Python oracle: 11 pass; warden 50 pass (fixtures got `families: []`); beyond-host and charged-peak pass.

## Does swap change what the warden measures?
The metric does not change under pressure: the warden reads the kernel `phys_footprint` of the unit tree, which charges compressed and swapped pages to the process the same as resident ones, so swap neither hides nor adds memory. What does change is the sampling: the tick is 2000 ms when calm and 500 ms when tight (`calmMilliseconds`, `tightMilliseconds`), so under pressure a transient spike is caught that the calm tick would miss, and a long swap-slowed compile also holds its peak for longer. That explains part of the apparent spread (a peak seen only in tight ticks) without the measurement being wrong; the larger part is real run-to-run variance of rustc over a generated shard (codegen unit scheduling). Follow-up, not changed: record the peak from one source (telemetry footprint of the tree at a fixed sampling rate) so that class measurements do not depend on the host state.

## Pressure cancels of the guarded heavy link (hub components)
Findings:
- `serial-backend` (and every other `action` of a carving entry) is a free-text label naming the carve it waits for; no code reads it. The serial back end itself is enforced by the scheduler from the crate type (a granted cdylib final appends `-Z no-parallel-backend`), not by the carving list.
- Before this change the warden had no notion of a declared heavy unit. Victims were ordered by criticality, then least work lost per megabyte (elapsed seconds per MB), then rank; `keepRunning` (1) keeps the dearest running subject. A young 6 GB link has a tiny elapsed per MB, so it ranked as the cheapest critical victim and was the first suspended; any unit suspended longer than `suspendMaximumSeconds` (120 s) was then an overdue candidate. That is the "zu lange angehalten" cancel at 6,015 MB, a cost of the ordering and the short maximum, not of the budget.
Change (schema first, principled, nothing about danger weakened):
- A declared final link is a subject whose ceiling is of kind `final` and listed on the carving list (the law lets it run alone, serial, in one slot). It is the dearest among compile units: `order` puts it after every other build-critical subject, so it is suspended last and, being the dearest, is the one `keepRunning` keeps running while other agents' units are suspended first.
- Its suspend maximum is `Policy.heavySuspendMaximumSeconds` (600 s, schema, bilingual description) instead of 120 s.
- Unchanged: the danger-line cancel (suspended units past `pauseDeadlineSeconds` are cancelled in order when available memory is below the danger line, the heavy link last), the ceiling and cap cancels, and the overdue resume/cancel rules for severe shortages.
- Tests: five fixture decisions (the Python oracle writes the expectations): a young declared link is kept running and the old library unit is suspended; a link suspended 130 s is not overdue where an ordinary unit is; 700 s is overdue; the danger line cancels a suspended link; an unlisted final and a listed library are not declared links. One older incident fixture changed its expectation (a listed final suspended 130 s is no longer resumed as overdue; the case name says why). warden 54 pass (one mounted real-process test was flaky once, passed twice after).
Remaining exposure: the warden can still cancel the link on "memory pressure is critical" when available memory is below the danger line (severe), by design; and a suspended link that cannot be paged back in without exhausting the room is cancelled by the page-in rule. Both are host safety. The lasting fix is the single-slot discipline: other agents' units should not run while the heavy link does.
