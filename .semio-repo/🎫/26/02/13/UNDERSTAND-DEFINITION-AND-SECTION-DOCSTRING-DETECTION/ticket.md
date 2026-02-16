---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-DEFINITION-MECHANISM
---

# Ticket

## Summary

Documented all definition/section docstring detection, breach, and autofix logic in semio-repo CLI main.go.

## Changes

None (read-only investigation).

## Log

### 1. Breach Kind Constants (main.go:10831-10870)

**Section breachs:**

- `BreachCodeSectionEmpty` = "code/section/empty"
- `BreachCodeSectionOrphanDefinition` = "code/section/orphan-definition"
- `BreachCodeSectionMissingStartName` = "code/section/missing-start-name"
- `BreachCodeSectionMissingEndName` = "code/section/missing-end-name"
- `BreachCodeSectionNameMismatch` = "code/section/name-mismatch"
- `BreachCodeSectionMissingIdentification` = "code/section/missing-identification"
- `BreachCodeSectionWrongFormat` = "code/section/wrong-format"
- `BreachCodeSectionWrongFormatSummaryTooLong` = "code/section/wrong-format/summary/too-long-summary"
- `BreachCodeSectionWrongFormatSpecsSplitBlock` = "code/section/wrong-format/specs/split-block"
- `BreachCodeSectionWrongFormatDocs` = "code/section/wrong-format/docs"
- `BreachCodeSectionMissingSummary` = "code/section/missing-summary"
- `BreachCodeSectionMissingSpecs` = "code/section/missing-specs"
- `BreachCodeSectionMissingDocs` = "code/section/missing-docs"

**Definition breachs:**

- `BreachCodeDefMissingIdentification` = "code/definition/missing-identification"
- `BreachCodeDefWrongFormat` = "code/definition/wrong-format"
- `BreachCodeDefNotNativeDocstring` = "code/definition/wrong-format/not-native-docstring"
- `BreachCodeDefMissingSummary` = "code/definition/missing-summary"
- `BreachCodeDefMissingSpecs` = "code/definition/missing-specs"
- `BreachCodeDefMissingDocs` = "code/definition/missing-docs"

**Comment breachs:**

- `BreachCodeCommentInline` = "code/comment/inline"
- `BreachCodeCommentBlock` = "code/comment/block"
- `BreachCodeCommentJSDoc` = "code/comment/jsdoc"

### 2. codePolicy Function (main.go:14812-14820)

Dispatches to sub-policies:

```go
func codePolicy(ctx *PolicyContext) []Breach {
    var breachs []Breach
    breachs = append(breachs, headerPolicy(ctx)...)
    breachs = append(breachs, sectionPolicy(ctx)...)
    breachs = append(breachs, commentPolicy(ctx)...)
    breachs = append(breachs, specsPolicy(ctx)...)
    breachs = append(breachs, emojiPolicy(ctx)...)
    breachs = append(breachs, docsPolicy(ctx)...)
    return breachs
}
```

### 3. Section Policy - Definition Checking (main.go:14470-14650)

Inside `sectionPolicy`, for each real definition range:

1. Skips test/benchmark files
2. Skips non-exported definitions (via `isExportedDefinition`)
3. Native docstring detection by language:
   - **TypeScript**: Checks if prev line ends with `**/` or `*/`, then scans back for `/**` opener. Parses content for identification `[...](semiorepo://definition/...)`, specs (RFC2119 keywords via `isSpecText`), and summary.
   - **C#/Rust**: Checks if prev line starts with `///`, then scans back through `///` lines.
   - **Go/Python**: Automatically set `isNativeDocstring = true` (their comment prefix IS the native docstring format).
4. If NOT native docstring but has summary/specs/identification: emits `BreachCodeDefNotNativeDocstring`
5. Checks for `hasIdentification`, `hasSummary`, `hasSpecs` and emits corresponding breachs.

### 4. Section Policy - Section Checking (main.go:14295-14380)

For each section (recursive):

1. Checks for empty sections (no non-comment/non-blank lines, no children)
2. For non-Header, non-empty-name, non-test sections:
   - Scans comment lines after section start for identification `[...](semiorepo://section/...)` and summary text
   - Emits `BreachCodeSectionMissingIdentification` and `BreachCodeSectionMissingSummary`

### 5. SectionDocLines (main.go:13758-13815)

Marks lines that are "section doc" lines (immune from comment-ban):

1. For non-Header sections, walks from `s.StartLine + 1` forward, collecting contiguous comment-prefix lines until blank/non-comment.
2. Also marks definition-preceding comment lines (walking back from `def.Start - 2` through comment prefix lines).
3. Results are cached per filePath.

### 6. DefinitionDocLines (main.go:13845-13905)

Marks lines that are "definition doc" lines (immune from comment-ban):

1. For each definition (including extras), walks back from `def.Start - 2`:
   - Matches `commentPrefix` lines for Go/Python
   - For TypeScript: matches JSDoc lines (`/**`, `*`, `*/`, `* ...`)
   - For C#/Rust: matches `///` lines
2. Results are cached per filePath.

### 7. Autofix Logic for BreachCodeDefNotNativeDocstring (main.go:22075-22185)

**TypeScript autofix:**

1. Collects all `//` comment lines above the definition
2. Categorizes them: summary, spec (RFC2119), TODO, identification
3. Rebuilds as JSDoc:
   ```
   /**
    * <summary lines>
    *
    * <spec lines>
    *
    * <todo lines>
    *
    *  * <identification line>
    **/
   ```
4. Replaces the `//` block with the JSDoc block

**C#/Rust autofix:**

1. Walks back from definition through `//` lines
2. Replaces `// ` with `/// ` (only the first occurrence per line)
3. Stops at `///` (already native) or blank/non-comment lines

### 8. Test Functions (main_test.go:3505-3710)

**TestDefinitionNativeDocstring** (8 cases):

- TS `//` → expects breach
- TS JSDoc `/**...*/` → no breach
- Go `//` → no breach (native)
- Python `#` → no breach (native)
- C# `//` → expects breach
- C# `///` → no breach
- Rust `//` → expects breach
- Rust `///` → no breach

**TestDefinitionNativeDocstringAutofix**: Tests TS `//` → JSDoc conversion, verifies `/**`, `**/`, summary, spec, identification all present.

**TestDefinitionJSDocExemptFromCommentBan**: Verifies JSDoc on definitions doesn't trigger `BreachCodeCommentJSDoc` or `BreachCodeCommentBlock`.

### 9. Source File Formats

**TypeScript (semio/js/semio.ts)**:

- Section: `// [🔖path#Name](semiorepo://section/...)` + `// Summary text.`
- Definition: JSDoc `/** ... **/` with summary, specs (MUST/SHOULD), and `*  * [🛠️...](...)`

**Python (semio/py/semio.py)**:

- Section: `# [🔖path#Name](semiorepo://section/...)` + `# Summary text.`
- Definition: `# spec MUST ...` + `# Summary.` + `# [🛠️...](...)` above `def`/`class`

**C# (semio/net/Semio/Semio.cs)**:

- Section: `/// [🔖path#Name](semiorepo://section/...)` + `/// Specs...` + `/// Summary.`
- Definition: `/// Summary.` + `/// Specs.` + `/// [🛠️...](...)` above class/method

**Go (semio/go/semio.go)**:

- Section: `// [🔖path#Name](semiorepo://section/...)` + `// Summary text.`
- Definition: `// Name MUST ...` + `// Summary.` + `// [🛠️...](...)` above `func`

**Rust**: No `lib.rs` file found (path `semio/rs/src/lib.rs` does not exist).

### 10. Key Helper Functions

- `isExportedDefinition(name, line, langName)` — language-specific check (export, Uppercase, public, pub)
- `requiresDefinitionSpecs(line, langName)` — whether a def needs specs (functions/classes yes, enums/interfaces no)
- `isSpecText(text)` — detects RFC 2119 keywords (MUST, SHOULD, MAY, SHALL, REQUIRED, RECOMMENDED, etc.)
- `isTestOrBenchmarkFile(file)` — skips test/benchmark files for definition checks

## Todos

- [x] Read BreachCode constants
- [x] Read codePolicy dispatch
- [x] Read sectionPolicy definition checking
- [x] Read sectionPolicy section checking
- [x] Read SectionDocLines and DefinitionDocLines
- [x] Read autofix logic
- [x] Read test functions
- [x] Read source file formats (TS, Python, C#, Go, Rust)

## Plan

Investigation complete. All current logic documented above.
