# RB1 — release builds, production hub posture, production `s` bundle, MCP install

Slice RB1 of ticket 26/09/18. Started 2026-09-21 04:00 (session 7). Machine: 10 cores / 32 GiB,
load average 165 at start, 165 GiB free on `/System/Volumes/Data`.

Scope (G15's ranked slices 1–3 plus the Dockerfile/README follow-ups):
1. `os-hub` release build → a real binary under the product's `dist`.
2. `os-mcp:build-release` → a real `semio-os-mcp`, proven over stdio from a `.mcp.json`-shaped config,
   with the exact Claude Desktop / Claude Code blocks written into `🌎️hub/README.md` and the root `README.md`.
3. A RELEASE `os-hub` in **production** mode on a **non-loopback** bind (port 7661) behind the documented
   allowlist/proxy posture, observed: `/healthz`, `/readyz`, credential sign-in, SIGTERM drain, restart reuse,
   zero stack overflows.
4. `build-s-react-release` end to end through the wasm mutex, then the production bundle served on 6081 and
   booted headless against 7661.
5. `🌎️hub/Dockerfile` / `compose.yaml` consistency with the release path.
6. Every new command registered as a `📜️script.ts` verb + `📋️project.json` target + `.vscode/launch.json` row.

**State left running at 12:00** (reuse, do not rebuild):

- **Production hub, release binary, pid 89834** on `http://192.168.178.70:7661`, data root
  `.🧬semio/🌐hub/rb1-prod`, jco-1.34 catalog, `OS_HUB_ALLOWED_ORIGINS=http://192.168.178.70:6081`.
  Every request needs `-H 'X-Forwarded-Proto: https'` or it is `403`. Verified answering at 12:00.
  Stop with `kill -TERM $(cat 🗑️generated/rb1-runtime/hub.pid)`.
- **`build-s-react-release` run 2 queued** on the wasm mutex (script pid 70953), position 4 behind
  `tc3c` (holding since 13:44), `pz1`, `s10`. Detached — it will run and write
  `🗑️generated/rb1-s-release-build.txt` without me. Run 1 failed; §4 has both its cause and the two
  defects that had to be fixed before a rerun was worth anything.

Captures: `🗑️generated/rb1-*.txt`. Key transcripts are inlined in the appendix at the end of this file so
they survive a `🗑️generated` sweep.

---

## 1. `os-hub` release build

**Run 1 — 2026-09-21 04:06:58 → 05:41:52, 5694 s (95 min), exit 1.** Not a defect in my slice's code:
the build died compiling `semio-framework-plugin` on three errors from a peer's mid-flight
`LocalizedLabel` refactor —

```
error[E0308]: 🔌️plugin/🦀️.rs:25352  expected `Vec<String>`, found `Vec<LocalizedLabel>`
error[E0599]: 🔌️plugin/🦀️.rs:11107  no method named `texts_mut` found for &mut LocalizedLabel
error[E0599]: 🔌️plugin/🦀️.rs:28898  no method named `texts_mut` found for LocalizedLabel
error: could not compile `semio-framework-plugin` (lib) due to 3 previous errors
```

`texts_mut` now exists (`🧰️framework/🔨️modules/🌐️locale/🏷️label/🦀️.rs:139`), so this was the
"Concurrent Cargo Workspace Churn" failure mode: a 95-minute release build is long enough for a peer
to land a half-finished cross-crate rename underneath it. Rerun launched 10:34 after the tree healed.

**Run 2 — 2026-09-21 10:34:33 → 11:08:53, 2360 s (39 min 20 s), exit 0.** Cargo's own figure:
`Finished \`release\` profile [optimized] target(s) in 39m 13s`.

| what | value |
|---|---|
| artifact | `🌎️hub/📦️packages/🦀️rust/dist/build/os-hub` |
| size | **145 256 624 bytes** (138.5 MiB), Mach-O 64-bit arm64, ad-hoc signed, `codesign --verify` → *satisfies its Designated Requirement* |
| prerequisite | `os-hub-admin` SPA → `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist`, 8 s |

This is the first `os-hub` release binary that has ever existed (G15 #1 / hub README: *"Never proven
at release profile… every hub boot recorded so far used `build-dev`"*). §3 runs it.

### What the build command actually is

`bun nx run os-hub:build` → `📋️project.json` `build` → `cd 🌎️hub/📦️packages/🦀️rust && bun ./📜️script.ts build`
→ `BuildScript` (`📜️script.ts:12088-12094`) → `buildCargoArtifacts(Cargo.toml, ["--release","--bin","os-hub"], …, {output:"dist/build"})`
then `signExecutableForDistribution`. Two things worth recording:

- **A private `CARGO_TARGET_DIR` is not something you pass to it.** `buildCargoArtifacts`
  (`🏗️native-build/🟦️.ts:99`) *overwrites* `CARGO_TARGET_DIR` with a fresh `mkdtemp` under the
  staging parent for every run. Preamble rule 25 is therefore satisfied automatically — and my
  slice's `target-rb1` directory is never written by this path, which is why it does not exist. The
  shared `build.build-dir` still supplies the intermediates, so a rerun is not from scratch.
- **The nx wrapper is the slow part, not the work.** `bun nx run os-hub-admin:build` sat 4 m 34 s at
  0 % CPU re-printing "Creating project graph nodes" and never reached the command (load average was
  165). The same target's underlying verb,
  `cd 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript && bun ./📜️script.ts build`, finished in
  **8 seconds**. Preamble rule 16, confirmed again; every step of `📜️rb1-release-builds.sh` now calls
  the project's own verb directly with `NX_DAEMON=false`.

## 2. `os-mcp:build-release` and the MCP install docs

### The build

| what | value |
|---|---|
| command | `cd 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust && bun ./📜️script.ts build-release` |
| started / ended | 2026-09-21 05:41:52 → 05:47:54 |
| wall time | **362 s**, exit 0 |
| artifact | `…/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp` |
| size | **33 831 536 bytes** (32.3 MiB), Mach-O arm64, ad-hoc signed by `signExecutableForDistribution` |

This is the first time this target has ever been executed (G15 #3: "the command exists and
type-checks; it has never been executed"). The binary's own usage line answers, so the
`profiles.build-release === "release"` contract in `🎚️config/🧱️binary-gate.json` is real.

### Proof that a user's copy-pasted config works

`🐍️rb1-mcp-install-probe.ts` (new) reads a config file in the exact shape the READMEs hand a user —
absolute path to `dist/build-release/semio-os-mcp`, `stdio`, `--folder`, `--scopes` — spawns whatever
that config names and drives `initialize` → `notifications/initialized` → `tools/list` → four
policy-gated `tools/call`s. Capture: `🗑️generated/rb1-mcp-install-probe.txt`, three configs
(`rb1-mcp-config-{documented,fake-only,corrected}.json`), 20 PASS / 1 FAIL (the FAIL is the
deliberate negative control below).

```
PASS  initialize answers   protocolVersion=2025-11-25 serverInfo={"name":"semio-os-mcp","version":"0.1.0"}
PASS  declares a tools capability  ["prompts","resources","tools"]
PASS  tools/list answers   27 tools
      action_cancel, action_invoke, action_prepare, artifact_create, artifact_export, artifact_open,
      artifact_snapshot, artifact_validate, capabilities_describe, capabilities_search,
      context_resolve, history_redo, history_undo, inference_approve, inference_cancel,
      inference_events, inference_get, inference_list, inference_run, inference_submit, job_cancel,
      job_get, transaction_begin, transaction_commit, transaction_rollback, ui_focus, ui_reveal
PASS  inference_list / artifact_create / inference_run all reach the tool (INPUT_INVALID, not PERMISSION_DENIED)
```

### Defect found by running it: three of the seven documented `--scopes` names are not scope names

`MCP_SCOPE_TABLE` (`🌉️mcp/🛡️policy/🦀️.rs:26-53`) is the whole set of `--scopes` names.
`artifact.open`, `artifact.create` and `inference.run` — three of the seven in **both** worked configs
of the MCP README, and therefore in everything copied from them — are **not in it**. `expand_scope`
(`:59-66`) passes an unknown entry through *literally* as a `CapabilityId`, by design, so they are
accepted silently and grant nothing.

Measured, with the release binary, as a negative control
(`rb1-mcp-config-fake-only.json`, `--scopes workspace.read,artifact.open,artifact.create,inference.run`):

```
FAIL  inference_run is not PERMISSION_DENIED   PERMISSION_DENIED — gate: jobs.spawn
```

while `tools/list` still advertises all 27 tools. The documented seven-name string happens to work
anyway, because `artifact.write` (a real name, also in the string) expands to
`artifacts.write, jobs.spawn` and covers what `inference.run` was meant to. So the shipped config is
not broken — it is three-sevenths decorative, and a user who trims it to the names that *look*
sufficient loses a whole tool family with no error anywhere.

**Fixed** by replacing the string everywhere with real names,
`workspace.read,artifact.read,artifact.write,inference.execute,ui.observe,ui.control`, verified green
in the same probe (`rb1-mcp-config-corrected.json`), and by documenting the trap and the full legal
list in the MCP README so it cannot drift back.

### The config blocks now in the docs

- `🧰️framework/…/🌉️mcp/README.md` — both worked blocks corrected + a new paragraph naming every legal
  `--scopes` value and the silent-passthrough trap, with the measured `PERMISSION_DENIED` evidence.
- `🌎️hub/README.md` § **For the people who will use this hub** (new, 79 lines) — what an operator
  hands a browser user and an AI-client user: the `s` bundle origin and `OS_HUB_ALLOWED_ORIGINS`, the
  `build-release` command, both copy-paste configs, and the `--hub`/`--space`/`--credential-file`
  swap with where the credential file actually comes from (the *Agent delegations* panel, not the
  operator).
- root `README.md` § 🛍️ Products — D2's missing edit, now in the tree (see §6).

## 3. Production hub posture, observed

`📜️rb1-production-runtime.sh` (new; replaces P4's loopback `📜️p4-production-runtime.sh` for the
network case). Capture `🗑️generated/rb1-production-runtime.txt`, full transcript in the appendix.
**Everything below was run against the release binary from §1**, copied (`rm` + `cp` + `codesign`,
never an in-place overwrite) out of `dist/build`, started as a plain process — no `cargo run`, no
`bun nx`, no dev launcher, no inherited fd 3 — on **192.168.178.70:7661** (`ipconfig getifaddr en0`),
data root `.🧬semio/🌐hub/rb1-prod`, mode `700`.

G15: *"Network-bound production has never been attempted by anyone, live or otherwise."* It has now.

### The three refusals, each naming itself

```
no OS_HUB_ALLOWED_ORIGINS    -> UnsafeAuthConfiguration("a production hub on a network interface requires
                                OS_HUB_ALLOWED_ORIGINS to name the origins its browsers are served from")
no OS_HUB_TRUSTED_FORWARDING -> UnsafeAuthConfiguration("a production hub on a network interface speaks
                                cleartext HTTP and requires OS_HUB_TRUSTED_FORWARDING=proxy, …")
no OS_HUB_CREDENTIAL_SIGN_IN -> UnsafeAuthConfiguration("production requires an identity authority: set
                                OS_HUB_CREDENTIAL_SIGN_IN=true …")
```

### Boot, and the posture it reports

```
[INFO] os-hub ready at http://192.168.178.70:7661
[INFO] bind scope network (192.168.178.70:7661), cross-origin policy allowlist, trusted forwarding proxy
{"level":"info","event":"server.readiness","outcome":"ok","detail":"addr=192.168.178.70:7661 scope=network"}
```

### The transport statement is enforced, not advisory

| request | code |
|---|---|
| `GET /healthz` with no `X-Forwarded-Proto` | **403** |
| `GET /healthz` with `X-Forwarded-Proto: http` | **403** |
| `GET /healthz` as the proxy (`X-Forwarded-Proto: https`) | **200** |
| `GET /readyz` as the proxy | **200**, `status: ready`, `mode: production`, `bindScope: network`, and every gate open — `directory`, `storage`, `artifactCasBarrier`, `artifactPublication`, `artifactAuthority`, `adminAssets` all `ready: true`, `publicSessionIssuance: true` |

`adminAssets.ready` matters on its own: it proves the release binary's compile-time admin path found
the SPA §1 built — the thing `OS_HUB_ADMIN_DIR` has to replace inside a container (§5).

### The cross-origin allowlist, against a real preflight

```
OPTIONS /auth/sessions  Origin: http://192.168.178.70:6081   -> 204, access-control-allow-origin: http://192.168.178.70:6081
OPTIONS /auth/sessions  Origin: https://evil.example.com     -> 0 allow-origin headers
```

### Credential sign-in over the network bind

```
POST /auth/sessions              -> 200   user_id 01a0c33b-26d6-7cf1-82ff-e53fee259ee0, token 108 chars
GET  /auth/sessions/me           -> 200   {"schema":"semio.directory.session-authority.v1", … "email":"ada@example.com","displayName":"Ada"}
POST /auth/sessions (wrong password) -> 400  {"event":"server.auth.session.mint","outcome":"refused","detail":"malformed-request"}
```

The first user was seeded by `os-hub credential set --email … --display-name Ada` with **no server
running**, which also emitted `[WARN] adopting an unstamped SQLite hub directory as format v1` —
correct behaviour for adopting a data root written before store-format stamping existed.

### HS1's 64 MiB pool-worker stack, in a release binary launched without cargo

The whole point of HS1's fix is that `.cargo/config.toml`'s `RUST_MIN_STACK = 67108864` floor exists
*only when cargo launches the process*. Here:

```
RUST_MIN_STACK in this environment: '(unset)'
stack overflow lines in the hub log so far: 0
SIGABRT/panic lines:                        0
stack overflow lines across both boots:     0
```

with eight concurrent authenticated `/directory/spaces` requests fanned out against it.
`WorkerPool::new`'s explicit `.stack_size(WORKER_STACK_BYTES)` (`⏳️async/🦀️.rs:1844,1893`) holds at
release profile in a standalone process. **Verified in the topology HS1's fix was written for**, for
the first time.

### SIGTERM: the drain, the held socket, the exit code

A TCP connection was opened and held across the signal:

```
sending SIGTERM to 89722 at 11:31:18
exit status after SIGTERM: 0
held socket: server closed the socket after 1.82s (recv returned 0 bytes)
the port after exit -> 000  (connection refused)
{"level":"info","event":"server.readiness","outcome":"cancelled","detail":"SIGTERM-received-draining-in-flight-work"}
```

Clean close, not a reset; exit **0**; the listener is gone afterwards.

### Restart reuses the sqlite root

```
POST /auth/sessions after restart -> 200
same user_id across restart: yes (01a0c33b-26d6-7cf1-82ff-e53fee259ee0)
```

### Store format stamps and the reused trusted catalog

```
authority   {"schema":"semio/hub/store-format/v1","store":"authority","version":1}
projections {"schema":"semio/hub/store-format/v1","store":"projections","version":1}
blobs       {"schema":"semio/hub/store-format/v1","store":"blobs","version":1}
sessions    {"schema":"semio/hub/store-format/v1","store":"sessions","version":1}

trusted-catalog/current.json  profileId=local-stdio-gis-open-v1  generationId=8086b61f33…  publicationRevision=1
{"level":"info","event":"server.catalog.publication","outcome":"ok","detail":"stage=CatalogResolved 9/9"}
```

A catalog published by a *different* binary was adopted by this one with no republish — 9/9 resolved,
the ~45-minute materialize skipped.

### Defect found by running it: the reused catalog root must match the binary's codegen policy

The first attempt used `.🧬semio/🌐hub/hs1-boot` as the seed, as my brief specified. The release
binary **refused to boot**:

```
Error: ArtifactAuthority(Catalog("trusted browser actor identity differs from its package or renderer"))
```

`hs1-boot`'s generation is stamped `jco-1.27.0`; a binary compiled after JC1's bump carries the
`jco-1.34` policy. This is the boundary G15 described from JC1's report — *"a hub binary rebuild that
changes the codegen/actor policy invalidates every previously published catalog outright"* — now
**observed against a release binary**, with the exact refusal text. Reseeded from
`.🧬semio/🌐hub/jc1-boot` (generation `8086b61f…`, `jco-1.34.0`) and everything above followed. The
script now takes the seed from `RB1_SEED` and documents the constraint at the assignment
(`📜️rb1-production-runtime.sh:24-29`).

### Two defects in my own probe, fixed

- A bare `wait` after fanning out eight background `curl`s waits for **every** background job — the
  hub included — so the run hung indefinitely at step 8. Now waits on the eight recorded pids.
- `tail -n "+$(wc -l < f)"` fails on macOS (`illegal offset -- +       9`): `wc -l` pads with spaces.

### What the hub is left in

Hub pid 89834 is still serving `192.168.178.70:7661` for §4's bundle probe; its pid file is
`🗑️generated/rb1-runtime/hub.pid`.

## 4. `build-s-react-release`, served and booted

### Run 1 — 11:09:46 → 13:39:01, 8955 s (2 h 29 min), **exit 130, no bundle**

101 min of nx run duration, **66 tasks succeeded, 33 `component-release` tasks failed**, and the
`build-s-react-release` task itself never ran. One root cause for all 33, and again peer churn, not
my slice:

```
error[E0277]: the trait bound `document::UiNodeRecord: Clone` is not satisfied
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:310:5
308 | #[derive(Clone, Debug, Default, PartialEq)]
309 | pub struct UiNodeTable {
310 |     entries: UiSnapshotNodes,      <- UiFixedList<UiNodeRecord, 128>
error: could not compile `semio-framework-ui-contract` (lib) due to 1 previous error
```

A peer was removing `Clone` from `UiNodeRecord` and my build read the tree between the two halves of
that edit. Both derives are consistent in the tree now (`UiNodeRecord` has no `Clone`, and
`UiNodeTable`'s derive no longer asks for it) and the file is committed clean.

**⚠️ The capture's last line says `RB1 S RELEASE DONE` and means nothing** — my script echoes it
unconditionally. The run's real verdict is the `exit=130` line above it and `files: 0`. Kept as
`🗑️generated/rb1-s-release-build-run1.txt`.

### Two defects found while preparing run 2 — both would have made the run worthless

**(a) `S_HUB_URL` is baked into the bundle but was not in the nx cache key.**
`🏗️builder/🌐️vite/🟦️.ts:237` defines `import.meta.env.VITE_S_HUB_URL` from `process.env.S_HUB_URL` at
Vite `define` time, so two bundles that differ only by which hub they sign in against are different
artifacts — but `build-<variant>-react-release`'s `inputs`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1059`) listed only file and runtime
inputs. A cached bundle would therefore be replayed for a different hub origin, silently.
**Fixed**: added `{ env: "S_HUB_URL" }, { env: "S_DATA_DIR" }` to that target's inputs.

**(b) Run 1 would have baked an empty hub URL.** My first script did not set `S_HUB_URL`, and an
empty define is not harmless: `readViteSEnv` returns `undefined` for `""`, and
`hubBootstrapOriginV1()` (`🏛️ShellHost/🟦️.tsx:1280-1294`) then falls back to **the page origin** —
i.e. the static server, not a hub — while every identity call site treats "no hub env" as "skip
identity entirely". The bundle would have booted and been incapable of signing in, with no error.
**Fixed** in `📜️rb1-s-release-build.sh`, which now exports
`S_HUB_URL=http://192.168.178.70:7661` with the reason at the assignment.

### Run 2 — queued

Requeued 13:51:23 with both fixes. Queue at 13:56: `tc3c` (holding), `pz1`, `s10`, **`rb1`**. Run 1's
66 successful tasks are in the nx cache, so run 2 should only rebuild the 33 that failed plus the
bundle step.

While requeueing I left an **orphaned mutex wrapper** of my own (pid 69364, `ppid=1`) holding a queue
slot ahead of the corrected run — killing the outer script had not killed the mutex child, and that
orphan would have taken the lock and run the *uncorrected* build, poisoning the nx cache for the
correct one. Killed by pid; the queue now holds exactly one `rb1` ticket. (Project memory
"Killing Wrapper Orphans Cargo", met from the other direction.)

### The coordinator's question: do the release components carry TC3b's `codec` export?

**Yes.** Checked against components run 1 did materialize at release profile (12:06–12:38), by their
canonical-ABI adapter names — `[async-lift]` is the export side, `[async-lower]` the import side:

```
semio_s_plugin_dag.wasm    codec async-lift: 17    codec async-lower: 0
exports seen: semio:framework/{checkpoint, codec, describe, jobs, reactor}
codec functions: apply-ops, genesis, pack-schema-hash, print-mirror
```

`semio:framework/codec@1.0.0` with all four functions of the WIT interface
(`🔌️plugin/🧬️schema/📜️.wit:1389`), on the export side only, in every release component checked
(`🎞️animate`, `🕸️dag`, `🏛️architect` — 29 codec strings, 144 `semio:framework/` strings each). So a
bundle from this tree can bind a post-TC3b hub such as TC3c's 7651, not only 7661's catalog.

### The release bundle cannot be checked by this fleet's own probes

Worth naming before anyone tries: `🐍️s2-*`, `🐍️b1a-*` and `🔬️catalog-smoke` all read
`window.__semioOsCatalogProbe`, which `🏛️ShellHost/🟦️.tsx:11714-11718` guards behind
`import.meta.env.DEV` and which therefore **does not exist in a release bundle** —
`🔬️catalog-smoke/🟦️.ts:263` says as much in its own failure text. So the coordinator's
`plugins.length` is not obtainable from a production bundle by any existing probe, and there is **no
production-safe catalog witness at all**: no automated gate can ever check what a shipped bundle
hosts. The readiness beacon (`data-semio-os-ready`, `:11683`) is *not* dev-gated and is the one
witness that survives, which is why `🐍️rb1-release-bundle-probe.mjs` (new) is built on it plus DOM
evidence a user could also see, rather than on the dev probe.

### Still open

The bundle was not produced, so it was not served on 6081 and not booted. **G15 #7 stays open.**
Everything around it is ready and waiting: `🐍️rb1-serve-release-bundle.ts` (static server —
`application/wasm`, SPA fallback that still 404s asset-looking paths, COOP/COEP for the workers'
`SharedArrayBuffer`), `🐍️rb1-release-bundle-probe.mjs` (the five rows above), and the production hub
itself on `192.168.178.70:7661` with `http://192.168.178.70:6081` already in its allowlist.

### Adjacent item closed instead: the distribution tarballs (G15 #2)

G15: *"no real tarball has ever been produced… P4 ran it end-to-end against a **stand-in** Mach-O
binary."* Both now exist, from the real release binaries, `🗑️generated/rb1-publish.txt`:

| target | tarball | size | sha256 |
|---|---|---|---|
| `bun ./📜️script.ts publish` in `🌎️hub/📦️packages/🦀️rust` | `dist/publish/os-hub-0.1.0-darwin-arm64.tar.gz` | 23 182 368 B | `01458f7c6a55598a7b4c118e6da1a5b944f5b72637f5778b6fd9975b4a91d1d5` |
| same in `…/🌉️mcp/📦️packages/🦀️rust` | `dist/publish/semio-os-mcp-0.1.0-darwin-arm64.tar.gz` | 11 682 122 B | `3e402b6c9e0c1e171a3ba678f3b3ef072fb2e5d8d6a043f548fe623a33b791be` |

Each with its sibling `.sha256`, each re-signed inside the payload before `tar`. Nothing is
uploaded, by design.

## 5. Dockerfile / compose

**Docker IS available on this host** — `docker info` answers, server 29.5.3, buildx 0.34.1 — so this
went further than my brief's fallback ("static review only") allowed for.

`🌎️hub/Dockerfile` and `🌎️hub/compose.yaml` were both written for the **development** topology
(`CMD ["bun","nx","run","os-hub:dev"]`, `OS_HUB_MODE=development`, the whole repository copied into
the runtime image) because they predate P4's production-mode fix. Rewritten for the production
topology:

| defect | file:line (old) | fix |
|---|---|---|
| runtime image carries the whole repo + `bun` + Nx, because "the binary cannot boot" | `Dockerfile` header + `COPY --from=builder /src /srv/semio-hub/repo` | binary-only runtime: `COPY` just `dist/build/os-hub` and the admin `📤️dist`; no `bun`, no repo |
| entrypoint is the dev launcher | `Dockerfile` `CMD ["bun","nx","run","os-hub:dev"]` | `ENTRYPOINT ["/usr/bin/tini","--","/usr/local/bin/os-hub"]` |
| `OS_HUB_BIND=127.0.0.1` **inside the container** | `Dockerfile` ENV, `compose.yaml` `OS_HUB_BIND: 127.0.0.1` | `0.0.0.0` — a loopback bind inside a container makes `-p`/`ports:` unreachable, so the published port answered nothing. The host-side publish stays `127.0.0.1:8787:8787`. |
| no `OS_HUB_ADMIN_DIR` | both | set to `/srv/semio-hub/admin`; the compile-time default points into the crate's source tree, which a binary-only image does not carry — the admin SPA would 404 |
| `HEALTHCHECK` speaks plain HTTP | `Dockerfile` HEALTHCHECK | sends `X-Forwarded-Proto: https`. With `OS_HUB_TRUSTED_FORWARDING=proxy`, `transport_security_middleware` (`🏗️bootstrap/🦀️.rs:2697`) is layered **outermost** on the whole router (`:9903`) — `/healthz` and `/readyz` included — and answers `403 x-semio-refusal: insecure-transport`. The old healthcheck would have marked every production container permanently unhealthy. |
| `os-hub:build-dev` built into the image | `Dockerfile` builder stage | dropped; the dev launcher is not part of a server image |
| builder stage goes through `nx run` | `Dockerfile` builder stage | calls each project's own `📜️script.ts` verb (measured 8 s vs 4 m 34 s of Nx graph, §1) |
| compose has no admin identity | `compose.yaml` | `OS_HUB_ADMIN_SUBJECTS: credential.password.v1:ada@example.com` — production refuses to boot without it, and the provider string is the one §3 proves works |

Validated statically (`🗑️generated/rb1-docker-check.txt`):

```
$ docker build --check -f 🌎️hub/Dockerfile .      →  Check complete, no warnings found.   (exit 0)
$ docker compose -f 🌎️hub/compose.yaml config     →  resolves, full service map printed    (exit 0)
```

Neither had ever been run before; both files carried "UNBUILT, UNRUN" headers. **The image itself is
still not built**: the builder stage compiles the hub's full release dependency graph, which §1
measures at 25–95 min on this machine outside a container, and a cold container would redo all of it
with no shared `build-dir`. That is an honest remaining gap, not a claim.

## 6. Command registration (script verb / nx target / launch row)

Audited the three commands this slice runs. Two were already fully registered; one was missing its
launch row.

| command | `📜️script.ts` verb | `📋️project.json` target | `.vscode/launch.json` row |
|---|---|---|---|
| `os-hub:build` | `BuildScript`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts:12088` | `build` (+ `publish`, `build-dev`) | `📦️build🌎️os-hub` — present (`:4229`) |
| `@semio-tech/framework-os-mcp-rs:build-release` | `BuildReleaseScript`, `…/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:700` | `build-release` (+ `publish`) | `📦️build-release🌉️os-mcp` (`:4259`), `🚚️publish🌉️os-mcp` (`:4270`) — present |
| `@semio-tech/framework-os-dev:build-s-react-release` | generated target → `bun ../../🚚️distribution/📜️script.ts build s react release` | generated by `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1056` | **MISSING — added** |

The `s` bundle — the product's flagship distribution artifact — was the only
`build-<variant>-react-release` target with no launcher row, while `cad`, `puzzle3d`, `puzzle5d` and
`shooting` all had one. Added `📦️build🖥️s` (`group 4_build`) to **both** `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc`, in the existing order/grouping, immediately before `📦️build📐️cad`.
Both files re-parse (391 and 341 configurations).

My own slice scripts (`📜️rb1-release-builds.sh`, `📜️rb1-hub-release-build.sh`,
`📜️rb1-s-release-build.sh`, `📜️rb1-production-runtime.sh`, `🐍️rb1-mcp-install-probe.ts`) are ticket
scratch under preamble rule 5 and are deliberately **not** registered as product commands.

## Defects fixed

Product/build defects, at the root, all found by *running* the product the way a user would:

| # | defect | file:line | how it was found |
|---|---|---|---|
| 1 | Three of the seven documented `--scopes` names (`artifact.open`, `artifact.create`, `inference.run`) are not scope names; `expand_scope` passes unknowns through literally and they grant nothing, silently | `🧰️framework/…/🌉️mcp/README.md` ×2 blocks, `🌎️hub/README.md` ×2, `README.md` ×2 — all corrected to `workspace.read,artifact.read,artifact.write,inference.execute,ui.observe,ui.control`; trap documented against `🛡️policy/🦀️.rs:26-66` | running the release binary from the documented config, negative control → `inference_run` `PERMISSION_DENIED` |
| 2 | `OS_HUB_ADMIN_SUBJECTS` documented with a provider string that does not exist (`semio.hub.credential`, and `semio.hub.credential/v1` in the systemd unit). The real one is `CREDENTIAL_IDENTITY_PROVIDER = "credential.password.v1"` (`🌎️hub/🔐️auth/🦀️.rs:49`); a wrong provider still boots and simply 401s every admin route | `🌎️hub/README.md:85,92,480` + a new table row explaining the format and the silent-401 failure mode | reading `authenticate_admin_principal` (`🏗️bootstrap/🦀️.rs:2700-2712`) while writing §3's boot env |
| 3 | The hub Dockerfile bound `OS_HUB_BIND=127.0.0.1` **inside** the container, so the published port answered nothing | `🌎️hub/Dockerfile` ENV, `🌎️hub/compose.yaml` — now `0.0.0.0`, host publish stays loopback | rewriting for the production topology |
| 4 | The Docker `HEALTHCHECK` spoke plain HTTP; under `OS_HUB_TRUSTED_FORWARDING=proxy` the transport layer is outermost over the whole router and 403s `/healthz`/`/readyz`, so every production container would be permanently unhealthy | `🌎️hub/Dockerfile` HEALTHCHECK — now sends `X-Forwarded-Proto: https` | §3 measured the 403 directly |
| 5 | A binary-only image would 404 the admin SPA: the compile-time `admin_dir` default points into the crate's source tree | `🌎️hub/Dockerfile`/`compose.yaml` — `OS_HUB_ADMIN_DIR=/srv/semio-hub/admin` | §3's `/readyz` `adminAssets.ready` showed what the source-tree default buys |
| 6 | Dockerfile/compose shipped the **development** topology (repo-in-image, `bun nx run os-hub:dev`) after production mode became reachable | `🌎️hub/Dockerfile`, `🌎️hub/compose.yaml` rewritten; `🌎️hub/README.md` § Container image rewritten | G15 #4; validated with `docker build --check` |
| 7 | `build-s-react-release` — the product's flagship bundle — had no `.vscode/launch.json` row while four lesser variants did | `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — added `📦️build🖥️s` | §6 registration audit |
| 7b | `S_HUB_URL`/`S_DATA_DIR` are baked into the `s` bundle at Vite define time but were not part of `build-<variant>-react-release`'s nx cache key, so one cache entry would be replayed for a different hub origin | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1059` — added `{ env: "S_HUB_URL" }, { env: "S_DATA_DIR" }` | preparing §4's run 2 |
| 8 | Root `README.md` had no getting-started at all; D2's report claimed the edit and G15 proved it was not in the tree | `README.md` § 🛍️ Products, +115 lines | G15's finding, re-verified |
| 9 | Four claims in `🌎️hub/README.md` were stale the moment §1/§3 ran ("Production mode has never been run", "Never proven at release profile", the container gap, the tarballs) | `🌎️hub/README.md` — each replaced with what was measured, and the catalog-policy constraint added as a real gap in place of the retired one | running the thing the docs described |

Not defects in the product, but worth recording:

- **The nx wrapper, not the work, is the cost.** `nx run os-hub-admin:build` sat 4 m 34 s at 0 % CPU
  in project-graph computation and never reached the command; the underlying verb took **8 s**.
  Preamble rule 16 confirmed; the Dockerfile's builder stage was changed to call the verbs directly
  for the same reason.
- **`buildCargoArtifacts` ignores `CARGO_TARGET_DIR`** (`🏗️native-build/🟦️.ts:99` overwrites it with
  a per-run `mkdtemp`), so preamble rule 25 is automatic on this path and `target-rb1` stays empty.
- Two defects in my own probe scripts (bare `wait`, `wc -l` padding) — §3.

## Honest gaps

1. **§4 is not done.** Run 1 spent 2 h 29 min and died on a peer's half-landed `Clone` removal with
   33 of 99 tasks failed and no bundle; run 2 is queued 4th on the fleet wasm mutex. The bundle was
   never produced, never served on 6081 and never booted headless, so **no claim is made about
   whether the production bundle boots or hosts its plugins**. G15 #7 stays open. Run 2 is detached
   and writes `🗑️generated/rb1-s-release-build.txt`; the serve and probe scripts are written and
   unrun. Anyone finishing this: read §4's "two defects found while preparing run 2" first — an
   uncorrected rerun produces a bundle that cannot sign in.
1b. **`plugins.length` is not obtainable from a release bundle**, so even a successful run 2 cannot
   answer the coordinator's plugin-count question with the fleet's existing probes (§4).
2. **No real reverse proxy.** §3 simulates the TLS terminator by sending the `X-Forwarded-Proto` /
   `X-Forwarded-Host` headers Caddy or nginx would send. The enforcement is proven; the proxy configs
   in `🌎️hub/README.md` are still correct-by-construction, not loaded by any proxy.
3. **No browser touched the production hub.** §3's CORS evidence is a real preflight over a real
   network socket, but from `curl`, not from a browser enforcing the response.
4. **The container image is still unbuilt.** `docker build --check` and `docker compose config` both
   pass and Docker is available here, but a real `docker build` recompiles the hub's whole release
   graph in a cold container with no shared `build-dir`; not attempted.
5. **The `--hub`/`--credential-file` MCP loop is documented, not driven.** §2 proves the release
   binary end to end over `--folder`. The hub-space variant is written into both READMEs from the
   existing implementation and the AgentDelegations panel; nobody ran it (G15 #9).
6. **The MCP registry skips two plugins**, seen in every §2 run's stderr and not mine to fix:
   `puzzle` — `🔣️.json did not decode as a PackageDescriptor: missing field artifactSchema`; `stdio`
   — `no committed descriptor at ✏️s/🔌️plugins/🗄️stdio/🔣️.json`. PZ1/DS1 territory.
7. **§1's timing is not a clean benchmark.** Run 1 (95 min, failed) ran at load average 165; run 2
   (39 min, succeeded) at ~50–90, with the shared `build-dir` already warm from run 1. A cold
   release build on a quiet machine was never measured.
8. **`os-hub credential set` warned** `adopting an unstamped SQLite hub directory as format v1` on
   the reused root. Correct behaviour for a pre-stamping root, but it means §3's store-format
   evidence is an *adoption*, not a fresh creation.

## Files changed

Product / repository:

| file | change |
|---|---|
| `README.md` | +115 — status note and Getting started under sketchpad / studio / cloud / assistant, including both worked MCP client configs (D2's missing edit, G15 #5) |
| `🌎️hub/README.md` | new § *For the people who will use this hub* (79 lines); § *Container image* rewritten for the production topology; `OS_HUB_ADMIN_SUBJECTS` provider fixed ×3 + documented; four stale claims replaced with measured facts; tarball sizes/checksums recorded; the catalog-policy gap added |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` | both worked configs' `--scopes` corrected; new paragraph naming every legal scope and the silent-passthrough trap with its measured `PERMISSION_DENIED` |
| `🌎️hub/Dockerfile` | rewritten: binary-only production runtime image, `os-hub` under `tini` as entrypoint, `0.0.0.0` bind, `OS_HUB_ADMIN_DIR`, proxy-aware healthcheck, builder stage calls the project verbs directly |
| `🌎️hub/compose.yaml` | rewritten to match: production mode, the two network statements, an admin subject, `OS_HUB_ADMIN_DIR` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `📦️build🖥️s` row added in the existing `4_build` order/grouping |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` | `S_HUB_URL`/`S_DATA_DIR` added to `build-<variant>-react-release`'s nx cache key |

Ticket folder (new):

| file | what |
|---|---|
| `📓️rb1-release-builds-and-production-posture.md` | this report |
| `📜️rb1-release-builds.sh` | admin SPA + `os-hub` release + `os-mcp` release, sequential |
| `📜️rb1-hub-release-build.sh` | the `os-hub` release build alone (rerun after the peer-churn failure) |
| `📜️rb1-s-release-build.sh` | `build-s-react-release` through the wasm mutex, one hold |
| `📜️rb1-production-runtime.sh` | §3's network-bind production observation |
| `🐍️rb1-mcp-install-probe.ts` | drives a `.mcp.json`-shaped config through `initialize`/`tools/list`/gated `tools/call` |
| `🐍️rb1-serve-release-bundle.ts` | static server for the production bundle (`application/wasm`, SPA fallback, COOP/COEP) — **written, unrun** |
| `🐍️rb1-release-bundle-probe.mjs` | release-bundle boot + hub sign-in probe built on the non-dev-gated beacon — **written, unrun** |

Build outputs (generated, not tracked): `🌎️hub/📦️packages/🦀️rust/dist/{build,publish}`,
`…/🌉️mcp/📦️packages/🦀️rust/dist/{build-release,publish}`,
`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist`, data root `.🧬semio/🌐hub/rb1-prod`.

## Appendix — captured transcripts

Kept here verbatim because `🗑️generated/` is swept between sessions and P4's equivalent evidence was
lost exactly that way.

### A. `📜️rb1-production-runtime.sh` — the network-bind production observation (2026-09-21 11:29–11:32)

```
=== 0. the release binary — rm + cp + codesign, never an in-place overwrite ===
codesign: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-runtime/os-hub: replacing existing signature
codesign: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-runtime/os-hub: valid on disk
codesign: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-runtime/os-hub: satisfies its Designated Requirement
size: 145256624 bytes
file: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-runtime/os-hub: Mach-O 64-bit executable arm64
bind address: 192.168.178.70  port: 7661  origin: http://192.168.178.70:6081

=== 1. data root — a reused published catalog root, operator-private ===
data: total 1568
data: drwx------@  9 ueli  staff     288 Sep 21 11:10 .
data: drwxr-xr-x@ 13 ueli  staff     416 Sep 21 11:16 ..
data: drwxr-xr-x@  3 ueli  staff      96 Sep 21 11:10 artifact-cas
data: drwxr-xr-x@  5 ueli  staff     160 Sep 21 11:10 db
data: -rw-r--r--@  1 ueli  staff  741376 Sep 21 11:10 directory.db
data: drwxr-xr-x@  2 ueli  staff      64 Sep 21 11:10 extension-modules
data: drwxr-xr-x@  3 ueli  staff      96 Sep 21 11:10 inference
data: drwxr-xr-x@  6 ueli  staff     192 Sep 21 11:10 instance
data: drwx------@  8 ueli  staff     256 Sep 21 11:10 trusted-catalog
mode: drwx------ /Users/ueli/Documents/semio/.🧬semio/🌐hub/rb1-prod

=== 2. the three refusals a network bind must produce before it is allowed to boot ===
no OS_HUB_ALLOWED_ORIGINS    -> Error: UnsafeAuthConfiguration("a production hub on a network interface requires OS_HUB_ALLOWED_ORIGINS to name the origins its browsers are served from")
no OS_HUB_TRUSTED_FORWARDING -> Error: UnsafeAuthConfiguration("a production hub on a network interface speaks cleartext HTTP and requires OS_HUB_TRUSTED_FORWARDING=proxy, which states that a TLS-terminating reverse proxy is the only thing that can reach this socket")
no OS_HUB_CREDENTIAL_SIGN_IN -> Error: UnsafeAuthConfiguration("production requires an identity authority: set OS_HUB_CREDENTIAL_SIGN_IN=true to use the hub's own credentials, or configure an IdentityAssertionVerifier adapter")

=== 3. first user through the operator verb — no server running, no fd 3 ===
credential-set: 01a0c33b-26d6-7cf1-82ff-e53fee259ee0

=== 4. boot — production, NETWORK bind, plain process ===
booted pid 89722 on 192.168.178.70:7661
boot: [INFO] os-hub ready at http://192.168.178.70:7661
boot: [INFO] bind scope network (192.168.178.70:7661), cross-origin policy allowlist, trusted forwarding proxy

=== 5. the transport statement is enforced, not advisory ===
GET /healthz WITHOUT X-Forwarded-Proto  -> 403
GET /healthz with X-Forwarded-Proto:http -> 403
GET /healthz as the proxy                -> 200
GET /readyz  as the proxy                -> {"schema":"semio.hub.readiness/v1","status":"ready","runId":"production","mode":"production","bindScope":"network","authentication":{"kind":"identity-assertion-verifier","bootstrapReady":true,"publicSessionIssuance":true},"directory":{"ready":true},"storage":{"ready":true},"artifactCasBarrier":{"ready":true},"artifactPublication":{"ready":true},"artifactCasSweeper":{"ready":true,"execute":false,"defaultMode":"dry-run"},"artifactAuthority":{"ready":true},"adminAssets":{"ready":true},"features":{"openPlan":true,"openPlanExchange":true,"rebootstrap":true,"mcpWorkspace":true,"inference":true}} [200]


=== 6. the cross-origin allowlist ===
preflight from the allowed origin http://192.168.178.70:6081 -> 204
  allow-origin header: access-control-allow-origin: http://192.168.178.70:6081
  a foreign origin:    0 allow-origin header(s)

=== 7. credential sign-in over the network bind ===
POST /auth/sessions -> 200
user_id: 01a0c33b-26d6-7cf1-82ff-e53fee259ee0  token: session.v1.1740c… (108 chars)
GET /auth/sessions/me -> {"schema":"semio.directory.session-authority.v1","sessionBindingSha256":"ed1b53d5501a477a2c5ff9539fa81fcf23784cda32e25e03706a79f18bc3b653","authorizationGeneration":1,"userId":"01a0c33b-26d6-7cf1-82ff-e53fee259ee0","email":"ada@example.com","displayName":"Ada","expiresAt":1790026276519,"sessionKind"
a wrong password      -> 400

=== 8. pool-worker stack — HS1's 64 MiB fix in a binary launched WITHOUT cargo ===
RUST_MIN_STACK in this environment: '(unset)' (cargo's .cargo/config.toml floor is NOT in play here)
stack overflow lines in the hub log so far: 0
SIGABRT/panic lines:                        0

=== 9. SIGTERM with a socket held open — the drain, the socket, the exit code ===
sending SIGTERM to 89722 at 11:31:18
exit status after SIGTERM: 0
held socket: socket opened
held socket: server closed the socket after 1.82s (recv returned 0 bytes)
tail: illegal offset -- +       9: Invalid argument
the port after exit -> 000

=== 10. restart — the same sqlite root, the same user ===
POST /auth/sessions after restart -> 200
same user_id across restart: yes (01a0c33b-26d6-7cf1-82ff-e53fee259ee0)
stack overflow lines across both boots: 0

=== 11. the durable store format stamps, and the reused trusted catalog ===
authority: {"schema":"semio/hub/store-format/v1","store":"authority","version":1}
projections: {"schema":"semio/hub/store-format/v1","store":"projections","version":1}
blobs: {"schema":"semio/hub/store-format/v1","store":"blobs","version":1}
sessions: {"schema":"semio/hub/store-format/v1","store":"sessions","version":1}
trusted-catalog pointer: {"profileId":"local-stdio-gis-open-v1","generationId":"8086b61f336e08b5c483ca96525e8e77d6322becbe1569aa6cfedbf3b595ae6b","bundleSha256":"9061e8ea07e8165916329f51fe1eccfa6e6fcd921e929a1ab71ab336443368be","publicationRevision":"1"}
catalog: {"level":"info","event":"server.catalog.publication","outcome":"ok","detail":"stage=CatalogResolved 9/9"}
catalog: [INFO] bind scope network (192.168.178.70:7661), cross-origin policy allowlist, trusted forwarding proxy

=== hub 89834 left running on 192.168.178.70:7661 for the s-bundle probe; stop it with: kill -TERM $(cat /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-runtime/hub.pid) ===
```

### B. `🐍️rb1-mcp-install-probe.ts` — the release MCP binary from a `.mcp.json`-shaped config

```
=== scopes: documented ===
config   /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-config-documented.json :: mcpServers.semio
command  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp
args     ["stdio","--folder","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-folder","--scopes","workspace.read,artifact.open,artifact.create,artifact.write,inference.run,ui.observe,ui.control"]
PASS  initialize answers  protocolVersion=2025-11-25 serverInfo={"name":"semio-os-mcp","version":"0.1.0"}
PASS  declares a tools capability  ["prompts","resources","tools"]
PASS  tools/list answers  27 tools
      tools: action_cancel, action_invoke, action_prepare, artifact_create, artifact_export, artifact_open, artifact_snapshot, artifact_validate, capabilities_describe, capabilities_search, context_resolve, history_redo, history_undo, inference_approve, inference_cancel, inference_events, inference_get, inference_list, inference_run, inference_submit, job_cancel, job_get, transaction_begin, transaction_commit, transaction_rollback, ui_focus, ui_reveal
PASS  capabilities_describe (workspace.read)  isError {"content":[{"text":"no such capability: ","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"NOT_FOUND","details":null,"message":"no such capability: ","retryable":false}}
PASS  inference_list is not PERMISSION_DENIED  reached the tool — gate: artifacts.read (workspace.read grants it) :: {"content":[{"text":"6 declared inference(s) in this workspace","type":"text"}],"isError":false,"resultType":"complete","structuredContent":{"declared":[{"algor
PASS  artifact_create is not PERMISSION_DENIED  reached the tool — gate: artifacts.write + jobs.spawn (artifact.write / inference.execute grant them) :: {"content":[{"text":"artifactId is required","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INPUT_INVALID","details":null,"
PASS  inference_run is not PERMISSION_DENIED  reached the tool — gate: jobs.spawn (inference.execute grants it) :: {"content":[{"text":"artifactKind is required","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INPUT_INVALID","details":null
=== scopes: documented: all rows green ===

=== scopes: fake-only ===
config   /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-config-fake-only.json :: mcpServers.semio
command  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp
args     ["stdio","--folder","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-folder","--scopes","workspace.read,artifact.open,artifact.create,inference.run"]
PASS  initialize answers  protocolVersion=2025-11-25 serverInfo={"name":"semio-os-mcp","version":"0.1.0"}
PASS  declares a tools capability  ["prompts","resources","tools"]
PASS  tools/list answers  27 tools
      tools: action_cancel, action_invoke, action_prepare, artifact_create, artifact_export, artifact_open, artifact_snapshot, artifact_validate, capabilities_describe, capabilities_search, context_resolve, history_redo, history_undo, inference_approve, inference_cancel, inference_events, inference_get, inference_list, inference_run, inference_submit, job_cancel, job_get, transaction_begin, transaction_commit, transaction_rollback, ui_focus, ui_reveal
PASS  capabilities_describe (workspace.read)  isError {"content":[{"text":"no such capability: ","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"NOT_FOUND","details":null,"message":"no such capability: ","retryable":false}}
PASS  inference_list is not PERMISSION_DENIED  reached the tool — gate: artifacts.read (workspace.read grants it) :: {"content":[{"text":"6 declared inference(s) in this workspace","type":"text"}],"isError":false,"resultType":"complete","structuredContent":{"declared":[{"algor
PASS  artifact_create is not PERMISSION_DENIED  reached the tool — gate: artifacts.write + jobs.spawn (artifact.write / inference.execute grant them) :: {"content":[{"text":"artifactId is required","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INPUT_INVALID","details":null,"
FAIL  inference_run is not PERMISSION_DENIED  PERMISSION_DENIED — gate: jobs.spawn (inference.execute grants it)
=== scopes: fake-only: 1 RED row(s) ===

=== scopes: corrected ===
config   /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-config-corrected.json :: mcpServers.semio
command  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release/semio-os-mcp
args     ["stdio","--folder","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/rb1-mcp-folder","--scopes","workspace.read,artifact.read,artifact.write,inference.execute,ui.observe,ui.control"]
PASS  initialize answers  protocolVersion=2025-11-25 serverInfo={"name":"semio-os-mcp","version":"0.1.0"}
PASS  declares a tools capability  ["prompts","resources","tools"]
PASS  tools/list answers  27 tools
      tools: action_cancel, action_invoke, action_prepare, artifact_create, artifact_export, artifact_open, artifact_snapshot, artifact_validate, capabilities_describe, capabilities_search, context_resolve, history_redo, history_undo, inference_approve, inference_cancel, inference_events, inference_get, inference_list, inference_run, inference_submit, job_cancel, job_get, transaction_begin, transaction_commit, transaction_rollback, ui_focus, ui_reveal
PASS  capabilities_describe (workspace.read)  isError {"content":[{"text":"no such capability: ","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"NOT_FOUND","details":null,"message":"no such capability: ","retryable":false}}
PASS  inference_list is not PERMISSION_DENIED  reached the tool — gate: artifacts.read (workspace.read grants it) :: {"content":[{"text":"6 declared inference(s) in this workspace","type":"text"}],"isError":false,"resultType":"complete","structuredContent":{"declared":[{"algor
PASS  artifact_create is not PERMISSION_DENIED  reached the tool — gate: artifacts.write + jobs.spawn (artifact.write / inference.execute grant them) :: {"content":[{"text":"artifactId is required","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INPUT_INVALID","details":null,"
PASS  inference_run is not PERMISSION_DENIED  reached the tool — gate: jobs.spawn (inference.execute grants it) :: {"content":[{"text":"artifactKind is required","type":"text"}],"isError":true,"resultType":"complete","structuredContent":{"code":"INPUT_INVALID","details":null
=== scopes: corrected: all rows green ===
```

### C. Build lines

```
os-hub-admin build      exit=0  seconds=8
os-hub release run 1    exit=1  seconds=5694   (peer churn: LocalizedLabel / texts_mut)
os-mcp build-release    exit=0  seconds=362    staged .../dist/build-release/semio-os-mcp
os-hub release run 2    exit=0  seconds=2360   Finished `release` profile [optimized] target(s) in 39m 13s
                                               -rwxr-xr-x  145256624  dist/build/os-hub

[publish] .../dist/publish/os-hub-0.1.0-darwin-arm64.tar.gz
[publish] sha256 01458f7c6a55598a7b4c118e6da1a5b944f5b72637f5778b6fd9975b4a91d1d5
[publish] .../dist/publish/semio-os-mcp-0.1.0-darwin-arm64.tar.gz
[publish] sha256 3e402b6c9e0c1e171a3ba678f3b3ef072fb2e5d8d6a043f548fe623a33b791be

$ docker build --check -f 🌎️hub/Dockerfile .    ->  Check complete, no warnings found.  (exit 0)
$ docker compose -f 🌎️hub/compose.yaml config   ->  resolves                            (exit 0)
```
