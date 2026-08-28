# Workflow Actual Source 34 Independent Review

This is a read-only review of `🧪️workflow-actual-source-34`; no target execution or production
change was made.

## Observed Mounts

The client mounts the current Workflow component by its canonical file and the controller walks
both Workflow and Run mutation collections, including Rust and JSON files. The two required
complete-variant codec names exist in the current component at the expected source functions.
The controller's expected total is explicitly 47, matching the root-provided 22 component, 23
leaf and 2 aggregate categories; this review does not independently claim a compiled roster.

## Blocking Harness Defects

1. `listing.output.split("\n").filter(line => line.endsWith(": test"))` does not trim CRLF.
   The retained immutable-registry client demonstrated that Bun-captured rust test listings can
   retain `\r`; this expression then counts zero tests despite a valid listing. Normalize each
   line with `trimEnd()` before the suffix check.

2. The artifact map accepts rlib paths and fingerprints a sibling rmeta when present, but rustc
   receives only each rlib `--extern`. The paired rmeta is neither validated before creating the
   run nor passed alongside its rlib. A missing rmeta can therefore throw during the initial
   fingerprint after a run directory is created without a retained result, and a mismatched rmeta
   is not a compiler input. Require each rlib/rmeta pair before the run and pass both same-hash
   formats in the one rustc command, as the independent registry and Mini gates do.

## Boundary Note

The external client aliases `semio_framework_os_kernel` as `protocol` and `store`; it does not
link a separately supplied protocol/store artifact. That may be intentional public-facade testing,
but it is not a direct dependency-identity check for Workflow's normal crate graph. Keep that
limitation explicit if the controller remains facade-bound rather than adding a claim of full
crate-graph equivalence.
