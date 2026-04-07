# Ticket

## Todos
# Previously

`scripts/log.ts` created logs with minimal frontmatter (date/slug/author/summary/model), with model defaulting from env and no support for tracking user prompts or task-scoped git stats.

# Plan

Extend log frontmatter with `prompts` and `stats`.
Make `--model` and an initial `--prompt` required on `create` and validate models via an enum.
Add CLI commands to append prompts and to maintain task-scoped affected files and git line stats.
Update `README.md` and `AGENTS.md` to describe the mechanism and CLI usage.

# Changes

Extended `scripts/log.ts` frontmatter with `prompts` and `stats` (base commit, affected files, +/− lines, updatedAt).
Added CLI commands: `prompt`, `files`, `stats`, and `models` and made `create` require `--model` + `--prompt`.
Documented the updated log workflow in `README.md` and `AGENTS.md`.

## Changes

## Log

## Summary
# Summary

Extend log.ts prompts/model/stats
