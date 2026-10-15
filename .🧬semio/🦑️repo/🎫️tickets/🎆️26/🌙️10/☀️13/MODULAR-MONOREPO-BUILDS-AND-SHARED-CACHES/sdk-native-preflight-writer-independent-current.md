# SDK Native Preflight Writer Independent Audit

Source-only review on 2026-10-15. No mounted-image reads, runtime/probes, builds, or tests. Root's copy-only runtime was not touched.

## Compiler and Preflight Roles

No concrete execution of an Nx native-copy loader was established during the inspected compiler/preflight roles.

`tools/🔏️authorizers/🟦️.ts:11` lists `nx/src/native` as an allowed external intrinsic. This permits capture and compilation without bundling that module; it is not itself a runtime import. `authorizers/📦️snapshot/🟦️.ts:23-26` resolves external modules as external in Bun.build and only compiles admitted Source. The compiler child in `authorizers/📜️script.ts:19-20` compiles and writes the resulting program; it does not import or execute that program.

The runtime entry reexports SDK functions. In `caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts:19`, the actual `require("nx/src/native")` is inside the exported `selectedGraphSdkPortsV1` function, alongside other Nx requires. The planner requires in `selection/📥️scope/🔌️sdk/🟦️.ts:6` are also inside its function. The worker native require in `membership/📥️journal/👷️worker/📜️script.ts:54` is inside the worker runner; its final top-level conditional only starts it for an actual filesystem-journal worker. The supported preflight child is a main Node process.

`authorizers/📥️loading/🟦️.ts:14` imports the generated module then checks that named exports are functions. It does not call the SDK functions or start a journal worker. Therefore an allowed external string alone is insufficient evidence of a native loader copy during this exact preflight. No top-level direct Nx module import was found in the inspected captured Source areas.

## Patch and Binary Protocol Limits

The authored patch `caching/🩹️patches/nx@23.2.0.patch` changes native database/workspace storage callers, but no `NX_NATIVE_FILE_CACHE_DIRECTORY` or native-loader copy-cache override was found there. The new SDK child environment policy likewise does not control that variable. This is not evidence that the variable executes during preflight.

`sdk/🧊️binary/📥️publication/🟦️.ts:19` owns immutable binary-object publication and inode binding. It does not intercept subsequent package-loader copyFile calls. Binary sharing must not be cited as proving that an independently executing Nx native loader cannot create another native-cache copy. The actual installed Nx native-loader implementation is outside this Source-only inspection, so its default path, copy behavior, and any later-role byte peak remain unestablished here.

The inspected evidence supports keeping the exact compiler/preflight writer closure unchanged on this concern; it does not qualify all later Nx execution roles or arbitrary dynamic imports.
