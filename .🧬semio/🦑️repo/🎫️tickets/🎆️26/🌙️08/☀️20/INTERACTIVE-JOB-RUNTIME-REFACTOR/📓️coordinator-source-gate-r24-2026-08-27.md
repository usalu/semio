# Independent Source Self-Test R24

The coordinator actually reran the canonical source self-test target after the paired Surface and shared-list checkpoint: **1,009 self-tests pass**, 33 exact-factory-proof owners, 255 custom and 25 generic rows, exit 0. This is not a full command census, all-app runtime, or static-interactivity findings pass.

```text
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=1009 clean.



 NX   Successfully ran target verify-interactivity for project workspace



exit_code=0
```

