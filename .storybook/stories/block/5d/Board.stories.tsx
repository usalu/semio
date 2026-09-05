// #region 🧲️Header
// 💻️ .storybook/stories/block/5d/Board.stories.tsx
// Specs: Host the framework renderer's `🖥️Board2dHost` for the block5d app's `block5d-board` window — the part
// kind's 2D projection plus its grips — driven by the REAL shipped example documents
// (`🗿️artifacts/🖐️5d/…/📚️examples/*/🖼️assets/*/🗣️.dsl.semio`).
// Summary: block5d is the ONE block app whose command table is fully wired (`BLOCK5D_RETAINED_TOOL_IDS`, all
// seven actions `Migrated`, a real `Block5dRetainedCommandJobFactory`), so this story's local reducer
// (`reduceBlock5dStoryAction`, `../scene.ts`) emulates the same six document-shaping actions the app itself
// admits: `patchPartKind`/`addGripKind`/`removeGripKind`/`addGrip`/`removeGrip`/`setActiveExample`. Same
// mount-the-host-against-a-`UiComponentSceneNode` pattern as `../2d/Board.stories.tsx` and
// `../puzzle/5d/Timeline.stories.tsx`; the 3D half of the same document lives in `./World.stories.tsx`
// (block5d has TWO windows, `block5d-board` and `block5d-world`, so the pair is split by window rather than
// composed into one file the way puzzle's single 5D timeline story is).
// The right-hand panel is the window's own Rust `render` output — part-kind label + `"2d grips: <n>"`
// (`✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs`) — extended with the per-grip-kind/per-grip geometry the
// board draws. As in every board story, `Board2dHost`'s canvas needs a board-2d WASM session that Storybook
// does not provide, so the panel is the assertable half.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Board2dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { BLOCK5D_STORY_EXAMPLE_IDS, block5dBoardRenderLines, block5dStoryStateFor, buildBlock5dBoardSceneNode, reduceBlock5dStoryAction, type Block5dStoryState } from "../scene";

//#region StoryHost
const BLOCK5D_STORY_CONTROLLER_ID = "block5d-story";

function Block5dBoardStoryHost({ initialExampleId, interactive }: { readonly initialExampleId: string; readonly interactive: boolean }): ReactElement {
  const [state, setState] = useState<Block5dStoryState>(() => block5dStoryStateFor(initialExampleId));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceBlock5dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: BLOCK5D_STORY_CONTROLLER_ID, action, args: { surfaceId: "block5d.play.board", ...args } }),
    [onAction],
  );

  const node = useMemo(() => buildBlock5dBoardSceneNode(state.snapshot, interactive), [state, interactive]);
  const lines = useMemo(() => block5dBoardRenderLines(state.snapshot), [state]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId: state.exampleId,
        partKind: state.snapshot.partKind.label,
        part2d: state.snapshot.part2d,
        gripKindIds: state.snapshot.gripKinds.map((kind) => kind.id),
        gripIds: state.snapshot.grips.map((grip) => grip.id),
        gripCount: state.snapshot.grips.length,
      }),
    [state],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        {BLOCK5D_STORY_EXAMPLE_IDS.map((exampleId) => (
          <button key={exampleId} type="button" data-testid={`block5d-example-${exampleId}`} onClick={() => dispatch("setActiveExample", { exampleId })} disabled={exampleId === state.exampleId}>
            {exampleId}
          </button>
        ))}
        <button type="button" data-testid="block5d-add-grip-kind" onClick={() => dispatch("addGripKind", { label: "Story Kind", color: "hsl(0 52% 48%)" })}>
          Add grip kind
        </button>
        <button type="button" data-testid="block5d-remove-grip-kind" onClick={() => dispatch("removeGripKind", {})}>
          Remove grip kind
        </button>
        <button type="button" data-testid="block5d-add-grip" onClick={() => dispatch("addGrip", {})}>
          Add grip
        </button>
        <button type="button" data-testid="block5d-remove-grip" onClick={() => dispatch("removeGrip", {})}>
          Remove grip
        </button>
        <button type="button" data-testid="block5d-patch-part-kind" onClick={() => dispatch("patchPartKind", { field: "label", value: `${state.snapshot.partKind.label} ✳️` })}>
          Patch part kind
        </button>
      </div>
      <div style={{ display: "flex", flex: "1 1 auto", minHeight: 0 }}>
        <div style={{ position: "relative", flex: "1 1 auto", minWidth: 0 }}>
          <Board2dHost node={node} onAction={onAction} />
        </div>
        <ol data-testid="block5d-board-window" style={{ flex: "0 0 380px", margin: 0, padding: "4px 8px", overflow: "auto", listStyle: "none", fontFamily: "var(--font-mono, monospace)", fontSize: 11 }}>
          {lines.map((line) => {
            const kind = state.snapshot.gripKinds.find((candidate) => line.startsWith(`  ◦ ${candidate.label} (${candidate.id})`));
            return (
              <li key={line} style={{ display: "flex", alignItems: "center", gap: 6, whiteSpace: "pre" }}>
                {kind ? <span aria-hidden="true" style={{ display: "inline-block", width: 10, height: 10, borderRadius: 2, background: kind.color }} /> : null}
                <span>{line}</span>
              </li>
            );
          })}
        </ol>
      </div>
      <pre data-testid="block5d-board-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧱️block🖐️5d/Board",
  component: Block5dBoardStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Block5dBoardStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎬️ `🌲️hexagonal-cut-concrete-forest-left` — one `b-l` grip kind and one grip at −0.1 rad / radius-2d 3 on a circular part glyph of radius 20. */
export const HexagonalCutConcreteForestLeft: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-left",
    interactive: true,
  },
};

/** 🏢️ `🏢️nakagin-capsule` (`Capsule J`) — one `door` grip kind and one grip at −π/2. */
export const NakaginCapsule: Story = {
  args: {
    initialExampleId: "nakagin-capsule",
    interactive: true,
  },
};

/** 🔒️ Read-only projection — `interactive: false`, matching the block5d viewer's non-mutating surface. */
export const ReadOnlyBoard: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-left",
    interactive: false,
  },
};
