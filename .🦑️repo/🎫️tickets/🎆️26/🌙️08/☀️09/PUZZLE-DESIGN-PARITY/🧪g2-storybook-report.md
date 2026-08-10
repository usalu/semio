# G2 Storybook Report — Wave 6

**Ticket:** `PUZZLE-DESIGN-PARITY`  
**Scope:** `.storybook/stories/puzzle/{2d,3d,5d}/` only (G2)

## Summary

Extended puzzle fixture/timeline stories with **Capsule Dream**, pointed imports at **shipped artifact DSL** paths (replacing stale `s/plugin/puzzle/app/*/example/*.puzzle*` raw imports where touched), and surfaced **connection parameters** (`gap`, `shift`, `rise`, `rotation`, `turn`, `tilt`, `x`, `y`) plus **`anchor`** (and 5d diagram `x`/`y` on parts) in each story’s debug `<pre>` panel.

## Stories added

| Dimension | File | Story | Fixture source |
|-----------|------|-------|----------------|
| 2d | `2d/Fixtures.stories.tsx` | `CapsuleDream` | Ticket `🌙️capsule-dream-out/🗣️dream.2d.dsl.semio` (pending `◻2d/📚️examples/🌙️capsule-dream`) |
| 3d | `3d/World.stories.tsx` | `CapsuleDream` | Ticket `🌙️capsule-dream-out/🗣️dream.3d.dsl.semio` (pending `🧊️3d/📚️examples/🌙️capsule-dream`) |
| 5d | `5d/Timeline.stories.tsx` | `CapsuleDream` | Artifact `🖐️5d/📚️examples/🌙️capsule-dream/🖼️assets/🗣️dream.dsl.semio` |

Existing **Nakagin Capsule Tower** and **Concrete Forest** stories unchanged in behavior; imports for nakagin/concrete now use `🗿️artifacts/*/📚️examples/*/🖼️assets/🗣️*.dsl.semio` on 2d/3d/5d fixture stories.

## Connection param surfacing (debug JSON)

Each fixture host story now appends `connectionParams`:

- **2d `Fixtures`:** `edges.{total, withNonzeroParams, sample}` with eight numeric fields; `nodeAnchors` histogram (`fixed` / `derived` / …).
- **3d `World`:** `attractions.{total, withNonzeroParams, sample}` (same eight fields).
- **5d `Timeline`:** `fasteners.{total, withNonzeroParams, sample}`; `partAnchors`; `partDiagramSample` (`id`, `x`, `y` from `part["2d"]`).

`2d/Board.stories.tsx` left unchanged — hand-authored micro-fixtures for interaction utilities, not design-parity DSL demos.

## Follow-ups (out of G2 scope)

- Promote ticket `dream.2d` / `dream.3d` DSL into `◻2d` / `🧊️3d` example units and switch story imports from ticket folder to artifact paths (mirror 5d).
- Capsule Dream stories are **large** (2880 nodes/objects/parts); expect slower Storybook load than Nakagin (180).
- Runtime Storybook build not executed in this subtask (wasm package path not available from bare `bun -e`); verify via launch Storybook + open new stories.

## Files changed

- `.storybook/stories/puzzle/2d/Fixtures.stories.tsx`
- `.storybook/stories/puzzle/3d/World.stories.tsx`
- `.storybook/stories/puzzle/5d/Timeline.stories.tsx`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/PUZZLE-DESIGN-PARITY/🧪g2-storybook-report.md` (this file)
