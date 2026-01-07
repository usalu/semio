# Previously

Ticket creation required a summary even though the summary is only needed when the ticket is finished.

# Plan

- Remove summary from `ticket create`.
- Require `--summary=...` only on `ticket finish`.
- Store `summary` in ticket frontmatter on finish.
- Update developer docs to match the CLI.

# Changes

- Updated `scripts/log.ts` to remove summary from `ticket create` and require `--summary=...` on `ticket finish`.
- Updated `README.md` and `AGENTS.md` ticket usage examples and schema description.
