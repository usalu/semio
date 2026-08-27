# Source Contract Self-Test Execution

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'`

Exit code: 0. Independent coordinator execution, not a full command census or native runtime gate. No application functionality or latency credit is inferred from passing source mutants.

```text

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=254 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=834 clean.



 NX   Successfully ran target verify-interactivity for project workspace



```

