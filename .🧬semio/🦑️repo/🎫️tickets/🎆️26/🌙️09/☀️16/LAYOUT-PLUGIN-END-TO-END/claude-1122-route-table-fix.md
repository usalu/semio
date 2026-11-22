# Route table check (claude-1122)

Validator: `readProcessTestTableV1` in `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts` raises `process.testing.route-table-invalid` when `schema.definitions.Table` (sibling `🧬️schema/🔣️.json`) rejects a table. It is fed the process, caching and build tables by `🔬️test-file`, `🗂️suites` and `⚡️caching/🗂️suites`.

Script: `claude-1122-route-table-check.ts` (run via `repo tools exec bun -- <script> [tables...]`).

Result at 10:30 on 2026-11-22:
- process, caching and build tables: schema valid, no duplicate ids.
- Coverage: caching has 1 unrouted file (`🩹️patches/🧪️tests/🔭️loader/🟦️.ts`), build has 1 (`🐢️speed/🧪️tests/🟦️.ts`). Both are coverage gaps, not schema failures; they belong to peers adding suites.
- `🧾️ci-report/🗂️suites/🔣️.json` is a different document (CI policy, own schema); validating it as a route table fails by design, which is a false lead.
- `repo show project repo` now infers the `repo` owner and prints the project; the error no longer reproduces.

Root cause: not reproducible. The build route table was edited at 09:40 today; the failure was a transient invalid intermediate state of a peer edit, observed repeatedly by the resident service until the edit settled. No source change made.
