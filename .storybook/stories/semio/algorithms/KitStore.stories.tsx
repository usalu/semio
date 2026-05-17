// #region 🧲Header
// semio-algorithms/Kit/Store — drive semio WASM KitStoreHandle in Storybook
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion

import type { Design, Kit } from "@semio/react";
import { Design as DesignEntity } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import metabolismKit from "@semio/assets/fixtures/metabolism.kit.semio.json";
import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/algorithms";

import { CommandForm } from "../../../semio/algorithms/kit-store/CommandForm";
import { ALL_CHANGE_KIT_ROOT_KEYS, CHANGE_TYPE_COMMAND_KEYS, KIT_STORE_COVERAGE_ROWS } from "../../../semio/algorithms/kit-store/commandSchema";
import { DiffViewer } from "../../../semio/algorithms/kit-store/DiffViewer";
import { applyEntityPlaceholders, EntityPicker } from "../../../semio/algorithms/kit-store/EntityPicker";
import { EventsFeed } from "../../../semio/algorithms/kit-store/EventsFeed";
import { HistoryControls, KitTreeGraph } from "../../../semio/algorithms/kit-store/HistoryControls";
import { SnapshotViewer } from "../../../semio/algorithms/kit-store/SnapshotViewer";
import { useKitStore } from "../../../semio/algorithms/kit-store/useKitStore";

const kitJson = metabolismKit as unknown;
const anyKit = kitJson as { designs?: { id: string }[]; types?: { id: string }[]; name?: string };
const seedDesign = anyKit.designs?.[0];
const firstTypeId = anyKit.types?.[0]?.id ?? "";
const firstDesignId = seedDesign?.id ?? "";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "ks-ent", kind: WindowKind.DESIGN_INPUT, label: "Entity ids", component: EntityWindow },
  { id: "ks-hist", kind: WindowKind.DESIGN_INPUT, label: "VCS + history", component: HistoryWindow },
  { id: "ks-tree", kind: WindowKind.DESIGN_INPUT, label: "Kit tree", component: KitTreeWindow },
  { id: "ks-cmd", kind: WindowKind.DESIGN_INPUT, label: "Commands (JSON)", component: CommandWindow },
  { id: "ks-diff", kind: WindowKind.DESIGN_INPUT, label: "Last result", component: DiffWindow },
  { id: "ks-snap", kind: WindowKind.DESIGN_INPUT, label: "Snapshot / theKit", component: SnapWindow },
  { id: "ks-evt", kind: WindowKind.DESIGN_INPUT, label: "Events", component: EventsWindow },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      {
        kind: "column" as const,
        size: 26,
        children: [
          {
            kind: "stack" as const,
            size: 40,
            children: [{ kind: "window" as const, windowKindId: "ks-ent", title: "Entity ids" }],
          },
          {
            kind: "stack" as const,
            size: 60,
            children: [{ kind: "window" as const, windowKindId: "ks-hist", title: "VCS + history" }],
          },
        ],
      },
      {
        // 🌳 GitKraken-style kit tree sits next to VCS controls so committing + visualising share screen space.
        kind: "stack" as const,
        size: 24,
        children: [{ kind: "window" as const, windowKindId: "ks-tree", title: "Kit tree" }],
      },
      {
        kind: "column" as const,
        size: 26,
        children: [
          {
            kind: "stack" as const,
            size: 55,
            children: [{ kind: "window" as const, windowKindId: "ks-cmd", title: "Commands" }],
          },
          {
            kind: "stack" as const,
            size: 45,
            children: [{ kind: "window" as const, windowKindId: "ks-diff", title: "Last result" }],
          },
        ],
      },
      {
        kind: "column" as const,
        size: 24,
        children: [
          {
            kind: "stack" as const,
            size: 55,
            children: [{ kind: "window" as const, windowKindId: "ks-snap", title: "Snapshot / theKit" }],
          },
          {
            kind: "stack" as const,
            size: 45,
            children: [{ kind: "window" as const, windowKindId: "ks-evt", title: "Events" }],
          },
        ],
      },
    ],
  },
};

type KitStoreFrameCtx = ReturnType<typeof useKitStore>;

const KitFrameContext = React.createContext<KitStoreFrameCtx | null>(null);

function useKitFrame(): KitStoreFrameCtx {
  const c = React.useContext(KitFrameContext);
  if (!c) throw new Error("Kit store frame");
  return c;
}

function EntityWindow() {
  const s = useKitFrame();
  return (
    <div className="h-full min-h-0">
      {!s.handle && !s.initErr ? <div className="text-muted-foreground p-2 text-xs">Loading WASM…</div> : null}
      {s.initErr ? <div className="text-destructive p-2 text-xs">{s.initErr}</div> : null}
      <EntityPicker
        handle={s.handle}
        jsonForPlaceholders={s.cmdMode === "readKit" ? s.readJson : s.changeJson}
        onJsonChange={s.cmdMode === "readKit" ? s.setReadJson : s.setChangeJson}
        onApplyPlaceholders={(raw) =>
          applyEntityPlaceholders(raw, {
            typeId: firstTypeId,
            designId: firstDesignId,
            fileId: "",
            folderId: "",
            authorId: "",
            pieceId: "",
            connectionId: "",
          })
        }
      />
      <div className="text-muted-foreground max-h-40 overflow-auto border-t border-zinc-200 p-1 text-[9px] dark:border-zinc-800">
        <div className="font-medium text-foreground">Coverage checklist</div>
        <ul className="m-0 list-disc pl-4">
          {KIT_STORE_COVERAGE_ROWS.map((r) => (
            <li key={r.group + r.key}>
              {r.group}: {r.key}
            </li>
          ))}
        </ul>
        <div className="mt-1 font-mono">
          root keys ({ALL_CHANGE_KIT_ROOT_KEYS.length}): {ALL_CHANGE_KIT_ROOT_KEYS.join(", ")}
        </div>
        <div className="mt-1 font-mono">ChangeType keys ({CHANGE_TYPE_COMMAND_KEYS.length})</div>
      </div>
    </div>
  );
}

function HistoryWindow() {
  const s = useKitFrame();
  return (
    <div className="h-full min-h-0">
      <HistoryControls
        handle={s.handle}
        initErr={s.initErr}
        onLog={s.log}
        sessionId={s.sessionId}
        onSessionId={s.setSessionId}
        draftId={s.draftId}
        onDraftId={s.setDraftId}
        txId={s.txId}
        onTxId={s.setTxId}
        cpId={s.cpId}
        onCpId={s.setCpId}
        altId={s.altId}
        onAltId={s.setAltId}
        msg={s.msg}
        onMsg={s.setMsg}
        onInspectCheckpoint={(checkpointId) => {
          s.setMatAt(checkpointId);
        }}
      />
    </div>
  );
}

function KitTreeWindow() {
  const s = useKitFrame();
  const selection = React.useMemo(
    () => ({
      onCheckpointSelect: s.setCpId,
      onAlternativeSelect: s.setAltId,
      onSessionSelect: s.setSessionId,
      onDraftSelect: s.setDraftId,
    }),
    [s.setCpId, s.setAltId, s.setSessionId, s.setDraftId],
  );
  return (
    <div className="h-full min-h-0">
      <KitTreeGraph handle={s.handle} selection={selection} selectedCheckpointId={s.cpId} selectedAlternativeId={s.altId} selectedSessionId={s.sessionId} selectedDraftId={s.draftId} />
    </div>
  );
}

function CommandWindow() {
  const s = useKitFrame();
  return (
    <div className="h-full min-h-0">
      <CommandForm
        handle={s.handle}
        mode={s.cmdMode}
        onMode={s.setCmdMode}
        changeJson={s.changeJson}
        onChangeJson={s.setChangeJson}
        readJson={s.readJson}
        onReadJson={s.setReadJson}
        executeJson={s.executeJson}
        onExecuteJson={s.setExecuteJson}
        onCommandRun={s.onCommandRun}
      />
    </div>
  );
}

function DiffWindow() {
  const s = useKitFrame();
  const last = s.last
    ? {
        forward: s.last.forward,
        result: s.last.result,
        error: s.last.error,
      }
    : null;
  return (
    <div className="h-full min-h-0">
      <DiffViewer last={last} />
    </div>
  );
}

function SnapWindow() {
  const s = useKitFrame();
  return (
    <div className="h-full min-h-0">
      <SnapshotViewer handle={s.handle} matAt={s.matAt} onMatAt={s.setMatAt} />
    </div>
  );
}

function EventsWindow() {
  const s = useKitFrame();
  return <EventsFeed events={s.events} onClear={s.onClear} filter={s.filter} onFilterChange={s.setFilter} />;
}

function KitStoreFrame() {
  const store = useKitStore(kitJson);

  const design = React.useMemo(() => {
    if (seedDesign) {
      return new DesignEntity(seedDesign as any);
    }
    return new DesignEntity({} as any);
  }, []);

  const context = React.useMemo<AlgorithmContextValue>(
    () => ({
      kit: kitJson as unknown as Kit,
      design: design as Design,
      outputDesign: design as Design,
      selectedPieceIds: [],
      onSelectedPieceIdsChange: () => {},
    }),
    [design],
  );

  return (
    <KitFrameContext.Provider value={store}>
      <AlgorithmApp id="kit-store" label="Kit / Store (WASM)" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />
    </KitFrameContext.Provider>
  );
}

const meta = {
  title: "semio/algorithms/Store",
  component: KitStoreFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof KitStoreFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <KitStoreFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Kit.*Store.*WASM/i, { timeout: 120_000 });
  },
};
