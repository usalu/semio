---
date:
  created: '2025-12-14T22:02:37.644Z'
  updated: '2025-12-14T22:04:32.675Z'
slug: LOG-FRONTMATTER-DATE-LINES
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Reshape log frontmatter and migrate logs
model: gpt-5.2-codex
prompts:
  - >-
    Remove stats nesting, add nesting to lines {added;removed}, add nesting to
    date {created,updated}, rename base to commit, and migrate all existing logs
    to the new format.
commit: 7765b633fe739bc29cd811ac7ec884e782e2e945
affectedFiles:
  - AGENTS.md
  - README.md
  - log/2025/11/17/REFACTOR.md
  - log/2025/11/18/BREADCRUMB-RENDER-ERROR.md
  - log/2025/11/18/BREADCRUMB-SHIFT-ISSUE.md
  - log/2025/11/18/ENTITY-ID-REFACTOR.md
  - log/2025/11/18/MIGRATION-ISSUES.md
  - log/2025/11/19/FLATTEN-DESIGN-DIAGNOSIS.md
  - log/2025/11/19/FLATTEN-DESIGN.md
  - log/2025/11/20/SCHEMA-CHANGES-NAMES-AND-INTERFACES.md
  - log/2025/11/21/COMPLETE-KIT-PERSISTENCE.md
  - log/2025/11/21/I18N-SCRIPT-FIXES.md
  - log/2025/11/21/KIT-DIFF-TEST.md
  - log/2025/11/21/KIT-IMPORT-EXPORT-COMPLETE.md
  - log/2025/11/21/KIT-IMPORT-EXPORT.md
  - log/2025/11/21/PIECE-DISPLAY-METADATA.md
  - log/2025/11/21/PORT-LOOKUP-FIX.md
  - log/2025/11/21/SQL-FILES-DEEP-EQUALITY.md
  - log/2025/11/21/TRANSACTION-UNIFICATION.md
  - log/2025/11/22/DIFF-IMPLEMENTATION-COMPLETE.md
  - log/2025/11/22/FIXTURE-DATA-ISSUES.md
  - log/2025/11/22/IMPORT-EXPORT-EQUALITY.md
  - log/2025/11/22/IMPORT-EXPORT-FIXES.md
  - log/2025/11/22/KIT-APP-FILE-DROP.md
  - log/2025/11/22/KIT-IMPORT-EXPORT-DIAGNOSIS.md
  - log/2025/11/22/KIT-IMPORT-EXPORT-REMAINING.md
  - log/2025/11/22/MIGRATION-PORT-RESOLUTION.md
  - log/2025/11/22/PANEL-SECTION-HIERARCHY.md
  - log/2025/11/22/SIDE-WEAK-ENTITY.md
  - log/2025/11/22/SQL-TYPESCRIPT-COMPLIANCE.md
  - log/2025/11/22/WORKBENCH-PIECES-MERGE.md
  - log/2025/11/23/CONNECTION-UV-RENAME.md
  - log/2025/11/23/IMPORT-EXPORT-COMPLETE.md
  - log/2025/11/23/VALIDATION-SYSTEM.md
  - log/2025/11/24/AGENTS-REPORTS-UPDATE.md
  - log/2025/11/24/CI-CD-COMMANDS.md
  - log/2025/11/24/LOG-SYSTEM.md
  - log/2025/11/24/POWERSHELL-TO-TYPESCRIPT.md
  - log/2025/11/24/UI-ID-SYSTEM-ANALYSIS.md
  - log/2025/11/25/DESIGN-WINDOWS-LAYOUT.md
  - log/2025/11/25/FLATTEN-DESIGN-FIX.md
  - log/2025/11/25/KIT-ZIP-KIT-FIX.md
  - log/2025/11/26/DRAG-DROP-FINISH.md
  - log/2025/11/26/TUTORIALS-CONSOLIDATION.md
  - log/2025/11/26/UI-ELEMENT-IDS.md
  - log/2025/11/26/VSCODE-EXTENSION-FIX.md
  - log/2025/11/27/CLEAN-UP-DEBUG-LOGS.md
  - log/2025/11/27/DOCS-APP-TEST.md
  - log/2025/11/27/DOCS-HEADINGS-FIX.md
  - log/2025/11/27/DROP-COORDS.md
  - log/2025/11/27/HOME-DROP-ZONE-KIT-IMPORT.md
  - log/2025/11/27/HOME-KIT-ZIP-IMPORT.md
  - log/2025/11/27/SQL-JS-IMPORT.md
  - log/2025/11/27/TYPE-APP-TOOLBAR-FIX.md
  - log/2025/11/28/CLEAN.md
  - log/2025/11/28/CSHARP-SYNC-WITH-JS.md
  - log/2025/11/28/CSHARP-UNIT-TESTS.md
  - log/2025/11/28/DIAGRAM-PIECE-INTERACTION.md
  - log/2025/11/28/DRAG-DROP-FIX.md
  - log/2025/11/28/DRAG-DROP-IMPORT-TEST.md
  - log/2025/11/28/PIECE-HOVER-FIX.md
  - log/2025/11/28/REPORTS-DOC.md
  - log/2025/11/29/CSHARP-TESTS-SYNC.md
  - log/2025/11/29/KIT-PERFORMANCE-FIX.md
  - log/2025/11/29/LOADING-ERROR-MECHANISMS.md
  - log/2025/11/29/METABOLISM-IMPORT-PERF.md
  - log/2025/11/29/PYTHON-TESTS-SYNC.md
  - log/2025/11/29/SETTINGS_PANELS.md
  - log/2025/11/29/kit-import-test-fix.md
  - log/2025/11/30/KIT-SNAPSHOT-OPTIMIZATION.md
  - log/2025/11/30/STATE-MANAGEMENT-REFINEMENT.md
  - log/2025/11/30/YJS-UNATTACHED-MAP.md
  - log/2025/12/01/MIGRATE-KIT-MODELS.md
  - log/2025/12/01/MODEL-TAG-SELECTION.md
  - log/2025/12/01/SCHEMA-TAGS-CONCEPTS-MODELS.md
  - log/2025/12/01/SKETCHPAD-TEST-EXTEND.md
  - log/2025/12/01/SKETCHPAD-TEST-RESTRUCTURE.md
  - log/2025/12/01/SKETCHPAD-TESTS.md
  - log/2025/12/02/FILE-MIME-SCHEMA.md
  - log/2025/12/02/FIX-DESIGN-PAN-PERF.md
  - log/2025/12/02/FIX-DIFF-TEST.md
  - log/2025/12/02/FIX-HOVER-PERF.md
  - log/2025/12/02/FIX-INFINITE-LOOP-FOOTER.md
  - log/2025/12/02/FIX-KIT-IMPORT.md
  - log/2025/12/02/FIX-TAMBOUR-MODEL-WARNING.md
  - log/2025/12/02/KIT-SERIALIZATION-FIXES.md
  - log/2025/12/02/MODEL-LOADING-FIX.md
  - log/2025/12/02/SKETCHPAD-STATE-REFACTOR.md
  - log/2025/12/02/SKETCHPAD-TEST-ENHANCE.md
  - log/2025/12/02/STORE-OVERFETCH-FIX.md
  - log/2025/12/03/COORDINATE-SYSTEM-FIX.md
  - log/2025/12/03/CSHARP-SCHEMA-SYNC.md
  - log/2025/12/03/DESIGN-APP-GRANULAR-SUBSCRIPTIONS.md
  - log/2025/12/03/DESIGN-APP-PERFORMANCE.md
  - log/2025/12/03/ENTITY-ID-DIFF-REFACTOR.md
  - log/2025/12/03/GRASSHOPPER-COMPONENTS.md
  - log/2025/12/03/GRASSHOPPER-REFLECTION-REMOVAL.md
  - log/2025/12/03/INFINITE-LOOP-NAVBAR-FOOTER.md
  - log/2025/12/03/PANEL-TESTS-FIX.md
  - log/2025/12/03/PYTHON-TESTS-COMPLETE.md
  - log/2025/12/03/SCHEMA-ENTITY-ID-REFACTOR.md
  - log/2025/12/03/SCHEMA-SYNC.md
  - log/2025/12/03/STATE-MANAGEMENT-OPTIMIZATION.md
  - log/2025/12/03/TYPE-APP-STATE-OPTIMIZATION.md
  - log/2025/12/03/VALIDATION-UNIFICATION.md
  - log/2025/12/04/FIX-LINTING.md
  - log/2025/12/04/I18N-FIX-ALL.md
  - log/2025/12/04/PANEL-TESTS-SIMPLIFIED.md
  - log/2025/12/04/TYPESCRIPT-ERRORS-FIX.md
  - log/2025/12/05/FULL-XSTATE-IMPLEMENTATION.md
  - log/2025/12/05/FULL-XSTATE-TRANSITION.md
  - log/2025/12/05/XSTATE-MIGRATION-COMPLETE.md
  - log/2025/12/05/XSTATE-MIGRATION.md
  - log/2025/12/06/SKETCHPAD-XSTATE-REFACTOR.md
  - log/2025/12/06/XSTATE-PURE-MIGRATION.md
  - log/2025/12/07/GRANULAR-STORE.md
  - log/2025/12/07/GRANULAR-SUBSCRIPTIONS.md
  - log/2025/12/07/STATE-WRITES.md
  - log/2025/12/08/KIT-DIFF-TESTS.md
  - log/2025/12/08/MIGRATE-DESIGN-APP-TRIADIC-HOOKS.md
  - log/2025/12/08/SKETCHPAD-XSTATE-CONTEXT-FIX.md
  - log/2025/12/08/STATE-MACHINE-REFACTOR.md
  - log/2025/12/08/STATE-MANAGEMENT-REFACTOR.md
  - log/2025/12/08/UI-STATE-MACHINE.md
  - log/2025/12/09/APP-STATE-DECOUPLE.md
  - log/2025/12/09/DESIGN-DRAGDROP-TEST.md
  - log/2025/12/09/DESIGN-TEST-FLAT-PLANES.md
  - log/2025/12/09/HOOK-REFACTOR.md
  - log/2025/12/09/KIT-CONCEPT-NAMES.md
  - log/2025/12/09/KIT-ENTITIES-EXPORT.md
  - log/2025/12/09/KIT-ENTITIES-LOOKUP.md
  - log/2025/12/09/PLAYWRIGHT-DND-TEST.md
  - log/2025/12/09/SKETCHPAD-FSM-HIERARCHY.md
  - log/2025/12/09/SKETCHPAD-VITE-EXTERNAL-IMPORT.md
  - log/2025/12/09/SQL-WASM-SKETCHPAD-BUILD.md
  - log/2025/12/09/STATE-MACHINE-REFACTOR.md
  - log/2025/12/10/CLEAN-CODEBASE.md
  - log/2025/12/12/APP-PLUGIN-REFACTOR.md
  - log/2025/12/12/CREATE-INTERFACE-TAG-CLICK.md
  - log/2025/12/12/DEV-JS-SPECIALIZED-DEV.md
  - log/2025/12/12/EXTEND-PANEL-TESTS.md
  - log/2025/12/12/KIT-ROW-EXPAND.md
  - log/2025/12/12/NAVBAR-PANEL-ORDER.md
  - log/2025/12/12/ORIGIN-IN-HOOKS.md
  - log/2025/12/12/PANEL-TOGGLE-FIX.md
  - log/2025/12/12/TABLE-ROW-HEIGHT-FOOTER.md
  - log/2025/12/12/TRANSACTION-CONTEXT.md
  - log/2025/12/12/TRANSACTION-PROP-REMOVAL.md
  - log/2025/12/12/TRIADIC-HOOKS-REFACTOR.md
  - log/2025/12/14/KIT-DETAILS-PANEL.md
  - log/2025/12/14/LOG-FRONTMATTER-DATE-LINES.md
  - log/2025/12/14/LOG-PROMPTS-STATS.md
  - log/2025/12/14/PANEL-TOGGLE-FIX.md
  - log/2025/12/14/PANEL-TOGGLE-HOOKS-BUG.md
  - log/2025/12/14/TOOLBAR-TOOLS-E2E.md
  - log/2025/12/14/TSC-ERRORS.md
  - log/prompts.md
  - scripts/log.ts
lines:
  added: 1745
  removed: 240
---
# Previously
`scripts/log.ts` stored task tracking under a nested `stats` object and used a single `date` timestamp, which made it harder to evolve the schema and update git-derived line stats consistently across multiple prompts.

# Plan
Flatten task tracking fields in frontmatter (remove `stats`), rename `base` to `commit`, and nest `lines` and `date` as objects.
Update the CLI commands to write/read the new structure and keep `date.updated` current.
Migrate all existing logs in `log/` to the latest frontmatter structure.
Update `README.md` and `AGENTS.md` to document the new format and commands.

# Changes
Replaced `stats` frontmatter with `commit`, `affectedFiles`, and `lines.{added,removed}`, and changed `date` to `{created,updated}` in `scripts/log.ts`.
Extended `tsx scripts/log.ts migrate` to rewrite frontmatter for all existing logs and skip YAML undefined values by normalizing defaults.
Updated `README.md` and `AGENTS.md` log system documentation to the new frontmatter format and stats workflow.
