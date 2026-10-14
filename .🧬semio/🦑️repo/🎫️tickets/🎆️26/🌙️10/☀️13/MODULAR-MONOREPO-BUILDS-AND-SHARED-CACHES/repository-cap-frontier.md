# Repository Physical Bound Frontier

## Current Qualified Envelope

The authored generated-storage maximum is64GiB. Its seven declared partition ceilings sum66,639,101,952 bytes; control reserves1,048,576 bytes and filesystem headroom2,079,326,208 bytes. These sum68,719,476,736 bytes. Only actual macOS fixed filesystem runtime is qualified. Linux loop/ext4 and Windows fixed VHD providers have typed neutral contracts and fail closed, but their actual operating-system runtime remains pending.

This is a ceiling for admitted managed writers. It is not a ceiling for the checkout's authored inputs, Git database, retained old ordinary trees, arbitrary raw tool overrides or external shell writers. A complete checkout assertion requires those additional classes to be settled or explicitly included. A quota declaration or a newly invented deletion lease cannot establish old physical custody.

At the current checkpoint, direct lstat observations of the seven backing files and their image receipts measure66,631,762,802 logical bytes and58,044,592,128 allocated bytes. Mounted descendants are excluded from that sum, avoiding counting their backing twice. Actual allocation may grow within each fixed file/volume limit; this observation is not a smaller permanent ceiling. Control's current du observation is16KiB. The ordinary control namespace has bounded individual issued writers, not a separate quota filesystem.

## Measured Retained Baseline

These are current allocated observations, not deletion authority. Directory totals below are not all disjoint; the Framework/Hub/extensions rows deliberately include their own descendants.

| Physical Tree | Allocated Bytes | Qualification |
| --- | ---: | --- |
| `.git` |15,316,520,960| Authored repository history; outside generated-cache envelope |
| Git-listed regular input inodes |4,150,345,728| Current tracked input observation;3,737,629,893 logical bytes,153,252 unique regular inodes;552 tracked paths currently absent |
| `.cache/held-javascript` |4,674,502,656| Retained unqualified original module data; no reader deletion |
| Old `.🧬semio/🦑️repo/⚡️cache/tools` |26,155,294,720| Old SDK/install data remains outside new tools8GiB |
| Old `.nx` |23,508,721,664| Old workspace/cache data; installed-tool links do not prove all old reads ended |
| Old Cargo preparation cache |7,176,192| Prior ordinary preparation snapshots/queues; selected new native producer preparation is now inside its actual compiler store |
| Current ticket generated output |133,140,480| Ordinary redirected tool logs and prior artifacts; active ticket remains open |
| Framework whole tree |10,597,326,848| Mixed authored inputs and older generated descendants |
| Hub whole tree |11,594,575,872| Mixed authored inputs and older generated descendants |
| Extensions whole tree |4,438,069,248| Mixed authored inputs and older generated descendants |

Large identified descendants include Framework `🤖️generated/process/cargo-build/debug/build` at818,776KiB; Hub `node_modules` at1,535,900KiB; extensions `node_modules` at1,535,784KiB; Hub Rust `🗑️generated/test-artifacts` at1,419,756KiB; and Hub GIS Rust `dist` at882,640KiB. They need their own original custody classification. A whole-tree size cannot be relabeled as authored Source merely because the path starts inside Framework or Hub.

The read-only Git input observation uses `git ls-files -z` and lstat, deduplicating device/inode pairs. Git's filesystem-monitor daemon termination was printed while that read completed; the method does not include newly authored untracked inputs or promise a stable future input size. No Git mutation was requested or performed.

## Remaining Write Ingress

1. Default raw Cargo compilation now refuses at its actual compiler version probe before target/build creation. Managed `withRepositoryCompilerStoreV1` rechecks an actually issued live reservation before projecting the real compiler and clearing the default wrapper. This does not constrain an explicitly overridden raw `RUSTC`, `RUSTC_WRAPPER`, `CARGO_BUILD_RUSTC`, `CARGO_BUILD_RUSTC_WRAPPER`, a raw `--config` compiler selection, or a different raw target/build path. Such launches are unqualified.
2. Cargo registry-only `fetch`, `update` and lock operations can write without entering a compiler probe. Qualified repository acquisition forces bounded Cargo home/TMP and retains original registry custody. Raw registry commands and arbitrary source replacement/configuration remain unqualified.
3. The generic `repositoryCargoPreparationStorageV1(root)` still authors an ordinary prior preparation directory. The selected native artifact producer and runtime verifier now override it inside held compiler storage and use owned asynchronous preparation. Other direct/synchronous preparation consumers need the same whole-lifetime admission, including general repository process/wasm/dependency-direction paths. Preparation's existing neutral port also conflates scratch/receipt storage and queued lease directory; separating those actual roles must preserve the original operation rather than fake a facade.
4. `repoTestArtifactEnvironment` can directly mkdir an arbitrary `SEMIO_TEST_ARTIFACT_DIR` or an ordinary fallback. The generic repository test owner has authentic diagnostics scopes and retention, but direct caching Source test routes/private ticket routes and this separate helper are not automatically qualified by that owner. External shell redirection of agent logs is likewise outside a retained repository-store callback.
5. Arbitrary command producers can still write relative output, their own cache options or ambient external directories. Tools/products/dependencies environment projection confines the managed known writers; it is not an operating-system sandbox for arbitrary authored subprocesses.
6. Existing ordinary Hub/extensions module roots, Rust output/dist/test trees and old build/cache snapshots remain prior data. The new Source binding publication does not grant retirement over these unrelated roots.

These gaps must not be hidden by environment-only path checks, by summing mounted trees, or by claiming that an inactive-looking PID proves no reader remains.

## Original Custody for Bounded Retirement

First, declare each actual generated physical owner: exact data root, device/inode identity, immutable generation/receipt and partition class. Observed prior data is recorded as unqualified baseline until its producer and readers supply original custody. Classification alone gives no deletion permission.

Every future managed reader must acquire a genuine generation loan before resolving modules or opening artifacts, keep that loan through child completion and lazy consumption, and release it only after awaited close. Image SH protects the backing lifetime; a distinct generation SH protects the exact data that retention may unlink. Fixed sharded SQLite resources avoid unbounded lease metadata while directory/input identities remain exact and immutable.

Every future producer must hold genuine writer custody through staging, publication and terminal child settlement. It records actual publication and original close events. A settled producer is established by its owned handle/exited/stream-close callbacks, not by scanning or killing a foreign process.

Retirement then obtains exact generation EX, rechecks immutable receipt and physical root identity under that custody, performs the actual removal, and acknowledges resulting physical bytes before releasing authority. Retiring an allocation in a journal without removing its actual bytes is insufficient. A process crash releases its real kernel loan; catalog EX and closed metadata headroom allow safe recovery without PID-age inference or ABA adoption.

Old consumers must first be routed through that real protocol or settle under their already-held original owner. The protocol cannot backdate missing loans. Until all relevant old readers/writers settle, those roots remain charged as retained baseline and the whole-checkout bound remains unqualified. Preserve their original bytes and live SDK readers; do not invent a compatibility reader or a deletion lease that none of them observed.

## Coherent Maximum Policy

Keep the generated64GiB contract exact and fail closed at its authentic providers. Separately publish authored input/Git observations and the still-retained ordinary baseline, with disjoint owner classifications where available. The currently supportable expression is:

`checkout physical bytes = managed generated backings + authored input/Git bytes + retained/unqualified generated roots + other uncontrolled writes`

The first term has a declared finite ceiling. The remaining terms do not yet have one. If a hard maximum for every checkout byte is required, authored input/Git writes and every ingress must also be governed by an actual finite enclosing filesystem or separate finite providers. Replacing a live shared checkout with such a filesystem cannot safely be inferred from current cache leases: it requires the actual Source/Git/coeditor custody and settled old physical owners. A measured baseline plus64GiB is an observation formula, not a perpetual whole-repository ceiling.

No old tree was deleted during this audit. No complete whole-repository64GiB claim is made.


## Configured Cargo and Next Admission Frontier

The current checkout `.cargo/config.toml` has fixed target/build namespaces and the default compiler refusal, but no admitted package-registry opener. The actual supported native dependency executable now enters held Rust storage before Cargo update/fetch/install and forces `CARGO_HOME`/TMP. The runtime verifier and dependency-direction metadata capture also retain actual Rust storage through settlement. Bare registry builtins and arbitrary source/configuration/compiler overrides remain outside the qualified boundary.

The installed nightly and stable official Cargo configuration documentation (`share/doc/rust/html/cargo/reference/config.html`) explicitly states that aliases cannot redefine builtins. Therefore, a default config alias cannot intercept `fetch`, `update` or `metadata`. A `CARGO_HOME` path selector alone would also allow ordinary directory creation after backing closure; it is not a valid physical admission proof. Ambient `RUSTUP_HOME` is another concrete unbounded target-provision writer in the native dependency route; actual tools8 custody plus an installed scoped toolchain is required before redirecting it, since an empty home would break zero-touch compiler resolution.

New isolated Cargo argument-admission schema/corpus is authored at `caching/storage/enforcement/compiler/ingress/arguments`. Its first expected missing-implementation RED is preserved, then the neutral Source law passed 1/43 with an independent SQLite `json_each`/window-token query and Ajv. It admits only the already-issued target, exact compiler configuration and exact build/target inline configuration, and respects the executable argument delimiter. It is deliberately not yet wired into runtime while Nx retains a constant compiler defining Source across its genuine four-phase proof. It must be integrated behind an actually issued compiler reservation and qualified through ordinary routes before counting it as an enforcing producer boundary.
