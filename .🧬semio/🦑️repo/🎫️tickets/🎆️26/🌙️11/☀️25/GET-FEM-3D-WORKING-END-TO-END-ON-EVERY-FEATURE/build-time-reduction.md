# ⏱️ Build Time Reduction for the Plugin Wasm Loop

## Measured today

| Fact | Value | Source |
|---|---|---|
| One FEM wasm build | **36 min 40 s** (`Finished prototype … in 36m 40s`) | `🗑️generated/nfd-wasm-last.log` |
| Crates recompiled in that build | **589** | Same log: `Compiling` lines |
| Attempts needed (build 8) | **6** (peers' half-done edits) | `wasm-retry.out` |
| Profile | `opt-level="s"`, `lto="thin"`, `codegen-units=16`, `incremental=false`, `CARGO_INCREMENTAL=0`; 70 per-package overrides | `native-fem-profile.toml`, `native-fem-dev.sh` |
| Unit of build | One cdylib `semio-hub-fem-component`. It links the framework runtime and reactor, plus FEM **2D and 3D**, each with editor and viewer. | `🌎️hub/🧩️compositions/🏗️fem/🧩️component` |
| Machine | 16 cores, 64 GB; the build runs `-j 6` | |

The build is slow for two reasons. First, any edit low in the framework, such as job context, cleanup turns or value, changes that crate's artifact, and Cargo then rebuilds every dependent, so about 600 crates rebuild. Second, each crate is compiled whole, with no incremental reuse, and then thin-LTO-linked.

Browser-only bugs, such as today's live-cleanup stall, need this loop several times.

## Options, ranked by gain per effort

### 1. Iteration build profile (framework build policy) — biggest quick win

- **Change:** add a dedicated iteration profile next to `prototype`: `incremental = true`, `lto = false`, `codegen-units = 256`, `opt-level = 1`, and keep the hot-numeric overrides at `opt-level = 3`.
- **Keep `prototype` and release for verification and performance timing.** These builds measure turn costs that depend on optimization (F3).
- **Where it belongs:** the repo build policy, which already selects profiles behind the tool-neutral `repo run` interface, so every dev and agent gets the same profile.
- **Expected effect:**
  - a leaf edit (plugin or editor) drops from about 37 min to a few minutes, because incremental reuse lets unchanged codegen units be skipped;
  - a framework-root edit still rebuilds every dependent, but each crate compiles faster (no LTO, more parallel codegen). Option 3 addresses that cascade.

### 2. Native parity for browser-only host behaviour (framework test fixtures)

- **Change:** let `OriginalJobFixtureCaller` and the app-law fixtures simulate the browser host: async host and worker waits that span turns, per-turn frame deadlines (`slice_each_turn` was added today), and 32 pumps per reactor turn.
- **Why:** today's boot fault took about 4 wasm builds because the native law completes every host wait in one turn.
- **Effect:** most host-timing bugs then reproduce in a native test, which takes minutes instead of a 37-minute wasm build. The wasm loop becomes verification only.

### 3. Cut the rebuild cascade: contracts versus implementations (framework architecture)

- **Problem:** high-fan-in framework crates mix vocabulary (types, traits) with implementation, so an implementation-only edit rebuilds about 600 dependents.
- **Change:**
  - split each hot node into a small, stable contract crate (types, traits, WIT-shaped interfaces) and an implementation crate that only the composition root (hub component) depends on;
  - dependents compile against the contract;
  - inject the implementation at the composition root.
- **Effect:** an implementation edit rebuilds the edited crate plus the root, a handful of crates instead of 589.
- **Fit:** this matches the existing rules: domain-neutral framework, domain-specific extensions, and interfaces in front of implementations.
- **How:** measure fan-in with `repo graph` and start with the top 10 crates by dependents, which are likely the job context, value, retained-clone and plugin runtime crates.

### 4. Split the component along the component-model boundary (framework and plugin)

- **Change:** compile the framework runtime (reactor, runtime, job, value) once as its own wasm component. Each plugin artifact becomes a guest component that imports it through WIT; the repo already has a guest contract. Compose them at publish time instead of statically linking one cdylib.
- **Effect:** plugin edits never recompile the framework, framework edits never recompile plugins, and both link in seconds.
- **Cost:** the largest change, but also the most sustainable one for multi-app work: every plugin reuses the same runtime artifact, and the 50 machines can share it.

### 5. One component per artifact (plugin level)

- **Change:** build a `fem3d` component that does not compile the FEM 2D editor and viewer, plus a separate `fem2d` one, instead of one `fem` cdylib for both.
- **Effect:** removes the 2D plugin tier from 3D iteration, roughly a third of the plugin-tier crates.
- **Cost:** small, and it is independent of options 1 to 4.

### 6. Build from a consistent snapshot (repo tooling)

- **Problem:** retries come from compiling the live, replicated tree mid-edit (6 attempts in build 8).
- **Change:** build from an atomic snapshot of the replica, for example the last tree in which every crate type-checked, published by the archivist or the replica daemon.
- **Effect:** no wasted half-build attempts.

### 7. Shared artifact cache across machines (repo tooling)

The repo build tools already have cache lanes (`cache_status`). Use them for the wasm target too, so a framework crate is compiled once for all 50+ machines. The native route used here bypasses them because `repo run` never got past the graph read; fix that first.

## Recommendation

1. **Now:** option 1 for my own iteration builds (ticket-local profile), and option 2 as part of the live-cleanup fix, which adds async-host test vectors.
2. **Next, as their own tickets:**
   - option 1 in the repo build policy;
   - option 5;
   - options 3 and 4 as the long-term architecture (they compound);
   - options 6 and 7 in repo tooling.
