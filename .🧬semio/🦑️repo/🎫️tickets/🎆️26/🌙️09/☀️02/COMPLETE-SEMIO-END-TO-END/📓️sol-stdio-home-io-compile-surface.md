# Stdio Home I/O Compile Surface

## Contract

The neutral `semio.stdio.home-io-surface/v1` fixture fixes three distinct codec surfaces and the corresponding OS guest surface:

- Direct Home formats: `csv`, `json`, `xlsx`, `zip`.
- Required shared codec closure: `binary`, `deflate`, `txt`, `xml`.
- Native OS-host codec: `native-dwg-codec`, admitting only the existing DWG artifact tree.
- Framework OS feature: `space-guest`; the native `os-host-full` feature includes it plus sync and ZIP.

The full Stdio plugin remains 36 artifacts. `plugin-root` selects `full-artifact-catalog`; catalog consumers continue to name or forward that full feature explicitly. Space selects `home-io`; the native OS host selects only `native-dwg-codec`; Hub and GIS retain their complete-catalog edges. If a full catalog host also reaches either narrow surface, Cargo feature union is safe and the full catalog wins.

## TDD and source verdict

The first registered Nx run failed at the missing `plugin-root -> full-artifact-catalog` feature edge. After the source split, the same command is green:

```text
bun nx run @semio-tech/stdio-plugin:home-io-surface-check --skip-nx-cache
stdio-home-io-surface-oracle: AJV=1 direct=4 shared=4 full=36 consumers=37
```

The follow-up native-host split was developed schema-first. Its first registered source run was RED on the absent `native-dwg-codec` feature. After the exact DWG outer gate and OS-host dependency were changed, the current gate is GREEN:

```text
stdio-home-io-surface-oracle: AJV=1 direct=4 shared=4 full=36 consumers=36
home-directory-identity-rows-check: checks=8 clean
```

The corpus rejects any non-DWG module admitting `native-dwg-codec`, rejects an OS-host full-catalog edge, and continues to require `plugin-root -> full-artifact-catalog`, all 36 artifacts, Space `home-io`, and the Hub/GIS complete-catalog selections. A resumed source inspection found that the crate's top-level `compile_error!` admission guard still accepted only Home I/O or the full catalog even though the DWG module admitted the new feature. The guard now accepts the exact three named surfaces, and the language-agnostic corpus pins that condition. The registered source gate remains GREEN after the repair: `AJV=1 direct=4 shared=4 full=36 consumers=36`.

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

The current Space component/descriptor producer is registered separately at launch order `411.081`, using the retained ticket-owned `home-space-component-sol-target` with one Cargo job and 24-hour command/build/orchestration budgets. Plugin-registry generation and the immediate freshness check are green after this entry and the concurrently landed worker-maintenance entries. Materialization session `50835` reached terminal exit `1` after `semio-framework-os-infinite` completed with warnings; Cargo returned `101`, but the retained terminal stream exposed no exact compiler diagnostic, signal or budget line and emitted no descriptor receipt. Under extreme host memory pressure this is not classified as a source failure. Registered warm reproduction `59241` compiled the current Space WASIp2 component in `18m 17s`, independently extracted a 61.3 MiB core module, and is active in the descriptor-tool stage; terminal hashes are still pending and are not inferred from intermediate files.

The older ticket-owned processes were pre-split snapshots and therefore could not qualify current source. With root coordination, Space session `45110` and Home session `80016` were cancelled; the verified orphan Cargo/rustc tree under the latter was terminated as well. Home session `68964` subsequently compiled the old `home-io + full-artifact-catalog` union for more than an hour, but became obsolete when the native DWG boundary landed. Its exact shell, Cargo and rustc tree was validated against `public-member-open-sol-target`, gracefully terminated, and is explicitly unqualified. Current Home session `90899` uses the same warm cache with `CARGO_BUILD_RUSTFLAGS=-Z threads=1`; its registered group now contains the plugin-host inference law, both real OS-host DWG round-trip laws and the three Home identity-row laws. The native verdict is pending.
