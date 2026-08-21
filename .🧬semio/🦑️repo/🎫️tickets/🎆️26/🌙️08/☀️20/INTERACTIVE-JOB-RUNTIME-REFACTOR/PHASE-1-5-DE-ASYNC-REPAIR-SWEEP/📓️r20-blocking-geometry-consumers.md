# R20 Blocking Geometry Consumer Closure

## Outcome

Obsolete synchronous executor bridges were removed from every currently reached Flow, CAD, and Process B-Rep consumer identified by the interactivity audit. The kernel interface is synchronous, so these wrappers did not represent suspension and only hid work behind `block_on`.

## Changed consumers

- Flow B-Rep and Draw extensions call the synchronous kernel directly.
- Process3D inference/replay calls primitives, transforms, booleans, tessellation, disposal, kind, and volume directly.
- CAD interaction, geometry import, general I/O, and inference paths call the synchronous kernel directly.
- Non-suspending CAD mesh/OBJ bridge helpers were made synchronous, matching all existing direct call sites.
- The neural engine no longer creates or uses a Rayon pool; deterministic topological component evaluation is sequential until it is partitioned onto the shared worker lanes.

## Verification

- Targeted source census is zero for `block_on` across all changed Flow/CAD/Process files.
- The workspace interactivity audit fell from 161 findings to 57, with blocking bridges reduced to 15. None of the remaining 15 are in these consumers.
- `@semio-tech/flow-extension-brep-rust` and `@semio-tech/flow-extension-draw-rust` quick gates both reached their shared direct dependency and stopped in `semio-s-plugin-stdio` with 899 pre-existing de-async errors before compiling either Flow extension. No Flow pass claim is made.
- CAD and Process gates are deferred for the same direct stdio blocker; their edits remain compiler-gated, not reported as passing.

## Remaining owner boundary

The non-PDF stdio repair worker owns the 899-error dependency wall. Re-run the Flow, CAD, Process, and neural quick gates immediately after stdio reaches zero.
