# Compose-to-Puzzle5D Migration: Canonical Assets Census

## A. Compose Side

### 1. Fixture Kit Directory Tree (depth 4)

```
/Users/ueli/Documents/semio/compose/fixture/kit/
├── dev/
│   ├── abbau-aufbau/
│   │   └── wip/
│   │       └── initialKit/
│   └── metabolism/
│       └── wip/
│           └── initialKit/
```

**Kits present:**
- `abbau-aufbau` — under `dev/abbau-aufbau/wip/initialKit/`
- `metabolism` — under `dev/metabolism/wip/initialKit/`

### 2. Metabolism Kit Fixtures

#### metabolism.kit.light.compose.json
- **Size:** 5.27 MB (5,529,408 bytes)
- **Top-level keys:** `schema`, `wip`
- **Structure:** `wip.initialKit` contains metadata only
  - Typologies: 2
  - Families: 2
  - Concepts: 2
  - Qualities: 2
  - Types: 0 (metadata only)
  - Designs: 0 (metadata only)
- **Pieces/Connections:** None in metadata structure

#### metabolism.shallow.kit.compose.json
- **Size:** 7.25 MB (7,604,424 bytes)
- **Top-level keys:** `id`, `name`, `version`, `description`, `icon`, `image`, `preview`, `remote`, `homepage`, `license`, `createdAt`, `updatedAt`, `concepts`, `tags`, `types`, `designs`, `qualities`, `files`, `folders`, `authors`, `families`, `typologies`, `hash`
- **Counts:**
  - Typologies: 2
  - Families: 2
  - Concepts: 2
  - Qualities: 2
  - **Types: 50**
  - **Designs: 10**
  - Tags: 2
  - **Total pieces (across all designs): 7,200**
  - **Total connections (across all designs): 3,580**
  - **Total ports (across all designs): 0**
  - **Total representations (across all designs): 0**

#### metabolism.meta.kit.compose.json
- **Size:** 537 bytes
- **Top-level keys:** `id`, `version`, `homepage`, `icon`, `license`, `image`, `description`, `createdAt`, `remote`, `name`, `updatedAt`, `preview`
- **Content:** Kit metadata reference only (no fixture data)
- **Metadata reference:** `version: "r25.07-1"`, remote: `https://github.com/usalu/metabolism/archive/refs/tags/r25.07-1.zip`

### 3. Nakagin Design Fixtures

#### nakagin-capsule-tower.shallow.design.compose.json
- **Size:** 163,042 bytes (159 KB)
- **Top-level structure:** Design object with direct pieces/connections arrays
- **Counts:**
  - **Pieces: 180**
  - **Connections: 179**
  - Name: "Nakagin Capsule Tower"

#### nakagin-capsule-tower.with-diff.design.compose.json
- **Size:** 231,092 bytes (226 KB)
- **Note:** Contains diff-augmented design structure

### 4. The 180/179 Invariant

**Test location:** `/Users/ueli/Documents/semio/compose/server/hub/rs/bin.rs:4039-4049`

```rust
#[test]
pub fn nakagin_design_parses_180_pieces() {
    let design = load_nakagin_design_json();
    let pieces = design["pieces"].as_array().unwrap();
    assert_eq!(pieces.len(), 180, "nakagin design should have 180 pieces");
}

#[test]
pub fn nakagin_design_parses_179_connections() {
    let design = load_nakagin_design_json();
    let conns = design["connections"].as_array().unwrap();
    assert_eq!(conns.len(), 179, "nakagin design should have 179 connections");
}
```

**Fixture verification (via jq):**
```bash
jq '{pieces: (.pieces | length), connections: (.connections | length)}' \
  /Users/ueli/Documents/semio/compose/fixture/nakagin-capsule-tower.shallow.design.compose.json
# Output: {"pieces": 180, "connections": 179}
```

**Verdict:** ✓ Confirmed. The fixture file `nakagin-capsule-tower.shallow.design.compose.json` contains exactly 180 pieces and 179 connections.

### 5. Compose Client Example: Metabolism

**Location:** `/Users/ueli/Documents/semio/compose/client/example/metabolism/`

**Contents:** Badge directory with `.shields` files

**Badge counts:**
| Metric | Count | Status |
|--------|-------|--------|
| types | 45 | orange |
| designs | 4 | red |
| pieces | 1,260 | red |
| connections | 1,256 | red |
| ports | 120 | green |
| qualities (attributes) | 696 | red |
| representations | 225 | orange |
| lods | 2 | green |
| tags | 1 | red |

**Discrepancy note:** Badge counts (pieces=1,260, connections=1,256, types=45) differ from `metabolism.shallow.kit.compose.json` fixture counts (pieces=7,200, connections=3,580, types=50). This suggests badges track a subset (likely "light" or "default" design collection).

---

## B. Puzzle5D Side

### 6. Example Codec Inventory — CRITICAL

**Location:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`

#### Asset & Test File Enumeration

| Example | Asset File | Size | Present? |
|---------|-----------|------|----------|
| **🏗️nakagin-capsule-tower** | 🗣️tower.dsl.semio | 164.29 KB | ✓ DSL |
| | 🎒️tower.pack.semio | 270 B | ✓ PACK |
| | 📡️tower.spr.semio | 267 B | ✓ SPR |
| | 🔧️tower.op.semio | 285 B | ✓ OP |
| | .json (canonical) | — | ✗ MISSING |
| **🌙️capsule-dream** | 🗣️dream.dsl.semio | 2.89 MB | ✓ DSL |
| | 🎒️dream.pack.semio | 106 B | ✓ PACK |
| | 📡️dream.spr.semio | 105 B | ✓ SPR |
| | 🔧️dream.op.semio | 182 B | ✓ OP |
| | 🏅golden-poses.json | 840.46 KB | ✓ JSON (poses only) |
| **🌲️concrete-forest** | 🗣️forest.dsl.semio | 3.02 KB | ✓ DSL |
| | 🎒️forest.pack.semio | 286 B | ✓ PACK |
| | 📡️forest.spr.semio | 283 B | ✓ SPR |
| | 🔧️forest.op.semio | 279 B | ✓ OP |
| | .json (canonical) | — | ✗ MISSING |

#### Encoding Verdict

**Missing canonical JSON snapshots:**
- ✗ `🏗️nakagin-capsule-tower`: No `.json` file (only DSL + pack/spr/op)
- ⚠ `🌙️capsule-dream`: Only `🏅golden-poses.json` (pose subset, not full canonical snapshot)
- ✗ `🌲️concrete-forest`: No `.json` file (only DSL + pack/spr/op)

**Summary:** All three examples lack committed canonical JSON snapshots. Migration must generate these from DSL sources or designate golden-poses.json as sufficient.

### 7. Parts & Fasteners Counts (Puzzle5D)

#### From Test Assertions

**🌙️capsule-dream** (explicit test counts in `/🧪️tests/🦀️test.rs:8-9`):
```rust
assert_eq!(projection.parts.len(), 2880);
assert_eq!(projection.fasteners.len(), 2864);
```
- **Parts: 2,880**
- **Fasteners: 2,864**

**🏗️nakagin-capsule-tower** (no explicit test count; DSL parsed content):
- Estimated parts from DSL line count: ~250 data rows (methodology: lines starting with `  "` and UUID format)
- Fasteners count: UNKNOWN — no test assertion

**🌲️concrete-forest** (no explicit test count):
- Fasteners count: UNKNOWN — no test assertion

#### Comparison with Compose 180/179

**Nakagin mismatch:**
- Compose fixture: 180 pieces, 179 connections
- Puzzle5D estimated: ~250 parts (from DSL)
- **Status:** DOES NOT MATCH — puzzle5d DSL carries more detailed decomposition than compose design

**Note:** The puzzle5d DSL format is not pure JSON and requires custom parsing via `crate::artifacts::puzzle5d::dsl::parse_dsl()`. The 250-line count is approximate and may include section headers or metadata.

### 8. Test Fixture Convention (Puzzle5D)

**Test directory pattern:** Each example has a `🧪️tests/` directory with exactly two files:
- `🟦️test.ts` — TypeScript test (~300–900 bytes)
- `🦀️test.rs` — Rust test (~1,700–3,500 bytes)

#### Established Test Shape

**Rust test convention** (all three examples follow):

```rust
#[semio_framework_async_macros::async_test]
async fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️<name>.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text)
        .expect("example dsl parses");
    // Additional assertions (if present):
    // - assert_eq!(projection.parts.len(), <N>);
    // - assert_eq!(projection.fasteners.len(), <N>);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
}

#[semio_framework_async_macros::async_test]
async fn op_pack_and_spr_assets_are_nonempty() {
    assert!(include_str!("../🖼️assets/🔧️<name>.op.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/🎒️<name>.pack.semio").len() > 64);
    assert!(include_bytes!("../🖼️assets/📡️<name>.spr.semio").len() > 64);
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() { /* ... */ }

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() { /* ... */ }
```

**Capsule-dream extension:**
```rust
async fn flatten_matches_golden_poses_to_1e4() {
    let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
    let mut projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text)
        .expect("example dsl parses");
    crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::inferences::
        flat_position::flatten_snapshot_inplace(&mut projection);
    let golden: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(include_str!("../🖼️assets/🏅golden-poses.json"))
        .expect("golden json");
    assert_eq!(golden.len(), 2880);
    // Position mismatch validation (tolerance 1e-4)
}
```

**Recommendation for migration:** Adopt this convention as the canonical test shape; add `parts.len()` and `fasteners.len()` assertions for all examples once counts are finalized.

### 9. Component Source Fields (Rust)

#### 🏗️nakagin-capsule-tower
**File:** `/🦀️component.rs`

```rust
pub const ID: &str = "nakagin-capsule-tower";
pub async fn label() -> LocalizedLabel { /* "Nakagin Capsule Tower", "Nakagin-Kapselturm" */ }
pub const ICON: &str = "building";
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️tower.dsl.semio");
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️tower.op.semio");
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️tower.pack.semio");
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️tower.spr.semio");

pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {
    ExampleSource::new(ID, label(), document_json(), ICON)
});
```

#### 🌙️capsule-dream
**File:** `/🦀️component.rs`

```rust
pub const ID: &str = "capsule-dream";
pub async fn label() -> LocalizedLabel { /* "Capsule Dream", "Kapseltraum" */ }
pub const ICON: &str = "building";
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️dream.dsl.semio");
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️dream.op.semio");
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️dream.pack.semio");
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️dream.spr.semio");
pub const GOLDEN_POSES_JSON: &str = include_str!("🖼️assets/🏅golden-poses.json");

pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {
    ExampleSource::new(ID, label(), document_json(), ICON)
});
```

#### 🌲️concrete-forest
**File:** `/🦀️component.rs`

```rust
pub const ID: &str = "concrete-forest";
pub async fn label() -> LocalizedLabel { /* "Concrete Forest", "Betonwald" */ }
pub const ICON: &str = "list-tree";
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️forest.dsl.semio");
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️forest.op.semio");
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️forest.pack.semio");
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️forest.spr.semio");

pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {
    ExampleSource::new(ID, label(), document_json(), ICON)
});
```

**ExampleSource Fields:**
- `ID: &str` — stable identifier
- `label(): LocalizedLabel` — native + German text
- `document_json(): String` — async fn that parses DSL and serializes to JSON
- `ICON: &str` — icon name (e.g., "building", "list-tree")
- *Optional:* `GOLDEN_POSES_JSON` (capsule-dream only)

---

## Summary & Migration Implications

| Item | Finding |
|------|---------|
| **Item 4: 180/179 Test** | ✓ VERIFIED in `/Users/ueli/Documents/semio/compose/server/hub/rs/bin.rs:4039-4049`; fixture contains exactly 180 pieces, 179 connections |
| **Item 7: Encoding Inventory** | **CRITICAL:** All three examples lack committed `.json` canonical snapshots. Nakagin and concrete-forest have zero JSON assets. Capsule-dream only has `golden-poses.json` (pose subset). Migration must generate canonical JSONs from DSL or designate goldens as sufficient. |
| **Item 8: Part/Fastener Counts** | Capsule-dream: 2,880 parts / 2,864 fasteners (test-asserted). Nakagin: Compose has 180 pieces (verified); puzzle5d DSL has ~250 parts (unverified estimate). Mismatch suggests puzzle5d carries finer decomposition. Concrete-forest: counts UNKNOWN. |
| **Item 9: Test Convention** | Established pattern: DSL parse → round-trip validation; pack/spr/op nonempty checks; inference determinism. Capsule-dream adds golden-poses flattening test. Recommend standardizing parts/fasteners assertions for all examples. |

