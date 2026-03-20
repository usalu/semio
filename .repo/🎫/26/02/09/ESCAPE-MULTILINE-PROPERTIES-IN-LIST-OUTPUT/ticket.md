---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Fixed multiline properties in CLI list output by escaping newlines (\n, \r\n, \r) to spaces in collectEntityProps's appendNonEmpty helper. This ensures all property values (summaries, descriptions, prompts, commit messages) are single-line strings when rendered between backtick delimiters in markdown output and in human-readable output. Added test TestCollectEntityProps_MultilineEscaped covering tickets, goals, policies, and commits with multiline content.
## Changes

## Log

## Todos

## Plan
