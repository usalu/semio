---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Researched the complete policy/violation system in main.go: documented all 31 ViolationCode constants, the Policy/PolicyDef/PolicyContext/Violation/ViolationKindMeta structs, 4 registered policies, CheckPoliciesWithContext flow, applyAutofixes mechanism, and 7 test patterns.
## Findings

### 1. ViolationCode Constants (main.go L9666–9703)

`ViolationKind` is a `string` type alias (L9666). All constants:

**Code: Header** (prefix `code:header:`)
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationCodeHeaderMissingRegion` | `code:header:missing-region` | No |
| `ViolationCodeHeaderWrongFileId` | `code:header:wrong-file-id` | **Yes** |
| `ViolationCodeHeaderMissingContributors` | `code:header:missing-contributors` | No |
| `ViolationCodeHeaderMissingSummary` | `code:header:missing-summary` | No |
| `ViolationCodeHeaderMissingLicense` | `code:header:missing-license` | No |
| `ViolationCodeHeaderMissingLicenseRegion` | `code:header:missing-license-region` | No |
| `ViolationCodeHeaderWrongLicense` | `code:header:wrong-license` | No |
| `ViolationCodeHeaderMissingSpecsRegion` | `code:header:missing-specs-region` | No |

**Code: Section** (prefix `code:section:`)
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationCodeSectionEmpty` | `code:section:empty` | **Yes** |
| `ViolationCodeSectionOrphanDefinition` | `code:section:orphan-definition` | No |
| `ViolationCodeSectionMissingStartName` | `code:section:missing-start-name` | No |
| `ViolationCodeSectionMissingEndName` | `code:section:missing-end-name` | **Yes** |
| `ViolationCodeSectionNameMismatch` | `code:section:name-mismatch` | **Yes** |

**Code: Comment** (prefix `code:comment:`)
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationCodeCommentInline` | `code:comment:inline` | **Yes** |
| `ViolationCodeCommentBlock` | `code:comment:block` | **Yes** |
| `ViolationCodeCommentJSDoc` | `code:comment:jsdoc` | **Yes** |

**Code: Unicode**
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationCodeUnicodeEmojiVariation` | `code:unicode:emoji-variation` | **Yes** |

**Dev-Docs** (prefix `dev-docs:`)
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationDevDocsMissingFile` | `dev-docs:missing-file` | Yes |
| `ViolationDevDocsMissingFolder` | `dev-docs:missing-folder` | Yes |
| `ViolationDevDocsWrongFilePath` | `dev-docs:wrong-file-path` | Yes |
| `ViolationDevDocsWrongFolderPath` | `dev-docs:wrong-folder-path` | Yes |
| `ViolationDevDocsWrongFileName` | `dev-docs:wrong-file-name` | Yes |
| `ViolationDevDocsWrongFolderName` | `dev-docs:wrong-folder-name` | Yes |
| `ViolationDevDocsWrongFileOrder` | `dev-docs:wrong-file-order` | Yes |
| `ViolationDevDocsWrongFolderOrder` | `dev-docs:wrong-folder-order` | Yes |
| `ViolationDevDocsMissingComponent` | `dev-docs:missing-component` | Yes |
| `ViolationDevDocsWrongComponentName` | `dev-docs:wrong-component-name` | Yes |
| `ViolationDevDocsWrongComponentOrder` | `dev-docs:wrong-component-order` | Yes |

**Sketchpad** (prefix `sketchpad:`)
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationSketchpadImportThirdParty` | `sketchpad:import:third-party-outside-elements` | No |
| `ViolationSketchpadStateMultipleMachines` | `sketchpad:state:multiple-machines` | No |
| `ViolationSketchpadStateCreateActor` | `sketchpad:state:create-actor-usage` | No |
| `ViolationSketchpadStateYjsAppState` | `sketchpad:state:yjs-app-state` | No |
| `ViolationSketchpadStateForbiddenStore` | `sketchpad:state:forbidden-store` | No |
| `ViolationSketchpadHooksNonTriadic` | `sketchpad:hooks:non-triadic` | No |

**Repo**
| Constant | Value | Autofixable |
|---|---|---|
| `ViolationRepoMissingCommand` | `repo:missing-command` | No |
| `ViolationRepoMissingTicketTracking` | `repo:missing-ticket-tracking` | No |

### 2. Struct Definitions

**`ViolationPriority`** (L5861): `string` alias with `high`, `medium`, `low` values.

**`Violation`** (L7685–7693):
```go
type Violation struct {
    ID      string        `json:"id"`
    Summary string        `json:"summary"`
    Kind    ViolationKind `json:"kind"`
    Scope   string        `json:"scope"`
    Line    int           `json:"line,omitempty"`
    Column  int           `json:"column,omitempty"`
    Excerpt string        `json:"excerpt,omitempty"`
}
```
Methods: `Priority()` → delegates to `Kind.Info().Priority`, `Autofixable()` → delegates to `Kind.Info().Autofixable`.

**`ViolationKindMeta`** (L6900–6907):
```go
type ViolationKindMeta struct {
    Kind        ViolationKind     `json:"kind"`
    PolicyID    string            `json:"policyId"`
    Priority    ViolationPriority `json:"priority"`
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
    ViolationKinds []*ViolationKindMeta `json:"violationKinds"`
}
```

**`PolicyDef`** (L9974–9985): Internal struct used for registering policies:
```go
type PolicyDef struct {
    ID          string            `json:"id"`
    Name        string            `json:"name"`
    Description string            `json:"description"`
    Scopes      []string          `json:"scopes"`
    Priority    ViolationPriority `json:"priority"`
    Kinds       []ViolationKind   `json:"kinds"`
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
Key methods: `Files()`, `ReadText(file)`, `Sections(file)`, `IgnoreDirectives(file)`, `IsIgnored(file, line, kind)`, `CreateViolation(...)`, `FilterIgnored(violations)`.

**`violationKindInfoTable`** (L9704–9972): Map of `ViolationKind` → `ViolationKindMeta` with every kind's reason, solution, and autofixable flag. Fallback in `Info()` (L9961) returns a generic "Unknown violation" for unregistered kinds.

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
func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Violation, error) {
```
- If `policyIDs` is non-empty, filters policies by matching ID.
- If `policyIDs` is empty, filters policies by `matchesScope(p.Scopes, ctx.Scope)`.
- Iterates matched policies, calls `policy.Run(ctx)`, collects all violations.

### 5. Fix Flow (L18804–18843)

`repoContext.Fix(scope)`:
1. Calls `CheckPolicies(...)` to get all violations.
2. Separates into `autofixable` (where `v.Autofixable() == true`) and `remaining`.
3. Groups autofixable violations by file via `extractFileFromScope(v.Scope)`.
4. Calls `applyAutofixes(file, vs)` per file.
5. Returns `FixResult{Fixed, Remaining, Violations}`.

### 6. applyAutofixes (L18845–19058)

```go
func applyAutofixes(file string, violations []Violation) (int, error)
```

**Algorithm:**
1. Reads file content, gets `LanguagePlugin` for the file.
2. **Sorts violations by line number descending** (bottom-up to avoid line shifts).
3. Maintains a `linesToRemove` map for batch line removal.
4. Switches on `v.Kind` for each violation:

| Kind | Fix Strategy |
|---|---|
| `ViolationCodeHeaderWrongFileId` | Replace the line with `prefix + " " + expectedId` |
| `ViolationCodeSectionEmpty` | Walk backward/forward to find section start/end, mark all lines + one surrounding blank for removal |
| `ViolationCodeSectionMissingEndName` | Call `findMatchingSectionStartName()` (L19061) to walk backward through nested sections, then `FormatSectionEnd(name)` |
| `ViolationCodeSectionNameMismatch` | Same as `MissingEndName` - find matching start and replace end marker |
| `ViolationCodeCommentInline` | Walk forward from violation line, removing contiguous comment lines (respecting skip directives, TODO markers, [DEBUG], region markers). If `v.Column > 1`, strips trailing comment from code line. |
| `ViolationCodeCommentBlock` / `ViolationCodeCommentJSDoc` | Walk forward from start, handling single-line and multi-line block comments. Preserves surrounding code when comment is inline. |
| `ViolationCodeUnicodeEmojiVariation` | Strip `\uFE0E` and `\uFE0F` from the line |

5. After processing all violations, applies `linesToRemove` to filter lines.
6. Post-removal: collapses consecutive blank lines.
7. Writes the modified content back to disk.

### 7. How to Add a New Violation Kind

1. Add a `ViolationCodeXxx ViolationKind = "category:subcategory:name"` constant (~L9668).
2. Add a `ViolationKindMeta` entry in `violationKindInfoTable` (~L9704) with `Kind`, `Priority`, `Reason`, `Solution`, `Autofixable`.
3. Add the kind to the appropriate `PolicyDef.Kinds` slice (~L11505).
4. Implement detection in the policy function (e.g., `headerPolicy`, `sectionPolicy`, `commentPolicy`, or a new policy function).
5. If autofixable, add a `case ViolationCodeXxx:` in `applyAutofixes` (~L18857).
6. Add tests in `main_test.go`.

### 8. Test Patterns (main_test.go)

**Pattern 1: Fixture-based end-to-end** (`TestFixApplyAutofixes` L1591):
- Uses real fixture files: `semio/assets/repo/some/folder/file_fixable.tsx` and `file_fixable_expected.tsx`
- Sets `rootDir` to repo root, runs `CheckPoliciesWithContext`, then `applyAutofixes`, compares output to expected file.

**Pattern 2: Temp dir unit tests** (`TestFixSectionMissingEndName` L1657, `TestFixSectionNameMismatch` L1690, `TestFixSectionEmpty` L1723, `TestFixInlineComment` L1756, `TestFixBlockComment` L1789, `TestFixJSDocComment` L1822, `TestFixMultipleViolationsSameFile` L1855):
- Creates temp dir, sets `rootDir`, writes content string, creates `[]Violation` manually, calls `applyAutofixes`, asserts on result.

**Pattern 3: ScanComments-based** (`TestFixImprovedCommentLogic` L1898):
- Uses `lang.ScanComments(ctx, file, content, lines)` to generate violations from content, then applies autofixes.

**Pattern 4: Detection tests** (`TestFixHeaderWrongFileIdDetection` L1505, `TestFixHeaderWrongFileIdIdempotent` L1478):
- Creates content with/without violations, runs `CheckPoliciesWithContext`, asserts violation presence/absence.

**Pattern 5: End-to-end detect-fix-verify** (`TestFixHeaderWrongFileIdEndToEnd` L1541):
- Detect → fix → re-detect to verify no violations remain.

**Pattern 6: GraphQL integration** (`TestViolationKindsNonEmpty` L423, `TestViolationsNonEmpty` L483):
- Queries GraphQL endpoint and asserts non-empty collections.

**Pattern 7: Comment scanning per language** (`TestScanCommentsGo` L2019, `TestScanCommentsPython` L2126, `TestScanCommentsCSharp` L2225):
- Tests `ScanComments` method per language with various content scenarios.

### Key Line Numbers

**main.go:**
- `ViolationPriority` type: L5861
- `Violation` struct: L7685
- `LanguagePlugin` interface: L7718
- `BaseLanguage.ScanComments`: L8140
- `TypeScriptLanguage.ScanComments`: L8469
- `ViolationKind` type + constants: L9666–9703
- `violationKindInfoTable`: L9704–9972
- `ViolationKind.Info()`: L9961
- `PolicyDef` struct: L9974
- `var policies` slice: L11503
- `PolicyContext` struct: L11641
- `NewPolicyContext`: L11652
- `ParseIgnoreDirectives`: L11717
- `PolicyContext.CreateViolation`: L11769
- `extractFileFromScope`: L11782
- `CheckPolicies`: L11801
- `CheckPoliciesWithContext`: L11806
- `headerPolicy`: L11856
- `sectionPolicy`: L12015
- `commentPolicy`: L12277
- `codePolicy`: L12306 (aggregator for header+section+comment+emoji)
- `Policy` struct: L6879
- `ViolationKindMeta` struct: L6900
- `repoContext.Fix`: L18804
- `applyAutofixes`: L18845
- `findMatchingSectionStartName`: L19061

**main_test.go:**
- `TestViolationKindsNonEmpty`: L423
- `TestViolationsNonEmpty`: L483
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
- `TestFixMultipleViolationsSameFile`: L1855
- `TestFixImprovedCommentLogic`: L1898
- `TestFixConfigIgnored`: L1997
- `TestScanCommentsGo`: L2019
- `TestScanCommentsPython`: L2126
- `TestScanCommentsCSharp`: L2225

## Changes

No code changes - research only.

## Log

- Searched main.go for ViolationCode constants, Policy structs, CheckPoliciesWithContext, applyAutofixes, and policy functions.
- Read all relevant code sections and test patterns.
- Documented all 31 violation kinds, 4 policy definitions, the full fix flow, and 7 test patterns.

## Todos

## Plan
