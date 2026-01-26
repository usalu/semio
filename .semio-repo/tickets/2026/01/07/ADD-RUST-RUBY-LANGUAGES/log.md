# Log

## 2026-01-07 21:45

Added Rust and Ruby language support to `go/repo/repo.go`:

### Rust (`RustLanguage`)
- Extensions: `.rs`
- Regions: `// #region Name` / `// #endregion Name`
- Definitions: `fn`, `struct`, `enum`, `trait`, `impl`, `type`, `const`, `static`, `mod` (with optional `pub`)
- Uses brace scoping (not indent)
- Extra orphan definitions for external module declarations (`mod name;`)

### Ruby (`RubyLanguage`)
- Extensions: `.rb`, `.rake`, `.gemspec`
- Regions: `# region Name` / `# endregion Name`
- Definitions: `def`, `class`, `module` (with namespace support like `Foo::Bar`)
- Custom `ParseDefinitions` tracking `end` keywords for proper block scoping
- Extra orphan definitions for module declarations

Both registered in `languageRegistry`.
