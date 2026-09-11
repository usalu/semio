# Assembly Artifact Mount (2026-09-11)

Closes the unmounted assembly editor/viewer packet from `📓️assembly-mount-2026-09-10.md`.
The procedural plugin now declares a real assembly app (not only generation2d/generation3d).

## Apps

| App id | Surface |
|---|---|
| `s.assembly@1/*#editor` | `AssemblyEditor` |
| `s.assembly@1/*#viewer` | `AssemblyViewer` |

Plugin identity: `s.procedural.assembly`. Document dialect: `s.assembly@1/*` (schema id `s.assembly`).
Playground: `variant = "assembly"`, ports react **6019** / wgpu **6119**.

`ProceduralApps` now includes `AssemblyEditor` and `AssemblyViewer`. `plugin()` registers
`.artifact(assembly::declaration())`, editor+viewer mutation rosters, examples, and
`OnArtifactKind` activation.

## Window kinds

| Surface | Kind | EN | DE |
|---|---|---|---|
| editor + viewer | `framework.window.tree` | Structure | Struktur |

Editor structure window: nine mutation actions (create/delete slot/rule, connect/disconnect slots,
change/remove weight, change-seed) plus `set-node`. Extra actions are
`InteractiveJobClassification::Migrated`.

## Content (not empty envelopes)

Examples (real DSL, EN+DE labels):

- `two-room-corridor` (seed 7, 3 slots)
- `wall-roof-facade-strip` (seed 42, 4 slots)

Codecs: `AssemblySnapshot` (`ArtifactDsl` + `ArtifactPack`, envelope `procedural.assembly`),
`AssemblyMutation` (`OpText` + `OpBinary`), `AssemblyEditorCommand: OpBinary`.
Native composer writes `s.assembly@1/*`. `s.stdio.txt@utf-8` is import/export format only —
generation3d already owns that dialect claim in this plugin.

Schema-first: `assembly_artifact_schema_descriptor()` (20 leaves) + inferences +
`derive_artifact_facets!` → `AssemblyBuilder` / `AssemblyAnalyzer` / `AssemblyComposer`.

Localization: EN “Assembly” / DE “Montage” on the artifact; window EN/DE as above.

## Tests run

| Target | Filter / file | Result |
|---|---|---|
| `semio-s-artifact-procedural-assembly` `--features component-app-assembly --lib` | `mount_contract`, `structure`, `example` | **28 passed**, 0 failed, 1 ignored |
| `semio-s-plugin-procedural --lib` | `assembly_apps_are_declared_on_the_plugin`, `assembly_manifest_examples_are_registered_on_the_editor_surface`, `assembly_editor_and_viewer_share_dialect` | **3 passed** |
| mount-contract Python | language-agnostic | **ok** |

Mount-contract triad: Rust, TypeScript, Python, feature, `expected.json`.

## Claim constraints learned while mounting

- Artifact identity **must** be `s.procedural.assembly` (`s.<plugin>.<artifact>`).
- Native composer **writes** `s.assembly` and **reads** `s.procedural.assembly` so the owner check passes without stealing `s.stdio.txt`.
- Composer capability claims must **exactly** equal the write-dialect coordinate (`s.assembly@1/*`). A second dialect claim fails `runtime-capability`.
- `s.stdio.txt@utf-8/*` is already claimed by `s.procedural.generation3d.composer.txt`.

## Files created

- assembly schema descriptor + derived facets
- assembly IO (native DSL/pack + txt import/export helpers)
- assembly mount-contract tests (Rust / TypeScript / Python / feature / expected.json)
- this report

## Files updated

- assembly crate root: `declaration()`, `definition()`, schema/IO mounts, EN+DE
- assembly editor: structure actions, OpBinary command, Migrated jobs
- assembly Cargo.toml: stdio-txt + `component-app-assembly`
- procedural plugin root: `ProceduralApps` + artifact/editor/viewer/examples/activation
- procedural plugin Cargo.toml: assembly dep + playground row
- procedural surface tests: assembly app/example/dialect tests (borrow fix)

No ShellHost / wgpu plugin-bridge / poll ABI edits.
