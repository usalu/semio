# Source Contract Self-Test Execution R15

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'`

Exit code: 0. Independently executed by the coordinator. Exact factory owners: 33; custom rows: 255; generic rows: 25; source self-tests: 929. This includes the helper-bound dispatch/capture predicates and private peer-commit authority obligations. It does not establish full command coverage or native/browser runtime timing.

```text

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=929 clean.



 NX   Successfully ran target verify-interactivity for project workspace



```

