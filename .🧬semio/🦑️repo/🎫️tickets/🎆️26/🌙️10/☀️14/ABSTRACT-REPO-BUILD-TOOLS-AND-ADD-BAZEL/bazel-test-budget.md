# Bazel Permanent Test Budget And Registration

Read-only inspection; no production, launch asset or task registration change and no runtime started. Receipt: `generated/bazel-registration-current.json`.

## Exact Current Chain

The real target is `workspace:test-build-tool-bazel`, not `test-build-bazel`. Root `📋️project.json:3532` declares `repo:script`, cache false, no outputs and command `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts test bazel`.

The English and German launch entries in both actual launch and seed invoke `bun repo run workspace:test-build-tool-bazel --exclude-dependencies --skip-cache`. Neither entry authors SEMIO_SCRIPT_POLICY. The target does not author a policy either. `runBuildCliV1` preserves an incoming policy/deadline exactly; absent an incoming wire and explicit policy, its current default is `{version:1,owner:"repository:build",maximumElapsedMilliseconds:0}` (no Original deadline). It is inaccurate to call the actual launch's Original five minutes. A genuine120 or300 incoming caller instead retains that exact finite deadline.

`TestScript.run` passes the complete Bun test file to one `runOwnedCommand` with `scriptInvocationBudget(this.invocation,300000)`. Thus the existing single child has a hard five-minute ceiling, intersected with the current Original remaining; this is not a fresh Original. `scriptInvocationBudget` returns min(requested,currentremaining) for finite Originals and the requested bound for the zero-policy case.

Nine runtime cases plus seven Source laws and the installed Nx parity case are permanently registered in that file. The actual cache case alone measured85.47425seconds (whole selected file87.45seconds), with four real Bazel runs. Eight runtime cases and installed parity remain unmeasured on the split fixture. Neither the total sum nor a permanent public target timeout has been measured; an impossibility claim would be premature. The earlier monolith did exhaust the unchanged120 gate, but that is not proof of a300 target failure.

## Recommended Boundary If The Complete Child Cannot Fit

Keep each runtime case an explicit domain operation. Extend the existing owner route to `test bazel source`, `test bazel parity`, and `test bazel runtime <literal-phase>`, with the phase read from the closed language-neutral fixture's nine names. Unknown/multiple phases refuse before spawning. The owner passes a closed filter/environment to the existing file, retains genuine Original/wire/signal/publisher, and intersects each child with the existing300 ceiling and (for runtime cases) its existing120 bound. It does not construct a new envelope or reset a deadline between cases.

Register literal `workspace:test-build-tool-bazel-source`, `...-parity`, and `...-runtime-cache|nested|uncached|birth|services|refusals|cancellation|configuration|offline` in the existing root project. Each calls only the same owner script route; the assets remain hand-authored and schema-first. Add matching English/German launch entries next to the current Bazel gate entries, in current grouping/order. Independent phase launches are separate user operations, not a fabricated aggregate pass. Tests verify every registered phase occurs exactly once, each command's argv reaches the actual owner, no assertion case is dropped, and original controls are unchanged; use actual schema/Ajv plus independent argv/SQLite coverage.

For a whole-operation aggregate, use a neutral declared task graph (no executable copied to a ticket): the aggregate depends on those literal targets, each target keeps its own bounded child, and the scheduler retains the SAME incoming Original across all task admissions/settlement. Never issue a new policy/envelope per phase or silently retry a timed-out phase under another clock. Serialize heavy Bazel cases unless measured physical admission and independent workspaces justify concurrency. The aggregate must publish terminal completion of all children, not count a scheduled/skipped child as completed.

An aggregate cannot be promised to finish within a finite inherited deadline if the measured serial sum exceeds that deadline. In that case it must refuse/cancel under that SAME Original. Independently selected phase receipts establish the full matrix only as multiple explicitly separate operations; they do not claim a single300 aggregate success. Do not raise the300 child ceiling, increase an existing caller policy, or replace the current full target with a subset while retaining its end-to-end name. Before changing the aggregate registration, measure all phases and attempt the current public target under its actual admitted policy; if it already fits, retain its simpler single-child route.

## Qualification Work Still Required

Current Source is held for SDK closure/setup. After SDK success, run the current installed Nx adapter first. Run each remaining Bazel case under its existing genuine120, retaining exact call counts/time and cleanup. The later unique fixture root should also receive a fresh cache case if repeatability is claimed. A full registered target receipt remains distinct from ticket-selected case receipts. No Source or asset change is justified by an unmeasured guessed total.

## Authorized Implementation

Root subsequently authorized the permanent no-arg owner to loop the closed cases under one unchanged Original, each120-second child, preserving Source/parity coverage. This batch is retained in `bazel-phase-dispatch.md`; the earlier read-only conditional plan above is historical design evidence. Actual full aggregate runtime remains pending.
