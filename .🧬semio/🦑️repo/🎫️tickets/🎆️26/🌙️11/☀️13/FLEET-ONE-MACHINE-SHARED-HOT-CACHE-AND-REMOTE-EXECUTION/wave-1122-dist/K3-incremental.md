# K3: incremental sessions never reach the fleet

Lane K3, lock `lock.unit-incremental`, 2026-11-22 21:15 to 21:50. UC = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/`.

## 1. Findings: every path that hands `-C incremental` to a unit

| Path | State before | Verdict |
|---|---|---|
| Cargo dev profile / `CARGO_INCREMENTAL` gives workspace members `-C incremental=DIR` | the only source of the flag (no `[profile]` or `.cargo` setting in the repo sets it) | origin |
| Wrapper local compile, `UC/📜️script.ts:114` | strips the session (`withoutIncrementalV1`, `🔏️key:164`) when the unit is fleet-identified (`shared`) and not focused | correct |
| Offload to a worker (`🚀️offload/🗺️paths:152`) | strips the session from the shipped command line | correct |
| Shared + non-focus + session still present | impossible after the strip; `incrementalBuild` (script.ts :216) is therefore false only when the strip applied | correct |
| Focus unit | keeps the session, `incrementalBuild` true, `remote-skip incremental`, no upload after the compile | correct |
| Unit not fleet-identified (no endpoint, breaker, unidentified) | keeps the session, published locally | LEAK (below) |
| Local hit offer, script.ts :177 `offerFleetUnitV1` | offered every local entry to the fleet without knowing how it was compiled | LEAK: a focus or unshared unit compiled with a session and later hit locally was queued and uploaded; the uploader (`📤️uploader:75`) read the newest entry blindly |

The 23 `diverged` evidence entries are all dated 02:54 CET (ids `1795312492...` to `...500...`), which is before round 5 landed the strip (`wave-1122/result-E3.md` section 2: the 64 first divergences were the same class). The live counters show `diverged +0` in the E1 9-minute window. So the strip at :114 works; the remaining hole was the offer-on-local-hit (and the uploader trusting any entry).

## 2. Rule now (simple)

- A unit that is fleet-identified and not focused is compiled without a session (unchanged, :114).
- A unit compiled with a session (focus, or any unit compiled while unshared) is published locally with `session:true` in its entry (schema `Entry.session`, `UnitEntryV1.session`, `publishUnitV1` input `session`).
- No fleet path may carry a `session` entry: the local-hit offer (script.ts :177) skips it, and `uploadUnitV1` refuses it (`entry.session===true` returns false), so even an old spool job cannot upload one.
- `incrementalBuild` moved above `publish` so the same value drives the marker and the existing `remote-skip incremental` branch. The docstring of `wrapRustcV1` states the rule (it used to claim focus units are published like all others).

## 3. The 28-byte `.rmeta` difference (`dsl_record_ownership`)

Measured on `evidence/1795312497913-unit-f4ef7de5b2ef-g1-m10/{kept,refused}.2`:
- The rlib's `lib.rmeta` member is a 480-byte Mach-O stub (the compiler runs with `-Zembed-metadata=no`); the real metadata is only the standalone `.rmeta`. E1's statement "lib.rmeta identical" compared these stubs, so it said nothing about the metadata. Only 1 of 23 entries differs in `.rmeta` (`differing: [rlib, rmeta]`); 22 differ in `rlib` only.
- The 28 bytes are order swaps, not content: 6 single-byte pairs at stride 7 (offsets 22466/22473, 135057/135064, 135636/135643, 249420/249426, 535623/550395) and one block where two adjacent 8-byte values trade places (267816..267831). Two records of the metadata tables are encoded in a different order, with the same sizes and values. No path, no timestamp, no session text.
- Reproduction attempt (scratch, rustc 1.95.0, 1,200-item crate with derives, macros, blanket impls, closures; `-Zembed-metadata=no`): incremental vs incremental `.rmeta` identical, plain vs plain `.rmeta` and `.rlib` identical, incremental rlib differs run to run (the CGU token), incremental `.rmeta` differs from the plain one in the header (byte 9). So a session already changes the metadata between a session and a plain compile, which the key (it ignores the session) cannot tell apart: this is a second reason the strip must hold. The record swap itself did NOT reproduce, so its trigger is in that crate (proc-macro derive output or a hash-ordered encoder table), not the session token.
- Status: not fixed (not reproducible in a synthetic crate; a clean answer needs two plain compiles of `semio_framework_dsl_record_ownership` on one machine, about a cold build of its dependency closure). The law test below compares `.rmeta` bytes, so a recurrence on a real fleet pair surfaces as `diverged` with the rmeta in `differing`.

## 4. Tests

New: `UC/🛰️fleet/🧪️tests/🧷️session/🟦️.ts` (real Cargo under the wrapper, real cache server, `CARGO_INCREMENTAL=1`):
1. Two machines with separate fleet servers compile the same workspace (both compile, both publish): the `core` and `leaf` rlib and rmeta are byte-identical across the machines, no incremental directory exists, no `incremental` skip event, entries carry no `session` (determinism law; the independent comparison is the byte digest of two independent compiles by plain Cargo/rustc).
2. A focused unit (`leaf`) keeps its session (`incremental` directory for `leaf` only), its entry is `session:true`, `remote-skip incremental` is logged, a local rebuild hit does not queue it in the spool, and a third machine finds the dependency `core` in the fleet but never `leaf`.

Runs (bare `bun test` per file; `repo test-file` for the new file):
- new session file 2/0 (repo test-file, 83.7 s)
- `🧪️tests/🌱️incremental` 1/0 (repo test-file; existing: session kept in lane, still published locally)
- `🛰️fleet/🧪️tests` 18/0
- `🧪️tests/🕵️leaks` 2/0; uploader 4/1 (spool-half-fill test, 20 s, load); unit-cache main file 21/7 (ENFILE, sealed snapshot, `plain Cargo build` byte digest differs from a plain build: not session related because that test runs with CARGO_INCREMENTAL=0 and the key files K1/K2 are editing; not verified on a quiet tree).
- Per the 21:55 addendum no more bare bun runs; the ones above were `./`-scoped inside the member before it was issued.

Note: `repo test-file` on four other files of this member failed in 2 to 6 s with no body output (not investigated; the same files pass under bare `bun test`, see counts in landings).

## 5. Open
1. Reproduce the `.rmeta` record swap on the real crate with two plain (non-incremental) compiles on one machine.
2. Fleet machines that still run pre-round-5 wrappers (the 02:54 evidence) can still upload session bytes; the uploader now refuses only session-marked entries, not entries written by old wrappers. Evidence purge of the 23 stale entries is for the retention lane.
