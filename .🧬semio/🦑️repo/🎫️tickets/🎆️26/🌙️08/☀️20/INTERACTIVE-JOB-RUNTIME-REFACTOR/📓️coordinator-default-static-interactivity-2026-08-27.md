# Default-Environment Static Interactivity Gate

Command: `bun x nx run workspace:verify-interactivity --skip-nx-cache`

Exit code: 1. This independent coordinator run uses no temporary Nx environment overrides. It is the static policy/discovery gate, not the separate full tool-job census, native/Wasm application gate or all-app latency measurement.

```text

> nx run workspace:verify-interactivity

> bun ./📜️script.ts verify interactivity

[verify interactivity apps] 32 descriptor(s), 101 app declaration(s), 57 launch-only product surface(s), 158 total surface(s), 4760 action row(s), 101 launch-covered app context(s), 0 missing launch context(s), 237 dev launch surface(s), 25 hostile/oracle self-test(s).
[0m[1m12929 |[0m     [[0m[32m"whole-document-producer"[0m, reconcile[0m[3m[1m.replace[0m([0m[32m"current.retained.get_index(self.next_node)"[0m, [0m[32m"current.retained.values().cloned().collect::<Vec<_>>().get(self.next_node)"[0m)],
[0m[1m12930 |[0m     [[0m[32m"missing-document-outcome"[0m, reconcile[0m[3m[1m.replace[0m([0m[32m"pub struct SurfaceDocumentOutcome"[0m, [0m[32m"struct SurfaceDocumentOutcome"[0m)],
[0m[1m12931 |[0m   ][0m[2m;[0m
[0m[1m12932 |[0m   [0m[35mfor[0m ([0m[35mconst[0m [name, mutatedReconcile] of producerMutations) [0m[35mif[0m (interactivityLiveReconcileFailures(mutatedReconcile, patches, reactor, value, schema).length === [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[verify interactivity] live reconcile self-test [0m${name}[0m[32m was falsely accepted.`[0m)[0m[2m;[0m
[0m[1m12933 |[0m   [0m[35mconst[0m failures = interactivityLiveReconcileFailures(reconcile, patches, reactor, value, schema)[0m[2m;[0m
[0m[1m12934 |[0m   [0m[35mif[0m (failures.length !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[verify interactivity] live reconcile baseline was falsely rejected: [0m${failures[0m[3m[1m.join[0m([0m[32m"; "[0m)}[0m[32m`[0m)[0m[2m;[0m
                                               [31m[1m^[0m
[0m[31merror[0m[2m:[0m [1m[verify interactivity] live reconcile baseline was falsely rejected: live reconcile fixed credit changed or disappeared: max_bytes: 2 * 1_024 * 1_024; live reconcile fixed credit changed or disappeared: SURFACE_RECONCILE_AGGREGATE_BYTES: usize = 8 * 1_024 * 1_024; production reactor does not mount one pre-reserved reconcile opportunity with deferred-storm and instance-close rearm[0m
[0m      [2mat [0m[0m[1m[3minteractivityLiveReconcileSelfTests[0m[2m ([0m[0m[36m[2m/Users/ueli/Documents/semio/[0m[36m📜️script.ts[0m[2m:[0m[33m12934[0m[2m:[33m40[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minteractivityAuditRun[0m[2m ([0m[0m[36m[2m/Users/ueli/Documents/semio/[0m[36m📜️script.ts[0m[2m:[0m[33m11562[0m[2m:[33m3[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrunInteractivityAudit[0m[2m ([0m[0m[36m[2m/Users/ueli/Documents/semio/[0m[36m📜️script.ts[0m[2m:[0m[33m10074[0m[2m:[33m20[0m[2m)[0m
[0m
[2mBun v1.3.14 (macOS arm64)[0m
Warning: command "bun ./📜️script.ts verify interactivity" exited with non-zero status code


 NX   Running target verify-interactivity for project workspace failed

Failed tasks:

- workspace:verify-interactivity

Hint: run the command with --verbose for more details.


```

