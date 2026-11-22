# Strict Layout Listener Capacity Wait

The live 6179 task's OS PID is 94097. Its unified execution session is 30865. The process has no listening TCP socket yet and is holding the shared host-capacity gate plus fixed image holds. The lease registry shows zero artifact leases for this process. The reported `semio.compiler.host-capacity/v1` progress describes the following counted-core picker loop. The already-shared gate is not blocking it.

This machine exposes 10 compiler cores. The repository's authored generic counted request computes a preferred five cores and requires three free before admission.

| Permit range | Owner | Evidence |
|---|---|---|
| 0–4 | Layout guest wrapper PID 65522 | Rustc child 44804 was runnable at 73.9% CPU and 327 MiB RSS after 33 minutes 42 seconds on the final Layout component unit |
| 5–7 | Office guest wrapper PID 81240 | Rustc child 59688 was runnable at 77.8% CPU and 391 MiB RSS after 22 minutes 26 seconds on the final Office component unit |
| 8 | XLSX test wrapper PID 68178 | Active Cargo children included rustc work started in the preceding seconds |
| 9 | Free | One available permit is below the listener's admitted minimum |

The safe immediate path is to keep PID 94097 queued. When Office retires its three permits, the listener can start with Layout still holding its five. The Office process is doing real work and has no verified completion estimate. None of these processes were changed or restarted.

The generic serve owner takes the counted grant around Vite `createServer` and the optimizer scan. The WGPU path projects an owned configuration and sets `configFile:false`, so Vite does not bundle the configuration. The installed Vite source sets the dependency optimizer to undefined when discovery is disabled and the include list is empty, which is exactly the WGPU configuration. The scan wait therefore has no optimizer work. The generic three-core floor does not express a measured dependency of this startup path. The repository's only authored fixed class is `bun-bundle`, which requests two to four units. No current WGPU one-core startup declaration or runtime override exists.

One future scoped policy could reserve exactly one core during the captured WGPU configuration and listener setup, keep both task controls and lease order enforcement, and leave generic and React serve admission alone. That alone would expose an HTTP listener without proving a usable cold WGPU page. Vite's dev HTML transform injects `/@vite/client` even with pre-transform warmup disabled. Its first JS request and dependent client imports use the separate uncached transform path, which currently requires three cores. Published boot, renderer and plugin routes use static middleware. A smaller startup policy must also cover or measure the finite cold client transform path under one free core, without relaxing the guard for general transforms.
