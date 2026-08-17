# W2-SDK2 Report — Closing the `SemanticMutation` Gap on `PluginBuilder::editor`/`::viewer`

Lane: W2-SDK2, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Escalation source:
`📓️w2-stdio-geometry-report.md`'s "SDK gaps found" §1 — 32 of the geometry packet's 40 stdio subsets
had complete, compiling `👁️viewer`/`✏️editor` trees that could not be registered into `plugin()`.

## Traced root cause

Read `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (`PluginBuilder::viewer`/
`::editor`, region `🔖️Surfaces`) and `…/🔌️plugin/🦀️component.rs` (`ArtifactEditor`/`ArtifactViewer`/
`EditorApp`/`ViewerApp`, region `🔖️Surfaces`).

`PluginBuilder::viewer::<V>`/`::editor::<E>` carried `where V::Mutation: protocol::SemanticMutation<
V::Snapshot>` (`::editor` mirrors it). Neither `ArtifactViewer` nor `ArtifactEditor` themselves declare
that bound — both only require `type Mutation: protocol::Mutation<Self::Snapshot> + …` (contract §2.1/
§2.2). Inside each builder method the bound fed exactly **one** thing: a non-capturing
`owner_mutation_roster::<V>() -> (&'static str, &'static [protocol::SemanticDescriptor])` thunk, pushed
onto `self.owner_mutation_rosters`, itself only consumed by `try_build()` →
`runtime.extend_contributions(…, &owner_mutation_rosters, …)` (`🔌️plugin/🦀️component.rs:3856`), which
folds `V::DOCUMENT_SCHEMA` + `<V::Mutation as SemanticMutation<_>>::kinds()` into the process-wide
`OWNER_MUTATION_ROSTER_REGISTRY` — the "owner half" of the WIT export `contributor.list-artifact-
mutations` (`WireMutationRosterEntry`, `🔌️plugin/🦀️component.rs:4117-4264`). Nothing else in
`try_build()` reads `owner_mutation_rosters`; an empty roster is not an assembly error (confirmed by
the host test's own comment, see Verification).

`protocol::SemanticMutation<P>` (`📡️spr/🎮️command/🦀️component.rs:516`) is documented on the trait
itself as "Implemented only by `#[derive(Mutations)]`, never by hand," and is explicitly framed as a
**future ratchet**: "End-state (final ratchet, once every artifact's dispatch enum implements it):
`ArtifactApp`/`ArtifactStore` bounds tighten from `Mutation` to `SemanticMutation`." That ratchet had
already been pulled early, and only on two of the SDK's four surface-construction entry points
(`PluginBuilder::viewer`/`::editor`; `document_app` carries the same bound but is being deleted per
contract §2.1, not touched here) — years ahead of the 32/40 stdio subsets (and, per the grep below,
most other plugins) whose mutation enums are still hand-rolled `impl protocol::Mutation`.

**Conclusion: option (a).** The bound gated a genuinely optional capability (roster registration, not
routing), exactly as contract §2.2 anticipates by making `ArtifactViewer::Mutation` decode-only ("the
store's op log must still decode"). Registering the app/routing side has no runtime or type dependency
on `SemanticMutation` at all.

## Fix applied (framework, in-lease)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`, region `🔖️Surfaces`:

- `viewer::<V>(def)` / `editor::<E>(def)` — **bound removed**. They now only build the `App`/factory
  and push `app_schema_descriptors`; any `V`/`E` satisfying `ArtifactViewer`/`ArtifactEditor` compiles
  and registers, full stop.
- Two new opt-in methods, `viewer_mutation_roster::<V>()` / `editor_mutation_roster::<E>()` — still
  carry the `SemanticMutation` bound, still push the same `owner_mutation_roster::<V>` thunk onto
  `self.owner_mutation_rosters`. Chain one after `.viewer::<V>(def)`/`.editor::<E>(def)` for a surface
  whose `Mutation` already derives `SemanticMutation`; skip it for the rest. `commit_owner_mutation_
  roster`'s idempotent-or-typed-conflict discipline (`plugin-assembly.owner-mutation-roster`) is
  unchanged — untouched code path.

This is additive: no existing call site's *compilation* changes (a removed bound only widens what
compiles). What changes is that `.viewer::<V>()`/`.editor::<E>()` alone no longer auto-registers a
roster row — call sites that want that capability now say so explicitly. See "Follow-up owed to other
W2 packets" below — this is real and I did not paper over it.

Verified with `RUSTC_WRAPPER="" cargo check -p semio-framework-plugin --all-targets --keep-going`:
**0 errors** (`🧪️w2-sdk2-framework-cargo.txt`).

## Fix applied (stdio, in-lease)

`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, region `👁️✏️SurfacesP3StdioGeometry`: all **40** geometry
subsets now registered (previously 8). For the 8 whose `Mutation` already derives `SemanticMutation`
(semio `brep`/`drawing`/`graph`/`kit`/`mesh`/`object`/`table`/`text` — unchanged from the geometry
report's audit) I chained `.editor_mutation_roster::<E>()`/`.viewer_mutation_roster::<V>()` right after
their `.editor()`/`.viewer()` calls, so their roster contribution is **unchanged** from before this fix
(zero regression inside my own lease). The other 32 (11 more semio + step×7 + ifc×5 + dwg×2 + dxf +
gltf + obj + stl + ply + las + bcf) are registered with `.editor::<E>(def)`/`.viewer::<V>(def)` alone —
no roster call, since their `Mutation` is hand-rolled and does not satisfy the bound. Every type/module
path was read off `📦️glue.rs`'s existing `P3-stdio-geometry` flat-module mounts and each subset's own
`pub struct …Editor`/`…Viewer` — none guessed. No `🧬️schema/**`/`🚪️io/**` file touched; `📦️glue.rs` was
not touched (all 40×2 module mounts already existed from the original geometry packet).

Only `👁️✏️SurfacesP3StdioGeometry` was edited; the sibling `SurfacesP1StdioMedia`/`SurfacesP2StdioData*`
regions (owned by other packets) were not touched.

### Surface count, before vs after

| | editor+viewer registrations in `plugin()` |
|---|---:|
| Before (P1 34 + P2 62 + P3 16) | **112** |
| After (P1 34 + P2 62 + P3 80) | **176** |

Delta: **+64** = 32 subsets × 2 roles — exactly the geometry packet's reported gap. Counted live with
`grep -c 'builder = builder\.editor::<\|builder = builder\.viewer::<' "✏️s/🔌️plugins/🗄️stdio/🦀️component.rs"`
(176, current file) against the arithmetic of the pre-edit region sizes (P1 17×2, P2 (7+19+5)×2, P3 was
8×2). This also reconciles with the geometry report's corrected subset count (40, not 44) and with
stdio's own subset total: 17(P1)+31(P2)+40(P3) = 88 subsets × 2 = 176.

## Verification actually run

- `RUSTC_WRAPPER="" cargo check -p semio-framework-plugin --all-targets --keep-going`: **0 errors**
  (`🧪️w2-sdk2-framework-cargo.txt`).
- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets --keep-going`, three full runs
  (`🧪️w2-sdk2-cargo-run1.txt` → `-run2.txt` → `-run3.txt`; final copied to `🧪️w2-sdk2-cargo.txt` = run3):
  771–816 errors total across the three runs, **0 with a primary `-->` location anchored in either file
  I touched** (`🔌️plugin/🏗️builder/🦀️component.rs`, `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`) in any run.
  The error *count* moved between runs (815 → 816 → 772) and run2/run3 additionally showed errors in
  `📦️glue.rs` and `…/✳️mesh/✏️editor/🦀️component.rs` that were absent from run1 — confirmed via
  `git status --porcelain` (both `M`, uncommitted) and `stat` (mtimes ~17:12–17:22, i.e. during this
  session, after my edits landed) that neither file was touched by me; both are being actively grown by
  the live FULL-STDIO peer ticket (`📦️glue.rs` is already documented mid-growth across sibling packets
  in the geometry report) — the same class of concurrent-workspace churn that report pre-warned about.
  Not my regression, not my lease.
- No new `SemanticMutation`/`mutation_roster` compile errors in any run (grepped explicitly).
- Confirmed the roster-idempotency assumption for the 8 chained subsets: editor and viewer share the
  identical `Mutation` type and `DOCUMENT_SCHEMA` const per subset (checked `semio_brep`'s editor/viewer
  root files: both `type Mutation = SemioBrepMutation`, both `DOCUMENT_SCHEMA = SEMIO_BREP_DOCUMENT_
  SCHEMA = "stdio.semio.brep"`) — so `.editor_mutation_roster()` and `.viewer_mutation_roster()` push
  byte-identical `(document_schema, kinds())` rows, matching `commit_owner_mutation_roster`'s documented
  idempotent-re-registration discipline rather than tripping its typed-conflict path.
- Read the host test that exercises the roster end-to-end (`🔌️plugin/🖥️host/🦀️component.rs:3301-3304`,
  `cad_roster_bytes = cad.list_artifact_mutations().expect(…)`) — its own comment says `"cad's real
  (today likely empty) roster registers without conflict"`, i.e. it already tolerates an empty roster.
  Confirms the split does not risk that test regardless of what other plugins do.

## Follow-up owed to other W2 plugin packets (reported, not invented around)

31 other plugin-root files (outside stdio, outside this lease) call `.editor::<E>()`/`.viewer::<V>()`
today and will keep compiling unchanged, but any of their subsets whose `Mutation` already satisfies
`SemanticMutation` will silently stop contributing an owner-mutation-roster row unless their own
plugin-root file adds the matching `.editor_mutation_roster::<E>()`/`.viewer_mutation_roster::<V>()`
call. Found with:

```
grep -rln '\.editor::<\|\.viewer::<' ✏️s/🔌️plugins --include='*.rs' | grep -v 🗄️stdio | grep -v 🗿️artifacts
```

31 files: `✒️writer`, `➗️mathematical`, `🌀️procedural`, `🌊️flow`, `🌍️gis`, `🌿️vcs`, `🎞️animate`,
`🎥️shooting`, `🎪️demonstrator/🛂️manifest/🎪️demonstrator`, `🎬️sequence`, `🏗️fem`, `🏛️architect`,
`🏭️process`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`, `📐️cad`, `📕️norm`, `📖️playbook`,
`📜️imperative`, `📸️remodel`, `🔋️energy`, `🔱️trinity`, `🕸️dag`, `🖍️draw`, `🖨️raster`, `🗒️note`,
`🧩️puzzle`, `🧱️block`, `🪐️space`, `🪵️sourcing` (component.rs each, plugin root). This was flagged, not
fixed, because those files are each other packets' leases — same discipline the geometry packet used
reporting the original gap. Each packet should audit which of its subsets' `Mutation` already derives
`SemanticMutation` (same check the geometry packet ran) and chain the opt-in call for those.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — region `🔖️Surfaces`:
  removed the `SemanticMutation` bound from `viewer`/`editor`, added `viewer_mutation_roster`/
  `editor_mutation_roster`.
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` — region `👁️✏️SurfacesP3StdioGeometry`: registered the
  remaining 32 subsets, chained the roster opt-in for the original 8.

Not touched: `🧬️schema/**`, `🚪️io/**`, `📦️glue.rs`, any other plugin's `🦀️component.rs`, any other
region of stdio's plugin root.

Scratch (ticket folder): `🧪️w2-sdk2-cargo-run1.txt`, `-run2.txt`, `-run3.txt`, `🧪️w2-sdk2-cargo.txt`
(= run3), `🧪️w2-sdk2-framework-cargo.txt`.
