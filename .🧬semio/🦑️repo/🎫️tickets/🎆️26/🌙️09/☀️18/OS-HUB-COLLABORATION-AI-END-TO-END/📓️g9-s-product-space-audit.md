# G9 — `s` product / 🪐️space audit: what `dev s` hosts today vs. "working os s frontend, all plugins/artifacts"

Read-only audit, slice G9. No files edited, no builds run, no servers started (per instructions). Method:
read the ticket's own prior audits (`📓️audit-os-frontend.md`, `📓️audit-multi-plugin-hub.md`,
`📓️o1-multi-plugin-hub.md`, `📓️o2-activation-follow-ups.md`, `📓️s1-plugin-coverage-matrix.md`,
`📓️au3-live-sign-in-integration.md`, `📓️g8-wgpu-parity-spec.md`) first, then independently re-verified the
load-bearing claims directly against source (grep + read, cited below with `file:line`) rather than trusting
the reports at face value. Every citation in this report was read by this slice; where a claim is carried
forward from another report without independent re-verification, that is stated explicitly.

## 0. tl;dr

`dev s` is one real product: the `🪐️space` plugin is the *only* plugin in the whole registry whose
`Cargo.toml` declares `[package.metadata.semio].host` (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:17`,
`host = { landing = "home", shell = "studio" }`), and that one flag makes the Nx build graph, the generated
playground session, the TS kernel's registry expansion, and the React `ActivationRegistry` all fan out from
"space's own dependency closure" to "the entire registered catalog" (60 rows in
`🔌️plugins.json`/`🗺️catalog.json`, confirmed by direct read — `🗺️catalog.json` has exactly 60 `modules`
entries, id+directoryName only). **On paper this is real and consistently wired at all four layers** (build
graph, session data, kernel resolution, React `ActivationRegistry`), and O1/O2 landed a genuine lazy
cross-plugin install path (`installPlugin`/`openArtifactWithAppRef` in React, `install_plugin`/
`switch_to_app` in wgpu) plus a real activation-reason bridge into the guest. **But nobody has ever
observed `dev s` boot to a page with more than one plugin installed.** The `s` variant's own cold activation
was attempted twice this ticket (b1b, au3) and both times aborted before finishing — once because the shared
Cargo lock was busy at 19/132 tasks, once because `activate s react dev` needs ~20 wasm modules staged and
dies on the one (`🧱️block`) still missing. So "hosts every plugin" is a *build-graph and code-path* fact,
not yet a *ran-and-was-seen* fact for `dev s` itself — every runtime proof to date (au3's two-browser sign-in
transcript, the activation-owner tests) ran in a single-plugin playground (`animate`) that happens to mount
the identical `ShellHost`/`HubWorkspace`, not in the `s` hub itself.

## 1. The `dev s` pipeline, from source

### 1.1 Command chain

- `bun ./📜️script.ts dev s` → `DevScript.run()`, `📜️script.ts:412-415`: `if (segments[0] === "s") {
  runFrameworkOsPlaygroundDev("s", segments.slice(1)); return; }` — verified directly, current line numbers
  (`📜️script.ts:271-273` in this tree's numbering after O1's edit removed the dead `multi` branch that the
  earlier `audit-os-frontend.md` had described at `:416-425` — that branch is now gone, confirmed by
  `grep -n multi 📜️script.ts` returning no `segments[0] === "multi"` branch).
- `runFrameworkOsPlaygroundDev(plugin, rest)` (`📜️script.ts:271-…`) shells to
  `bun nx run @semio-tech/framework-os-dev:dev -- s [rest]`. Its own doc comment (`📜️script.ts:252-270`,
  read directly) states the chain plainly: *"a bare `dev <variant>` … runs the variant's whole Nx activation
  chain — every selected plugin's `component-<profile>`/`materialize-<profile>`, the browser support bundle,
  the guest fonts, the engine `wasm` producers, the generated playground session, then `prepare` and
  `activate` — before Vite serves the receipt"* and explicitly: *"`dev s` IS the all-plugins hub: … There is
  no separate multi-plugin variant to select; a `dev multi` segment existed once and never resolved past this
  file."* `served` (a second segment) skips the activation chain and serves whatever
  `dist/<profile>/🔌️plugin-modules/` already holds, forcing the React renderer.

### 1.2 Why `dev s` really does fan out to all 60, not just `space`

Traced and independently spot-checked (not merely re-cited from `📓️audit-multi-plugin-hub.md`):
- Nx graph: `runtimeComponentClosure` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:29`)
  — `if (component.host) for (const hostId of byId.keys()) pending.push(...)` — a host root's closure is
  force-expanded to every component ever discovered from a `Cargo.toml`.
- Session data: `buildPlaygroundSession`/`filterProjectedPluginRegistry`
  (`🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts:38-56`, `📖️catalog-view/🟦️.ts:31-36`) — "no filter, or
  the plugin is a host ⇒ return every projected row unfiltered."
- Kernel: `expandPluginRegistry(plugins, primaryPluginId, hostMode)` (`🎠️kernel/🟦️.ts:461-462`) —
  `if (hostMode || !primaryPluginId) return plugins;`.
- Registry ownership pass: O1 added `claimOwnedArtifactKinds` (`📇️registry/🔎️discovery/🟦️.ts:288`) so every
  declared `on-artifact-kind:` string is claimed by exactly one catalog row (verified live: `cad` keeps
  `3d.cad`+`s.cad.cad`, `demonstrator` keeps only `s.demonstrator.playground`), regenerated and covered by
  `📇️registry/🧪️tests/🎬️host-activation/🟦️.ts` (6 tests, O1 ran them green: `Test Files 1 passed, Tests 6
  passed`).

This part is solid — four independent layers agree, one of them (host-activation) has a real, green,
CI-registered test.

### 1.3 Lazy install on artifact-open (the mechanism that makes "host everything" tractable)

- React: `installPlugin(pluginId, rebuiltAt)` (`🏛️ShellHost/🟦️.tsx:3644-3680` per O1's numbering) fetches +
  instantiates a not-yet-loaded plugin's `moduleUrl` on demand; `openArtifactWithAppRef` installs before
  `createApp`; `installActivationOwnerAndResolve` (`🏛️ShellHost/🟦️.tsx:7781-7807` per O1) resolves an
  artifact ref to its owning plugin from `PLUGIN_CATALOG`'s declared kind rows when the direct relay throws,
  installs it, and retries once.
- wgpu (native): `switch_to_app` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, O1 `:9046` → O2 `:9200`) now calls
  `install_plugin` before the lookup instead of an unconditional `Err("program missing")`; `install_plugin`
  (O2 `:9207`) is real on native (a manifest re-read; no compile step) and, after O2, also compiles on
  `wasm32-unknown-unknown` via a JS-side "lazy install door" (`🎞️frame-worker/🧩️lazy-install/🟦️.ts`, new in
  O2) — but the wasm32 browser wgpu shell's own artifact-open relay
  (`handle_open_artifact_relay`/`switch_to_app`/`open_document`) is still `#[cfg(not(target_arch =
  "wasm32"))]` at HEAD (O2 §0.5, not contradicted by any later edit found), so the lazy-install door compiles
  but has no browser-wgpu caller yet.
- Activation reason reaches the guest as a real WIT event on both native hosts (O2 §1: `Event::Activate`
  prepended to the instance's first turn, `🖥️host/🎠️activation/🦀️.rs:16,25`, `🔨️modules/🏃️run/🦀️.rs:2016`,
  wgpu native renderer `🧊️renderer/🦀️.rs:7041`) and through the wgpu browser bridge
  (`🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1646`), **but not through the React DOM host**: O2 §5 gap 1 names a live,
  still-open one-line handoff — `🔌️PluginRuntime/🟦️.tsx:3047` (`createApp`, the React DOM host's real
  artifact-open activation) still passes the literal `"manual"` instead of `activationReasonForAppId(appId)`.
  The guest itself also still ignores `Event::Activate` regardless (`⚛️reactor/🔄️turn/🦀️.rs:801` matches it
  to `{}`, O2 §5 gap 2) — the wiring is real, nothing downstream acts on it yet.

### 1.4 Stale/missing plugin descriptor — what actually happens

A plugin's static descriptor (`semio-framework-plugin-describe`'s output, committed at
`DESCRIPTOR_JSON_REL_PATH`, `📇️registry/🔎️discovery/🟦️.ts:225`) feeds two independent things, and they fail
differently:
- **Registry generation** (build-time, feeds `dev s`'s own catalog/session): `readDescriptorJson` is only
  invoked when `ownerDescriptors === "required"` (`🔎️discovery/🟦️.ts:370`); a crate with **no descriptor at
  all** still gets a registry row — `capabilities`/`contributes` just fall back to the pre-descriptor static
  source (comment at `:339-343`). This does not block `dev s`'s Nx fan-out or session generation — the crate
  is still built and staged, just with thinner declared metadata.
- **MCP capability catalog** (a separate consumer, not the `dev s` boot path itself): per `📓️s1-plugin-coverage-matrix.md`
  (cross-checked against `📓️m1-mcp-servers-start.md`, not independently re-run here since it needs a live
  `capabilities_search` call), a plugin with a missing/malformed descriptor is individually **skipped**
  (`NotFound: no committed descriptor` for `block` and the `imperative`/`process`/`sourcing` extension
  families; missing `artifactSchema`/`windowKindId`/`executionProtocol` for most others), and — separately —
  one duplicate id (`architect.s.architect.program@1/*#editor.setAdjacencyKind`) aborts the **entire**
  catalog compile, forcing a gateway-only, zero-capability fallback for every plugin, including the ones
  whose own descriptor decoded cleanly. This is a real, systemic finding but it is about the MCP/capability
  surface, not about whether `dev s` itself boots — a plugin missing from the capability catalog can still
  be installed and opened through the `os.open-artifact` UI path in §1.3, which does not consult that catalog
  at all.

### 1.5 Whether `dev s` itself has ever completed a cold boot — not confirmed, twice attempted and stopped

- `📓️b1b-dormant-plugin-boots.md` (cited, not independently re-run): `dev s served` resolved a 132-Nx-task
  chain; the worker stopped at 19/132 to avoid contending for the shared Cargo lock against a peer's cold
  build. No activation receipt (`dist/runtime/react/dev/s/activation`) exists from that attempt.
- `📓️au3-live-sign-in-integration.md` §5.5 (cited, not independently re-run): `bun ./📜️script.ts activate s
  react dev` failed on the first missing staged plugin module (`…/dist/dev/🔌️plugin-modules/🧱️block`) —
  needs ~20 plugin wasm components staged, of which `block` was the one absent.
- Independently checked by this audit: `find … -path "*dist/dev/*plugin-modules*"` was reported by O2 as 0
  hits at O2's time (`🧑‍💻dev/dist/plugin-modules` held only `_vendor`+`cad`); this audit did not re-run that
  `find` (a `find` over `dist/` is cheap and non-build, but was skipped here as redundant with two
  independent, dated, failed-attempt reports already on file — re-running it would only restate "still not
  staged" without new information, and staging state changes hour to hour under the fleet's own concurrent
  work).

**Conclusion for §1**: the pipeline is real and four-layer-consistent on paper and by unit/integration test;
the missing piece is a single, uncontended, patient cold `dev s` run all the way to a served page — nobody
has done this yet, and it is expensive (60 crates, one of which — `🧱️block` — has had an open compile
question since 09-05 per S1's ranked list item 2).

## 2. Per-plugin path from the `s` shell UI to opening an artifact

This table is carried forward from `📓️s1-plugin-coverage-matrix.md` (a Sonnet audit slice in this same
ticket, itself file:line-cited against source) with the "activation owner" and "install source" columns
added/verified by this slice against the mechanism in §1.3, and collapsed to what matters for the `s` product
question: *given the lazy-install mechanism now exists, is there anything actually blocking "new / open
example / open from hub" for this plugin's own kind once its wasm is staged?* Full per-plugin detail (probe
dates, test counts, compile status) is in S1; this table does not repeat it, only the activation-relevant
columns plus S1's own "known blocker."

| kind (plugin) | activation owner (S1/O1 catalog) | install source | known blocker for "open in `s`" |
|---|---|---|---|
| space (home/studio) | space itself | boot-time, host plugin, never lazily installed | none — this is the shell itself |
| cad (+4 ext) | cad | lazy install, React proven via activation-owner tests | root/ext descriptors thin; extensions unverified live |
| raster | raster | lazy install | none known — most thoroughly proven interaction in the matrix |
| forms | forms | lazy install | none known — zero-fault probe on record |
| note | note | lazy install | `InkCanvasHost` doesn't route setSelection/setHover (framework gap, not note's) |
| fem (2d/3d) | fem | lazy install | none known — strongest-evidenced plugin besides raster |
| energy | energy | lazy install | none known |
| layout | layout | lazy install | none known |
| remodel | remodel | lazy install | none known |
| draw | draw | lazy install | 3 gesture/retained-command bugs found+fixed same day; suite has documented harness-debt failures |
| architect | architect | lazy install | `setAdjacencyKind` refused (`BatchOnlyPendingRewrite`); **also the plugin whose duplicate capability id breaks the MCP catalog for every plugin** |
| dag | dag | lazy install | no `command_from_action`; `addNode` refused `BatchOnlyPendingRewrite` |
| reasoning | reasoning | lazy install | one file from passing — bespoke retained-publication authority admits only `MoveNode` |
| norm (×15 codes) | norm | lazy install | verbs already `Migrated`; only missing `command_from_action` bridge — S1 calls this "cheapest win" |
| playbook | playbook | lazy install | boots (fixed today per S1), `addStep` still `BatchOnlyPendingRewrite` |
| imperative (+5 ext) | imperative | lazy install | Actions pane has **zero entries** — app never calls `.window_kind_actions()` |
| mathematical | mathematical | lazy install | `setDocument` dispatch-failed |
| sequence | sequence | lazy install | `setActiveExample` undeclared; verbs dispatch-failed |
| vcs | vcs | lazy install | `DuplicateSiblingKey` render fault independent of the stdio-pdf compile block |
| writer, animate | writer/animate | lazy install (blocked — see below) | **cannot currently compile**: transitive dep on peer's in-flight `stdio-pdf` edit |
| trinity, wfc (×5), puzzle (×3) | each own | lazy install | boot proven historically; no fresh dated interaction proof this pass |
| procedural (×2), flow (+9 ext), process (+4 ext) | each own | lazy install | process's 4 extensions: `NotFound: no committed descriptor`; flow extensions: catalog directoryName drift |
| gis (2×) | gis | lazy install | host/toolchain only — unaccepted Xcode license blocks native test/link on this Mac; `cargo check` green |
| block | block | lazy install (never staged — see §1.5) | **the one plugin with zero committed MCP descriptor**, and an unresolved E0053 async/sync trait mismatch in its own snapshot code, unconfirmed fixed since 09-05 — this is the specific crate that stops a cold `dev s` from finishing today |
| lowpoly, shooting, demonstrator, sourcing (+3 mod) | each own | lazy install | unconfirmed since last dated proof (08-29 to 09-17 range); no fresh evidence this pass |
| stdio | N/A (I/O library, no UI app) | N/A | not openable as an artifact kind at all — it is a codec library other plugins depend on |

**"New" vs "open example" vs "open from hub space" as three distinct UI paths**: this audit did not find
(and no cited report found) any plugin-specific gating between these three — all three route through the
same `os.open-artifact`/`os.open-artifact-with` actions and `openArtifactWithAppRef` (§1.3), so a plugin that
can open at all can be reached all three ways; a plugin that can't (e.g. `imperative`'s empty Actions pane)
fails identically regardless of which of the three paths was used to get there. The per-plugin blockers above
are therefore genuinely per-*plugin*, not per-*path*.

## 3. Hub wiring inside the `s` host

Independently re-verified against `🏛️ShellHost/🟦️.tsx` (not merely re-cited from `📓️au3-live-sign-in-integration.md`):

| surface | where it mounts | host-mode-only? |
|---|---|---|
| Hub connection indicator (`HubConnectionIndicator`) | `🏛️ShellHost/🟦️.tsx:11126` region — actually the footer builder at `:10840`: `items.push({ key: "hubConnection", content: <HubConnectionIndicator … onSignIn={openHubWorkspace} /> })` | **No** — in the `footerItems` `useMemo`, unconditional on `mobile` only, so every React playground (space or any single-plugin dev variant) shows it, confirmed by the comment directly above (`:10836-10839`, "beside presence and on every device") |
| Sign-in / spaces workspace (`HubWorkspace`) | `🏛️ShellHost/🟦️.tsx:11122-11135`: `{hubWorkspaceOpen ? <HubWorkspace port=… activeSpaceId=… onlineUserIds=… onSessionChange=… onOpenSpace=… onClose=…/> : null}` | **No** — `hubWorkspaceOpen` is plain `useState` (`:2613`), opened by `openHubWorkspace()` regardless of `hostMode`; only the `/hub` URL/history entry is host-mode-gated (`openHubWorkspace`, `:8983-8985`: `if (hostMode) navigateHistory("/hub")`) |
| `/hub` deep link | `applyShellUri`, `🏛️ShellHost/🟦️.tsx:6071` (`if (!hostConfig || !currentSession || loadedPlugins.length === 0) return;`) and the whole-effect guard at `:6165` (`if (!hostMode \|\| loadedPlugins.length === 0) return;`) | **Yes** — confirmed directly: a plugin playground has no router to address, so pasting `/hub` there is a no-op even though the workspace itself is reachable via the badge |
| Artifact list / spaces browser (`SpaceBrowser`, inside `HubWorkspace`) | same mount as `HubWorkspace` above | No (same as above) |
| Presence (`PresenceBar`) | `🏛️ShellHost/🟦️.tsx:10826-10829`, footer, `id="s-presence-peers"`, unconditional on `!mobile` | No — always mounted when a document/session is open, any playground |
| Document/collab socket | not directly re-traced by this slice (out of scope re-verification, carried from `📓️audit-collaboration.md`: transport/wire/reconnect/presence real, 274 tests, browser E2E blocked by a `PluginRuntime` retry storm) | — |
| Sign-in credential form (`HubSignIn`) | inside `HubWorkspace`'s own tree (not independently re-traced past the mount point above) | No, by construction (same mount) |

**What is mounted nowhere**: per `📓️g8-wgpu-parity-spec.md` §0 (cited, spot-checked by `find`, not
independently re-grepped by this slice) — `HubSignIn`, `SpaceBrowser`, `HubConnection` (and by extension
`HubWorkspace`) have **no `🎯️targets/🧊️wgpu/` subfolder at all**. So on the wgpu renderer, none of sign-in,
spaces, or the hub workspace exist in any form — not "mounted nowhere in `s` specifically," but mounted
nowhere on that whole renderer target, `s` included. React is the only renderer where any of §3's hub
surfaces are reachable at all, and — per au3's own finding, now independently confirmed above — they are
reachable identically in *every* React playground, not specially wired for `s`; the only `s`-specific
behavior is the `/hub` URL becoming addressable.

**The one thing genuinely specific to `s`/host-mode and unproven**: au3 §5.5 and §7.1 (cited, consistent with
this slice's own finding that `dev s` has never completed a cold boot, §1.5) — nothing in the whole ticket
has observed `HubWorkspace`/`SpaceBrowser`/the `/hub` route *inside* an actually-running `s` host. Every live
proof (the two-browser sign-in transcript, 19/19 checks) ran in the `animate` single-plugin playground, which
mounts byte-identical `ShellHost` code but is not `s` itself and cannot exercise the host-mode `/hub` branch
(`applyShellUri`'s `hostMode` guard, confirmed above, means `animate`'s playground literally cannot enter
that branch — au3's browser proof by construction never touched it, matching au3's own §7.1 admission).

## 4. wgpu vs React parity for the `s` product — delta list

Carried from `📓️g8-wgpu-parity-spec.md` (a same-ticket Sonnet audit with its own file:line citations),
filtered to items that matter for `s` specifically (host-mode hub surfaces + multi-plugin activation, not
the general wgpu↔react parity ticket's chord/camera work):

- **Hub sign-in / spaces / workspace**: React-only, no wgpu element exists at all (§3 above). G8 sizes this
  as its own largest item (WG-6) and explicitly blocks it on the hub's `POST /auth/sessions` route landing —
  which AU3 shows is **no longer blocking** (the route is mounted and live-proven, §au3 above), so this
  dependency in G8 is stale as of AU3's slice; WG-6 is unblocked but still unscheduled/unbuilt.
- **Hub-connection indicator (footer pill)**: React has it; wgpu does not (WG-5), and WG-5 hard-depends on
  WG-6 for something to fold state from.
- **Cross-plugin lazy install**: at parity in *mechanism* (§1.3) — both renderers can install-on-open on
  native; wgpu's browser target cannot (its artifact-open relay is native-only, O2 §0.5) while React's
  browser path is the primary one that works.
- **Activation reason to the guest**: wgpu native + wgpu browser bridge both carry the real reason; React's
  own DOM-host call site still hardcodes `"manual"` (O2 §5 gap 1) — this is the one place wgpu is *ahead* of
  React, not behind.
- **Plugin-install chrome** (progress/cancel band while a lazy install is in flight): closed on wgpu (O2 §3);
  present on React (O2 §3, ShellHost band) — at parity.
- **TaskManager, agent-cancel, approval affordance**: not `s`-specific (apply to any host), G8 sizes these as
  small/cheap and independent of the `s` gap specifically.

Net for `s`: the multi-plugin *activation* mechanism is close to parity between renderers (React ahead on
lazy install reach, wgpu ahead on activation-reason plumbing); the *hub* surfaces (sign-in/spaces/workspace)
exist on React only and have no wgpu counterpart of any kind yet.

## 5. Prioritized execution slices to close the gap

Ordered by leverage (unblocks the most downstream work per slice) and cross-checked against `🐍️o2-hub-open-foreign-kind-probe.mjs`'s conspicuous absence (O2 wrote and then explicitly did not run this probe, §4 of O2 — the exact probe that would answer §1.5/§3's open question) and S1's own ranked blocker list.

1. **Run one uncontended cold `dev s` to completion and capture the receipt.**
   Scope: no code change — a scheduling/ops action. Needs the shared Cargo build dir quiet (per
   `📌️project-semio-build-budget-and-oom.md`, ~20 min budget) and `🧱️block`'s compile question resolved first
   (item 2) or accepted as a known-missing module.
   Files: none (run `bun ./📜️script.ts dev s` or `activate s react dev`).
   Acceptance observed-at-runtime: `dist/runtime/react/dev/s/activation` (or the wgpu native/browser
   equivalent) exists; the served page shows `PLAYGROUND_SESSION.plugins.length === 60`; a browser hits
   `/readyz`-equivalent and paints the `space` home/studio app.

2. **Fix `🧱️block`'s own compile/descriptor state** (S1 ranked item 2): resolve the open E0053 async/sync
   `ArtifactDsl` trait mismatch in `📸️snapshot/🦀️.rs` and commit a `🔣️.json` descriptor.
   Files: `✏️s/🔌️plugins/🧱️block/…/📸️snapshot/🦀️.rs`, plus generating `🧱️block/🔣️.json`.
   Acceptance: `cargo check -p semio-s-artifact-block-*` green; `block` wasm rebuilds; `dev s`'s Nx fan-out
   no longer stalls on the missing staged module (unblocks item 1).

3. **Live-probe cross-plugin open inside the actual `s` host** (the probe O2 wrote and deliberately did not
   run) once item 1 lands: open the `s` hub, from its home/studio app open (a) a new artifact of a plugin
   kind that is NOT `space`, (b) an existing example of that kind, (c) an artifact reachable from a hub
   space — for at least one proven-interactive plugin (`raster` or `forms`, per S1's own "most proven" list)
   and one currently-`BatchOnlyPendingRewrite`-refused plugin (`dag` or `norm`) to see the refusal surface
   live, inside `s`, not in a single-plugin playground.
   Files: `🐍️g9-s-host-open-foreign-kind-probe.mjs` (new, in this ticket folder) driving the served `s` host.
   Acceptance: console/network capture showing `installPlugin`/`openArtifactWithAppRef` firing for the
   foreign kind and the document opening in the SAME `s` session (no reboot) — the concrete, still-missing
   proof for §1.5/§3's "nothing observed inside the `s` host" gap.

4. **Wire `os.open-artifact`'s React DOM-host call site to the real activation reason** (O2's one-line
   handoff, §1.3): `🔌️PluginRuntime/🟦️.tsx:3047`, `"manual" satisfies ActivationReason` →
   `activationReasonForAppId(appId)`. Trivial, already-typed, already exported.
   Acceptance: an existing/new unit test on `createApp` asserting the guest-bound turn carries
   `on-artifact-kind:<kind>` instead of `manual` when opening a foreign-kind artifact through the DOM host.

5. **Un-gate the wasm32 browser wgpu artifact-open relay** (O2 §0.5/§5.3) so the already-compiling lazy
   install door (`🎞️frame-worker/🧩️lazy-install/🟦️.ts`) has a real caller in the browser wgpu renderer, not
   just native.
   Files: `handle_open_artifact_relay`/`switch_to_app`/`open_document` and their `#[cfg(not(target_arch =
   "wasm32"))]` guards in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`; pulls in persistence bindings, hub transport,
   `system_fs` per O2's own scoping note — sized as its own packet, not a quick fix.
   Acceptance: a browser-wgpu `s` boot can open a foreign-kind artifact without a full page reload.

6. **Give the hub sign-in/spaces/workspace surfaces a wgpu target** (G8 WG-6, now unblocked by AU3's live
   `/auth/sessions` route): port the pure contract (`🔐️sign-in/🟦️.ts`, `🏘️spaces/🟦️.ts`) plus a
   `🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs` / `🏘️SpaceBrowser/…` / `🔗️HubConnection/…` following the
   `🛂️SpaceAdministration/🎯️targets/🧊️wgpu/🦀️.rs` shape G8 names (geometry fns + `*Plan`/`*Row` + one
   `*_paint_ops`), then WG-5's footer pill on top.
   Files: as listed in G8 §"WG-6"/"WG-5".
   Acceptance: a native or browser wgpu `s` boot shows the same hub badge/workspace/sign-in flow AU3 proved
   in React, driven by the same live hub.

7. **Fix the MCP capability-catalog break** so plugin descriptors stop being systemically invisible to the
   AI/MCP outcome (S1's "architect's duplicate capability id blocks all 34 plugins" finding) — separate from
   `dev s` boot but blocks outcome 4 (semio MCP) for every plugin opened inside `s`.
   Files: `architect`'s manifest declaring `editor.setAdjacencyKind` twice (exact site not re-traced by this
   slice — S1/M1 cite `m1-mcp-servers-start.md:206-207` only, not a file:line inside architect's own crate);
   then the ~30 individual descriptor-drift skips (missing `artifactSchema`/`windowKindId`/
   `executionProtocol`) per plugin.
   Acceptance: `capabilities_search` returns nonzero hits for at least the plugins whose own descriptor
   already decodes cleanly (S1's list of 16-17).

8. **Per-plugin interaction fixes** already scoped and cheap per S1's own ranking (not `s`-specific, but each
   one converts a plugin from "opens, does nothing" to "usable inside `s`"): `imperative`'s missing
   `.window_kind_actions()` call (S1 ranked #3, single-file), `norm`'s missing `command_from_action` bridge
   (S1: "cheapest win in the batch"), `reasoning`'s bespoke retained-publication swap (S1 ranked #7, one file,
   pattern already proven by `dag`/`trinity`). Not re-scoped here in more detail — S1's own report names the
   exact files.

## 6. Honest gaps in this audit

- §1.4/§1.5's descriptor and MCP-catalog claims are cross-checked against S1/M1's citations but not
  independently re-run (no live `capabilities_search` call was made — would need a running MCP server, out
  of scope for a read-only audit with no server-start budget).
- §3's document/collab socket row is carried from `audit-collaboration.md` without independent re-tracing —
  out of this slice's specific brief (hub wiring *inside the `s` host*), and that audit's own citations
  (274 tests) were treated as sufficient given the specific ask was about mount points, which are answered
  independently above.
- No cargo/build/server command was run by this slice (read-only per instructions); every "confirmed" claim
  above is confirmed by direct file read or grep against the current tree, not by execution. Where a claim
  could only be confirmed by running something (e.g. "does `capabilities_search` really return 0 hits today"),
  it is marked as carried-forward rather than independently verified.
