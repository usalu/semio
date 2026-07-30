// #region 🧲Header
// 💻 .storybook/stories/framework/hosts/UiInterpreter.stories.tsx
// Specs: Exercise `interpretUiNode` (`framework/os/renderer/js/react/index.tsx`) directly against hand-written
// `UiNode` JSON trees — no wasm, no scene protocol, just the declarative-UI half of the renderer barrel.
// Summary: Three fixtures — a button-in-a-stack, a two-section tree (with a nested item and an inline
// `toggle` control), and a field/section "properties panel" (input/select/slider/keyValue) — each rendered
// through `interpretUiNode`, which is what transitively exercises `renderUiControl` (every `UiControlNode`
// case) and `uiTreeNodeToTreePanelConfig` (the `"tree"` case delegates to `DeclarativeTreePanel`, which calls
// it directly). A debug readout records the last dispatched `ActionDescriptor` so button clicks, toggle
// flips, select/slider changes, and tree selection all visibly round-trip.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type ReactElement } from "react";

import { interpretUiNode } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, UiNode } from "@semio-tech/framework-core";

const STORY_UI_INTERPRETER_CONTROLLER_ID = "ui-interpreter-story";

//#region Fixtures
/** 🔘 A stack of text + a `button` control node. */
const BUTTON_TREE: UiNode = {
  type: "stack",
  direction: "vertical",
  gap: "tight",
  padding: "none",
  children: [
    { type: "text", value: "Click the button below:" },
    { type: "button", id: "story-add-item", iconId: "plus", label: "Add Item", action: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "addItem", args: {} } },
    { type: "separator" },
    { type: "text", value: "A disabled, loading variant:", emphasize: true },
    { type: "button", id: "story-add-item-loading", iconId: "loader", label: "Working…", action: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "noOperation", args: {} }, loading: true, disabled: true },
  ],
};

/** 🌳 Two sections — "Layers" (a selected leaf, a nested child, a hover-reveal row action) and "Settings" (a `toggle` `UiControlNode` embedded in a tree item). */
const TREE_TREE: UiNode = {
  type: "tree",
  selectedIds: ["item-background"],
  selectionChange: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "selectTreeIds", args: {} },
  sections: [
    {
      id: "sec-layers",
      label: "Layers",
      defaultOpen: true,
      items: [
        {
          id: "item-background",
          label: "Background",
          iconId: "square",
          selected: true,
          action: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "selectItem", args: { id: "item-background" } },
          actions: [{ iconId: "trash-2", label: "Delete", revealOnHover: true, action: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "deleteItem", args: { id: "item-background" } } }],
        },
        {
          id: "item-foreground",
          label: "Foreground",
          iconId: "square",
          defaultOpen: true,
          items: [{ id: "item-foreground-shadow", label: "Shadow", iconId: "circle" }],
        },
      ],
    },
    {
      id: "sec-settings",
      label: "Settings",
      defaultOpen: true,
      items: [
        {
          id: "item-visible",
          label: "Visible",
          control: { type: "toggle", id: "toggle-visible", iconId: "eye", pressed: true, text: "Visible", onChange: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "toggleVisible", args: {} } },
        },
      ],
    },
  ],
};

/** 🎛️ A `section` wrapping two `field`-labeled controls (input/select), a bare `slider`, and a `keyValue` summary — the "properties panel" shape. */
const PANEL_TREE: UiNode = {
  type: "section",
  id: "panel-properties",
  label: "Properties",
  defaultOpen: true,
  children: [
    { type: "field", id: "field-name", label: "Name", description: "Display label", child: { type: "input", id: "input-name", inputKind: "text", value: "Board A", onChange: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "setName", args: {} } } },
    {
      type: "field",
      id: "field-kind",
      label: "Kind",
      child: {
        type: "select",
        id: "select-kind",
        value: "seed",
        items: [
          { value: "seed", label: "Seed" },
          { value: "handle", label: "Handle" },
        ],
        onChange: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "setKind", args: {} },
      },
    },
    { type: "slider", id: "slider-opacity", value: 0.8, min: 0, max: 1, step: 0.05, unit: "α", onChange: { controllerId: STORY_UI_INTERPRETER_CONTROLLER_ID, action: "setOpacity", args: {} } },
    { type: "keyValue", entries: [{ label: "Id", value: "node-42" }, { label: "Updated", value: "2026-07-19" }] },
  ],
};
//#endregion Fixtures

//#region StoryHost
function UiInterpreterStoryHost({ node }: { readonly node: UiNode }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ flex: "1 1 auto", minHeight: 0, overflow: "auto", padding: 16 }}>{interpretUiNode(node, { onAction })}</div>
      <pre data-testid="ui-interpreter-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌hosts/UiInterpreter",
  component: UiInterpreterStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof UiInterpreterStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🔘 `renderUiControl`'s `"button"` case, plus the `"separator"`/`"text"` node kinds. */
export const Button: Story = {
  args: { node: BUTTON_TREE },
};

/** 🌳 The `"tree"` node case — `DeclarativeTreePanel` calls `uiTreeNodeToTreePanelConfig` internally to build sections/items/nested items, a row action, and a `toggle` `UiControlNode`. */
export const Tree: Story = {
  args: { node: TREE_TREE },
};

/** 🎛️ `"section"`/`"field"` wrapping `renderUiControl`'s `"input"`/`"select"` cases, plus the bare `"slider"` and `"keyValue"` node kinds. */
export const Panel: Story = {
  args: { node: PANEL_TREE },
};
