# Terra Independent P4e Acceptance Audit — 2026-08-24

## Verdict

**RED — P4e is not acceptable yet.** The mounted-path, spatial, checkpoint/token, and renderer work is substantially present, and the retained P4d R7/R8/R9–R11 predicate remains intact. Two P4e acceptance contracts are nevertheless unproven and contradicted by the current source: the constructor does not cap/refuse every fixture/catalog root, and the existing cap refusal returns a terminal fault before publishing the required bounded rejection diagnostic.

## Required Evidence Read

- Root `AGENTS.md`.
- `📓️terra-p4e-constructor-spatial-checkpoint-preview-packet-2026-08-24.md`.
- `📓️terra-p4d-r9-r11-independent-acceptance-audit-2026-08-24.md`.
- `📓️sol-p4e-constructor-spatial-checkpoint-preview-implementation-2026-08-24.md`.
- The seven P4e production/verifier files, their current diff, fixtures, and callers.

## Accepted Portions

1. Initial rebuild, soft replan, and refresh all enter `start_fill_preparation`; mounted production source has no whole `FillBuilder::new`, `configure`, `refresh_meshes`, or `rebuild_collision_index` call. The fill stages include fixture, catalogs, meshes, entries, spatial, lookup, and configuration.
2. Spatial storage is fixed (`FixedOwnerMap<Cell, FixedOwnerSet<String>>`), cell coverage is `CollisionCellSpan`, and production broad phase drives `begin_query`/`step_query` rather than scanning `self.placed`. Replacement/removal/query cursors carry `CollisionIndexOwner`; collision narrowing has part/sampling cursors.
3. The dormant `FillJobCheckpoint`/JSON restore and BTree/rebuild-clear escapes are absent from the P4e fill/geometry sources. The live session checkpoint is the P4d registry token (`fill_checkpoint_bytes` / `restore_persisted_fill`). `FillBuilder::progress` publishes empty producer collections and clones only the bounded diagnostic.
4. `world_fill_preview_json` installs `fillBuildPreview` independently of the optional ghost. `World3dHost` validates the identity and all diagnostic fields, renders the overlay without a ghost, and only renders a ghost owned by a valid diagnostic.
5. The P4d envelope predicate still requires registry-exclusive post-admission ownership, reclaimable `Closing`, checked semantic identity allocation, and its R7/R8/R9–R11 self-tests. P4e did not widen those rules.

## P4e Blocking Findings

### P4e-B1 — Constructor cap/+1 contract excludes catalog and fixture branches

`FillBuilder::begin_preparation` preflights only fixture **objects**, meshes, and two weight maps (`fill/🦀️component.rs:2093-2096`). It excludes fixture attractions and target volumes, all three catalog collections, and `kind_compatibility`.

Those omitted inputs are copied, one item per turn, into ordinary unbounded `Vec`s: fixture attractions/objects/target volumes at `:2240-2266`, catalog objects/vortices/cables at `:2269-2287`, and kind compatibility at `:2381-2404`. A catalog at `FIXED_OWNER_SLOTS + 1` therefore reaches `PrepareCatalogs` and is accepted instead of producing the packet-required permanent attributable refusal. The only named constructor cap fixture constructs only `Fixture.objects` (`:3259-3286`); it does not cover catalog or mesh roots as required by the P4e packet.

This violates the explicit fixture/mesh/catalog cap and cap-plus-one acceptance predicate. Cursorization alone does not satisfy that predicate.

### P4e-B2 — Capacity refusal cannot publish the required rejection diagnostic

The same construction-time preflight sets `collection_over_capacity` before the first job grant (`:2093-2196`). `InteractiveJob::step` then returns `Fault("fill-fixed-collection-capacity")` at `:3117-3119`, before `prepare_one`, `publish_preview`, or a rejection reason is set. The initialized diagnostic remains `rejection_reason: None` (`:2117-2140`).

`world_fill_preview_json` can transport only an existing active `fill_progress().preview` and rejects the terminal state (`main/🦀️component.rs:404-419`). After the fault/terminal route it has no admitted diagnostic to render. Thus the required cap-plus-one bounded rejection/no-ghost diagnostic is absent, despite the renderer being able to display one when supplied.

This violates the required diagnostic transport contract for rejection/no-candidate states and the P4e preview cap/+1 predicate.

## Verifier And Fixtures

`bun 📜️script.ts verify interactivity --self-test` executed both Puzzle P4d and P4e static baselines and all 21 P4e string mutations before reaching its unrelated deny. The P4e baseline/mutations themselves did not throw. That is useful regression coverage, but not acceptance proof for B1/B2: its constructor fixture predicate checks only the fixture name, and the test uses only `Fixture.objects`; it contains no catalog/mesh cap-plus-one or refusal-diagnostic assertion.

## Scoped Gates

- `rustfmt --edition 2021 --check` over the five P4e Rust files: **PASS** (silent).
- Scoped `git diff --check` over the seven P4e source/verifier files: **PASS** (silent).
- `bun 📜️script.ts verify interactivity --self-test`: Puzzle P4d/P4e self-tests reached successfully; command ends **DENY** only on pre-existing P1q DB source predicates:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`: missing fixed-page/exact rejected-owner input ownership.
  - the same DB file: missing I/O admission caps.

No Cargo, Nx, Wasm, browser, runtime, or network command was run. P5b was not started.
