# F45 result: F42 identity conversion verified, live impact checked

## Live impact (21:08-21:12 CET, before any edit)
- Replica daemon on this Mac (pid 44501, started 20:42, after F42) holds `.cache/f/daemon.lock` with `startMs`, publishes (`published`/`applied` events 21:06), relay log shows bootstrap progress.
- All origins refresh attest/memory/pool at 21:08 (m04 stale since 20:45, m09 never ready: both before F42). Other machines' store tmp names already use `<origin>-<pid>-<startMs>-<hex>`: they run F42 code.
- No startMs/identity error in replica/relay logs, ready logs, sidecar logs (only ENFILE from load 78-125). No wedge observed.
- Pre-emptive fix landed anyway: a record WITHOUT `startMs` reads as `startMs: 0` (pid-only judgement); a malformed `startMs` stays a hard error. Writers never write 0.

## Tests (one at a time, repo test-file, load 45-98), final state all green
identity 7+4 (law); unit-cache zombie 4, light-slot 4, live-holder 2, headroom 20, temporaries 14, units 14, registry 2, offload 10 (first run timed out 180 s under load, rerun green), lane 2, unit-cache main 29, fleet 20; cas 68, pins 2; graph admission 55; journal 132; resilience 26, beat 6, killpoints ok; consolidate 11; sidecars 12; replica main 69, resilience.ts 25, boot.ts 24, killpoints 4, sidecars.ts 3; remote-cache retention 18, server 10, collector 8. No `💾️store` test file exists.

## Faults fixed
1. supervisor copied alone to `.cache/f/boot/stage.ts` could not resolve the identity import (product bug, real boot fallback). Now self-contained twin.
2. cas test: import inside a Python string, missing real import.
3. Stale fixtures: cas pins hard-error assertion, replica legacy staging names.
Load-only timing flakes (resilience yield, beat) passed on rerun at load 44.

## Open
Area lock `replica` was an ownerless folder; I did not own it. Not converted (63 files in the law baseline) unchanged.
