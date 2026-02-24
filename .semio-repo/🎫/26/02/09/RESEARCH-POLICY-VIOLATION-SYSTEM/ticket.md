---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Researched the complete policy/breach system in main.go: documented all 31 BreachCode constants, the Policy/PolicyDef/PolicyContext/Breach/StatuteMeta structs, 4 registered policies, CheckPoliciesWithContext flow, applyAutofixes mechanism, and 7 test patterns.

## Findings

### 1. BreachCode Constants (main.go L9666–9703)

`Statute` is a `string` type alias (L9666). All constants:

**Code: Header** (prefix `code:header:`)
| Constant | Value | Autofixable |
|---|---|---|
| `BreachCodeHeaderMissingRegion` | `code:header:missing-region` | No |
| `BreachCodeHeaderWrongFileId` | `code:header:wrong-file-id` | **Yes** |
| `BreachCodeHeaderMissingContributors` | `code:header:missing-contributors` | No |
| `BreachCodeHeaderMissingSummary` | `code:header:missing-summary` | No |
| `BreachCodeHeaderMissingLicense` | `code:header:missing-license` | No |
| `BreachCodeHeaderMissingLicenseRegion` | `code:header:missing-license-region` | No |
| `BreachCodeHeaderWrongLicense` | `code:header:wrong-license` | No |
| `BreachCodeHeaderMissingRequirementsRegion` | `code:header:missing-requirements-region` | No |

**Code: Section** (prefix `code:section:`)
| Constant | Value | Autofixable |
|---|---|---|
| `BreachCodeSectionEmpty` | `code:section:empty` | **Yes** |
| `BreachCodeSectionOrphanDefinition` | `code:section:orphan-definition` | No |
| `BreachCodeSectionMissingStartName` | `code:section:missing-start-name` | No |
| `BreachCodeSectionMissingEndName` | `code:section:missing-end-name` | **Yes** |
| `BreachCodeSectionNameMismatch` | `code:section:name-mismatch` | **Yes** |

**Code: Comment** (prefix `code:comment:`)
| Constant | Value | Autofixable |
|---|---|---|
| `BreachCodeCommentInline` | `code:comment:inline` | **Yes** |
| `BreachCodeCommentBlock` | `code:comment:block` | **Yes** |
| `BreachCodeCommentJSDoc` | `code:comment:jsdoc` | **Yes** |

**Code: Unicode**
| Constant | Value | Autofixable |
|---|---|---|
| `BreachCodeUnicodeEmojiVariation` | `code:unicode:emoji-variation` | **Yes** |

**Dev-Docs** (prefix `dev-docs:`)
| Constant | Value | Autofixable |
|---|---|---|
| `BreachDevDocsMissingFile` | `dev-docs:missing-file` | Yes |
| `BreachDevDocsMissingFolder` | `dev-docs:missing-folder` | Yes |
| `BreachDevDocsWrongFilePath` | `dev-docs:wrong-file-path` | Yes |
| `BreachDevDocsWrongFolderPath` | `dev-docs:wrong-folder-path` | Yes |
| `BreachDevDocsWrongFileName` | `dev-docs:wrong-file-name` | Yes |
| `BreachDevDocsWrongFolderName` | `dev-docs:wrong-folder-name` | Yes |
| `BreachDevDocsWrongFileOrder` | `dev-docs:wrong-file-order` | Yes |
| `BreachDevDocsWrongFolderOrder` | `dev-docs:wrong-folder-order` | Yes |
| `BreachDevDocsMissingComponent` | `dev-docs:missing-component` | Yes |
| `BreachDevDocsWrongComponentName` | `dev-docs:wrong-component-name` | Yes |
| `BreachDevDocsWrongComponentOrder` | `dev-docs:wrong-component-order` | Yes |

**Sketchpad** (prefix `sketchpad:`)
| Constant | Value | Autofixable |
|---|---|---|
| `BreachSketchpadImportThirdParty` | `sketchpad:import:third-party-outside-elements` | No |
| `BreachSketchpadStateMultipleMachines` | `sketchpad:state:multiple-machines` | No |
| `BreachSketchpadStateCreateActor` | `sketchpad:state:create-actor-usage` | No |
| `BreachSketchpadStateYjsAppState` | `sketchpad:state:yjs-app-state` | No |
| `BreachSketchpadStateForbiddenStore` | `sketchpad:state:forbidden-store` | No |
| `BreachSketchpadHooksNonTriadic` | `sketchpad:hooks:non-triadic` | No |

**Repo**
| Constant | Value | Autofixable |
|---|---|---|
| `BreachRepoMissingCommand` | `repo:missing-command` | No |
| `BreachRepoMissingTicketTracking` | `repo:missing-ticket-tracking` | No |

### 2. Struct Definitions

**`BreachPriority`** (L5861): `string` alias with `high`, `medium`, `low` values.

**`Breach`** (L7685–7693):

```go
type Breach struct {
    ID      string        `json:"id"`
    Summary string        `json:"summary"`
    Kind    Statute `json:"kind"`
    Scope   string        `json:"scope"`
    Line    int           `json:"line,omitempty"`
    Column  int           `json:"column,omitempty"`
    Excerpt string        `json:"excerpt,omitempty"`
}
```

Methods: `Priority()` → delegates to `Kind.Info().Priority`, `Autofixable()` → delegates to `Kind.Info().Autofixable`.

**`StatuteMeta`** (L6900–6907):

```go
type StatuteMeta struct {
    Kind        Statute     `json:"kind"`
    PolicyID    string            `json:"policyId"`
    Priority    BreachPriority `json:"priority"`
    Reason      string            `json:"reason"`
    Solution    string            `json:"solution"`
    Autofixable bool              `json:"autofixable"`
}
```

**`Policy`** (L6879–6886):

```go
type Policy struct {
    ID             string               `json:"id"`
    Name           string               `json:"name"`
    Description    *string              `json:"description,omitempty"`
    Scopes         []string             `json:"scopes"`
    Statutes []*StatuteMeta `json:"statutes"`
}
```

**`PolicyDef`** (L9974–9985): Internal struct used for registering policies:

```go
type PolicyDef struct {
    ID          string            `json:"id"`
    Name        string            `json:"name"`
    Description string            `json:"description"`
    Scopes      []string          `json:"scopes"`
    Priority    BreachPriority `json:"priority"`
    Kinds       []Statute   `json:"kinds"`
    Run         PolicyFunc        `json:"-"`
}
```

**`PolicyContext`** (L11641–11650):

```go
type PolicyContext struct {
    Scope         Scope
    RootDir       string
    Bundles       []Bundle
    fileCache     map[string]string
    sectionCache  map[string][]Section
    ignoreCache   map[string]map[int][]string
    filesOverride []string
}
```

Key methods: `Files()`, `ReadText(file)`, `Sections(file)`, `IgnoreDirectives(file)`, `IsIgnored(file, line, kind)`, `CreateBreach(...)`, `FilterIgnored(breachs)`.

**`statuteInfoTable`** (L9704–9972): Map of `Statute` → `StatuteMeta` with every kind's reason, solution, and autofixable flag. Fallback in `Info()` (L9961) returns a generic "Unknown breach" for unregistered kinds.

### 3. Policy Registration (L11503–11598)

Policies are registered in the `var policies = []PolicyDef{...}` slice. Four policies exist:

1. **`code`** (L11505–11528): Scopes `**/*.{ts,tsx,py,cs,go}`. Runs `codePolicy` which aggregates:
   - `headerPolicy` (L11856) - file header validation
   - `sectionPolicy` (L12015) - region/section nesting validation
   - `commentPolicy` (L12277) - inline/block/JSDoc comment detection
   - `emojiPolicy` - emoji variation selector detection

2. **`dev-docs`** (L11530–11545): Scopes `README.md`, `AGENTS.md`. Runs `devDocsPolicy`.

3. **`sketchpad`** (L11547–11565): Scopes `js/sketchpad/**/*.{ts,tsx}`. Runs `sketchpadPolicy`.

4. **`repo`** (L11567–): Scopes `go/repo/main.go`, `js/vscode/package.json`, etc. Runs `repoPolicy`.

### 4. CheckPoliciesWithContext (L11806–11832)

```go
func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Breach, error) {
```

- If `policyIDs` is non-empty, filters policies by matching ID.
- If `policyIDs` is empty, filters policies by `matchesScope(p.Scopes, ctx.Scope)`.
- Iterates matched policies, calls `policy.Run(ctx)`, collects all breachs.

### 5. Fix Flow (L18804–18843)

`repoContext.Fix(scope)`:

1. Calls `CheckPolicies(...)` to get all breachs.
2. Separates into `autofixable` (where `v.Autofixable() == true`) and `remaining`.
3. Groups autofixable breachs by file via `extractFileFromScope(v.Scope)`.
4. Calls `applyAutofixes(file, vs)` per file.
5. Returns `FixResult{Fixed, Remaining, Breachs}`.

### 6. applyAutofixes (L18845–19058)

```go
func applyAutofixes(file string, breachs []Breach) (int, error)
```

**Algorithm:**

1. Reads file content, gets `LanguagePlugin` for the file.
2. **Sorts breachs by line number descending** (bottom-up to avoid line shifts).
3. Maintains a `linesToRemove` map for batch line removal.
4. Switches on `v.Kind` for each breach:

| Kind                                                | Fix Strategy                                                                                                                                                                                     |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `BreachCodeHeaderWrongFileId`                       | Replace the line with `prefix + " " + expectedId`                                                                                                                                                |
| `BreachCodeSectionEmpty`                            | Walk backward/forward to find section start/end, mark all lines + one surrounding blank for removal                                                                                              |
| `BreachCodeSectionMissingEndName`                   | Call `findMatchingSectionStartName()` (L19061) to walk backward through nested sections, then `FormatSectionEnd(name)`                                                                           |
| `BreachCodeSectionNameMismatch`                     | Same as `MissingEndName` - find matching start and replace end marker                                                                                                                            |
| `BreachCodeCommentInline`                           | Walk forward from breach line, removing contiguous comment lines (respecting skip directives, TODO markers, [DEBUG], region markers). If `v.Column > 1`, strips trailing comment from code line. |
| `BreachCodeCommentBlock` / `BreachCodeCommentJSDoc` | Walk forward from start, handling single-line and multi-line block comments. Preserves surrounding code when comment is inline.                                                                  |
| `BreachCodeUnicodeEmojiVariation`                   | Strip `\uFE0E` and `\uFE0F` from the line                                                                                                                                                        |

5. After processing all breachs, applies `linesToRemove` to filter lines.
6. Post-removal: collapses consecutive blank lines.
7. Writes the modified content back to disk.

### 7. How to Add a New Breach Kind

1. Add a `BreachCodeXxx Statute = "category:subcategory:name"` constant (~L9668).
2. Add a `StatuteMeta` entry in `statuteInfoTable` (~L9704) with `Kind`, `Priority`, `Reason`, `Solution`, `Autofixable`.
3. Add the kind to the appropriate `PolicyDef.Kinds` slice (~L11505).
4. Implement detection in the policy function (e.g., `headerPolicy`, `sectionPolicy`, `commentPolicy`, or a new policy function).
5. If autofixable, add a `case BreachCodeXxx:` in `applyAutofixes` (~L18857).
6. Add tests in `main_test.go`.

### 8. Test Patterns (main_test.go)

**Pattern 1: Fixture-based end-to-end** (`TestFixApplyAutofixes` L1591):

- Uses real fixture files: `semio/assets/repo/some/folder/file_fixable.tsx` and `file_fixable_expected.tsx`
- Sets `rootDir` to repo root, runs `CheckPoliciesWithContext`, then `applyAutofixes`, compares output to expected file.

**Pattern 2: Temp dir unit tests** (`TestFixSectionMissingEndName` L1657, `TestFixSectionNameMismatch` L1690, `TestFixSectionEmpty` L1723, `TestFixInlineComment` L1756, `TestFixBlockComment` L1789, `TestFixJSDocComment` L1822, `TestFixMultipleBreachsSameFile` L1855):

- Creates temp dir, sets `rootDir`, writes content string, creates `[]Breach` manually, calls `applyAutofixes`, asserts on result.

**Pattern 3: ScanComments-based** (`TestFixImprovedCommentLogic` L1898):

- Uses `lang.ScanComments(ctx, file, content, lines)` to generate breachs from content, then applies autofixes.

**Pattern 4: Detection tests** (`TestFixHeaderWrongFileIdDetection` L1505, `TestFixHeaderWrongFileIdIdempotent` L1478):

- Creates content with/without breachs, runs `CheckPoliciesWithContext`, asserts breach presence/absence.

**Pattern 5: End-to-end detect-fix-verify** (`TestFixHeaderWrongFileIdEndToEnd` L1541):

- Detect → fix → re-detect to verify no breachs remain.

**Pattern 6: GraphQL integration** (`TestStatutesNonEmpty` L423, `TestBreachsNonEmpty` L483):

- Queries GraphQL endpoint and asserts non-empty collections.

**Pattern 7: Comment scanning per language** (`TestScanCommentsGo` L2019, `TestScanCommentsPython` L2126, `TestScanCommentsCSharp` L2225):

- Tests `ScanComments` method per language with various content scenarios.

### Key Line Numbers

**main.go:**

- `BreachPriority` type: L5861
- `Breach` struct: L7685
- `LanguagePlugin` interface: L7718
- `BaseLanguage.ScanComments`: L8140
- `TypeScriptLanguage.ScanComments`: L8469
- `Statute` type + constants: L9666–9703
- `statuteInfoTable`: L9704–9972
- `Statute.Info()`: L9961
- `PolicyDef` struct: L9974
- `var policies` slice: L11503
- `PolicyContext` struct: L11641
- `NewPolicyContext`: L11652
- `ParseIgnoreDirectives`: L11717
- `PolicyContext.CreateBreach`: L11769
- `extractFileFromScope`: L11782
- `CheckPolicies`: L11801
- `CheckPoliciesWithContext`: L11806
- `headerPolicy`: L11856
- `sectionPolicy`: L12015
- `commentPolicy`: L12277
- `codePolicy`: L12306 (aggregator for header+section+comment+emoji)
- `Policy` struct: L6879
- `StatuteMeta` struct: L6900
- `repoContext.Fix`: L18804
- `applyAutofixes`: L18845
- `findMatchingSectionStartName`: L19061

**main_test.go:**

- `TestStatutesNonEmpty`: L423
- `TestBreachsNonEmpty`: L483
- `TestFixCommand`: L1250
- `TestFileHeaderId`: L1271
- `TestDeriveFileKind`: L1380
- `TestFixHeaderWrongFileId`: L1442
- `TestFixHeaderWrongFileIdIdempotent`: L1478
- `TestFixHeaderWrongFileIdDetection`: L1505
- `TestFixHeaderWrongFileIdEndToEnd`: L1541
- `TestFixApplyAutofixes`: L1591
- `TestFixSectionMissingEndName`: L1657
- `TestFixSectionNameMismatch`: L1690
- `TestFixSectionEmpty`: L1723
- `TestFixInlineComment`: L1756
- `TestFixBlockComment`: L1789
- `TestFixJSDocComment`: L1822
- `TestFixMultipleBreachsSameFile`: L1855
- `TestFixImprovedCommentLogic`: L1898
- `TestFixConfigIgnored`: L1997
- `TestScanCommentsGo`: L2019
- `TestScanCommentsPython`: L2126
- `TestScanCommentsCSharp`: L2225

## Changes

No code changes - research only.

## Log

- Searched main.go for BreachCode constants, Policy structs, CheckPoliciesWithContext, applyAutofixes, and policy functions.
- Read all relevant code sections and test patterns.
- Documented all 31 statutes, 4 policy definitions, the full fix flow, and 7 test patterns.

## Todos

## Plan
