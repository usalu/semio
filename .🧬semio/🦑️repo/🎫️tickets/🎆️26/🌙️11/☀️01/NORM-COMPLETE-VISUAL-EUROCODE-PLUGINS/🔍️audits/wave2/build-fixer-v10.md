# Build-fixer v10 (en1993 run 13, 73 errors in the model crate)
Source: full cargo log `cargo-artifacts-hWtwG9.log`. All errors were in the model crate (twin `📸️snapshot/🦀️.rs` and the handwritten `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`); downstream crates were never compiled. No freeze present, no twin touched by hand beyond the Rust enum text below, `schema twins check --strict` via heavy.sh: drift 0 (rust 2433 identical).

## Root causes and fixes
- `FireRating` / `WeldPattern` unresolved (E0425/E0433/E0432): typed-enums v4 `convert.py` skipped inserting the model enum because a same-named enum exists in the checks crates (`part_1_2::FireRating`, `weld_group::WeldPattern`). Added both `pub enum` definitions (same derives as the sibling enums; `R30..R120`, `Line|TwoLines|Box`) to the model snapshot Rust before `FireExposure` / `WeldGroup`; their `impl`/`Display` already existed. The kernel enums in checks stay (pure kernels, mapped by `pattern()` / `parse_rating(rating.as_str())`).
- Snapshot examples (`String::new()` for converted enum fields, 10 E0308): `stainless_forming`, `assumed_class`, `patch_type`, `fabrication_class`, `lattice_kind`, `environment` now `<Enum>::Unset`.
- SQLite io: `FRAME_MEMBER_BASE` const added (no float columns, like `FRAME_BAR`); 34 x `?` incompatible (`read_x(text(..)?)` got `String`, expects `&str`): all `read_*(` calls take `&text(..)`; 17 x E0308 in `preflight_sqlite_snapshot_encoding`: `admit_strings` takes `&[&str]` and every enum field is passed as `<enum>_text(v.field)` (15 fields); admit loops added for `frame_actions` and `frame_member_bases`; E0063: `Ok(Self{..})` now includes `frame_actions, frame_member_bases`.
- Verified all `<enum>_text` tokens equal the JSON schema enum values (python check).
- Table count wiring (not a compile error): admission `SQLITE_SCHEMA,37 -> 39` in `🛂️admission/🦀️.rs` and `🦀️.rs`, io test `tables.len() 37 -> 39` and `("tables",37) -> 39` (DDL has 39 tables, en1994 pattern admission = table count).

## Other en1993 crates (grep of String-era uses of the 28 enum-typed fields)
- scene `diagrams`: `member.load_application.clone()` / `moment_diagram.clone()` passed `&String`-era to `m_cr_rolled_nm(&str, &str)` -> `.as_str()`.
- scene-members `layout::is_upright` and `members` pinned test: `to_ascii_lowercase().as_str()` matches -> `SteelMemberType` variants.
- checks `special-wiring` test: `n.support = "free".into()` -> `FrameNodeSupport::Free`; mutations unit test `stainless_forming: String::new()` -> `StainlessForming::Unset`.
- `En1993Snapshot` literals elsewhere use `..` or `to_snapshot` (complete). Enum-import scan over all crates: clean.

## Unverified
Nothing compiled; the first rebuild is the test. Possible follow-ups: borrowck in `&text(..)?` temporaries (should be fine), downstream crates (checks, scene, editor) are compiled for the first time.
