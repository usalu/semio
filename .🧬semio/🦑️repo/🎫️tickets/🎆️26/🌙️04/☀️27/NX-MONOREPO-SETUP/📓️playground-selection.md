# Authored Playground Selection

Public development selection now reads Cargo metadata through the existing source catalog discovery. It no longer schedules registry generation inside the root selection helper or rejects an empty generated catalog before Nx can create a task graph. The generated catalog remains a runtime input for consumers of asset and example projections.

The language-neutral fixture starts with only an authored manifest. Bun’s TOML projection matches the independent @iarna/toml parser for its identity, alias and ports. A same-length, same-mtime source edit changes the selected port immediately. Invalid generated JSON cannot affect selection; duplicate aliases/variants, invalid ports and external source paths are rejected. The repository contract suite passed with the actual root dev selection and the new fixture (playground-selection-green.log). Full development startup still awaits the preparation/server integration.

Nx’s workspace watcher batches changes and provides project/dependency selection. The intended preparation observer will use this mechanism, with a repository script callback for platform-neutral argument handling. See [Nx workspace watching](https://nx.dev/docs/kb/workspace-watching); installed Nx 21 behavior was separately qualified in the daemon report.
