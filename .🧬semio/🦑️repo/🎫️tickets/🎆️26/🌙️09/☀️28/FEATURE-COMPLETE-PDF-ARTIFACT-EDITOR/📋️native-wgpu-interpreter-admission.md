# Native WGPU Interpreter Admission

The first governed WGPU ProgramBridge and Shell link stopped before Rustc because its upstream core interpreter exceeded the 262,144-byte per-crate source boundary. The live tree then added portable prepared-image wire ownership, bringing the measured source closure to 20 files, 4,151 active lines and 251,973 bytes on the subsequently observed checkout. It still exceeded the active-line limit.

A shared workspace peer installed a separate dependency-free `semio-plugin-interpreter-numeric` owner at the existing numeric-semantics path. That owner now contains the public scalar values, traps, and numerical execution. The core interpreter remains the instruction and instance owner and re-exports those exact public types. The installed core library measured 22 files, 3,525 active lines and 252,229 source bytes across Linux, macOS, Windows and WASI. The numeric package measured three files, 652 lines and 27,355 source bytes across the same targets. The core's aggregate unit-test closure remains over the independent test limits. The WGPU production build uses the admitted library closure and has passed source admission.

The independent WebAssembly oracle ran ten authored scalar cases. The repository's pinned Rust compiler linked the same cases directly against the live numeric source. Signed and unsigned division, traps, wrapping addition, ties-to-even rounding, IEEE signed zero, NaN truncation and float bitcasts all matched. A separately staged cut was abandoned when the equivalent peer source appeared in the live tree.

- [Neutral scalar fixture](🔣️interpreter-scalar-ledger.json)
- [Independent WebAssembly oracle](📜️interpreter-scalar-oracle.mjs)
- [Linked Rust scalar tests](📜️interpreter-scalar-law.rs)
- [Current governed WGPU link](🗑️generated/native-wgpu-save/wgpu-link-open.log)
