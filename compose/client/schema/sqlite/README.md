# Summary

SQLite schema definitions for compose kit persistence.

# Docs

`🛢️schema.sql` is the canonical normalized SQLite layout for `.compose/kit.db`.

It mirrors the current Rust `KitFullDto` graph and stores ordered collections explicitly, so roundtrips do not rely on embedded JSON snapshots or unstable row ordering.

# 💯️Requirements

- Persist the full kit graph through relational tables.
- Preserve vector ordering with explicit `ordinal` columns.
- Keep parent ownership explicit through foreign keys.
