# Result M6: no metric without a source, fleet scope, overhead and publication volume

Paths below `🧰️framework/`. `M` = `🔨️modules/🏃️process/🏗️build/📈️metrics`, `W` = `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden`, `B` = `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `R` = `B/🌐️remote-execution/🛰️sidecar`. Kept scripts: `🧰️scripts/M6/`.

## 1. The 8 metrics now have a source (`Unsourced` is empty, pinned by the law test)
| metric | scope | source |
|---|---|---|
| `process.repo.cpu` | origin | the warden. Its process table (`ps`) had no CPU column, so the brief's "already sampled" was not true: the quick table now has `time=` (no extra process, same `ps`), rows carry `cpuSeconds`, the classifier reports the owned rows, the loop adds the CPU each classified process gained between two ticks (an exited compiler keeps its share, first tick = baseline, a young newcomer counts whole, an old newcomer does not) into `subjects.cpuSeconds` of `state.json` (cumulative). The collector turns two warden states into a rate (100 = one core); gap on restart, long silence, and where the table has no CPU column (Windows). |
| `exec.queue.running`, `exec.queue.queued` | fleet | `scheduler.status()` in the scheduler process (no file), fleet collector `remote-execution` |
| `exec.offload.in` | fleet | sum of `offload.<origin>.offloadedIn` of the scheduler's persisted counter ledger (cumulative over restarts and machines, only grows); source changed from `offload-counters` to `remote-execution` |
| `cache.remote.entries`, `cache.remote.bytes` | fleet | `server.store.usage()` in the cache server process (no walk, no file), fleet collector `remote-cache-status` |
| `graph.read.facts` | origin | `sources.caches.rowStore.rows` of the graph service's `slice-memory.json` (the rows it read into its row store; already read for the owners), relabelled "Facts read into the graph index" |
| `cache.local.bytes` | origin | `bytes` of the CAS `v1/census.json` (written by every collector scan, re-parsed by mtime only); `casCensusPathV1` moved next to `casCounterDirectoryV1` in the store, the collector re-exports it |

## Fleet scope (schema-declared)
- Catalogue `scope: origin|fleet` (new `Scopes` definition en/de, every metric has one); 5 are `fleet`: the three `exec.*`, `cache.remote.entries/bytes`. Collectors carry a `scope` too, the law test checks each id's catalogue scope equals its collector's.
- Not a pseudo-origin: `<store>/v1/fleet/metrics/<source>/t*.ring` (`fleet/` already holds `cache.json`; `origins/` stays machines only). One writer per source: the machine that runs the scheduler writes `remote-execution`, the machine that runs the cache server writes `remote-cache`; each ring holds only the ids of its source. Local rings `<host-capacity>/metrics/fleet/<source>/`. Started from `sidecar/🗓️scheduler` and `sidecar/🗄️cache` (`startFleetScopeMetricsV1`), 1 s timer, collector cadence 10 s. A failover starts a new local ring on the new host and overwrites the published one (history of the old host's ring is lost, by design: no election, no sync).
- The origin rings keep the whole catalogue layout (fleet ids stay as empty slots, +5 x 36 B per row) so a reader may ask any id without knowing scope: the dashboard (M4) reads `origins/<o>/metrics` with ids from its group table, which contains the fleet ones, and an unknown id is an error. For M4: the per-origin offload-in, queue running/queued and remote cache entries/bytes columns are now empty on origin rings; read them from `fleet/metrics/<source>/` (`read_series` on that directory).

## 2. Overhead (this Mac, bun 1.4.2, load average 38-47 during the runs, real sources, 300 s, publication to a scratch store)
Cuts: published `t0` is now the real 300-bucket ring (the extra 60-bucket live ring and its second write per tick are gone); no file copy per look; `scanHeadV1` reads 16 bytes per row instead of the whole file (and is skipped for freshly created files), which was the 98 MB transient (13 MB of zero-filled buffers plus copies).
- Before (M3): 0.50 % of a core, transient peak 98 MB, +12 MB at the end.
- After: **0.206 % of one core** average over 300 s, including idle bun (0.07 %, an empty 1 s timer measured alone), the first step (23 ms CPU, creating the rings) and the 69-file first publication. Steady steps 0.095 %. Per step CPU: collectors about 33 us per tick and about 470 us per 10 s slow step, ring write 17 us, publication look about 100 us (see `profile.ts`, `profile2.ts`, `profile3.ts`: 304 us per step with slow and publish steps included = 0.03 % in a tight loop; the real run is higher because every step starts cold after a 1 s sleep).
- Memory: **peak +10.0 MB over the readiness closure** (20 ms sampling during the first 5 s, per-step afterwards), end -6.7 MB (GC). The marginal import of the collector in the readiness service is 0.5 MB (the unit counter, warden, cas, nx, ledger modules are already loaded by it). Ring open dropped from +19 MB (fresh) and +17 MB (reopen) to +6 MB and +1 MB (`stages.ts`). Standalone, without the readiness closure, the product sources import adds 27 MB: that is the shared closure, not the collector. Budget 30 MB / 0.5 % met with a margin; target < 0.25 % and < 20 MB met.
- Not measured: Linux and Windows.

## 3. Publication volume (`volume.ts`, 4 simulated hours with the real publisher, 5 s looks)
Rows are appended in place (rows first, header cursor last); a tier is created whole only at its first row. Per machine: hour 1 4.95 KB/s (includes the first whole copies of t0..t3, about 7 MB once), hour 2 4.30 KB/s (t4 whole copy), **steady 2.80 KB/s** (t0 2.5 KB/s, t1 0.25 KB/s, t2 0.04 KB/s), against the 75 KB/s budget (M3: about 75 KB/s). The test asserts < 4 KB/s steady over 200 s. A restarted publisher continues from the published header.

## Tests (`repo test-file`, one file at a time)
- metrics `🧪️tests/🟦️.ts` 12/0; collector `🛰️collector/🧪️tests/🟦️.ts` 21/0 (new: scope law, empty unsourced list, repo CPU rate vs a Python oracle, incremental publication incl. an independent Python reader of the appended rows, restart continuation, layout change, volume, fleet-scope collector); product sources `R/📈️metrics/🧪️tests/🟦️.ts` 5/0 (facts, census bytes, scheduler and cache sources, fleet starter); ready `R/🚦️ready/🧪️tests/🟦️.ts` 58/0; cas 70/0 and hits 1/0; dashboard fleet 4/0.
- warden `W/🧪️tests/🟦️.ts` 50 pass, 2 fail: `every classification equals the independent Python oracle ...` and `ceiling: a carving-listed unit gets its declared budget`, both about the compile-unit carving ceiling (budget numbers), not about the CPU column; neither touches the code I changed (new tests: CPU column parse vs Python, loop accounting 0, 6, 10 and no field without the column). Not confirmed pre-existing by a run without my change (no live experiments).
- dangling-imports exit 0 for the metrics member, the warden, the product metrics, scheduler, cache, cas store and collector folders.

## Not verified / notes
- Linux and Windows runs; the macOS `ps time=` form and the Linux form are parsed (Python oracle), Windows rows have no CPU, so `process.repo.cpu` is a gap there.
- The Rust reader was not rerun (reader and file format unchanged); published t0 is now 300 buckets instead of 60.
- Sidecars were not restarted live; scheduler and cache sidecar modules load and bundle (`bun build`), no unit test exists for them.
- Existing published rings of M3 (60-bucket t0, live ring) are replaced at the next look (layout/size differs, whole copy).
