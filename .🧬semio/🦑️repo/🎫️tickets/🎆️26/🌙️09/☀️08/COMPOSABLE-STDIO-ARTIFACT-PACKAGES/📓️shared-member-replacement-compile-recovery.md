# Shared Member Replacement Compile Recovery

The consolidated Semio retry compiled a pre-current `semio-framework-plugin` snapshot and stopped before tests. Its newest fingerprint receipt, `🗑️generated/semio-full-recovery-3-compiler-diagnostics.json`, contains seven cascaded `E0277` errors for `M::Open: Send`, one `E0107` at the replacement admission target, and one `E0004` for six newly introduced replacement states.

A current-source verification at 2026-09-09 15:55 Europe/Berlin found all three causes already settled by concurrent work:

- `MemberFactory::Open` requires `MemberOpenOperation<Member = Self> + Send`.
- `try_begin_artifact_store_replacement` constructs `ArtifactStoreReplacementAdmissionTarget::<A, M>`.
- `poll_artifact_store_replacement` maps initialization, member opening, closure validation, view preparation, and candidate-ready states to `Pending`; rejected and retirement states map to `Progress`; terminal committed, cancelled, and fault states remain distinct.

The plugin and Store roots shared the same 15:55:25 source mtime when checked. Their scoped `git diff --check` passed. No duplicate source edit or formatter invocation was made during this verification. Native acceptance is still pending a fresh compile against this source; the failed Semio receipt proves only the older compiler snapshot.

A separate read-only source audit accepted the current ownership boundaries: the opened member operation is transferable to its worker, the admission target carries both the artifact and member types through the replacement envelope, and every nonterminal state preserves pending or retirement progress rather than being mistaken for completion. The audit also confirmed that the schema reexport used by the PDF feature is visible. This remains source evidence only until a current native route reaches terminal success.

The current native compile boundary is now accepted. Ordinary Nx PDF build receipt `🗑️generated/pdf-native-cache-35-qualified-owner-recovery-local.txt` compiled `semio-framework-plugin`, stdio contract, Binary, and PDF against the settled source, finished the Cargo development profile in 51 minutes, and exited all four Nx tasks with status 0. Its structured receipt is `🗑️generated/pdf-native-cache-35-qualified-owner-recovery-local-run.json` with build hash `3349577625346025899`. This clears the nine stale Semio compiler diagnostics; it does not substitute for artifact-specific runtime laws.
