# Current Native Gate Renewal — 2026-11-05

## Workspace Actual Compiler Failure

Full unfiltered workspace invocation 10411 reached actual compilation after source admission completed at 1122/1122. Its retained log workspace-one-job.log reports E0432 in Job outcome continuation source: semio_framework_job_payload did not export OriginalReturnStructure and require_return_structure. Cargo reports semio-framework-job-context failed to compile; no workspace suite pass is claimed.

At inspection, the current canonical payload package glue already explicitly reexports both names from its original_return_escrow module. The definitions exist in the canonical return-escrow owner. File modification time was 15:05:24 UTC. No duplicate source edit was made. This is current-source verification, not proof of the precise cause of the older captured compilation view.

A public process snapshot at 1793891354090 showed the owned wrapper chain 57828 → 51952 → 56980 and only cache-collector children, with no Cargo or rustc executable child. After the conclusive compiler failure, only this owned failed cleanup was canceled. Handle 10411 returned terminal exit 1. Subsequent unfiltered public snapshots at 1793891420902 and 1793891424381 found none of those three old PIDs and no matching holder, waiter, admission grant or admission waiter.

One replacement, handle 19262, runs the same full unfiltered workspace test scope through normal repo cargo with requested parallelism 1, prototype configuration and shared store. Its log is workspace-return-structure-current.log. Current compiler and test outcomes remain required.

## Component Actual Failure and Renewal

Component invocation 13885 returned terminal exit 1. Its retained component-canonical.log contains 134 Compiling lines and reached a kernel-identity host-side unit before the outer cargo:build timeout of 3591718 ms. The retained log reports that unit could not compile and wrapper exit 1, but no individual rustc diagnostic was retained. This is neither a WASM link success nor a proven specific current-source error.

Read-only audit observed that the printed host-side kernel-identity invocation has no native-owned extern. Canonical identity, kernel-base and media-jobs bindings have since been independently reviewed. The audit does not infer success from this source change.

One fresh ordinary invocation, handle 38749, runs the same component package, library and wasm32-wasip2 target with normal requested parallelism 1. It does not increase limits or bypass admission. Its log is component-current.log. Current native/WASM compilation, link, publication and host provenance remain unproven.
