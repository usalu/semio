# stdio Stabilization and Flow Recheck

## Snapshot

- The stdio dirty count was unchanged at 361 paths across two coordinator samples.
- The repository-wide dirty count changed from 545 to 551 during the same interval, so unrelated work remains active.
- No source, plugin registry, generator, or stdio path was edited by this recheck.

## Flow Integration Result

`bun nx run semio-framework-os-flow-core:test-quick --skip-nx-cache` now compiles the framework, Graph, 2D, 3D, UI, schema, compiler, plugin, and Infinite dependencies successfully. It reaches `semio-s-plugin-stdio` and fails on one missing generated/mounted source path:

```text
🛢️artifacts/🣊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️no-mutation/🦀️component.rs
```

The stale mount is `semio-s-plugin-stdio` Rust glue line 2259 (`pub mod no_mutation`). This is an externally owned stdio/glTF registrar inconsistency. There is no Flow-host or neural-DAG error. N-01 remains source-complete but integration-quarantined until the stdio registrar owner atomically removes or restores that mount according to its current semantic disposition.
