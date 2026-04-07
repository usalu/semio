# URI Scheme Migration Report

## Summary

This report documents all hardcoded URI strings in `repo/cli/main.go` that use the OLD scheme and need updating to the NEW scheme. It is organized by functional area.

**Key finding:** Most `GetURI()` methods, `buildFileUriFromPath()`, `buildSectionUriFromPath()`, `buildDefinitionUriFromIdValue()`, and the `IdToUri()` function already use the NEW scheme. The hardcoded OLD-scheme URIs are concentrated in **inline constructions** (tree nodes, file walkers, ticket rendering, MCP resource definitions, and MCP resource handlers).

---

## Category 1: File URI Construction in Walkers (Lines 2803, 3040, 3045)

These construct `semiorepo://file/...` URIs inline instead of calling `buildFileUriFromPath()` or `GetURI()`.

| Line | OLD String                                       | Should Be                                                                                                                  |
| ---- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| 2803 | `"semiorepo://file/" + PathToUriPath(relPath)`   | Call `buildFileUriFromPath(relPath)` (which produces hierarchical `semiorepo://p/{pkc}/X/b/{bkc}/Y/fd/{fkc}/Z/f/filename`) |
| 3040 | `"semiorepo://file/" + PathToUriPath(relParent)` | Call `buildFileUriFromPath(relParent)`                                                                                     |
| 3045 | `"semiorepo://file/" + PathToUriPath(relLoc)`    | Call `buildFileUriFromPath(relLoc)`                                                                                        |

---

## Category 2: Ticket URIs in Tree Building (Lines 4416, 4418, 4526-4527)

These construct old-style `semiorepo://ticket/...` URIs instead of calling `Ticket.GetURI()`.

| Line | OLD String                                                                                                         | Should Be                                                                                                              |
| ---- | ------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| 4416 | `fmt.Sprintf("semiorepo://ticket/%s", PathToUriPath(slug))`                                                        | `fmt.Sprintf("semiorepo://y/%s/tk/%s", ..., slug)` or ideally construct a Ticket and call `.GetURI()`                  |
| 4418 | `fmt.Sprintf("semiorepo://ticket/%d/%02d/%02d/%s", tTime.Year(), tTime.Month(), tTime.Day(), PathToUriPath(slug))` | `fmt.Sprintf("semiorepo://y/%02d/m/%02d/d/%02d/tk/%s", year, month, day, slug)` — matches `Ticket.GetURI()` new scheme |
| 4526 | `strings.HasPrefix(n.URI, "semiorepo://ticket/")`                                                                  | `strings.HasPrefix(n.URI, "semiorepo://y/")` and then parse y/m/d/tk structure; or use `UriToId()`                     |
| 4527 | `strings.TrimPrefix(n.URI, "semiorepo://ticket/")`                                                                 | Parse using new URI format `semiorepo://y/YY/m/MM/d/DD/tk/SLUG`                                                        |

---

## Category 3: Tree Node Category URIs (Lines 4774, 4823, 4899, 4992, 5007, 5014, 5023, 5042, 5134, 5159, 5206)

These are category nodes in the tree builder that use old-style collection URIs.

| Line | OLD String                                                    | NEW String                                                                                                                                                                           |
| ---- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 4774 | `URI: "semiorepo://projects"`                                 | `URI: "semiorepo://p"`                                                                                                                                                               |
| 4823 | `URI: "semiorepo://folders"`                                  | `URI: "semiorepo://fds"`                                                                                                                                                             |
| 4899 | `URI: "semiorepo://goals"`                                    | `URI: "semiorepo://gs"`                                                                                                                                                              |
| 4992 | `URI: "semiorepo://drafts"`                                   | `URI: "semiorepo://drs"`                                                                                                                                                             |
| 5007 | `URI: "semiorepo://policies"`                                 | `URI: "semiorepo://pls"`                                                                                                                                                             |
| 5014 | `URI: "semiorepo://policy/" + PathToUriPath(p.ID)`            | `URI: "semiorepo://pls/pl/" + PathToUriPath(p.ID)` (or call `p.GetURI()` if Policy has one — currently `Policy.GetURI()` at line 8732 already returns `"semiorepo://pls/pl/" + ...`) |
| 5023 | `URI: "semiorepo://contributors"`                             | `URI: "semiorepo://cs"`                                                                                                                                                              |
| 5042 | `URI: "semiorepo://commits"`                                  | `URI: "semiorepo://cms"`                                                                                                                                                             |
| 5134 | `URI: "semiorepo://statute/" + StatuteIdToUriPath(string(k))` | `URI: "semiorepo://sts/" + StatuteIdToUriPath(string(k))`                                                                                                                            |
| 5159 | `URI: "semiorepo://statute/" + StatuteIdToUriPath(prefix)`    | `URI: "semiorepo://sts/" + StatuteIdToUriPath(prefix)`                                                                                                                               |
| 5206 | `URI: "semiorepo://territory/" + PathToUriPath(g.Name)`       | Keep or rename to match Territory.GetURI() (line 13479 also uses `"semiorepo://territory/"`) — no new scheme specified for territories                                               |

---

## Category 4: `parseSemioIdentificationLink` Prefix Checks (Lines 16691, 16960, 16997, 17055, 17070, 17090, 17107, 17156)

These pass OLD-style URI prefixes to `parseSemioIdentificationLink()`. The function checks `strings.HasPrefix(uriValue, uriPrefix)`. The actual URIs embedded in source files already use the new hierarchical scheme, so the prefix check must match.

| Line  | OLD Prefix Passed           | Should Be                                                                                                                                                                                                                                                              |
| ----- | --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 16691 | `"semiorepo://section/"`    | These are used to **detect** identification links in comments. The actual URIs in comments are already hierarchical (e.g., `semiorepo://...file.../s/SectionName`). The prefix should be just `"semiorepo://"` or the function should detect `/s/` in the URI instead. |
| 16960 | `"semiorepo://definition/"` | Same — should be `"semiorepo://"` or detect `/d/` in the URI.                                                                                                                                                                                                          |
| 16997 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |
| 17055 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |
| 17070 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |
| 17090 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |
| 17107 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |
| 17156 | `"semiorepo://definition/"` | Same                                                                                                                                                                                                                                                                   |

**NOTE**: `SectionHeaderUri()` (line 22377) calls `GetArtifactURI("section", data)` which returns the NEW hierarchical form (e.g., `semiorepo://p/.../b/.../f/.../s/...`). Similarly `DefinitionHeaderUri()` (line 22400) returns the NEW form. So the prefix `"semiorepo://section/"` will **never match** these new URIs. This is a **functional breakage** — the policy checker cannot find identification links.

---

## Category 5: `FileURI` / `FolderURI` Helpers on CodebaseContext (Lines 18040, 18047)

| Line  | OLD String                                                          | Should Be                                                                                                 |
| ----- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 18040 | `return "semiorepo://file/" + PathToUriPath(NormalizePath(path))`   | `return buildFileUriFromPath(NormalizePath(path))`                                                        |
| 18047 | `return "semiorepo://folder/" + PathToUriPath(NormalizePath(path))` | `return buildFileUriFromPath(NormalizePath(path))` (Folder.GetURI() already calls `buildFileUriFromPath`) |

---

## Category 6: Ticket File Resolution URI Parsing (Lines 19502-19516)

| Line  | OLD String                                             | Should Be                                                                             |
| ----- | ------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| 19502 | `strings.HasPrefix(normalized, "semiorepo://file/")`   | Also handle new hierarchical file URIs containing `/f/` segment                       |
| 19503 | `strings.TrimPrefix(normalized, "semiorepo://file/")`  | Extract path from hierarchical URI using `extractFileAndSectionsFromUri()` or similar |
| 19506 | `strings.HasPrefix(normalized, "semiorepo://files/")`  | `strings.HasPrefix(normalized, "semiorepo://fis/")`                                   |
| 19507 | `strings.TrimPrefix(normalized, "semiorepo://files/")` | `strings.TrimPrefix(normalized, "semiorepo://fis/")`                                  |
| 19515 | `strings.HasPrefix(uri, "semiorepo://file/")`          | Also handle new hierarchical form                                                     |
| 19516 | `strings.TrimPrefix(uri, "semiorepo://file/")`         | Extract path from hierarchical URI                                                    |

---

## Category 7: StreamFolders/StreamFiles URI Construction (Lines 20967, 21040, 21127)

| Line  | OLD String                                                                     | Should Be                                                                                        |
| ----- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| 20967 | `URI: "semiorepo://folder/" + PathToUriPath(buildFolderID(relPath, bundleID))` | Call `buildFileUriFromPath(relPath)` (which builds hierarchical URI) — same as `Folder.GetURI()` |
| 21040 | `URI: "semiorepo://file/" + PathToUriPath(NormalizePath(relPath))`             | Call `buildFileUriFromPath(relPath)` (single file case in StreamFiles)                           |
| 21127 | `URI: "semiorepo://file/" + PathToUriPath(NormalizePath(relPath))`             | Call `buildFileUriFromPath(relPath)` (walk case in StreamFiles)                                  |

---

## Category 8: Ticket URI in Rendering (Line 21771)

| Line  | OLD String                                                                                                               | Should Be                                                                                                                                            |
| ----- | ------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| 21771 | `fmt.Sprintf("semiorepo://ticket/%02d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, PathToUriPath(ticket.Slug))` | `fmt.Sprintf("semiorepo://y/%02d/m/%02d/d/%02d/tk/%s", ticket.Year, ticket.Month, ticket.Day, PathToUriPath(ticket.Slug))` or call `ticket.GetURI()` |

---

## Category 9: MCP Resource Definitions (Lines 31191-31279)

| Line  | OLD URI                                            | NEW URI                                                                                                     |
| ----- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| 31191 | `"semiorepo://root"`                               | `"semiorepo://"`                                                                                            |
| 31195 | `"semiorepo://bundles"`                            | `"semiorepo://bs"` (or remove — bundles are accessed via `semiorepo://p/{pkc}/X/b/{bkc}/Y`)                 |
| 31199 | `"semiorepo://bundle/{id}"`                        | `"semiorepo://p/{pkc}/{project}/b/{bkc}/{bundle}"`                                                          |
| 31203 | `"semiorepo://folders"`                            | `"semiorepo://fds"`                                                                                         |
| 31207 | `"semiorepo://folder/{path}"`                      | `"semiorepo://fd/{fkc}/{path}"` or hierarchical form                                                        |
| 31211 | `"semiorepo://files"`                              | `"semiorepo://fis"`                                                                                         |
| 31215 | `"semiorepo://file/{path}"`                        | Hierarchical: `"semiorepo://p/{pkc}/{project}/b/{bkc}/{bundle}/f/{file}"` or loose `"semiorepo://f/{file}"` |
| 31219 | `"semiorepo://sections/{path}"`                    | `"semiorepo://p/.../f/{file}/ss"` (file-scoped) — or keep as template with handler adaptation               |
| 31223 | `"semiorepo://section/{path}#{sectionpath}"`       | Hierarchical: `{file-uri}/s/{sectionSlug}`                                                                  |
| 31227 | `"semiorepo://definitions/{path}"`                 | `{file-uri}/ds`                                                                                             |
| 31231 | `"semiorepo://definition/{path}#{name}"`           | `{parent-uri}/d/{dkc}/{name}`                                                                               |
| 31235 | `"semiorepo://tickets"`                            | `"semiorepo://tks"`                                                                                         |
| 31239 | `"semiorepo://ticket/{year}/{month}/{day}/{slug}"` | `"semiorepo://y/{year}/m/{month}/d/{day}/tk/{slug}"`                                                        |
| 31243 | `"semiorepo://goals"`                              | `"semiorepo://gs"`                                                                                          |
| 31247 | `"semiorepo://goal/{slug}"`                        | `"semiorepo://g/{slug}"`                                                                                    |
| 31251 | `"semiorepo://policies"`                           | `"semiorepo://pls"`                                                                                         |
| 31255 | `"semiorepo://policy/{id}"`                        | `"semiorepo://pls/pl/{id}"`                                                                                 |
| 31259 | `"semiorepo://statutes"`                           | `"semiorepo://sts"`                                                                                         |
| 31263 | `"semiorepo://statute/{id}"`                       | `"semiorepo://sts/{id}"`                                                                                    |
| 31267 | `"semiorepo://contributors"`                       | `"semiorepo://cs"`                                                                                          |
| 31271 | `"semiorepo://contributor/{id}"`                   | `"semiorepo://cs/{id}"`                                                                                     |
| 31275 | `"semiorepo://commits"`                            | `"semiorepo://cms"`                                                                                         |
| 31279 | `"semiorepo://commit/{oid}"`                       | `"semiorepo://cms/{oid}"`                                                                                   |

---

## Category 10: MCP Resource Handler URI Parsing (Lines 32421-32810)

| Line  | OLD Prefix Used                                                         | NEW Prefix                                                                                                                                |
| ----- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| 32421 | `strings.TrimPrefix(request.Params.URI, "semiorepo://bundle/")`         | Parse from `"semiorepo://p/{pkc}/{project}/b/{bkc}/{bundle}"` — extract bundle name                                                       |
| 32460 | `strings.TrimPrefix(request.Params.URI, "semiorepo://folder/")`         | Parse from hierarchical `"semiorepo://fd/{fkc}/{path}"` or full hierarchy                                                                 |
| 32499 | `strings.TrimPrefix(request.Params.URI, "semiorepo://file/")`           | Parse from hierarchical `"semiorepo://...f/{file}"` — use `extractFileAndSectionsFromUri()`                                               |
| 32519 | `strings.TrimPrefix(request.Params.URI, "semiorepo://sections/")`       | Extract file path from `{file-uri}/ss`                                                                                                    |
| 32539 | `strings.TrimPrefix(request.Params.URI, "semiorepo://section/")`        | Parse from `{file-uri}/s/{slug}`                                                                                                          |
| 32569 | `strings.TrimPrefix(request.Params.URI, "semiorepo://definitions/")`    | Extract file path from `{file-uri}/ds`                                                                                                    |
| 32589 | `strings.TrimPrefix(request.Params.URI, "semiorepo://definition/")`     | Parse from `{parent-uri}/d/{dkc}/{name}`                                                                                                  |
| 32635 | `strings.TrimPrefix(request.Params.URI, "semiorepo://ticket/")`         | Parse from `"semiorepo://y/{YY}/m/{MM}/d/{DD}/tk/{slug}"`                                                                                 |
| 32685 | `strings.TrimPrefix(request.Params.URI, "semiorepo://goal/")`           | `strings.TrimPrefix(request.Params.URI, "semiorepo://g/")` — then strip `/g/` separators for sub-goals                                    |
| 32732 | `strings.TrimPrefix(request.Params.URI, "semiorepo://policy/")`         | `strings.TrimPrefix(request.Params.URI, "semiorepo://pls/pl/")`                                                                           |
| 32771 | `strings.TrimPrefix(request.Params.URI, "semiorepo://violation-kind/")` | `strings.TrimPrefix(request.Params.URI, "semiorepo://sts/")` (NOTE: this already uses wrong prefix `violation-kind` instead of `statute`) |
| 32810 | `strings.TrimPrefix(request.Params.URI, "semiorepo://contributor/")`    | `strings.TrimPrefix(request.Params.URI, "semiorepo://cs/")`                                                                               |

---

## Additional Findings (Outside Requested Areas)

### Line 221: Entity Kind URI

| Line | OLD String                                              | Notes                                                   |
| ---- | ------------------------------------------------------- | ------------------------------------------------------- |
| 221  | `"semiorepo://entitykind/" + PathToUriPath(entityKind)` | No new scheme specified for entitykind — may keep as-is |

### Line 13479: Territory URI

| Line  | OLD String                                         | Notes                                                  |
| ----- | -------------------------------------------------- | ------------------------------------------------------ |
| 13479 | `"semiorepo://territory/" + PathToUriPath(g.Name)` | No new scheme specified for territory — may keep as-is |

### Lines 38475, 38514: URI Parsing Utilities

These use `strings.TrimPrefix(uri, "semiorepo://")` which is scheme-agnostic and works with both old and new URIs. No change needed.

---

## Already Migrated (No Change Needed)

These `GetURI()` methods and builder functions already produce NEW-scheme URIs:

| Location          | Entity                            | Current URI Format                                                         |
| ----------------- | --------------------------------- | -------------------------------------------------------------------------- |
| Line 7460         | `Repo.GetURI()`                   | `"semiorepo://"` ✅                                                        |
| Line 7599         | `Project.GetURI()`                | `"semiorepo://p/{pkc}/{name}"` ✅                                          |
| Line 7673         | `Bundle.GetURI()`                 | `"semiorepo://p/{pkc}/{project}/b/{bkc}/{bundle}"` ✅                      |
| Line 7832         | `Folder.GetURI()`                 | Calls `buildFileUriFromPath()` ✅                                          |
| Line 8084         | `File.GetURI()`                   | Calls `buildFileUriFromPath()` ✅                                          |
| Line 8125-8132    | `Section.GetURI()`                | Calls `buildSectionUriFromPath()` or `"semiorepo://s/..."` ✅              |
| Line 8173-8181    | `Definition.GetURI()`             | Calls `buildDefinitionUriFromIdValue()` or `"semiorepo://d/..."` ✅        |
| Line 8261         | `Contributor.GetURI()`            | `"semiorepo://cs/..."` ✅                                                  |
| Line 8301         | `Draft.GetURI()`                  | `"semiorepo://drs/..."` ✅                                                 |
| Line 8382         | `Commit.GetURI()`                 | `"semiorepo://cms/..."` ✅                                                 |
| Line 8479-8492    | `Ticket.GetURI()`                 | `"semiorepo://y/YY/m/MM/d/DD/tk/SLUG"` or `"semiorepo://g/.../tk/SLUG"` ✅ |
| Line 8732         | `Policy.GetURI()`                 | `"semiorepo://pls/pl/..."` ✅                                              |
| Line 8762         | `StatuteMeta.GetURI()`            | `"semiorepo://pls/pl/.../sts/..."` ✅                                      |
| Line 10329        | `Todo.GetURI()`                   | `"semiorepo://tos/..."` ✅                                                 |
| Line 10366        | `Breach.GetURI()`                 | `"semiorepo://brs/..."` ✅                                                 |
| Line 12890-12892  | `Goal.GetURI()`                   | `"semiorepo://g/..."` ✅                                                   |
| Line 39386        | `buildFileUriFromPath()`          | Hierarchical `semiorepo://p/.../b/.../fd/.../f/...` ✅                     |
| Line 39425        | `buildSectionUriFromPath()`       | Hierarchical `{file-uri}/s/...` ✅                                         |
| Line 39440        | `buildDefinitionUriFromIdValue()` | Hierarchical `{parent-uri}/d/{dkc}/...` ✅                                 |
| Lines 39018-39666 | `IdToUri()`                       | All new scheme ✅                                                          |

---

## Summary Statistics

- **Total hardcoded OLD-scheme URIs found: ~45 locations**
- **Critical functional breakage: 8 locations** (parseSemioIdentificationLink prefix checks at lines 16691, 16960, 16997, 17055, 17070, 17090, 17107, 17156)
- **MCP resource definitions: 18 URIs** to update (lines 31191-31279)
- **MCP handler parsers: 11 functions** to update (lines 32421-32810)
- **Tree builder: 12 URIs** to update (lines 4774-5159)
- **File/folder stream constructors: 5 URIs** to replace with `buildFileUriFromPath()` calls (lines 2803, 3040, 3045, 21040, 21127)
- **Ticket URIs: 4 locations** to update (lines 4416, 4418, 21771, plus parser at 4526-4527)
- **CodebaseContext helpers: 2 methods** to update (lines 18040, 18047)
- **File resolution: 6 prefix checks** to update (lines 19502-19516)
