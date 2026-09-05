// #region 🧲️Header
// 💻️ .storybook/stories/block/2d/Fixtures.stories.tsx
// Specs: Prove every shipped block2d example document round-trips through `🖥️Board2dHost` unchanged — the
// block counterpart of `../puzzle/2d/Fixtures.stories.tsx`.
// Summary: Same split of duties as puzzle's pair: `./Board.stories.tsx` owns the interaction mechanics (the
// story-local `Block2dCommand` emulator, one example at a time), this file owns FIXTURE COVERAGE — it mounts
// every `📚️examples/*` unit read-only, side by side, so a document that stops parsing or stops projecting into
// a board scene shows up here rather than in a single hand-picked story. Puzzle can reuse its plugin's own
// `parse_dsl` through the `puzzle2dParseDslJson` wasm export; block ships no such export (its crate is a WASM
// component, not a `wasm-bindgen` module), so `../dsl.ts` reads the same text grammar in TypeScript.
// The per-example panel is the Rust viewer window's own `render` output (`block2dBoardRenderLines`), and the
// debug readout carries the handle-kind → handle assignment that is the only difference between the shipped
// `left` and `right` examples (`h4`/`h5`/`h6`).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useMemo, type ReactElement } from "react";

import { Board2dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { BLOCK2D_STORY_EXAMPLE_IDS, block2dBoardRenderLines, block2dStoryStateFor, buildBlock2dSceneNode } from "../scene";

//#region StoryHost
function Block2dFixturePane({ exampleId }: { readonly exampleId: string }): ReactElement {
  const state = useMemo(() => block2dStoryStateFor(exampleId), [exampleId]);
  const node = useMemo(() => buildBlock2dSceneNode(state.snapshot, false, `block2d.view.board.${exampleId}`, `block2d-fixture-${exampleId}`), [state, exampleId]);
  const lines = useMemo(() => block2dBoardRenderLines(state.snapshot), [state]);

  return (
    <section style={{ display: "flex", flex: "1 1 0", flexDirection: "column", minWidth: 0, borderLeft: "1px solid var(--border, #3333)" }}>
      <h3 style={{ margin: 0, padding: 4, fontSize: 12 }}>{exampleId}</h3>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <Board2dHost node={node} onAction={() => undefined} />
      </div>
      <ol data-testid={`block2d-fixture-window-${exampleId}`} style={{ flex: "0 0 40%", margin: 0, padding: "4px 8px", overflow: "auto", listStyle: "none", fontFamily: "var(--font-mono, monospace)", fontSize: 11 }}>
        {lines.map((line) => {
          const kind = state.snapshot.handleKinds.find((candidate) => line.startsWith(`  ◦ ${candidate.label} (${candidate.id})`));
          return (
            <li key={line} style={{ display: "flex", alignItems: "center", gap: 6, whiteSpace: "pre" }}>
              {kind ? <span aria-hidden="true" style={{ display: "inline-block", width: 10, height: 10, borderRadius: 2, background: kind.color }} /> : null}
              <span>{line}</span>
            </li>
          );
        })}
      </ol>
    </section>
  );
}

function Block2dFixturesStoryHost({ exampleIds }: { readonly exampleIds: readonly string[] }): ReactElement {
  const debug = useMemo(
    () =>
      JSON.stringify(
        exampleIds.map((exampleId) => {
          const { snapshot } = block2dStoryStateFor(exampleId);
          return {
            exampleId,
            nodeKind: snapshot.nodeKind.id,
            camera: snapshot.camera2d,
            handleKindIds: snapshot.handleKinds.map((kind) => kind.id),
            handleAssignment: snapshot.handles.map((handle) => `${handle.id}:${handle.handleKind}`),
          };
        }),
      ),
    [exampleIds],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flex: "1 1 auto", minHeight: 0 }}>
        {exampleIds.map((exampleId) => (
          <Block2dFixturePane key={exampleId} exampleId={exampleId} />
        ))}
      </div>
      <pre data-testid="block2d-fixtures-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧱️block◻️2d/Fixtures",
  component: Block2dFixturesStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Block2dFixturesStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 📚️ Every shipped block2d example at once — a document that stops parsing or projecting breaks this story, not a hand-picked one. */
export const AllExamples: Story = {
  args: {
    exampleIds: BLOCK2D_STORY_EXAMPLE_IDS,
  },
};

/** 🎬️ Just `🌲️hexagonal-cut-concrete-forest-left`, full width — 6 handle kinds, 11 handles. */
export const HexagonalCutConcreteForestLeft: Story = {
  args: {
    exampleIds: ["hexagonal-cut-concrete-forest-left"],
  },
};

/** ➡️ Just `➡️hexagonal-cut-concrete-forest-right`, full width — the mirrored `h4`/`h5`/`h6` kind assignment. */
export const HexagonalCutConcreteForestRight: Story = {
  args: {
    exampleIds: ["hexagonal-cut-concrete-forest-right"],
  },
};
