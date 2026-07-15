import { useMemo } from "react";
import { Button, Icon, closestCenter, DndContext, DndCSS, SortableContext, useLabel, useSortable, verticalListSortingStrategy, type DragEndEvent } from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import type { ActionDescriptor, ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region ProtocolListHost
//#region Types
type ProtocolBlockRecord = { readonly id: string; readonly label: string; readonly kind: string; readonly description?: string };
type ProtocolStepRecord = { readonly id: string; readonly title: string; readonly description?: string; readonly blocks: readonly ProtocolBlockRecord[] };
type ProtocolPaletteEntryRecord = { readonly blockKind: string; readonly label: string; readonly iconId: string };
const PALETTE_DRAG_MIME = "application/x-semio-protocol-block-kind";
//#endregion Types

//#region Helpers
function resolveProtocolIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

function dispatchProtocolAction(onAction: (action: ActionDescriptor) => void, controllerId: string, action: string, args: Record<string, unknown>): void {
  onAction({ controllerId, action, args });
}
//#endregion Helpers

//#region SortableRow
/** 🧩 Wraps a row in dnd-kit's sortable machinery so it can be reordered within its enclosing `SortableContext`. */
function SortableRow({ id, children }: { readonly id: string; readonly children: (dragHandleProps: { readonly ref: (node: HTMLElement | null) => void; readonly style: React.CSSProperties }) => React.ReactNode }) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const style: React.CSSProperties = { transform: DndCSS.Transform.toString(transform), transition, opacity: isDragging ? 0.5 : 1 };
  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      {children({ ref: setNodeRef, style })}
    </div>
  );
}
//#endregion SortableRow

//#region Block
function ProtocolBlockCard({ block, stepId, controllerId, onAction }: { readonly block: ProtocolBlockRecord; readonly stepId: string; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <SortableRow id={block.id}>
      {() => (
        <div className="semio-protocol-block-card flex items-center gap-2 rounded border border-border bg-panel p-single" data-block-id={block.id}>
          <Icon icon="grip-vertical" size="small" />
          <div className="min-w-0 flex-1">
            <div className="truncate text-xs font-medium">{block.label}</div>
            <div className="truncate text-xs text-muted-foreground">{block.kind}</div>
          </div>
          <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchProtocolAction(onAction, controllerId, "removeBlock", { stepId, blockId: block.id })} />
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Block

//#region Step
function ProtocolStepCard({
  step,
  palette,
  controllerId,
  onAction,
}: {
  readonly step: ProtocolStepRecord;
  readonly palette: readonly ProtocolPaletteEntryRecord[];
  readonly controllerId: string;
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const blockIds = useMemo(() => step.blocks.map((block) => block.id), [step.blocks]);

  function handleBlockDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = step.blocks.findIndex((block) => block.id === over.id);
    if (index === -1) return;
    dispatchProtocolAction(onAction, controllerId, "moveBlock", { blockId: active.id, fromStepId: step.id, toStepId: step.id, index });
  }

  return (
    <SortableRow id={step.id}>
      {() => (
        <div
          className="semio-protocol-step-card flex flex-col gap-2 rounded border border-border bg-panel p-single"
          data-step-id={step.id}
          onDragOver={(event) => {
            event.preventDefault();
            event.dataTransfer.dropEffect = "copy";
          }}
          onDrop={(event) => {
            event.preventDefault();
            const kind = event.dataTransfer.getData(PALETTE_DRAG_MIME);
            if (!kind) return;
            dispatchProtocolAction(onAction, controllerId, "addBlock", { stepId: step.id, kind });
          }}
        >
          <div className="flex items-center gap-2">
            <Icon icon="grip-vertical" size="small" />
            <div className="min-w-0 flex-1 truncate text-sm font-medium">{step.title}</div>
            <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchProtocolAction(onAction, controllerId, "removeStep", { stepId: step.id })} />
          </div>
          {step.description && <div className="text-xs text-muted-foreground">{step.description}</div>}
          <DndContext collisionDetection={closestCenter} onDragEnd={handleBlockDragEnd}>
            <SortableContext items={blockIds} strategy={verticalListSortingStrategy}>
              <div className="flex flex-col gap-1">
                {step.blocks.map((block) => (
                  <ProtocolBlockCard key={block.id} block={block} stepId={step.id} controllerId={controllerId} onAction={onAction} />
                ))}
              </div>
            </SortableContext>
          </DndContext>
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Step

//#region Palette
function ProtocolPalettePanel({ palette, controllerId, onAction }: { readonly palette: readonly ProtocolPaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <div className="semio-protocol-palette flex shrink-0 flex-col gap-1 border-l border-border p-single">
      {palette.map((entry) => (
        <div
          key={entry.blockKind}
          draggable
          onDragStart={(event) => {
            event.dataTransfer.setData(PALETTE_DRAG_MIME, entry.blockKind);
            event.dataTransfer.effectAllowed = "copy";
          }}
          className="flex cursor-grab items-center gap-1 rounded border border-border p-single text-xs"
          onClick={() => dispatchProtocolAction(onAction, controllerId, "addBlock", { kind: entry.blockKind })}
        >
          <Icon icon={resolveProtocolIcon(entry.iconId)} size="small" />
          {entry.label}
        </div>
      ))}
    </div>
  );
}
//#endregion Palette

//#region Component
export function ProtocolListHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.protocolList;
  const steps = useMemo(() => {
    if (!scene) return [] as ProtocolStepRecord[];
    try {
      return JSON.parse(scene.stepsJson) as ProtocolStepRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const palette = useMemo(() => {
    if (!scene) return [] as ProtocolPaletteEntryRecord[];
    try {
      return JSON.parse(scene.paletteJson) as ProtocolPaletteEntryRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const stepIds = useMemo(() => steps.map((step) => step.id), [steps]);
  const stepsLabel = useLabel("ui.protocolList.steps");
  const addStepLabel = useLabel("ui.protocolList.addStep");
  const emptyLabel = useLabel("ui.protocolList.empty");

  if (!scene) return <div className="semio-protocol-list-empty">{emptyLabel}</div>;

  function handleStepDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = steps.findIndex((step) => step.id === over.id);
    if (index === -1) return;
    dispatchProtocolAction(onAction, node.controllerId, "moveStep", { stepId: active.id, index });
  }

  return (
    <div className="semio-protocol-list-host flex h-full min-h-0 w-full" data-surface-id={node.surfaceId}>
      <div className="flex min-w-0 flex-1 flex-col gap-2 overflow-auto p-single">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">{stepsLabel}</span>
          <Button className="h-medium shrink-0 px-2" icon="plus" text={addStepLabel} type="button" variant="outline" onClick={() => dispatchProtocolAction(onAction, node.controllerId, "addStep", {})} />
        </div>
        <DndContext collisionDetection={closestCenter} onDragEnd={handleStepDragEnd}>
          <SortableContext items={stepIds} strategy={verticalListSortingStrategy}>
            <div className="flex flex-col gap-2">
              {steps.map((step) => (
                <ProtocolStepCard key={step.id} step={step} palette={palette} controllerId={node.controllerId} onAction={onAction} />
              ))}
            </div>
          </SortableContext>
        </DndContext>
      </div>
      <ProtocolPalettePanel palette={palette} controllerId={node.controllerId} onAction={onAction} />
    </div>
  );
}
//#endregion Component
//#endregion ProtocolListHost
