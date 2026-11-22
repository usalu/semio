# E1: spool-full status and output divergence (read-only audit)

Date: 2026-11-22, 21:04 to 21:13 CET. No repository state was changed. Raw dumps are in the session scratchpad (`e1/raw/`, `e1/classify.py`, `e1/arcmp.py`), not in this folder.

## 1. Is spool-full still happening?

Answer: no, not in the 9-minute window. It is historical. Two caveats below.

Snapshots: four `repo fleet status --json` reads. The first two attempts hit the 5 s reader timeout for the `fleet` section, so the usable baseline is the read at 21:04:01. The last read is at 21:13:04, 542.8 s later (about 9 min, short of the requested 10). Intermediate reads at +450 s and +506 s agree.

Per-machine deltas, baseline to last read (cumulative counters, so delta = new events):

| Machine | hit | miss | skipped | reasons that moved | offloadedIn |
|---|---|---|---|---|---|
| g1-m08 | +275 | +541 | +63 | key-incomplete +59, path-dependent +3, upload-failed +1 | +84 |
| g1-m10 | 0 | +851 | +13 | key-incomplete +12, upload-failed +1 | +5 |
| g1-server | +1 | +14 | +1 | key-incomplete +1 | +27 |
| g1-m01, m02, m06, m13 | 0 | 0 | 0 | none | +180, +64, +297, +115 |
| g1-m12 | 0 | 0 | 0 | none (offloadedOut +767) | 0 |
| g1-m03, m07 | 0 | 0 | 0 | none | 0 |

Fleet totals over the window: `spool-full` +0 (no machine), `upload-expired` / `upload-expired-down` +0, `diverged` +0, `publish-interface` +0, `unrecorded` +0. Reasons that moved: key-incomplete +72, path-dependent +3, upload-failed +2. Hits +276, misses +1,406.

Cumulative `spool-full` is unchanged on every machine: m02 52,586; m06 50,412; m07 12,845; m10 10,292; m12 836.

Caveats:
- m10 is the only machine that compiled a lot locally in the window (+851 misses) and it recorded no new `spool-full`. That is the evidence that the E74 round-2 spool fix holds under load on m10.
- m02, m06, m07 and m12 made no local compiles in the window (only offload in or out), so zero growth there is not proof of a fix. Their backlog never drained, but they also produced no new spool candidates.
- Code versions: the attestation digest (`/Users/Shared/semio-store/v1/origins/*/attest.json`, field `code`) is `f4016b8a81a0` on every live origin (m01, m02, m03, m06, m07, m08, m10, m11, m12, m13, server), with wall times of 21:12 (m11 at 20:56). m04 (`5b85cc7ffbe2`, offline 5.9 h) and m05 (`d4dfecabb3e3`, offline 96 min) run older code. m09 has no attestation. The attestation shows the replicated code, not when each wrapper process restarted, so a machine may still run the previous wrapper until it restarts (E74 notes this).

## 2. Output divergence

Evidence list: 119 entries (`project-inference` 64, `unit` 23, `owner-pool` 21, `closure` 11). All 23 `unit` entries were analysed. Every one is `refused`, with the same key on both sides and kept on g1-m07 (22 entries) or g1-m12 (1, composition_admission). The other side is g1-m10 (22) or g1-m02 (1).

The evidence keeps both rlibs on disk: `.cache/b/p/m/fleet/cache-server/fleet/evidence/<id>/kept.1` and `refused.1` (plus `.2` for the `.rmeta` when present). I compared them with an ar parser (scratchpad `e1/classify.py`).

Five entries, shown with `repo remote-cache evidence show`:

| Entry (crate) | kept / refused | archive members | rlib bytes | first difference (byte) | differing bytes | CGU token kept / refused |
|---|---|---|---|---|---|---|
| `semio_framework_job-cd310d33bd29fe78` | g1-m07 / g1-m10 | 99 | 5,020,816 | 691,618 | 1,152 | 1tyqktw / 1h73va5 |
| `semio_framework_job_context-5e5027375ef53682` | g1-m07 / g1-m10 | 107 | 5,883,608 | 768,834 | 1,248 | 0kr8qnc / 0o45i8m |
| `semio_framework_composition_admission-f6e800a3aed89ed5` | g1-m12 / g1-m02 | 20 | 132,424 | 12,708 | 238 | 1p16e49 / 087d47q |
| `semio_framework_dsl_record_ownership-53ad8f84895b9018` | g1-m07 / g1-m10 | 76 | 7,152,064 | 1,004,787 | 1,022 | 0xaw6h3 / 1vvij61 |
| `semio_framework_dsl-db5f4f28ba5b59f7` | g1-m07 / g1-m10 | 110 | 2,096,504 | 163,810 | 1,498 | 16oq8r1 / 099a9lt |

The `dsl_record_ownership` entry also has a standalone `.rmeta` (568,122 bytes) that differs in 28 bytes, in two places: a swapped byte pair around offset 22,466 and a 16-byte block around offset 267,816. This is an open item (see below).

What differs, for all 23 entries (identical result in every case):
- `lib.rmeta` inside the rlib: identical bytes (same metadata, same SVH and crate hash).
- All `.rcgu.o` object bodies: identical bytes. Only the archive member names differ.
- The member names differ only in one 7-character token, the second dotted segment, for example `semio_framework_job-cd310d33bd29fe78.04qouon0w374gq1s8yq7sd7ag.1tyqktw.rcgu.o` on kept versus `...1h73va5.rcgu.o` on refused. The token is the same for every member of one archive and differs between the two sides.
- `lib.rmeta-link` (the archive's internal list of object names) differs in the same token, in 119 to 749 bytes per rlib (the 23 entries range from 13 to 144 changed names). This explains the 2 x differing-byte count in the table (names in the headers plus the link list).

Classification totals: 23 of 23 entries have identical `lib.rmeta` and identical object bodies. The only changed members are `lib.rmeta-link` and the member names (13 to 144 per rlib).

Root cause (most likely): the unit is compiled with an incremental session (`-C incremental=...`). With an incremental session, rustc 1.95.0 (aarch64-apple-darwin, the fleet toolchain) puts a per-run random token into the codegen-unit object names. Reproduced locally in `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/e1/rustc-exp`: two builds of the same crate with the same flags and output name gave `...1rw0lhi.rcgu.o` and `...1qk0k68.rcgu.o`. The token varies run to run even with the same incremental directory. The bytes of the objects and `lib.rmeta` do not change. The fleet key does not carry the session, and the identity profile only records `inc=1`, so two machines building the same key with incremental on produce different rlibs and the second one is refused as `diverged`.

Why the code allows it: `script.ts` (`UC/📜️script.ts:215`) skips incremental units with reason `incremental`, but the condition is `unit.incremental!==undefined && remote===undefined && !(shared && !settings.focus.has(crate))`. A shared, non-focus crate therefore bypasses the incremental skip and is shared even when it was compiled with an incremental session. The `incremental` skip count is only 1 to 2 on the live counters, which matches a narrow path. Not verified in this audit: which Cargo invocation sets `-C incremental` for the shared-store crates. Next step: trace the flag on those units (one command, `key/🟦️.ts:46` already parses it).

Side finding (probe, not fleet): a non-incremental rlib from the same source is byte-identical when the working directory is the same, but differs in `lib.rmeta` (34 bytes) when built from a different absolute directory. So the absolute path leaks into `lib.rmeta` (the path-dependent class already tracked by `leakedPathsV1`). The 23 divergent fleet entries do not show this, so their paths matched.

Most likely root cause per divergent crate: the same as above for all five (incremental-session token in the CGU object member names, and in `lib.rmeta-link`). The `dsl_record_ownership` standalone `.rmeta` (28 bytes) is a second, smaller source, not yet traced.

## Open items
1. Trace which build path compiles shared, non-focus units with `-C incremental`. Either drop the incremental flag for shared units, or normalise the CGU token before hashing and publishing (the `-C incremental` exemption at `script.ts:215` is the place).
2. The `.rmeta` 28-byte difference (`dsl_record_ownership`): identify the 16-byte block and the swapped bytes. Possibly a session hash in the early pipelined `.rmeta` (not the `lib.rmeta` copy in the rlib).
3. Spool: confirm m02, m06, m07 and m12 drain and do not refill under load (they need a real local compile window).
4. The `fleet` reader timed out on 2 of 4 reads (5 s), so the fleet picture needs a longer read window or a retry.

Tests and checks run: none modified. Read-only commands: `repo fleet status --json` (4 times), `repo remote-cache evidence list` and `show` (5 entries), `ar x` / `cmp` / `strings` on the evidence rlibs, a local rustc probe in the scratchpad.
