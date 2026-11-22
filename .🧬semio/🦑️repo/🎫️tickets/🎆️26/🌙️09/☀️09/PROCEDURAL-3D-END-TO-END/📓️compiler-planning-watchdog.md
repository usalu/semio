# Cargo planning and the compiler idle watchdog

The canonical Generation3d WGPU launcher admitted every source unit, then exited during the Flow Dictionary unit-graph command. Its stderr reported only Cargo's package-cache wait. The child ended with SIGKILL and no browser route started.

The driver previously counted Cargo's unit-graph reads among the execution children used by the compiler-idle watchdog. A slow unit-graph child has no Rust unit or component stage while it waits. Its process therefore met the same condition as a stalled compiler. The active graph captures now remain in the driver's owned cancellation roster and are excluded from the compiler-idle decision. Compiler execution receives a fresh activity timestamp when its child starts. If a driver failure terminates a graph child, the graph operation preserves the driver's reason.

The focused watchdog fixtures reproduce the previous false refusal and pass all seven observations with the corrected policy. The existing wait, localized retry, and progress checks also pass. Attempts to drive the larger isolated component test manually stopped earlier at the repository's authentic compiler-ingress boundary, before reaching the watchdog. The mounted WGPU verification is therefore still open.

The original WGPU launcher is terminal. The corrected launch is queued behind the currently active React boot publication so both renderers can use the same compiled component set. If React exits first, the queued launcher starts from the current checkout and prepares the WGPU boot components itself. The original WGPU browser watcher is still attached to the exact Generation3d route.
