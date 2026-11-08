STATUS: UNVERIFIED — lock/sync done (marker present) but every run is refused pre-compiler by the job-context source budget; no publication/close proof yet

## Runs
- W-runtimeb-102/103/104 (aussuchen smoke), logs `🗑️generated/1105b/W-runtimeb-10{2,3,4}.txt`: rc 1, "Rust crate Source refused before compiler: semio-framework-job-context" (maxSourceBytes 262144; the #[path] closure is ~274 KB per runtime; runtime's folder split only shrank ⏱️context to 255,829 B). Owned by the job-context split lane, not me.

## Static work
- Generator 1.6 MB close release: root (Generation3dPreviewCommandWork dead inline fields) fixed by runtime, 952 B (W-runtime-g3d2). Remaining single-step >1 MiB release = boxed FlowSourceRead in Generation3dSessionCommandWork (proc/flow; claims 20:25); needs FlowSourceRead/ReadSources to page its own release. Not mine per runtime.
- Settlement idle bound (artifact-app-laws settlement :257): added TEMP `[DEBUG] rb-gate settle-idle` eprintln of typed_operation_diagnostic + close_diagnostic; REMOVE after the first run that shows it.
- Inherited unproven fix: ChildContentReadAliases strong_count rule + law (aussuchen Retiring stall).

## Next
When job-context is admitted: rerun smoke.sh (W-runtimeb-105), read settle-idle diagnostic, then statik (1105b-tools/statik/smoke7.sh).
