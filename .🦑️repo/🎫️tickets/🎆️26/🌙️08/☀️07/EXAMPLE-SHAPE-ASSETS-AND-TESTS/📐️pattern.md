# Example Shape Migration Pattern (W2 Puzzle Pilot)

Canonical template for W3 shards. Follow exactly — handcraft content, no adapters.

## Target layout

```
✏️s/🔌️plugins/<plugin>/
  🗿️artifacts/<artifact>/
    🔺️diff 🗣️dsl 🎒️pack 🔧️op 📡️spr ⚙️engine   # facets unchanged
    📚️examples/
      <emoji>+VS16+<kebab-slug>/
        🦀️component.rs
        🟦️component.ts
        🖼️assets/
          🗣️<name>.dsl.semio
          🔧️<name>.op.semio
          🎒️<name>.pack.semio
          📡️<name>.spr.semio
          # optional: 🔺️<name>.diff.semio, media with 🖼️/🧊️/📄️/🎬️
        🧪️tests/
          🦀️test.rs
          🟦️test.ts
  🎛️apps/<app>/
    📚️examples/                 # NOT under ⚙️engine
      <emoji>+VS16+<kebab-slug>/
        🦀️component.rs
        🟦️component.ts
        🖼️assets/🎮️<name>.cmd.semio
        🧪️tests/{🦀️test.rs,🟦️test.ts}
```

Replace `🧪️tests` / `️test.*` with taxonomy `exampleTestsDirName` / `exampleTestLeafFilenames`.

Slug law: `^.+\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$` (emoji + VS16 + kebab).

## Forbidden (delete, do not rename)

- Slugs: `♻️reuse`, `♻️default`, `📕️default`, `♻️semio`
- Plural dirs: `🗣️dsls`, `🎒️packs`, `🔧️ops`, `📡️sprs`
- Dead nested `🦀️component.rs` under plural trees
- Plugin-root `📚️examples/` (except special cases owned by W3 S6)

## Definition leaf — Rust (`🦀️component.rs`)

```rust
//! 📚️ Example `🏗️nakagin-capsule-tower` for artifact `puzzle3d`.

use std::sync::LazyLock;

use semio_framework::LocalizedLabel;
use semio_framework_os_kernel::plugin::ExampleSource;

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "nakagin-capsule-tower";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm")
}

/// 🖼️ Icon id.
pub const ICON: &str = "building";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("🖼️assets/🗣️tower.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("🖼️assets/🔧️tower.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️tower.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️tower.spr.semio");

fn document_json() -> String {
    let projection = crate::artifacts::puzzle3d::dsl::parse_dsl(DSL_TEXT)
        .unwrap_or_else(|error| panic!("{ID} example dsl parses: {error}"));
    serde_json::to_string(&projection).expect("serialize example")
}

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {
    ExampleSource::new(ID, label(), document_json(), ICON)
});
```

Notes:
- `SOURCE.document_json` must be the payload the app loads (usually JSON from parsed DSL). For puzzle2d, strip `camera` before serialize.
- App cmd examples use `CMD_TEXT` / `🎮️*.cmd.semio` as the payload instead of DSL JSON.
- Every asset under `🖼️assets/` must be **> 64 bytes** (policy).

## Definition leaf — TypeScript (`🟦️component.ts`)

```ts
/** 📚️ Example `🏗️nakagin-capsule-tower`. */
export const id = "nakagin-capsule-tower";
export const label = { en: "Nakagin Capsule Tower", de: "Nakagin-Kapselturm" } as const;
export const icon = "building";
export const dslPath = new URL("./🖼️assets/🗣️tower.dsl.semio", import.meta.url);
export const opPath = new URL("./🖼️assets/🔧️tower.op.semio", import.meta.url);
export const packPath = new URL("./🖼️assets/🎒️tower.pack.semio", import.meta.url);
export const sprPath = new URL("./🖼️assets/📡️tower.spr.semio", import.meta.url);
```

## Tests

`🧪️tests/🦀️test.rs` — parse + round-trip DSL via `include_str!`:

```rust
//! 🧪️tests for example `🏗️nakagin-capsule-tower`.

#[test]
fn dsl_asset_parses_and_round_trips() {
    let text = include_str!("../🖼️assets/🗣️tower.dsl.semio");
    assert!(text.len() > 64);
    let projection = crate::artifacts::puzzle3d::dsl::parse_dsl(text).expect("parses");
    store::test_support::assert_dsl_round_trip(&projection);
    store::test_support::assert_dsl_pack_equivalence(&projection);
}
```

`🧪️tests/🟦️test.ts` — read via `node:fs` (never `?raw`):

```ts
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example 🏗️nakagin-capsule-tower", () => {
  it("ships a non-empty dsl asset", () => {
    const text = readFileSync(join(here, "../🖼️assets/🗣️tower.dsl.semio"), "utf8");
    expect(text.length).toBeGreaterThan(64);
  });
});
```

## Glue wiring (`📦️packages/🦀️rust/📦️glue.rs`)

Add `//#region 📚️Examples` with `#[path]` leaves + `#[cfg(test)]` test mods:

```rust
//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/<art>/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/<art>/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
        // …concrete_forest, app demo-session, …
    }
}
//#endregion 📚️Examples
```

## App registration

```rust
.example_source(&*crate::examples::puzzle3d::concrete_forest::SOURCE)
.example_source(&*crate::examples::puzzle3d::nakagin_capsule_tower::SOURCE)
```

Prefer `SOURCE` over restating id/label/payload/icon. Update `ActionArgOption` lists to use `ID`/`label()` from the same leaves.

## Facet includes

Point facet `🗣️dsl/🦀️component.rs` `include_str!` consts at the new asset paths, e.g.:

```rust
pub const PUZZLE3D_NAKAGIN_EXAMPLE_TEXT: &str =
    include_str!("../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio");
```

## TypeScript package

1. Re-export leaves from `📦️index.ts`.
2. Add `️vitest.config.ts` with `include: ["../../…/🧪️tests/🟦️test.ts", …]`.
3. Wire `📜️script.ts` test via `runVitest(..., "️vitest.config.ts")`.

## Migration steps (checklist)

1. Inventory existing example roots under the plugin.
2. Delete placeholder slugs (`♻️reuse`, …); keep real content under emoji-slugs.
3. Flatten plural dirs → `🖼️assets/<kindEmoji><free-name>.<kind>.semio`.
4. Delete dead nested `🦀️component.rs` shims.
5. Handcraft missing op/pack/spr/cmd payloads (>64 bytes). Reuse non-empty `.semio`; expand stubs.
6. Write `🦀️component.rs` / `🟦️component.ts` with `SOURCE`.
7. Write `🧪️tests` (Rust parse/round-trip + TS fs reads).
8. Move app examples out of `⚙️engine` to `🎛️apps/<app>/📚️examples/`.
9. Wire glue `//#region 📚️Examples` + cfg(test) paths.
10. Update facet `include_str!`, `create_*_app().example_source(&*…::SOURCE)`, index exports, vitest.
11. Verify: `bun nx run <plugin>-plugin:test-quick`, `bun nx run <plugin>-js:test`, `bun ./📜️script.ts examples verify <plugin>`.

## Puzzle pilot appendix

| Owner | Examples |
| --- | --- |
| artifacts ◻2d / 🧊️3d / 🖐️5d | `🌲️concrete-forest`, `🏗️nakagin-capsule-tower` |
| apps ◻2d / 🧊️3d / 🖐️5d | `🎬️demo-session` (`🎮️demo.cmd.semio`) |

Asset stems: CF uses `forest.*`, Nakagin uses `tower.*`, apps use `demo.cmd.semio`.
