# Ticket

## Todos

- [x] Analyze `ListTickets` logic to see if it recurses.
- [x] Refactor `CreateTicket` to respect `parent` argument for directory structure.
- [x] Refactor `LatestTicket`, `ReadTicket`, `ListTickets` to support flexible depths.
- [x] Update path generation logic.
- [x] Test nested ticket creation.

## Changes

- Modified `main.go` to implement recursive ticket listing in `ListTickets` and `StreamTickets`.
- Updated `CreateTicket` to resolve parent ticket by slug and nest child tickets inside parent folder, ensuring `YYYY/MM/DD` path consistency.
- Updated CLI command parsers (`close`, `reopen`, `change`) to handle paths with nested slugs (parts > 4).
- Added `FindTicketBySlug` helper.
- Fixed `GetProjects` compilation error by aliasing it to `LoadBundles`.

## Summary

Implemented support for nested tickets.
- Refactored `ListTickets` and `StreamTickets` to use recursive `filepath.WalkDir` to find tickets in subdirectories.
- Updated `CreateTicket` to resolve parent ticket and construct nested slug/path `PARENT/CHILD`.
- Updated CLI command parsers (`close`, `reopen`, `change`) to handle paths with nested slugs by joining all parts after date components.
- Verified nested creation, listing, closing, and reopening with test tickets.
- Fixed a compilation error regarding `GetProjects` by using `LoadBundles`.
- Fixed CLI parsing for `ticket close` where arguments with nested slugs were truncated.
