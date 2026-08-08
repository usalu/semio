# Wave 5 — norm `en1995`–`en1999` artifact schema facets

## Summary

Renamed bare `Document` to `En1995Snapshot` … `En1999Snapshot` with matching `XArtifact` / `XDiff`, added fifteen handcrafted schema leaves per artifact, moved `🎒️pack` under `📸️snapshot`, implemented sparse `XDiff` + `SetSnapshot` mutations, updated engines to `ArtifactEngine` with distinct `Artifact`/`Snapshot`, and updated matching `🎛️apps/📘️en1995`–`📘️en1999` play apps. Policy shows **zero** artifact-schema breaches for these five keys. `cargo check -p semio-s-plugin-norm` still fails on **unrelated** glue (`din4108` pack path) and **pending** glue wiring for this wave’s `snapshot`/`schema` modules.

---

## Per-artifact field inventory

All compliance input fields from the former `Document` struct are **`persistent`** on `XArtifact` and `XSnapshot` (equality). Shared inspection UI adds **`shared-ui`**: `selectedCheckIndex` (`Option<u32>`) on `XArtifact` only.

| Key | Snapshot fields (all `persistent`) | `shared-ui` on artifact |
| --- | --- | --- |
| `en1995` | `annex`, `mEdKnm`, `nEdKn`, `vEdKn`, `wMm3`, `aMm2`, `bMm`, `hMm`, `fMK`, `fC0K`, `serviceClass`, `loadDuration`, `mCritKnm`, `fEdKn`, `aEfMm2`, `fVK`, `fireDurationMin`, `sectionDepthMm`, `aVertMS2`, `nCyclesBridge` | `selectedCheckIndex` |
| `en1996` | 22 structural/masonry fields incl. `masonryClass`, `designSituation`, `exposure`, `mortar`, … | `selectedCheckIndex` |
| `en1997` | 22 geotechnical fields incl. `designApproach`, pile/footing/settlement fields, … | `selectedCheckIndex` |
| `en1998` | 49 seismic/retrofit/silo/tank/tower/foundation/wall fields | `selectedCheckIndex` |
| `en1999` | 26 aluminium fields incl. fatigue, weld, sheet, shell, `annex` | `selectedCheckIndex` |

`en1996` keeps helper enums (`MasonryClass`, `part_2::ExposureClass`, `part_2::MortarClass`) on the artifact root module.

---

## Diff shape (all five)

`XDiff`: optional `artifact: XArtifact` (whole replace, wins); optional scalar per persistent snapshot field; optional `selectedCheckIndex: Option<Option<u32>>` (`shared-ui`). Runtime: `MutationDiff<XSnapshot>`, `apply_to_artifact`, `absorb`, `diff_set_snapshot` helper. Sole mutation: `SetSnapshot { snapshot }` with `#[serde(tag = "mutation")]`.

---

## Glue / Cargo / TypeScript (integrator — verbatim pattern per key)

Replace `<key>` / `<Prefix>` (e.g. `en1995` / `En1995`). Repeat for `en1996` … `en1999`. Remove legacy `#[path = ".../🎒️pack/..."]` at artifact root.

```rust
        #[path = "../../🗿️artifacts/📘️<key>/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️<key>/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️<key>/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️<key>/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️<key>/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️<key>/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️<key>/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️<key>/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/📘️<key>/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
```

**Descriptor registration** (in glue registry section, one per artifact):

```rust
schema::register_artifact_schema(crate::artifacts::en1995::schema::en1995_artifact_schema_descriptor());
// … en1996::schema::en1996_artifact_schema_descriptor() through en1999
```

**`Cargo.toml`** (if not already present for GIS-style plugins):

```toml
semio-framework-schema = { path = "…/🧰️framework/…/schema/…" }
```

**`📦️index.ts`** (mirror GIS):

```typescript
export * from "./artifacts/en1995/schema/component";
// … snapshot/diff schema TS leaves as needed for parity tests
```

**Apps glue** (`📦️glue.rs` apps section): point `set_document` → `set_snapshot` module paths for `en1995`–`en1999` only.

---

## Gate tails (verbatim)

### Policy

```bash
cd /Users/ueli/Documents/semio && bun ./📜️script.ts policy 2>&1 | rg -i 'en199[5-9]'
```

```
```

(empty — no breaches)

Direct check:

```bash
cd /Users/ueli/Documents/semio && bun -e "import { policyArtifactSchemaBreaches } from './📜️script.ts'; console.log(policyArtifactSchemaBreaches(process.cwd()).filter(x => /en199[5-9]/i.test(x)).length)"
```

```
0
```

### `cargo check -p semio-s-plugin-norm`

```bash
cd /Users/ueli/Documents/semio && DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-norm 2>&1 | tail -20
```

```
warning: `semio-framework-plugin` (lib) generated 16 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Checking semio-s-plugin-norm v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/././../../🗿️artifacts/📕️din4108/🎒️pack/🦀️component.rs`: No such file or directory (os error 2)
  --> ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs:74:9
   |
74 |         pub mod pack;
   |         ^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-norm` (lib) due to 1 previous error
```

**FAIL** — blocked by sibling wave (`din4108` pack relocation) and missing integrator glue for `en1995`–`en1999` `snapshot`/`schema`/`diff::schema` mounts (expected before integrator runs).

---

## Files touched (high level)

- `🗿️artifacts/📘️en1995` … `📘️en1999`: `🧬️schema/*`, `📸️snapshot/🧬️schema/*`, `📸️snapshot/🎒️pack/*` (moved), `🔺️diff/🧬️schema/*`, `🔺️diff/🦀️component.rs`, `🧬️mutations/*` (`📄set-snapshot`), `⚙️engine`, `🗣️dsl`, `📡️spr`, `🔧️op`, `🦀️component.rs` roots
- `🎛️apps/📘️en1995` … `📘️en1999`: `🦀️component.rs`, `🎮️commands/📤️set-snapshot/*`
- Ticket probes: `🧪wave5-norm-en1995-1999-*.py`, this report

**Not edited (per brief):** `📦️packages/🦀️rust/📦️glue.rs`, `Cargo.toml`, `📦️packages/🟦️typescript/📦️index.ts`

---

## Not validated

- Full crate compile until glue integrator + `din4108` pack fix land
- `cargo test -p semio-s-plugin-norm --lib` (same blocker)
- SPR binary baselines after `SetSnapshot` wire change (may need integrator/fixup pass on `📡️component.protocol.semio`)
- Vitest TS package tests (not run)
