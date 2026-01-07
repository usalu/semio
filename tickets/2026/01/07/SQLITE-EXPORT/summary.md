# Summary

Added `export` CLI command that exports all repo data to a SQLite database file. Uses pure Go SQLite driver (modernc.org/sqlite). Exports repo metadata, bundles, folders, files, sections, contributors, tickets, policies, violation kinds, and violations. Schema includes views for metrics computation.
