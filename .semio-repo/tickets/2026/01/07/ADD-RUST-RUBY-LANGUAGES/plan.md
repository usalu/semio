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
