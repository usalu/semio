# Ticket

## Todos

# Previously

- `scripts/log.ts` exposed a log-centric workflow (`create`, `update`, `finish`) where new interactions could be appended before finishing the previous one.
- The latest-interaction finish step accepted optional file lists and computed per-file line stats from git diffs.

# Plan

- Add ticket command hierarchy and ticket close semantics.
- Enforce interaction lifecycle (no overlapping interactions).
- Require file lists for interaction start/finish and persist them on start/finish.
- Update dev docs to ticket terminology and workflow.

# Changes

- `scripts/log.ts` now exposes `ticket open`, `ticket interaction start`, `ticket interaction finish`, and `ticket close` while keeping legacy aliases.
- `updateLog` now represents interaction start, requires file lists, and refuses to start if the latest interaction is unfinished.
- `finishIteration` now requires file lists, overwrites the interaction file lists on finish, records `finished`, and computes per-file line stats.
- Tickets can be finished via `finishTicket` only when the latest interaction is finished; ticket frontmatter stores `status`, `created`, and `finished`.
- `README.md` and `AGENTS.md` document the ticket workflow and constraints.

## Changes

## Log

## Summary

# Summary

Rename log workflow to ticket+interactions
