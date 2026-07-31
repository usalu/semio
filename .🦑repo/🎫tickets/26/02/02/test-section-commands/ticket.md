# Ticket

## Todos

- [x] Analyze existing tests for `rename`, `extract`, `integrate` section commands in `repo/go`
- [x] Extend `main_commands_test.go` to test these commands for every supported language
- [x] Refactor code if necessary to support testing or fix bugs
- [x] Verify tests pass

## Plan

1. Identify where section commands are implemented in `repo/go`.
2. Identify supported languages.
3. Add test cases for `rename`, `extract`, `integrate` for each language in `main_commands_test.go`.
4. Run tests and fix issues.

## Log

- Added `TestSectionCommands` in `repo/go/main_commands_test.go` covering 12 languages (TS, Go, Python, C#, Rust, Ruby, Shell, TOML, YAML, SQL, GraphQL, Markdown).
- Fixed `extract` command GraphQL query in `repo/go/main.go` (requested non-existent `success` field).
- Fixed `integrate` command GraphQL query in `repo/go/main.go` (used non-existent `IntegrateInput`).
- Refactored `YamlLanguage` in `repo/go/main.go` to support sections (added regex).
- Refactored `TomlLanguage` in `repo/go/main.go` to use standard `# region` comments for compatibility with generic tests.
- Fixed C# test case validation to use proper `#region` syntax.
- All tests passed.
