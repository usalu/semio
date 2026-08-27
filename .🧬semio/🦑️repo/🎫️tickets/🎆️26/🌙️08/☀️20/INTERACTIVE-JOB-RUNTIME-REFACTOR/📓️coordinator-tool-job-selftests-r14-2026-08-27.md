# Source Contract Self-Test Execution R14

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'`

Exit code: 0. Source-only hostile-fixture verification. This does not establish complete command coverage or native/browser runtime timing.

```text

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=898 clean.



 NX   Successfully ran target verify-interactivity for project workspace



```

