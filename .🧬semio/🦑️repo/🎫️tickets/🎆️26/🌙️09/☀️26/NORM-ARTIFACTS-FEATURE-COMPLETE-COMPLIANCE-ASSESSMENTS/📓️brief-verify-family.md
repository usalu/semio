# Brief — Wave D Adversarial Family Verification (read-only)

Ticket root: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS`
Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji><family>/🏅️standards/🔖️1/🪆️subsets/✳️any/`

You are a sceptical reviewer. Assume the implementer overclaimed. Do not edit source files. You MAY run read-only commands and the test runner. Write output ONLY to `📓️verify-<family>.md` in the ticket root (temporary logs under `🗑️generated/verify-<family>/`). No git-modifying commands. Do not use the IDE search tool; use `rg`/`find` from the repo root with quoted paths.

> **ADDENDUM 14:54 (coordinator) — empty catalogue tables are blocking.** `reference_tables()` (catalogue panel) must not return `Vec::new()`. The tables must be the same numbers `evaluate()` uses (shared const or the check reads the table), with clause id, distinct en/de titles, and units. A test must show one evaluated limit equals the matching cell. Already filled and only to be re-checked: en1990, din16798, en1992, en1999, vdi3805. Still empty at 14:54: din4108, din18599, en1991, en1993, en1994, en1995, en1996, en1997, en1998, iso16757.

> **ADDENDUM 14:42 (coordinator) — enforce CORRECTION 14:42.** Re-check every instance listed for the family in `📓️audit-perturbation-gaming.md` (each unresolved one is blocking); run `rg -n "let _ ="` over evaluate/inference code; read the perturbation test and flag as blocking: explanation text in the signature, ratio slack, exemptions beyond descriptive `id`/`name`/`title`/`labelEn`/`labelDe` (reference ids must be perturbed to dangling values), root-key/first-item-only walks, whole-subtree skips.

> **ADDENDUM 14:37 (coordinator) — perturbation gaming is blocking.** Grep the family for `fingerprint`, `1e-9 *`, `1e-12 *`, `* 1e-<n>` added to computed values, fields echoed into explanations/quantities without normative effect, and comments like "so editing changes the report". Each instance is blocking. The perturbation test must assert status/computed/limit/utilization changes, not explanation-only changes; a test that accepts explanation-only changes is blocking. For every leaf, spot-check that the check it changes actually uses it normatively.

> **ADDENDUM 13:43 (coordinator).** Always run tests with `--skip-nx-cache -- --no-fail-fast`; skipped/ignored tests are blocking. Check every item of "CORRECTION 13:27" in `📓️brief-wave-c-family.md` explicitly. For structural families (en1992–en1999): the subject must carry members + characteristic actions/load cases combined per EN 1990 (+ DE NA) inside `evaluate()`; hand-typed design effects (M_Ed/V_Ed/N_Ed scalars) as the only action input are blocking. Any editable field that no check reads is blocking (exception: purely descriptive `name`/`title` labels that are used as the entity label in the report). Never propose "narrow the scope" as a fix option. The perturbation test is scope-aware: each leaf is perturbed in a committed example where it applies; exempting leaves because they are N/A in the default example is blocking.

## Inputs

- Binding contracts: `📓️spec-core-assessment-model.md`, `📓️brief-wave-c-family.md` (definition of done).
- Baseline: `📓️audit-<family>.md`. Implementer claim: `📓️impl-<family>.md`.

## Checks (each → PASS / FAIL with file:line evidence)

1. **Subject completeness** — the snapshot is a complete hierarchical subject for the norm's scope in SI units; no leftover flat demo scalars; every input the checks read is a real field (no hidden constants standing in for inputs, e.g. hardcoded β, γ, ψ, exposure classes).
2. **Clause coverage** — list every check id + clause. Compare against what the norm part actually requires for the declared scope; name missing mandatory verifications. Flag tautologies (`limit == computed`, always-pass, status independent of inputs).
3. **Numerics** — pick ≥ 3 checks and recompute by hand from the default example; results must match within 0.5 %. Report the derivation.
4. **Applicability** — gating to `NotApplicable` with a reason where the subject makes a clause irrelevant; no silent skips.
5. **National annex** — DE vs EN recommended values actually differ where the norm differs; tests prove it.
6. **Report quality** — (paths per spec v1.2: `[index]` or `[id=<id>]`; every emitted path must parse under that grammar and resolve to an existing element + field) — every check has localized `title`/`explanation` (en + de, not identical placeholder copies); `subject` paths resolve to real snapshot paths; every `Fail` carries ≥ 1 remedy whose `target` is a real editable path and whose `required` value, when applied, flips the check to pass (verify at least 2 by reasoning or by a test).
7. **Examples** — ≥ 1 compliant and ≥ 1 non-compliant example exist, decode, and evaluate to the claimed verdicts; the default is realistic (not tuned to trivially pass everything).
7b. **Inputs UX** — the family's field-metadata table (see `📓️impl-b2-app-surface.md`) is NOT `empty_field_meta`: every editable leaf (use `[]` wildcards for lists) has en + de label and SI unit; every enum/select has localized choice labels (en + de); no raw codes shown to users.
8. **Mutations & schema** — granular mutation leaves exist for editable paths; all facets (🦀️/🟦️/🔗️/🔣️/🛰️) agree with the Rust snapshot; taxonomy regenerated.
9. **Tests** — run `bun nx run @semio-tech/norm-<family>-rs:test` from the repo root (save output to `🗑️generated/verify-<family>/test.txt`). Record executed/passed/failed counts; a run with 0 executed tests is a FAIL. Confirm the numeric worked examples, DE-vs-EN, remedy law, and third-party JSON-schema validation tests exist and actually assert values (not `!is_empty()`).
10. **Stubs** — `rg` for `todo!`, `unimplemented!`, `placeholder`, `TODO`, `stub`, `dummy`, `0.0 /*`, fixture-bound `evaluate` in the family tree.

## Deliverable `📓️verify-<family>.md`

- Verdict line: `VERDICT: PASS` or `VERDICT: FAIL (<n> blocking)`.
- Table of the 10 checks with PASS/FAIL and evidence.
- **Blocking fix list**: numbered, concrete, each with file path, what is wrong, and exactly what the fix must be (so a Grok implementer can act without re-auditing).
- Non-blocking observations.

Final chat reply: the verdict line, blocking count, and the path of the markdown file only.
