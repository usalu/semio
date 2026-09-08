# Headless Stdio Metadata Capture

## Scope

This slice hardens the standalone Rust import probes used after the exact Stdio catalog build. It now has one successful captured-frontier native import result, but it does not claim guest execution, Hub activation, immutable whole-build provenance, or qualification of source changes made after that build was admitted.

## Retained metadata owner

`proveHeadlessStdioImports` still selects the sole `.rmeta` from the exact Cargo `compiler-artifact` row after binding the Stdio crate source and sorted `full-artifact-catalog` feature set. The selected file must now be a regular non-symlink direct child of the command's private `cargo-target/debug/deps` directory.

Before any import process starts, `captureHeadlessStdioMetadata` reads that file through `readStableBuildFile`, writes `headless-imports-*/libsemio_s_plugin_stdio.rmeta` with exclusive creation, and records `stdio.capture.json`. The retained `lib*` name is required by rustc's direct `--extern` artifact grammar; the initially proposed `stdio.rmeta` basename was rejected before metadata decoding. The receipt contains the crate source path, sorted features, original metadata path, captured path, sole dependency root, byte length and SHA-256. Both the retained read and the copy/hash observe cancellation and progress at each 64 KiB boundary; an interrupted copy closes its descriptor and removes only its incomplete private destination. The import command points `--extern semio_s_plugin_stdio` only at the captured file and supplies exactly one `-L dependency=...`, the original private `debug/deps` directory. The executable directory is no longer a resolver.

The successful private run captured 1,631,805,231 bytes. The stable capture therefore uses a fixed 2 GiB ceiling and one retained source read buffer. `readStableBuildFile` allocates that one exact-size buffer and reads it in 64 KiB chunks, invoking the injected cancellation/progress check before every chunk. The private copy is written and hashed from 64 KiB views into the same buffer; it does not reread the 1.52 GiB destination or allocate a second full-size buffer. The helper returns only its primitive receipt, so the source buffer becomes unreachable on return, and the caller requests a full Bun collection before starting import processes. This is a bounded one-buffer implementation, not a streaming-memory claim: its peak intentional metadata payload is the complete source file, up to 2 GiB, plus small chunk views and hash state.

## Command isolation

The full provider, `--stdio-only` provider and Hub native-catalog-selection launchers now have distinct ticket-generated artifact roots. Each Cargo target is the `cargo-target` child of its command's artifact root. Both Hub commands reject missing roots, workspace-relative roots, targets outside the artifact owner, and shared launch paths before Cargo admission.

## Deterministic evidence

The oracle-only registered provider gate is GREEN. Its capture law creates two isolated command roots, captures different metadata, replaces one live source path after capture and proves the private bytes/SHA remain unchanged. It also replaces a source path during the descriptor-stable read and observes refusal, rejects a second dependency resolver, and rejects an outside Cargo target.

The Hub native-catalog-selection oracle-only gate is GREEN with the same launch isolation check. Plugin registry generation and `check-generated` are GREEN, so `.vscode/launch.json` matches the seed.

## Prior E0463

The earlier `t6LWYf/00` run compiled Stdio and passed its first six native laws, then the old live-metadata import returned `E0463`. Its build completed at 08:37:27 and the failed import was written at 08:37:37. The currently visible unhashed `.rmeta` was replaced later at 08:40:34, so it is not the file image consumed by that failure. The old harness recorded neither an immutable pre-import digest nor a retained copy, and the current evidence cannot distinguish a transient live-target substitution from another metadata-consumption fault. Native positive/negative import laws remain unclaimed until the hardened private-copy harness runs.

Private run `QjQhhc/00` passed all six selected native Stdio ownership/catalog laws and the actual-payload AJV check, then reproduced the positive import refusal against captured SHA-256 `8c659345fdc4ce59da3f6327c02cbbb736dbf6d67c4af214a03de20682282633` (1,631,719,138 bytes). Its full diagnostic adds the missing discriminator: rustc rejected the copied `stdio.rmeta` location as an unknown extern type and required a `lib*` artifact filename before emitting the trailing E0463. This was a capture-name fault, not metadata-byte corruption.

## Captured-frontier native result

Warm private retry session `76497` is terminal GREEN. Its catalog run is `qkwXCJ/00`; all six exact Stdio native ownership/catalog laws and the actual-payload AJV check passed. The emitted native test executable is 131,678,752 bytes with receipt and independently recomputed SHA-256 `d337c22d3b3ab85d5cdba696bb7a3f2c78e98f2b4e40ae1a5373360a73c90aae`.

The import proof copied the sole full-catalog metadata artifact to `headless-imports-5EPpGB/libsemio_s_plugin_stdio.rmeta`. Its receipt and independent hash both report SHA-256 `46f4c91cf6443d947fed040133ea2b4f6fc62b133abbec2d399e1c434530395b` over exactly 1,631,805,231 bytes. The complete native-catalog import exited successfully with no diagnostic. The four forbidden guest-app/editor/plugin/viewer imports each exited with the harness-required single `E0432`; every import retained the same captured file identity. This proves five exact Rust metadata import outcomes through one immutable copy and one private dependency root; it does not execute a guest or mount a client.

The separate budget run is `4WeBQa/00`. Both `catalog_projection_budget_matches_serde_and_refuses_before_overdraw` and `catalog_projection_preflight_matches_actual_serde_payload_bytes` passed. Its 388,783,920-byte lib-test executable has receipt and independently recomputed SHA-256 `574bdba1aa24a6ed8af20b4d1ebe159c794f7e73201f896bd5ff5200995019e3`.

This result is deliberately source-frontier scoped. The command loaded its Stdio and framework inputs before the concurrent `ArtifactKindSpec` import/export ownership change from borrowed vectors to owned strings. The receipts remain authoritative for the admitted captured frontier and the hardened metadata-copy mechanism, but they do not qualify that later descriptor semantic change; a fresh build is required for the new source frontier.
