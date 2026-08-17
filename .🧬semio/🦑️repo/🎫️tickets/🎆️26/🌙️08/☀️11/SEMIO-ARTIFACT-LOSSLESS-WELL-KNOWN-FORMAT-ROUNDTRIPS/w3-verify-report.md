# W3 Independent Verification Report

Verifier: W3 verify agent. Read all 4 W3 sub-agent reports (`w3-mp4avi-report.md`, `w3-mp3wav-report.md`,
`w3-epwtsv-report.md`, `w3-html-report.md`) and re-checked every claim from disk, not from the reports'
own prose. Baseline is `w1b-close-report.md` (pre-W3: `cargo test -p semio-s-plugin-stdio --lib` →
**1231 passed; 0 failed**; policy → **21513 high-priority breaches / 25 rules**).

## Headline finding

**The full-crate gate is currently RED, not just "foreign-blocked".** All four W3 reports told the
truth that they never personally observed a green `cargo test -p semio-s-plugin-stdio --lib` and
attributed this entirely to *other* agents' concurrently-dirty files (semio's image/animation/workflow
subsets — confirmed genuinely foreign, see below). That part checks out. But three of the four
sub-scopes (**epw, mp3, wav**) and one more (**html**) also have **real, own-scope compile errors**
that their own verification method (`cargo check --lib`, which does not compile `#[cfg(test)]` code)
could not have caught, and did not catch. I confirmed this two independent times, several minutes
apart, with identical results (stable, not concurrent churn):

- `cargo check -p semio-s-plugin-stdio --lib --tests` → same error set both times:
  **34 errors in `🌦️epw`**, **3 in `🎵️mp3`**, **3 in `🔊️wav`**, **2 in `🌐️html`** — zero in `🎥️mp4` / `📼️avi` / `📑️tsv`.
- `cargo test -p semio-s-plugin-stdio --lib` → **does not compile** (49 errors on one run, 42/45 on a
  repeat — the semio-subset foreign errors fluctuate with other agents' concurrent edits, but the
  epw/mp3/wav/html errors below were identical and stable across both runs).

Root cause of the epw/mp3/wav bugs (all three, identical shape): the `🧬️mutations/🦀️component.rs`
test module calls `.apply()`/`.inverse()`/`.absorb()` via method syntax but the file's `use protocol::…`
line imports `Mutation`/`OpText`/`OpBinary` and never `MutationDiff` — so the trait providing those
methods is out of scope. This is the **exact same bug class** the mp4/avi report says it self-caught
and fixed in its own files ("§3, bug 2"). It was never applied to epw/mp3/wav.

Root cause of the html bug: `📸️snapshot/🦀️component.rs`'s `FIXTURE` `include_str!` path has one `../`
too many (7 instead of 6), pointing at `🗿️artifacts/📚️examples/…` (doesn't exist) instead of
`🗿️artifacts/🌐️html/📚️examples/…` (the real location). **This exact error is already visible in the
html agent's own saved evidence file** (`w3-html-test1.txt:764`), yet the html report's Verification
section claims "all 8 laws' test bodies compiled clean" — that claim is contradicted by the report's
own attached log.

Net effect: right now, **none of the 7 new artifacts' `codec_retention_law` (or any other law) test can
actually be executed to see a `test result: N passed` line** — for epw/mp3/wav/html because of the bugs
above, for mp4/avi/tsv because the crate is one compilation unit and can't finish compiling regardless
of their own cleanliness. No agent report contains real passing-test numbers for any of the 7 artifacts'
8 laws, and neither could I produce any.

## Per-artifact table

| # | Check | mp4 | avi | mp3 | wav | epw | tsv | html |
|---|---|---|---|---|---|---|---|---|
| 1 | `codec_retention_law` test exists, targets real W0 fixture (not synthetic) | PASS | PASS | PASS | PASS | PASS | PASS | FAIL* |
| 1b | Fixture file actually present on disk at claimed path | PASS (43KB) | PASS (732B) | PASS (1725B) | PASS (16044B) | PASS (6124B) | PASS (287B) | PASS (1185B) |
| 2 | Engine genuinely moved/real (box-walk / RIFF, not stub) | PASS (~1113 LOC: 678+262+173, real `stts/ctts/stsc/stsz/stco/co64/stss` walk + real SPS/avcC h264 parse) | PASS (468 LOC, real RIFF/`avih`/`strh`/`strf` walk) | n/a | n/a | n/a | n/a | n/a |
| 3 | mp3 frame-header real bit-field extraction | n/a | n/a | PASS (real 11-bit sync scan, full MPEG1/2/2.5×I/II/III bitrate + sample-rate tables, real frame-size formula) | n/a | n/a | n/a | n/a |
| 4 | epw 35 record columns + 8 header lines present | n/a | n/a | n/a | n/a | PASS (35 fields counted directly in `EpwRecord`; all 8 header lines represented — line 1 typed `EpwLocation`, lines 2–8 verbatim-retained fields + `EpwDataPeriods`) | n/a | n/a |
| 5 | html void-element set real + honest-boundary documented | n/a | n/a | n/a | n/a | n/a | n/a | PASS (real 14-elem WHATWG set incl. `img br hr input meta`; well-formed-only boundary documented in module doc comment + 2 dedicated rejection tests) |
| 6 | Own-scope compiles (`cargo check --lib --tests`) | PASS (0 errors) | PASS (0 errors) | **FAIL (3 errors)** | **FAIL (3 errors)** | **FAIL (34 errors)** | PASS (0 errors) | **FAIL (2 errors)** |
| 6b | 8 laws actually observed passing at runtime (real `test result:` numbers) | **FAIL — could not run, crate-wide compile blocked** | **FAIL — same** | **FAIL — same, plus own bug** | **FAIL — same, plus own bug** | **FAIL — same, plus own bug** | **FAIL — could not run, crate-wide compile blocked** | **FAIL — same, plus own bug** |

\* html's `codec_retention_law` test is written correctly and does target the real fixture, but its
`include_str!` path is broken (see above), so the test **does not compile**, meaning it cannot currently
be said to "actually round-trip" anything — this is a real regression from the report's claim.

## Item 7 — full gate

- `cargo test -p semio-s-plugin-stdio --lib` — **does not compile.** Baseline (`w1b-close-report.md`)
  was **1231 passed; 0 failed**. Current state is a hard regression: 0 tests can run crate-wide.
  Raw output: `w3verify-cargo-check-tests-final.txt` (42-error run), plus the earlier 49-error
  `cargo test` run embedded in this session's tool output (not separately saved due to size; same
  epw/mp3/wav/html error set both times).
- `bun ./📜️script.ts policy` → **21523 high-priority breaches across 25 rules** (`w3verify-policy-final.txt`),
  vs. baseline **21513 / 25**. A net +10 breach delta. Given the crate doesn't compile, this comparison
  is secondary — policy is a static/lint pass independent of `cargo`, so it ran to completion, but the
  small delta is not itself the headline problem here.

## Foreign-error sanity check (confirms what it should, doesn't excuse the rest)

The `🧿️semio` image/animation/workflow subset errors (`print_diff`/`parse_diff`/`print_op`/`parse_op`/
`apply` missing from scope in `SemioImageDiff`/`SemioImageMutation`/`SemioAnimationDiff`) that all four
reports blame on concurrent sibling agents **are indeed genuinely foreign**: `git status --porcelain`
confirms those exact files are dirty/uncommitted right now, entirely outside any of the 7 artifacts'
write scopes, matching the reports' own classification. This part of every report's self-defense is
accurate. The problem is narrower and more specific: three (soon four) of the four reports' own scopes
also independently fail to compile, for reasons unrelated to the foreign churn, and `cargo check --lib`
(without `--tests`) — the only command all four agents actually ran repeatedly — structurally cannot
surface `#[cfg(test)]`-gated errors like these. This is a verification-method gap, not a fluke.

## Overall verdict

**FAIL.** Content-quality claims (2)(3)(4)(5) hold up under direct source inspection — the engines,
bit-field tables, EPW field count, and HTML void-element set are genuinely real, not stubs. But the
ticket's own bar ("re-run cargo test … confirm all 8 laws pass per artifact with real numbers") is not
met for any of the 7 artifacts, and 3–4 of the 7 have real, previously-unreported compile errors in
their own test code (epw/mp3/wav: missing `MutationDiff` import; html: broken `include_str!` path).
These four bugs are small and mechanical (each is a 1-line fix), but they are real regressions from
the W1b baseline's 1231-passing gate to a crate that does not compile at all, and must be fixed — and
the fix then re-verified with actual `cargo test` output — before this ticket can be considered closed.

## Recommended immediate fixes (not applied by this verify pass — read-only verification)

1. `🌦️epw/…/🧬️mutations/🦀️component.rs:13` — add `MutationDiff` to the `use protocol::{…}` import.
2. `🎵️mp3/…/🧬️mutations/🦀️component.rs:8` — same fix (`use protocol::{Mutation, MutationDiff};`).
3. `🔊️wav/…/🧬️mutations/🦀️component.rs:8` — same fix.
4. `🌐️html/…/📸️snapshot/🦀️component.rs:711` — drop one `../` from the `include_str!` path
   (`"../../../../../../📚️examples/🎬️demo/🖼️assets/example.html"`, 6 not 7).
5. After 1–4: re-run `cargo test -p semio-s-plugin-stdio --lib "artifacts::(mp4|avi|mp3|wav|epw|tsv|html)::"`
   and the full `cargo test -p semio-s-plugin-stdio --lib` gate, and only then re-attempt ticket close.

## Evidence files added by this verify pass

- `w3verify-cargo-check-tests-final.txt` — `cargo check -p semio-s-plugin-stdio --lib --tests`, 42 errors,
  own-scope breakdown: epw 34, mp3 3, wav 3, html 2, mp4/avi/tsv 0.
- `w3verify-cargo-test-mp4-filtered-fullcrate.txt` — an earlier `cargo test --lib "artifacts::mp4::"`
  full-crate compile attempt (49 errors that run), same own-scope breakdown.
- `w3verify-policy-final.txt` — `bun ./📜️script.ts policy` full output, 21523/25.
