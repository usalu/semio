# PCG And Assembly Preservation — Repeat 695

Completed all eight delegated ranges against immutable B b84bf526c7ba26f99c4054b47da1c1a4d5064e3d, H 8f91299152ceab6c19df64c251f5ad23252ce18b, S 6e8eff657fb70675db7ad30b890f61b7a3e201d7. Existing split child owners, H public/sibling visibility and module mounts remain. Four live source files were written with fresh-byte guards and atomic replacement; no Git mutation was used.

## Hunk Dispositions

- **fem-pcg-assembly-58** (B 1120–1119): Retained authored blank-line empty insertion before Pcg region close in run-and-entry.
- **fem-pcg-assembly-59** (B 332–489): Removed seven obsolete PcgJob numerical method copies after relocating the whole authored math methods to PcgCheckpoint; job publication/close/constructor methods remain on PcgJob.
- **fem-pcg-assembly-60** (B 65–64): Preserved legitimate empty insertion of the full ten-method PcgCheckpoint impl in the existing job-and-steps child, with current sibling visibility.
- **fem-pcg-assembly-61** (B 1531–1551): Replaced old 18-field AssemblyCsrBuild declaration with assembly plus shared CsrTripletBuild owner. Existing S constructors/run/close methods already match this new defining receiver.
- **fem-pcg-assembly-62** (B 180–181): Changed AssemblyPlan free_new and compact_of_new to authored AssemblyOrder owners, retaining current field visibility.
- **fem-pcg-assembly-63** (B 178–178): Changed AssemblyPlan inv_perm to authored AssemblyOrder owner, retaining current field visibility.
- **fem-pcg-assembly-64** (B 157–158): Changed AssemblyCheckpoint merged_full and merged_free to authored SparseTriplet paged owners.
- **fem-pcg-assembly-65** (B 62–63): Changed AssemblyPartitionBuffer full and free to authored SparseTriplet paged owners.

## Receiver And Caller Evidence

PCG now has exactly one complete PcgCheckpoint implementation: new, reset_spmv, finish, step_diagonal, step_spmv, step_initial_residual, step_precondition, step_update, step_direction and advance_one. The incoming free pcg entry now advances mathematical checkpoint state directly. PcgJob retains current publication funding, scheduling, cancellation, outcome desk, mounted matrix transfer and original close controls. The removed seven job math copies are superseded by their authored checkpoint definitions, not dropped computations.

The H mounted RHS constructor and the H job close turn, granted close and InteractiveJob close bodies are retained. Incoming construction/restore close signatures use the same StepContext and OriginalTurnFailure; their close_granted helpers retain physical owner custody and exact gate/admission behavior. The completed child is closed through InteractiveJob before the outer construction/restore frontier is retired.

Assembly existing incoming function bodies already matched immutable S. The four stale defining declarations were repaired together: partition and checkpoint paged SparseTriplet fields, AssemblyPlan AssemblyOrder fields, and AssemblyCsrBuild shared builder fields. This aligns construction, merge, visual lookup, publication and close with their actual state owners. H mounted construction and original interactive close/granted close behavior remain. No duplicated CSR pipeline, compatibility owner or replacement shared API was introduced.

## Source Audit

- **56 PCG** and **70 Assembly** top-level incoming functions have exact S bodies at one actual receiver, allowing current visibility.
- **10 PCG** and **19 Assembly** incoming declarations match at one actual owner, allowing visibility and the optional final field comma.
- **10 Rust files** passed source-only balanced delimiter scanning with zero issues. Each source facade retains all **4** current module/test mounts.
- Final SHA-256 hashes for all ten files and per-function/receiver/declaration witnesses are recorded in generated/domain-pcg-assembly-result-repeat-695.json.
- No build, compiler, typechecker, tests, runtime, resolver or generator was run.

## Files Updated

- ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🪜️job-and-steps/🦀️.rs — 1dbc24be80e631de5af8f5e5dd87f0fa6bb1bd9f3d37d58eed4bb9b646eb2970
- ✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🏁️run-and-entry/🦀️.rs — 729a3e0ce893e6d68c88fac097dce947c2212136a2755252dd51a1bb2e30b72e
- ✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🧷️job-state-and-construction/🦀️.rs — c0c03d485314ac25e13b81202ca839f1b241c77909ec73a062b5219ec9f520ad
- ✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🔶️csr-build-and-run/🦀️.rs — cecfbe55b7331e5ecfa1323f6d952848a36761f7f1f5491322a38c57f984b6a8

## Authored Shared API Gap

Immutable S references SparseTriplet, CsrTripletBuild and AssemblyOrder but supplies no definitions or imports. Independent immutable S/current FEM source searches found no defining type, struct or enum. The source pass preserves these authored migration references and their consumer fields; it does not invent shared implementations. Domain confirmed this disposition.

The source subtrees are frozen for independent audit. Domain owns the shared relocation ledger update. Catalog hash refresh remains deferred until the parent freezes the whole fleet.
