# WGPU Single-Enqueue R17 Compile RED

Actual canonical mixed route exited1 before executing any tests. OS-kernel library compilation failed with four diagnostics: unresolved semio_framework_async::TokioHostRuntime; Result<Option<os_store::component::Backbones>,os_vcs::VcsError> is not a future; two future-cannot-be-sent-between-threads errors. The summary reports31warnings. The selected WGPU interlock did not execute, so this is not its intended semantic RED. Vitest was not reached. No all-three metrics, timing, funding or observer success is claimed.

Command: NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/framework-renderer-wgpu:test --skip-nx-cache --args='exhaustive --lib runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-wgpu-single-enqueue-r17-2026-08-28.md'

The sole native process finished; relevant source holds were released immediately to parent and Dag. No second compiler or rerun was launched. The unfinished native resident package is excluded by the separately preserved52-package Cargo workspace dependency superset.

## Source Capture Boundaries

The broad capture was truncated and is explicitly unusable as a complete snapshot. The bounded capture received5037records out of5088enumerated paths; one interval returned49records. A fresh actual100-record post-dispatch supplement is separately labeled, not recovered historical bytes. Neither capture is claimed atomic or a full compiler-input closure. Independent resident source edits and taxonomy metadata changes remain separately attributed.

Files:
- 📓️wgpu-single-enqueue-r17-dependency-graph-2026-08-28.md
- 📓️wgpu-single-enqueue-r17-selected-inputs-2026-08-28.md (truncated attempt)
- 📓️wgpu-single-enqueue-r17-bounded-inputs-2026-08-28.md
- 📓️wgpu-single-enqueue-r17-post-dispatch-input-supplement-2026-08-28.md
- 🧪️member-wgpu-single-enqueue-r17-2026-08-28.md

## Actual Tool-Captured Output

The canonical runner exposed only the four-line compiler summary below. Full rendered errors/warnings were subsequently found in the existing same-target OS-kernel fingerprint output and copied exactly to 📓️wgpu-single-enqueue-r17-retained-compiler-diagnostics-2026-08-28.md; no compiler rerun or unknown-line reconstruction occurred.

Exact spans: directory/client native import481 names TokioHostRuntime from async instead of the compiler-suggested services module; Store sync900 awaits detach_backbone's actual Result; Store sync2245's native ActorTurnFuture is not Send because it awaits codec.compile_dsl at1263 and codec.print_mirror at1277, whose erased futures lack Send. These are observed compiler diagnostics, not approval to add flags/defaults or implement compiler suggestions mechanically.

The actual runner source invokes cargo nextest list --list-type binaries-only --message-format json --profile exhaustive -p semio-framework-os-renderer-wgpu --lib before execution. No --features override was requested. The WGPU manifest explicitly requests OS-kernel sync+ureq on native targets; OS-kernel defaults include deflate. The adjacent fingerprint feature metadata below predates this failed run (its modification time is retained), so it is corroborating existing metadata, not a freshly captured rustc command line.

```json
{
  "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/.fingerprint/semio-framework-os-kernel-06c8650a35eec8f9/lib-semio_framework_os_kernel.json",
  "modified": "2026-08-27T22:52:09.589Z",
  "metadata": {
    "rustc": 4695965181093895000,
    "features": "[\"default\", \"deflate\"]",
    "declared_features": "[\"default\", \"deflate\", \"dsl-fixture-sweep-full\", \"sync\", \"typegen\", \"ureq\", \"worker\"]",
    "target": 700504429530532200,
    "profile": 4140219120151045600,
    "path": 8684535670941020000,
    "deps": [
      [
        3037468093297214500,
        "dsl_derive",
        false,
        2291114749915057400
      ],
      [
        4093251733041600000,
        "futures",
        false,
        16627485829494774000
      ],
      [
        5575064383624564000,
        "semio_framework_hash",
        false,
        9630645668301750000
      ],
      [
        7636735136738807000,
        "miniz_oxide",
        false,
        15099855897846102000
      ],
      [
        8186484761278817000,
        "protocol",
        false,
        5980492470420364000
      ],
      [
        8278633811172106000,
        "semio_framework_async",
        false,
        1781228011845358300
      ],
      [
        8663351094627808000,
        "semio_framework_job",
        false,
        1873906383439457500
      ],
      [
        9394460649638302000,
        "tokio",
        false,
        8513174167642443000
      ],
      [
        13077212702700853000,
        "base64",
        false,
        5059888305089044000
      ],
      [
        13548984313718624000,
        "serde",
        false,
        12449259616051732000
      ],
      [
        13665985940634835000,
        "blake3",
        false,
        5544025388776664000
      ],
      [
        13795362694956884000,
        "serde_json",
        false,
        13496340835856677000
      ],
      [
        17543102014077323000,
        "pack",
        false,
        12368874898661257000
      ],
      [
        18372475104564267000,
        "zip",
        false,
        17127037824260502000
      ]
    ],
    "local": [
      {
        "CheckDepInfo": {
          "dep_info": "debug/.fingerprint/semio-framework-os-kernel-06c8650a35eec8f9/dep-lib-semio_framework_os_kernel",
          "checksum": false
        }
      }
    ],
    "rustflags": [
      "-Z",
      "threads=8"
    ],
    "config": 8362119626465529000,
    "compile_kind": 0
  }
}
```

```text
> nx run @semio-tech/framework-renderer-wgpu:test --args=exhaustive --lib runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation --no-fail-fast -- --nocapture

> bun ./📜️script.ts test exhaustive --lib runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation --no-fail-fast -- --nocapture

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

error[E0432]: unresolved import `semio_framework_async::TokioHostRuntime`
error[E0277]: `Result<std::option::Option<os_store::component::Backbones>, os_vcs::VcsError>` is not a future
error: future cannot be sent between threads safely
error: future cannot be sent between threads safely
error: could not compile `semio-framework-os-kernel` (lib) due to 4 previous errors; 31 warnings emittedWarning: command "bun ./📜️script.ts test exhaustive --lib runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation --no-fail-fast -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-renderer-wgpu failed

Failed tasks:

- @semio-tech/framework-renderer-wgpu:test

Hint: run the command with --verbose for more details.
```
