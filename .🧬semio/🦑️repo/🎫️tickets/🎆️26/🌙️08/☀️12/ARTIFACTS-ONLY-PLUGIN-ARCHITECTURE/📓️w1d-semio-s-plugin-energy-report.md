# W1d — `🔋️energy` (`semio-s-plugin-energy`): `.setup()` elimination — verification pass

## Result: `.setup()` is GONE from this plugin. Zero residue.

The framework agent's W1d pass (`📓️w1d-declaration-gaps-report.md`, Gap A) already landed the fix
before this pass started — it added `ArtifactDeclarationBuilder::document_codec_bare::<Snapshot,
Mutation>(schema)` (sibling to `.document_codec::<A: ArtifactApp>()`, for headless library plugins
with zero `ArtifactApp`s) and used it to close energy's one `.setup()` reason. This pass **verified**
that landing rather than re-doing it.

## What was registered, and where it now lives

Energy is a headless library plugin (zero apps, one artifact `s.model`). Its only `.setup()` reason
was `register_document_codec`, registering `EnergyModelSnapshot`/`EnergyModelMutation`'s pack↔dsl
codec directly against `store::register_document_codec` — because `.document_codec::<A:
ArtifactApp>()` requires a real `ArtifactApp` to bind `A::Snapshot`/`A::Mutation`, and this plugin has
none.

- `✏️s/🔌️plugins/🔋️energy/🦀️component.rs` — `plugin()` is now `Plugin::builder("energy").label(…)
  .version(…).artifact(crate::artifacts::model::declaration()).library()`. No `.setup(` call anywhere
  in the file (grep-verified, whole-plugin-tree grep for `^\s*\.setup(` returns nothing).
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️component.rs` — `declaration()` gained
  `.document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA)`
  as its last builder call before `.build()`.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` —
  the old free fn `register_document_codec()` is deleted (its only caller was the removed `.setup()`
  call); a comment marks it deleted with the ticket reference.

`grep -rn 'register_document_codec' ✏️s/🔌️plugins/🔋️energy/` now matches only doc-comment prose
explaining the history — zero live call sites, zero live fn definitions.

## No residue to narrow `.setup()` to

Unlike puzzle (which kept `.setup()` for two OS media-bridge calls with no corresponding declaration
field), energy had exactly one `.setup()` reason and it is fully expressible by
`document_codec_bare` — same bounds the plugin's code already satisfied via the old
`store::ArtifactCodec::of::<EnergyModelSnapshot, EnergyModelMutation>(...)` call, per the framework
report. Nothing was invented or worked around.

## Verification performed this pass

**1. Every `#[path]` in `📦️glue.rs` resolves.** Parsed all 84 non-`"."` `#[path = "..."]` attributes
in `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`, resolved each relative to the file's own
directory, checked existence on disk. **All 84 resolve. Zero missing.**

**2. Every `include_str!`/`include_bytes!` resolves.** Found 36 invocations across 11 files under
the energy artifact tree (schema/inferences/snapshot/mutations/diff component+text+binary facets,
plus the demo example test). Resolved each argument path relative to its containing file's directory
per Rust's `include!` semantics. **All 36 resolve. Zero missing.**

**3. `cargo metadata --no-deps` — OK.**
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo metadata --no-deps \
  --manifest-path ✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml
→ exit 0 (only a harmless "specify --format-version explicitly" warning on stderr; 402,821 bytes of
  valid metadata JSON on stdout)
```
Log: `scratch-w1d-energy-metadata-stdout.txt` / `scratch-w1d-energy-metadata-stderr.txt`.

**4. `cargo check -p semio-s-plugin-energy --all-targets` — BLOCKED-CHURN, exit 101, classified (c) upstream.**
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-energy --all-targets
→ exit 101
error[E0433]: cannot find `inferences` in `schema`  (×4, all in ✏️s/🔌️plugins/🗄️stdio/…)
error: could not compile `semio-s-plugin-stdio` (lib) due to 4 previous errors; 601 warnings emitted
```
Full log (6,911 lines): `scratch-w1d-energy-check.txt`.

**Classification: (c) upstream, not energy's bug, not mine to fix.**
- `grep -c "Compiling semio-s-plugin-energy"` on the log → **0**. Cargo never reached
  `semio-s-plugin-energy`; it died in `semio-s-plugin-stdio`, an upstream dependency, first.
- `grep -i "energy"` on the log's error lines → **0 matches**. No error mentions any energy path.
- All 4 `E0433` errors are in `🗄️stdio` artifact engine/io files (`🎒️zip`, `🧊️obj`, `🟪️stl`, `📼️avi`),
  same error class (`cannot find inferences in schema`) the W1d framework report already documented
  as a live, in-progress edit by another session (`🗄️stdio` is explicitly peer-held, off-limits per
  the ticket's hard rules).
- **Live-churn evidence, checked directly by me, not inferred from the framework report:** `stat -f
  '%Sm'` on the 4 exact failing files shows mtimes `23:47:42`–`23:49:17`; this cargo check ran at
  `23:50:12` — the files were touched 1–3 minutes before my check, well inside an active edit window,
  not a stale pre-existing break. Per the ticket's "retry-and-wait, do not patch" protocol, `🗄️stdio`
  was not touched.
- **Confidence energy itself is sound absent a green run:** `document_codec_bare`'s where-clause
  (defined in `semio-framework-plugin`) type-checked cleanly — `semio-framework-plugin --all-targets`
  is independently reported green by the framework pass, and Rust checks a generic fn body against
  its own where-clause at definition time, independent of call sites. The energy call site
  instantiates it at exactly the `<EnergyModelSnapshot, EnergyModelMutation>` pair the prior (now
  deleted) `store::ArtifactCodec::of::<…>(...)` call already proved satisfies those bounds. This is
  reasoning, not a compiler result on this crate — flagged as such, not claimed as a pass.
- **Re-run needed once `🗄️stdio` is green.** `semio-s-plugin-energy --all-targets` should be re-run
  by whoever picks this back up once the upstream `🗄️stdio` churn settles; nothing in this pass's own
  diff is implicated.

## Files touched this pass

None — this was a verification-only pass. All source changes for this plugin were made by the
framework agent's W1d pass (see "Files touched" in `📓️w1d-declaration-gaps-report.md`, the five
`🔋️energy`-prefixed entries). Files written this pass are scratch logs only:
`scratch-w1d-energy-metadata-stdout.txt`, `scratch-w1d-energy-metadata-stderr.txt`,
`scratch-w1d-energy-check.txt`.
