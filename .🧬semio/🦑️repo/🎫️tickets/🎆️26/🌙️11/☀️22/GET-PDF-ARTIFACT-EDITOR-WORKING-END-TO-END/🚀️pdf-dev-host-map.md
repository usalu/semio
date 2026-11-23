# PDF Dev Host Map

Read-only exploration, 2026-11-22. No source file, no git state and no server was changed or started.

## 1. Variant files and diff (stdio-pdf vs stdio-xlsx)

Both variants have the same five files: `🏗️builder/🌐️vite/🟦️.ts`, `🚀️entry/🟦️.ts`, `🧪️tests/🚪️route/🟦️.ts`, `🧪️tests/🚪️route/🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`.

- `🧬️schema/🔣️.json` points `ownerRoot`, `viteConfig` and `browserEntry` at the PDF variant folder `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf`.
- `🏗️builder/🌐️vite/🟦️.ts` and `🚀️entry/🟦️.ts` are identical to the XLSX ones apart from the guard.
- `🚀️entry/🟦️.ts` guard: `variant !== "stdio-pdf-1-7"` (XLSX uses a regex over three variants). It calls `bootFrameworkOsDev({variant, formatCatalog: S_FORMAT_CATALOG_V1, catalogRows: composeSpecificOsCatalogV1(...).rows, brands: []})`, so it mounts the shell with the product format catalog.
- The route test (`🧪️tests/🚪️route/🟦️.ts`) is larger in the PDF variant. It adds a `--staged-binding` mode and a PDF picked-file routing check, and it asserts a runtime closure of `["stdio", "stdio-pdf"]`.
- Fixture: variant `stdio-pdf-1-7`, plugin `stdio-pdf`, app `s.stdio.pdf@1.7/*#editor`, reactPort 6260, wgpuPort 6360, dependsOn `["stdio"]`, artifactRef `s.stdio.pdf@1.7/*`. Input is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf`.
- XLSX fixture: variant `stdio-xlsx`, plugin `stdio-office`, app `s.stdio.xlsx@ecma-376/*#editor`, ports 6272 and 6372, contribution set.

## 2. Which target launches the PDF pane

- Playground row, in `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust/Cargo.toml`:
  `variant = "stdio-pdf-1-7"`, `app = "s.stdio.pdf@1.7/*#editor"`, `ports = { react = 6260, wgpu = 6360 }`.
- The row has NO `devContribution` key. Every XLSX row has one (`devContribution = "✏️s/🧑‍💻dev/🎭️variants/📕️stdio-xlsx/🧬️schema/🔣️.json"`). So the PDF dev contribution is not bound in the checked-in manifest. The route test only passes with `--staged-binding`, which injects it. This is a real gap to resolve before serving.
- Dev serve target name is built by `devServeCommandV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts` (line 470):
  `@semio-tech/framework-os-dev:dev-${variant}-${renderer}-${profile}`, with env `SEMIO_PLUGIN=<variant>`, `SEMIO_RENDERER`, `S_OS_PORT`.
- The XLSX ticket used `dev-stdio-xlsx-react-prototype` (profile `prototype`). By the same pattern the PDF pane is:
  `repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype` (port 6260).
- Caveat: I could not confirm that this target exists. No checked-in JSON/TOML/TS file contains `dev-stdio-xlsx-react-prototype` or `dev-stdio-pdf`; the names are generated from the playground rows. The only literal target in the repo tests is `dev-s-react-prototype`. `repo show project` could not answer (see section 3).
- The PDF project named `@semio-tech/stdio-pdf-plugin` (`🌎️hub/…/📘️pdf/📦️packages/🦀️rust/📋️project.json`) only has a `test` target (`repo ./📜️script.ts test`). It does not serve anything. The Cargo crate is `semio-hub-stdio-pdf`.

## 3. Repo MCP status (read-only)

- `show_project @semio-tech/stdio-pdf` FAILED after about 50 s: "this machine holds no sealed generation of the workspace ... Start the whole read on purpose (`repo graph --graph-policy fresh`)". Same for `show_targets`.
- `doctor`: verdict `degraded`, 0 errors, 13 warnings. Notable items:
  - Tools missing: cmake, docker, ffmpeg, gh, ninja. Fix: `repo tools setup all`.
  - Long exclusive holds: `repo build prototype` (PID 96470) held cargo-custody for 2675 s, and PID 11558 held an unclassified exclusive lease for 2595 s. Both are idle at about 0.1 to 0.9 % CPU. Ask the owner or stop them.
  - Idle run-many (PID 11324) and graph-plan (PID 35340) holders, also idle.
  - `dependencies.inputs-newer` on `bunfig.toml`. Fix: `repo dependencies ensure`.
  - Graph implementation newer than generation 260.
- `fleet_status`: 6 running, 0 blocked, 0 waiting. Storage and fleet readers timed out after 5 s. Workers g1-m04 (Linux, disk-short) and g1-m05 (Windows, stage-timeout) are stale for 8.4 h and 4.2 h.
- `leases_status`: the graph namespace is shared by at least three running graph-plan or build processes. One out-of-order lock violation was recorded (not enforced).
- Verdict: the fleet is not blocked, but the graph has no sealed generation, so a `current`-policy project read cannot answer. Serving the pane needs a fresh graph load, and the previous ticket's notes show that load repeatedly failed on a live-edited worktree (see `browser-hosts.md` in the XLSX ticket).

## 4. How the XLSX ticket drove the browser, and reuse for PDF

- `run-browser-io.ts` (header): wraps `repo run` targets. `host` runs `@semio-tech/framework-os-dev:dev-s-react-prototype` (6070). `xlsx-host` runs `@semio-tech/framework-os-dev:dev-stdio-xlsx-react-prototype` (6272). Watcher modes (`*-on-ready`) poll the port before running a journey.
- `probe-xlsx-pane.ts` (header): `fetch` to `127.0.0.1:6272` must answer. Then `ensureRepositoryBrowserPathV1` from `🧰️framework/…/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts`. Then Playwright `chromium.launch()`, loaded via `createRequire` from the OS-dev package. It records page errors and console refusals.
- `standalone-pane-journey.ts` (header): full journey (open, edit A1, save, reopen in en and de, dialects, refusal). Fixtures are `.xlsx` files under the XLSX artifact subsets. Screenshots and `journey.json` go to `🗑️generated/standalone-journey`.
- Reuse for PDF: yes, the harness pattern is reusable. Use `ensureDevServe` (from `🚀️local-hub/…/🟦️.ts`, which spawns `devServeCommandV1` and reuses a port that already answers) with port 6260, plus the same Playwright probe. The PDF journey needs its own script: the PDF fixture input path above and `artifactRef` `s.stdio.pdf@1.7/*`. The XLSX scripts themselves cannot be reused as-is because their URLs, fixtures and expected labels are XLSX-specific.

## 5. Launch config

- `/Users/ueli/Documents/semio/.claude/launch.json` EXISTS (1072 bytes, modified 2026-11-22 01:32). I did not read its contents; read it to see whether a PDF entry is already configured.

## Exact commands

Serve the PDF pane (only after the graph load completes; not run here):

```
cd /Users/ueli/Documents/semio && repo run @semio-tech/framework-os-dev:dev-stdio-pdf-1-7-react-prototype
```

Then open `http://127.0.0.1:6260/`.

Prerequisite check (read-only):

```
cd /Users/ueli/Documents/semio && repo doctor --only=graph,leases,processes
cd /Users/ueli/Documents/semio && repo leases status
```

Fresh graph read, which is the blocker (long-running, do not start without intent):

```
cd /Users/ueli/Documents/semio && repo graph --graph-policy fresh
```

## Open issues before the pane can run

1. Add `devContribution = "✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧬️schema/🔣️.json"` to the `stdio-pdf-1-7` row in the PDF `Cargo.toml`, as the XLSX rows do. Otherwise the dev target is not bound.
2. Confirm the generated target name with `repo show project @semio-tech/framework-os-dev --graph-policy fresh` (long read).
3. Get a fresh graph generation (blocked on the live tree, see section 3).
4. Clear the idle long-held leases and missing tools (`repo tools setup all`) if they block.
5. Write a PDF journey script analogous to `standalone-pane-journey.ts`, using port 6260 and the PDF fixture.
