---
slug: LOG-FRONTMATTER-DATE-LINES
summary: Reshape log frontmatter and migrate logs
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.935Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      Remove stats nesting, add nesting to lines {added;removed}, add nesting to
      date {created,updated}, rename base to commit, and migrate all existing
      logs to the new format.
    date:
      started: "2025-12-14T22:02:37.644Z"
    model: gpt-5-2
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 7765b633fe739bc29cd811ac7ec884e782e2e945
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 11
            removed: 2
        - path: README.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/17/REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/18/BREADCRUMB-RENDER-ERROR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/18/BREADCRUMB-SHIFT-ISSUE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/18/ENTITY-ID-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/18/MIGRATION-ISSUES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/19/FLATTEN-DESIGN-DIAGNOSIS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/19/FLATTEN-DESIGN.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/20/SCHEMA-CHANGES-NAMES-AND-INTERFACES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/COMPLETE-KIT-PERSISTENCE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/I18N-SCRIPT-FIXES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/KIT-DIFF-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/KIT-IMPORT-EXPORT-COMPLETE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/KIT-IMPORT-EXPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/PIECE-DISPLAY-METADATA.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/PORT-LOOKUP-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/SQL-FILES-DEEP-EQUALITY.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/21/TRANSACTION-UNIFICATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/DIFF-IMPLEMENTATION-COMPLETE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/FIXTURE-DATA-ISSUES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/IMPORT-EXPORT-EQUALITY.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/IMPORT-EXPORT-FIXES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/KIT-APP-FILE-DROP.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/KIT-IMPORT-EXPORT-DIAGNOSIS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/KIT-IMPORT-EXPORT-REMAINING.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/MIGRATION-PORT-RESOLUTION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/PANEL-SECTION-HIERARCHY.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/SIDE-WEAK-ENTITY.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/SQL-TYPESCRIPT-COMPLIANCE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/22/WORKBENCH-PIECES-MERGE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/23/CONNECTION-UV-RENAME.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/23/IMPORT-EXPORT-COMPLETE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/23/VALIDATION-SYSTEM.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/24/AGENTS-REPORTS-UPDATE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/24/CI-CD-COMMANDS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/24/LOG-SYSTEM.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/24/POWERSHELL-TO-TYPESCRIPT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/24/UI-ID-SYSTEM-ANALYSIS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/25/DESIGN-WINDOWS-LAYOUT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/25/FLATTEN-DESIGN-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/25/KIT-ZIP-KIT-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/26/DRAG-DROP-FINISH.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/26/TUTORIALS-CONSOLIDATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/26/UI-ELEMENT-IDS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/26/VSCODE-EXTENSION-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/CLEAN-UP-DEBUG-LOGS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/DOCS-APP-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/DOCS-HEADINGS-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/DROP-COORDS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/HOME-DROP-ZONE-KIT-IMPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/HOME-KIT-ZIP-IMPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/SQL-JS-IMPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/27/TYPE-APP-TOOLBAR-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/CLEAN.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/CSHARP-SYNC-WITH-JS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/CSHARP-UNIT-TESTS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/DIAGRAM-PIECE-INTERACTION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/DRAG-DROP-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/DRAG-DROP-IMPORT-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/PIECE-HOVER-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/28/REPORTS-DOC.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/CSHARP-TESTS-SYNC.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/KIT-PERFORMANCE-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/LOADING-ERROR-MECHANISMS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/METABOLISM-IMPORT-PERF.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/PYTHON-TESTS-SYNC.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/SETTINGS_PANELS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/29/kit-import-test-fix.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/30/KIT-SNAPSHOT-OPTIMIZATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/30/STATE-MANAGEMENT-REFINEMENT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/11/30/YJS-UNATTACHED-MAP.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/MIGRATE-KIT-MODELS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/MODEL-TAG-SELECTION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/SCHEMA-TAGS-CONCEPTS-MODELS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/SKETCHPAD-TEST-EXTEND.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/SKETCHPAD-TEST-RESTRUCTURE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/01/SKETCHPAD-TESTS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FILE-MIME-SCHEMA.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-DESIGN-PAN-PERF.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-DIFF-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-HOVER-PERF.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-INFINITE-LOOP-FOOTER.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-KIT-IMPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/FIX-TAMBOUR-MODEL-WARNING.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/KIT-SERIALIZATION-FIXES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/MODEL-LOADING-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/SKETCHPAD-STATE-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/SKETCHPAD-TEST-ENHANCE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/02/STORE-OVERFETCH-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/COORDINATE-SYSTEM-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/CSHARP-SCHEMA-SYNC.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/DESIGN-APP-GRANULAR-SUBSCRIPTIONS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/DESIGN-APP-PERFORMANCE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/ENTITY-ID-DIFF-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/GRASSHOPPER-COMPONENTS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/GRASSHOPPER-REFLECTION-REMOVAL.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/INFINITE-LOOP-NAVBAR-FOOTER.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/PANEL-TESTS-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/PYTHON-TESTS-COMPLETE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/SCHEMA-ENTITY-ID-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/SCHEMA-SYNC.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/STATE-MANAGEMENT-OPTIMIZATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/TYPE-APP-STATE-OPTIMIZATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/03/VALIDATION-UNIFICATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/04/FIX-LINTING.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/04/I18N-FIX-ALL.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/04/PANEL-TESTS-SIMPLIFIED.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/04/TYPESCRIPT-ERRORS-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/05/FULL-XSTATE-IMPLEMENTATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/05/FULL-XSTATE-TRANSITION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/05/XSTATE-MIGRATION-COMPLETE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/05/XSTATE-MIGRATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/06/SKETCHPAD-XSTATE-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/06/XSTATE-PURE-MIGRATION.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/07/GRANULAR-STORE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/07/GRANULAR-SUBSCRIPTIONS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/07/STATE-WRITES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/KIT-DIFF-TESTS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/MIGRATE-DESIGN-APP-TRIADIC-HOOKS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/SKETCHPAD-XSTATE-CONTEXT-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/STATE-MACHINE-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/STATE-MANAGEMENT-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/08/UI-STATE-MACHINE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/APP-STATE-DECOUPLE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/DESIGN-DRAGDROP-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/DESIGN-TEST-FLAT-PLANES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/HOOK-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/KIT-CONCEPT-NAMES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/KIT-ENTITIES-EXPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/KIT-ENTITIES-LOOKUP.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/PLAYWRIGHT-DND-TEST.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/SKETCHPAD-FSM-HIERARCHY.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/SKETCHPAD-VITE-EXTERNAL-IMPORT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/SQL-WASM-SKETCHPAD-BUILD.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/09/STATE-MACHINE-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/10/CLEAN-CODEBASE.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/APP-PLUGIN-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/CREATE-INTERFACE-TAG-CLICK.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/DEV-JS-SPECIALIZED-DEV.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/EXTEND-PANEL-TESTS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/KIT-ROW-EXPAND.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/NAVBAR-PANEL-ORDER.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/ORIGIN-IN-HOOKS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/PANEL-TOGGLE-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/TABLE-ROW-HEIGHT-FOOTER.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/TRANSACTION-CONTEXT.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/TRANSACTION-PROP-REMOVAL.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/12/TRIADIC-HOOKS-REFACTOR.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/KIT-DETAILS-PANEL.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/LOG-FRONTMATTER-DATE-LINES.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/LOG-PROMPTS-STATS.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/PANEL-TOGGLE-FIX.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/PANEL-TOGGLE-HOOKS-BUG.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/TOOLBAR-TOOLS-E2E.md
          lines:
            added: 11
            removed: 2
        - path: log/2025/12/14/TSC-ERRORS.md
          lines:
            added: 11
            removed: 2
        - path: log/prompts.md
          lines:
            added: 11
            removed: 2
        - path: scripts/log.ts
          lines:
            added: 11
            removed: 2
      created: []
      removed: []
    lines:
      added: 1738
      removed: 316
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
