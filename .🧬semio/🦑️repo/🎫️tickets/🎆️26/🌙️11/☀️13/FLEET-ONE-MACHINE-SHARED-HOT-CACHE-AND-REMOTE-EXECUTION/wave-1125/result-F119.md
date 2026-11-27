# F119: path-to-`$id` rule for the 1673 id-less schema modules (STOPPED at step 2, nothing applied)

Lane F119, 2026-11-26. Read-only. No repo file, schema, catalog or lock was touched; only `T/🧰️scripts/F119/{dump-modules.ts,prove-rule.py}` were created. Gate from the brief: the rule must reproduce at least 99% of the existing conforming ids; it reproduces **24.4%** (1558 of 6388), so I stopped as instructed.

## 1. Grammar (reused exactly, from the generator)
`schemaDocumentIdParts` / `schemaScopeIdFromDocumentId` with `taxonomy.schemaExportResolution`: `$id` = `idBase` + `<scope segments>/<facet>.json`, `idBase` = `https://json.schemas.assets.semio-tech.com/`; facet `^[a-z0-9]+(?:-[a-z0-9]+)*\.json$`; scope = segments joined by `.` matching `^[a-z0-9]+(?:-[a-z0-9]+)*(?:\.[a-z0-9]+(?:-[a-z0-9]+)*)*$`; at least one scope segment. Consistency law: facet documents keep the root's scope path. Mutation leaf id is `<scope>/mutation/<verb>/schema.json`, aggregate `mutations.json` (taxonomy `mutationScopeSegment`, `mutationLeafFacetFilename`).

## 2. The candidate rule and its match rate
Candidate (best effort, `T/🧰️scripts/F119/prove-rule.py`): strip the emoji prefix of every segment, drop `🔨️modules` and `🧬️schema`, `🧬️mutations` -> `mutation`; `🧰️framework/🛍️products/X` -> `X/...`; `🧰️framework/🔨️modules/X` -> `framework/X/...`; plugins `s/<plugin>/<artifact>[/<version>/<subset> unless any]/<rest>` dropping editor/viewer/modes; facet `schema.json`.
Data: 8528 modules with a root document, 6946 have a root `$id`, 6388 of them conform to the grammar (558 do not: `urn:`, `schema://`, `s.fem.3d.presence/...`, ids without a facet file, etc., so they are excluded from the base and are themselves unaddressable findings).

| Area | conforming | scope path matches | exact match |
|---|---:|---:|---:|
| plugins (`✏️s/🔌️plugins`) | 5952 | 1448 | 1328 |
| framework modules | 121 | 91 | 75 |
| os | 219 | 151 | 98 |
| repo | 50 | 31 | 24 |
| hub | 27 | 23 | 23 |
| print | 6 | 6 | 6 |
| mit-bestand | 5 | 4 | 2 |
| impl-other / pets / course / teaching / semio-tech | 8 | 3 | 2 |
| **total** | **6388** | **1757 (27.5%)** | **1558 (24.4%)** |
Non-plugin only: 230 of 436 exact (53%); even there it fails.

## 3. Mismatch classification (4830 mismatches)
- 3942 hand-named or restructured scopes: the id is an authored name, not the path (`trace/record` -> `framework/trace-record`; `trace/memory` -> `framework/guest-memory`; `🧩️compositions/🗄️stdio` -> `s/stdio`; `🔨️plugin-module/🌎️hub-source` -> `os/plugin/registry/hub-source`; `product/pets` for `products/pets`; `os/dsl` for `framework/dsl`; `pixels/selection` without the `framework` prefix; `schema` module -> `framework/schema/schema.json`).
- 688 `app/`/`window/` lane scheme: viewer/editor lanes take scheme `app` and not `s` (`app/writer/writer/config`), windows collapse `modes/edit/windows/main` to `window/main` or `main-window`, and `window/<plugin>/<win>/...` exists as a third prefix. Editor and viewer lanes of one artifact share one id (the documented 38-way `app.stdio.semio.config` duplicates), so path -> id is not injective, let alone invertible.
- 199 facet filename differs: `component.json` (os, framework, dev: 77 rows), `artifact.json`, `opening.json`, `snapshot.json`, `provenance.json`, `texts.json`, `cases.json`, `contribution`... The facet name is a per-document authored name.
- 1 mutation verb renamed (descriptor name, e.g. dir `⚙️editor-settings` -> `set-editor-settings`, `🎥️cam` -> `set-camera`): the verb comes from the mutation descriptor, not the path. (Plugin mutation leaves are the bulk of the 5036 `s/.../mutation/...` ids; the ones that match are those whose directory already equals the verb.)

## 4. What this means for the 1673 id-less modules (not applied)
Areas of the 1674 `schema-module-id-missing` rows (1673 modules plus 1 facet doc): framework modules 580, repo 546, os 286, plugins 190, teaching 25, impl-other (dev/modules) 24, print 15, hub 4, semio-tech 2, mit-bestand 1, course 1. The deterministic rule would be right for roughly the 53% non-plugin share in the best case and nobody can tell which half. Writing guessed ids would (a) not be "the" rule, (b) create catalogued scopes whose names later need rewriting repo-wide (every `$ref` and twin), and (c) risk new `schema-scope-ambiguous` / `schema-document-id-duplicate` rows (lane ids are shared by design today).
- A defensible split exists and needs a coordinator decision before any apply: (1) the regular, self-describing part (framework `🧬️schema` leaf modules, repo, hub, print, teaching: scope path rule + `schema.json`, matching ~75% in those areas) could be applied with a per-id uniqueness/collision gate, after a taxonomy decision on the `framework/` and `os/` prefixes and on `component.json` vs `schema.json` for os/dev; (2) plugin lane/window modules need an authored naming table (the `app`/`window`/`s` scheme and the verb names), which belongs in the taxonomy as data (a `schemaIdNaming` section), not in a script.
- Recommendation: add the rule to the taxonomy as schema-first data (prefix table per owner level, facet filename per level, lane scheme per lane kind), make the generator expose `schemaDocumentIdForModule(modulePath)` from that table, prove it with a law "rule reproduces every existing conforming id or the mismatch is listed in an explicit exceptions table", then apply. That is a design step, not a codemod; the 24.4% base rate shows the table is large (about 4800 exceptions today, nearly all plugin lanes).

## 5-6. Not done
Steps 3 (dry run), 4 (apply), 5 (catalog regeneration, dump before/after) and 6 (tests) were not run: the gate in step 2 failed, so the catalog and findings are unchanged (`schema-module-id-missing` stays 1673). No lock was taken, no file in the repo changed.

## Reproduce
`bun T/🧰️scripts/F119/dump-modules.ts <scratch>/dump.json` (read-only, same inventory the checker uses) then `python3 T/🧰️scripts/F119/prove-rule.py <scratch>/dump.json`. The proof script refuses an empty path and never writes to the repo.
