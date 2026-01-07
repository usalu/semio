# Summary

Added TOML, YAML, SQL, and GraphQL language plugins to the repo CLI language registry in `go/repo/repo.go`. TOML supports section parsing via `[section]` headers. YAML is configured for indent-based scoping. SQL supports `-- #region` regions and CREATE statement definition detection. GraphQL supports `# #region` regions and type/query/mutation definition detection.
