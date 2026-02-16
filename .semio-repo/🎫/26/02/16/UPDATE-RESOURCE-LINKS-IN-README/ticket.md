---
goal: R26-02/UPDATED-DOCS/UPDATED-DEV-DOCS/UPDATED-README-MD
---

# Ticket

## Summary

Updated stale README resource links to current repo paths; verified all local markdown link targets resolve.
## Changes
- Updated all stale local image/list paths from `/assets/...` to `semio/assets/...`.
- Updated stale schema links:
  - `./sqlite/schema.sql` -> `./semio/sqlite/schema.sql`
  - `./jsonschema/kit.json` -> `./semio/jsonschema/kit.json`
- Updated stale AI resource links:
  - `.github/chatmodes` / `.github/chatmodes/*.chatmode.md` -> `.github/agents/semio.agent.md`
  - removed stale `.claude/agents/*.md` references and replaced with current GitHub agent resource.
- Updated examples links from old GitHub root paths to current repo-relative paths:
  - `semio/examples`
  - `semio/examples/starters`
  - `semio/examples/hello-semio`
  - `semio/examples/geometry`
  - `semio/examples/metabolism`
- Updated typography resource link:
  - `/assets/fonts/README.md` -> `semio/assets/README.md#fonts`
- Removed stale commented studio image link pointing to a missing file.
- Re-validated local markdown links in `README.md` against filesystem targets (fragments ignored for existence check): no missing local targets remain.

## Log
- Gathered context with `./semio-repo/cli/cli tree "readme resources links"`.
- Opened ticket: `26/02/16/UPDATE-RESOURCE-LINKS-IN-README`.
- Validated missing targets with local existence checks and path scans.
- Patched `README.md`.
- Re-ran local link validation to confirm all local targets exist.

## Todos
- [x] Patch stale links in `README.md`.
- [x] Re-run local link target checks against updated `README.md`.
- [ ] Close ticket with summary and touched files.

## Plan
1. Replace stale local resource paths with current monorepo paths.
2. Replace stale agent/chatmode links with current agent resource location.
3. Replace stale examples URLs with current repo paths.
4. Re-validate all non-http README links resolve locally.
