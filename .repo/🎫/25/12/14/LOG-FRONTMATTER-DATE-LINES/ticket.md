# Ticket

## Todos
# Previously

`scripts/log.ts` stored task tracking under a nested `stats` object and used a single `date` timestamp, which made it harder to evolve the schema and update git-derived line stats consistently across multiple prompts.

# Plan

Flatten task tracking fields in frontmatter (remove `stats`), rename `base` to `commit`, and nest `lines` and `date` as objects.
Update the CLI commands to write/read the new structure and keep `date.updated` current.
Migrate all existing logs in `log/` to the latest frontmatter structure.
Update `README.md` and `AGENTS.md` to document the new format and commands.

# Changes

Replaced `stats` frontmatter with `commit`, `affectedFiles`, and `lines.{added,removed}`, and changed `date` to `{created,updated}` in `scripts/log.ts`.
Extended `tsx scripts/log.ts migrate` to rewrite frontmatter for all existing logs and skip YAML undefined values by normalizing defaults.
Updated `README.md` and `AGENTS.md` log system documentation to the new frontmatter format and stats workflow.

## Changes

## Log

## Summary
# Summary

Reshape log frontmatter and migrate logs
