# 📓️ WR3 — headless command routes, headless view state, and the export surface

Slice: outcome 4's permanent gate `client-e2e`, from WR2's **13/16** toward 16/16, by the three named
root causes WR2 left (§7.1, §7.2, §10 gap 2). Continues WR1 (runtime port) and WR2 (command/response
wire); the live-shell twin is LB1 §10–§11.

**Status: landed, session 6 (2026-09-20 16:35).** Four of the five pieces are measured at runtime:
the seven new paged routes (§2, native laws), the export surface (§4.1–§4.2), the port-honouring
`produce_media` (§4.4, real SVG bytes out of a real guest) and the shell-side media OUT port (§4.5,
`agent-bridge-check` 55/55). `action_prepare` is NOT fixed and is root-caused one layer below WR2
(§3). The permanent gate reads **12/16** and, as §5.0 measures, is currently being served by a
peer's live shell rather than by the headless workspace it names.

## 0. Inherited state

`git status --short` on this slice's paths carried **no prior WR3 edit** and `🗑️generated/` no
`wr3-*` capture; what is inherited is WR1's runtime port, WR2's command/response wire (13/16) and
LB1 §10–§11's live-shell twin. Read as spec: `📓️wr2-headless-command-response-wire.md` whole,
`📓️wr1-wasmtime-runtime-port.md` §5–§7, `📓️lb1-live-bridge-action-routing.md` §10–§11.

**A measurement about the gate's own arithmetic, before anything else.** `runOsMcpClientJourney`
(`🌉️mcp/🟦️.ts:~470`) **returns early** the moment `action_prepare` is red
(`if (prepared.isError === true) return steps;`). The 16 steps WR2 reports are therefore *16 steps
that ran*, not a fixed denominator: the journey behind that early return declares a further ~12
(`resources/subscribe`, `action_invoke`, the updated-notification, `artifact_snapshot`, the live
head, `history_undo`/`redo`, `transaction_begin`/`rollback`, `inference_list`/`run`/`job_get`/
`job_cancel`). So "16/16" is not reachable as a literal: greening `action_prepare` *raises* the
denominator. The honest target is **zero red rows**, and the count printed beside it will be ~28.

## 1. The three reds as inherited — and what each needs

| red row | WR2's reading | what this slice owes it |
| --- | --- | --- |
| `os: capability catalog health` | 58 diagnostics = 29 plugins × 2; 14 stale generated descriptors | **not mine** — A3's `describe` regeneration |
| `os: action_prepare` | `typed command frames require an owner-qualified manifest command key` — `setSnapshot` is a window-kind action and the headless workspace has no window instance / view state | a real headless window instance + `baseDispatchViewState` twin |
| `os: artifact_export` | `draw` declares no media output port (`declaredExportFormats: []`) **and** `MediaOut`'s tag is refused by the paged `AppCommand` decoder | the media route + the export surface declaration + bytes back to the MCP caller |

## 2. The structural gap — CLOSED for transaction and media

`📡️spr/🧵️channel/🦀️.rs`. The decoder and the encoder in that one file are twins: `encode_app_command`
already wrote every tag (0–36), and `PagedAppCommandDecodeCursor::step`'s `Header` arm admitted only
`0–4, 6–9, 13, 15, 16, 27, 29–36`, refusing the rest with
`plugin.command-route-state-machine-required`. **The wire was never missing; only the guest's
admission was.** Every route added here decodes the bytes the encoder was already writing — no wire
change, no version bump, no fixture rewrite.

| # | what | where |
| --- | --- | --- |
| 1 | `route_field_plan(tag) -> Option<(leading, has_ops, trailing)>` — the ONE table stating each route's field counts, read by the new decoder and pinned by the encoder's own write order | `🧵️channel/🦀️.rs` |
| 2 | `PagedRouteFieldsDecode` — the retained decoder all seven new routes share: **one bounded field read per step**, an exact `try_reserve_exact` for `prepared_ops`, and a `close_step` that releases one accumulated field per grant | same |
| 3 | `TRANSACTION_PREPARED_OPS_MAXIMUM = 1024` — declared on BOTH sides (`encode_app_command` now refuses to write more; the decoder refuses to admit more), so a guest never reserves an unbounded op roster out of a host-supplied count. The same fixed shape `DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS` gives an archive | same |
| 4 | `DecodedAppCommandOwner::close_step` — bounded retirement for the seven new commands: `prepared_ops` drains **one op per step** ahead of the flat stages, then `payload`/`origin`/`txn_id`/`mutation_id`/`label` (media: `data`/`descriptor`/`port`), each with its exact restoration target when a field is larger than the step's grant | same |
| 5 | the trailing-bytes rejection path (`plugin.command-decode-trailing`) grew real arms for the seven routes instead of `unreachable!` | same |

Routes now admitted: **10 `MediaIn`, 11 `MediaOut`, 12 `MediaFingerprint`, 17 `TransactionPrepare`,
18 `TransactionCommit`, 19 `TransactionRollback`, 20 `TransactionUndo`, 21 `TransactionRedo`.**

**Presence is deliberately NOT added**, and that is a correction to WR2 §10 gap 2. Tag 28 is not
missing a route — it *has* one: `PresenceCommandCursor`, a reserved ingress with one exact page per
peer (`PagedCommand::try_from_presence_pages`), and `PluginCommandIngress::step`
(`🔌️plugin/🦀️.rs:36909`) refuses kind 28 on the generic cursor **by design**, naming it
(`"Presence commands require the reserved presence ingress authority"`). Adding a generic route for
it would have been a second, weaker admission for a command that already has a stricter one.

### 2.1 Measured — `🗑️generated/wr3-paged-route-laws.txt`

Every round-trip law in `🧵️channel/🧪️tests/🔬️unit/🦀️.rs` before this slice drove
`decode_app_command` — the **flat, host-side** decoder, which never refused these tags. That is why
a gap this wide was invisible to a green test file. The new laws drive
`PagedAppCommandDecodeCursor`, the decoder **compiled into every guest**:

```
test os_spr::channel::tests::paged_ingress_admits_every_media_route ... ok
test os_spr::channel::tests::paged_ingress_admits_every_transaction_route ... ok
test os_spr::channel::tests::paged_ingress_still_refuses_an_undeclared_route_by_name ... ok
test os_spr::channel::tests::a_cancelled_media_route_releases_one_field_at_a_time ... ok
test result: ok. 12 passed; 0 failed
```

`cargo test -p semio-framework-os-kernel --lib` (rule 25 private target dir `target-wr3`), 2 m 31 s
cold. The refusal law pins that "admitted" did not become "everything is admitted": an undeclared
tag is still `plugin.command-route-state-machine-required`.

## 3. `action_prepare` — the view state is real, and it is NOT the blocker

This slice was briefed to mint a headless window instance and view state so that window-kind actions
resolve as they do in the shell. That view state is buildable and the exact recipe is below — but
**routing `action_prepare` through the window lane would be wrong**, and the measurement that says
so is in this repository, not in an opinion.

### 3.1 The recipe, for the record

Everything a real `ActionAddress` needs is already committed data, and nothing has to be invented:

| field | source, measured |
| --- | --- |
| `plugin_id` / `app_id` / `window_kind_id` / `action_id` | `CapabilitySource::Action{..}` — the catalog ALREADY retains the first window kind that declares each verb (`🗂️catalog/🦀️.rs` `app_action_verbs`, whose own doc says the retained kind is "the `window_kind_id` a dispatch has to address") |
| `mode_id` | `AppDefinition.default_mode_id` (`draw`: `edit`) |
| `window_instance_id` | the base instance id, which **is** the window kind id — `sessionWindowInstances` (`🛠️ShellHelpers/🟦️.tsx:5437`): `app.windowKinds.map(kind => ({ id: kind.id, … }))` |
| `ViewModel.window_instances` | one `ViewWindowInstance{id: kind.id, window_kind_id: kind.id}` per declared window kind |

The live proof that this shape works headlessly already exists and is green:
`✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs:236` drives a REAL gis component with
exactly this `ViewModel` (`gis2d-main` as kind, instance and `window_id`) over
`AppCommand::Command`, and the guest's `admit_addressed_action_view` accepts it.

### 3.2 Why the gateway must not use it for `prepare`

`ActionAdapter` (`🔀️dispatch/🦀️.rs`) is a genuine two-phase protocol:

- `prepare` (`:723`) = `ReadHistory` (baseline) + **`PureCommand`**, and keeps `ops` on the handle.
  It must NOT change the document — its whole contract is a preview plus a revision baseline.
- `invoke` (`:877`) = `ReadHistory` (must still equal the baseline) + `TransactionPrepare{ops}` +
  `TransactionCommit`, minting the undo token.

`AppCommand::Command` is the shell's dispatch lane and it **applies**: the guest arm
(`🔌️plugin/🦀️.rs:37748`) runs `handle_action_invocation` → `dispatch_typed_command_inner`, sets
`mutated = true` and answers `AppFrame::Invocation` with a `history_patch`. And
`RoutingArtifactChannel` caches **one channel, one guest instance, per plugin id**
(`🏠️workspace/🦀️.rs:1886`), so prepare and invoke land on the same live guest. Routing `prepare`
through the window lane would therefore apply the mutation at prepare time and again at commit:
`action_invoke` would still look green, and `history_undo` (which the journey asserts returns the
head to the baseline) would fail with half the edit still applied. That is a worse gate than a red
`action_prepare`, because it is a green one that lies.

### 3.3 The actual root cause, one layer below WR2's

WR2 read the refusal as "the headless gateway has no window". It is narrower than that. The
`PureCommand` lane — the *pure* lane, the only one whose contract matches `prepare` — reaches
`ArtifactApp::dispatch_command_frame`, whose single implementation in the repo (the shared
`VcsArtifactApp<A>` wrapper, `🔌️plugin/🦀️.rs:26765`) is a **three-line unconditional refusal**:

```rust
async fn dispatch_command_frame(&mut self, _command_bytes: &[u8], _meta: &ActionMeta) -> Result<InvocationResult, Fault> {
    Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-exact-key"), "typed command frames require an owner-qualified manifest command key before deserialization"))
}
```

Two things are missing there, and only the second is a window:

1. **The payload is not owner-qualified.** The gateway sends `{"capabilityId": …, "input": …}` as
   opaque bytes. The guest's sibling lane decodes a `ManifestActionInvocation` and resolves
   `A::command_from_action(action_id, args)` under an admitted tool proof (`:26753`). The pure lane
   is handed no key at all, so it cannot select a command-specific pre-serde envelope — which is
   precisely what its refusal says.
2. **The reactor gives it no view state.** The `PureCommand` arm builds
   `ActionMeta { …, view_state: None }` (`🔌️plugin/🦀️.rs:38151`) and the wire `AppCommand::PureCommand`
   has no `view_state` field to carry one. So even a fixed key would reduce a window-kind verb
   against no window.

The fix is therefore a **three-part, schema-first change in the guest SDK**, not a host-side view
state: carry the same `ManifestActionInvocation` + `ViewModel` pair the Command lane already uses on
`PureCommand` (one shape, both lanes), and give `dispatch_command_frame` a real body that resolves
the command and runs the app's reducer **without** `dispatch_emit` — returning the `Emit` wire the
reactor already reads back through `take_last_emit_wire`. That is an owned change to
`🔌️plugin/🦀️.rs`'s shared `VcsArtifactApp` dispatch pipeline plus a wire field, and it is
**deliberately not taken here**: it is a second slice-sized piece of work in a file three other
agents are editing this session, and half-landing it (the wire field without the pure reducer) would
leave the same red row with a longer message. Named, measured, and handed on — §7 gap 1.

## 4. `artifact_export` — three gaps, all three named, two closed in source

WR2 §7.2 named two. There were three, and the first one is the interesting one.

### 4.1 `AppDefinition.media_outputs` is empty for every plugin in the repo

`export_ports_for` → `installed_artifact_kinds` (`🏠️workspace/🦀️.rs:2405`) read OUT ports from
`app.media_outputs` **only**. Measured over every committed descriptor: that array is `[]` for all
35 plugins, including the ones that demonstrably export. An app's real OUT ports live in
`AppIo.ports` with `direction: out` — `writer: text:out`, `lowpoly: mesh:out`, `cad: brep:out`,
`draw: vector:out`. So the gateway was reading a field nothing fills, and
`declaredExportFormats: []` was a true statement about the wrong field.

**Fixed**: new `app_media_out_ports(app)` (`🏠️workspace/🦀️.rs`) unions `media_outputs` with the
`direction == Out` half of `io.ports`, then appends the implicit **`artifact:out`** — which is a
real declared surface, not an invention: `AppIo::all_ports` gives it to every app and
`ArtifactEditor::export_media`'s provided body answers it with the document's own pack
(`🔌️plugin/🦀️.rs:32103`). App-specific ports come first, so an export with no requested format
takes the app's own exporter and falls back to the pack, never the other way round.

### 4.2 `draw` declared a DEFAULT `AppIo` while exporting for real

`draw` has had a real exporter and a real port all along:
`drawing_vector_out_port()` (`vector:out`, 2D vector) and `DrawingPlayApp::export_media` answering
it through `drawing_document_to_svg` — with a green law,
`drawing_io_declares_vector_out_and_export_media_covers_both_ports`. But `create_drawing_app()`
**never called `.io(drawing_io())`**, the one line `writer_io()`/`lowpoly_io()`'s apps do call. So
the manifest carried `AppIo::default()` and the committed descriptor showed
`"io": {"artifactSchema": "", "ports": [], "exportFormats": []}` for an app whose own trait method
returns the opposite.

**Fixed**: `.io(drawing_io())` on the editor builder
(`✏️s/🔌️plugins/🖍️draw/…/✏️editor/🦀️.rs`, before `.build_definition()`), then the plugin's own
producer verb (`bun ./📜️script.ts describe` in `🖍️draw/📦️packages/🦀️rust`) re-emits
`🛂️.descriptor.semio` + `🔣️.json` with their own two-pass hashes — never a hand-edited JSON, which
WR2 §8 correctly refuses.

### 4.3 `MediaOut` could not cross the headless ingress

Closed by §2: tag 11 is now an admitted route with a native law. Without §2 this slice's other two
export fixes would have changed the refusal text and nothing else.

### 4.4 `produce_media` answered the document pack for EVERY port

With §4.1–§4.3 landed, `artifact_export` returned bytes for the first time — and the bytes were
wrong. Measured on a real compiled `draw` component (`🗑️generated/wr3-export-probe.txt`, run 1):

```
create kind=s.draw.drawing isError=false … pluginId=draw sizeBytes=615
export isError=false port=vector:out base64Bytes=824 availablePorts=["vector:out","artifact:out"]
export bytes=617 svgMarker=no head="\xff\x02\x89SEM…\x17\x00\x00\x00drawing.drawing.pack v1\x89SPK…"
```

617 bytes of the drawing's own **document pack**, delivered under the name of the SVG port, with no
`<svg` anywhere in it. `PluginApp::produce_media`'s provided body (`🔌️plugin/🦀️.rs:12719`) —
the route `plugin_produce_media` → `AppCommand::MediaOut` reaches — **ignores `port` entirely**:

```rust
async fn produce_media(&mut self, port: &str) -> Result<MediaArtifact, MediaArtifactError> {
    let files = self.document_pack().await…;
    Ok(MediaArtifact { descriptor: …Document { schema: self.artifact_schema()… }, data: store::encode_document_pack_bytes(&files.pack, &files.spr).await })
}
```

`VcsArtifactApp` overrides `export_media` (`🔌️plugin/🦀️.rs:30757`) to reach the app's real
`A::export_media_with_request_context`, but nothing connected the two: the ABI-level port producer
never asked the app what the port produces. Every app with a real exporter — `draw: vector:out`,
`writer: text:out`, `lowpoly: mesh:out`, `cad: brep:out` — shipped its pack instead.

**Fixed**: `VcsArtifactApp::produce_media` (`🔌️plugin/🦀️.rs:30756`, immediately above
`export_media`) asks `self.export_media(port)` first for any port other than `artifact:out`,
projecting `MediaPayload::Structured{schema, json}` to `data = json.into_bytes()` with a
`MediaWireFormat::Binary{format_kind: schema}` descriptor. `artifact:out`, and any port whose app
answers `MediaError::NotImplemented`, keep the document-pack answer **bit for bit** — so no path
that works today changes shape. `cargo check -p semio-s-plugin-draw --target wasm32-wasip2
--profile wasm-dev` rc=0 (`🗑️generated/wr3-plugin-check.txt`).

### 4.5 The shell side of the same port (LB1 §11.1, AP1's (f8))

One port, both channels. The guest half is §4.4; the shell half was LB1's last named refusal —
*"no media OUT port exists on `PluginWasmHandle` at all"*. Landed here, three hunks:

| file:line | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` (`AppChannelClient`, beside `loadDocument`) | `mediaOut(port)` — sends the SAME `AppCommand::MediaOut` the headless gateway sends |
| `🔌️PluginRuntime/🟦️.tsx:182` + the `adaptPluginHandle` body | `exportAppMedia?(instanceId, port)` on `PluginWasmHandle`, the exact sibling of `readAppDocumentPack`: `Error` frame → named throw, `Media` frame → `{port, descriptor, data}`, otherwise `null` |
| `🏛️ShellHost/🟦️.tsx:10334` | the `exportMedia` arm answers `{kind: "exported", …}` off that port; the two refusal paths (no port on the handle, no `Media` in the reply) each name themselves |

No fixture change was needed — LB1 §10.2 had already pinned `exported{port,descriptor,data}` on
both banks. `agent-bridge-check`: **55 passed / 55**, exit 0
(`🗑️generated/wr3-agent-bridge-check.txt`), i.e. the bar LB1 left is unbroken and the arm that used
to refuse now has a body.

## 5. Measured

| # | what | evidence | verdict |
| --- | --- | --- | --- |
| 1 | the seven new paged routes decode in the guest's own cursor, bounded, and an undeclared tag is still refused by name | `wr3-paged-route-laws.txt` | **12 passed / 0 failed** |
| 2 | `semio-framework-os-kernel --lib` (the crate the channel compiles into) | same run | green, 2 m 31 s cold |
| 3 | `cargo check -p semio-framework-os-mcp --lib` after the gateway changes | `wr3-mcp-check.txt` | rc=0, 0 errors |
| 4 | `draw`'s committed descriptor now declares its export surface | `🔣️.json` editor app: `io.artifactSchema = "drawing.document"`, `io.ports = [{"id":"vector:out","direction":"out",…}]` (was `""` / `[]`) | **landed**, emitted by `draw`'s own `describe` (`wr3-draw-describe.txt`, `descriptor=5f6a4c6d…`) |
| 5 | `artifact_export` end to end over the real `.mcp.json` `semio` server, against a real `s.draw.drawing` artifact | `wr3-export-probe.txt` | **`isError=false`, `port=vector:out`, 824 base64 bytes, `availablePorts=["vector:out","artifact:out"]`** |
| 6 | the exported bytes are the app's own SVG, not its pack | `wr3-export-probe.txt`, after §4.4 and a `draw` rebuild through the fleet mutex | **`svgMarker=yes`** — `<svg viewBox="0 0 1024 1024" … ><g id="layer-root"/></svg>`, 117 bytes, painted by `draw`'s own `drawing_document_to_svg` and carried over `MediaOut` |
| 7 | the shell route's `exportMedia` arm answers instead of refusing (§4.5) | `wr3-agent-bridge-check.txt` | **55 passed / 55**, exit 0 — the acceptance bar LB1 left, unbroken |
| 8 | the `client-e2e` gate | `wr3-client-e2e.txt` | **12/16** — and it is no longer measuring the headless lane at all: §5.1 |

### 5.0 The permanent gate is being served by the LIVE shell, not the headless workspace

The last run (`wr3-client-e2e.txt`, 16:30) is **12/16**, and every red in it is a *shell-route*
refusal — in a gate whose server is started with `--folder <tmpdir>`:

```
PASS  os: artifact_create (a real plugin artifact kind) — kind=s.animate.presentation pluginId=animate sizeBytes=28380
FAIL  os: artifact_export — `animate` rejected ExportMedia on `artifact:out` (plugin.unavailable):
      this shell has no media OUT port … `PluginWasmHandle` exposes none …
FAIL  os: action_prepare — capability `animate.…#editor.setFrame` is owned by plugin `animate`,
      which has no open instance in the attached shell (the shell reports 1 open instance(s))
```

`the shell reports 1 open instance(s)` is decisive: `HeadlessWorkspace::open_session_artifact_channel`
resolved `ChannelKind::Shell` and drove AP1's live shell. The typed `artifact_create` **passed**
through it (28 380 B), and both remaining reds are LB1/AP1's shell-route items — the second of them
is AP1's own (f8), which §4.5 now answers. Two consequences worth naming:

1. **This gate no longer proves the headless lane.** Whether it is green depends on whether a peer's
   shell happens to be attached and what it has open. A `--folder` gate should pin
   `ChannelKind::Headless`; channel selection is LB1's (`🌉️mcp/🦀️.rs` `server_for_workspace_options`
   → `bind_shell_route`), so this is reported, not changed here.
2. **The headless lane itself is green for this slice's legs**, measured directly with
   `🐍️wr3-export-probe.ts` (same client class, one named kind, `channel@t0 headless`):
   `draw` → `vector:out` SVG, `animate` → `artifact:out` 487 B, both `isError=false`.

### 5.1 The gate's target plugin moved, mid-session, to one that hangs

`wr3-client-e2e.txt` (this slice's run, after everything above) no longer reaches `artifact_export`
or `action_prepare` at all. The journey picks its target by
`capabilities_search({query:"set", kind:["mutation"]})` and takes **hit 0**. With `animate`'s
descriptor repaired by A3's `describe` sweep at 14:14, hit 0 changed from
`draw.…#editor.setSnapshot` to `animate.s.animate.presentation@1/*#editor.setFrame`, and
`artifact_create` for `animate`'s kind **never answered inside the client's 240 s budget**:

```
PASS  os: capabilities_describe — animate.s.animate.presentation@1/*#editor.setFrame input={}
error: tools/call did not answer within 240000ms
```

`animate`'s component is **104 015 592 B** (rebuilt 14:14) against `draw`'s 62 484 148 B — the
largest staged component in the repo, and the first one this gate has ever tried to open. The
catalog also improved underneath the run: **44 diagnostics, down from WR2's 58**, with only
`cad-extension-aec-building-energy`, `stdio` and `trinity` still named — A3's sweep landing.

So the gate is currently blocked on a plugin nobody has opened headlessly before, not on the three
reds this slice owns. `🐍️wr3-export-probe.ts` exists precisely because of that: it drives the same
client class against **one named kind**, so an export measurement survives a moving target.

## 6. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` | §2 — `route_field_plan`, `PagedRouteFieldsDecode`, `TRANSACTION_PREPARED_OPS_MAXIMUM`, the `RouteFields` state + header dispatch, bounded close for the cursor and for `DecodedAppCommandOwner`, the trailing-field arms, and the encoder's own prepared-ops cap |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` | §2.1 — four new laws driving the PAGED cursor (`🔖️PagedRouteAdmission`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | §4.1 — `app_media_out_ports`, and `installed_artifact_kinds` reading it |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | §4.4 — `VcsArtifactApp::produce_media` (one new method, immediately above its `export_media`) |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | §4.2 — `.io(drawing_io())` on `create_drawing_app` |
| `✏️s/🔌️plugins/🖍️draw/🔣️.json` + `🛂️.descriptor.semio` | regenerated by `draw`'s OWN `describe` verb, never hand-edited |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | §4.5 — `AppChannelClient.mediaOut(port)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | §4.5 — `PluginWasmHandle.exportAppMedia` (declaration + `adaptPluginHandle` body) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | §4.5 — the `exportMedia` arm's real body |

Probe: `🐍️wr3-export-probe.ts`. Captures: `wr3-paged-route-laws.txt`, `wr3-mcp-check.txt`,
`wr3-plugin-check.txt`, `wr3-draw-describe.txt`, `wr3-mcp-build.txt`, `wr3-client-e2e.txt`,
`wr3-export-probe.txt`, `wr3-draw-rebuild.txt`.
Staged: `draw`'s rebuilt component copied (rm+cp) into the shared
`⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_draw.wasm`, 62 484 148 B; the MCP binary
rebuilt through `bun ./📜️script.ts build`.

## 7. Honest gaps

1. **`action_prepare` is unchanged and still red** — root-caused one layer below WR2 (§3): the pure
   lane's `dispatch_command_frame` is an unconditional refusal in the shared SDK, and the window
   lane cannot be substituted for it without applying the mutation at prepare time. The fix is
   named and scoped in §3.3; it is a slice of its own.
2. **The permanent gate is not green, and it is no longer measuring the headless lane** (§5.0). It
   is 12/16 with `capability catalog health` (A3's, 44 diagnostics and improving) plus two
   shell-route refusals. Pinning `--folder` to `ChannelKind::Headless` is LB1's call and is the
   single change that would make this gate mean what its name says again.
3. **§4.5 is proven by the bridge suite, not by a live shell.** `agent-bridge-check` is 55/55 and
   the arm has a real body, but no `exportMedia` has yet been driven through a booted editor
   session — AP1's (f8) is the run that would show it.
4. **Only `draw` is proven on the new media route.** The transaction routes are proven by native law
   against the real decoder, but no guest has yet been driven through `TransactionPrepare` over the
   headless ingress end to end — WR2's `plugin_artifact_channel_mutation_verbs_are_real_round_trips`
   should now reach further than `TransactionPrepare`, and that has not been re-run.
5. **Presence was not given a generic route, deliberately** (§2), which corrects WR2 §10 gap 2.
6. **Only `draw` and `animate` were rebuilt/driven on the new routes.** Every other staged
   component still carries the old decoder until it is rebuilt; peers' own `describe` runs pick the
   new routes up for free, since the change is in the shared channel crate.
7. **`🏃️run`'s workflow executor now sees port-specific media where it used to see the document
   pack** (§4.4). Its `consume_media` default accepts only a matching `Document{schema}` wire, so a
   port-to-port hop that silently transferred the wrong bytes now answers `NoImporter` by name.
   That is the honest shape, but it is a behaviour change nobody has re-run a workflow against.
