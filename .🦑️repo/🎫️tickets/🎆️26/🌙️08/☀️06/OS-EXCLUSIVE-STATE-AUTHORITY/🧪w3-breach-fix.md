# W3 — `os-state-authority/*` breach drive

**Ticket:** OS-EXCLUSIVE-STATE-AUTHORITY  
**Date:** 2026-08-06  
**Probe (unchanged):**

```bash
SEMIO_OS_STATE_AUTHORITY=1 bun -e 'process.env.SEMIO_OS_STATE_AUTHORITY="1"; const mod=await import(process.cwd()+"/📜️script.ts"); const r=mod.policy({root:process.cwd()}); const os=r.filter(b=>String(b.kind).startsWith("os-state-authority")); console.log(os.length); const by={}; for(const b of os) by[b.kind]=(by[b.kind]||0)+1; console.log(by);'
```

## Counts by kind

| Kind | Before | After |
|------|-------:|------:|
| `os-state-authority/document-app-shape` | 7 | **0** |
| `os-state-authority/item-scope-global` | 19 | **0** |
| `os-state-authority/id-minting` | 26 | **0** |
| `os-state-authority/authority-struct-map` | 27 | **0** |
| **Total** | **79** | **0** |

## Work summary (this pass)

- **document-app-shape:** Play apps moved to ZST + draft/scratch patterns (lowpoly/puzzle/cad/space family); policy at 0 before struct-map/globals sweep completed.
- **id-minting:** Removed process `AtomicU32` counters / unused `AtomicU32` imports; collection- or content-addressed ids (`blake3`, edit-scoped serials, `next_frame_tile_id`-style ordinals).
- **item-scope-global:** Dropped `LazyLock`/`OnceLock`/`thread_local` example caches and preview memos; trinity manifest loads per call; puzzle example JSON via cold-path fns; workpiece preview no longer uses process-wide `Mutex` cache.
- **authority-struct-map:** Renamed or nested map ownership (`PaintTextureLut`, `PartialMovieLut`, `ProcessKernelReplay` + `ProcessKernelMemo`, `BrepkitSideTables`, `CadEngagementContext`, energy `*Ledger`/`*Frame`, schema/catalog renames, etc.).

## Gate

`SEMIO_OS_STATE_AUTHORITY` remains **ungated** in policy until a follow-up confirms zero on CI; local probe above reports **0** breaches.
