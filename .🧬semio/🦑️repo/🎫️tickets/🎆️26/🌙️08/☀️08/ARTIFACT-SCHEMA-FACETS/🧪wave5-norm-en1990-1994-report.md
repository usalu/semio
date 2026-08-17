# 🧪 Wave 5 — Norm `en1990`–`en1994` artifact schema facets

## Summary

Renamed bare `Document` → `En1990Snapshot` … `En1994Snapshot` with matching `En199XArtifact` / `En199XDiff`. Added fifteen handcrafted leaves per artifact (`🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`), moved `🎒️pack` under `📸️snapshot/🎒️pack`, wired sparse `En199XDiff` runtime + `SetSnapshot` mutations, and updated each artifact’s `⚙️engine` + `🎛️apps/📘️en199X` to use snapshot types. Shared `NormConfig.selected_check_index` is modeled as `shared-ui` on each `En199XArtifact` / diff. Policy gate is clean; `cargo check -p semio-s-plugin-norm` still fails until glue integrator remounts `schema` / `snapshot` / `pack` and `set-snapshot` paths.

---

## Per-artifact inventory

### `en1990` — `En1990Artifact` / `En1990Snapshot` / `En1990Diff`

| Field (camel) | State |
| --- | --- |
| `gK`, `qK` (`En1990QkEntry[]`), `resistanceKn`, `consequenceClass`, `annex`, `seismicAEdKn` | persistent |
| `selectedCheckIndex` | shared-ui |

**Diff shape:** optional `artifact` (whole `En1990Artifact`); optional scalars per persistent field; `qK` via `En1990QkList { values }`; optional `selectedCheckIndex` (`null \| uint32`).

### `en1991` — `En1991Artifact` / `En1991Snapshot` / `En1991Diff`

| Field (camel) | State |
| --- | --- |
| `areaM2`, `category`, `annex`, `selfWeightMaterial`, `selfWeightThicknessM`, `assumedGKKnM2`, `fireCurve`, `fireResistanceMin`, `fireMemberCapacityC`, `snowZone`, `snowAltitudeM`, `enSKKnM2`, `windZone`, `enVBMS`, `deltaTK`, `constructionActivity`, `accidentalMassT`, `accidentalSpeedKmH`, `bridgeLane`, `bridgeSpanM`, `bridgeLaneWidthM`, `bridgeMomentResistanceKnm`, `craneClass`, `hoistClass`, `hoistingSpeedMS`, `siloBulkDensityKnM3`, `siloHeightM`, `siloHydraulicRadiusM`, `siloMu`, `siloK`, `cS`, `cD` | persistent |
| `selectedCheckIndex` | shared-ui |

**Diff shape:** same sparse pattern as en1990 (optional `artifact` + per-field optionals + `selectedCheckIndex`).

### `en1992` — `En1992Artifact` / `En1992Snapshot` / `En1992Diff`

| Field (camel) | State |
| --- | --- |
| `annex`, `mEdKnm`, `vEdKn`, `fCk`, `bMm`, `dMm`, `aSMm2`, `fYk`, `rhoL`, `nEdKn`, `pKn`, `aCMm2`, `useFem`, `spanM`, `udlKnM`, `fireRating`, `providedAxisDistanceMm`, `bridgeSigmaCMpa`, `bridgeDeltaSigmaSMpa`, `tightnessClass`, `hdOverH`, `liquidSigmaSMpa`, `liquidRhoPEff`, `liquidFCtEffMpa`, `liquidESMpa`, `liquidSRMaxMm`, `anchorHEfMm`, `anchorCracked`, `anchorFUkMpa`, `anchorFYkMpa`, `anchorASMm2`, `anchorDMm`, `anchorC1Mm`, `anchorNEdKn`, `anchorVEdKn` | persistent |
| `selectedCheckIndex` | shared-ui |

**Diff shape:** optional `artifact` + per-field optionals + `selectedCheckIndex`.

### `en1993` — `En1993Artifact` / `En1993Snapshot` / `En1993Diff`

All former `Document` scalars/strings (annex, resistance/actions, fire, stainless, plated, silo, bolt, weld, fatigue, tension, HSS, bridge, tower, pile, crane fields — 60+ persistent members) plus `selectedCheckIndex` (shared-ui).

**Diff shape:** optional `artifact` + optional entry per persistent field + `selectedCheckIndex`.

### `en1994` — `En1994Artifact` / `En1994Snapshot` / `En1994Diff`

| Field (camel) | State |
| --- | --- |
| `annex`, `mEdKnm`, `vEdKn`, `mPla`, `mPlRd`, `eta`, `vLRd`, `insulationThicknessMm`, `fireRating`, `deckType`, `deltaSigmaMpa`, `fatigueDetail`, `dMm`, `hScMm`, `fCkMpa`, `fUMpa`, `eCmMpa`, `vEdPerStudKn`, `spanM`, `fYMpa`, `nCyclesStud`, `deltaTauStudMpa` | persistent |
| `selectedCheckIndex` | shared-ui |

**Diff shape:** optional `artifact` + per-field optionals + `selectedCheckIndex`.

---

## Glue integrator — apply for **each** of `en1990` … `en1994`

Replace `<key>` / `<folder>` (`📘️en1990` … `📘️en1994`). Inside `📦️packages/🦀️rust/📦️glue.rs` artifact module:

```rust
        #[path = "../../🗿️artifacts/<folder>/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/<folder>/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/<folder>/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/<folder>/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/<folder>/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        // REMOVE root-level:
        //   #[path = ".../🎒️pack/🦀️component.rs"] pub mod pack;

        // mutations: rename set_document → set_snapshot
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/<folder>/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/<folder>/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/<folder>/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
```

**`Cargo.toml`** (lib): register fifteen-leaf descriptors (pattern per key):

```rust
// in artifact_schema_registry() or equivalent:
registry.push(crate::artifacts::en1990::schema::en1990_artifact_schema_descriptor());
registry.push(crate::artifacts::en1991::schema::en1991_artifact_schema_descriptor());
registry.push(crate::artifacts::en1992::schema::en1992_artifact_schema_descriptor());
registry.push(crate::artifacts::en1993::schema::en1993_artifact_schema_descriptor());
registry.push(crate::artifacts::en1994::schema::en1994_artifact_schema_descriptor());
```

**`📦️packages/🟦️typescript/📦️index.ts`** (per key):

```ts
export type { En1990Artifact, En1990Snapshot, En1990Diff } from "../../🗿️artifacts/📘️en1990/🧬️schema/🟦️component.ts";
export type { En1990Snapshot as En1990SnapshotFacet } from "../../🗿️artifacts/📘️en1990/📸️snapshot/🧬️schema/🟦️component.ts";
export type { En1990Diff as En1990DiffFacet } from "../../🗿️artifacts/📘️en1990/🔺️diff/🧬️schema/🟦️component.ts";
// repeat for en1991–en1994 with matching prefixes
```

---

## Gate tails (verbatim)

### Policy (`bun -e` — piped `policy` CLI is silent)

```
POLICY_OK
```

### Policy CLI + rg

```
RG_EMPTY
```

### `cargo check -p semio-s-plugin-norm`

```
    Checking semio-s-plugin-norm v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust)
error: couldn't read `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/././././../../🗿️artifacts/📘️en1990/🧬️mutations/📤️set-document/🦠️mutation/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs:170:17
    |
170 |                 pub mod mutation;
    |                 ^^^^^^^^^^^^^^^^^

error: could not compile `semio-s-plugin-norm` (lib) due to 1 previous error
```

**Interpretation:** FAIL expected until glue integrator updates `set-snapshot` paths, mounts `schema` / `snapshot` / `diff::schema`, and moves `pack` under `snapshot`. Same class of error will appear for `🎒️pack` on other keys until glue is updated.

---

## Files touched (high level)

**Created:** per artifact × 15 schema leaves under `🧬️schema/`, `📸️snapshot/🧬️schema/`, `🔺️diff/🧬️schema/`; `📸️snapshot/🎒️pack/` (moved from root).

**Updated:** `🦀️component.rs` (root re-export snapshot), `📸️snapshot/🧬️schema/🦀️component.rs` (DSL/pack codecs), `🔺️diff/🦀️component.rs`, `🧬️mutations/🦀️component.rs`, `⚙️engine/🦀️component.rs`, `🧬️mutations/📄set-snapshot/**`, dependent `🔧️op` / `🗣️dsl` / `📡️spr` / examples / tests under each artifact; `🎛️apps/📘️en1990` … `📘️en1994` (snapshot type imports).

**Removed (moved):** `🗿️artifacts/📘️en199X/🎒️pack/` → `📸️snapshot/🎒️pack/`; `🧬️mutations/📤️set-document/` → `📄set-snapshot/`.

**Not edited (per brief):** `📦️packages/🦀️rust/📦️glue.rs`, `Cargo.toml`, `📦️packages/🟦️typescript/📦️index.ts`.

**Ticket helpers:** `🧪wave5-norm-en1990-1994-generate.py`, `🧪wave5-norm-en1990-1994-fixup.py`, `🧪wave5-norm-engine-fix.py`.

---

## Not validated

- Full `cargo check` / `cargo test -p semio-s-plugin-norm --lib` after glue integration (blocked on integrator).
- `bunx vitest run` for norm TS package (not requested; nx budget).
- End-to-end app command rename `setDocument` → `setSnapshot` in manifest wire ids (apps still expose `setDocument` action id; mutation enum is `SetSnapshot`).
