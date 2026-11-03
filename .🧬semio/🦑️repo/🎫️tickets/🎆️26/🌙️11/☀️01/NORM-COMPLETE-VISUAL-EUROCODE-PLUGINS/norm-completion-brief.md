# ⚖️ Norm-Completion Work Package Brief

Read `fleet-brief.md`, `/home/deck/semio/AGENTS.md`, `✏️s/🔌️plugins/📕️norm/AGENTS.md` and `🔍️audits/norm-architecture.md` (recipes for adding checks/entities, project names, test targets) first.

## Goal
Make one Eurocode artifact **norm complete**: every verification in its target spec (`📐️specs/<code>-target.md`) is implemented correctly in the compliance engine, schema-first, test-driven, NA-DE aware, and validated against an independent oracle.

## Ownership (single writer)
You own, for your artifact `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji>enXXXX/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
- `🧬️schema/**` (snapshot model, `💡️inferences`, `🧬️mutations`, `🔺️diff`, schema twins `🔣️.json` `🔗️.graphql` `🛰️.proto` `🟦️.ts`), `🚪️io/**`, `🔮️oracles/**`, `🧪️tests/**`, `📚️examples/**`, `🖼️assets/**`, fixtures.
- `✏️editor/🏷️field-meta/**` (labels/units/ranges for every new field, EN + DE).
You do **not** own `✏️editor/**` windows/panels/commands, `👁️viewer/**`, or any `🎬️scene` folder (visual agents own them) nor shared crates (`⚖️compliance`, `📇️registry`). For changes there, add a line to `📌️important/requests.md`.

## Rules
1. Fix the defects listed in `🔍️audits/<code>-inventory.md` first (wrong formulas, placeholders, hard-coded utilisations, legacy aliases such as "Compatibility"/"older helpers", `-2` suffix groups). No legacy, no compatibility layer, no migrations: hand-fix fixtures/examples/assets.
2. Then implement every missing catalogue entry of the spec, grouped by part. Each check returns a `CheckResult` with clause id, utilisation, quantities with units, and remedies where meaningful. No placeholder utilisations.
3. Values marked `[V]`, `[T]`, `[verify]` or `NA-DE ?` in the spec are unverified: prefer values already encoded in the repo (`⚖️compliance/📜️annex-and-tables`, existing code) and list every remaining unverified value in your report file under "Unverified normative values".
4. Test-driven: for each catalogue group add unit tests with the spec's worked examples, at least one language-agnostic `🥒️.feature`, and extend the Python oracle. The oracle must not merely transliterate the Rust; where a third-party library exists (see `📐️specs/oracle-strategy.md` if present) use it, otherwise use an independent formulation (numeric integration, different derivation). Run Python with `-I`.
5. Build and test only through the repo MCP tools (load with ToolSearch `select:mcp__repo__test,mcp__repo__run,mcp__repo__show_projects`, pass `language: "en"`). Never cargo/nx/bazel directly. Keep the artifact compiling and tests green at the end.
6. Concise code, no comments inside definitions, docstrings start with a unique emoji, EN + DE labels, paths < 256 chars. No git writes. Others edit nearby files concurrently: never revert foreign changes; re-read before editing.
7. Logs go to `🗑️generated/<code>/` in the ticket folder; delete them when done.

## Deliverable
Report file `🔍️audits/<code>-completion.md` in the ticket folder: implemented catalogue ids, fixed defects, tests added and their results (exact MCP calls), unverified values, remaining gaps (must be empty unless blocked, then say why). Final reply: 5-line summary plus that path.
