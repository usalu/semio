# Important

- Coordinator-only writes: `📓️status.md`, `🌊️workflow.json`, `📓️freeze-ledger.md`.
- Workers write only their own `📓️w*-*.md` and `scratch-*.txt`.
- Exact leaf path scopes only; no ancestor/descendant overlaps.
- Never widen scope; stop and report.
- Hot files require freeze ledger acquire/release.
- Do not create new test files; extend existing or move with examples.
- Consume inference ticket API; do not invent a second cache model.
- No git mutating commands.
