# WASI Ownership Lint Validation

The replication and UI contract libraries were checked with Clippy for wasm32-wasip2 and -D warnings.

{
  "status": 101,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T04:56:07.969Z",
  "finishedAt": "2026-09-09T04:58:22.612Z",
  "counts": {
    "unused_qualifications": 4,
    "unfulfilled_lint_expectations": 2
  }
}

## Verified Target-Width Repairs

The first strict WASI run reported four unnecessary size_of qualifications and two unfulfilled large-enum expectations in the causal DAG. Removed exactly those qualifications and limited the two enum expectations to 64-bit targets. The insertion-result expectation remains unconditional because WASI fulfilled it. Both files parsed before guarded writes. Recompilation is pending.

- 🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs
- 🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs
# WASI Ownership Lint Validation

The replication and UI contract libraries were checked with Clippy for wasm32-wasip2 and -D warnings.

{
  "status": 101,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T04:59:18.727Z",
  "finishedAt": "2026-09-09T04:59:31.676Z",
  "counts": {
    "unfulfilled_lint_expectations": 1
  }
}

## Fixed List Target Width

Wasm869 checked replication without diagnostics and reached the UI contract. The fixed-list branch expectation is unfulfilled on WASI, so it now applies only to 64-bit targets, preserving its fixed-allocation layout on both widths. The source parsed before the guarded write. Final focused recompilation is pending.

- 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🦀️.rs
# WASI Ownership Lint Validation

The replication, UI contract, and UI component libraries were checked with Clippy for wasm32-wasip2 and -D warnings.

{
  "status": 101,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T05:00:23.691Z",
  "finishedAt": "2026-09-09T05:01:31.576Z",
  "counts": {
    "unfulfilled_lint_expectations": 7
  }
}

## Preparation Refusal Target Width

Wasm871 checked replication and UI contract without diagnostics, then reported seven unfulfilled result-size expectations for preparation refusals. All seven are now conditional on 64-bit pointer width, as supported by the native and WASI compiler diagnostics. Other refusal annotations remain unchanged because their expectations were fulfilled. The file parsed before its guarded write; recompilation is pending.

- 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️prepared.rs
# WASI Ownership Lint Validation

The replication, UI contract, and UI component libraries were checked with Clippy for wasm32-wasip2 and -D warnings.

{
  "status": 0,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T05:02:18.374Z",
  "finishedAt": "2026-09-09T05:02:38.669Z",
  "counts": {}
}

## Final Focused Verification

Wasm874 finished successfully with zero compiler diagnostics and zero Cargo warning lines. This covers the replication and UI contract libraries plus the UI library with its wgpu component feature on wasm32-wasip2. It does not yet certify the complete plugin fleet, native linking, browser renderer or runtime tests. The compilation preserved repository Rust flags and used the ticket-local target directory with incremental compilation disabled.

[
  {
    "name": "wasm864",
    "args": [
      "clippy",
      "-p",
      "semio-framework-replication",
      "-p",
      "semio-framework-ui-contract",
      "--lib",
      "--target",
      "wasm32-wasip2",
      "--message-format=json",
      "--keep-going",
      "-j2",
      "--",
      "-D",
      "warnings"
    ],
    "status": 101,
    "signal": null,
    "reason": "exit",
    "startedAt": "2026-09-09T04:56:07.969Z",
    "finishedAt": "2026-09-09T04:58:22.612Z",
    "counts": {
      "unused_qualifications": 4,
      "unfulfilled_lint_expectations": 2
    },
    "cargoWarnings": []
  },
  {
    "name": "wasm869",
    "args": [
      "clippy",
      "-p",
      "semio-framework-replication",
      "-p",
      "semio-framework-ui-contract",
      "--lib",
      "--target",
      "wasm32-wasip2",
      "--message-format=json",
      "--keep-going",
      "-j2",
      "--",
      "-D",
      "warnings"
    ],
    "status": 101,
    "signal": null,
    "reason": "exit",
    "startedAt": "2026-09-09T04:59:18.727Z",
    "finishedAt": "2026-09-09T04:59:31.676Z",
    "counts": {
      "unfulfilled_lint_expectations": 1
    },
    "cargoWarnings": []
  },
  {
    "name": "wasm871",
    "args": [
      "clippy",
      "-p",
      "semio-framework-replication",
      "-p",
      "semio-framework-ui-contract",
      "-p",
      "semio-framework-ui",
      "--features",
      "semio-framework-ui/wgpu",
      "--lib",
      "--target",
      "wasm32-wasip2",
      "--message-format=json",
      "--keep-going",
      "-j2",
      "--",
      "-D",
      "warnings"
    ],
    "status": 101,
    "signal": null,
    "reason": "exit",
    "startedAt": "2026-09-09T05:00:23.691Z",
    "finishedAt": "2026-09-09T05:01:31.576Z",
    "counts": {
      "unfulfilled_lint_expectations": 7
    },
    "cargoWarnings": []
  },
  {
    "name": "wasm874",
    "args": [
      "clippy",
      "-p",
      "semio-framework-replication",
      "-p",
      "semio-framework-ui-contract",
      "-p",
      "semio-framework-ui",
      "--features",
      "semio-framework-ui/wgpu",
      "--lib",
      "--target",
      "wasm32-wasip2",
      "--message-format=json",
      "--keep-going",
      "-j2",
      "--",
      "-D",
      "warnings"
    ],
    "status": 0,
    "signal": null,
    "reason": "exit",
    "startedAt": "2026-09-09T05:02:18.374Z",
    "finishedAt": "2026-09-09T05:02:38.669Z",
    "counts": {},
    "cargoWarnings": []
  }
]
# WASI Ownership Lint Validation

The replication, UI contract, UI component, and framework action-bus libraries were checked with Clippy for wasm32-wasip2 and -D warnings.

{
  "status": 0,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T05:08:05.536Z",
  "finishedAt": "2026-09-09T05:08:37.200Z",
  "counts": {}
}

## Action Bus WASI Verification

The expanded focused WASI check passed without compiler diagnostics or Cargo warning lines, including semio-framework’s action bus. Exact evidence:

{
  "name": "wasm881",
  "args": [
    "clippy",
    "-p",
    "semio-framework-replication",
    "-p",
    "semio-framework-ui-contract",
    "-p",
    "semio-framework-ui",
    "-p",
    "semio-framework",
    "--features",
    "semio-framework-ui/wgpu",
    "--lib",
    "--target",
    "wasm32-wasip2",
    "--message-format=json",
    "--keep-going",
    "-j2",
    "--",
    "-D",
    "warnings"
  ],
  "status": 0,
  "signal": null,
  "reason": "exit",
  "startedAt": "2026-09-09T05:08:05.536Z",
  "finishedAt": "2026-09-09T05:08:37.200Z",
  "counts": {},
  "cargoWarnings": []
}
