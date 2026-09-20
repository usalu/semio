# Host, Authored-Material, and Raster-Residency Boundaries

Read-only source audit, 2026-09-20. No build or runtime pass was run. Citations are repository-relative.

## A. Hub/session boundary

### Confirmed current state

The React contract is complete enough to name the required boundary. `hubConnectionSummaryV1` folds **every attached** `ArtifactSyncStatus`: session `signedOut` wins; otherwise `live`, `connecting`, `backoff/reconnecting`, then offline; live peers use the maximum, never a sum (`📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx:173-258`). `ShellHost` owns `verifiedSessionAuthority`, passes `Object.values(syncStatusByDocumentId)`, and opens a real mounted workspace through `openHubWorkspace` (also `/hub` in host mode) (`🧱️elements/🏛️ShellHost/🟦️.tsx:2467-2494,2652-2677,8998-9045,10885-10920`). The footer only offers `framework.hub.signIn` when that verified authority is absent.

The current WGPU working tree has already added a useful first slice: `hub_connection_state()` maps **one native** `sync_status` to `ShellHubConnectionState`; the wasm branch hard-codes offline (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8500-8527`). Its footer renders `s-hub-connection` through the ambient status painter, which has no event (`:14636-14657,22220-22226`); the existing footer test intentionally asserts these status hits are not actions (`🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs:330-385`). This is not the required sign-in/workspace seam. It should be retained as the native single-document input to the new aggregate, not reimplemented.

The authoritative transport already exists. `ArtifactHost` is the host-process registry for every document actor and fan-out; `ArtifactEvent::Status(ArtifactSyncStatus)` is its status event (`🏪️store/🔄️sync/🦀️.rs:522-565,1207-1260`). Its policy is explicit: native uses bounded worker-pool turns, browser wasm has the browser-local actor, and WASI-P2 does not link this host actor (`:1-13`). The current WGPU shell deliberately owns one `ShellSyncChannel`, closes it when another document opens, and stores one `sync_status` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1080-1097,8316-8328,8854-8876`).

The canonical directory layer is also present: `ShellDirectoryClient = DirectoryClient<ShellDirectoryTransport>`, native transport is `NativeDirectoryTransport`, browser transport is `BrowserDoorDirectoryTransport` (`🎯️targets/🧊️wgpu/📇️directory-door/🦀️.rs:188-194`). It already uses the same `/auth/sessions/me`, `/directory/spaces`, and `/directory/commands` routes as React (`📇️directory/🔌️client/🦀️.rs:4,208-211,827,967`; `🎯️targets/🧊️wgpu/📇️directory-door/🦀️.rs:17-18`). Browser Shell currently polls only `/auth/sessions/me` and explicitly lacks the directory Home WS and document-sync backbone (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9815-9860`). No WGPU source occurrence of `framework.hub.signIn` or `openHubWorkspace` was found; existing Space Administration is a command/pane counterpart, not the Hub workspace.

### Smallest implementation packet

1. Give the **Shell host**, not the painter, a bounded document-subscriber registry keyed by the existing `ArtifactDocumentKey`. On attach/open, subscribe to that key's `ArtifactEvent::Status`; on detach, retire that subscriber. Publish a target-neutral `ShellHubProjectionV1`:

   ```text
   authority = NoVerifiedSession | VerifiedSession { authority_generation }
   documents = [{ document_key, remote: Detached|Connecting|Live{peer_count}|Backoff{...} }]
   ```

   It contains no bearer/session capability and has no UI-mutable `signedIn` boolean. The summary function consumes this projection and exactly implements the React priority, maximum-peer, and document-count rules. `VerifiedSession` is emitted only by the existing identity/session verification owner; credentials remain inside the directory/session owner.

2. Feed that projection to **both** native and browser WGPU Shells. Native fans out from the existing `ArtifactHost`; browser must wire the already-supported browser actor/Directory Door instead of reducing browser status to offline. WASI-P2 remains outside this host feature. Do not add a parallel auth system.

3. Make `s-hub-connection` conditional: ambient status in every state except `NoVerifiedSession`; in that state its action is `framework.hub.signIn` and opens one owned Hub workspace surface. That surface uses the existing auth/session protocol (`POST /auth/sessions`, then verified `/auth/sessions/me`) and existing directory client to list/open spaces. The footer is only the opener. `open space` must route through the Shell's existing document/space route/open-owner, not set a synthetic active-space field. The workspace stays openable in playground and host flows; host flow additionally routes `/hub`, matching React.

### Test seams and acceptance

Use `🏪️store/🔄️sync` unit coverage for simultaneous document actors, `ShellSync` component cases for the exact aggregate oracle (`🧱️elements/🔄️ShellSync/🧪️tests/🧩️component/🟦️.tsx:63-105`), Directory Door request tests, and WGPU identity/footer tests. Add one language-neutral fixture with three documents: detached, connecting, and two live peers (2 and 5), plus authority transitions. Assert both target summaries are `live, peers=5, documents=4`; assert signed-out masks live status and clicking the only actionable footer state opens the real workspace. The current footer no-action assertion must become conditional, preserving the ambient behavior once authority is verified. Runtime authentication remains unverified by this audit.

## B. Authored GLB PBR, shadows, and GPU texture residency

### Confirmed loss points

The lighting packet is source-present: world resolves ambient/sun and one `neutral_material` into `ScenePass3d` (`♾️infinite/🌍️world/🦀️.rs:7929,12173-12235`; `🖱️ui/🎬️scene/📐️math/🦀️.rs:1288-1338`). It is intentionally pass-wide. The WGPU GLB schema retains only POSITION/NORMAL/TEXCOORD_0/indices/mode; `GlbPrimitiveSchema` has no material, and the parser has no `materials`, `textures`, or `images` sections (`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:1151-1268,1646-1720`). The materializer flattens primitives into one `Mesh3dLease` and writes only geometry (`:2145-2468`). World therefore caches one mesh lease per URL and submits every GLB through the pass-wide neutral PBR (`♾️infinite/🌍️world/🦀️.rs:8922-8955,12173-12385`).

React is not an authored-material oracle today: `GlbInstanceMesh` loads with `GLTFLoader` but replaces each mesh's GLTF material with a new environment/selection `MeshStandardMaterial` (`🧱️elements/🌐️World3dHost/🟦️.tsx:2308-2358`). It does enable Canvas shadows, mesh cast/receive, and sun casting (`:7390-7475`; `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3661-3735`), whereas the WGPU `WorldEnvironmentShadowRecord` is explicitly dead and WGPU has no shadow-map consumer (`♾️infinite/🌍️world/🦀️.rs:844-890`). Thus authored PBR and shadows are a shared contract expansion, not a WGPU-only parity copy.

### Smallest schema-first packet

Extend the GLB schema and its world cache together; do not send GLB maps through `TexturedDraw3d` (that is the reference-plane pipeline).

```text
AuthoredGlbAssetV1
  parts: [{ mesh: Mesh3dLease, primitive_material: MaterialId }]
  materials: [PbrMaterialV1]
  images: [EmbeddedImageV1]

PbrMaterialV1
  base_color_factor: rgba
  base_color_texture: Option<TextureRef { image, tex_coord } >       // sRGB
  metallic_factor, roughness_factor
  metallic_roughness_texture: Option<TextureRef>                     // linear
  normal_texture: Option<TextureRef { image, tex_coord, scale }>     // linear
  emissive_factor: rgb
  emissive_texture: Option<TextureRef>                               // sRGB
  alpha: Opaque | Mask { cutoff } | Blend
  double_sided: bool

EmbeddedImageV1 = { buffer_view, mime_type, sampler }
```

Keep the current fixed GLB item-credit model (`GLB_SCHEMA_ITEM_CAPACITY = 512`, `🧊️renderer/🦀️.rs:938`) and validate image view spans, texture/image indices, MIME and decoded dimensions before allocation. Unsupported extensions or external image URIs must produce a named unsupported-asset result, never silently become neutral material. The current world image decoder (`image::ImageReader`) and bounded RGBA path are reusable (`♾️infinite/🌍️world/🦀️.rs:15002-15038`), but the **consumer** requiring repair is the world mesh draw/shader path: split draw batches by `parts`, bind per-material uniforms and sampled maps, add UV/tangent handling for normal maps, select opaque/masked/blended pipelines, and set culling from `double_sided`. `WORLD3D_SHADER` currently receives only vertex color plus global material; the only sampled world shader is the reference plane (`🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:240-288,343-383`).

Shadows need a separate `ShadowPassV1` consumer: directional-light matrix plus depth atlas, caster draws from every opaque/masked GLB part, and world fragment sampling with `enabled`, opacity and softness. It must be used by both React and WGPU; toggling `castShadow` alone is not a WGPU implementation.

### Existing capacities, claims, and the newly confirmed raster gap

The fetch/decode owner is already safe to extend: `WorldAssetIoAuthority` has 64 request slots, 1,024 × 16 KiB response pages, explicit cancellation and stepped return; the mesh registry has 256 slots (`♾️infinite/🌍️world/🦀️.rs:948,14065-14571`). Keep authored parts/images under that exact URL/generation/revision claim and retire all leases/images with the owning world asset.

Reference replacement reveals a separate GPU residency defect. World now removes obsolete `reference_pixels`, rejects late A bytes, and re-offers active B pixels on every render (`♾️infinite/🌍️world/🦀️.rs:10573-10639,12385-12410`; `🧪️tests/🧲️scene-input-residency/🦀️.rs:72-115`). But `RasterTextureTable` commits staged entries into `live` and retires only a previous entry with the **same key**; unrelated live keys persist until surface close (`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1959-2129`). It therefore fills at 256 entries / 256 MiB (`:818-822,1622-1725`) after distinct reference URLs even though CPU world residency remains one. Existing `scene-input-residency` tests only prove the CPU registry/source routing (`📺️renderer/🧪️tests/🧲️scene-input-residency/🟦️.ts:30-58`).

Before adding GLB maps, add a bounded `RasterKeepSetV1`/retirement packet owned by the presenter. It must union texture keys reachable from: (a) committed last-valid packet, (b) pending candidate/presenting packet, (c) prior packet during `PreparedRenderReplacement`, and (d) staged/upload/abort owners until their existing witness retirement is terminal. Retire only live keys outside that union, one stepped owner at a time. Do **not** evict based only on the newest world's draws: an in-flight or aborted frame can still own A. Then admit B. The presenter already exposes the necessary packet-lifetime boundaries (`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3129-3210`) and raster commit/abort witnesses (`🧊️gpu/🦀️.rs:819-850`).

### Fixtures and acceptance

Reuse `🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🧊️gltf-codec/🧊️single-triangle-embedded.glb` for geometry regression, and use the checked-in `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🏞️balconies/⚡️z/🧊️capsule-with-balcony_z.glb` as a real embedded base-color-texture/roughness asset. A source scan found 369 GLBs and this asset has one material, texture and image. Add a small authored-PBR GLB fixture covering all listed slots, alpha mask and double-sided; inspect it with the existing Three `GLTFLoader` as the third-party oracle, then compare the decoded `AuthoredGlbAssetV1` fields and WGPU draw-packet/pipeline selection. Add the scene-lighting fixture to the shadow runtime case: enabled sun/shadow must visibly affect a receiver; disabled must not allocate/use the shadow pass.

For raster residency, drive more than 256 distinct A→B reference replacements through prepared/presented/abort lifetimes, keep a prior packet alive across one replacement, then assert only union-reachable keys remain and B still renders. This is the missing test beyond the existing CPU-only law.
