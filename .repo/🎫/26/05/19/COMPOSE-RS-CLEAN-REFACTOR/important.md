# Compose Rs Clean Refactor — Gap Surfaces

## Stable macro pattern (do not revert)

Rust `macro_rules!` cannot expand `other_macro!(@names)` inside another macro matcher. Use **inlined name lists** in three arms of each `*_name_list` macro:

- `@names` — token list only
- `@apply_families` / `@apply_relays` — expands `gap_surface_*! { … }`
- `@register` — expands `register_*! { @expand $builder; … }`

Module init: `gap_surface_family_name_list!(@apply_families);`

## Repair scripts

| File | Purpose |
|------|---------|
| `gap_surface_names.json` | Canonical 196 family + 50 relay idents |
| `rebuild_schema_gap_surfaces.py` | Replaces `schema_gap_surfaces` tail from JSON |
| `fix_register_braces.py` | `{{ }}` on `@expand`; strips bad patterns |

Run both after any concurrent edit to `//#region schema_gap_surfaces`.

## Verified (2026-05-20)

- `cargo test --lib`: **43 passed**, 1 ignored (`test-log.txt`)
- `cargo clippy --lib --tests -D warnings`: **exit 0** (`clippy-log.txt`)

## Low disk on C: (Windows)

```powershell
$env:CARGO_TARGET_DIR='E:\cargo-target-compose'
$env:TEMP='E:\cargo-tmp-compose'
$env:TMP='E:\cargo-tmp-compose'
```

Golden fingerprint: `assets/compose/kit-store.golden.expected.compose.json` → `1a9a217ca1ac63d89883abe5bc13e36268becf6cb4ef743105af0a11fcf3653a`.
