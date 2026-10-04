# 🚚️ Explore — CDN static publishing and Docker publishing precedents (quiz split deployment)

Read-only exploration of `C:\git\semio` on 2026-09-29. Nothing in the repository was modified except this file. Every
snippet is quoted verbatim from the cited path; "verified" means read in the file, "inferred" means concluded from
artifacts and marked as such.

Target under design: (a) the proctor (`🎓️teaching/🛂️proctor`, crate `teaching-proctor`, bin `proctor`) zero-touch on
Docker at `proctor.quizzes.architektur-und-technologie.de`; (b) the client site (`🎓️teaching/🏛️architecture/❓️quiz`,
`@teaching/architecture-quiz`) on a CDN at `quizzes.architektur-und-technologie.de`.

## 0. Findings in one screen

| Question | Answer (all verified unless marked) |
|---|---|
| Which CDN does the repo publish to? | The repo **builds** CDN-ready folders and never uploads them. The artifact contract is GitHub Pages: `CNAME` + `.nojekyll` + `404.html`/`index.html` (`_headers` only on play's asset pages). Upload is manual. |
| Any CI? | No. `.github/workflows/` does not exist; `README.md` says "`.github/workflows/` is empty and nothing is uploaded anywhere". Four workflows (`gh-pages.yml`, `play-sites.yml`, `playwright.yml`, `repo-test.yml`) were deleted in `de617a7c17f`. `.github/` holds only `dependabot.yml`, `agents/`, `hooks/` (disabled). |
| Credentials for publishing? | None exist anywhere in the repo. No token, no secret name, no `gh`/`wrangler`/`rsync`/`aws` call. |
| Which script/target/launch row publishes play / projektetage? | None publishes. `build` writes `dist/pages/<page>/` (play) or `dist/` (projektetage). Launch rows are `📦️build🏢️semio-tech🎡️play` (group `4_build`, order `11`) and `🛠️dev📽️projektetage` (no build row for projektetage). |
| How does os-hub get published? | `bun nx run os-hub:publish` writes a **local** `dist/publish/os-hub-<version>-<platform>-<arch>.tar.gz` + `.sha256`. "nothing is uploaded or pushed anywhere". |
| Registry / image name / multi-arch? | **No registry precedent at all.** `rg` finds no `docker push`, `docker login`, `buildx`, `--platform linux`, `ghcr.io/usalu` in any tracked file or script (root `📜️script.ts` and the hub scripts included). Local image name only: `semio/os-hub` and `semio/architecture-quiz`. Single-arch (host arch). `ghcr.io` occurs only as devcontainer feature ids. |
| Compose with a TLS proxy service? | **No precedent.** Hub and quiz compose files publish plain HTTP on `127.0.0.1` and document a host-installed Caddy/nginx. One stale `Caddyfile` (coordinator) uses `{$DOMAIN:…}` and a compose service name but has no compose file. |
| Backend origin for a CDN client? | Build-time Vite `define` (`import.meta.env.VITE_S_HUB_URL`, `import.meta.env.SEMIO_PLAY_PAGE_ORIGINS`), with a page-origin fallback when empty. Same-origin mount `/_semio/hub` (dev proxy / deployment proxy). **No** runtime config file, no meta tag, no `window.__X__` precedent. Play and demonstrator do not talk to the hub at all. |
| Proctor CORS ready for credentials-free cross-origin fetch + preflight? | Yes for `https://quizzes.architektur-und-technologie.de` once it is in `PROCTOR_ALLOWED_ORIGINS`: OPTIONS is answered `204` outermost with echoed origin and `access-control-allow-headers: content-type`; the quiz client sends only `content-type: application/json`. Gap: no `Access-Control-Max-Age`, so every `POST /commands` and `POST /queries` pays a preflight. |
| Old domain references | 17 tracked files outside tickets (section 4). |
| Never built | The quiz image (`docker-image-build`) and the Caddyfile (`caddy validate`) have never been run: the site-infra report says the Docker daemon was not running. |

## 1. CDN static publishing

### 1.1 What "publishing" means today

No script uploads anything. The root README states it directly (`README.md:128-131`):

```
> **Status, plainly.** This repository is the source, not a distribution. There is no installer, no
> published container image, no package on any registry and no hosted instance; `.github/workflows/`
> is empty and nothing is uploaded anywhere. Everything below is built from this checkout after
> `npm run setup`, and every release binary named below lands under its own product's `dist/`.
```

The taxonomy registers the two GitHub Pages marker files as fixed contracts
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`):

```json
    "nojekyll": {
      "pathPattern": "**/.nojekyll",
      "authority": "GitHub Pages",
      "reason": "Jekyll processing opt-out",
      ...
      "verification": "GitHub Pages deployment",
    "github-pages-cname": {
      "pathPattern": "**/CNAME",
      "authority": "GitHub Pages",
      "reason": "Custom domain discovery",
```

The mit-bestand interim report names the host (`♻️mit-bestand/📋️bericht/📋️zwischenbericht/📋️zwischenbericht.tex:653`):
"Die vorgesehenen Kosten für Hosting sind bislang nicht angefallen, da bis jetzt die Seite kostenlos über GitHub Pages
bereitgestellt werden konnte." So the 33.projektetage site was hosted on GitHub Pages (verified statement, upload
mechanism not in the repo).

The play page budget equals the GitHub Pages site size guideline (`PLAY_PAGE_BUDGET_BYTES = 1_000_000_000`, inferred
link). `_headers` in play's satellite pages is Netlify/Cloudflare Pages syntax; GitHub Pages ignores it and already
answers `access-control-allow-origin: *` (inferred, not stated in the repo). The code therefore proves the **artifact
shape**, not the upload target. The shape is portable to any static host.

### 1.2 Shared Vite plugin: `CNAME`, `.nojekyll`, `404.html`, `index.html`

Every Vite site of the repo gets its deploy markers from one function
(`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`):

```ts
/** @emoji 🚫️ Vite: writes `dist/.nojekyll` on every build (unconditionally — any static host that runs
 * Jekyll, e.g. GitHub Pages, silently drops files/dirs starting with `_` otherwise, breaking Vite's own
 * `__vite-browser-external-*.js` shim chunk) and `dist/CNAME` when a brand declares `cnameHost`. */
export function staticDeployMarkerVitePlugins(cnameHost: string | undefined): OwnedBuildPlugin[] {
  ...
      closeBundle() {
        if (!writeOutput) return;
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, ".nojekyll"), "");
        if (cnameHost) writeFileSync(resolve(outDir, "CNAME"), `${cnameHost}\n`);
      },
```

`404.html` is a byte copy of the emoji entry, written by `semioEmojiIndexHtmlVitePlugin`. On GitHub Pages `404.html` is
the SPA fallback for path routes (deep links answer HTML with status 404, inferred behaviour of that host):

```ts
/** @emoji 📄️ Conventional static-host entry filenames emitted beside the constitutional emoji HTML entry. */
export const STATIC_SITE_HTML_ALIASES = ["index.html", "404.html"] as const;
...
    closeBundle() {
      if (!writeOutput) return;
      const source = resolve(outDir, fileName);
      if (!existsSync(source)) return;
      const html = readFileSync(source);
      for (const alias of STATIC_SITE_HTML_ALIASES) {
        writeFileSync(resolve(outDir, alias), html);
      }
    },
```

`semioHostHtmlVitePlugin(repoRoot, { …, cnameHost })` wires it (`SemioHostHtmlSpec.cnameHost`: "Custom domain this app's
static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root, alongside the
always-written `.nojekyll` marker"). The quiz site already uses this path, see section 1.5.

Cache headers: no site sets any. GitHub Pages fixes its own; `_headers` is used by play only for CORS on satellites.
The proctor's own static host constants (`🎓️teaching/🛂️proctor/🔨️modules/🌐️site/🦀️.rs`) are the only cache policy in
the quiz area: `IMMUTABLE = "public, max-age=31536000, immutable"` for `assets/`, `REVALIDATE = "no-cache"` for
`*.html`, `SHORT = "public, max-age=3600"` for the rest. A CDN with `_headers` support could mirror them (optional).

### 1.3 `🏢️semio-tech/🎡️play` — multi-page CDN build

Entry: `bun nx run @semio-tech/semio-tech-play:build` → `📋️project.json` target:

```json
    "build": {
      "executor": "nx:run-commands",
      "dependsOn": [
        "prepare-release"
      ],
      "options": {
        "cwd": "🏢️semio-tech/🎡️play",
        "command": "bun ./🔨️modules/📦️site/📜️script.ts build",
        "forwardAllArgs": false
      },
      "cache": true,
      "outputs": [
        "{projectRoot}/dist/pages"
      ]
    },
```

`🔨️modules/📦️site/📜️script.ts`:

```ts
      await prefetchPlayMapTiles(this.repoRoot);
      await buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist/site"), owner: "play:site", signal: controller.signal, environment: { SEMIO_BUILD_MODE: "ship", SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) });
      publishPlayPages(join(root, "dist/site"), join(root, "dist/pages"), PLAY_HOST);
```

`publishPlayPages` (`🔨️modules/📦️site/📄pages/🟦️.ts`) is the whole "publish": it splits `dist/site` into one folder per
CDN page and writes the host markers into each:

```ts
  for (const pageEntry of pages) {
    const destination = join(pagesDir, pageEntry.name);
    mkdirSync(destination, { recursive: true });
    for (const name of pageEntry.directories) renameSync(join(siteDir, name), join(destination, name));
    writeFileSync(join(destination, "CNAME"), `${pageEntry.host}\n`);
    writeFileSync(join(destination, ".nojekyll"), "");
    if (pageEntry.name !== "play") {
      writeFileSync(join(destination, "_headers"), "/*\n  Access-Control-Allow-Origin: *\n");
    }
  }
```

Hosts are derived from a schema-first catalog `🔨️modules/🧩️runtime/🔣️.json` (`"host": "play.semio-tech.com"`,
schema in `🧬️schema/🔣️.json`) and a suffix constant:

```ts
export const PLAY_ASSET_PAGE_SUFFIX = "assets.semio-tech.com";
export function playPageHost(name: string, apex: string): string {
  return name === "play" ? apex : `${name}.${PLAY_ASSET_PAGE_SUFFIX}`;
}
```

The 1 GB budget is enforced at build time (`assignPlayPages` throws "at or above the … byte CDN page limit"), tested
language-agnostically in `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpages/🟦️.ts`.

Where the artifacts go is written only in the ticket status file
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/📓️status.md`, "## 09-28"):

```
`prepare release` reported 69 panes and 60 components. `bun ./🔨️modules/📦️site/📜️script.ts build` exited 0 in 117s ...
Upload each folder under `🏢️semio-tech/🎡️play/dist/pages`:

| folder | host | bytes |
| play | play.semio-tech.com | 176474018 |
| map | map.assets.semio-tech.com | 609289505 |
| media | media.assets.semio-tech.com | 203051322 |
| modules | modules.assets.semio-tech.com | 711825652 |

Each page is under the 1_000_000_000 byte budget. `play` has `index.html` and `CNAME`. The three asset pages have `CNAME` and `_headers` with `Access-Control-Allow-Origin: *`.
```

The ticket also holds `📜️static-serve.py` ("CDN-like static server for a built site") for local smoke tests. There is
no `publish`/`deploy` nx target, no launch row and no root script for play beyond `build`:
`package.json` `"build:semio-tech:play": "bun nx run @semio-tech/semio-tech-play:build"`. `dist/pages` is git-ignored
(`.gitignore:308 dist`).

### 1.4 `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` — single-page CDN build

No `publish` step at all. The package `@semio-tech/mit-bestand-praesentation-projektetage`
(`📦️packages/🟦️typescript/`) builds with plain Vite; the Vite config carries the CNAME
(`🏗️builder/🌐️vite/🟦️.ts`):

```ts
  base: "./",
  publicDir: resolve(bundleRoot, "../../🌐️public"),
  plugins: [
    semioServeCloseVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "33. Projektetage",
      entry: "./🟦️.ts",
      bodyClass: "h-screen w-screen overflow-hidden",
      cnameHost: "33.projektetage.zukunft-bau.mit-bestand.de",
    }),
    semioEmojiIndexHtmlVitePlugin(bundleRoot),
```

Target (`📋️project.json`): `"build": { … "command": "bun ./📜️script.ts build", … "outputs": ["{projectRoot}/dist"] }`, and
`📜️script.ts` `BuildScript` calls `runViteBuild(this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts")`. The output
`📦️packages/🟦️typescript/dist/` (git-ignored by `**/📦️packages/*/dist/`) was observed to contain `.nojekyll`,
`CNAME` (`33.projektetage.zukunft-bau.mit-bestand.de`), `🌐️.html`, the emoji-named media, `assets/`. The site is
static and talks to no backend.

### 1.5 CI history (deleted, still in git)

`git show de617a7c17f^:.github/workflows/gh-pages.yml` — docs site to GitHub Pages, manual trigger, no secrets
(`GITHUB_TOKEN` via `id-token`/`pages` permissions):

```yaml
name: docs (GitHub Pages)
on:
  workflow_dispatch:
permissions:
  contents: read
  pages: write
  id-token: write
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v2
      ...
      - name: Install and build docs site
        run: |
          bun install --frozen-lockfile
          bun nx run @semio-tech/compose-sketchpad-docs:build
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: compose/sites/docs/dist
  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

`play-sites.yml` (same commit) built a matrix that included the projektetage and only verified + uploaded workflow
artifacts, never deployed them:

```yaml
      - name: Verify static site artifacts
        run: |
          test -f "${{ matrix.dist }}/🌐️.html"
          test -f "${{ matrix.dist }}/.nojekyll"
          test -f "${{ matrix.dist }}/CNAME"
      - name: Upload dist artifact
        uses: actions/upload-artifact@v4
```

So the only CI-shaped precedent for CDN deployment is `actions/upload-pages-artifact@v3` + `actions/deploy-pages@v4`
(one Pages site per repository/environment, custom domain by `CNAME`). It is the repo's history, not its current state.

### 1.6 How a CDN client learns a backend origin (precedents)

There are three, none of them a runtime config file:

1. **Build-time Vite `define` (the hub client `s`).**
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`:

   ```ts
    // 👥️ Non-secret collaborative endpoint metadata; authority stays inside the local relay.
    "import.meta.env.VITE_S_HUB_URL": JSON.stringify(process.env.S_HUB_URL ?? ""),
    "import.meta.env.VITE_S_DATA_DIR": JSON.stringify(process.env.S_DATA_DIR ?? ""),
   ```

   Consumer `🏛️ShellHost/🟦️.tsx` (`readViteSEnv`, `hubBootstrapOriginV1`): empty ⇒ `undefined` ⇒ the page origin
   ("a shell embedded beside its own hub"); non-empty ⇒ `new URL(declared).origin`. The Nx build target treats the
   value as a cache input because it bakes into the artifact
   (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1136-1140`):

   ```
   // 🌐️ `S_HUB_URL` and `S_DATA_DIR` are baked into the bundle at Vite `define` time
   // (`🏗️builder/🌐️vite/🟦️.ts:237`, `import.meta.env.VITE_S_HUB_URL`), so two bundles that differ
   // only by which hub they sign in against are DIFFERENT artifacts. Without these env inputs the
   // cache key ignores them and a cached bundle silently answers for the wrong hub.
   inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" }, { env: "S_DATA_DIR" }, ...
   ```

   The hub client fetches cross-origin with `credentials: "include"` and the hub answers with an origin allowlist
   (`OS_HUB_ALLOWED_ORIGINS`, section 3). `🌎️hub/README.md:590-593` (section "A browser") documents the CDN-style split: "Serve the release
   bundle … from any static web server, put its origin in `OS_HUB_ALLOWED_ORIGINS`, and put both behind the same
   TLS-terminating proxy."

2. **Build-time `define` only on `vite build` (play asset origins).**
   `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`:

   ```ts
    define: { "import.meta.vitest": "undefined", ...(command === "build" ? { "import.meta.env.SEMIO_PLAY_PAGE_ORIGINS": JSON.stringify(JSON.stringify(playPageOrigins(PLAY_HOST))) } : {}) },
   ```

   Consumer (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts`):

   ```ts
   /** 🌐️ CDN page origins keyed by root route. Empty unless a production build injects them. */
   export function readPublishedPageOrigins(): Readonly<Record<string, string>> {
     let raw: string | undefined;
     try { raw = import.meta.env.SEMIO_PLAY_PAGE_ORIGINS as string | undefined; } catch { return {}; }
     if (!raw) return {};
   ```

   Dev (`serve`) stays same-origin; only the release build injects absolute origins. This is the exact shape wanted
   for the quiz site.

3. **Same-origin mount fronted by a proxy.** `HUB_SAME_ORIGIN_MOUNT = "/_semio/hub"`: "the dev server forwards it to
   `S_HUB_URL`, and a deployment fronts the hub there, so module downloads never cross an origin"
   (`…/🌎️hub-source/🟦️.ts:53-55`); the dev proxy is in the `s` Vite `server.proxy["/_semio/hub"]`. A CDN cannot proxy,
   so this does not apply to a CDN-hosted quiz site.

No `fetch("./config.json")`, no `<meta name=…>` read, no `window.__…__` config global exists (searched).

The quiz site today is same-origin only (`🎓️teaching/🏛️architecture/❓️quiz/🟦️.ts`):

```ts
const root = document.getElementById("root");
if (root) mountQuiz(root, { proctor: "", tenant: ARCHITECTURE_QUIZ_TENANT });
```

`QuizOptions.proctor` is documented as "the proctor base URL (`""` is the site's own origin)" and
`proctorTransport(baseUrl)` strips trailing slashes and prefixes `${root}${request.path}`
(`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts:61-83`). The seam already exists; only
the value must change. The current Vite config is a static object with `base: "/"`, a `server.proxy` of the gateway
routes and `cnameHost: "quizze.architektur-und-technologie.de"`:

```ts
const proctor = `http://127.0.0.1:${process.env.PROCTOR_PORT ?? "8791"}`;
const gatewayRoutes = ["/instance", "/commands", "/queries", "/actors", "/scopes"];
...
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "Quizze · Architektur und Technologie",
      entry: "./🟦️.ts",
      loading: { title: "Quizze · Architektur und Technologie" },
      cnameHost: "quizze.architektur-und-technologie.de",
    }),
    semioEmojiIndexHtmlVitePlugin(siteRoot),
...
  define: { "import.meta.vitest": "undefined" },
  server: {
    fs: { allow: [repoRoot] },
    proxy: Object.fromEntries(gatewayRoutes.map((route) => [route, { target: proctor, ws: true }])),
  },
```

## 2. Docker publishing

### 2.1 What `os-hub:publish` is

Launch row (`.vscode/launch.json:4683`, group `4_build`, order `206.1601`):

```json
    {
      "name": "🚚️publish🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1601
      }
    },
```

Neighbours (same group, following order): `📦️build🗄️os-hub` `206.159`, `📦️build-dev🗄️os-hub` `206.16`,
`🚚️publish🗄️os-hub` `206.1601`, `🛫️preflight-catalog🗄️os-hub` `206.160145`, `🚚️publish-catalog🗄️os-hub`
`206.16015`, `🚚️publish🌉️os-mcp` `206.1603`. Nx target (`🌎️hub/📦️packages/🦀️rust/📋️project.json`, project `os-hub`):

```json
    "publish": {
      "executor": "nx:run-commands",
      "cache": false,
      "outputs": ["{projectRoot}/dist/publish"],
      "dependsOn": ["build"],
      "options": {
        "cwd": "🌎️hub/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts publish"
      }
    }
```

`🌎️hub/📦️packages/🦀️rust/📜️script.ts` (the docstring is the authoritative statement):

```ts
/** 🚚️ Packages `build`'s staged release binary as a versioned, checksummed tarball under
 * `dist/publish` — the missing link between "a release binary compiles" and "an operator can be
 * handed one". Local artifact only: nothing is uploaded or pushed anywhere, and where the tarball
 * goes next is a deployment decision this repository does not take. Reached through the root
 * `bun ./📜️script.ts publish os-hub`. */
class PublishScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Select publish through Nx without additional arguments");
    packageNativeRelease({
      binary: join(this.root, "dist", "build", process.platform === "win32" ? "os-hub.exe" : "os-hub"),
      name: "os-hub",
      version: workspaceCargoVersion(this.repoRoot),
      output: join(this.root, "dist", "publish"),
    });
```

Root `📜️script.ts` `PublishScript` (line 15283) is a slice map, the place a new slice would be registered:

```ts
    /** 🚚️ Every slice that has a real, versioned, checksummed local deliverable today. Each target
     * depends on its own release build and writes `<project>/dist/publish/<name>-<version>-<platform>-<arch>.tar.gz`
     * plus a `.sha256`; none of them uploads or pushes anything anywhere. */
    const map: Record<string, string> = {
      "os-hub": "os-hub:publish",
      "os-mcp": "@semio-tech/framework-os-mcp-rs:publish",
    };
```

Root `package.json:256` `"publish": "bun nx run workspace:publish"`; root `📋️project.json:1762` target `publish`
runs `bun ./📜️script.ts publish` with `forwardAllArgs`.

### 2.2 The hub container image (never pushed)

Names and tags are local only: `🌎️hub/🧪️tests/🐳️docker-image/🟦️.ts` `export const HUB_IMAGE_REPOSITORY = "semio/os-hub";`
`docker build … --file 🌎️hub/Dockerfile --tag semio/os-hub:<tag> .` (fixture
`🌎️hub/🧫️fixtures/🐳️docker-image-v1/🔣️.json`). Nx: `os-hub-ts:docker-image-build` (depends on `workspace:prepare`) and
`os-hub-ts:docker-image-check`; launch rows `📦️docker-image-build🌎️hub🟦️` (order `900.0027`) and
`⚖️docker-image-check🌎️hub🟦️`. No `buildx`, no `--platform`, no `push`. Both verbs spawn `docker` as one argv without a
shell (cross-platform). The quiz `🚀️deploy/🟦️.ts` copies this design.

Dockerfile essentials (`🌎️hub/Dockerfile`):

```dockerfile
ARG RUST_IMAGE=rust:1-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim
...
ENV OS_HUB_DATA=/srv/semio-hub/data \
    OS_HUB_ADMIN_DIR=/srv/semio-hub/admin \
    OS_HUB_BIND=0.0.0.0 \
    OS_HUB_PORT=8787 \
    OS_HUB_MODE=production \
    OS_HUB_CREDENTIAL_SIGN_IN=true \
    OS_HUB_STORAGE_BACKEND=fs \
    OS_HUB_DIRECTORY_BACKEND=sqlite
...
HEALTHCHECK --interval=15s --timeout=5s --start-period=120s --retries=10 \
  CMD curl --fail --silent --show-error --max-time 4 \
      --header "X-Forwarded-Proto: https" \
      "http://127.0.0.1:${OS_HUB_PORT}/readyz" > /dev/null
...
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/os-hub"]
```

The hub is **not** zero-touch: the operator must seed the first user before the first start (`credential set`), state
`OS_HUB_ALLOWED_ORIGINS`, `OS_HUB_TRUSTED_FORWARDING=proxy` and `OS_HUB_ADMIN_SUBJECTS`, and install a proxy. The
README (`🌎️hub/README.md:530-534`, "Container image") also records: "the image has **not been built**".

### 2.3 Hub compose and proxy (no Caddy service)

`🌎️hub/compose.yaml` (`name: semio-hub`) publishes loopback only and leaves TLS to the host:

```yaml
    # 🌐️ Published on the host's loopback only: the hub speaks plain HTTP and always will, so it
    # belongs behind a TLS-terminating reverse proxy on this host (`🌎️hub/README.md` §
    # "TLS-terminating reverse proxy"). Inside the container the bind is `0.0.0.0` — a loopback bind
    # inside a container would make this mapping unreachable.
    ports:
      - "127.0.0.1:8787:8787"
    ...
    stop_signal: SIGTERM
    stop_grace_period: 30s
    restart: unless-stopped
```

The hub README's Caddy example (`🌎️hub/README.md:452-476`) is a host-installed Caddy forwarding to `127.0.0.1:8787`:

```caddyfile
hub.example.com {
	encode zstd gzip

	# WebSocket upgrades: Caddy v2 reverse_proxy passes them through unchanged,
	# but the read timeout must not cut an idle collaboration socket.
	reverse_proxy 127.0.0.1:8787 {
		transport http {
			read_timeout 0
			write_timeout 0
		}
		header_up X-Forwarded-Proto {scheme}
		header_up X-Forwarded-For {remote_host}
	}
	...
```

"Caddy obtains and renews the certificate itself; nothing else is required for TLS."

The only Caddy-plus-compose-service idea in the repo is a legacy Next.js file
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/Caddyfile`, no compose file beside it):

```caddyfile
# Caddy reverse proxy config for compose repo server.
# Replace {$DOMAIN} with your actual domain or use env var.

{$DOMAIN:repo.compose.dev} {
    reverse_proxy web:8787
}
```

Useful only for two idioms: env placeholder with default (`{$NAME:default}`) and the compose service DNS name as
upstream. There is **no** compose file in the repo with a `caddy` service, and `.devcontainer/docker-compose.yml`
is a single workspace container.

The quiz stack today (`🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/`) mirrors the hub: `compose.yaml` publishes
`127.0.0.1:8791:8791`, `Caddyfile` is copied to the host (`sudo cp Caddyfile /etc/caddy/Caddyfile && sudo systemctl
reload caddy` in the README) and points at `127.0.0.1:8791`. It builds locally (`build:` context `../../../..`, image
`semio/architecture-quiz:latest`), i.e. `docker compose up --build --detach` compiles the Rust workspace on the host.

## 3. CORS and production gating for a cross-origin client

### 3.1 Hub

`🌎️hub/🏗️bootstrap/🦀️.rs` `cors_middleware` / `apply_cors_headers` (verbatim):

```rust
async fn cors_middleware(State(policy): State<CrossOriginPolicyV1>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let origin = request.headers().get(axum::http::header::ORIGIN).cloned();
    if request.method() == axum::http::Method::OPTIONS {
        let mut response = Response::builder().status(StatusCode::NO_CONTENT).body(axum::body::Body::empty()).unwrap_or_default();
        apply_cors_headers(response.headers_mut(), origin.as_ref(), &policy);
        return response;
    }
    ...
fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&axum::http::HeaderValue>, policy: &CrossOriginPolicyV1) {
    if let Some(origin) = origin {
        headers.append(axum::http::header::VARY, axum::http::HeaderValue::from_static("Origin"));
        if origin.to_str().is_ok_and(|value| policy.admits(value)) {
            headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
            headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, axum::http::HeaderValue::from_static("true"));
        }
    }
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_METHODS, axum::http::HeaderValue::from_static("GET, POST, PUT, HEAD, OPTIONS"));
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, axum::http::HeaderValue::from_static("authorization, content-type"));
}
```

Policy table (`🌎️hub/README.md:315-319`): `OS_HUB_ALLOWED_ORIGINS` set ⇒ `allowlist` (echo the request's own origin +
credentials for exactly the listed origins); unset + loopback bind ⇒ `loopback-development` (any loopback origin on any
port); unset + network bind ⇒ `closed`. Entries are whole serialized origins, ASCII-case-insensitive, max 32, no
path/query/trailing slash/wildcard. A refused origin gets the normal answer without a grant; `Vary: Origin` always. A
CDN-hosted client is therefore an entry like `OS_HUB_ALLOWED_ORIGINS=https://s.example.com`. The hub sets no
`Access-Control-Max-Age` either.

### 3.2 Proctor (mirrors the hub; verified)

`🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs`, table and gate:

```rust
//! | `PROCTOR_MODE` | `development` or `production` | loopback bind → development, else production |
//! | `PROCTOR_ALLOWED_ORIGINS` | comma-separated `scheme://host[:port]` origins granted CORS | loopback bind → any loopback origin, else none |
//! | `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` (a TLS-terminating proxy is the only client) | `none` |
...
            ProctorMode::Production if !self.bind.is_loopback() && !matches!(self.origins, CrossOriginPolicy::Allowlist(_)) => Err(ConfigError(format!("a production proctor on a network interface requires {ALLOWED_ORIGINS} to name the origins its browsers are served from"))),
            ProctorMode::Production if !self.bind.is_loopback() && self.forwarding != Forwarding::TerminatingProxy => Err(ConfigError(format!("a production proctor on a network interface speaks cleartext HTTP and requires {TRUSTED_FORWARDING}=proxy, stating that a TLS-terminating reverse proxy is the only thing that reaches it"))),
```

`🎓️teaching/🛂️proctor/🔨️modules/🧩️instance/🦀️.rs` (outermost layer, before the settling middleware; `OPTIONS` never
reaches the router):

```rust
async fn gatekeeping(State(gate): State<Arc<Gate>>, request: Request, next: Next) -> Response {
    let proto = request.headers().get("x-forwarded-proto").and_then(|value| value.to_str().ok());
    if !gate.forwarding.secure(proto) {
        let mut response = StatusCode::FORBIDDEN.into_response();
        response.headers_mut().insert(REFUSAL_HEADER, HeaderValue::from_static("insecure-transport"));
        return response;
    }
    let origin = request.headers().get(header::ORIGIN).cloned();
    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        grant(response.headers_mut(), origin.as_ref(), &gate.origins);
        return response;
    }
    let mut response = next.run(request).await;
    grant(response.headers_mut(), origin.as_ref(), &gate.origins);
    response
}

fn grant(headers: &mut HeaderMap, origin: Option<&HeaderValue>, policy: &CrossOriginPolicy) {
    headers.remove(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    headers.remove(header::ACCESS_CONTROL_ALLOW_CREDENTIALS);
    if let Some(origin) = origin {
        headers.insert(header::VARY, HeaderValue::from_static("Origin"));
        if origin.to_str().is_ok_and(|value| policy.admits(value)) {
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
            headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
        }
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, HEAD, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type"));
}
```

Assessment for a credentials-free cross-origin quiz client on `https://quizzes.architektur-und-technologie.de`:

| Check | Result |
|---|---|
| Client requests | `proctorTransport` -> `fetchWithTimeout(url, { method, headers, body })`, no `credentials` option (default `same-origin`, so none cross-origin). Anonymous `ServerClient.headers()` adds `authorization`/`x-semio-capability` only when a credential is set (quiz sets none). `POST` adds `content-type: application/json` => non-simple => preflight for `POST /commands` and `POST /queries`. `GET /instance` is simple. |
| Preflight answer | `204` on any path, `access-control-allow-origin: <echo>` for an allow-listed origin, `access-control-allow-methods: GET, POST, HEAD, OPTIONS`, `access-control-allow-headers: content-type` (static, not an echo of `Access-Control-Request-Headers`). Enough for the quiz. `authorization` would fail; the quiz never sends it. |
| Credentials | ACAO is the echoed origin (never `*`), so `credentials: 'include'` would also work; irrelevant here. |
| Allowlist value | `https://quizzes.architektur-und-technologie.de` exactly (scheme + host, no slash, ASCII-case-insensitive compare). A dev site on `http://localhost:6061` is NOT admitted in a production network bind; add it to the comma list only for a staging run. |
| Preflight behind the proxy | The gate runs first: preflight must also carry `X-Forwarded-Proto: https`. Caddy adds it. A direct `curl -X OPTIONS` to the container port without the header gets `403 x-semio-refusal: insecure-transport`. |
| WebSockets | The quiz React client does not use WebSockets (searched); browsers do not apply CORS to them and the gate has no Origin check on upgrades. |
| Tests | `🧩️instance/🧪️tests/🔬️unit/🦀️.rs` `the_cross_origin_grant_follows_the_policy`; e2e `🧪️tests/🌐️end-to-end/🦀️.rs:381-384` sends `OPTIONS /commands` with `Origin: http://localhost:6061` and `Access-Control-Request-Method: POST`, expects `204` + echoed origin, and a foreign origin gets `204` without ACAO. Neither asserts a max-age. |
| **Gap** | No `Access-Control-Max-Age`. Without it browsers cache a preflight only for a few seconds (Chrome default 5 s), so a quiz session doubles its `POST` traffic (`record-answer` commands, projected queries). Add `access-control-max-age: 7200` (Chrome's cap) in `grant()` for `OPTIONS` responses and cover it in both tests. |
| Cache/`Vary` | `Vary: Origin` is emitted whenever an `Origin` header is present. Correct for a shared cache. |

## 4. Every reference to the old domain `quizze.architektur-und-technologie.de`

Search: `rg -i 'quizze\.'` over the working tree excluding `node_modules`, `.git` and the `.🧬semio/🦑️repo/🎫️tickets`
folders (tracked and untracked). No hit in `.vscode/launch.json`, root `package.json`, root `📋️project.json` or any
`.github` file. Also no other `architektur-und-technologie` string exists.

Site host, rename to `quizzes.architektur-und-technologie.de`:

| File:line | Content (short) |
|---|---|
| `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts:16` | docstring "the quiz website of `quizze.…`" |
| `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts:32` | `cnameHost: "quizze.architektur-und-technologie.de",` |
| `🎓️teaching/🏛️architecture/❓️quiz/🟦️.ts:1` | docstring "Entry of `quizze.…`" |
| `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json:5` | `description` |
| `🎓️teaching/🏛️architecture/❓️quiz/README.md:3, 42, 43, 58, 59, 72` | title line, deploy table, DNS block, first-start `curl` |
| `🎓️teaching/🏛️architecture/README.md:1` | H1 |
| `🎓️teaching/README.md:11, 20` | layout table and tree |
| `🎓️teaching/🛂️proctor/README.md:27` | `PROCTOR_ALLOWED_ORIGINS` example value (becomes the SITE origin `https://quizzes.architektur-und-technologie.de`) |

Proctor host (`proctor.quizzes.architektur-und-technologie.de`) and site origin (allowlist value):

| File:line | Content | New value |
|---|---|---|
| `…/❓️quiz/🚀️deploy/Caddyfile:1, 6` | header comment and site address | proctor host |
| `…/❓️quiz/🚀️deploy/Dockerfile:3, 9, 74` | header comment; `docker run … PROCTOR_ALLOWED_ORIGINS=https://quizze…`; ENV comment | comment: proctor; both `PROCTOR_ALLOWED_ORIGINS` values: site origin `https://quizzes.…` |
| `…/❓️quiz/🚀️deploy/compose.yaml:1, 26` | header comment; `PROCTOR_ALLOWED_ORIGINS: https://quizze…` | comment: proctor; value: site origin |
| `…/❓️quiz/🚀️deploy/🟦️.ts:1, 19` | docstring; `export const QUIZ_SITE_HOST = "quizze.architektur-und-technologie.de";` | split into a site host and a proctor host constant. The constant is consumed at `:110` (`x-forwarded-host` header) and `:141` (`PROCTOR_ALLOWED_ORIGINS=https://${QUIZ_SITE_HOST}`) |

Not the old domain, no rename required:

- `https://quizze.example` (fake `.example` origin) in `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🧪️tests/🔬️unit/🦀️.rs:33,34,36,57,59-65` and
  `🎓️teaching/🛂️proctor/🔨️modules/🧩️instance/🧪️tests/🔬️unit/🦀️.rs:42,45,46`. Optional consistency rename to
  `quizzes.example`.
- German "Quizze" (plural of "Quiz") in titles, the catalog title `Quizze Architektur und Technologie`, the site
  `<title>` `Quizze · Architektur und Technologie` (asserted by the image check as `SITE_TITLE`), i18n strings and
  fixtures. It is the language word, keep it.

Ticket folder hits (excluded by the brief, historical): `QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️{site-infra-report,
research-quiz-content-data,explore-ui-web-patterns,explore-server-product,design,content-report,closing-summary,
audit-content}.md`, `🎫️ticket.json`, `close_ticket.py`.

## 5. Recommended target design

Principle: copy the established mechanisms (Vite `cnameHost` markers, build-time `define` that only `build` injects,
hub-style production gating, per-project `📜️script.ts` verbs behind one-line nx targets, launch rows in the existing
groups) and fill only what the repo lacks: a self-contained compose stack with a Caddy service and a registry push.

### 5.1 Site on the CDN (`https://quizzes.architektur-und-technologie.de`)

Artifact contract (already produced by `bun nx run @teaching/architecture-quiz:build` once the host is renamed):
`📦️packages/🟦️typescript/dist/{index.html,404.html,🌐️.html,CNAME,.nojekyll,assets/,🖼️assets/,favicon*}`.
`base: "/"` is right for a domain root. `404.html` gives path routes (`/leaderboard`) a fallback on GitHub Pages.
DNS: `quizzes` CNAME to the Pages owner's `<owner>.github.io` (inferred, per GitHub Pages custom-domain rules).

Changes:

1. `🏗️builder/🌐️vite/🟦️.ts`: `cnameHost: "quizzes.architektur-und-technologie.de"`; switch to `defineConfig(({ command }) => …)`
   and follow the play idiom, so only a release build bakes the production proctor origin and dev stays same-origin
   through the existing proxy:

   ```ts
   define: { "import.meta.vitest": "undefined", ...(command === "build" ? { "import.meta.env.VITE_PROCTOR_URL": JSON.stringify(process.env.PROCTOR_URL ?? "https://proctor.quizzes.architektur-und-technologie.de") } : {}) },
   ```

   Keep the default host in one authored place. Play keeps its host in a schema-first JSON
   (`🔨️modules/🧩️runtime/🔣️.json` + `🧬️schema`), the quiz already has `QUIZ_SITE_HOST` in `🚀️deploy/🟦️.ts` but that module
   imports `node:child_process` and the owned-execution library, too heavy for a Vite config. Put the two hosts
   (`siteHost`, `proctorHost`) in one small pure module or JSON beside it and import that from `🚀️deploy/🟦️.ts`, the
   Vite config and the tests. Verify the file name against the taxonomy (`bun ./📜️script.ts verify taxonomy report
   --scope 🎓️teaching`) as the site-infra report did.
2. `🟦️.ts` entry: `mountQuiz(root, { proctor: <import.meta.env.VITE_PROCTOR_URL ?? "">, tenant: ARCHITECTURE_QUIZ_TENANT })`,
   read defensively like `readViteSEnv` (`try { … } catch { return "" }`), so Vitest/dev stay `""`. No other client
   change: `proctorTransport` already prefixes the base URL.
3. Optional local cross-origin rehearsal: the dev proctor is loopback, so its default policy already admits any loopback
   origin. `PROCTOR_URL=http://127.0.0.1:8791 bun nx run @teaching/architecture-quiz:build`, serve `dist` on another port.
4. Publishing verb, following hub `publish` semantics (local artifact only, no credentials in the repo): add
   `publish` to `@teaching/architecture-quiz` (`📋️project.json` target `publish`, `dependsOn: ["build"]`,
   `cache: false`, `"command": "bun ./📜️script.ts publish"`; `📜️script.ts` router `.register("publish", …)`; verb in
   `🚀️deploy/🟦️.ts`). It should do what the deleted `play-sites.yml` verify step did (`index.html`, `404.html`,
   `.nojekyll`, `CNAME` == site host, no `localhost` in the bundle, the bundle contains the proctor origin) and copy
   the result to `dist/pages/quizzes/` (play's `dist/pages/<page>` layout, git-ignored). Uploading stays the operator's
   step, or a workflow (below). Launch row `🚚️publish🎓️teaching🏛️architecture❓️quiz` in group `4_build`, order
   `11.15` (between `11.1` build and `11.2` proctor build; keep the emoji-prefix naming of `🚚️publish🗄️os-hub`).
   Root script `"publish:teaching:architecture-quiz"` next to the existing `build:teaching:architecture-quiz` entries
   (`package.json:204-214`), or a slice `"architecture-quiz"` in the root `PublishScript` map.
5. Optional CI (the repo's own history shows the shape, `de617a7c17f^:.github/workflows/gh-pages.yml`): a
   `workflow_dispatch` workflow with `permissions: pages: write, id-token: write` and
   `actions/upload-pages-artifact@v3` + `actions/deploy-pages@v4`, no stored secrets. Needs a dedicated Pages
   repository if `usalu/semio` itself must not become the Pages source. This is the only credential-free path.
6. Optional `_headers` (only if the CDN is Cloudflare Pages/Netlify): `/assets/*` immutable, everything else
   revalidate, mirroring `🛂️proctor/🔨️modules/🌐️site/🦀️.rs` constants. GitHub Pages ignores it. No CORS header is needed
   on the CDN: the browser calls the proctor, and the proctor grants the site origin.

### 5.2 Proctor on Docker (`https://proctor.quizzes.architektur-und-technologie.de`)

Deploy directory stays `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/` with the three fixed-filename contracts
(`**/Dockerfile` verified by `docker build`, `**/Caddyfile` by `caddy validate`, `**/compose.yaml` by
`docker compose config`, all in `taxonomy.json`) plus the existing operator module `🟦️.ts`. Reason to keep it there
rather than under `🛂️proctor`: one proctor process serves one catalog (`PROCTOR_CATALOG` is a single required file), and
the image bakes the architecture catalog and its quizzes. The proctor keeps no Dockerfile of its own (design §"Dockerfile-less").

**Dockerfile** (`🚀️deploy/Dockerfile`), proctor only:

- Drop the bun stage: remove `curl … bun.sh/install`, `bun install --frozen-lockfile`, the site `bun ./📜️script.ts build`,
  `COPY --from=builder …/dist /srv/quiz/site`, and `PROCTOR_SITE` (unset = "none (no site)" per the proctor README). The
  image shrinks by the whole JS toolchain and the cold build by the monorepo `bun install`.
- Keep `cargo build --release --locked --package teaching-proctor --bin proctor` with `CARGO_TARGET_DIR=/out`, the
  `/stage/content` tar of catalog + quizzes, `tini`, user `quiz`, `VOLUME /srv/quiz/data`, `HEALTHCHECK` on `/instance`
  with `X-Forwarded-Proto: https`, `ENTRYPOINT ["/usr/bin/tini","--","/usr/local/bin/proctor"]`, `CMD ["serve"]`.
- **Bake the safe defaults** so a bare `docker run`/`compose up` boots (zero-touch), replacing the current "refuses to
  start until the operator states them":

  ```dockerfile
  ENV PROCTOR_DATA=/srv/quiz/data \
      PROCTOR_CATALOG="/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json" \
      PROCTOR_BIND=0.0.0.0 \
      PROCTOR_PORT=8791 \
      PROCTOR_MODE=production \
      PROCTOR_ALLOWED_ORIGINS=https://quizzes.architektur-und-technologie.de \
      PROCTOR_TRUSTED_FORWARDING=proxy
  ```

  The three-statement production gate then holds by default, and stays overridable for staging.

**compose.yaml** — the whole host contract is `docker compose up -d`. Sketch (validate with `docker compose config`):

```yaml
name: architecture-quiz

services:
  proctor:
    image: ghcr.io/usalu/architecture-quiz-proctor:latest   # registry name is a decision, see 5.3
    build:
      context: ../../../..
      dockerfile: 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile
    expose:
      - "8791"
    environment:
      PROCTOR_ALLOWED_ORIGINS: ${PROCTOR_ALLOWED_ORIGINS:-https://quizzes.architektur-und-technologie.de}
    volumes:
      - proctor-data:/srv/quiz/data
    stop_signal: SIGTERM
    stop_grace_period: 30s
    restart: unless-stopped

  caddy:
    image: caddy:2
    ports:
      - "80:80"
      - "443:443"
      - "443:443/udp"
    environment:
      PROCTOR_HOST: ${PROCTOR_HOST:-proctor.quizzes.architektur-und-technologie.de}
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro
      - caddy-data:/data
      - caddy-config:/config
    depends_on:
      proctor:
        condition: service_healthy
    restart: unless-stopped

volumes:
  proctor-data:
  caddy-data:
  caddy-config:
```

Points that make it zero-touch and correct:

- `expose`, not `ports`, for the proctor: nothing but Caddy can reach the container port, so
  `PROCTOR_TRUSTED_FORWARDING=proxy` is true by construction (the hub README: "the setting is only true if it is
  true: nothing but the proxy may be able to reach the port"). The old loopback publish existed only for a host Caddy.
- `depends_on … service_healthy` uses the image `HEALTHCHECK` (Compose honours it); Caddy starts only when `/instance`
  answers.
- Volumes: `proctor-data` (the only durable state, `proctor.sqlite` + WAL/SHM; backup unit, proctor stopped) and
  `caddy-data` (ACME account and certificates; losing it re-issues certificates and can hit Let's Encrypt rate limits).
- `restart: unless-stopped`, `stop_grace_period: 30s` (already the repo idiom), `443/udp` for HTTP/3.
- The README's manual `sudo cp Caddyfile /etc/caddy/Caddyfile && sudo systemctl reload caddy` step disappears.
- Behaviour to verify on a Docker host (not verifiable here): with both `image:` and `build:`, whether a plain
  `docker compose up -d` pulls the registry image or builds when the tag is absent locally. If it builds, use
  `pull_policy: always` or document `docker compose pull` first.

**Caddyfile**, container form, reusing the `{$NAME:default}` idiom of the coordinator Caddyfile and the hub example's
forwarded-proto line:

```caddyfile
{$PROCTOR_HOST:proctor.quizzes.architektur-und-technologie.de} {
	encode zstd gzip
	header {
		Strict-Transport-Security "max-age=31536000"
		X-Content-Type-Options "nosniff"
		Referrer-Policy "no-referrer"
	}
	reverse_proxy proctor:8791 {
		header_up X-Forwarded-Proto {scheme}
	}
}
```

Caddy obtains and renews the certificate itself; DNS `A`/`AAAA` for the proctor host and ports 80/443 (and 443/udp)
are the only host prerequisites, plus Docker Engine with the compose plugin. No `Access-Control-*` header is set in
Caddy; the proctor's `grant()` owns CORS (a second source would duplicate `Vary`/ACAO).

**Proctor code** (small, Rust, tests first): add `access-control-max-age: 7200` to preflight (`OPTIONS`) answers in
`🧩️instance/🦀️.rs` `grant()` (or only in the `OPTIONS` branch of `gatekeeping`), extend the unit test
`the_cross_origin_grant_follows_the_policy` and the e2e preflight assertion. Nothing else in the proctor changes:
`PROCTOR_SITE` is already optional and an API-only proctor answers unknown GET with a JSON 404.

**Operator verbs** (`🚀️deploy/🟦️.ts`, wired through `📜️script.ts`, called by nx targets, registered in `launch.json`):

- Split constants: `QUIZ_SITE_HOST = "quizzes.architektur-und-technologie.de"`, new
  `QUIZ_PROCTOR_HOST = "proctor.quizzes.architektur-und-technologie.de"`; the image check sets
  `PROCTOR_ALLOWED_ORIGINS` (or relies on the baked default) with the site origin and sends
  `x-forwarded-host` with the proctor host (`probe`, line 110).
- `checkQuizImage`: remove the site assertions (`/` title, `/leaderboard` fallback, `SITE_TITLE`); add a cross-origin
  step: `OPTIONS /commands` and `OPTIONS /queries` with `Origin: https://quizzes.architektur-und-technologie.de`,
  `Access-Control-Request-Method: POST`, `Access-Control-Request-Headers: content-type`, expecting `204`, the echoed
  origin, the max-age; a foreign origin gets no ACAO; then a real `POST /queries` with that `Origin`.
- New `docker-stack-check`: `docker compose config`, then `docker compose up -d --wait` on a throw-away project name
  and volumes, wait for `service_healthy`, `docker compose down --volumes`. It proves the compose file, the Caddy
  wiring (Caddy will not get a public certificate locally, so assert the proctor via the compose network and the
  Caddyfile with `docker run --rm -v …:/etc/caddy/Caddyfile caddy:2 caddy validate --config /etc/caddy/Caddyfile`).
- Nx targets in `@teaching/architecture-quiz` `📋️project.json` next to `docker-image-build`/`docker-image-check`
  (`cache: false`, `forwardAllArgs: true`, cwd the package, command `bun ./📜️script.ts <verb>`), and matching launch
  rows in the existing order: `📦️build…🐳️docker-image` (4_build, 11.3), `⚖️gate…🐳️docker-image` (4_gate, 11.2). New
  rows: `🚚️publish🎓️teaching🏛️architecture❓️quiz🐳️docker-image` (4_build, `11.35`) and
  `⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-stack` (4_gate, `11.25`). Root `package.json` scripts follow the
  `build:teaching:architecture-quiz:docker-image` pattern.
- Add `"dependsOn": ["workspace:prepare"]` to `docker-image-build` like the hub target `os-hub-ts:docker-image-build`
  (the crates include generated sources that `workspace:prepare` publishes; the quiz target has no such dependency
  today, and the proctor crate's need is unverified).

### 5.3 Registry publishing (the only real gap)

The repo has no registry. The only host evidenced is GitHub (`origin https://github.com/usalu/semio.git`,
`package.json` `repository.url`), so the natural, credential-light choice is GHCR:
`ghcr.io/usalu/<name>` pushed with a developer's own `docker login ghcr.io` (or CI `GITHUB_TOKEN` with
`packages: write`). This is a recommendation, not a precedent. Follow the hub `publish` shape but add the push behind an
explicit flag so the default stays a local, verifiable artifact:

- `QUIZ_IMAGE_REPOSITORY` becomes registry-qualified for publishing; keep the local `semio/architecture-quiz` tag for
  `docker-image-build`/`-check`, then `docker tag` + `docker push` in a `docker-image-publish [--tag <t>]` verb.
  Tag with the workspace Cargo version (`workspaceCargoVersion`, as `PublishScript` does) plus `latest`.
- Multi-arch: no precedent (single-arch everywhere, `packageNativeRelease` names `<platform>-<arch>`). Cross-building this
  Rust workspace under QEMU is very slow. Publish `linux/amd64`, and keep the `build:` block in compose so an ARM host
  can `docker compose build`.
- Make the package public so a fresh host pulls without login (zero-touch).
- Do not add multi-registry or signing logic; the repo states there is no distribution and no secrets are stored.

### 5.4 What "zero-touch" requires (checklist)

1. Host: Docker Engine + compose plugin, DNS for the proctor host, 80/443/443-udp open. Nothing else installed.
2. `git clone … && cd 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy && docker compose up -d` works with no `.env`, no `sudo`,
   no file copy, no proxy install (or, without a clone: a published image and the two small files).
3. Defaults baked into the image and the compose file: production mode, site-origin allowlist, trusted forwarding,
   catalog and data paths, proctor host in Caddy. Overrides only through `${VAR:-default}`.
4. Persistence: named volumes `proctor-data` and `caddy-data`; documented stop-then-copy backup exactly as today
   (`docker compose stop proctor`, volume `architecture-quiz_proctor-data`).
5. Health and ordering: image `HEALTHCHECK`, Caddy `depends_on … service_healthy`, `restart: unless-stopped`,
   `stop_grace_period: 30s` so SIGTERM drains and SQLite checkpoints.
6. Automatic TLS: Caddy ACME, `caddy-data` persisted.
7. Cross-platform authoring rule: the operator verbs spawn `docker` as one argv without a shell (Windows/macOS/Linux),
   like the existing `docker()` helper; the devcontainer has docker-in-docker
   (`ghcr.io/devcontainers/features/docker-in-docker:2` in the bootstrap fixture).
8. Site: Vite build bakes the production proctor origin; deploy marker files exist; the artifact check fails the publish
   when they do not.
9. README (`❓️quiz/README.md` "Deploy"): rewrite around two artifacts, drop the Caddy install step, list both DNS
   records (site CNAME to Pages, proctor A/AAAA), and state the CORS contract (`PROCTOR_ALLOWED_ORIGINS` = site origin).

## 6. Open risks and unverified items

- The Docker image and Caddyfile have never been run (site-infra report §2: Docker daemon absent, Caddy not installed).
  The rust builder pulls the pinned nightly (`rust-toolchain.toml`: `nightly-2026-07-07`, wasm32 targets) on the first
  `cargo` call, needs the network, and is slow. The hub README records the same "unbuilt" status and 2.5 GiB/job.
- Whether the proctor crate needs the `workspace:prepare` generated sources inside the image build is unverified.
- GitHub Pages specifics (404 status on the SPA fallback, `_headers` ignored, default ACAO `*`, 1 GB limit) are
  inferred host behaviour, not repository facts.
- Compose `image`+`build` pull-or-build behaviour on `up -d` needs a run on a Docker host.
- The actual CDN vendor is not stated anywhere in the repo; the design above only relies on the portable static
  artifact (index/404/CNAME/.nojekyll) and on the proctor's CORS, not on a vendor feature.
- Root `.dockerignore` header names the old two-image setup (`🌎️hub/Dockerfile` and the quiz Dockerfile); still correct
  after the split, no change needed.
