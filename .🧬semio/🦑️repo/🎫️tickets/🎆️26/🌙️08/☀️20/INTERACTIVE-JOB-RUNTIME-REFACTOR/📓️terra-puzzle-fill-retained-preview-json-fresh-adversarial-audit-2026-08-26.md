# Puzzle Fill Retained Preview JSON Fresh Adversarial Audit

## Verdict

**RED — the retained transport is substantively implemented, but the shared renderer parser does not fail closed against the declared fixed shape, and both app label resolvers retain a default/fallback path.** No production files were edited during this audit.

## Exact Blockers

### RED-1 — Renderer accepts a non-eight-item candidate page

The language-neutral schema requires `candidatePage` to have both `minItems: 8` and `maxItems: 8`. The renderer only rejects pages longer than eight, so malformed pages with zero through seven entries are accepted and rendered.

- Schema: `precompute/🪣️fill/🧪️fixtures/🔭️preview-json.schema.json`, `candidatePage` has `minItems: 8` and `maxItems: 8`.
- Production parser: `World3dHost/🟦️component.tsx:1155-1157` uses `diagnostic.candidatePage.length > 8`; it must instead require `length !== 8`.
- The renderer law test covers labels and malformed labels, but not a short `candidatePage`; therefore the defect is not currently held by a law.

This is a direct failure of the required fixed-cap, fail-closed renderer contract and affects both Puzzle3d and Puzzle5d because both feed this same `World3dHost` parser.

### RED-2 — Renderer accepts malformed `candidateGhost` values

The same parser accepts an array or an arbitrary object as `candidateGhost`, because JavaScript arrays satisfy `typeof value === "object"`. It validates only equality with selected outer fields, not that a ghost is a non-array record with all required typed fields (`targetVortexFullId`, `objectKindId`, `sourceVortexIndex`, `meshUrl`, finite `origin`, finite `orientation`).

- Schema: `preview-json.schema.json` defines `candidateGhost` as either the strict `ghost` object or `null`.
- Production parser: `World3dHost/🟦️component.tsx:1159-1166` has no array exclusion or complete record validation.
- The parsed value then controls the fill ghost route at `World3dHost/🟦️component.tsx:3869-3870`.

This is an independent fail-closed defect. A retained page must be rejected unless it matches the full renderer contract, rather than only the subset consumed by the diagnostic text.

### RED-3 — The localized status-label path still defaults/falls back

The required label itself is localized in all EN/DE × native/reuse cells, but both active resolvers silently map an unrecognized locale to English and unrecognized terminology to native. This violates the requested no-default/fallback requirement.

- Puzzle3d: `terminology/🦀️component.rs:112-113` maps every non-`de*` locale to `Locale::En` and uses `unwrap_or(Terminology::Native)`.
- Puzzle5d: `terminology/🦀️component.rs:80-81` maps every non-`de*` locale to `Locale::En` and every non-`reuse` terminology to native.
- Both visible 3D consumers pass these resolved labels into the retained cursor: Puzzle3d `windows/🧊️main/🦀️component.rs:403-410`; Puzzle5d `windows/🧊️3d/🦀️component.rs:197-203`.

The hardcoded renderer fallback is absent, but that does not remove the resolver fallback before the value reaches the renderer.

## Confirmed GREEN Evidence

| Contract | Static evidence |
| --- | --- |
| Actual Puzzle3d visible 3D consumer uses retained page | `world_fill_preview_json` reads `fill_preview_json_page` and `render` passes it as the world scene brush-preview slot (`windows/🧊️main/🦀️component.rs:403-410`, `:462-472`). |
| Actual Puzzle5d visible 3D consumer uses the same retained page | Its precompute adapter directly delegates to Puzzle3d (`🧠️precompute/🦀️component.rs:60-61`), and its real 3D render path supplies that value to `world3d_scene_extended` (`windows/🧊️3d/🦀️component.rs:197-203`, `:208-220`). |
| No fill whole-preview serde route remains | Scoped negative source census found none of `serde_json::to_vec(&self.preview)`, `serde_json::to_value(build)`, or `fill_progress().preview`. The remaining P3 `serde_json::to_value(&preview)` is the distinct brush-preview path at `windows/🧊️main/🦀️component.rs:396`. |
| Retained fixed-cap cursor | `FillPreviewJsonCursor` has explicit retire/census/reserve/encode/validate/ready/rejected/close/terminal phases; caps are 4096/128/256 bytes (`precompute/🪣️fill/🦀️component.rs:28-30`, `:412-662`). |
| Byte cap and byte-based labels | Rust validates `str::len()` against the byte caps before encoding (`fill/🦀️component.rs:462-489`); the renderer counts UTF-8 bytes by code point (`World3dHost/🟦️component.tsx:1117-1124`). Source laws include output-cap/+1 and renderer ASCII/multibyte status-label cap/+1. |
| One-unit cursor and bounded page advancement | Each cursor `step` takes one fuel unit before one phase/unit transition (`fill/🦀️component.rs:501-621`); the session grants at most 256 one-fuel units until a two-millisecond deadline (`precompute/🦀️component.rs:1437-1453`). |
| Freshness, cancellation, checkpoint/progress | Identity includes operation, base revision, registry generation, generation, and sequence; label/color changes invalidate the cursor (`fill/🦀️component.rs:33-50`, `:501-529`). Checkpoints/progress are returned in `Pending`; cancellation preserves the last ready page; source laws cover stale, cancellation and locale replacement (`:453-458`, `:4349-4429`). |
| Incremental close and exact terminal handback/retry | The fill envelope has a checked-out terminal handle, contention-preserving `resume`, one-grant `close_step`, stale fencing, and source laws for exact handback, retry and interrupted close (`precompute/🦀️component.rs:569-685`, tests at `:2490-2763`). |
| Worker preview and commit stay empty | `FillBuilder::publish_preview` and `complete` return empty retained payloads (`fill/🦀️component.rs:4004-4022`). |
| Empty fill preview/commit unchanged | The page returns `None` for `complete`, while worker preview and both commit streams remain empty (`precompute/🦀️component.rs:1441-1444`; `fill/🦀️component.rs:4004-4022`). |
| Visible/ARIA label parity and no renderer English fallback | The overlay uses `diagnostic.statusLabel` in both visible text and the `role=status` ARIA label (`World3dHost/🟦️component.tsx:2855-2895`). |
| Language-neutral fixture and serde oracle exist | The Rust law test loads the JSON schema/law fixture and requires EN and DE retained output to equal the independent serde oracle byte-for-byte (`fill/🦀️component.rs:4298-4326`). |

## Safe Checks Executed

All commands below were read-only or formatting checks; no Cargo, Nx, Bun/Vitest, Wasm, browser, cache, or production mutation was run.

- `rustfmt --edition 2021 --check` on the seven touched Puzzle3d/Puzzle5d Rust sources: exit 0.
- `jq empty` on `🔭️preview-json.schema.json` and `🔭️preview-json-law.json`: exit 0.
- Scoped `git diff --check` across the reported Rust, renderer, test, and JSON fixture files: exit 0.
- Scoped negative `rg` census for the three removed fill aggregate serialization paths: no matches.

Compilation and runtime behavior are deliberately not claimed.
