# Remedy law (wave 2, REMEDY-LAW)

Nothing compiled or run (no cargo/rustc/`repo run`); Python only. No git writes.

## Law
- Shared type: `CheckBuilder::try_build() -> Result<CheckResult, MissingRemedy>`; `build()` panics on `Err` in every profile (replaces the debug-only `debug_assert!`). A failing check passes only with a `.remedy(..)` or a named post-pass `.remedies_attached_by("fn")` (new builder method; the scanner checks that `fn` exists in the same artifact or a shared crate). Typestate was not used: 153 files and 700+ sites call `assess(..)` and a post-pass design (en1992 minimal remedies, design-case lever, en1995 `finish`) needs a deferral marker anyway. File: `⚖️compliance/✅️check-results/🦀️.rs`.
- Rust unit tests (`⚖️compliance/🧪️tests/🔬️unit/🦀️.rs`): `remedy_law_rejects_a_failing_check_without_remedy_in_every_profile` (try_build Err + build panics), `remedy_law_accepts_a_remedy_or_a_named_post_pass`.
- Repo test `⚖️compliance/🧪️tests/🩹️remedy-law/{🥒️.feature,🐍️.py}` (wired by convention like path-budget/ownership): static scanner over `CheckResult::assess(` chains and every `.build()` chain of en1990..en1999 and the shared crates (tests and din/vdi/iso artifacts excluded, `--all` adds them); 17 synthetic self-test cases; exit 1 on any finding. Current run: 492 sites, 0 findings. Ticket runner: `🧰️scripts/remedy-law/scan.py`.

## Findings fixed (all sites the scanner reported; the first naive pass had 62, the stricter chain/variable pass found more)
- shared `design-case`: unknown/wrong-kind input Fail gets `Remedy::remove` of the input row; every failing catalogue result without lever remedy gets an advisory (non-applicable) "revise inputs" remedy (makes the `evaluate_case` post-pass sound).
- en1990: bridge working life 100 a (exactly), RC vs CC (one_of RC default), DSL/IL levels (one_of recommendation).
- en1991: vortex shedding (widen element), gas explosion volume domain (clamp volume), fire validity (clamp O or q_f), hipped-roof c_pe,10 deviation (set to Table 7.5), catalogue Warn mode (revise inputs), braking pinned `Pass` (bounded by the 900 kN cap), `na-de-edition` pinned `Pass`.
- en1992: 40 member/anchor/fire/cover sites name `attach_minimal_remedies` (their remedy post-pass). Edited while `freeze.txt` named build-en1992 (it appeared between my two reads; en1992 graph-plan restarts, sorry).
- en1993: bolt spacing e1/e2/p1/p2 min/max (at_least/at_most on endDistance/edgeDistance/pitch/gauge); catalogue results name `evaluate_case`. The three plate checks (plateweld, grout, mjrd) were not touched (v17).
- en1994: b_eff vs spacing (raise spacing), tabulated column fire (raise outer size), catalogue names `evaluate_case`.
- en1995: CLT layers/symmetry/mixed-grade and layout failures (advisory remedies via new `advice`), floor creep (raise EI), `assessed` names `finish`, catalogue names `evaluate_case`.
- en1997: governing-situation summaries copy the remedies of the governing check (new parameter `governing_remedies`, 5 callers).
- en1998: bridge/tower modal participation (advisory), LFM applicability (elevation regularity or advisory), chord rotation core b_0/h_0 (at_least), catalogue names `evaluate_case`.
- en1999: fatigue m1 (one_of), thin-wall local buckling (thicker sheet).
- en1996: no finding.

## Open items
- `din4108` (outside the ten codes, `--all` only): `din4108.integrity.duplicate.{table}.{id}` duplicate-id check at the `integrity` file line 104 has no remedy; owner din4108.
- en1995 `finish`: a failing check with an empty `levers` list still gets no remedy; en1992 `attach_minimal_remedies` and the design-case notice guarantee one, `finish` does not.
- The scanner is heuristic (no type information): helper-built builders are covered via the `.build()` chain pass, a remedy attached to a variable is trusted by name; dynamic Fail without remedy (a branch the static view cannot see) is still caught by `build()` at run time.
- Nothing compiled: check `Remedy`/`Quantity` imports in the touched files (en1990 partial-factor-sets no longer needs `Quantity`), `Remedy { applicable: false, .. }` struct update (fields are public), `near_limit()` re-export in en1991 hipped-checks.

- ready for rebuild: remedy law (appended to build-en1990..en1999 `latest.md`)

## v2: runtime never crashes
- `CheckBuilder::build()` no longer panics: a failing check without remedy is returned with the `MissingRemedy::notice()` text appended to its explanation (EN "No remedy defined for this failing check", DE "Für diese nicht erfüllte Nachweisführung ist keine Abhilfe hinterlegt"), shown like any note (CheckResult has no separate diagnostics field). `try_build()` still returns `Err(MissingRemedy)`; both go through one private `assemble()`.
- Unit tests: `remedy_law_try_build_rejects_a_failing_check_without_remedy` (Err, build does not panic), `remedy_law_build_never_panics_and_attaches_the_missing_remedy_diagnostic`.
- en1995 `finish`: with empty `levers` a failing check gets an advisory remedy "Increase the cross-section or the strength class, or reduce the span or the load." / DE equivalent.
- Scanner: 492 sites, 0 findings, 17 self-test cases. Nothing compiled.
