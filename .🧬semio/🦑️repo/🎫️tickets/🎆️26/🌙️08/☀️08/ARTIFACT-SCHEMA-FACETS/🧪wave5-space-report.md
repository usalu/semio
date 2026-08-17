# Wave 5 Report — Space (`semio-s-plugin-space`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🪐️space/**` (production) plus this ticket folder.
Artifact `🏠️home`: key `home`, prefix `SHome`, schema id `s.space.home`. Former snapshot type `SHomeDocument` → `SHomeSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document schema id (`s.home`) |
| `catalogGeneration` | persistent | studio catalog generation counter |
| `activePanelTab` | local-ui | from `HomeConfig` |
| `locale` | local-ui | from `HomeConfig` |

Snapshot facet = exactly the two persistent fields. `DocumentApp::Draft` = `NoDraft`.

## 2. Diff-delta shape

`SHomeDiff` sparse field delta:

- `artifact: Option<Box<SHomeArtifact>>` — whole replacement wins
- persistent: `schema`, `catalogGeneration`
- local-ui: `activePanelTab`, `locale`

`MutationDiff<SHomeSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise.

`SHomeMutation` constructs `SHomeDiff` (`SetCatalogGeneration` → `catalogGeneration: Some(value)`; `SetSnapshot` → `diff_set_snapshot`).

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (unchanged from pre-refactor space glue). Nested:

- `artifacts::home::schema`
- `artifacts::home::snapshot::{schema, pack}`
- `artifacts::home::diff::{component, schema}`
- `artifacts::home::engine`

TypeScript `📦️index.ts` mirrors schema/snapshot/diff/pack exports.

## 4. Other structural changes

- Pack moved: `🎒️pack/` → `📸️snapshot/🎒️pack/`; language registration points at `snapshot::pack`
- `SHomeDocument` removed; `SHomeSnapshot` in `📸️snapshot/🧬️schema`, re-exported from artifact root
- `SHomeEngine` owns real `SHomeArtifact` + cached `SHomeSnapshot` (`type Artifact = SHomeArtifact`, never aliased to snapshot)
- `home_artifact_schema_descriptor()` registered from `register_pilot_languages` → `engine::register_artifact_schema`
- `HomeConfig` envelope id set to `home.config` (`#[dsl(id = "home.config")]`)
- Example `🗣️example.dsl.semio` replaced with round-tripping `semio shome v1` body
- Tests use `store::os_store::test_support::*`
- `semio-framework-schema` dependency added to plugin crate

## 5. Plugin space vs framework space

| | Plugin (`✏️s/🔌️plugins/🪐️space`) | Framework (`🧰️framework/…/🪐️space`) |
| --- | --- | --- |
| Scope | `SHome*` home artifact, Home app, Studio app shell | OS `SpaceProjection`, `OsSpaceDocument`, backbone catalog |
| Renamed | `SHomeDocument` → `SHomeSnapshot` only under `🗿️artifacts/🏠️home` | Untouched (`SpaceProjection` renamed by another wave) |
| Engine | `🗿️artifacts/🏠️home/⚙️engine` (`SHomeEngine`) | Version-controlled space container / zip IO |

## 6. Gate tails (verbatim)

### cargo check -p semio-s-plugin-space

```
error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1340:58
     |
1340 | fn write_zip_file<W: std::io::Write + Seek>(writer: &mut zip::ZipWriter<W>, name: &str, bytes: &[u8], options: zip::write::SimpleF...
     |                                                          ^^^ use of unresolved module or unlinked crate `zip`
     |
     = help: if you wanted to use a crate named `zip`, use `cargo add zip` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `zip` in this scope
    --> 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../../🔨️modules/🪐️space/🦀️component.rs:1346:58
     |
1346 | fn read_zip_entry<R: std::io::Read + Seek>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<Vec<u8>, SpaceZipError> {
     |                                                          ^^^ use of unresolved module or unlinked crate `zip`
     |
     = help: if you wanted to use a crate named `zip`, use `cargo add zip` to add it to your `Cargo.toml`

Some errors have detailed explanations: E0277, E0432, E0433, E0599, E0603.
For more information about an error, try `rustc --explain E0277`.
warning: `semio-framework-os` (lib) generated 8 warnings
error: could not compile `semio-framework-os` (lib) due to 116 previous errors; 8 warnings emitted
```

### cargo test -p semio-s-plugin-space --lib

```
(same `semio-framework-os` compile failure — tests did not run)
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'home|space'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter for home/space: **0** breaches.

## 7. Shared-surface blocker (fixup wave)

- `semio-framework-os` currently fails to compile (116 errors including missing `zip` crate on framework `🪐️space` module, plus `WorkflowMutationDsl` / `OpBinary` gaps). Blocks `cargo check` / `cargo test` for `semio-s-plugin-space` until framework fixup lands. Per brief, framework `🪐️space` is out of scope for this agent.

## 8. Not validated

- Lib test suite execution (blocked by framework-os build)
- Bundled `🗣️example.dsl.semio` Rust round-trip test (blocked)
- Full `bun nx run workspace:verify-gate`
- Runtime UI / playground smoke
