# 📓️ Report — work package F (depiction, React package, stories gallery, launch entries)

Ticket `2026/10/02/QUIZ-PETS`. `PR` = `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react`, `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder. All results below are from runs on 2026-10-02 on the Windows host (bun 1.4.2).

## 1. What exists

| File | What |
|---|---|
| `PR/📦️packages/🟦️typescript/package.json` | `@semio-tech/pets-react`; exports `.` → `./🟦️.tsx`, `./🎨️.css` → `../../🎨️.css`; deps `@semio-tech/pets` (workspace), `react`, `react-dom`; dev deps all already installed (`gl-matrix 3.4.3`, `jsdom`, `@testing-library/react`, `aria-query` + types for sibling G, `vite`, `vitest`, `@vitejs/plugin-react`, `typescript`) |
| `PR/📦️packages/🟦️typescript/tsconfig.json` | extends the root; `paths` for `@semio-tech/pets-react` and `@semio-tech/pets`; includes modules, stories, test config and `P/🧪️tests/*/🟦️.tsx` by glob |
| `PR/📦️packages/🟦️typescript/📋️project.json` | nx `@semio-tech/pets-react`: `test`, `test-quick`, `test-long`, `test-exhaustive`, `typecheck`, `dev` (continuous, not cached, no port baked in) |
| `PR/📦️packages/🟦️typescript/📜️script.ts` | `test [level]` (vitest, jsdom), `typecheck` (tsc), `dev` (`runViteBunxDev`, `PETS_STORIES_PORT`, default 6071, fixed port) |
| `PR/📦️packages/🟦️typescript/🟦️.tsx` | glue re-export |
| `PR/🟦️.tsx` | barrel: loads `🎨️.css`, exports `depict`, `paint`, `depictionMarkup`, `Depiction` and the schema names the public API mentions; sibling G added its exports (survey, pacing, layer) |
| `PR/🎨️.css` | `.pet-layer`, `.pet`, paint classes, forced-colours and print rules; plain CSS, nothing animates |
| `PR/🔨️modules/🖌️depiction/🟦️.ts` | `depict(species, document = globalThis.document)`, `paint(depiction, actorFrame, scale = 1)`, `depictionMarkup(species, actorFrame)` |
| `PR/🧪️tests/🎚️config/🟦️.ts` | vitest config, jsdom, `include: ["../../../../🧪️tests/*/🟦️.tsx"]` (glob), aliases for both packages |
| `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` | 11 tests (see §3) |
| `P/🔮️oracles/🔣️.json` | anchored edit: oracle `pets-react-gl-matrix` (`hostPath` = the react package) |
| `PR/📖️stories/{🌐️.html, 🟦️.tsx, 🎨️.css}` | the gallery (see §2) |
| `PR/🏗️builder/🌐️vite/🟦️.ts` | Vite dev config of the gallery and the menagerie module |
| `.claude/launch.json` | entries `pets-stories` (6071) and `architecture-pets-stories` (6072, `PETS_MENAGERIE=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`) |
| `.vscode/🧩️launch.seed.jsonc` | six rows after `🛠️dev❓️quiz⚛️react🪁️typecheck`: `🧪️test🐾️pets🟦️` 213.69, `🧪️test🐾️pets⚛️react` 213.7, `🧪️test🐾️pets🦀️` 213.71, `🛠️dev🐾️pets⚛️react🪁️typecheck` 213.72, `🛠️dev🐾️pets📖️stories` 213.73, `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` 213.74 |
| `.vscode/launch.json` | regenerated (never hand-edited) |
| `TK/reference_menagerie.ts`, `TK/stories_probe.mjs`, `TK/stories_appear_probe.mjs` | ticket tools (inputs, kept) |

## 2. The gallery and how to open it

- VS Code: launch row `🛠️dev🐾️pets📖️stories` (sample menagerie, http://127.0.0.1:6071/) or `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` (http://127.0.0.1:6072/). Claude preview: `pets-stories`, `architecture-pets-stories`. CLI: `bun nx run @semio-tech/pets-react:dev` (root alias `bun run dev:pets:stories`).
- `PETS_STORIES_PORT` (default 6071), `PETS_MENAGERIE` = path relative to the repository root to a module or a JSON document; default `P/🧫️fixtures/🧬️schema-conformance/🔣️.json`. The gallery shows the **outermost valid menagerie** it finds in what the file exports (breadth first, validated with the core's `menagerieIssues`): a default export, a named export such as `ARCHITECTURE_MENAGERIE`, a JSON menagerie, or the `menagerie` member of the conformance vectors. When nothing valid is found it lists the issues of the outermost invalid one; when the file does not exist it says so, and it reloads by itself when the file appears.
- **Species** view: controls (mood, lid, blink with the core's `lidAt`, face left, speed incl. pause, size), a roster to jump from, then every species on a light and on a dark ground: at rest (large) and every clip looping (clips that do not loop replay after a pause), each with its declared `size` box, the ground line and a caption (`clip id · seconds · loop/once · activities that play it`). Poses come from the core (`sampleClip`, `solveRig`, `restPose`), the drawing from the product's `depict`/`paint`; pupils follow the pointer (`lookOffset`). Only specimens on screen are painted.
- **Sandbox** view: mock cards (`data-pet-surface`) with a button on an edge, a note (`data-pet-keepout`) on an edge and text, a scrolling shelf of cards, and `<PetLayer>` from the barrel with switches for scene, mode (`off` unmounts), quiet, capacity, scale, seed, plus "Poke a pet", "Scroll the cards", "Rearrange the cards", "Take a card away". Theme: "Dark page" toggles `.dark` on the root. Labels in English and German (language switch, starts in the browser's language).

## 3. Verification (commands and their real results)

| Command | Result |
|---|---|
| `bun install` (repo root) | `Checked 1652 installs across 1846 packages (no changes)` — sibling A had already installed after my manifest landed; `bun.lock` lists `@semio-tech/pets-react` |
| `bun ./📜️script.ts test` in the package dir | `Test Files 4 passed (4)`, `Tests 69 passed (69)` (mine: `🖌️pet-depiction` 11 passed; the others are G's) |
| `bun ./📜️script.ts test pet-depiction` | `Test Files 1 passed (1)`, `Tests 11 passed (11)` |
| `bun ./📜️script.ts typecheck` | exit 0, no error |
| `bun nx run @semio-tech/pets-react:test` / `:typecheck` | exit 0 (`4 passed`, `70 passed` — G added a test in between) / exit 0 |
| `bun nx run @semio-tech/pets-react:dev` (background), `curl http://127.0.0.1:6071/` | `HTTP 200 text/html`; server log without any error line |
| `node TK/stories_probe.mjs --url http://127.0.0.1:6071/ --out TK/🗑️generated/wp-f --name final` | exit 0: 31 responses, all 200 (document, menagerie module, stories, core modules, depiction, survey, pacing, layer, both CSS files); no failed request, no page error, no console error. 3 species, 71 `svg.pet`; first pet `transform: translate(128px, 168px) scale(4, 4)`, computed `transform-origin: 0px 0px`, `overflow: visible`, body fill `rgb(30, 155, 141)` from `--pet-body`; mirrored `scale(-4, 4)`; German title `Tiergeschichten`, `lang="de"`. Sandbox: 8 surfaces, `.pet-layer` with `aria-hidden="true"`, three actors (`blobby`, `hoppy` on Card 1 at y 316.89, `floaty` hovering); after "Rearrange" both riders moved with their card (x 238.73 → 946.73, y 316.89 → 252.89) |
| Screenshots looked at | `TK/🗑️generated/wp-f/{reference-species, reference-species-mirrored-dark, reference-sandbox, sample-species-full, sample-sandbox, sample-sandbox-rearranged, final-species-mirrored-dark}.png`: rigs correct on both grounds, mirrored pets stay inside their size box (mirroring around the feet), lids and mouth follow the sliders, pets stand on the card edge and avoid the button |
| `PETS_STORIES_PORT=6072 PETS_MENAGERIE=🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts bun nx run @semio-tech/pets-react:dev` + probe | server up on 6072 (env passes through nx); page says `No file at 🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts` (sibling J has not written it yet) |
| `node TK/stories_appear_probe.mjs --url http://127.0.0.1:6072/ --file TK/🗑️generated/wp-f/late_menagerie.ts` | exit 0: notice → file written → 1 story, heading `Pet stories · Late` → file removed → notice again |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🐾️pets"` | `clean=true errors=0 warnings=0` |
| `NX_PLUGIN_NO_TIMEOUTS=true bun nx run @semio-tech/plugin-registry:generate` | exit 0, `.vscode/launch.json regenerated`; 6 pets rows present, project pickers list both pets packages |
| `bun nx run @semio-tech/plugin-registry:check-generated` | exit 0 |
| `bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` (test harness) | no finding names the oracle registry or my suite (the only pets findings were three steps of A's `🧬️schema-conformance` feature, in progress at the time) |

All servers I started are stopped (ports 6071 and 6072 free; only processes whose command line was my Vite config / `pets-react:dev` were killed).

What the suite `🖌️pet-depiction` checks: root attributes and no `style`/`script`/id/`[style]` inside; group order and the face above the named part or last; geometry, paint classes and stroke widths of all four shape kinds; a CSS rule for every paint class, the layer and actor rules, forced-colours and print hiding, no animation/transition/`@apply`/`@import`; palette custom properties, CSSOM `transform` (mirrored, scaled, rounded) and `opacity`; every matrix string against gl-matrix `mat2d` composing the same rig (≤ 0.0005), a limb tip and a pupil carried through the written transforms by gl-matrix; lids and mouth; **no mutation record** for an unchanged or sub-rounding frame and exactly the expected records for single changes (jsdom `MutationObserver`); no `setAttribute("style")`, `innerHTML`, `outerHTML`, `cssText`; `depictionMarkup` equal to the live tree (node equality and serialisation).

## 4. Decisions and deviations from the design

1. **`.pet` CSS gets four declarations the design does not list**: `width: 1px; height: 1px; overflow: visible; transform-origin: 0 0` (and `stroke-linejoin/linecap: round` on `.pet`, inherited). Without a box and a corner origin, `scale(-s, s)` mirrors an outer `<svg>` around the centre of its 300×150 default box instead of the feet. Verified in Chromium.
2. **Overridable paints**: ink, paper and pupil are `var(--pet-ink, var(--foreground, #001117))`, `var(--pet-paper, #f7f3e3)`, `var(--pet-pupil, #001117)` — the design's values by default, customisable by a host.
3. **Rounding**: matrices, pupil offsets, lid squash, mouth bend and opacity to 3 decimals; the actor's place to 2 decimals (0.01 px). The mouth carries `pet-fill-none pet-stroke-ink`.
4. **Signatures**: `depict(species, document = globalThis.document)` and `paint(depiction, frame, scale = 1)` satisfy both the brief and design §6.1. `Depiction` is a plain readonly record (`element`, `followers`, `sockets`, `pupils`, `mouth`, `painted`).
5. **`depictionMarkup`** carries no place, opacity or palette (those exist only in the CSSOM; a `style` attribute is forbidden). A page sets the palette custom properties on an ancestor.
6. **The barrel imports `./🎨️.css`** (as the quiz barrel does), so a host that lazy-loads `@semio-tech/pets-react` gets the styles with that chunk; the `./🎨️.css` export exists as well.
7. **Menagerie hand-over**: the document's inline module script imports `{ source, origin }` from the builder's virtual module `pets-stories:menagerie` and calls `mountStories`. This avoids an ambient `.d.ts` and lets the gallery accept named exports (design §9 exports `ARCHITECTURE_MENAGERIE`, not a default).
8. **No in-memory fallback to `TK/reference-species.json` in product code**: a missing file gives a readable notice instead, and `TK/reference_menagerie.ts` wraps the reference species for use through `PETS_MENAGERIE`. A's sample fixture exists now, so the default works.
9. **`PetLayer` is imported statically from the barrel** (G's layer and E's stage landed while I worked); the guarded lazy import was removed again.
10. **The builder is not in the package's `tsconfig` include**: it imports two shared Vite helpers (`semioEmojiIndexHtmlVitePlugin`, `semioServeCloseVitePlugin`) from the ui styling builder, whose module tree has unrelated type errors from concurrent work. It is exercised by running it.
11. **Dependency cache per port** (`node_modules/.vite/stories-<port>` in the package) so the two galleries never rewrite each other's cache or the repo-wide one.
12. **No `P/🧫️fixtures/🖌️pet-depiction`**: the suite builds its species inline and takes its frames from gl-matrix.

## 5. Open

- `architecture-pets-stories` shows "No file at …" until sibling J writes `🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts`; it reloads by itself when the file appears.
- **Port 6072** is also listed as a user port of the `space` React playground (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`: `user_ports = { react = [6072, 6073] }`). The design fixes 6072; with `fixedPort` the script reports "already in use — dev server appears to be running" and exits 0 when anything answers there. Pick another port if both are used together.
- The seed row `🧪️test🐾️pets🦀️` runs `@semio-tech/pets-rs:test`, which exists only after phase 2.
- In the species view the look direction is computed in rig coordinates and ignores the rotation of the eye's bone; good enough for judging pupils.
- `TK/🗑️generated/wp-f/` holds the logs, reports and screenshots of this work package; it goes when the ticket's generated folder is deleted.
