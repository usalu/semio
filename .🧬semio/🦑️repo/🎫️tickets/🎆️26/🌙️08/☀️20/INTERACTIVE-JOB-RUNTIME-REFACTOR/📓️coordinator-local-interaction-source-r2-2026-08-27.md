# Local Interaction Source Gate R2

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-replication-rs:test-local-interaction-source --skip-nx-cache`

Exit code: 0. Independent coordinator run of schema/semantic, retirement and paged-query source/oracle gates; this is not a mounted transport or restore result.

```text

> nx run @semio-tech/framework-replication-rs:test-local-interaction-source

> bun ./📜️script.ts test-local-interaction-source

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] Local-interaction source cases=12 hostileRejections=9 oracle=immer semanticKeyBytes=6650 nativeRuntimeClaims=0
[DEBUG] Local-interaction retirement source cases=2 hostileRejections=2 grants=1,64,4096 oracle=lodash runtimeClaims=0
[DEBUG] Local-interaction query source cases=2 partitions=3 hostileRejections=3 oracle=node-crypto nativeRuntimeClaims=0



 NX   Successfully ran target test-local-interaction-source for project @semio-tech/framework-replication-rs



 NX   Nx detected a flaky task

  @semio-tech/framework-replication-rs:test-local-interaction-source

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

