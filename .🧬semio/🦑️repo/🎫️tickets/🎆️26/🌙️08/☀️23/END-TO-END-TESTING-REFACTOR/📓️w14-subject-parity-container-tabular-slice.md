# W14 — subject + parity for the container/tabular slice (9 stdio cases)

Date 2026-08-24. Raw run logs: `w14-parity/*.txt` (one file per case per phase, unfiltered stdout).
Every `[test]` line below is copied verbatim from the tool's own output, never through a pipe.

## Result table

| case | subject `[test]` (after) | parity before | parity after |
|---|---|---|---|
| `mutate-avi-1-0` | `executed=27 passed=27 failed=0 errored=0` | 27/27 | **27/27** |
| `mutate-mp4-isobmff` | `executed=21 passed=21 failed=0 errored=0` | could not run (host panic) | **21/21** |
| `mutate-wav-riff-pcm` | `executed=11 passed=11 failed=0 errored=0` | 10/11 | **11/11** |
| `mutate-zip-2-0` | `executed=15 passed=15 failed=0 errored=0` | 15/15 | **15/15** |
| `mutate-deflate-rfc1950` | `executed=11 passed=11 failed=0 errored=0` | 11/11 | **11/11** |
| `mutate-csv-rfc4180` | `executed=13 passed=13 failed=0 errored=0` | 13/13 | **13/13** |
| `mutate-tsv-iana` | `executed=15 passed=15 failed=0 errored=0` | 0/0 (host would not build) | **15/15** |
| `mutate-epw-energyplus` | `executed=27 passed=27 failed=0 errored=0` | 27/27 | **27/27** |
| `mutate-binary-raw` | `executed=20 passed=20 failed=0 errored=0` | 0/0 | 0/0 (declared `@no-oracle-raw-buffer-no-format`) |

`bun ./📜️script.ts contract` — exit 0, `0 high-priority breach(es) across 0 rule(s)`.

## Divergences and their attribution

1. **`mutate-mp4-isobmff` — encoder panic, whole host down (a, our codec).**
   `encode_mp4` asserted `chunk_sample_counts` summed to `samples.len()` and aborted the process
   (`left: 47, right: 46`). `chunk_sample_counts` is a RETAINED `stsc`/`stco` layout hint; the sample
   list is the truth, and `SetSnapshot` carries a whole caller-supplied document. Fixed at the cause:
   `normalized_chunk_sample_counts` reconciles to the one-chunk normal form instead of aborting.
   Verified empirically — the run immediately after the io fix and before the adapter fix executed
   `mutate-set-snapshot`/`inverse-set-snapshot` green on a deliberately stale grouping.
2. **`mutate-mp4-isobmff` — adapter handed over an internally inconsistent document (b).**
   The subject's `set-snapshot` popped a sample without re-grouping. Fixed with `grouping_for`.
3. **`mutate-mp4-isobmff` — `identity-round-trip` asserted the wrong byte law (b).**
   `Mp4Snapshot` has no raw-byte escape hatch and `encode_mp4` rebuilds `moov` into one deterministic
   normal form that is already this fixture's layout — pinned independently on the FULL recording by
   `🚪️io/🦀️component.rs::exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte`. Switched to
   `law::carrier_is_exact` + `law::round_trip_preserves`, documented in code and feature. The oracle
   half keeps `reparsed_not_copied` (`mp4` 0.14 is a different writer).
4. **`mutate-wav-riff-pcm` — same wrong byte law, and self-contradictory (b).**
   The case's own oracle handler already argued that RIFF/WAVE 16-bit PCM has ONE canonical layout
   and that byte-identity is the format being canonical — then demanded the opposite of the subject.
   Canonicity is a property of the format, not of one writer. Both halves now assert
   `carrier_is_exact` + `round_trip_preserves`.
5. **`mutate-tsv-iana` — subject host did not compile (b).**
   `<TsvSnapshot as store::ArtifactDsl>` — `store` is not linked into the generated host. Switched to
   the repo convention `use semio_s_plugin_stdio::ArtifactDsl;`.
6. **Peer-owned, self-resolved:** `semio-s-plugin-stdio` was red twice mid-session from another
   session's in-flight refactors (docx `NamedTripleDiff::order`, pdf `allocate_object`). Both were
   fixed by their owner within minutes; nothing was changed here on their account.

No comparison profile was touched, no `ignoreKeys` added, no fixture swapped, no assertion relaxed.

## Stale claims deleted

`mutate-zip-2-0/🦀️component.rs:10` said the subject phase was "peer-blocked right now (concurrent
os-kernel refactor)". It is not: `cargo check -p semio-framework-os-kernel --lib` is exit 0 and this
case reports `parity=15/15`. No other case in this slice carried the claim.

## Known limitation

`a_stale_chunk_grouping_is_reconciled_against_the_sample_list` (mp4 `🚪️io`) is written as a plain
`#[test]` and compiles, but the stdio plugin's `--lib test` target does not build repo-wide
(913 pre-existing errors from the async-convention debt, none of them in this test's line range), so
the unit-test binary could not be run. The reconciliation itself IS verified end-to-end, by the run
described in item 1.
