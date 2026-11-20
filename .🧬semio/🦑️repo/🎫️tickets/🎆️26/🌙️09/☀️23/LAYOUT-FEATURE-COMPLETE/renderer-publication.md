# WGPU Renderer Publication

Both renderer retries stopped at Rust error E0061 in `WindowConfigOwnerRegistry::advance`. The erased store owner required the retained turn ceiling between its one-item grant and receiving authority. The current shared source passes that value through. No source edit was needed for this failure.

The raw Cargo retry passed the source gate and started recompiling toolchain units in a different lane. It was cancelled before starting the canonical Nx renderer target. The canonical task validates the prepared wasm-bindgen version against Cargo.lock and checks for prepared Binaryen before it starts Cargo. It publishes and verifies the generated JavaScript and WebAssembly in the renderer products directory.

The canonical prototype retry runs with one worker and the existing prepared toolchain. Its initial capacity wait completed. The task entered the checkout graph queue.

At 620 seconds of graph waiting, the active scan had captured 173 of 593 selected inputs. The host service serializes slice loads for one checkout because they share a fact store. Machine capacity was also occupied, with 400 MiB of effective headroom in the preceding snapshot.

The ticket's `stage-layout-renderer.ts` now invokes the same renderer compiler owner using the repository's issued workspace-script providers. The owner validated wasm-bindgen 0.2.129, checked prepared Binaryen, and began its 822-unit Rust source admission. The queued Nx copy was then cancelled to leave one renderer producer active.

The first owned run admitted all 822 source units and captured a 1,104-unit Cargo compilation plan. Its mandatory recheck observed a changed plan and returned temporary status 75. The wrapper now repeats the same admitted invocation on that status, with a cancellable delay and at most six attempts.

The next run confirmed active edits in the admitted closure. Compared with its earlier passes, the fresh snapshot added 276 source bytes in `semio-os-dsl` and 1,544 in `semio-plugin-plugin-program`. The original retry wrapper imported a guard from an entrypoint that did not export it. The runner and its status-bearing error class now come from the same module, and a pinned Bun check verified the status 75 detection.

The corrected wrapper received and retried a live status 75 refusal. Its second child passed the plan rechecks and entered actual Cargo compilation.

When the wrapper's display stopped forwarding fresh compiler messages, the renderer's own Cargo lane log continued to grow. The current compiler passed `semio-plugin-window-config` and continued through the plugin ABI with 411 compile events and zero errors.

The earlier renderer audit also listed obsolete `InteractiveJob::step` implementations. The current frame, asset decoder, compile and task-ingress implementations use the framework's return claim parameter and `InteractiveJobStep` return type.
