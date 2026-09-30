# Final Owned Schema Inventory

Actual final-source global schema-check failed after fresh catalog/docs generation. The full count is recorded below; no baselines, exemptions or eligibility changes were added.

Selected authored path fragments:

- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema/`
- `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/`
- `/🧰️ui-action-collection/`
- `/🧱️boxed-fixed-slots/`
- `/🧭️direction/`
- `/🌊️session/`
- `/🛡️access-policy/`
- `/📦️deployment/`
- `/🎬️media/🧬️schema/`
- `/🎬️sequence/🚪️io/`
- `/🧩️composition-laws/`
- `/🧩️extension/🚪️retirement/`
- `/🏗️build/🔍️freshness/`

Actual selected diagnostic lines: 0.

None.

Global check header and conclusion:

```text
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run workspace:schema-check --skip-nx-cache "--output-style=static"
Waiting for graph construction in another process to complete

> nx run workspace:schema-check

> bun ./📜️script.ts schema check

{"code":"schema-module-id-missing","path":"♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema","detail":"♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json must declare the $id that names this module's scope."}
{"code":"schema-owner-ineligible","path":"♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema","detail":"♻️mit-bestand/🧺️demonstrator/🐚️shell is not a declared schemaScopeOwnerLevels level."}
{"code":"schema-dialect-not-draft-07","path":"♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json","detail":"$schema must be http://json-schema.org/draft-07/schema#, got null."}
{"code":"schema-module-id-missing","path":"♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json","detail":"Declare $id as https://json.schemas.assets.semio-tech.com/<scope path>/<facet>.json."}
{"code":"schema-owner-ineligible","path":"✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧬️schema","detail":"✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config is not a declared schemaScopeOwnerLevels level."}
[schema check] schema-mutation-leaf-id=313
[schema check] schema-mutation-leaf-schema-absent=10
[schema check] schema-owner-ineligible=434
[schema check] schema-placement-forbidden-filename=8
[schema check] schema-ref-unresolved=237
[schema check] schema-scope-ambiguous=149
[schema check] shared-code-table=43 unshared-codes=0
Warning: command "bun ./📜️script.ts schema check" exited with non-zero status code


 NX   Running target schema-check for project workspace failed

Failed tasks:

- workspace:schema-check

Hint: run the command with --verbose for more details.

  Run duration:      59.3s
  Cache:             Skipped (--skip-nx-cache)
  Critical path:     58.6s (1 task)
  Recoverable time:  <1ms

  Recommendations:
    - Cache: drop --skip-nx-cache to restore unchanged tasks instantly.
    - Speed up or split the longest tasks on the critical path:
        workspace:schema-check    58.6s
error: script "nx" exited with code 1
```

This establishes only the selected authored schema domains; whole repository schema health remains failing.

Parsed actual global diagnostics: 9390.

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧬️schema/`: 0 diagnostic(s).
- `🧰️framework/🔨️modules/🧊️3d/🧬️schema/`: 0 diagnostic(s).
