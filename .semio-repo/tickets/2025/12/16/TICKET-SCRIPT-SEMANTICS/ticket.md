# Ticket

## Todos
# Previously

- `scripts/log.ts` exposed a log-centric workflow (`create`, `update`, `finish`) where new iterations could be appended before finishing the previous one.
- The latest-iteration finish step accepted optional file lists and computed per-file line stats from git diffs.

# Plan

- Add ticket command hierarchy and ticket close semantics.
- Enforce iteration lifecycle (no overlapping iterations).
- Require file lists for iteration start/finish and persist them on start/finish.
- Update dev docs to ticket terminology and workflow.

# Changes

- `scripts/log.ts` now exposes `ticket open`, `ticket iteration start`, `ticket iteration finish`, and `ticket close` while keeping legacy aliases.
- `updateLog` now represents iteration start, requires file lists, and refuses to start if the latest iteration is unfinished.
- `finishIteration` now requires file lists, overwrites the iteration file lists on finish, records `finished`, and computes per-file line stats.
- Tickets can be finished via `finishTicket` only when the latest iteration is finished; ticket frontmatter stores `status`, `created`, and `finished`.
- `README.md` and `AGENTS.md` document the ticket workflow and constraints.

## Changes

## Log

## Summary
# Summary

Rename log workflow to ticket+iterations
