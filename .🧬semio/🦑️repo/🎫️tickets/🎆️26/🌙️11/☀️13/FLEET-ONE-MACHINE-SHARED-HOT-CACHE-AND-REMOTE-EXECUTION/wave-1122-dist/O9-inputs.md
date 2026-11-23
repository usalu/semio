# O9: offload inputs, why "dependency closure" declines

## Finding
- The `inputs-too-large` / `dependency closure` event has exactly one producer: `dependencyClosureV1` returning `undefined` (`🌳️inputs/🟦️.ts`, called at `🗺️paths/🟦️.ts:257`). It returned `undefined` in three cases: `readdirSync` of ANY `-L dependency=` directory threw, ANY artifact file could not be read, or more than `maximumClosureFiles` (32768) artifacts were selected. No byte limit is involved in that function (`maximumInputBytes` guards only native search dirs and OUT_DIR; `maximumUploadBytes` is applied in `📜️action/🟦️.ts:86` to the bytes the content store lacks, already "bytes actually uploaded").
- The live events took 69 to 140 ms. Scanning the closure of a real unit takes minutes under the current load (below), so those declines are the early IO-failure exits: a `-L dependency=` directory (cargo lists one per dependency unit, 224 for kernel) that is missing or unreadable, turned into a misleading "too large". The count limit cannot trip (the play tree has 268 unit dirs, ~110 crates reachable from kernel).
- I could not reproduce the vanished directory itself (the play tree dirs are all present now); the diagnosis rests on the timing and on the code paths.
- What the walk counts: only the extern directories (`-L dependency=` set, one per unit out dir), never a whole lane dir; per selected artifact it ships rlib + rmeta (+ dylib) and reads every file as text for 16-hex extra-filename tokens. Dep-info of the slot lists only sources and the unit's own outputs here, not externs, so it cannot be the oracle; the extra-filename tokens in the metadata are the exact reference list rustc itself uses.

## Fix (requester side only, applies to the next build start)
1. Missing or unreadable directories and vanished files are skipped; if a needed file is truly unreadable the shipping step declines `unreadable-input` naming the path. Only the artifact-count bound still declines, and its detail now says so.
2. Metadata is read from the rmeta only (rlib only when no rmeta exists): same reference list, a fraction of the bytes read.
3. A unit whose crate types are only `lib`/`rlib` loads dependencies from rmeta (cargo itself passes rmeta externs to such units); indirect rlibs are no longer shipped. Directly named externs and every file for linking units (bin, proc-macro, cdylib, dylib, staticlib, wasm finals) are unchanged.
4. Limits stay on uploaded bytes (CAS dedup makes repeats free).

## Measurement (play tree build dirs, 268 unit dirs / 775 files / 837 MB, direct externs of `semio_framework_kernel`, plan only, `taskpolicy -b`)
- Before: 440 files, 546 MB shipped (read all of them for tokens; ~5 min at load 55).
- After (library unit): 221 files, 304 MB (rmeta only); linked units unchanged. Verdict for the kernel closure: planned, no decline.
- Not measured: a live offload-run after the change (cannot restart the play build).

## Tests
- New: a directory of 300 unrelated rlib/rmeta pairs plus a 2-crate chain ships only the chain (linked: 4 files, library: 2 rmeta), a missing directory is skipped.
- Updated: closure test (missing dir -> empty), split-roots (indirect rlib absent), stub oracle (metadata read from rmeta). The rustc-compiles-the-plan oracle test passes with indirect deps rmeta-only.
- `repo test-file`: offload 41/0, paths 4/0, cluster 11/1. The cluster failure is an extra `.semio-key` sidecar pair in the unit dir assertion (peer work on key sidecars), the unit still ran remotely with zero local rustc.

## Open
- The exact vanished directory in the live run is unidentified; the next event, if any, carries no new detail for IO skips (they are silent now). Consider an `offload-declined` counter for skipped dirs.
