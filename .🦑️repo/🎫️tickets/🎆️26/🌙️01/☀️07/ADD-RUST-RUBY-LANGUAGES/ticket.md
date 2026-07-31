# Ticket

## Todos

# Plan

1. Add RustLanguage struct with:
   - Extensions: `.rs`
   - Regions using `// #region` / `// #endregion`
   - Definition detection for `fn`, `struct`, `enum`, `trait`, `impl`, `type`, `const`, `static`, `mod`
   - Extra orphan definitions for module declarations (`mod name;`)

2. Add RubyLanguage struct with:
   - Extensions: `.rb`, `.rake`, `.gemspec`
   - Regions using `# region` / `# endregion`
   - Definition detection for `def`, `class`, `module`
   - Custom ParseDefinitions using `end` keyword tracking
   - Extra orphan definitions for module declarations

3. Register both languages in `languageRegistry`

## Changes

## Log

# Log

## 2026-01-07 21:45

Added Rust and Ruby language support to `./repo/cli/cli.go`:

### Rust (`RustLanguage`)

- Extensions: `.rs`
- Regions: `// #region 🔖️Name` / `// #endregion 🔖️Name`
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

## Summary

# Summary

Added Rust and Ruby language plugins to the repo CLI language registry in `./repo/cli/cli.go`. Both languages support regions (`// #region` for Rust, `# region` for Ruby), definition parsing, and headers. Ruby includes custom `end`-based block scoping for accurate definition range tracking.
