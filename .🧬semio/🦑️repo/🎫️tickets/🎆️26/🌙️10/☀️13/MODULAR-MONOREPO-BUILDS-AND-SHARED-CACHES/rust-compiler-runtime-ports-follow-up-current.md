# Compiler Runtime Ports Follow-Up

The compiler entry still fails its actual256fileSource capture after the sealed-code reader/publication split. The bounded diagnostic is evidence of refusal, not a complete graph or a successful compiler build. Do not enlarge the limit or retry an unchanged failed Original.

Current inspected call path:

1. The unit Script and direct rustc port import the Rustup installation owner through compiler-store setup and compiler identity lookup.
2. The installation owner imports rustupTomlValueV1 from installation/🔤️toml. That adapter returns Bun.TOML.parse on Bun, but its runtime module also imports prepareBun for Node callers.
3. The Node fallback preserves original tooling/store/reservation checks, reads the pinned Bun recipe, provisions the exact parser, writes bounded temporary input, and invokes a controlled child. Removing this path without a real replacement would break cold Node setup.
4. Source scanning already excludes type-only imports. The scanner captures runtime imports before any bundle qualification, so the fallback provider is included even for the actual Bun compiler entry.
5. Twelve production compiler-store call sites were found in the library, including Rust graph preparation, direct unit compilation, Cargo admission/exact/launcher, native artifacts, runtime verification and wasm policy. A runtime-port change must cover actual Node and Bun callers and preserve zero-touch provisioning and complete generation custody.

Next architectural candidate: separate runtime parsing/provider implementations behind an explicit first-party port and pass the selected implementation through the actual composition roots. Alternatively, separate authenticated installed-generation reading from online provisioning while preserving the real parent cold-bootstrap path. Neither is implemented or accepted by this note. Do not use a callback, copied digest or synthetic provider as a substitute for private Native custody. Do not infer bundle import.meta resource behavior from Source inspection.

Native/default SDK/publication/compiler/cache-image/OS-inventory holds remain. The three workers remain terminal after application network permission was revoked; no restart or alternate route is authorized. Repository ticket MCP is unavailable in current tool metadata. The goal and ticket remain active.
