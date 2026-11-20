# Current-checkout constrained lab audit

No 8 GiB / four-core acceptance run was started. This audit exercised the actual lab plan, machine-state reader, host endpoint selection, and the macOS socket port without running the lab or its readiness stages.

## Ticket-owned paths

| Path or operation | Observed on this checkout |
| --- | --- |
| Ticket's generated directory | 118 UTF-16 units, 144 UTF-8 bytes |
| Lab work directory below it | 129 UTF-16 units, above the lab's limit of 40; also nested in the source checkout |
| Machine state below that work directory | 134 UTF-16 units, above the machine-state protocol's limit of 120 |
| Shorter of the two host socket endpoint spellings below that work directory | 186 UTF-8 bytes, above the host protocol's limit of 96 |

The lab refuses an in-checkout work directory and uses a separate external `/tmp/sl-…` runtime by design. The machine-state reader returns no declaration for the planned ticket path. If a run skipped that check, the repository path mapper would use the live checkout's state. The production host endpoint constructor returned the typed `runtime-path` refusal for the ticket runtime. Independent Python socket binding failed with `AF_UNIX path too long` for an absolute 165-byte pathname. A short relative socket succeeded only after changing the isolated test process's working directory to the containing directory. The repository's host clients, daemon and stages use canonical checkout working directories and a shared absolute endpoint, so that single-process test does not provide a lab transport.

The generated directory's host volume had about 51.4 GiB free when inspected. The lab's existing 40 GiB preflight was not the blocker at that time.

## Source containment

The machine-state override routes `.cache` and `.nx`. The full readiness path still runs `dependencies sync --all`, whose producer publishes `node_modules` bindings at workspace source endpoints. This can replace live checkout links with an isolated lab generation. The lab's own Nx setup also uses the checkout's `.nx` lookup. The current `.nx` is a real directory, so the lab's link helper would retain it on the live source. These effects cannot be contained by assigning the temporary directory or by merely relaxing the lab work-directory limit. The `workspace`-only plan replaces the other four readiness stages with `repo --help` and cannot establish full bootstrap acceptance.

The supported full lab uses a per-machine writable frozen checkout and its own Git index, with a short external runtime. The requested run excludes an external scratch runtime and recursive source copying. The existing harness also refuses Windows explicitly. Supporting the requested layout requires an isolated writable source view or separate dependency-binding owner, an admissible long state path throughout native tooling, and a local host transport that can rendezvous under ticket ownership without requiring that long absolute Unix socket. No partial override was applied.

R197's earlier frozen-tree test reached readiness at 8 GiB / four cores while exceeding its footprint limit (3,531 MiB against 3,276 MiB). It is not evidence of a passing constrained verdict for the current code.

A follow-up probe of the local Docker CLI through the repo tool interface found no running daemon at the configured Unix socket. No local container runtime was available for an isolated 8 GiB / four-core alternative at the time of the check.

## Change detection under a cache ancestor

The current lab test reproduced a false zero in source-change detection. Absolute find patterns excluded a complete checkout whenever its parent directory was named `.cache`, including the scratch root supplied by the repository tool. The scanner now prunes excluded directories by their location inside the scanned checkout, reads native modification timestamps and propagates marker, traversal and ignore-reader errors. It keeps the existing time and output bounds. A language-neutral case and independent Python filesystem walk verified newer, older, equal-timestamp, explicitly ignored and generated paths. Both the lab and frozen-checkout suites passed, 33 tests and 186 assertions.

## Ticket-owned sparse volume feasibility

A ticket-backed sparse APFS volume can provide a short independent namespace. The current lab cannot keep its complete runtime on that volume or bound its backing growth against a concurrent live graph read. This check was read-only. No image was created, no volume was mounted, no source files were copied, and no daemon or lab run was started.

### Ticket ownership and source paths

- Git `check-ignore -v --no-index` returned the root `.gitignore` rule `🗑️generated` for both a proposed sparse image and a proposed sparsebundle band. I passed the same paths to the replica's real `compileIgnoreV1` and `sharedV1` using the current root ignore text. The generated directory, image and band each returned `ignored=true, shared=false`; this retained audit returned `ignored=false, shared=true`. The replica loads that same matcher at startup.
- Use a real mount outside the checkout. An alias resolving back into the ticket would still fail the canonical `--work` containment check. A proposed mount at `/private/tmp/slv` leaves the automatically named `slw-XXXXXX` directory at 27 characters and the machine state at 32, below the limits of 40 and 120. The backing file would be the only new path under the ignored ticket directory. A tracked source path would be addressed through the mount rather than nested below the ticket directory. The frozen view still contains recursively copied tracked source bytes inside its backing image. The initial cross-volume freeze cannot use APFS copy-on-write cloning with the live checkout; the subsequent clone within the image can. The tracked non-ticket blobs at HEAD sum to 1.81 GiB across 167,944 paths. That is a reference size only; the current live modifications, nonignored additions, metadata and writable stage outputs also consume space.

### Runtime and live bindings

- `sliceLabPlansV1` puts machine root, state, temporary directory, home, cache, registries and the fleet store below its work directory. It independently derives `XDG_RUNTIME_DIR` as `/tmp/sl-<id><index>` and strips inherited values. The lab creates that directory and uses it for host sockets, logs and spilled graph slices. The options schema has no runtime root, and `--out` only redirects copied reports. The external short socket location works with the existing design. Moving it onto the image needs an explicit, short canonical runtime root and the existing 96-byte endpoint check.
- Use `--frozen` for dependency binding isolation. Its private checkout and index keep source-owned `node_modules` and `.nx` changes off the live checkout. `linkAmbientV1` still links eligible live `node_modules` entries and `.cache/b/d/m/j`, and recreates ignored live symlinks. These are live references without mount-enforced read-only custody. The harness measures live mutations; the mount by itself does not prohibit writing through those references. `--frozen=<dir>` can reuse an earlier snapshot and therefore does not by itself establish that current code was read.

### Physical admission

- Native filesystem counters measured 926.35 GiB total and 50.22–51.15 GiB available on the live checkout and ticket backing device. The authoritative CAS policy at that capacity derives a 1.5 GiB start cushion and a 12 GiB background recovery threshold. Replica preflight separately requires 10 GiB free on the worktree volume. Preserving the 12 GiB recovery threshold leaves 38.22–39.15 GiB of potential physical growth before accounting for live writes and image overhead. Preserving the 10 GiB replica startup floor leaves 40.22–41.15 GiB before those costs. Neither is an allocation reservation.
- The slice lab requires 40 GiB free at `--work` and performs the check on that device before freezing. With a sparse image this reads the mounted filesystem's free blocks and does not account for the live checkout's remaining space. A larger nominal image would satisfy that check without provisioning the physical bytes. The measured host capacity does not establish a safe 40 GiB physical budget alongside a live read and its recovery headroom. No current high-water bound exists for this cold frozen run.
- The machine declaration sets a 3,276 MiB process-tree verdict for 8,192 MiB, a 1,536 MiB graph booking floor and a 2,048 MiB minimum graph booking. It uses private admission and lease directories. Those settings do not reserve physical RAM or four CPU cores against jobs already running on the 32 GiB host. During these read-only probes ordinary tool admission reported a 4,096 MiB physical floor and available memory falling below 8 GiB with raised pressure, then recovering above 14 GiB. No sustained host grant for a second readiness read was established.

The current full-stage entry and flags are shown below as a reference for the inspected path. This command retains the external runtime and the ambient bindings above. The work and report paths refer to a proposed mount only.

```sh
repo tools exec bun -- "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/📜️script.ts" run \
  --machines=1 --memory=8192 --cores=4 --stages=all --frozen \
  --work=/private/tmp/slv --out=/private/tmp/slv/report
```

## Runtime containment

Each machine now uses `<work>/m<N>/u` as its private runtime. Socket paths, host logs and spilled slices therefore use the same volume as the lab's work directory. The canonical work-directory limit remains 40 characters. Before freezing the source or provisioning any machine, the lab invokes the production endpoint selector with the account's real UID. It accepts the regular spelling or the compact spelling when the complete socket path fits the 96-byte budget, and reports the existing bilingual refusal otherwise. Activity sampling and report export read the selected host log directory, including the compact spelling.

Cleanup removes the directory the lab created only if its device and inode still match. A normal invocation with a deliberately inadmissible work path refused before provisioning, removed its generated child, and preserved the caller's sibling sentinel. A path-only probe using the proposed short mount produced four distinct endpoint paths of 67 bytes each.

The language-neutral fixture covers regular selection, compact selection at 96 bytes, refusal at 97 bytes even when the OS can address that path, and a path larger than the OS limit. Python's Unix-socket binding API independently checked the non-existent path fixtures without creating socket files. The lab, frozen checkout, home and host identity suites passed on macOS with 67 tests and 484 assertions. No image or full lab was started. The same full-stage command above now keeps the runtime on its work volume. The physical resource reservation and ambient bindings remain separate acceptance constraints.

At 18:24 UTC the coordinator reported 39.50 GiB free on the ticket backing volume, below the lab’s 40 GiB preflight even before preserving the host recovery reserve. The normal capacity CLI supports a sustained request with `repo capacity run --memory=8192 --cpu=4`. At this observation it reported 5,903 MiB of admission headroom and room for 2.4 CPU cores while the whole read held its grant. No lab or image allocation was submitted.

## Incomplete frozen inputs

A regression reproduced the lab publishing a completed receipt after failing to copy a source file into the snapshot. A second regression reproduced inclusion of an unrelated file left in an unfinished named snapshot. Materialization now propagates filesystem errors, including allocation failures, and creates files exclusively. An entry already absent at the initial source inspection remains a legitimate concurrent deletion. A named freeze rejects an unfinished nonempty destination, prepares new destinations in a same-volume private directory and publishes the receipt only after the source tree is in place. A Python filesystem oracle independently verified regular-file and symlink collision cases. The local slice-lab and frozen-checkout suites passed 36 tests and 218 assertions. The current 8 GiB acceptance run remains outstanding.

## Dependency bootstrap isolation

A new fixture reproduced the full bootstrap inheriting three writable live dependency bindings. A lab run that provisions dependencies now always uses a private checkout, starting without inherited root modules, ignored workspace package links or the live JavaScript generation cache. Both the freeze and the per-machine clone enforce this choice. A named snapshot records whether it inherited dependency bindings and refuses reuse for a different mode. Python inspected the actual machine and frozen filesystems and confirmed the three bindings were absent while a source-owned relative link remained readable. Workspace-only runs can still use ambient dependencies. The combined suites passed 37 tests and 224 assertions. At 18:46 UTC the physical backing volume had 42.15 GiB free, leaving 30.15 GiB once the 12 GiB recovery threshold is preserved. That remains below the lab’s 40 GiB requirement.

A two-machine edit regression also reproduced cleanup overwriting a file replaced by the build. The lab now appends and restores against the retained descriptor of the private source file and checks its stamp before truncation. The isolated lab and frozen-checkout suites passed 38 cases and 227 assertions.

The Go launcher compiler previously placed its build module, temporary files, compile cache and GOPATH on the host scratch volume. A Python compiler probe observed all four outside the lab. The launcher now places these on the machine volume and removes its private job directory after either exit. A genuine Go compilation and the full home suite passed 27 tests and 245 assertions.

The full lab also continued with a seeded launcher when its own compilation failed. Two red cases reproduced that fallback. A missing compiler, missing source, failed compilation or failed macOS signature now stops admission before the machine starts. A successful build installs the verified local output by rename. Each machine compiles its own checkout, including any requested edit before launch. The combined home, frozen and lab suites passed 65 tests and 472 assertions, including a real Go launcher build and execution.

At 20:40 UTC, the coordinator’s ordinary `repo cache status` ledger reported 39.91 GiB in use, 56.46 GiB on disk, a 198.31 GiB machine budget and zero bytes to evict. Filesystem free space was about 40 GiB. The difference between occupied cache content and on-disk allocation does not establish that any particular blocks can be reclaimed while the coordinator holds its graph grant. The ledger check exited zero and left the running graph in custody.
