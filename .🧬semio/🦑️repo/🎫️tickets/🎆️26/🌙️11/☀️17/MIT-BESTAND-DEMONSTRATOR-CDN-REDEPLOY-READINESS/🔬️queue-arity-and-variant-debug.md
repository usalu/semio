# Queue arity and variant debug

## November 22, 16:45 PST continuation

The `JOBS=1` queue of `procedural` then `demonstrator` was left running. Both roots failed before a guest was staged. Disk stayed near 28 GiB free on the host and on `.cache/b/r/m`.

### Procedural

Attempt 1 failed because `scene_surface` gained a name and a description. Those call sites in the window kits were already updated before attempt 2.

Attempt 2 then failed while compiling:

- `semio-s-artifact-stdio-contract-window-kit`: `MeshWindowKit::render` takes a `Locale`.
- `semio-s-procedural-core-semantic-ui`: `TextWindowKit::render_read_only` takes a `Locale`.

The current sources already pass `locale` at both sites. A scan of production Rust under the framework, s, and hub trees found no remaining `WindowKit::render` call that omits `locale`. Attempt 2 compiled the files from before that edit. Procedural still needs one more `JOBS=1` queue after demonstrator releases the compiler.

### Demonstrator

Attempt 1 stopped in `semio-framework-value-native-owned`. A temporary `[DEBUG]` print in `value/decode/owned/value/variant` matched `NativeRecordKey` without that type in scope. The same error was reported twice. The print was removed. The following statement, which reads the tag and builds the record matcher, is unchanged.

Attempt 2 is rebuilding the descriptor emitter, then the demonstrator root, against that source.

### Shard worker

The newest file in the plugin TypeScript package is `script.ts` from November 9. The staged `shard-worker.js` is from November 20 and is 43,087 bytes. No restage is required unless that package changes again.

### Next

When demonstrator stages, queue `procedural` alone with `JOBS=1`. When both roots are staged, build the site with `MEM=4096 bash native-wasm.sh site`, run `verify-static.sh` for all 16 boots, then package.


## Requeue after 17:10 PST

Demonstrator attempt 2 compiled 1,466 units and then failed in four viewers. Each `render` parameter was named `_view_state` while the body read `view_state.locale`. The parameter was renamed in those viewers and in seven more viewers with the same split, so the next compile does not stop on the next crate.

`JOBS=1 ./native-queue.sh demonstrator procedural` is running. The descriptor emitter completed. The demonstrator component crate is the remaining compiler job. It is over the admission budget, so the broker is frozen and is not starting another unit beside it. Host and rust-image free space were restored to about 19 GiB by removing the stale transpile and optimize stage caches (about eight hours old). The staged browser modules were left in place.

## Demonstrator receipt after the successful link

The demonstrator release linked and staged (`1473` units, `0` failures, run `wZPuGlB`). `release-receipts.py record` then rejected it: `expected one optimized browser core proof for semio_hub_demonstrator_component, found 0`. The stage cache from that build is present. Transpile schema is 3 and adds a `maps` tool digest. Optimize schema is 2 and still names `wasm-opt`. The checker only admitted transpile schemas 1 and 2 and optimize schema 1, so the new chain was invisible.

`matchesTranspileKey` now admits schema 3 when `maps` is a 64-hex digest and the other transpile tools stay exact. `admittedOptimizeSchema` admits optimize schemas 1 and 2 when `wasm-opt` is a non-empty string. The sqlite oracle in `test-browser-core-lineage.ts` covers both. Recording the already staged module then wrote `🗑️generated/confirmed/demonstrator.json`. The retry that had started another full demonstrator compile was stopped so it would not replace that artifact. `JOBS=1 ./native-queue.sh procedural` is the remaining root.
