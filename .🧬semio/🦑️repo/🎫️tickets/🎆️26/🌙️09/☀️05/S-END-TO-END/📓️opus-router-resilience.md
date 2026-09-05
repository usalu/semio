# Lane H — Router resilience + demonstrator surface dependencies

Ticket `26/09/05/S-END-TO-END` · Opus implementer · started 14:25, report written 15:1x.

## Defect, restated from live evidence

`AppRouter.build failed: plugin "demonstrator" contributes a surface for "s.cad.cad@1/*" without depending on owner "cad"` (×8, `📓️baseline-runtime.md` 14:05). Two independent causes:

1. **Framework**: `AppRouter.build` threw on the FIRST breaching manifest, so one plugin's authoring defect left the whole session with `appRouter === null` — no "Open with…", no Settings default-apps table, no opening relay, for EVERY app.
2. **Artifact**: the demonstrator wasm in the live dev cache carries a manifest with `dependencies: null` and eight foreign surfaces. Verified directly, not inferred:

```
$ python3 -c '…json.load("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🎪️demonstrator/🔣️.json")…'
deps: null
apps: ['s.demonstrator.playground@1/*#editor', 's.demonstrator.playground@1/*#viewer',
       's.procedural.procedural3d@1/*#editor', 's.cad.cad@1/*#editor', 's.puzzle.puzzle3d@1/*#editor',
       's.sourcing.curation@1/*#editor', 's.sourcing.curation@1/*#viewer',
       's.process.process3d@1/*#editor', 's.process.process3d@1/*#viewer', 's.gis.gismap@1/*#editor']
artifactKinds: []
```

The `.core.wasm` beside it is dated Aug 27, so the runtime manifest is a stale artifact: the source fix only reaches the shell through the Wave 2 catalog rebuild (lane F).

## Changes

### 1. Kernel TS — per-plugin fault isolation (schema-first contract)

`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`

- `AppRouter` (region `🔖️AppRouter`, class doc :573-586) now carries `faultByPluginId: ReadonlyMap<string, Fault>`.
- `AppRouter.build` (:600-680) is **total — it never throws**. Each manifest is staged (`staged`/`stagedRefKeys`) and committed only if none of its apps breaches; on a breach the plugin contributes **no** surface at all (never a partial prefix) and one typed `Fault` is recorded under its plugin id. `artifactKinds` ownership claims survive the exclusion, exactly like Rust `unregister_plugin`, so a later contributor can never inherit an excluded plugin's kind.
- New accessors: `AppRouter.pluginFaults()` (sorted by plugin id) and `AppRouter.faultFor(pluginId)` (:681-692).
- Fault scope now carries `appId` as well as `pluginId`, so the excluded surface is named.

### 2. Shared language-neutral fixture

- `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json` — four manifests (`cad` owner, `demonstrator` borrowing `s.cad.cad` with no dependency, `aec-building` borrowing it WITH a dependency, `twin` registering the same `AppRef` twice) plus `expectedOwners`, `expectedRoutes`, `expectedFaults`.
- `…/🧬️.schema.json` — draft-07 schema, `additionalProperties:false` throughout; used as the independent Ajv oracle in the TS tests.
- Directory shape uses the taxonomy's generic `fixture-case` kind (`🧫️<kebab-slug>` under `🧫️fixtures`, precedent: `📕️norm/🗿️artifacts/⚖️en1990/🧫️fixtures/🧫️child-owner-isolation`), so no `🔣️taxonomy.json` edit and no new semantic emoji were needed.

### 3. Rust host — the authority, same contract

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`

- `AppRouterState.plugin_faults: BTreeMap<String, Fault>` (:8654-8658).
- `register_manifest` (:8666-8720) rewritten to stage-then-commit: the previous version pushed each app's surface as it went and returned `Err` mid-loop, i.e. a breaching manifest could leave a **partial** registration behind. Now all-or-nothing, the fault is recorded under the plugin id and carries `FaultScope { plugin_id, app_id }`.
- `register_manifests(&[(String, PluginManifest)]) -> Vec<Fault>` — the total twin of TS `AppRouter.build`.
- `plugin_faults()` / `fault_for(plugin_id)`; `unregister_plugin` also clears the plugin's fault so a fixed hot-reload comes back clean.

### 4. Assembly-time gate + testkit helper (so the defect cannot ship again)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` `app::surface_dependency_breaches(&PluginManifest) -> Vec<String>` (next to the existing `register_contributions` §4 gate): every app bound to an artifact kind the manifest does not own must name that kind's owner among its dependencies. Owner is derived from the canonical `s.<plugin>.<artifact>` grammar via `ArtifactKindId::plugin()` — the same derivation `register_contributions` uses, and the same one `preflight_artifact_identity` already forces on every declared kind, so this is **one** source of truth, not a second dependency list. Non-canonical kind ids (e.g. host-media `3d.*`) are skipped, not guessed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs` `try_build` calls it after the app factories register and **before** `begin_artifact_assembly`, failing with `plugin-assembly.surface-dependency-gate` — nothing commits when the gate fires.
- `semio_framework_plugin::testkit::assert_surface_dependencies_declared(&manifest)` (SurfaceTestkit region, beside `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`).

### 5. ShellHost — the fault is visible, the session is not

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

- `ShellCatalogProbe.plugins[]` gained `routerFault?: { code, message }` (:626-631).
- New exported pure builder `shellCatalogProbePlugins(registry, pluginStatusById, router)` (:636-650) — one row per installed registry entry, install status plus the router fault; a plugin is never dropped from the list.
- The `appRouter` memo (:5100-5119) no longer try/catches — `AppRouter.build` is total — and a sibling memo maps plugin id → fault.
- The `🔖️CatalogSmokeProbe` effect feeds `shellCatalogProbePlugins`, and a new effect logs one permanent `console.error("AppRouter excluded plugin …")` per excluded plugin (not `[DEBUG]`: an excluded plugin installs cleanly, so this is the only non-probe signal).

### 6. Demonstrator manifest

`✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs`

A concurrent session had already staged `.depends_on("cad"|"gis"|"procedural"|"process"|"puzzle"|"sourcing"|"stdio", VersionReq::Any)` plus a literal-list assertion (`git diff HEAD` on the file, index state `M `) while this lane was mid-flight; that work was **kept, not reverted**, and my own const-based variant was withdrawn. Added on top: test `every_borrowed_surface_is_backed_by_a_declared_dependency`, which derives the requirement from the built manifest's own app dialects through the new testkit helper — so adding a borrowed app without its dependency fails the crate's own suite, independent of the literal list.

## Commands and outputs

Both TS suites below were re-run unchanged after the session reset (18:23 and 18:24, same pass counts) against the on-disk state this report describes.

### Kernel vitest (fixture + regression)

```
$ bunx vitest run --config 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts -t "AppRouter"
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel
 Test Files  1 passed | 1 skipped (2)
      Tests  2 passed | 48 skipped (50)
   Duration  12.64s

$ bunx vitest run --config 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts
 Test Files  2 passed (2)
      Tests  50 passed (50)
   Duration  16.26s
```

### ShellHost react test (new)

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🟦️.ts 🧯️router-plugin-faults.test.ts
 RUN  v4.1.10 …/🎯️targets/⚛️react
 Test Files  1 passed (1)
      Tests  2 passed (2)
   Duration  83.25s
```

### Existing opening relay (regression, same `AppRouter.build`)

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🟦️.ts 🚪️opening.test.ts
 Test Files  1 passed (1)
      Tests  2 passed (2)
   Duration  19.15s
```

### Renderer typecheck (whole package, includes ShellHost + the new test)

```
$ cd …/🎯️targets/⚛️react && bunx tsc --noEmit -p tsconfig.json          # EXIT 2
total errors: 62
     15 ../../../../../../../🧵️backbone-worker.ts
      8 ../../../../../../../🟦️.ts
      7 …/🧱️elements/🛠️ShellHelpers/🟦️.tsx
      7 …/🧱️elements/🛂️SpaceAdministration/🟦️.tsx
      4 …/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx
      …
      1 …/🧱️elements/🏛️ShellHost/🟦️.tsx
```

All 62 are pre-existing peer work in flight (space administration, backbone worker, repo library, `Bun` types). **Zero errors in anything lane H wrote**: the single `🏛️ShellHost` error is `:5456` `buildOsCommands(… activeTutorials …)` — a `TutorialDefinition` label mismatch in a region this lane never touched — and `🧯️router-plugin-faults.test.ts` and `🎠️kernel/🟦️.ts` produce none.

### Rust — host `AppRouter` fixture parity (the authority)

The ticket's shared `target-s-e2e` was unusable: a peer's `cargo test -p semio-s-plugin-norm --lib --test surface_rende…` (pid 32112) had held `target-s-e2e/debug/.cargo-lock` for **3 h 37 min** (its rustc child alone at 2 h 20 min), and both of this lane's queued jobs sat on `Blocking waiting for file lock on build directory` for over an hour. Moved to a private `target-s-e2e-h` (same idiom lane C used with `target-s-e2e-c`), which cost one cold 44-minute build.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-h \
  cargo test -p semio-framework-plugin-host --lib --no-fail-fast --message-format=short \
  -- plugin_fault_isolation_matches_the_shared_fixture

warning: `semio-framework-plugin-host` (lib test) generated 24 warnings
    Finished `test` profile [unoptimized] target(s) in 44m 09s
     Running unittests 🦀️.rs (target-s-e2e-h/debug/deps/semio_framework_plugin_host-93c8012fd8822bb8)

running 1 test
test component::app_router_tests::plugin_fault_isolation_matches_the_shared_fixture ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 246 filtered out; finished in 0.01s
EXIT 0
```

Both languages therefore agree on the same vector file: same owners, same route order, same two fault codes attributed to the same two plugins, whole-plugin exclusion in both.

### Rust — demonstrator surface-dependency law

`semio-framework-plugin` (the crate carrying `surface_dependency_breaches`, the `try_build` gate and the testkit assertion) **compiles clean** — `warning: semio-framework-plugin (lib) generated 218 warnings`, zero errors — so the gate and the helper are proven to build.

The demonstrator crate's own test is **still building** at the time of writing:

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-h \
  cargo test -p semio-s-plugin-demonstrator --lib --no-fail-fast --message-format=short \
  -- every_borrowed_surface_is_backed_by_a_declared_dependency
…
   Compiling semio-s-plugin-stdio v0.1.0 (…/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust)
   # one rustc, 1 h 50 min elapsed and counting, at load average ~100
```

`semio-s-plugin-stdio` is a demonstrator dependency and is the single slowest crate in the repo (the coordinator's own wasm check of it ran 46 min earlier today); five more plugin crates plus demonstrator follow it. The job is detached (`nohup`, pid 21995) and keeps running; its log is `…/scratchpad/lane-h/demonstrator-test.txt` — `grep -E "test result:|^error" ` on it gives the verdict. Re-running that one command is the only outstanding verification for this lane.

## Blockers

- **Cargo throughput, not correctness.** The shared `target-s-e2e` is held for hours at a time by peer jobs (a `semio-s-plugin-norm` test held its build lock 3 h 37 min); this lane's private `target-s-e2e-h` paid a 44-minute cold build for the host test and the demonstrator test is still compiling `semio-s-plugin-stdio` after 1 h 50 min at load average ~100. Nothing here is blocked on a decision — only on machine time.
- **Gate reach not yet proven repo-wide.** `try_build`'s new `plugin-assembly.surface-dependency-gate` can only fire for a plugin that registers a surface on a foreign artifact kind. A repo-wide grep (`use <plugin>::{editor,viewer}::`) finds exactly one such crate outside the wgpu renderer — demonstrator — and extensions always carry their host as `dependencies[0]` by `ExtensionBundle`'s own rule, so no other crate should trip it; that reasoning is a grep, not a compile of all 59 crates. Wave 2's rebuild is where it gets exercised for real.
- **`🏃️run/🦀️.rs` is out of sync with the host's async API** (independent of this lane): `run/🦀️.rs:1800` calls `self.app_router.register_manifest(…)` and `:1802 owned_surface_gaps()` without `.await`, while the host declares both `async` (file mtime Sep 5 13:35 vs run Sep 4 13:41). That is a peer's in-flight async migration; not touched, and it will make `semio-framework-os` fail to compile until they land it. The `AppRouter` API change here is additive, so it neither causes nor worsens that.
- **Old ticket scratch scripts** `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-b-verify.ts` and `🧪️w1-d-parity.ts` assert that `AppRouter.build` THROWS the two surface faults. They are standalone evidence scripts of a closed ticket (no project/target references them), so they were left as-is; anyone re-running them will see the new total-build contract instead.
- **Catalog smoke report shape** (`🧑‍💻dev/📜️script.ts`, `🧬️catalog-smoke.schema.json`) still counts only `failed`/`crashed` install statuses in `failedPlugins`. The probe now also reports `routerFault` per plugin; folding it into the smoke's fail set belongs to lane A, whose files were being edited concurrently.
