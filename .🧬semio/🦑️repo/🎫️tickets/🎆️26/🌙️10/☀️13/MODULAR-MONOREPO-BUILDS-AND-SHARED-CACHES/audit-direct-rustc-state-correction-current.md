# Direct Rustc State Correction Independent Audit

2026-10-16. Source-only rereview of the eleven-file corrective epoch and prior P1 report. Both prior scoped P1 findings close in this Source epoch. No new scoped defect established. All eleven hashes match the corrective handback.

## Prior Finding Disposition

- **Raw image/port bypass — closed.** `state/💾️volume/🟦️.ts:48–55` exposes an empty branded token with private WeakMap membership and one exact-control, live, accepting, single-claim workspace callback. Facts contain no Native image/provider port/maintenance/close/reopen. Actual image operations and record writers are private. `state/🟦️.ts:23–27` consumes that callback, issues the actual loan, validates exact plan/content/context/reservation/workspace/refinement, and drains accepted compiler work. `compiler/🦀️rustc/🟦️.ts:33` still requires the genuine prototype loan; the generic funded/unfunded options union (`compiler/🟦️.ts:14–21`) removes the unused raw port from funded consumers without bypassing actual rustc's loan receiver. Actual Script line38 and test compiler helper line11 consume the funded branch with the real state loan.
- **Concurrent reopen losing child reader — closed.** `state/💾️volume/🟦️.ts:58–63` has no public reopen; all replacement runs through the production ledger. `state/📋️model/📨️receiver/🟦️.ts:8` applies admission synchronously before running effects; the reducer (`state/📋️model/🟦️.ts:21`) refuses replacement with any pending operation and refuses reads during replacement. Every returned child is retained immediately at volume line43, before admission validation. The exact previous child is removed only after its close resolves at line61. Parent and remaining children are retained in the receiver set for final settlement at line64; no single-current overwrite can discard a newly opened reader.

## Failure and Drain Review

Explicit failed flags preserve Error and thrown undefined in the physical owner and loan/workspace callbacks (volume lines44,62,64; state line27). The finite ledger preserves failures by admission order, joins all accepted siblings and reuses its close promise (receiver lines8–9). Workspace facade births close before pending drainage; its requested actual control settles canonically before replacement/commit (volume line62). Final ledger drainage precedes reverse receiver close (volume line64). Under the actual persistent Native implementation, image.close joins borrower and maintenance work before marking closed/releasing its guard; the correction retains failed-close receivers for final settlement rather than replacing them. Cancellation is carried through the genuine Original control; this is Source review, not a physical cancellation qualification.

The actual Script keeps result receipt capture, output publication and result retirement inside the genuine state callback (Script lines37–40). The prototype fixture helper and consumed-byte fixture use the same actual owner. No public structural Native registration path was introduced by these corrections.

## Evidence Boundary

No tests, compiler/image/SDK/cache/lease operations, Source replay, fake positive authority or physical probes were run in this audit. Recorded current evidence remains state BOTH17/221 and context/profile/unit regressions BOTH15/561. Expanded strict remains explicitly blocked by the separate moving CPU profile-request/view caller transition; prior strict GREEN is not relabeled. Original failing RED and strict logs remain preserved in the ticket. This audit does not expand known release/platform/CPU/funding runtime frontiers or selectedTaskScope.

## Exact Corrective Source Hashes

Findings use suffixes of the full paths in the table. The unchanged actual rustc port was also checked: SHA256 `ae0bae5b3cd9bf74564432963aee2767eb9075b8bcbec6e15aa29f176236c7ea` (matches the prior frozen port epoch).

| Repository Path | Bytes | SHA256 | Handback Comparison |
| --- | ---: | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json` | 11116 | `4f135ab8578eba434689953280a5c1dca4d54247b49b4d3d4f6a85a90a3fdd9c` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🔣️.json` | 19656 | `96424458495b033d4dff7cacdf5bc1ce942742067c5e305dbd059574b2af5509` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | 18778 | `ea361ac9794fa277668ebe0ad83c3f331d7d6b8fb33aaf48b175a5f0c9807f4a` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/🟦️.ts` | 5662 | `4e2792421f26dfe045edcd27e315be1258e128c44cfc36b57069be602bd87315` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/📨️receiver/🟦️.ts` | 1629 | `516722bf65c309e25c6dde7b00d2068fdeb1af50983e885d6d9a8fc76687a529` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts` | 19900 | `602a09dd887623a75f3952241ea1e6e9c0808ba8aec6aba169a54fd048ea65b2` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` | 6808 | `d7a6b599ae51038c4934100334c7977fb213d045fba59e9e6ee69a3c539bb9d7` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🟦️.ts` | 4460 | `ee7f68f377d3d00bf0a7c0b11064029b8b5eb2777e62bb8b0014b08cfbfe6b4b` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts` | 12045 | `b8ac972af3ee0de348b01b059e2bb106c11c7a2bd027ac9688446eb157d3f2bb` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🏗️compiler/🟦️.ts` | 2318 | `6317dcd50023093af03e58d06131bcda2737d061a3396963235f1713dbbbaaf9` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` | 81508 | `0b32b629cf7d1b3685b261e3b52742839f99bc7cca4aa68a2c081fcfaeabc771` | matches |
