# Ticket

## Todos

# Previously

- Ticket creation wrote an initial interaction and allowed `read` file tracking.
- Ticket finish only toggled status without aggregating files or computing ticket-level line totals.

# Plan

- Restrict interaction file tracking to `updated`, `created`, and `removed` only.
- Make ticket creation create a ticket only (no interaction) and store `author`, `created`, and `base` from git.
- On ticket close, aggregate all interaction files into ticket-level `files` and compute ticket-level `lines` from git diff against `base`.
- Update dev docs to reflect the new ticket workflow and schema.

# Changes

- `scripts/log.ts` no longer supports `read` file tracking; only `updated`, `created`, and `removed` are accepted and persisted.
- `ticket open` now creates a ticket without an interaction and records ticket-level `author`, `created`, and `base` from git.
- `ticket close` now aggregates all interaction files into ticket-level `files` and recomputes ticket-level `lines` from git diff against the ticket `base` commit.
- `README.md` and `AGENTS.md` document the updated ticket workflow and fields.

## Changes

## Log

## Summary

# Summary

Restrict ticket files and aggregate stats
