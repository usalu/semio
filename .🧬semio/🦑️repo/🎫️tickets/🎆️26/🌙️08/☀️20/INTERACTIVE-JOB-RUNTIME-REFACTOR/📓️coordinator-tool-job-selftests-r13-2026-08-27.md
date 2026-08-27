# Source Contract Self-Test Execution R13

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'`

Exit code: 0. Independent coordinator execution. Source tests do not certify full command coverage, native runtime or all-app timing.

```text

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=254 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=851 clean.



 NX   Successfully ran target verify-interactivity for project workspace



```

