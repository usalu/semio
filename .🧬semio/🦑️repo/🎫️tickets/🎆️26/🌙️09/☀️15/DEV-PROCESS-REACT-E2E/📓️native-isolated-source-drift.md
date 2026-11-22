# Isolated WGPU Cargo source drift

The first isolated run (PID 34321) started compiling at 17:43 UTC on 2026-11-21 and exited 101 at 18:58:43 UTC. Its log records one `semio-framework-artifact-reference` compile at line 33601. At line 86463 `semio-plugin-runtime-ops-document` fails to import `MostGeneralDialect` and `most_general_dialect` from that crate. The earlier shared-store `addr2line` error did not recur.

The live document component imports the names directly. The artifact-reference crate root includes `🌳️selection/🦀️.rs` and re-exports both public declarations. The inspected UTC mtimes were 18:44:04.459 for the caller, 18:44:04.887 for the crate root and 18:54:02.526 for the selection module. The worktree diff shows the imports and exports were introduced after the test began. The current sources agree, and the observed sequence supports a stale dependency during the failed run. Repo retired the original isolated store `ea63220d70d9`, taking that invocation’s metadata with it.

## Retry and checksum evidence

The only post-error activity at log line 86515 was `cargo:workspace-selection`. Both workspace selection and its final retained-source recheck publish that activity. The streaming Cargo path returns numeric exit status to its retaining callback, and `withCargoWorkspaceSelectionOnceV1` runs its final recheck before returning the status. No second source-admission sequence or Cargo compile started. This is consistent with ordinary retirement.

The held-retry message is emitted only after the failed unit's dependency probe produces a byte-drift witness and graph/source readmission succeeds. The message is absent. The preserved log does not report which gate declined the retry. It cannot distinguish a failed witness probe from a readmission that did not complete.

The independent editor build’s dependency directory in `.cache/b/r/m/u/l/p2e20ae` contained:

```text
prototype/build/semio-framework-artifact-reference/c52de6186db7f27b/out/semio_framework_artifact_reference-c52de6186db7f27b.d
```

Its first checksum row (line 17) is BLAKE3 `dc634a3916c69fd8ca36abb10f1d4e0f92b844437555be5c7d93a027ff1cd7c6`, length 1944, for the absolute crate root source path. Other sampled paths are absolute as well. This is the exact sibling filename `failedCompilerArtifactDriftV1` derives from an absolute `libsemio_framework_artifact_reference-c52de6186db7f27b.rmeta` under the supplied build root. The guard against relative checksum paths would not skip that row. The editor build provides an independent example of the same crate and configured layout. The original isolated artifact is unavailable, so the evidence does not justify changing the lookup layout or guessing a base for relative checksums.

No source patch or build command was started by this investigation. The parent started isolated retry PID 39223 after PID 34321 exited.

The isolated native WGPU check reached the Host WIT crate at 20:24 UTC and exited 101. It reported that `semio_framework::kernel::PluginDocumentTarget` was missing and `Effect::OpenPluginInstance` had no `document` field. The live framework facade independently omitted the target from its explicit kernel exports. The facade now re-exports it. The live defining kernel has the optional `document` field and its source modification timestamp is 213 ms later than the terminal log. This establishes that the currently observed kernel source postdates the failure. The exact earlier artifact supplying the old field list is unconfirmed. WIT, host and reactor conversions already carry the three target fields, and the native effect round-trip and canonical JSON fixture already cover them. The focused editor pane run was started after the isolated run released its compiler.
