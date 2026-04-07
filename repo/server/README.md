# Summary

Stateful Go service for persisting ticket state, semantic scopes, and collaboration warnings.

# Docs

The CLI sends unified diffs or file snapshots; the server parses them, reindexes affected files, updates claims, and emits conflict warnings and precommit blockers.
HTTP endpoints cover ticket lifecycle commands, diff ingestion, precommit checks, indexing, and read-only queries for warnings, breachs, and scopes.
Webhook receivers enrich GitHub issue events, and Discord notifications format prompt/summary headings to match ticket workflow conventions.

# 💯Requirements
