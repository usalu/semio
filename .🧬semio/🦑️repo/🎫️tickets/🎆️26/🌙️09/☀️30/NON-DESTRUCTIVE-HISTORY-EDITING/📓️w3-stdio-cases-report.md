# 📓️ W3-STDIO-CASES — red stdio cases, first lane (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. The first W3-STDIO-CASES executor (agent `adb239aa…`) left no report. This file
reconstructs its run from `📓️resume-evidence.md` §2.12 and `🗑️generated/w3-stdio-cases/`; the runner is `🧪️w3-stdio-run-cases.sh`.
Split by `📓️resume-evidence.md` §5:
- S2-STDIO-A (WP-2) owns xml and md and wrote §1–§3.
- WP-3 (STDIO-B) owns dxf, dwg and bcf and appends its own section.

## 1. Reconstructed outcome of session 1 (final battery 08:14–08:16, "[w3-stdio] done" at 08:16)

| case | executed / passed | parity | owner now |
|---|---|---|---|
| `📝️mutate-md-commonmark` (02:55) | 22/22 | **11/11** | WP-2 (done) |
| `📰️mutate-xml-1-0` | 28/28 | **1/14** | WP-2 (§2) |
| `🔀️mutate-bcf-2-1` | 34/34 | 17/17 | WP-3 |
| `🔀️mutate-bcf-2-1-snapshot` | 4/4 | 2/2 | WP-3 |
| `🔀️mutate-bcf-2-1-viewpoint` | 14/16 | 6/8 | WP-3 |
| `📰️mutate-dxf-r12` | 14/14 | 0/7 | WP-3 |
| `🖊️mutate-dwg-ac1024` / `-ac1018` | 10/12 each | 4/6 each | WP-3 |

Other session-1 results:
- `⚖️law::params_are_wire` (`🔮️oracles/⚖️law/🦀️.rs:239`) landed at 03:04.
- Oracle crate lib tests: 400 passed / 2 ignored (07:43, 524 s).
- The stdio `cargo check` was clean (07:29, 6 m 05 s).

**md.** The red row `mutate-set-snapshot`, which the W2-W-text lane recorded as 10 differences, was the `comrak` writer's
`<!-- end list -->` separator: comrak inserts it after a list that a code block or another list follows, and its own reader then counts
it as one more block. The fix leaves the profile alone. The oracle now projects the AST that `set-snapshot` produced and keeps the
rendering only as raw bytes, so forward and inverse edit one parsed tree, just as the subject edits one snapshot. The feature text records
the carve-out ("The oracle's answer is the tree it edited, never its writer's text"). Neither the `ordered-json-v1` profile nor
`ignoreKeys` was widened.

## 2. xml base pipeline: root cause and fix (S2-STDIO-A, 2026-10-02)

**Symptom.** Every row except one read "stage 1 (xml-compare) exited 0 without a valid report: probe report is not an object". Stage 0
(`xml-import`) passed.

**Root cause.**
- The `xml-compare` probe (`📰️xml/…/🧱️base/🔬️probes/📜️script.ts`) prints a report that embeds the full quick-xml projections of both
  sides of the 92 873-byte `word/document.xml`.
- The orchestrator (`spawnCapturedSync`, `📚️library/🟦️.ts`) reads probe stdout through a pipe.
- The probe ended with `process.exit(await main(…))`.
- Under bun, `process.exit()` right after `process.stdout.write` to a pipe truncates the output at exactly **65 536 bytes**. A file is
  unaffected. Measured with `🗑️generated/s2-stdio-a/stdout-exit-probe.ts`: 0.2, 2 and 8 MB payloads all arrive as 65 536 bytes over a
  pipe, and complete over a file.
- So `JSON.parse` failed and the stage had no report.
- The single green row was `minified-identity-round-trip`, over the 747-byte part, whose report is small.

**Fix.** `if (import.meta.main) process.exitCode = await main(process.argv.slice(2));`.
- Measured with `stdout-exitcode-probe.ts`: 0.2, 2 and 8 MB arrive intact, and exit code 1 is preserved.
- The docx probe's awaited write callback also arrives intact (`stdout-callback-probe.ts`).
- Applied to the 19 owned probe and generator scripts. The 22 non-owned scripts and the generic capture-wrapper fix are listed in
  `📓️w3-stdio-cases-2-report.md` § Session 2.

**Verification.** On the real 92 873-byte part, the fixed probe's `xml-compare` report is 1 983 102 bytes. It arrives complete through a pipe and parses (`status ok`, `equal true`, exit 0; 2026-10-02 03:03). The full `parity exhaustive --case 📰️mutate-xml-1-0` is pending, because the os-kernel was red from a peer refactor at 03:01. The parity
run builds the case host, and the probe's own `cargo run` of `quick-xml-oracle-codec` needs cargo as well. Target: 14/14.

## 3. Fixture digest drift found on the way (xml)

The xml base `set-declaration-applied/➡️after.xml` changed on 09-28 to the recipe's `encoding="utf-8"` (`🏭️generator/🔁️codec/🦀️.rs`
`set-declaration-applied`). Its manifest digest was stale, and it is refreshed now. All 1 684 fixture files of the 54 WP-2 catalogs
re-hash clean.
