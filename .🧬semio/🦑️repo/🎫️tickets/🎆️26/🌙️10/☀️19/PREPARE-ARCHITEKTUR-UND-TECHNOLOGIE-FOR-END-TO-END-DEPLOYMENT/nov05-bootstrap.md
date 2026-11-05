# Native Repository Bootstrap

The Windows host initially had no `repo`, Go, Rust, or workspace `node_modules`. The checked-in native Windows launcher installer provisioned checksum-pinned Go 1.26.9 and built the launcher at `C:\Users\Anna\AppData\Local\semio\launcher\bin\repo.exe`. The launcher then provisioned checksum-pinned Bun 1.4.2. Its first `--help` failed before CLI dispatch because authored workspace package `@semio-tech/pack-json-decode-ts` could not resolve.

Materializing the 133 authored root-manifest workspace junctions restored the launcher command interface without downloading external dependencies or editing source manifests. This exposed a missing zero-touch prerequisite: neither native bootstrap installs these links before importing the package-backed build entry. A dependency-free launcher repair is being developed with a neutral corpus, independent Ajv admission oracle, real cold runtime import, concurrent first-use, collision, and cancellation tests.

The native MCP installer completed through the supported repository command:

```text
repo capacity run --memory=768 --cpu=1 -- repo run repo-mcp:install --parallel=1 --lang=en
```

The generic `run` estimate requested 3584 MB while the host could not admit it. The explicit supported 768 MB, one-core grant admitted the small native bootstrap; its observed footprint was 570 MB. Capacity enforcement remained enabled. Cold source admission, registry-managed Go download/extraction, native compilation, and installation all completed successfully. The installer only published the native executable, persisted its user PATH selection, verified help, and wrote its receipt; it performed no Git mutations or hook changes.

Receipt: `C:\Users\Anna\AppData\Local\Semio\repo\installed.json`.

Initial installed executable: `C:\Users\Anna\AppData\Local\Semio\repo\6063b74e0c4280885215804e9ce594d24e2e2e7782260d740138ba63ccaf5276\repo.exe`.

Build output: `D:\semio\.🧬semio\🦑️repo\⚡️cache\🗃️bin\prototype\repo\repo.exe`.

The parent agent connected this executable's MCP endpoint, read `repo://goals`, and resumed this existing open ticket before source repairs began. No `AGENTS.md` file, source-control state, or worktree was modified.

## Cold Workspace Link Contract

The regression corpus and schema are under the launcher's `📦️workspaces` unit. The native Go regression and independent Ajv oracle are under `🧪️tests/📦️workspace-links`. The planned implementation remains dependency-free and runs before JavaScript handoff, through an injectable launcher effect. It validates the full authored workspace roster before publishing anything; finite roster and manifest limits, portable package names, path containment, Windows aliases, duplicate paths and case-folded package names, and occupied destinations are explicit refusals.

Unix hosts publish directory symlinks. Windows needs NTFS directory junctions created through the native system interface because ordinary directory symlink creation can require developer mode or elevation. Publication must never overwrite an unrelated destination; concurrent identical first uses should converge without exposing a partially initialized junction. Every roster and publication step checks cancellation.

The neutral tests include scoped/unscoped names, Unicode source folders, traversal/absolute/glob paths, Windows device/trailing-dot/trailing-space aliases, and missing names. Runtime regressions begin with no `node_modules`, concurrently prepare links, then ask the actual registered JavaScript runtime to import a package and produce `42`. Other cases cover duplicate roster entries, case-fold collisions, real/dangling/wrong destinations, escaped source/scope links, bounded manifest reads, and cancellation between publications.

The supported canonical Go test target is being run before production implementation. Its cold global source parser is expensive but reports live list/state progress. A direct script invocation was investigated and correctly refused `Original parent Script process invocation required`; no synthetic envelope or authority was supplied. The canonical target is therefore retained through a definitive result.

The canonical red run completed its live global source parser and reached the verified dependency singleton barrier. The parent requested intentional resource serialization: cancel only this dependency-waiting test process to release its approximately 650 MB observed footprint and 768 MB reservation while the `dependencies sync --all` writer and native toolchain preparation proceed. The writer was left running, caches and test sources were preserved, and no production cold-link implementation was written before the red test. Restart the canonical red target once the shared dependency generation is ready.

### Observed Black-Box Red

Authored the tiny `cold` fixture in this ticket without copying checked-in source: paired root manifests, one local `@cold/one` package exporting `42`, one import script, and its own minimal runtime registry. It initially has no `node_modules`. The existing installed native launcher was invoked directly as `repo ./cold.mjs` with this fixture as its working directory. It exited **1** with `Cannot find module '@cold/one'` from the fixture's import script under pinned Bun 1.4.2. The expected result is exit 0 and `42`. This observed feature defect establishes red before production implementation despite the canonical Go run's shared-dependency barrier.

### Implemented Cold and Managed Phases

The source patch now adds `Host.Prepare` before JavaScript handoff. Portable local aliases are validated completely before publication. Windows junction creation uses the native system call without requiring symlink privileges; publication uses explicit `MoveFileW` with no replacement. Unix publishes symlinks through no-replace link creation. Physical manifests are bounded and checked for identity, size, and modification stability. Parent identity and containment are rechecked after staging and before publication. Correct existing aliases return before directory creation or staging; subsequent starts cause no alias or namespace writes.

Managed dependency namespaces are a separate phase. The new shared `SourceNamespaceClassificationProtocol` is adjacent to the existing `SourceOwnerReady` schema. The Go classifier reads the declared JavaScript manager, dependency cache directory and native mount metadata, and recognizes only the exact generation layout, full 64-character generation/prefix relation, bounded closed receipt, and exactly matching Source/target binding. It returns `authority:false`, `readiness:false`, and `contentObserved:false` and leaves the complete managed namespace untouched, including projected workspace providers. Existing JavaScript admission remains the sole readiness and compiler authority. Arbitrary foreign links, mismatched claims, and forged fields are refused. Independent read-only review found no actionable managed-classifier defect; execution is still pending.

The corpus now covers 19 portable path/name rows, explicit 256-unit UTF-16 boundaries for ASCII, supplementary emoji and variation selectors, and nine managed binding cases. Native tests also exercise publication races, manifest links, parent swaps, cancellation and immutable projected-provider namespaces. The formatter runs through the registered Go tool and an authored `format.go` input rather than invoking an unregistered formatter binary. Native reinstall and effectful cold green remain held while the parent owns the shared dependency writer. Effectful cold green will materialize the authored `cold` inputs under `🗑️generated`; its runtime links and outputs will therefore stay in generated storage.

## Independent Preflight Repair Review

Reviewed the parent's current preflight implementation, closed facts schema, and neutral corpus while dependency work was serialized. The added `fold-safe` observation is restricted to Windows in the schema and is produced only for a journaled port plus an exact refusal listed by the existing directory-storage protocol. Unknown, near-match, cancellation, cleanup, unverified-case, and other-platform failures retain their refusal. Exchange failure, capacity, architecture, and native tool requirements remain independently enforced. The neutral cases include the three allowed refusal identities and disallowed paths, plus fold-safe hosts that still lack swap, toolchain, or capacity. No actionable defect was found in this review; execution remains the parent's validation responsibility.

## Current Native Green and Installation

The source-current native installer completed with exit 0 and published `C:\Users\Anna\AppData\Local\Semio\repo\4cbfc6499526282de516b1bfe2b9e2711d3f8138d827014526e03966fbc4e69a\repo.exe`. The version-1 installed receipt selects that digest and executable. Existing running processes retain their older immutable executable.

The actual canonical Go source projection initially omitted the two new domain members. Registering them in the shared taxonomy fixed both standalone native compilation and canonical layout consistently. A query of the actual canonical plan proves both files occur once in the launcher package; a checked-in layout regression covers the same mapping.

The generated-only cold fixture succeeded twice through the source-current cache executable with exit 0 and output `42`. Its child checks an authored registry marker proving the selected workspace is the fixture. The installed executable subsequently produced the same `42`. Both the cache and installed native help routes succeeded from the real checkout. Before/after snapshots of the actual checkout's root namespace, scoped rosters, link targets, identities, sizes and timestamps were identical. No Source aliases were repaired, replaced, or added during these proofs.

The actual existing `@semio-tech/framework` junction exposed two native Windows details: Go reports it as `ModeIrregular` although `Readlink` succeeds, and generic `EvalSymlinks` does not resolve it. Native link recognition now requires successful `Readlink` plus followed directory identity; canonical directory resolution uses the existing native host primitive. An actual no-write regression validates the existing matching alias. Full native script/help routes now use the launcher's shared start-directory and paired-manifest resolver, with nearest-workspace and explicit-override tests.

Registered Go tests against the actual canonical source projection passed the workspace, managed namespace, execution and script-route families. They exercise all 19 neutral rows, three UTF-16 boundary glyphs, nine managed receipt mutations, real unprivileged directory junctions, concurrency, cancellation, no-replace publication and projected-provider immutability. The linked-file-manifest case is skipped on this Windows host because file-symlink creation requires additional OS privilege; directory-junction tests executed. The paired-workspace CLI regression passed separately. Shared short test sandboxes keep generated paths within 256 UTF-16 units; the seeded runtime fixture now uses the platform executable name and path separator.

The independent Ajv corpus oracles, canonical TypeScript layout suite and preflight third-party/Python suites await the shared dependency generation. These pending checks are not represented as passing. The authored fixture inputs and helper scripts remain in the ticket; generated overlays, snapshots and cold runtime effects remain under `🗑️generated` for coordinated final cleanup.
