# Handcrafted Grammar — progress finish

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`

## Wave W4a (graph family) — done

- **Scope:** dag, reasoning (`wires`), sequence, mathematical, trinity (jack + rewrite facets).
- **Landed:** Artifact-specific `📖️component.grammar.semio` bodies; pack/spr protocol alignment; `🟦️component.ts` parse/print stubs (throw until WASM).
- **Apply:** `w4a-apply-graph-grammars.mjs` (re-runnable).
- **Verification:** `cargo test` / recognizer sweep **not run** on agent host.
- **Volume:** ~60 facet files in apply pass.

## Wave W4e (HOT LAST) — done

- **Scope:** flow, procedural (2d/3d), puzzle (2d/3d/5d), block (2d/3d/5d), cad, vcs.
- **Landed:** 33 grammar facets — replaced bulk stubs with artifact-shaped grammars (graph/geo/scene/catalog/document tables as appropriate); EDGEARROW retained on graph-family wire literals.
- **Apply:** `handcraft-w4e-hot.mjs`.
- **Verification:** Manual fixture cross-check; **cargo test** not run on host.
- **Gap:** CAD `🎬️interaction-spec` exists in Rust but no interaction grammar facet in taxonomy — no spec added.

## Writer P2/M6 (`lang_from` / EmbedFrom)

- **WriterProjection:** `#[dsl(lang_from = "language_id")]` on `text` in `✒️writer/🗿️artifacts/✒️writer/🦀️component.rs`.
- **Derive + schema:** `lang_from` → `Shape::EmbedFrom` in dsl derive; `Shape::EmbedFrom` and parse/print branches in dsl schema.
- **Cargo verify:** **BLOCKED** — root `Cargo.toml` parse error at line 152 (workspace members); see `🧪writer-p2-m6-cargo-check.txt`.

## TypeScript plugin facades (this session)

- **Scaffold run:** 32 plugins with TS packages; **0** new `📜️script.ts` / **0** new `📋️project.json`; **32** `📦️index.ts` rewrites (first run); **30** script import paths corrected to repo-lib `relative()` (writer `bun ./📜️script.ts test` OK).
- **Policy:** Root `policyComponentFileBreaches` extended so artifact facet dirs may carry `artifactSpecFilenames` specs and TypeScript `leafFilename` alongside rust leaf (registry `validateTaxonomyTree` already required them; no separate tree-purity file ban).

## Remaining gaps

1. Fix workspace `Cargo.toml` and run writer + dsl-schema `cargo check` / facet round-trip tests.
2. Run grammar recognizer sweep and plugin `cargo test` for W4a/W4e artifacts.
3. Wire WASM in facet `🟦️component.ts` (still throw stubs) and shrink handcrafted-grammar allowlists in root policy.
4. Other waves/plugins outside W4a/W4e graph/HOT slices — coverage matrix may still list open artifacts.
5. Trinity/jack multi-artifact naming: index exports use per-artifact ascii ids (e.g. `jack_dsl`, `rewrite_dsl`), not plugin slug alone.
