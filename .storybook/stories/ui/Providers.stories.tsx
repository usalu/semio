// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Providers.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import {
  createIconComponent,
  FlowProvider,
  GhostProvider,
  GlassTierProvider,
  InteractionProvider,
  Panel,
  PanelDockProvider,
  singleTreeLeaf,
  TransactionProvider,
  TreeStateProvider,
  UiChromeCompactProvider,
  UiChromeLabelPolicyProvider,
  useFlow,
  useGlassTier,
  usePanelGhost,
  useTransaction,
  useTreeState,
  useUiChromeCompact,
  useUiChromeLabelPolicy,
  type PanelDock,
  type PanelTabDockMove,
} from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ReactNode } from "react";
// #endregion 🔌Adapters

// 🧬#region 🧬Providers
// One doc-style file for every context provider in the barrel that isn't already exercised end-to-end by
// a component-specific story (Panel/PanelChromeTabBar exercise PanelDockProvider indirectly; this file
// probes each provider directly via its own exported hook so the contract itself is documented in isolation).

const meta = {
  title: "🖱️ui⚛️react/Providers",
  component: GhostProvider,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof GhostProvider>;

export default meta;

type Story = StoryObj<typeof meta>;

function ProbeShell({ title, children }: { readonly title: string; readonly children: ReactNode }) {
  return (
    <div className="flex w-80 flex-col gap-single border bg-panel p-double">
      <h4 className="text-xs font-semibold">{title}</h4>
      {children}
    </div>
  );
}

// #region 👻GhostProvider
function GhostProbe() {
  const ghost = usePanelGhost();
  return (
    <div className="flex flex-col gap-single text-xs">
      <span>active: {String(ghost?.active)}</span>
      <div className="flex gap-single">
        <button className="border px-single" onClick={() => ghost?.begin(null)}>
          begin()
        </button>
        <button className="border px-single" onClick={() => ghost?.end()}>
          end()
        </button>
      </div>
    </div>
  );
}

export const Ghost: Story = {
  name: "GhostProvider",
  render: () => (
    <ProbeShell title="GhostProvider → usePanelGhost()">
      <GhostProvider>
        <GhostProbe />
      </GhostProvider>
    </ProbeShell>
  ),
};
// #endregion 👻GhostProvider

// #region 🔤InteractionProvider
function InteractionProbe() {
  const [active, setActive] = useState<string | undefined>(undefined);
  const commands = { setActiveInteraction: (_elementId?: string, interactionId?: string) => setActive(interactionId) };
  return (
    <InteractionProvider commands={commands} activeInteraction={active}>
      <div className="flex flex-col gap-single text-xs">
        <span>activeInteraction: {active ?? "(none)"}</span>
        <button className="border px-single" onClick={() => commands.setActiveInteraction("probe", "resize-handle")}>
          setActiveInteraction("resize-handle")
        </button>
      </div>
    </InteractionProvider>
  );
}

export const Interaction: Story = {
  name: "InteractionProvider",
  render: () => (
    <ProbeShell title="InteractionProvider (consumed internally by interaction-aware chrome — not exported for direct context reads)">
      <InteractionProbe />
    </ProbeShell>
  ),
};
// #endregion 🔤InteractionProvider

// #region 🎛️PanelDockProvider
const Layers = createIconComponent("layers");
const Info = createIconComponent("info");

function PanelDockDemo() {
  const [dock, setDock] = useState<PanelDock>({
    anchors: {
      "top-left": [singleTreeLeaf({ id: "dock.layers", icon: Layers, name: "Layers", tree: { sections: [] } })],
      "top-middle": [],
      "top-right": [singleTreeLeaf({ id: "dock.info", icon: Info, name: "Info", tree: { sections: [] } })],
      "bottom-left": [],
      "bottom-middle": [],
      "bottom-right": [],
    },
  });
  const onTabDockDrop = (move: PanelTabDockMove) => {
    setDock((prev) => {
      const fromTabs = prev.anchors[move.fromAnchor].filter((tab) => tab.id !== move.tabId);
      const moved = prev.anchors[move.fromAnchor].find((tab) => tab.id === move.tabId);
      if (!moved) return prev;
      const toTabs = [...prev.anchors[move.target.anchor], moved];
      return { anchors: { ...prev.anchors, [move.fromAnchor]: fromTabs, [move.target.anchor]: toTabs } };
    });
  };
  return (
    <PanelDockProvider dock={dock} onTabDockDrop={onTabDockDrop} onTreeUnitDockDrop={() => {}}>
      <div className="relative h-64 w-full border bg-base">
        <Panel anchor="top-left" tabs={dock.anchors["top-left"]} />
        <Panel anchor="top-right" tabs={dock.anchors["top-right"]} />
      </div>
    </PanelDockProvider>
  );
}

export const PanelDock: Story = {
  name: "PanelDockProvider (drag a tab between the two panels)",
  render: () => (
    <ProbeShell title="PanelDockProvider → wires pointer-capture tab dragging across every Panel below it">
      <PanelDockDemo />
    </ProbeShell>
  ),
};
// #endregion 🎛️PanelDockProvider

// #region 🐹TransactionProvider
function TransactionProbe() {
  const transaction = useTransaction();
  const [log, setLog] = useState<string[]>([]);
  return (
    <div className="flex flex-col gap-single text-xs">
      <div className="flex gap-single">
        <button
          className="border px-single"
          onClick={() => {
            transaction?.start?.();
            setLog((prev) => [...prev, "start"]);
          }}
        >
          start()
        </button>
        <button
          className="border px-single"
          onClick={() => {
            transaction?.finalize?.();
            setLog((prev) => [...prev, "finalize"]);
          }}
        >
          finalize()
        </button>
      </div>
      <pre>{log.join(" → ") || "(no calls yet)"}</pre>
    </div>
  );
}

export const Transaction: Story = {
  name: "TransactionProvider",
  render: () => (
    <ProbeShell title="TransactionProvider → useTransaction()">
      <TransactionProvider transaction={{ start: () => {}, finalize: () => {}, abort: () => {} }}>
        <TransactionProbe />
      </TransactionProvider>
    </ProbeShell>
  ),
};
// #endregion 🐹TransactionProvider

// #region 📜TreeStateProvider
function TreeStateProbe() {
  const { openStates, setOpenState } = useTreeState();
  return (
    <div className="flex flex-col gap-single text-xs">
      <span>capsules open: {String(openStates["item-capsules"] ?? false)}</span>
      <button className="border px-single" onClick={() => setOpenState("item-capsules", !(openStates["item-capsules"] ?? false))}>
        toggle
      </button>
    </div>
  );
}

export const TreeState: Story = {
  name: "TreeStateProvider",
  render: () => (
    <ProbeShell title="TreeStateProvider → useTreeState()">
      <TreeStateProvider>
        <TreeStateProbe />
      </TreeStateProvider>
    </ProbeShell>
  ),
};
// #endregion 📜TreeStateProvider

// #region 🎛️UiChromeCompactProvider
function UiChromeCompactProbe() {
  const compact = useUiChromeCompact();
  return <span className="text-xs">compact: {String(compact)}</span>;
}

export const ChromeCompact: Story = {
  name: "UiChromeCompactProvider",
  render: () => (
    <div className="flex gap-double">
      <ProbeShell title="compact=false">
        <UiChromeCompactProvider compact={false}>
          <UiChromeCompactProbe />
        </UiChromeCompactProvider>
      </ProbeShell>
      <ProbeShell title="compact=true">
        <UiChromeCompactProvider compact={true}>
          <UiChromeCompactProbe />
        </UiChromeCompactProvider>
      </ProbeShell>
    </div>
  ),
};
// #endregion 🎛️UiChromeCompactProvider

// #region 🏷️UiChromeLabelPolicyProvider
function UiChromeLabelPolicyProbe() {
  const policy = useUiChromeLabelPolicy();
  return <span className="text-xs">policy: {policy}</span>;
}

export const ChromeLabelPolicy: Story = {
  name: "UiChromeLabelPolicyProvider",
  render: () => (
    <div className="flex gap-double">
      <ProbeShell title='policy="compact" (default)'>
        <UiChromeLabelPolicyProvider policy="compact">
          <UiChromeLabelPolicyProbe />
        </UiChromeLabelPolicyProvider>
      </ProbeShell>
      <ProbeShell title='policy="always" (e.g. navbar)'>
        <UiChromeLabelPolicyProvider policy="always">
          <UiChromeLabelPolicyProbe />
        </UiChromeLabelPolicyProvider>
      </ProbeShell>
    </div>
  ),
};
// #endregion 🏷️UiChromeLabelPolicyProvider

// #region 🧭FlowProvider
function FlowProbe() {
  const flow = useFlow();
  return (
    <span className="text-xs">
      inline: {flow.inline}, block: {flow.block}
    </span>
  );
}

export const Flow: Story = {
  name: "FlowProvider (nesting overrides only what it passes)",
  render: () => (
    <ProbeShell title="FlowProvider → useFlow()">
      <FlowProvider inline="rtl" block="up">
        <FlowProbe />
        <FlowProvider block="down">
          <FlowProbe />
        </FlowProvider>
      </FlowProvider>
    </ProbeShell>
  ),
};
// #endregion 🧭FlowProvider

// #region 🪟GlassTierProvider
function GlassTierProbe() {
  const tier = useGlassTier();
  return <span className="text-xs">tier: {tier}</span>;
}

export const GlassTier: Story = {
  name: "GlassTierProvider",
  render: () => (
    <ProbeShell title="GlassTierProvider → useGlassTier()">
      <GlassTierProvider tier="windowOptions">
        <GlassTierProbe />
      </GlassTierProvider>
    </ProbeShell>
  ),
};
// #endregion 🪟GlassTierProvider
// #endregion 🧬Providers
