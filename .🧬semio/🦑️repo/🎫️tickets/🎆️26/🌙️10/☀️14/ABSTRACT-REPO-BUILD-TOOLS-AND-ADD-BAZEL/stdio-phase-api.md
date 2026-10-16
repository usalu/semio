# Stdio Cargo Phase API Correction

The read-only public readiness audit found a concrete current API mismatch in `stdioCargoMetadata`: the tools-store argument was a `NativeCallerPhaseV1` descriptor rather than the issued control, and the Rust-toolchain call omitted the newly mandatory control argument. This was a Source finding; no failed Cargo execution was inferred.

The owner now passes the genuine bounded issued control to the tools store, its returned tools phase to the Rust toolchain, and the returned Cargo phase to actual command capture. Phase checks surround awaited progress, generation rechecks and capture. Existing native authority, child budget, signal alias, timer cleanup, transport limits and Original remain unchanged.

The exact retained `tooling-closures-propagation-current.json` contains 375 role entries (194 distinct paths). All six changed/new Stdio paths have zero intersection. This is an exact retained-role membership check, not a claim that a past backend capture or SDK receipt proves current loaded bytes. No backend capture, SDK/native/store operation or Cargo process was run here.

The schema-first two-case corpus covers live phase threading and cancellation after the actual generation recheck before command capture. The law executes the actual TypeScript function body through memory ports; SQLite independently provides phase order and Ajv independently validates the same portable corpus. It is invoked by existing `verifyStdioCommandOwnership`, reached through permanent Stdio package ownership/graph routes. Memory ports are test-only and confer no runtime authority.

- RED: `🗑️generated/stdio-phase-red.log`, actual assertion that tools-store received the native descriptor instead of the issued entry control.
- GREEN: `🗑️generated/stdio-phase-green.log`, exit 0, both cases.
- Existing full Source ownership: `🗑️generated/stdio-phase-ownership-current.log`, exit 0, nine owners, two routers, 36 artifacts, five commands.
- First strict attempt: `🗑️generated/stdio-phase-typescript-current.log`, one adjacent existing TS2345 at the pin-negative `assert.throws` call. Corrected by its supported message overload; no assertion removed.
- Final strict: `🗑️generated/stdio-phase-typescript-final.log`, session42323 terminal0, three owned roots plus actual transitive imports, 6.793 seconds / 53.207 seconds Original60 remaining.

Low independently reviewed the exact phase chain and retained signal/timer scope clean. Actual sealed Cargo execution and public selected-alias qualification remain pending Root release and an accepted current ALL pin where required.
