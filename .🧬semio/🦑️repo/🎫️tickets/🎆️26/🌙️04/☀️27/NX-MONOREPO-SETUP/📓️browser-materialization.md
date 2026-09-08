# Browser Materialization Producers

Every authored Cargo component now has separate `materialize-dev` and `materialize-release` targets. Nx orders the corresponding staged WASI component producer and browser support producer first; dependency output bytes participate in the materialization hash. A component exclusively owns its authored module-directory name below the browser package’s `dist/<profile>/🔌️plugin-modules` tree. The support producer owns only Preview2 shims and the shared shard worker.

The descriptor assembly/hash implementation is shared with the development router. The new materializer writes fresh descriptor bytes and JSON directly beside its bridge; it does not modify source-owner descriptors or extension installation state. It stages output only after transpilation and a real JSPI descriptor probe succeed. Cancellation propagates to bounded-output JCO/optimizer subprocesses and removes temporary materialization files. Release optimization uses the installed locked Binaryen executable explicitly.

The language-neutral graph contract failed before these targets existed and passed after implementation. Both `support-dev` and `support-release` ran successfully through Nx. The Note materialization target is currently waiting on its component prerequisite; no materialization/restoration runtime pass is claimed yet.

The existing development catalog scheduler and module directory are still active. Replacing their preparation, observer and consumers with these profile-specific outputs is the next integration step. Font bytes, variant-specific session and engine closure, native runtime descriptors, Hub publication and extension notifications still require that integration.

## Verified Support and Descriptor Behavior

After the native socket correction, `@semio-tech/plugin-registry:generate` successfully regenerated editor entries for all 118 materialization targets and both support profiles. The public repository graph contract passed again. The focused existing Vitest suite passed both the complete actor-export contract and byte-identical descriptor finalization against the native descriptor fixture (2 passed, 102 unselected). Browser support restoration passed for both deleted directories with identical hashes/modes, followed by an independent Node import of the restored Preview2 IO interfaces; see [support restoration](📓️browser-support-artifacts.md).

## Actual Note Producer

The canonical-socket Note materialization run completed successfully: WASI component staging, browser support, JCO transpilation, Node descriptor extraction, and atomic module publication all completed through Nx (`note-materialization-native-socket.log`). The combined deletion/restoration and independent Node/WIT consumer probe is running.
