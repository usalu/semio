// #region 🧲️Header
// 💻️ .storybook/stories/block/2d/Board.stories.tsx
// Specs: Host the framework renderer's `🖥️Board2dHost` for the block2d app's `block2d-board` window, driven by
// the REAL shipped example documents (`🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/*/🖼️assets/*/🗣️.dsl.semio`).
// Summary: Mounts the host directly against a `UiComponentSceneNode` (`componentKind: "board-2d"`) whose
// `fixtureJson`/`glyphCatalogsJson` are projected from a parsed `Block2dSnapshot` — the same shape
// `../puzzle/2d/Board.stories.tsx` mounts puzzle's board with — and a story-local reducer
// (`reduceBlock2dStoryAction`, `../scene.ts`) emulates block2d's own `app_commands!` set
// (`patchNodeKind`/`addHandleKind`/`removeHandleKind`/`addHandle`/`removeHandle`/`setActiveExample`) so the
// controlled scene ⇄️ session loop round-trips with no dev server and no plugin WASM.
// The window's Rust `render` (`👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs`) is a `ui_stack_vertical` of
// `ui_text` lines — node-kind label, every handle kind (label/id/color) and every handle
// (id/kind/angle°/radius) — so `block2dBoardRenderLines` renders those exact lines beside the canvas: that
// panel is the assertable, always-visible half of this story. `Board2dHost` paints through a board-2d WASM
// session and Storybook provides no `BoardSessionFactoryContext` (`.storybook/preview.tsx`'s `WASM_LOADERS`
// has no board-2d entry), so the canvas itself stays blank here exactly as it does in the puzzle board stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { Board2dHost } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";
import type { ActionDescriptor } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️";

import { BLOCK2D_STORY_EXAMPLE_IDS, block2dBoardRenderLines, block2dStoryStateFor, buildBlock2dSceneNode, reduceBlock2dStoryAction, type Block2dStoryState } from "../scene";

//#region StoryHost
const BLOCK2D_STORY_CONTROLLER_ID = "block2d-story";

function Block2dBoardStoryHost({ initialExampleId, interactive }: { readonly initialExampleId: string; readonly interactive: boolean }): ReactElement {
  const [state, setState] = useState<Block2dStoryState>(() => block2dStoryStateFor(initialExampleId));

  const onAction = useCallback((descriptor: ActionDescriptor): void => {
    setState((current) => reduceBlock2dStoryAction(current, descriptor.action, descriptor.args));
  }, []);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void => onAction({ controllerId: BLOCK2D_STORY_CONTROLLER_ID, action, args: { surfaceId: "block2d.play.board", ...args } }),
    [onAction],
  );

  const node = useMemo(() => buildBlock2dSceneNode(state.snapshot, interactive), [state, interactive]);
  const lines = useMemo(() => block2dBoardRenderLines(state.snapshot), [state]);
  const debug = useMemo(
    () =>
      JSON.stringify({
        exampleId: state.exampleId,
        nodeKind: state.snapshot.nodeKind.label,
        handleKindCount: state.snapshot.handleKinds.length,
        handleCount: state.snapshot.handles.length,
        handleKindIds: state.snapshot.handleKinds.map((kind) => kind.id),
        handleIds: state.snapshot.handles.map((handle) => handle.id),
      }),
    [state],
  );

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ display: "flex", flexWrap: "wrap", gap: 4, padding: 4 }}>
        {BLOCK2D_STORY_EXAMPLE_IDS.map((exampleId) => (
          <button key={exampleId} type="button" data-testid={`block2d-example-${exampleId}`} onClick={() => dispatch("setActiveExample", { exampleId })} disabled={exampleId === state.exampleId}>
            {exampleId}
          </button>
        ))}
        <button type="button" data-testid="block2d-add-handle-kind" onClick={() => dispatch("addHandleKind", { label: "Story Kind", color: "hsl(0 52% 48%)" })}>
          Add handle kind
        </button>
        <button type="button" data-testid="block2d-remove-handle-kind" onClick={() => dispatch("removeHandleKind", {})}>
          Remove handle kind
        </button>
        <button type="button" data-testid="block2d-add-handle" onClick={() => dispatch("addHandle", {})}>
          Add handle
        </button>
        <button type="button" data-testid="block2d-remove-handle" onClick={() => dispatch("removeHandle", {})}>
          Remove handle
        </button>
        <button type="button" data-testid="block2d-patch-node-kind" onClick={() => dispatch("patchNodeKind", { field: "label", value: `${state.snapshot.nodeKind.label} ✳️` })}>
          Patch node kind
        </button>
      </div>
      <div style={{ display: "flex", flex: "1 1 auto", minHeight: 0 }}>
        <div style={{ position: "relative", flex: "1 1 auto", minWidth: 0 }}>
          <Board2dHost node={node} onAction={onAction} />
        </div>
        <ol data-testid="block2d-board-window" style={{ flex: "0 0 380px", margin: 0, padding: "4px 8px", overflow: "auto", listStyle: "none", fontFamily: "var(--font-mono, monospace)", fontSize: 11 }}>
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
      </div>
      <pre data-testid="block2d-board-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {debug}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🧱️block◻️2d",
  component: Block2dBoardStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Block2dBoardStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🎬️ The `🌲️hexagonal-cut-concrete-forest-left` example — 6 handle kinds, 11 handles; every toolbar button round-trips through the story-local `Block2dCommand` emulator. */
export const HexagonalCutConcreteForestLeft: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-left",
    interactive: true,
  },
};

/** ➡️ The `➡️hexagonal-cut-concrete-forest-right` example — the same 6 handle kinds with a mirrored kind assignment on `h4`/`h5`/`h6`. */
export const HexagonalCutConcreteForestRight: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-right",
    interactive: true,
  },
};

/** 🔒️ The same document with `interactive: false` — the read-only `block2d-view-board` viewer window, whose Rust `render` produces exactly the lines in the right-hand panel. */
export const ReadOnlyViewerBoard: Story = {
  args: {
    initialExampleId: "hexagonal-cut-concrete-forest-left",
    interactive: false,
  },
};
