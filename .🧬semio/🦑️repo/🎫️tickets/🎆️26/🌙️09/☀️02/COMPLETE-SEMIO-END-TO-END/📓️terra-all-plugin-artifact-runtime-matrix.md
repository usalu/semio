# Terra All-Plugin/Artifact Runtime Matrix

Current-source, read-only audit — 2026-09-04. I ran only the registry's
bounded source-discovery function through Bun; it read manifests/descriptors
and did not build components, generate files, start a hub, or run browser/native
code. No runtime result is claimed below.

## Verdict

**RED for ordinary startup of every registered first-party plugin and artifact.**

The current registry discovers 59 component-bearing rows (33 plugins and 26
extensions) and 92 first-party plugin artifact directories. The static hub
provider knows exactly two package identities—`stdio` and `gis`—and can expose
26 and 2 codec factories respectively. That is valuable source authority, but
it is not a trusted selected bundle, document-open target, ShellHost lease, or
create/edit/save/export journey.

The direct source audit is stricter still: it returned **59 issues and zero
strict owner descriptor pairs**. Nineteen rows have no owner JSON/pack pair;
31 existing JSON descriptors exceed the registry's 64 KiB bound; four CAD
extensions declare the wrong role; and five Flow extensions declare the wrong
package id. Consequently no checked-in first-party descriptor is current
strict-catalog evidence. This does not prove every package fails a fresh
component build: `catalog-complete` deliberately ignores owner descriptors in
its source phase and instead expects fresh staged receipts. It does prove that
the current owner descriptors cannot be used as ordinary startup authority.

## Evidence and boundary

The registry source of truth is
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`:

* `auditPluginCatalogSources` discovers 59 Cargo component identities at
  lines 2443–2520. With `ownerDescriptors: "ignored"`, current source discovery
  itself has 59 valid identities. With its default owner-pair validation, the
  bounded read-only audit observed zero `sources` and the 59 issue categories
  below.
* `CATALOG_DESCRIPTOR_MAX_BYTES` is 64 KiB (line 2040) and is applied to both
  owner JSON and pack (lines 2391–2392). Hub trusted bundles allow a distinct
  4 MiB descriptor file maximum at
  `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:22`; this divergent
  limit is a policy decision that must be made once, not bypassed per plugin.
* `CatalogCompleteScript` reads only source identities with descriptors ignored,
  then requires a caller-owned fresh raw/core/descriptor receipt per row
  (registry script lines 2877–2904). No such 59-row fresh receipt is current
  runtime evidence.
* `NativeCodecProviderSetV1::linked` contains only `stdio` and `gis`
  (`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24–33`).
  The trusted loader consumes a binding only when a selected bundle row declares
  the same `(plugin, package, kind, schema, pack hash)`
  (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:344–504`).
* Hub startup supplies no catalog unless both server-owned
  `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` are set
  (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380–396`). It does not create a catalog
  from a browser, dev-cache, registry projection, or linked factory.

### Matrix legend

`D` is the owner-descriptor state observed by the strict source audit:

* `O64` — JSON/pack pair exists, but JSON exceeds the 64 KiB registry bound;
* `M` — both owner descriptor forms are absent;
* `R` — descriptor role differs from its Cargo component role; and
* `P` — descriptor `packageId` differs from Cargo component identity.

`Bundle` is a selected fresh trusted bundle, not a source descriptor. It is
`none` for all rows. `Native provider` means a compiled-in factory source only;
it is not an activation claim. `Ordinary UI + document path` combines browser
or native selected surface, create/open/save/edit/export, and a real process
law. It is `none` for every row because no row has all preceding authorities.

## Plugin/artifact matrix — 33 plugin rows / 92 artifact directories

| Plugin / first-party artifact roots | D | Selected bundle | Native provider | Ordinary UI + document path / real law |
| --- | --- | --- | --- | --- |
| `animate`: presentation | O64 | none | none | no selected surface, create/open/save/edit/export, or process law |
| `architect`: program | O64 | none | none | same |
| `block`: 2d, 3d, 5d | M | none | none | same |
| `cad`: cad | O64 | none | none | same |
| `dag`: dag | O64 | none | none | same |
| `demonstrator`: playground | O64 | none | none | same |
| `draw`: drawing | O64 | none | none | same |
| `energy`: model | O64 | none | none | same |
| `fem`: 2d, 3d | O64 | none | none | same |
| `flow`: flow | O64 | none | **none** — no Flow entry in `NativeCodecProviderSetV1` | source lifecycle/member work is not a trusted catalog/open path |
| `forms`: forms | O64 | none | none | no selected surface or end-to-end law |
| `gis`: gismap, gisterrain | O64 | none | **2 GIS codec receipts**, source-only | no trusted GIS bundle/open target, UI lease, edit/export, or process law |
| `imperative`: procedure | O64 | none | none | no selected surface or end-to-end law |
| `layout`: layout | O64 | none | none | same |
| `lowpoly`: lowpoly | O64 | none | none | same |
| `mathematical`: equation | O64 | none | none | same |
| `norm`: din4108, din16798, din18599, en1990–en1999, iso16757, vdi3805 | O64 | none | none | rich source UI/schema does not constitute ordinary activation |
| `note`: note | O64 | none | none | no selected surface or end-to-end law |
| `playbook`: playbook | M | none | none | same |
| `procedural`: assembly, generation2d, generation3d | O64 | none | none | same |
| `process`: process3d | O64 | none | none | same |
| `puzzle`: 2d, 3d, 5d | O64 | none | none | same |
| `raster`: raster | O64 | none | none | same |
| `reasoning-mindmap`: wires | O64 | none | none | same |
| `remodel`: remodeling | O64 | none | none | same |
| `s` (space): home, space | O64 | none | none | same |
| `sequence`: sequence | O64 | none | none | same |
| `shooting`: shooting | O64 | none | none | same |
| `sourcing`: curation | O64 | none | none | same |
| `stdio`: bcf, json, mp4, stl, avi, dxf, ifc, epw, wav, bmp, mp3, svg, semio, html, obj, deflate, zip, step, pdf, xml, md, tiff, txt, binary, jpg, png, dwg, xlsx, ply, docx, las, pptx, gif, gltf, csv, tsv | M | none | **26 stdio codec receipts**; the ten source roots `ifc, epw, wav, bmp, semio, html, txt, binary, gif, tsv` have no static native factory | no trusted stdio bundle/open target or real client process law |
| `trinity`: jack, rewriting | M | none | none | same |
| `vcs`: vcs | O64 | none | none — VCS is expressly absent from static provider inventory | no selected VCS open/save path |
| `writer`: writer | O64 | none | none | same |

This table counts directory-shaped artifact roots only. It intentionally does
not treat a Rust leaf, local fixture, application declaration, raw descriptor,
or ignored development module as a registered/openable artifact.

### Extensions — 26 component rows, no standalone artifact surface

Extensions must be loaded through their selected host; none is an independent
document provider or standalone application route. Every row has `Bundle=none`,
`Native provider=none`, and no ordinary browser/native/process proof.

| Host | Extension rows | D | Current runtime classification |
| --- | --- | --- | --- |
| `cad` | aec-building, aec-building-energy, aec-building-structure, spatial-shape | R ×4 | host/extension identity is rejected before contribution selection |
| `flow` | bim, draw | M ×2 | absent descriptor pair |
| `flow` | brep, math | O64 ×2 | over-bound raw descriptor |
| `flow` | dictionary, list, logic, primitive, text | P ×5 | wrong component package identity |
| `imperative` | control, effect, logic, math, text | M ×5 | absent descriptor pairs |
| `playbook` | procedural | M | absent descriptor pair |
| `process` | concrete, metal, robotic, wood | M ×4 | absent descriptor pairs |
| `sourcing` | beams, slabs, windows | M ×3 | absent descriptor pairs |

No extension table row has an artifact root by design. A UI screenshot, module
directory, or direct unit fixture cannot prove host selection, exactly-once
contribution, detach/revoke, or document authority.

## Browser and native cut-through

The browser shell has a source-level generic module path:
`loadPluginModule`/`fetchDescriptorManifest` are exported from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`,
and Shell state records loaded plugin URLs. The current tests there use mock
modules and descriptor fixtures. They do not bind a hub verified open plan's
component SHA-256/BLAKE3, descriptor SHA-256, generation, surface, grant,
scope, and parent dialect to a browser instance. Therefore all browser app
surfaces in the matrix remain source/mock coverage.

Native/WGPU is weaker for ordinary startup: its current
`wasm_program_exchange::attach_backbone` explicitly returns a “retired … no
replacement has landed” error at
`…/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281–282`. Its `load_wasm_plugins`
scans a module directory, can skip malformed/missing packages in space mode,
and substitutes an empty manifest when a local descriptor cannot be parsed
(`:723–805`). That is useful diagnostic loading, not trusted catalog selection
or document activation. It cannot close any matrix row.

## Narrowest next P0, ranked after the current Flow/GIS work

### 1. `stdio + gis` trusted two-package startup closure — first P0

Do **not** attempt 59 activation rows. The only currently declared static
factory closure is `stdio` plus `gis`; GIS depends on stdio. Define one server-only
fresh receipt/bundle profile containing exactly these two packages and only the
26+2 native codecs that the provider returns. Its component SHA-256 and BLAKE3,
descriptor SHA-256, package/version, codec schema hash, artifact kind, open
surface, grant, parent dialect, and catalog generation must agree end to end.

First resolve the descriptor policy as one schema contract: keep package
descriptors compact enough for the 64 KiB registry/staged bound **or** raise a
single bounded descriptor/closure limit with matching registry and hub hostile
tests. Do not accept a per-plugin exception. Generate `stdio`'s currently
missing pair and make a fresh receipt; regenerate/compact GIS's source pair;
then write the trusted bundle/profile from those fresh bytes. The bundle is
server input only—not a client-uploaded manifest or a dev cache.

This P0 can prove one actual GIS or stdio document plan and codec open. It does
not prove a Flow document, generic plugin UI, inference, mutation, or all
catalog activation.

### 2. Flow provider/open boundary — second P0

After public member opening is accepted, add an explicit Flow codec factory
receipt and provider entry from a verified Flow descriptor. The current static
provider table has no Flow branch, so no trusted Flow bundle can honestly open
today. Bind it to the same server-issued execution target lease rather than a
WGPU directory scan or browser module URL. This packet is independent of the
other 30 plugin providers but depends on the first packet's strict bundle
materialization shape.

### 3. Generated verified provider-set expansion — later, not a manual list

For each remaining plugin artifact, emit a native binding only from a verified
fresh receipt that joins descriptor identity to a concrete factory. Extensions
are tested as host contributions, not codecs. Withhold rows that are headless,
unsupported, or lack an explicit factory; do not manufacture an empty binding
to reach a count. This is the proper route to all-catalog activation.

## Required proof packet

### Neutral, language-agnostic fixture/oracle

Extend the existing native catalog-selection fixture/oracle registered by
`@semio-tech/plugin-registry:native-catalog-selection-check` (project
`📋️project.json:27–32`) with a two-package `stdio + gis` profile. Independently
encode/decode the bundle and assert:

* exact selected dependency closure and rejection of missing stdio/GIS, foreign
  package, duplicate codec, extra provider binding, wrong role/package,
  noncanonical/oversize descriptor, component SHA-256/BLAKE3, descriptor SHA,
  schema hash, surface, grant, parent dialect, and catalog generation;
* no input from a browser/module URL can select a package; cancellation/failure
  publishes no partial catalog generation; and
* a profile with `flow` is denied until an explicit Flow provider receipt exists.

### Focused native laws

After P0 implementation, run new exact laws through the existing hub and
registry `📜️script.ts` targets—not broad cache scans:

1. fresh trusted two-package receipt loads atomically and creates exactly one
   GIS or stdio open target; its changed byte/hash/version/surface/grant/parent
   dialect rejects before codec registration;
2. missing profile/bundle, bad dependency order, cancellation, and rotation
   leave the prior catalog generation usable or no catalog installed, never a
   hybrid; and
3. Flow selection rejects absent a concrete Flow factory; the ten non-factory
   stdio artifact roots reject as unavailable rather than selecting a different
   codec.

### Real process/browser/native law

Only after P0, launch a local hub with a fresh server-owned bundle/profile and
open one selected GIS/stdIO document using the actual open-plan/lease. Verify
the browser receives the exact immutable target identity before it loads a
surface; verify a native target either attaches the same target or reports the
current backbone limitation. A second principal reopens the persisted document
and sees one authorized edit. Wrong hash, revoked grant, catalog rotation, and
cancelled plan must not load or mutate. This is a new process law: current
registry checks, factory unit tests, mock ShellHost tests, and WGPU directory
scan are not substitutes.

## Nonclaims

No current first-party plugin/artifact has been credited as browser/native
runtime-ready, create/open/save/edit/export-ready, or process-proven by this
audit. The source inventory, static stdio/GIS receipt code, and individual
artifact tests remain useful prerequisites only.
