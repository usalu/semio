# Spatial session source admission

A release lane observed 262,176 bytes against the 262,144-byte source limit for the spatial session crate. Concurrent source changes moved code from that crate to its lower geometry owners before the focused audit.

The current production admission function accepted the actual spatial-session Cargo entry under the pinned compiler's wasm32-wasip2 cfg. The compiler-visible closure has 252,921 bytes, 12 files and 3,274 active lines, against the unchanged limits of 262,144 bytes, 32 files and 4,000 lines. The byte margin is 9,223. This check made no source changes and started no compiler build.
