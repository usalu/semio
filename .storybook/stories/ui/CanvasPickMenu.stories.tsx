// #region 🧲️Header

// 🥼️ .storybook/stories/ui/CanvasPickMenu.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { CanvasPickMenu, useCanvasPickInteraction, type CanvasHoverFocus, type CanvasPickRequest, type CanvasPickTarget } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

// 🎯️#region 🎯️CanvasPickMenu
const meta = {
  title: "🖱️ui⚛️react/CanvasPickMenu",
  component: CanvasPickMenu,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof CanvasPickMenu>;

export default meta;

type Story = StoryObj<typeof meta>;

const overlappingTargets: CanvasPickTarget[] = [
  { domain: "piece", id: "capsule-j-04", generality: 2, label: "Capsule J #04" },
  { domain: "wall", id: "wall-12", generality: 1, label: "Wall 12" },
  { domain: "opening", id: "opening-3", generality: 0, label: "Opening 3" },
];

/** @emoji 🎯️ Toggleable host so `Default` can show both the open menu and its dismissed (no-operation) state on demand. */
const CanvasPickMenuDemo = ({ request }: { readonly request: CanvasPickRequest | null }) => {
  const [currentRequest, setCurrentRequest] = useState<CanvasPickRequest | null>(request);
  const [hoveredKey, setHoveredKey] = useState<string | null>(null);
  const [pickedLabel, setPickedLabel] = useState<string | null>(null);
  return (
    <div className="relative flex h-60 w-90 items-center justify-center border text-sm text-muted-foreground">
      {currentRequest ? "Pick menu is open (portalled to document.body)" : "Pick menu is dismissed"}
      {pickedLabel ? <p className="absolute bottom-2 text-xs">Picked: {pickedLabel}</p> : null}
      <CanvasPickMenu
        request={currentRequest}
        hoveredKey={hoveredKey}
        onHoverKey={setHoveredKey}
        onPick={(target) => {
          setPickedLabel(target.label);
          setCurrentRequest(null);
        }}
        onDismiss={() => setCurrentRequest(null)}
      />
    </div>
  );
};

export const Default: Story = {
  args: {
    request: { targets: overlappingTargets, client: { x: 320, y: 220 } },
    hoveredKey: null,
    onHoverKey: () => {},
    onPick: () => {},
    onDismiss: () => {},
  },
  render: (args) => <CanvasPickMenuDemo request={args.request} />,
};

export const Dismissed: Story = {
  args: {
    request: null,
    hoveredKey: null,
    onHoverKey: () => {},
    onPick: () => {},
    onDismiss: () => {},
  },
  render: (args) => <CanvasPickMenuDemo request={args.request} />,
};

// #endregion 🎯️CanvasPickMenu

// #region 🪝️useCanvasPickInteraction
const leftHalfTargets: CanvasPickTarget[] = [
  { domain: "piece", id: "capsule-l-01", generality: 2, label: "Capsule L #01" },
  { domain: "wall", id: "wall-04", generality: 1, label: "Wall 04" },
];
const rightHalfTargets: CanvasPickTarget[] = [{ domain: "piece", id: "capsule-p-02", generality: 2, label: "Capsule P #02" }];

/** @emoji 🪝️ Fake "canvas" surface wiring pointer events straight through {@link useCanvasPickInteraction} — a stand-in for the real WebGL/2d canvas hosts that share this hook. */
const CanvasPickInteractionDemo = () => {
  const [focus, setFocus] = useState<CanvasHoverFocus | null>(null);
  const [lastSelected, setLastSelected] = useState<string | null>(null);
  const interaction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const el = document.getElementById("canvas-pick-interaction-story-surface");
      const rect = el?.getBoundingClientRect();
      const isLeftHalf = rect ? client.x - rect.left < rect.width / 2 : true;
      return isLeftHalf ? leftHalfTargets : rightHalfTargets;
    },
    onHoverFocus: setFocus,
    onSelectTarget: (target) => setLastSelected(target.label),
  });

  return (
    <div className="flex w-90 flex-col gap-single">
      <div
        id="canvas-pick-interaction-story-surface"
        className="flex h-40 items-center justify-center border bg-muted/30 text-xs text-muted-foreground"
        onPointerDown={(event) => interaction.onCanvasPointerDown({ x: event.clientX, y: event.clientY })}
        onPointerMove={(event) => interaction.onCanvasPointerMove({ x: event.clientX, y: event.clientY })}
        onPointerUp={(event) => interaction.onCanvasPointerUp({ x: event.clientX, y: event.clientY })}
        onPointerLeave={() => interaction.onCanvasPointerLeave()}
      >
        Click left half (2 overlapping targets) or right half (1 target)
      </div>
      <p className="text-xs">Hover focus: {focus?.target?.label ?? "—"}</p>
      <p className="text-xs">Last selected: {lastSelected ?? "—"}</p>
      <CanvasPickMenu request={interaction.pickMenu} hoveredKey={interaction.menuHoveredKey} onHoverKey={interaction.onMenuHoverKey} onPick={interaction.onMenuPick} onDismiss={interaction.dismissPickMenu} />
    </div>
  );
};

export const PointerInteraction: Story = {
  name: "useCanvasPickInteraction",
  args: {
    request: null,
    hoveredKey: null,
    onHoverKey: () => {},
    onPick: () => {},
    onDismiss: () => {},
  },
  render: () => <CanvasPickInteractionDemo />,
};

// #endregion 🪝️useCanvasPickInteraction
