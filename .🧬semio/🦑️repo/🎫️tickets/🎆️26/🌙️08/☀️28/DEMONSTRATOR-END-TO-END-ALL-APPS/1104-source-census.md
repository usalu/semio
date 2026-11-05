# Source Capture Census

`1104-source-census.mjs` calls the original kernel selector, original Source reader and original `captureNxAuthorizerSourcesV1` with `installed:true`. The registered Node process passively observes `fs/promises.open`, returns each original handle unchanged, records its real stat size, and restores the builtin. It first verifies that the actual Source reader emits exactly one observation. It copies no source and changes no capacity, policy, capture implementation or import scope.

The actual Node capture stopped at the unchanged 16 MiB guard with **1,822 original files and 16,780,794 bytes**. This is supplemental exploration: the build runs under Bun, whose scanner produced **1,788 files and 16,780,740 bytes** in the real failed commands. The census explicitly does not replace the actual declared commands.

A Bun invocation correctly refused the observer self-check because Bun's original named builtin binding did not reflect the patched default export. Its zero-row output is not capture evidence. The original capture was not called after that self-check failure.

The captured import graph establishes the shortest path from the actual runtime command kernel through `build/caller/child` to the new Storybook engine preparation owner, then root `.storybook` coordination. Coordination imports package inventory, canvas runtime assets, the OS dev engine module and the iframe styling module. Detailed names, real byte sizes and resolved edges are in disposable generated census JSON files. Runtime owns repair and actual-command validation.

Only one Rust file appeared in this actual observed capture: the registered Node native runtime addon (12,724 bytes). Flow Rust work is outside this census and shrinking its pump bodies would not address the observed TypeScript Source refusal.

Largest observed inputs include Bazel scope data (387,204 bytes), dependency storage source (293,383 bytes), filesystem descriptor source (206,336 bytes), and Bazel command source (162,591 bytes). These are observations, not a decision to remove any authority or executable dependency.
