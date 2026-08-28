# Independent Source Self-Tests R25

Actual coordinator run: **1009 self-tests clean**,33 exact-factory-proof owners,255 custom rows,25 generic rows,exit0. This verifies source self-tests/bootstrap at the recorded shared-source boundary only. It does not resolve the previous full census RED, certify all app commands, or establish guest/runtime/timing behavior.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'
```

## Actual Output

```text

> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=1009 clean.



 NX   Successfully ran target verify-interactivity for project workspace



```

