# F80: coordinator volume reclaim

## Free space (`df /System/Volumes/Data`)
- Before: 926 GiB, 36.7 GiB free (96 %).
- After (measured twice): 123 GiB free (87 %), later 114 GiB while other writers ran. Target (>= 82 GiB) met, about +86 GiB.

## Deleted, by class
| Class | Path | Size | Safety check |
|---|---|---|---|
| Stale fleet-store test staging | `/Users/Shared/semio-store/v1/tmp/procedural3d-cold-units-am00uln3` (idle-native 45 G, verified-products 4.8 G, ...) and `draw-guest-fcc8.21L2Yt` | ~51.6 GiB | mtime 00:23-03:14 (>6 h), `lsof +D` empty, no process names them |
| Wasmtime compiled cache | `~/.semio/cache/wasmtime/*/*.cwasm` | ~34 GiB | all 378 files 55+ days old, `lsof +D` empty, regenerable (cache miss recompiles) |
| App caches | `~/Library/Caches/copilot` 3.8 G, `Google` 1.9 G, `~/.cache/uv` 2.1 G, `ms-playwright` 0.54 G, `com.google.antigravity` 0.4 G | ~8.8 GiB | `lsof +D` empty |
| bun install cache | `~/.bun/install/cache` entries not read for 7 days | ~6.5 GiB | no `bun install` running |

Not touched: every `.cache/b/*` image (18 attached), `~/.sinc/play`, graph read tree, CAS (`~/.cache/semio/cas`, 53 GiB), `semio-store/v1/origins`, this session's scratchpad (4.1 GiB, lane scratch of the running wave), ticket `🗑️generated` folders (total < 0.5 GiB, so nothing to gain), no zombie or orphan processes found worth killing, `/private/var/folders/*/*/T` semio dirs are < 0.3 GiB.

Candidates left for the user (not stale-proven): `~/Library/Containers/com.docker.docker` Docker.raw 70 GiB (Docker not running, last written Nov 16), `~/Library/Application Support/Claude/vm_bundles` 11 GiB, `~/Zotero` 22 GiB.

## Retirement gaps found (no code change)
1. `~/.semio/cache/wasmtime` (`🔌️plugin/🖥️host/🔧️engine/🦀️.rs`, `default_compiled_cache_root`) has no size cap or age retirement: 34 GiB accumulated with 55-day-old entries. Needs a startup prune (LRU by mtime, cap relative to free space) next to `store_compiled_component`.
2. `semio-store/v1/tmp/<test>-cold-units-*` staging (a test or lane producing 51 GiB of idle-native copies) has no owner-lock or TTL sweep; nothing reclaims it when the owning run dies.
3. Neither class is visible to the CAS reserve/guard, which only accounts for its own store, so the volume filled outside it (as F79 noted).

I did not change code: gap 1 is a Rust edit in the wasm host that restarts supervisors on 14 machines during the running seal read and cannot be tested single-file with `repo test-file`; it needs its own lane (cap + prune function with a language-agnostic test of mtime ordering). Recommended follow-up lane.
