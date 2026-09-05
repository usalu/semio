# Stdio Home I/O Compile Surface

## Contract

The neutral `semio.stdio.home-io-surface/v1` fixture fixes two distinct sets and the corresponding OS guest surface:

- Direct Home formats: `csv`, `json`, `xlsx`, `zip`.
- Required shared codec closure: `binary`, `deflate`, `txt`, `xml`.
- Framework OS feature: `space-guest`; the native `os-host-full` feature includes it plus sync and ZIP.

The full Stdio plugin remains 36 artifacts. `plugin-root` selects `full-artifact-catalog`; every non-Space Cargo consumer names that full feature explicitly. Space alone selects `home-io`. If a full host also reaches Space, Cargo feature union is safe and the full catalog wins.

## TDD and source verdict

The first registered Nx run failed at the missing `plugin-root -> full-artifact-catalog` feature edge. After the source split, the same command is green:

```text
bun nx run @semio-tech/stdio-plugin:home-io-surface-check --skip-nx-cache
stdio-home-io-surface-oracle: AJV=1 direct=4 shared=4 full=36 consumers=37
```

The gate validates the neutral JSON with AJV, the exact root-module cfg closure, full-only plugin/editor/viewer ownership, the Space feature edge, the OS host's WASIp2 target fence, the shared OS Space/document/workflow guest modules, and every other production Cargo consumer's explicit full-catalog edge.

Cargo's own target-aware feature graph independently shows that the Space WASIp2 component reaches only the narrow feature:

```text
semio-s-plugin-stdio
└── semio-s-plugin-stdio feature "home-io"
    └── semio-s-plugin-space
```

The OS host's DWG implementation remains a native/non-WASIp2 capability and its full Stdio dependency moved under that same target condition. Guest-unavailable SVG/DWG stubs stay available without linking DWG. The Space export-media command law now uses a neutral byte carrier, because that law verifies effect production rather than the unrelated DWG codec.

## Registration and native qualification

Source and native launch entries are registered at orders `411.077` and `411.078`; plugin-registry generation and immediate freshness check exited zero. The narrow Stdio crate itself first reached Cargo GREEN (`Finished dev profile` in `45.45s`). The first Space pass then exposed 42 framework-OS exports hidden by the old all-or-nothing host feature. The schema and source gate now require `space-guest`, while `os-host-full` retains native sync/ZIP and includes that guest surface. The next pass reduced the failures to the missing `manage_space` module import, which is repaired and included in the Home identity source oracle. An intermediate warm attempt reached clean Stdio completion but the Space subprocess could not be spawned because the host returned `Resource temporarily unavailable (os error 35)` under concurrent builds; that is an orchestration-capacity receipt, not a Rust verdict.

The bounded registered current-source retry is terminal GREEN:

```text
session 26480 exit 0
stdio-home-io-surface-oracle: AJV=1 direct=4 shared=4 full=36 consumers=37
Finished `dev` profile [unoptimized] target(s) in 6.18s
Finished `dev` profile [unoptimized] target(s) in 10.28s
NX Successfully ran target home-io-surface-native-check for project @semio-tech/stdio-plugin
```

The two `Finished` lines are respectively the narrow Stdio and Space `wasm32-wasip2` checks in the registered script. This proves the reduced source graph compiles; it does not by itself claim descriptor production or browser activation.

The current Space component/descriptor producer is registered separately at launch order `411.081`, using the retained ticket-owned `home-space-component-sol-target` with one Cargo job and 24-hour command/build/orchestration budgets. Plugin-registry generation and the immediate freshness check are green after this entry and the concurrently landed worker-maintenance entries. Materialization session `50835` is active; its terminal hashes belong in the browser/process report rather than being inferred from this compile gate.

The two older ticket-owned processes were pre-split snapshots and therefore could not qualify current source. With root coordination, Space session `45110` and Home session `80016` were cancelled; the verified orphan Cargo/rustc tree under the latter was terminated as well. Those sessions are explicitly unqualified. Current Home exact qualification runs separately against the retained public-member cache.
