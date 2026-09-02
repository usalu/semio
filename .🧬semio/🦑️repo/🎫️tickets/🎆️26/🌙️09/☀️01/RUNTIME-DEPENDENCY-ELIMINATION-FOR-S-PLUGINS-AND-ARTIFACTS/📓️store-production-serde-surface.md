# 🎯️ The real remaining serde surface in 🏪️store

## Counting correction (third occurrence — see 📓️status.md)
Naive `grep -E 'serde|Serialize|Deserialize'` is unusable in this repo:
- `VcsError::Serialize(String)` / `::Deserialize(String)` are **error-enum variant names**, not serde.
- `operation_envelope_serde::to_value` is a **first-party** module whose name contains "serde".
- Real test usage lives in `#[cfg(test)] mod tests`, which a line-level `cfg_attr` filter misses.

Raw count 174 → genuine serde refs 105 → **production refs ≈ 8**.
Previously reported figures (75, "22→1") were both artifacts of bad counting.

## 🏪️store production blockers (the only real work)
1. **`ArtifactCursor` hand-written impls** — `🦀️.rs:2217-2231`, `impl serde::Serialize` /
   `impl<'de> serde::Deserialize<'de>`, delegating to the inner type.
2. **Three production derives** — `:2063` (+`#[serde(rename_all)]` `:2065`), `:3419`/`:3421`, `:17256`/`:17258`.
3. **The `serde_json::Value` pack API** — `:4817` `encode_json_value`, `:4822` `decode_json_value`,
   `:4864` `renormalize_json_wire_value`, `:4869` `json_value_to_dsl`, `:4874` `dsl_value_to_json`,
   `:4883` `json_values_equal`, `:9198` `impl ArtifactPack for serde_json::Value`,
   `:19820-19825` generic `ArtifactPack` bridge via `serde_json::to_value`.

Item 3 is the architectural one and the reason the manifest line cannot be cleared: it **exports API
typed on a third-party value**, which CLAUDE.md forbids outright ("MUST NOT export api that directly
or indirectly requires an interface/class/type outside of this codebase"). `DslValue` already exists
as the first-party equivalent; these signatures should be `DslValue`, with `serde_json` appearing
only in the oracle tests that compare against it.

## 🌿️vcs
Production surface is **line 6 alone** (`use serde::{Deserialize, Serialize};`), which exists only to
serve `mod tests`. Everything else is test-side or `#[cfg_attr(test, ...)]`. Move the import into the
test module and vcs is clean.

## Standing rule
Do not clear a `Cargo.toml` line that has not been compiled — the mistake that left 🏗️fem silently
uncompilable for two waves.
