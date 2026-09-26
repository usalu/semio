# Brief — Per-Family Norm Audit (read-only)

Repo root: `/Users/ueli/Documents/semio`. Ticket folder (TICKET): `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS`.
Read `TICKET/📓️coordination.md` first for the objective.

## Rules

- READ-ONLY on the codebase. The ONLY file you may create is your audit markdown in TICKET (name given in your task).
- Do NOT run any modifying git command. Do not build with cargo (other agents are building); reading, `rg`, `ls`, `find`, `wc` are fine.
- Paths contain emoji — always quote them. The Grep tool is unreliable on emoji paths; prefer `rg` via shell with quoted paths.
- Plugin root: `✏️s/🔌️plugins/📕️norm`. Shared core: `⚖️compliance/🦀️.rs` (`CheckResult`, `CheckReport`, `NormFamily`, `NationalAnnex`). Family root: `🗿️artifacts/<emoji><family>/`. Inside each family: `🦀️.rs` (crate root/entities), `🏅️standards/🔖️1/🪆️subsets/✳️any/` containing `🧬️schema/` (artifact schema `🦀️.rs` with compliance helpers, `📸️snapshot/` persisted shape in many facets, `🔺️diff/`, `🧬️mutations/<mutation>/`, `💡️inferences/` with `evaluate()`), `✏️editor/`, `👁️viewer/` (or similar), `📚️examples/`, `🖼️assets/`, `🚪️io/`, `🧪️tests/`, `🔮️oracles/`.

## What to produce: `TICKET/📓️audit-<family>.md` (≤ 450 lines, dense, concrete, with file paths and line numbers)

1. **Inventory** — snapshot/document fields (name, type, unit), composed children, mutation list, `evaluate()` call graph (which helper functions/parts it reaches), editor/viewer capabilities (which fields editable, how report shown), examples/assets present, tests present (and whether they assert numbers).
2. **Stub / fake detection** — list every: hardcoded computed value or constant standing in for an input (e.g. `check_reliability_index(3.9, …)`), "surrogate"/"simplified"/"approx" model, helper that ignores its inputs, part module never reached by `evaluate()`, DE national annex identical to EN where the real DIN EN NA differs, clause IDs that are wrong or vague, checks that can never fail, tests only asserting `!is_empty()`, placeholder text, `todo!`, empty folders (`📌️.empty.md`). Quote file:line.
3. **Complete subject definition** — from your engineering knowledge of this norm (and its German national annex / DIN specifics), define what the artifact must model to be the complete subject of assessment: domain entities, their fields with units and valid ranges, relationships (e.g. building → zones → envelope elements → layers; structure → members → sections → materials → load cases). Cover all parts of the norm family that the plugin claims (see `AGENTS.md` of plugin and the family doc comments).
4. **Check catalogue** — table: part · clause/equation/table · what is verified · required subject inputs · limit source (incl. DE-NA value vs EN recommended value where they differ) · failure meaning.
5. **Remediation strategy** — for each check: how the report should tell the user how to comply (e.g. analytic inversion to required value: "increase insulation thickness of layer X from 80 mm to ≥ 124 mm", "choose section HEB 240 or larger", "reduce span to ≤ 5.2 m", "add mandatory property Y"). State which subject field(s) the remedy targets and how to compute the target value.
6. **Report & UX gaps** — does the current output say what complies/doesn't and how to comply? Localized (en + de)? Does the UI expose all subject fields for editing and render the report readably?
7. **Target design** — proposed snapshot schema (Rust-ish struct sketch), `evaluate()` structure, example subjects (one realistic fully compliant, one realistic non-compliant with multiple failures), numeric worked-example tests (with expected values and their derivation), and the list of files that must change (schema facets, mutations, editor, viewer, examples, tests, oracles).
8. **Risks / open questions** for the coordinator.

Finish with a one-paragraph executive summary at the TOP of the file. Your final chat reply must be only the audit file path plus a 5-line summary.
