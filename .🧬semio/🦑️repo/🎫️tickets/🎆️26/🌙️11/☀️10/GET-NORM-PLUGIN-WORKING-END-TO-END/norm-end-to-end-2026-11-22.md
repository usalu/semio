# Norm End to End in Play, 2026-11-22

## Result

All 15 Standards apps pass the strict Play workflow on the current tree. Each run used port 6036 and verified signed delivery first. Each app then booted to shell readiness, acknowledged its curated archive and drew the model. The run switched the example picker to a different example and back, and both transfers acknowledged. No faults or failed requests occurred. Probe flags: `PLAY_TEST_PICKER_ROUNDTRIP`, `PLAY_TEST_OBSERVE_ARCHIVES`, `PLAY_TEST_REQUIRE_DRAW`, `VERIFY_SIGNED_MODULES`. Driver: `probe-norm-base-companions.py`; log: `🗑️generated/routes-final-2026-11-22.log`.

| Route | Component | Initial | Alternate | ms |
|---|---|---|---|---|
| din4108 | norm | Compliant ETICS dwelling | Failing thin insulation | 23871 |
| din16798 | norm | Compliant office | Non-compliant office | 23623 |
| iso16757 | norm | Conforming catalogue | Broken catalogue | 25595 |
| vdi3805 | norm | Demo | Nonconforming Valve Dataset | 25547 |
| din18599 | norm-energy | Demo | Compliant Detached House | 46688 |
| en1990 | norm-eurocode | curated | Rail Bridge Compliant | 24886 |
| en1991 | norm-eurocode | DE Office Compliant | Multi-Fail Noncompliant | 25106 |
| en1997 | norm-eurocode | Compliant geotechnical package | Compliant project | 24945 |
| en1998 | norm-eurocode | Seismic RC frame compliant | Seismic RC frame failing | 27467 |
| en1992 | norm-eurocode-structural | curated | Failing Under-Reinforced | 117379 |
| en1993 | norm-eurocode-structural | HEB 240 Compliant Frame | Overloaded Steel Frame | 27021 |
| en1994 | norm-eurocode-structural | Composite Floor Beam | Failing Composite Floor Beam | 28226 |
| en1995 | norm-eurocode-materials | Compliant Glulam Floor Beam | Compliant Glulam Footbridge | 29754 |
| en1996 | norm-eurocode-materials | Load-bearing Clay Wall | Multi-fail Masonry Wall | 30081 |
| en1999 | norm-eurocode-materials | Aluminium roof purlin | Non-compliant aluminium beam | 30604 |

The earlier open issues from `play-resume-2026-11-20.md` no longer reproduce. The DIN V 18599 alternate transfer works, EN 1992 redraws the restored revision (drawn revision 3), and DIN 4108 completes its round trip.

## Signed builds

- `norm-eurocode-structural` was rebuilt (run `wxo47ig`, digest `f02381e5…`).
- `norm-eurocode` was rebuilt (run `wAbcnCm`, digest `2ffa4e35…`).

Both drivers returned 75 from the shared-registry final recheck after publication. Play served the exact signed files, and the delivery verifier passed.

## Fixes

- **EN 1998 examples.** All five authored documents lacked the schema-required `cases` table, so the first `setActiveExample` decode failed. See `en1998-example-decode-2026-11-22.md` and `check-example-required-fields.py`.
- **Boot refusal.** A concurrent natural-source rollout made `purpose` required on the `instance-open` event in WIT, Rust and the guest lift. The TypeScript producers still omitted it, so every freshly built guest refused to boot with `original-turn.native-return-refused: authority`. See `eurocode-boot-authority-2026-11-22.md`.
- **Debug print.** Removed the leftover `[DEBUG] FEM WIT poll semantic failure` print from `poll_wit`.
- **Compile breaks from concurrent edits.**
  - `document_archive` now has the `M: 'static` bound required by `ChildComposition::archive_member_entries`.
  - `natural_source_can_close` checks `reserved_birth.is_none()` after `reserved_birth` became a `ManuallyDrop<Option<…>>`.
- **Native-build admission budget printed NaN.** The admission schema replaced `reserveBytes` with `maximumReserveBytes` and `reserveBasisPoints`, but the broker and memory share still read `reserveBytes`. `admissionReserveBytesV1` now computes `min(maximumReserveBytes, floor(effective × reserveBasisPoints / 10000))`, and the share and broker budget use it. Checked by `check-admission-reserve.ts`: 7,516,192,768 B yields a 1,879,048,192 B reserve, and the 64 GiB case is capped at 6 GiB.
- **Pinned browsers wiped by disk-pressure reclaim.**
  - Pressure reclaim evicted `tools/ms-playwright`, and nothing reinstalls browsers at launch.
  - The ledger schema now has a per-area `collect.pressure` (`reclaimable`|`pinned`). Pressure reclaim skips pinned areas, and `playwright` is pinned. See `playwright-cache-pressure-2026-11-22.md`.
  - Chromium 1248 was reinstalled through the registered `playwright` tool.
  - The native cache law no longer flags `".cache", "<word>"` outside `join`/`resolve` calls.
  - The cache-gc temp fallback now uses the declared `.cache/scratch` instead of `.cache/none`.
  - `repo:test-storage-ledger-oracle-processes` passes 7/7.

## Changed files (this session)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/*/*/🗣️.dsl.semio`: five documents.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🪢️members/📥️staging/🗃️document-load/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎟️natural-publication/🦀️.rs`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- `instance-open` producers and their tests: listed in `eurocode-boot-authority-2026-11-22.md`.
- Ledger schema, policy, ledger module, ledger tests and cache-gc runner: listed in `playwright-cache-pressure-2026-11-22.md`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧪️tests/⚖️law/🌐️native/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`
- Ticket inputs: `check-admission-reserve.ts`, `check-example-required-fields.py`, `probe-boot-refusal.mjs`.

## Notes

- EN 1992 is slower than the other routes. Shell readiness takes 56 s, the alternate example about 39 s and the restore about 20 s, against roughly 15 s for shell readiness elsewhere. The structural component is the heaviest of the five, about 180 MB.
- The other four components still carry their 2026-11-21 builds and boot without the new `purpose` field. Their next rebuild picks up all the fixes above.
