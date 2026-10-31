# Current Graph Farm Mode Audit

Read-only Source and selected live environment/argv inspection for schema issuer362816 and graph child363040. No Source edit, instrumentation, signal or extra run was made.

The progress counter's running projects are async callbacks inside the parent process, not an OS process count. In project-nodes/MJS, every inference is wrapped by createGraphParentMemoryGovernorV1, while active project names are reported independently. Four running projects can therefore legitimately be four callbacks in one Bun process.

The farm can remain absent in two ordinary paths:

1. Selected owner admission: graph/TS discoverBuildGraphV1 supplies options.ownerAdmission when selection.kind is not all. project-nodes sets farm to undefined in that mode.
2. Warm/mostly reusable records: observeProjectInferenceV1 and observeInferenceComputationV1 return validated cached answers before the prepare callback. openProjectFarmV1 starts only once at least24cache misses have called prepare and those misses constitute at least25% of processed projects. Valid hits never count toward engagement.

An engaged farm emits own/asking-host/start messages before serving jobs. The current log had zero such messages as well as zero worker failures/declines/recycles. Graph child363040 had no SEMIO_GRAPH_WORKERS override; its carried capacity plan named the real schema grant, and Root's actual reservation was6318MB, giving3159MB parent share under unchanged policy. The owned worker share would be eligible if the farm were engaged. The plan's carried memoryMegabytes12288 is scheduling data, not the actual qualified reservation; it cannot finance extra workers.

The farm's actual spawn uses ordinary child_process.spawn and no detached option. Captured workers use the semio-role=graph-farm-worker argument and the same captured launcher; they would normally appear below this graph child in Root's descendant samples. The Source contains no intentional out-of-tree worker daemon. Root's measured three-process lineage with no worker role is therefore consistent with a legitimately unengaged/selected farm, not evidence that the worker pool is reducing current memory. Existing runtime observations do not distinguish selected owner admission from insufficient miss ratio; no reason was invented.

The current gate can witness parent inference/resource behavior, but cannot certify physical worker recycling/retirement effectiveness if no worker launches. Those mechanisms retain their separate real-worker and delayed-exit acceptance in graph-memory-retirement-audit.md. The historical13416MB gate did launch a worker and had real staging failures, so its topology differs from this observation.

At a later read the same log advanced to288/532projects with parent-only memory admission/serial messages around2792MB against3159MB. This is progressing parent work, not a terminal outcome or pool-effect measurement.
