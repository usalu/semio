import { useMemo } from "react";
import { Button, Icon, closestCenter, DndContext, DndCSS, SortableContext, useLabel, useSortable, verticalListSortingStrategy, type DragEndEvent } from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import type { ActionDescriptor, ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region BlockListHost
//#region Types
type BlockRecord = { readonly id: string; readonly label: string; readonly kind: string; readonly description?: string };
type StepRecord = { readonly id: string; readonly title: string; readonly description?: string; readonly blocks: readonly BlockRecord[] };
type PaletteEntryRecord = { readonly blockKind: string; readonly label: string; readonly iconId: string };
const PALETTE_DRAG_MIME = "application/x-semio-block-list-block-kind";
//#endregion Types

//#region Helpers
function resolveBlockIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

function dispatchBlockListAction(onAction: (action: ActionDescriptor) => void, controllerId: string, action: string, args: Record<string, unknown>): void {
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
function BlockCard({ block, stepId, controllerId, onAction }: { readonly block: BlockRecord; readonly stepId: string; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <SortableRow id={block.id}>
      {() => (
        <div className="semio-block-card flex items-center gap-2 rounded border border-border bg-panel p-single" data-block-id={block.id}>
          <Icon icon="grip-vertical" size="small" />
          <div className="min-w-0 flex-1">
            <div className="truncate text-xs font-medium">{block.label}</div>
            <div className="truncate text-xs text-muted-foreground">{block.kind}</div>
          </div>
          <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeBlock", { stepId, blockId: block.id })} />
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Block

//#region Step
function StepCard({
  step,
  palette,
  controllerId,
  onAction,
}: {
  readonly step: StepRecord;
  readonly palette: readonly PaletteEntryRecord[];
  readonly controllerId: string;
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const blockIds = useMemo(() => step.blocks.map((block) => block.id), [step.blocks]);

  function handleBlockDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = step.blocks.findIndex((block) => block.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, controllerId, "moveBlock", { blockId: active.id, fromStepId: step.id, toStepId: step.id, index });
  }

  return (
    <SortableRow id={step.id}>
      {() => (
        <div
          className="semio-step-card flex flex-col gap-2 rounded border border-border bg-panel p-single"
          data-step-id={step.id}
          onDragOver={(event) => {
            event.preventDefault();
            event.dataTransfer.dropEffect = "copy";
          }}
          onDrop={(event) => {
            event.preventDefault();
            const kind = event.dataTransfer.getData(PALETTE_DRAG_MIME);
            if (!kind) return;
            dispatchBlockListAction(onAction, controllerId, "addBlock", { stepId: step.id, kind });
          }}
        >
          <div className="flex items-center gap-2">
            <Icon icon="grip-vertical" size="small" />
            <div className="min-w-0 flex-1 truncate text-sm font-medium">{step.title}</div>
            <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeStep", { stepId: step.id })} />
          </div>
          {step.description && <div className="text-xs text-muted-foreground">{step.description}</div>}
          <DndContext collisionDetection={closestCenter} onDragEnd={handleBlockDragEnd}>
            <SortableContext items={blockIds} strategy={verticalListSortingStrategy}>
              <div className="flex flex-col gap-1">
                {step.blocks.map((block) => (
                  <BlockCard key={block.id} block={block} stepId={step.id} controllerId={controllerId} onAction={onAction} />
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
function PalettePanel({ palette, controllerId, onAction }: { readonly palette: readonly PaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <div className="semio-palette flex shrink-0 flex-col gap-1 border-l border-border p-single">
      {palette.map((entry) => (
        <div
          key={entry.blockKind}
          draggable
          onDragStart={(event) => {
            event.dataTransfer.setData(PALETTE_DRAG_MIME, entry.blockKind);
            event.dataTransfer.effectAllowed = "copy";
          }}
          className="flex cursor-grab items-center gap-1 rounded border border-border p-single text-xs"
          onClick={() => dispatchBlockListAction(onAction, controllerId, "addBlock", { kind: entry.blockKind })}
        >
          <Icon icon={resolveBlockIcon(entry.iconId)} size="small" />
          {entry.label}
        </div>
      ))}
    </div>
  );
}
//#endregion Palette

//#region Component
export function BlockListHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.blockList;
  const steps = useMemo(() => {
    if (!scene) return [] as StepRecord[];
    try {
      return JSON.parse(scene.stepsJson) as StepRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const palette = useMemo(() => {
    if (!scene) return [] as PaletteEntryRecord[];
    try {
      return JSON.parse(scene.paletteJson) as PaletteEntryRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const stepIds = useMemo(() => steps.map((step) => step.id), [steps]);
  const stepsLabel = useLabel("ui.blockList.steps");
  const addStepLabel = useLabel("ui.blockList.addStep");
  const emptyLabel = useLabel("ui.host.emptyScene");

  if (!scene) return <div className="semio-block-list-empty">{emptyLabel}</div>;

  function handleStepDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = steps.findIndex((step) => step.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, node.controllerId, "moveStep", { stepId: active.id, index });
  }

  return (
    <div className="semio-block-list-host flex h-full min-h-0 w-full" data-surface-id={node.surfaceId}>
      <div className="flex min-w-0 flex-1 flex-col gap-2 overflow-auto p-single">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">{stepsLabel}</span>
          <Button className="h-medium shrink-0 px-2" icon="plus" text={addStepLabel} type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, node.controllerId, "addStep", {})} />
        </div>
        <DndContext collisionDetection={closestCenter} onDragEnd={handleStepDragEnd}>
          <SortableContext items={stepIds} strategy={verticalListSortingStrategy}>
            <div className="flex flex-col gap-2">
              {steps.map((step) => (
                <StepCard key={step.id} step={step} palette={palette} controllerId={node.controllerId} onAction={onAction} />
              ))}
            </div>
          </SortableContext>
        </DndContext>
      </div>
      <PalettePanel palette={palette} controllerId={node.controllerId} onAction={onAction} />
    </div>
  );
}
//#endregion Component
//#endregion BlockListHost
