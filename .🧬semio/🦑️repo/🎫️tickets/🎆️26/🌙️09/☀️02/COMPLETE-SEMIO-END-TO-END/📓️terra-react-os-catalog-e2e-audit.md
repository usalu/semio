# React OS Catalog-Wide End-to-End Audit

Scope: read-only source and current-worktree audit on 2026-09-03. No build, browser, hub, or test command was run. Counts below are observations, not claims that an artifact loads or that an E2E scenario passes.

## Outcome

React has a registered, host-mode `s` launch and a substantial generated catalog, but it is **not catalog-wide authenticated E2E ready**. The first production blocker is authority, not a missing React component: a plugin-supplied `os.open-artifact` relay can select `documentId`, `schema`, and `spaceId`; the React shell accepts them and uses the supplied space to construct its hub binding. The same shell still mints a bearer for an arbitrary `VITE_S_USER` email. An untrusted/compromised plugin must not be able to select a document scope, nor may a local CLI identity substitute for authenticated hub authorization.

The old collaboration harness messages saying that creation omits `documentId` and that React lacks a presence bar are stale test diagnostics. Current Space create/open producers include `documentId`, `schema`, and `spaceId`; current React renders `#s-presence-peers`. They are not evidence of a current failure and must be corrected before being used to triage a run.

## Source anchors and launch path

| Concern | Current source-backed state | Consequence |
| --- | --- | --- |
| Registered React host | `.vscode/launch.json:2460-2478` registers `🛠️dev🖥️s⚛️react` through `bun nx run @semio-tech/framework-os-dev:dev`, `S_OS_PORT=6070`, `SEMIO_PLUGIN=s`, `SEMIO_RENDERER=react`. User launches at `:2502-2547` use ports 6072/6073, a hub URL, user email, and distinct data dirs. Hub/MCP compounds are at `:7572-7597`. | There is a zero-touch *development* launch registration, including a two-browser-user shape. It is not an authenticated product launch. |
| Renderer selection | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/🟦️.ts:17-50` resolves the catalog boot and imports React unless renderer is `wgpu`; `⚙️vite.config.ts:8-170` carries the generated catalog and Vite identity/hub env. | React is the normal browser path, not a separate catalog implementation. |
| Host expansion | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2914-2942` recomputes a mismatched generated session against all catalog plugins/extensions; `ShellHost/🟦️.tsx:1574-1696,2374-2441` expands the `s` host graph, gates boot on primary `s`, and streams other modules. | A stale generated session file does not by itself restrict host-mode `s` to its old variant. Non-primary install failure is recorded rather than making host boot fail. |
| Dev-only module source | `kernel/🟦️.ts:2733-2843` provides `createDevPluginSource` from `/plugin-modules/watch` and extension source from `/extensions`; `dev/📜️script.ts:360-439` snapshots directories that merely contain a core WASM. | This is a local Vite/cache discovery mechanism, not the authenticated hub catalog/distribution source P4 requires. |
| Dynamic surface registry | `ShellHost/🟦️.tsx:4411-4449` builds `AppRouter` only from successfully loaded manifests; `:5251-5270` exposes “Open with…” only from those entries. | A filesystem artifact tree or a generated playground row cannot make a viewer/editor reachable. The owner module must load and contribute a valid manifest first. |

## Catalog and reachability census

### Package, module, and descriptor coverage

The generated package registry at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json` currently has 59 rows: 33 plugins and 26 extensions. Forty rows carry all three current `wasmSha256`, `coreWasmSha256`, and `descriptorSha256` fields; 19 do not. The generator deliberately permits an absent descriptor/WASM as a warning (`registry/📜️script.ts:1918-1993`), so this registry is neither a successful-build nor a trust proof.

| Layer | Declared/current count | What is actually established | Reachability status |
| --- | ---: | --- | --- |
| Registry packages | 59 = 33 plugins + 26 extensions | Full hash tuple: 40; incomplete: 19. | Declaration only; source code intentionally tolerates missing outputs. |
| Local `plugin-modules` cache | 57 logical dirs, plus `_shard` and `_vendor` | 56 have a `.core*.wasm` and component JS. `draw` and `layout` have no directory; `energy` has a directory but no core WASM. | The dev SSE scanner can only offer the 56 core rows; no portable/fresh build is implied. |
| Staged descriptors in that cache | 38 | The 19 missing staged descriptors are `block`, `flow-extension-bim`, `flow-extension-draw`, the five imperative extensions, `playbook`, `playbook-module-procedural`, the four process extensions, three sourcing modules, `stdio`, and `trinity`. | A module without descriptor cannot honestly contribute apps/artifact kinds. |
| Legacy cache residue | 31 dirs still contain `🟨️plugin-worker.js` | `dev/📜️script.ts:835-858` calls that file unconditionally stale under the shard-worker architecture. | A launch which reaches `preparePluginBuildTargets` sweeps it; the checked working cache itself is not a clean release inventory. |
| Extension install cache | 26 declared extension rows, current dirs observed | `dev/📜️script.ts:1012-1016` publishes them before hot-swap; Vite serves `/extensions`. | Local install material, not a signed/hub-authorized extension catalog. |

The package identifiers are complete in the generated registry: `animate, architect, block, cad, dag, demonstrator, draw, energy, fem, flow, forms, gis, imperative, layout, lowpoly, mathematical, norm, note, playbook, procedural, process, puzzle, raster, reasoning-mindmap, remodel, s, sequence, shooting, sourcing, stdio, trinity, vcs, writer`; and extensions `cad-extension-aec-building, cad-extension-aec-building-energy, cad-extension-aec-building-structure, cad-extension-spatial-shape, flow-extension-bim, flow-extension-brep, flow-extension-dictionary, flow-extension-draw, flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, flow-extension-text, imperative-extension-control, imperative-extension-effect, imperative-extension-logic, imperative-extension-math, imperative-extension-text, playbook-module-procedural, process-extension-concrete, process-extension-metal, process-extension-robotic, process-extension-wood, sourcing-module-beams, sourcing-module-slabs, sourcing-module-windows`.

### Playground/application declaration coverage

`registry/🤖️generated/🔣️playgrounds.json` and `🟦️playgrounds.ts` have 60 React/WGPU variants across 32 plugin IDs. (The registry’s `stdio` has no playground row.) Forty-one rows name an explicit app, representing 35 distinct editor app IDs; 52 rows declare sample data and 9 rows declare assets. The remaining 19 variants must fall back to whatever `editor` or first app a live module manifest exposes. No generated playground row declares a viewer app.

Explicit generated app IDs are:

`s.block.block2d`, `s.block.block3d`, `s.block.block5d`, `s.cad.cad`, `s.demonstrator.playground`, `s.energy.model`, `s.fem.fem2d`, `s.fem.fem3d`, `s.gis.gismap`, `s.gis.gisterrain`, `s.mathematical.equation`, `s.norm.din16798`, `s.norm.din18599`, `s.norm.din4108`, `s.norm.en1990` through `s.norm.en1999`, `s.norm.iso16757`, `s.norm.vdi3805`, `s.procedural.generation2d`, `s.procedural.generation3d`, `s.process.process3d`, `s.puzzle.puzzle2d`, `s.puzzle.puzzle3d`, `s.puzzle.puzzle5d`, `s.sourcing.curation`, `s.trinity.jack`, and `s.trinity.rewriting` (all `@1/*#editor`).

This is a launch catalog, not a surface taxonomy. The source tree has 87 artifact-kind trees, and each has both a `👁️viewer` and `✏️editor` path beneath `🏅️standards`; current inventory found no missing role path. The 87 identifiers are:

`2d, 3d, 5d, assembly, avi, bcf, binary, bmp, cad, csv, curation, dag, deflate, din16798, din18599, din4108, docx, drawing, dwg, dxf, en1990, en1991, en1992, en1993, en1994, en1995, en1996, en1997, en1998, en1999, epw, equation, flow, forms, generation2d, generation3d, gif, gisterrain, gismap, gltf, home, html, ifc, iso16757, jack, jpg, json, las, layout, lowpoly, md, model, mp3, mp4, note, obj, pdf, playbook, playground, ply, png, pptx, presentation, procedure, process3d, program, raster, remodeling, rewriting, s.space, semio, sequence, shooting, space, step, stl, svg, tiff, tsv, txt, vdi3805, vcs, wav, wires, writer, xlsx, xml, zip`.

Those paths do **not** register executable document codecs or live app manifests. The umbrella plan’s prior, still-relevant warning is that its registry/codec census found no executable codecs for its 48 declared schemas (`📋️master-plan.md:177`); this audit did not treat a directory name as a counterexample. Catalog-wide reachability requires a generated, validated mapping from each `(artifact kind, standard, subset, role)` to a trusted owner package and executable codec, then a loaded app manifest. It cannot be inferred from the 35 playground app IDs.

## User-flow trace

| Journey step | Existing path | Boundary or defect |
| --- | --- | --- |
| Plain/explicit open | `framework-os/🟦️.ts:31-121` parses `os.open-artifact`/`os.open-artifact-with`; explicit app selection must match the live `AppRouter`. `ShellHost:3042-3058` opens the resolved app and, if document/schema are present, starts the document. | Syntax/router validation is useful, but it is not authorization. `documentId`, `schema`, and optional `spaceId` pass through verbatim. |
| Space producer | `✏️s/🔌️plugins/🪐️space/.../create-artifact/🦀️.rs:43-45` and `.../open-artifact-with/🦀️.rs:20-26` now emit document, schema, and space fields; their nearby tests cover them. | This repairs the old omitted-document relay shape. It does not make the fields server-issued or membership-checked. |
| Hub document binding | `ShellHost:3312-3350` opens a worker with hub plus local-folder bindings. `:3049-3054` assigns relay `spaceId` to `openSpaceIdRef`, which feeds that hub binding. | **High:** a plugin controls a value that selects the hub scope. A new authenticated open-plan endpoint must replace this input. |
| Host/studio identity | `ShellHost:1107-1139` looks up manifest apps with `hostConfig.landingAppId`/`hostAppId`; the source itself documents at `:4104-4118` that these are aliases (`home`, `studio`), not canonical app IDs, and that `hostApp` is consequently undefined. Routes use that raw `hostAppId` at `:3240-3245`; host presence also compares it at `:4135-4141`. | **High runtime:** `/spaces/{id}/studio` cannot resolve the canonical Studio app through this path, and host-only presence heartbeat/chrome branches can stay inactive. This is distinct from the authority issue and can be repaired in parallel. |
| Identity/authentication | `ShellHost:1508-1515` invokes `DirectoryClient.mintSession(email)` when no cached token works; `framework-os/🟦️.ts:3989-3998` posts `{email}` to `/auth/sessions`. | **High:** current dev identity is arbitrary-email bearer minting. Local CLI env is policy/configuration, never proof of a hub principal. |
| Presence | Worker events are folded at `ShellHost:1401-1450`; `PresenceBar` is rendered at `:6860-6875`; per-document beats begin at `:4139-4185`. | Current intended invariant is document-wide roster keyed by structural document scope and authenticated actor. `surface` remains non-authoritative peer telemetry: it must not filter the roster or provide cross-space reach. |
| Public `(pack,spr)` restore | `backbone-worker.ts:783-815,851-901` rejects DB-private snapshot frames, validates bootstrap/control scope, assembles canonical public pair, atomically installs it, then permits tail commands. `RebootstrapRequired` rejects active transfer and closes for reconnect at `:866-874`. | Lower-level TS fallback has the right sequence, but this audit did not execute a hub/backend; it cannot prove a real backend serves a valid pair. P2-A/B/C authority remains a prerequisite. |
| Progress/cancel | Worker has `bootstrapProgress` at `backbone-worker.ts:723-759`, but casts it to `ArtifactEvent`; the declared union in `framework-os/🟦️.ts:605-617` and ShellHost event switch do not handle it. Close/unmount aborts work; no user cancellation control is exposed. | **Medium:** no visible, typed bootstrap progress/cancel state despite expensive recovery. |
| Renderer and assets | Vite serves declared static roots and some GIS tile proxies (`playgrounds.ts`, `vite.config.ts`). | Asset declarations are not catalog reachability and tile proxies add external-network behavior; offline/authorized GIS must have an explicit cached/denied state. |

## Deterministic blockers and classification

| Severity | Finding | Why it blocks the frontend goal | Required owner/order |
| --- | --- | --- |
| High | Relay-selected `spaceId`/document/schema and arbitrary-email bearer minting | A plugin can drive a user session toward a scope/schema that the hub did not authorize. It defeats revocation, cross-space isolation, and authentic actor binding before React can claim an authenticated catalog. | P4 auth/session and P4 descriptor/open-plan first. |
| High | Host aliases are compared as canonical app IDs | The live `s` manifest uses dialect-derived IDs while host config carries `home`/`studio`; current lookup makes `hostApp` undefined and direct studio switching/host-only heartbeat unavailable. | React host identity packet; independent of unfinished loader internals and safe to land in parallel. |
| High | No production `HubPluginSource`/trusted catalog-to-byte source | React currently discovers mutable Vite module directories and `/extensions`; it cannot consume a hub-authorized package/descriptor/hash decision. | P2 loader/catalog authority, then a small React adapter. |
| High | Taxonomy paths are not executable codec registration | 87 viewer/editor folder pairs and 35 generated editor apps leave unrepresented schemas/roles without a runtime codec/manifest proof. | Canonical catalog/codec capability packet after P2 authority. |
| Medium | Current cache is incomplete and stale-prone | As observed: no `draw`/`layout` module dirs, no `energy` core, 19 missing staged descriptors, and 31 legacy workers. `scanBuiltPluginModules` tests only for a core WASM. | Do not hardcode these as source failures; clean/rebuild and enforce a release manifest check. |
| Medium | P2-C progress/cancel has no React contract/UI | `bootstrapProgress` needs an unsafe cast and no ShellHost rendering; the only cancellation is document close/unmount. | React/P2-C bridge after P2 authenticated pair service. |
| Medium | Existing collab E2E is narrow and partly stale | `dev/📜️script.ts:2668-3350` prebuilds only `s` and `writer`, runs eight two-user steps, not every catalog item, and its lines 3017/3069 contain stale failure explanations. | Update the runner before using it as an acceptance gate. |
| Low | Generated session is mutable shared output | Current `dev/🤖️generated/🟦️session.ts` is invocation-specific; mismatch resolution mitigates it for host `s`. | Keep it out of production catalog authority and test it only as a dev boot input. |

## Smallest dependency-ordered implementation packet

1. **Finish P4 session authority, without changing React policy into authorization.** Replace the public email-mint route used by `DirectoryClient`/`ShellHost` with verified bearer/session acquisition. Preserve a zero-touch developer bootstrap only behind an explicitly local, non-network-exposed mode. Bind every HTTP and WS connection to the authenticated actor; revalidate on connect, command, reconnect, revoke, and kick.

2. **Add one hub-issued artifact open-plan contract.** Given only an artifact identity and requested role, the server revalidates membership and returns immutable descriptor identity, document scope, schema/pack-schema hash, chosen approved package/app surface, and public bootstrap/checkpoint references. Do not allow plugin relay fields to override any of those. `resolveArtifactOpeningRelay` should become local UI syntax/role selection only; `ShellHost` must call the plan before setting `openSpaceIdRef` or opening the worker.

3. **Repair the host-app identity seam in parallel.** Resolve host metadata aliases to canonical manifest app IDs once, then use the resolved objects/IDs for Studio routes, host session status, panel gating, and heartbeat. Add a minimal React/unit oracle for `/spaces/{id}/studio`, document-wide roster activation, and fallback rejection when either named host app is absent. Do not use `manifest.apps[0]` as a Studio fallback.

4. **Connect P2 trusted catalog/pair authority to a narrow browser `HubPluginSource`.** React consumes the server-selected package identity, descriptor digest, WASM/core hashes, and URL only after hash verification. It must not use `/plugin-modules/watch` or `/extensions` for authenticated remote workspaces. P2-A/B must resolve the 64 MiB public pair versus the 496 KiB DB blob ceiling with a bounded chunk-manifest CAS or coherent backend redesign; startup/catalog work must not imply that a 64 MiB artifact is durable today.

5. **Make catalog taxonomy executable and total.** Generate one checked mapping for all 59 package rows and each declared artifact `(kind, standard, subset, viewer/editor)` to descriptor, codec, owner, and allowed apps. Fail release/catalog validation for omitted descriptors, unbuilt outputs, unresolved app roles, duplicate surface claims, or absence of executable codec. Treat development cache discovery as separate from this manifest.

6. **Finish React P2-C control UX.** Add `bootstrapProgress` and typed failure/rebootstrap state to the worker wire union; render bounded received/total progress; expose explicit cancellation; discard old session state atomically; request a fresh server plan before re-Hello. Never render a stale result or carry old frontier/surface identity across document/space replacement.

7. **Repair and expand acceptance without broad catalog builds.** Correct stale collaboration assertions first. Add generated table-driven tests over every 60 variant, 59 package row, 35 declared app, and 87 artifact role paths. Then retain one real two-user authenticated socket scenario for a representative writable document and one forced lag/rebootstrap scenario; do not let it substitute for row-by-row catalog validation.

## Required independent oracles

- A language-neutral JSON fixture for the open-plan decision: same artifact/role inputs must produce a plan bound to one space/document/schema/package/digest/frontier; malicious relay substitution, expired/revoked bearer, wrong actor, and cross-space artifact must all fail without returning existence-sensitive data. Independently validate the fixture in a non-React client and the hub implementation.
- A second implementation of canonical descriptor/hash and `(pack,spr)` verification (for example, Rust host plus browser WebCrypto/JS) over good, swapped-package, swapped-descriptor, swapped-space, truncated, oversize, and stale-frontier vectors.
- A real-browser two-context oracle with independently authenticated users: both see the document-wide roster, only members receive document events, revoke/kick terminates and denies renewed plans, reconnect revalidates, and a forced P2-C lag replacement makes no old UI/mutation visible.
- A generated catalog oracle that reads descriptors/registered codecs rather than source directories and proves one approved viewer/editor decision (or an explicit unsupported result) for every declared artifact role. A bare folder must never count as a surface.

## Focused follow-up commands (not run here)

Use the registered VS Code compound `🧭️compound🖥️s👥️users🗄️os-hub` for interactive React/hub work once P4 is in place. Run small checks before any full catalog build:

```sh
bun nx run @semio-tech/plugin-registry:check
bun nx run @semio-tech/framework-os:test-quick
bun nx run @semio-tech/framework-renderer-react:test-quick
bun nx run @semio-tech/framework-os-dev:collab-e2e
```

The final command intentionally starts a real hub, builds only `s` and `writer`, launches two Vite servers and Playwright (`dev/📜️script.ts:2749-2850,3212-3350`). It needs the Rust wasm toolchain, the repo-local Playwright Chromium cache, free ports, and a usable SQLite/default hub backend. It is not a Docker proof, a full catalog build, an auth proof, or a P2-C forced-rebootstrap proof.

## Exit criteria

React catalog-wide readiness requires all of the following, not a green shell boot: canonical host-app resolution for direct Studio routing and document-wide presence; authenticated non-dev session issuance; server-authoritative open plans and revocation checks; verified trusted package/descriptor/pair acquisition; an executable-codec/surface decision for every catalog row; typed restore progress/cancel/rebootstrap behavior; and independent real-browser two-user evidence for permitted, revoked, lagged, and restarted paths. Until then, the sharp blocker is **untrusted client relay data selecting hub document scope/schema under an arbitrary-email bearer model**; the immediately parallel deterministic runtime blocker is the `home`/`studio` alias-versus-canonical-app-ID mismatch.
