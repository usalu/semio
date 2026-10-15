# Rust Cache Override Independent Audit

Read-only Source review, 2026-10-15. No tests, builds, Originals, image opens, cache deletion, or environment probes. Old unmanaged data remains untouched.

## Supported Cargo Ingress

No concrete external cache-directory escape established in the inspected supported Cargo routes.

- `.vscode/launch.json:4212` and `:4267` expose `bun repo cargo --store=… --configuration=… -- …`; launcher `library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:105` enters `withRepositoryCompilerStoreV1` before executing Cargo. Lines 138-140 pass selected bounded directories through dependency/input admission before resolution or compilation.
- `library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:39` replaces incoming `CARGO_HOME`, `CARGO_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`, `TMPDIR`, `TMP`, and `TEMP` with issued compiler-store paths. Line 44 replaces incoming `RUSTUP_HOME` and toolchain path with the retained tooling generation, disables automatic installation, and removes trace/dist-root overrides.
- `library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:34-35` requires bounded Cargo home and temporary paths and admits arguments against the actual build and target directories before dependency execution.
- `compiler/🚪️ingress/📥️arguments/🟦️.ts:10-11` accepts `--target-dir` only when equal to the issued target directory. It accepts `--config` only for the exact authored config path or exact issued build/target assignment; other values refuse. The protocol also refuses `--artifact-dir`, `--lockfile-path`, and `--root`.
- `library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts:22-25` replaces test/lint policy directories with actual store paths and routes compiler arguments through the same dependency admission. `library/🏃️process/🟦️.ts:81-89` requires relay Cargo/home/temp paths under actual backing and checks the policy build root before spawning Cargo.
- `library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts:24` routes supported dependency Cargo commands through bounded preparation and argument admission. Preparation owner `library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts:27-33` opens the compiler store first and passes its bounded environment to the child.

## Raw Environment Resolution Is Not Enforcing Custody

`library/⚡️caching/🦀️cargo/🗂️directories/🟦️.ts:19-26` deliberately resolves incoming environment overrides without authenticating backing. It is a path resolver, not proof of fixed storage. The inspected supported compiler/test/launcher callers replace those paths before execution. Other resolver consumers include maintenance, provenance, and cleanup; this audit does not certify every consumer as an enforcing compiler entry.

`.cargo/config.toml:8-9` points raw Cargo at fixed-image mount paths, but static configuration does not mount or authenticate an image. An unsupported manually launched Cargo command with overridden environment/config can select arbitrary directories; that is distinct from an escape through the inspected supported launch/Nx Cargo execution facades. No direct Cargo command or Cargo environment override appeared in the inspected editor settings. Editor generic Cargo arguments still reach argument admission.

## Limits

This review follows canonical Cargo launcher, test/lint, relay, dependency preparation, and editor Cargo ingress. It does not establish behavior of every generated Nx target or arbitrary custom recipe, nor process runtime behavior. Explicit generated Source publication such as `SEMIO_TYPEGEN_OUT` is a separate authored output operation, not treated as compiler-cache storage in this audit.
