---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Researched autofix/policy system.

## Findings

### 1. Fixture Files in assets/repo/some/folder/

14 files total:

- file.tsx, file.py, file.cs — old-format headers (bare ID, no [ID](URI))
- file_fixed.tsx, file_fixed.py, file_fixed.go, file_fixed.cs — fully compliant files
- file_fixable.tsx — fixture the autofix test operates on
- file_fixable_expected.tsx — expected output after autofix (currently IDENTICAL to file_fixable.tsx)
- file_invalid.tsx, file_invalid.py, file_invalid.go, file_invalid.cs — intentionally broken files
- file_empty_region.tsx — file with a Content section (not empty but named Content)

### 2. Header Policy (headerPolicy, L14517)

Checks every file with a language that SupportsHeaders(). Produces these statutes:

1. **FileMissingHeaderRegion** — No section named "Header" found
2. **FileWrongHeaderRegionFormat** — Header exists but uses old format (bare emoji+path, no `[ID](URI)`)
3. **FileMissingIdentification** — Header has no `[ID](URI)` line at all
4. **FileWrongIdentificationId** — ID text in `[ID](URI)` doesn't match expected FileHeaderId()
5. **FileWrongIdentificationUri** — URI in `[ID](URI)` doesn't match expected FileHeaderUri()
6. **FileMissingContributors** — No contributor line matching `\d{4}\s+[\w\s]+<[\w.@-]+>`
7. **FileMissingLicense** — No AGPL marker text found
8. **FileWrongLicense** — Has AGPL but also has MIT/Apache/BSD/non-AGPL GPL
9. **FileMissingSummary** — No non-identification, non-contributor, non-license, non-spec, non-TODO comment line (exempts test files)

**Zero-breach header format (TypeScript example for path `some/folder/file.tsx`):**

```
// #region 🔖️Header
//
// [💻️some/folder/file.tsx](composerepo://file/some/folder/file.tsx)
//
// 2025 Author Name <email@example.com>
//
// GNU Affero General Public License
// https://www.gnu.org/licenses/
//
// A summary line that is not a spec, license, or contributor.
//
// #endregion 🔖️Header
```

The key elements:

- A `[ID](URI)` markdown link line
- A contributor line matching `YYYY Name <email>`
- An AGPL license marker
- A summary line (any comment text that's not identification, contributor, license, spec keyword, or TODO)

### 3. Section Policy (sectionPolicy, L14773)

Checks non-Header sections for:

1. **SectionMissingStartName** — `#region` without a name
2. **SectionMissingEndName** — `#endregion` without section name
3. **SectionNameMismatch** — start name ≠ end name
4. **SectionEmpty** — no non-blank non-comment content lines between start/end (and no children)
5. **SectionMissingIdentification** — no comment line matching `[...](composerepo://section/...)`
6. **SectionMissingSummary** — no non-identification comment line after section start

**To avoid SectionMissingIdentification**: After the `#region 🔖️Name` line, within the section, there must be a comment matching:

```
// [🔖️filePath#SectionName](composerepo://section/filePath/SECTION-NAME)
```

The regex check is: `strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](composerepo://section/")`

**To avoid SectionMissingSummary**: After the identification comment, there must be at least one more comment line (any non-empty comment text that's not the identification). Example:

```
// SectionName MUST provide the foo functionality.
```

### 4. SectionHeaderId (L20332)

```go
func SectionHeaderId(filePath string, sectionPath string) string {
    data := map[string]interface{}{"path": filePath + "#" + sectionPath}
    return GetArtifactID("section", data)
}
```

GetArtifactID for "section" returns: `🔖️` + path value
So: `SectionHeaderId("a/b.tsx", "MySection")` → `🔖️a/b.tsx#MySection`

### 5. SectionHeaderUri (L20340)

```go
func SectionHeaderUri(filePath string, sectionPath string) string {
    path := filePath + "#" + sectionPath
    data := map[string]interface{}{"path": path}
    return GetArtifactURI("section", data)
}
```

GetArtifactURI for "section" returns: `composerepo://section/` + SectionIdValueToUriPath(path)
So: `SectionHeaderUri("a/b.tsx", "MySection")` → `composerepo://section/a/b.tsx/MY-SECTION`

### 6. FileHeaderId (L20284)

```go
func FileHeaderId(path string) string {
    kind := DeriveFileKind(filepath.Base(path))
    if kind == FileKindCode {
        // reads file, checks for shebang → may override to FileKindScript
    }
    data := map[string]interface{}{"path": path, "kind": kind}
    result := GetArtifactID("file", data)
    return result
}
```

GetArtifactID for "file" returns: `fileKindEmoji(data)` + path

- code → 💻️, test → 🥼️, script → 📜️, docs → 📃️, config → ⚙️, resource → 💾️, license → ⚖️
  So: `FileHeaderId("some/folder/file.tsx")` → `💻️some/folder/file.tsx`

### 7. FileHeaderUri (L20324)

```go
func FileHeaderUri(path string) string {
    data := map[string]interface{}{"path": path}
    return GetArtifactURI("file", data)
}
```

GetArtifactURI for "file" returns: `composerepo://file/` + path
So: `FileHeaderUri("some/folder/file.tsx")` → `composerepo://file/some/folder/file.tsx`

### 8. SectionIdValueToUriPath (L33804)

```go
func SectionIdValueToUriPath(value string) string {
    hashIdx := strings.Index(value, "#")
    if hashIdx < 0 { return value }
    filePath := value[:hashIdx]
    rest := value[hashIdx+1:]
    sectionParts := strings.Split(rest, "#")
    result := filePath
    for _, p := range sectionParts {
        result += "/" + Slugify(p)
    }
    return result
}
```

Splits on first `#`, then splits remaining by `#`. Each part is Slugified (uppercased, non-alphanum replaced with `-`).
Example: `"a/b.tsx#MySection"` → `"a/b.tsx/MY-SECTION"`
Example: `"a/b.tsx#Parent#Child"` → `"a/b.tsx/PARENT/CHILD"`

### 9. DefinitionIdValueToUriPath (L33822)

```go
func DefinitionIdValueToUriPath(value string) string {
    hashIdx := strings.Index(value, "#")
    paragraphIdx := strings.Index(value, "§")
    if hashIdx < 0 && paragraphIdx < 0 { return value }
    if hashIdx < 0 && paragraphIdx >= 0 {
        filePath := value[:paragraphIdx]
        defName := value[paragraphIdx+len("§"):]
        return filePath + "/" + Slugify(defName)
    }
    filePath := value[:hashIdx]
    rest := value[hashIdx+1:]
    parts := strings.Split(rest, "§")
    result := filePath
    for _, p := range parts {
        subParts := strings.Split(p, "#")
        for _, sp := range subParts {
            result += "/" + Slugify(sp)
        }
    }
    return result
}
```

Handles `filePath#section§defName` format. Splits on `#` first, then `§`, then slugifies each part.
Example: `"a/b.tsx#Types§MyFunc"` → `"a/b.tsx/TYPES/MY-FUNC"`

### 10. PathToUriPath (L12694)

```go
func PathToUriPath(path string) string {
    return strings.ReplaceAll(path, " ", "-")
}
```

Simply replaces spaces with hyphens. Nothing else.

### 11. Slugify (L12710)

```go
func Slugify(text string) string {
    // Insert hyphens at camelCase boundaries
    // Then uppercase everything, replace non-alphanumeric with -, trim
}
```

Example: `"SectionOne"` → `"SECTION-ONE"`, `"MySection"` → `"MY-SECTION"`

## Changes

No code changes — research only.

## Log

- Read all 14 fixture files in assets/repo/some/folder/
- Read headerPolicy (L14517-L14770)
- Read sectionPolicy (L14773-L14950)
- Read FileHeaderId (L20284), FileHeaderUri (L20324)
- Read SectionHeaderId (L20332), SectionHeaderUri (L20340)
- Read SectionIdValueToUriPath (L33804), DefinitionIdValueToUriPath (L33822)
- Read PathToUriPath (L12694), Slugify (L12710)
- Read GetArtifactID (L33892), GetArtifactURI (L34042)
- Read TestFixApplyAutofixes (main_test.go L1592)
- Confirmed file_fixable.tsx and file_fixable_expected.tsx are byte-identical

## Todos

- [x] Read all fixture files
- [x] Read headerPolicy
- [x] Read sectionPolicy
- [x] Read ID/URI functions
- [x] Document findings

## Plan

Research-only ticket. No code changes needed.
