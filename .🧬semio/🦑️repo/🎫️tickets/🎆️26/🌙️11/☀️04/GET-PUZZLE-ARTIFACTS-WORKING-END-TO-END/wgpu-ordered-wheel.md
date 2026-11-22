# WGPU Browser Wheel Ordering

The browser frame transport queues each wheel sample as an ordered event. Each sample retains its cursor position, horizontal and vertical deltas, event timestamp, and modifier snapshot. A wheel seals the preceding pointer and resize coalescing segment, so the same pointer can keep one move on either side of a wheel. Worker acknowledgements release the next FIFO page.

Each sample spends the existing bounded ordered ingress item and byte credits. A wheel also spends one of the 18 wire credits that the worker decoder shares with pointer moves and resizes. Each outgoing page stays within the 4 KiB limit. Exhausted ingress uses the existing ordered-input policy, which refuses the sample and retires the browser transport with `lossless-overflow`.

The browser worker validates each page before dispatching its events sequentially. The Rust wire decoder copies each wheel into one `DispatchEvent::Scroll`, and the native event queue admits it as a discrete event. Native Winit already captures the pointer and modifier state in each wheel callback. The 2D board applies each wheel with its own local pointer coordinates, and the 3D world queue folds pointer moves only.

The ordered-scroll fixture first reproduced a single merged wheel with the last sample’s modifiers. It now exercises the production transport with jsdom events, and checks FIFO at a worker acknowledgement and refusal at ingress capacity. The owned wheel application fixture goes through the canvas adapter and transport to verify same-point repeats, opposite directions, travelling anchors, deltas, and modifiers.

The four focused Node/Vitest suites passed 133 tests, including the owned wheel cases through the canvas projection and transport. The focused strict TypeScript check and scoped diff whitespace check passed. Verification covered the browser transport and worker wire contract without live Chromium rendering.
