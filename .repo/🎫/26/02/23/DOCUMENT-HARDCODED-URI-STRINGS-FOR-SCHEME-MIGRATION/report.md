# URI Scheme Migration Report

## Summary

This report documents all hardcoded URI strings in `repo/cli/main.go` that use the OLD scheme and need updating to the NEW scheme. It is organized by functional area.

**Key finding:** Most `GetURI()` methods, `buildFileUriFromPath()`, `buildSectionUriFromPath()`, `buildDefinitionUriFromIdValue()`, and the `IdToUri()` function already use the NEW scheme. The hardcoded OLD-scheme URIs are concentrated in **inline constructions** (tree nodes, file walkers, ticket rendering, MCP resource definitions, and MCP resource handlers).

---

## Category 1: File URI Construction in Walkers (Lines 2803, 3040, 3045)

These construct `composerepo://file/...` URIs inline instead of calling `buildFileUriFromPath()` or `GetURI()`.

| Line | OLD String                                         | Should Be                                                                                                                    |
| ---- | -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| 2803 | `"composerepo://file/" + PathToUriPath(relPath)`   | Call `buildFileUriFromPath(relPath)` (which produces hierarchical `composerepo://p/{pkc}/X/b/{bkc}/Y/fd/{fkc}/Z/f/filename`) |
| 3040 | `"composerepo://file/" + PathToUriPath(relParent)` | Call `buildFileUriFromPath(relParent)`                                                                                       |
| 3045 | `"composerepo://file/" + PathToUriPath(relLoc)`    | Call `buildFileUriFromPath(relLoc)`                                                                                          |

---

## Category 2: Ticket URIs in Tree Building (Lines 4416, 4418, 4526-4527)

These construct old-style `composerepo://ticket/...` URIs instead of calling `Ticket.GetURI()`.

| Line | OLD String                                                                                                           | Should Be                                                                                                                |
| ---- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| 4416 | `fmt.Sprintf("composerepo://ticket/%s", PathToUriPath(slug))`                                                        | `fmt.Sprintf("composerepo://y/%s/tk/%s", ..., slug)` or ideally construct a Ticket and call `.GetURI()`                  |
| 4418 | `fmt.Sprintf("composerepo://ticket/%d/%02d/%02d/%s", tTime.Year(), tTime.Month(), tTime.Day(), PathToUriPath(slug))` | `fmt.Sprintf("composerepo://y/%02d/m/%02d/d/%02d/tk/%s", year, month, day, slug)` — matches `Ticket.GetURI()` new scheme |
| 4526 | `strings.HasPrefix(n.URI, "composerepo://ticket/")`                                                                  | `strings.HasPrefix(n.URI, "composerepo://y/")` and then parse y/m/d/tk structure; or use `UriToId()`                     |
| 4527 | `strings.TrimPrefix(n.URI, "composerepo://ticket/")`                                                                 | Parse using new URI format `composerepo://y/YY/m/MM/d/DD/tk/SLUG`                                                        |

---

## Category 3: Tree Node Category URIs (Lines 4774, 4823, 4899, 4992, 5007, 5014, 5023, 5042, 5134, 5159, 5206)

These are category nodes in the tree builder that use old-style collection URIs.

| Line | OLD String                                                      | NEW String                                                                                                                                                                               |
| ---- | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 4774 | `URI: "composerepo://projects"`                                 | `URI: "composerepo://p"`                                                                                                                                                                 |
| 4823 | `URI: "composerepo://folders"`                                  | `URI: "composerepo://fds"`                                                                                                                                                               |
| 4899 | `URI: "composerepo://goals"`                                    | `URI: "composerepo://gs"`                                                                                                                                                                |
| 4992 | `URI: "composerepo://drafts"`                                   | `URI: "composerepo://drs"`                                                                                                                                                               |
| 5007 | `URI: "composerepo://policies"`                                 | `URI: "composerepo://pls"`                                                                                                                                                               |
| 5014 | `URI: "composerepo://policy/" + PathToUriPath(p.ID)`            | `URI: "composerepo://pls/pl/" + PathToUriPath(p.ID)` (or call `p.GetURI()` if Policy has one — currently `Policy.GetURI()` at line 8732 already returns `"composerepo://pls/pl/" + ...`) |
| 5023 | `URI: "composerepo://contributors"`                             | `URI: "composerepo://cs"`                                                                                                                                                                |
| 5042 | `URI: "composerepo://commits"`                                  | `URI: "composerepo://cms"`                                                                                                                                                               |
| 5134 | `URI: "composerepo://statute/" + StatuteIdToUriPath(string(k))` | `URI: "composerepo://sts/" + StatuteIdToUriPath(string(k))`                                                                                                                              |
| 5159 | `URI: "composerepo://statute/" + StatuteIdToUriPath(prefix)`    | `URI: "composerepo://sts/" + StatuteIdToUriPath(prefix)`                                                                                                                                 |
| 5206 | `URI: "composerepo://territory/" + PathToUriPath(g.Name)`       | Keep or rename to match Territory.GetURI() (line 13479 also uses `"composerepo://territory/"`) — no new scheme specified for territories                                                 |

---

## Category 4: `parseComposeIdentificationLink` Prefix Checks (Lines 16691, 16960, 16997, 17055, 17070, 17090, 17107, 17156)

These pass OLD-style URI prefixes to `parseComposeIdentificationLink()`. The function checks `strings.HasPrefix(uriValue, uriPrefix)`. The actual URIs embedded in source files already use the new hierarchical scheme, so the prefix check must match.

| Line  | OLD Prefix Passed             | Should Be                                                                                                                                                                                                                                                                  |
| ----- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 16691 | `"composerepo://section/"`    | These are used to **detect** identification links in comments. The actual URIs in comments are already hierarchical (e.g., `composerepo://...file.../s/SectionName`). The prefix should be just `"composerepo://"` or the function should detect `/s/` in the URI instead. |
| 16960 | `"composerepo://definition/"` | Same — should be `"composerepo://"` or detect `/d/` in the URI.                                                                                                                                                                                                            |
| 16997 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |
| 17055 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |
| 17070 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |
| 17090 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |
| 17107 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |
| 17156 | `"composerepo://definition/"` | Same                                                                                                                                                                                                                                                                       |

**NOTE**: `SectionHeaderUri()` (line 22377) calls `GetArtifactURI("section", data)` which returns the NEW hierarchical form (e.g., `composerepo://p/.../b/.../f/.../s/...`). Similarly `DefinitionHeaderUri()` (line 22400) returns the NEW form. So the prefix `"composerepo://section/"` will **never match** these new URIs. This is a **functional breakage** — the policy checker cannot find identification links.

---

## Category 5: `FileURI` / `FolderURI` Helpers on CodebaseContext (Lines 18040, 18047)

| Line  | OLD String                                                            | Should Be                                                                                                 |
| ----- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| 18040 | `return "composerepo://file/" + PathToUriPath(NormalizePath(path))`   | `return buildFileUriFromPath(NormalizePath(path))`                                                        |
| 18047 | `return "composerepo://folder/" + PathToUriPath(NormalizePath(path))` | `return buildFileUriFromPath(NormalizePath(path))` (Folder.GetURI() already calls `buildFileUriFromPath`) |

---

## Category 6: Ticket File Resolution URI Parsing (Lines 19502-19516)

| Line  | OLD String                                               | Should Be                                                                             |
| ----- | -------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| 19502 | `strings.HasPrefix(normalized, "composerepo://file/")`   | Also handle new hierarchical file URIs containing `/f/` segment                       |
| 19503 | `strings.TrimPrefix(normalized, "composerepo://file/")`  | Extract path from hierarchical URI using `extractFileAndSectionsFromUri()` or similar |
| 19506 | `strings.HasPrefix(normalized, "composerepo://files/")`  | `strings.HasPrefix(normalized, "composerepo://fis/")`                                 |
| 19507 | `strings.TrimPrefix(normalized, "composerepo://files/")` | `strings.TrimPrefix(normalized, "composerepo://fis/")`                                |
| 19515 | `strings.HasPrefix(uri, "composerepo://file/")`          | Also handle new hierarchical form                                                     |
| 19516 | `strings.TrimPrefix(uri, "composerepo://file/")`         | Extract path from hierarchical URI                                                    |

---

## Category 7: StreamFolders/StreamFiles URI Construction (Lines 20967, 21040, 21127)

| Line  | OLD String                                                                       | Should Be                                                                                        |
| ----- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 20967 | `URI: "composerepo://folder/" + PathToUriPath(buildFolderID(relPath, bundleID))` | Call `buildFileUriFromPath(relPath)` (which builds hierarchical URI) — same as `Folder.GetURI()` |
| 21040 | `URI: "composerepo://file/" + PathToUriPath(NormalizePath(relPath))`             | Call `buildFileUriFromPath(relPath)` (single file case in StreamFiles)                           |
| 21127 | `URI: "composerepo://file/" + PathToUriPath(NormalizePath(relPath))`             | Call `buildFileUriFromPath(relPath)` (walk case in StreamFiles)                                  |

---

## Category 8: Ticket URI in Rendering (Line 21771)

| Line  | OLD String                                                                                                                 | Should Be                                                                                                                                              |
| ----- | -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 21771 | `fmt.Sprintf("composerepo://ticket/%02d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, PathToUriPath(ticket.Slug))` | `fmt.Sprintf("composerepo://y/%02d/m/%02d/d/%02d/tk/%s", ticket.Year, ticket.Month, ticket.Day, PathToUriPath(ticket.Slug))` or call `ticket.GetURI()` |

---

## Category 9: MCP Resource Definitions (Lines 31191-31279)

| Line  | OLD URI                                              | NEW URI                                                                                                         |
| ----- | ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| 31191 | `"composerepo://root"`                               | `"composerepo://"`                                                                                              |
| 31195 | `"composerepo://bundles"`                            | `"composerepo://bs"` (or remove — bundles are accessed via `composerepo://p/{pkc}/X/b/{bkc}/Y`)                 |
| 31199 | `"composerepo://bundle/{id}"`                        | `"composerepo://p/{pkc}/{project}/b/{bkc}/{bundle}"`                                                            |
| 31203 | `"composerepo://folders"`                            | `"composerepo://fds"`                                                                                           |
| 31207 | `"composerepo://folder/{path}"`                      | `"composerepo://fd/{fkc}/{path}"` or hierarchical form                                                          |
| 31211 | `"composerepo://files"`                              | `"composerepo://fis"`                                                                                           |
| 31215 | `"composerepo://file/{path}"`                        | Hierarchical: `"composerepo://p/{pkc}/{project}/b/{bkc}/{bundle}/f/{file}"` or loose `"composerepo://f/{file}"` |
| 31219 | `"composerepo://sections/{path}"`                    | `"composerepo://p/.../f/{file}/ss"` (file-scoped) — or keep as template with handler adaptation                 |
| 31223 | `"composerepo://section/{path}#{sectionpath}"`       | Hierarchical: `{file-uri}/s/{sectionSlug}`                                                                      |
| 31227 | `"composerepo://definitions/{path}"`                 | `{file-uri}/ds`                                                                                                 |
| 31231 | `"composerepo://definition/{path}#{name}"`           | `{parent-uri}/d/{dkc}/{name}`                                                                                   |
| 31235 | `"composerepo://tickets"`                            | `"composerepo://tks"`                                                                                           |
| 31239 | `"composerepo://ticket/{year}/{month}/{day}/{slug}"` | `"composerepo://y/{year}/m/{month}/d/{day}/tk/{slug}"`                                                          |
| 31243 | `"composerepo://goals"`                              | `"composerepo://gs"`                                                                                            |
| 31247 | `"composerepo://goal/{slug}"`                        | `"composerepo://g/{slug}"`                                                                                      |
| 31251 | `"composerepo://policies"`                           | `"composerepo://pls"`                                                                                           |
| 31255 | `"composerepo://policy/{id}"`                        | `"composerepo://pls/pl/{id}"`                                                                                   |
| 31259 | `"composerepo://statutes"`                           | `"composerepo://sts"`                                                                                           |
| 31263 | `"composerepo://statute/{id}"`                       | `"composerepo://sts/{id}"`                                                                                      |
| 31267 | `"composerepo://contributors"`                       | `"composerepo://cs"`                                                                                            |
| 31271 | `"composerepo://contributor/{id}"`                   | `"composerepo://cs/{id}"`                                                                                       |
| 31275 | `"composerepo://commits"`                            | `"composerepo://cms"`                                                                                           |
| 31279 | `"composerepo://commit/{oid}"`                       | `"composerepo://cms/{oid}"`                                                                                     |

---

## Category 10: MCP Resource Handler URI Parsing (Lines 32421-32810)

| Line  | OLD Prefix Used                                                           | NEW Prefix                                                                                                                                  |
| ----- | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 32421 | `strings.TrimPrefix(request.Params.URI, "composerepo://bundle/")`         | Parse from `"composerepo://p/{pkc}/{project}/b/{bkc}/{bundle}"` — extract bundle name                                                       |
| 32460 | `strings.TrimPrefix(request.Params.URI, "composerepo://folder/")`         | Parse from hierarchical `"composerepo://fd/{fkc}/{path}"` or full document                                                                  |
| 32499 | `strings.TrimPrefix(request.Params.URI, "composerepo://file/")`           | Parse from hierarchical `"composerepo://...f/{file}"` — use `extractFileAndSectionsFromUri()`                                               |
| 32519 | `strings.TrimPrefix(request.Params.URI, "composerepo://sections/")`       | Extract file path from `{file-uri}/ss`                                                                                                      |
| 32539 | `strings.TrimPrefix(request.Params.URI, "composerepo://section/")`        | Parse from `{file-uri}/s/{slug}`                                                                                                            |
| 32569 | `strings.TrimPrefix(request.Params.URI, "composerepo://definitions/")`    | Extract file path from `{file-uri}/ds`                                                                                                      |
| 32589 | `strings.TrimPrefix(request.Params.URI, "composerepo://definition/")`     | Parse from `{parent-uri}/d/{dkc}/{name}`                                                                                                    |
| 32635 | `strings.TrimPrefix(request.Params.URI, "composerepo://ticket/")`         | Parse from `"composerepo://y/{YY}/m/{MM}/d/{DD}/tk/{slug}"`                                                                                 |
| 32685 | `strings.TrimPrefix(request.Params.URI, "composerepo://goal/")`           | `strings.TrimPrefix(request.Params.URI, "composerepo://g/")` — then strip `/g/` separators for sub-goals                                    |
| 32732 | `strings.TrimPrefix(request.Params.URI, "composerepo://policy/")`         | `strings.TrimPrefix(request.Params.URI, "composerepo://pls/pl/")`                                                                           |
| 32771 | `strings.TrimPrefix(request.Params.URI, "composerepo://violation-kind/")` | `strings.TrimPrefix(request.Params.URI, "composerepo://sts/")` (NOTE: this already uses wrong prefix `violation-kind` instead of `statute`) |
| 32810 | `strings.TrimPrefix(request.Params.URI, "composerepo://contributor/")`    | `strings.TrimPrefix(request.Params.URI, "composerepo://cs/")`                                                                               |

---

## Additional Findings (Outside Requested Areas)

### Line 221: Entity Kind URI

| Line | OLD String                                                | Notes                                                   |
| ---- | --------------------------------------------------------- | ------------------------------------------------------- |
| 221  | `"composerepo://entitykind/" + PathToUriPath(entityKind)` | No new scheme specified for entitykind — may keep as-is |

### Line 13479: Territory URI

| Line  | OLD String                                           | Notes                                                  |
| ----- | ---------------------------------------------------- | ------------------------------------------------------ |
| 13479 | `"composerepo://territory/" + PathToUriPath(g.Name)` | No new scheme specified for territory — may keep as-is |

### Lines 38475, 38514: URI Parsing Utilities

These use `strings.TrimPrefix(uri, "composerepo://")` which is scheme-agnostic and works with both old and new URIs. No change needed.

---

## Already Migrated (No Change Needed)

These `GetURI()` methods and builder functions already produce NEW-scheme URIs:

| Location          | Entity                            | Current URI Format                                                             |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------ |
| Line 7460         | `Repo.GetURI()`                   | `"composerepo://"` ✅                                                          |
| Line 7599         | `Project.GetURI()`                | `"composerepo://p/{pkc}/{name}"` ✅                                            |
| Line 7673         | `Bundle.GetURI()`                 | `"composerepo://p/{pkc}/{project}/b/{bkc}/{bundle}"` ✅                        |
| Line 7832         | `Folder.GetURI()`                 | Calls `buildFileUriFromPath()` ✅                                              |
| Line 8084         | `File.GetURI()`                   | Calls `buildFileUriFromPath()` ✅                                              |
| Line 8125-8132    | `Section.GetURI()`                | Calls `buildSectionUriFromPath()` or `"composerepo://s/..."` ✅                |
| Line 8173-8181    | `Definition.GetURI()`             | Calls `buildDefinitionUriFromIdValue()` or `"composerepo://d/..."` ✅          |
| Line 8261         | `Contributor.GetURI()`            | `"composerepo://cs/..."` ✅                                                    |
| Line 8301         | `Draft.GetURI()`                  | `"composerepo://drs/..."` ✅                                                   |
| Line 8382         | `Commit.GetURI()`                 | `"composerepo://cms/..."` ✅                                                   |
| Line 8479-8492    | `Ticket.GetURI()`                 | `"composerepo://y/YY/m/MM/d/DD/tk/SLUG"` or `"composerepo://g/.../tk/SLUG"` ✅ |
| Line 8732         | `Policy.GetURI()`                 | `"composerepo://pls/pl/..."` ✅                                                |
| Line 8762         | `StatuteMeta.GetURI()`            | `"composerepo://pls/pl/.../sts/..."` ✅                                        |
| Line 10329        | `Todo.GetURI()`                   | `"composerepo://tos/..."` ✅                                                   |
| Line 10366        | `Breach.GetURI()`                 | `"composerepo://brs/..."` ✅                                                   |
| Line 12890-12892  | `Goal.GetURI()`                   | `"composerepo://g/..."` ✅                                                     |
| Line 39386        | `buildFileUriFromPath()`          | Hierarchical `composerepo://p/.../b/.../fd/.../f/...` ✅                       |
| Line 39425        | `buildSectionUriFromPath()`       | Hierarchical `{file-uri}/s/...` ✅                                             |
| Line 39440        | `buildDefinitionUriFromIdValue()` | Hierarchical `{parent-uri}/d/{dkc}/...` ✅                                     |
| Lines 39018-39666 | `IdToUri()`                       | All new scheme ✅                                                              |

---

## Summary Statistics

- **Total hardcoded OLD-scheme URIs found: ~45 locations**
- **Critical functional breakage: 8 locations** (parseComposeIdentificationLink prefix checks at lines 16691, 16960, 16997, 17055, 17070, 17090, 17107, 17156)
- **MCP resource definitions: 18 URIs** to update (lines 31191-31279)
- **MCP handler parsers: 11 functions** to update (lines 32421-32810)
- **Tree builder: 12 URIs** to update (lines 4774-5159)
- **File/folder stream constructors: 5 URIs** to replace with `buildFileUriFromPath()` calls (lines 2803, 3040, 3045, 21040, 21127)
- **Ticket URIs: 4 locations** to update (lines 4416, 4418, 21771, plus parser at 4526-4527)
- **CodebaseContext helpers: 2 methods** to update (lines 18040, 18047)
- **File resolution: 6 prefix checks** to update (lines 19502-19516)
