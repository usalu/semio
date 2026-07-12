import { lazy, Suspense, type ReactElement, type ReactNode } from "react";
import {
  Button,
  ChromeAwareWindowScrollSurface,
  Field,
  Icon,
  IconSelector,
  Input,
  Ring,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Section,
  Slider,
  Stepper,
  Textarea,
  Toggle,
  Tree,
  VirtualFileSystem,
  catalogueTreeDragController,
  classifyIconSelectorMode,
  cn,
  renderControlIcon,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig,
} from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import type { ActionDescriptor, UiControlNode, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode } from "./os-shell.tsx";

const Canvas2dHost = lazy(() => import("./components/canvas-2d-host.tsx").then((module) => ({ default: module.Canvas2dHost })));
const NodeGraphHost = lazy(() => import("./components/node-graph-host.tsx").then((module) => ({ default: module.NodeGraphHost })));
const RasterHost = lazy(() => import("./components/raster-host.tsx").then((module) => ({ default: module.RasterHost })));
const TableHost = lazy(() => import("./components/table-host.tsx").then((module) => ({ default: module.TableHost })));
const TextEditorHost = lazy(() => import("./components/text-editor-host.tsx").then((module) => ({ default: module.TextEditorHost })));
const World3dHost = lazy(() => import("./components/world-3d-host.tsx").then((module) => ({ default: module.World3dHost })));
const GisMapHost = lazy(() => import("./components/gis-map-host.tsx").then((module) => ({ default: module.GisMapHost })));
const Puzzle2dBoardHost = lazy(() => import("./components/puzzle-2d-board-host.tsx").then((module) => ({ default: module.Puzzle2dBoardHost })));
const IconRenderHost = lazy(() => import("./components/icon-render-host.tsx").then((module) => ({ default: module.IconRenderHost })));
const NoteCanvasHost = lazy(() => import("./components/note-canvas-host.tsx").then((module) => ({ default: module.NoteCanvasHost })));

function ComponentSceneFallback() {
  return <p className="text-muted-foreground p-2 text-xs">Loading surface…</p>;
}

function renderComponentSceneHost(node: Extract<UiNode, { type: "componentScene" }>, onAction: (action: ActionDescriptor) => void): ReactNode {
  const host = (() => {
    switch (node.componentKind) {
      case "canvas-2d":
        return <Canvas2dHost node={node} onAction={onAction} />;
      case "world-3d":
        return <World3dHost node={node} onAction={onAction} />;
      case "node-graph":
        return <NodeGraphHost node={node} onAction={onAction} />;
      case "text-editor":
        return <TextEditorHost node={node} onAction={onAction} />;
      case "table":
        return <TableHost node={node} onAction={onAction} />;
      case "raster":
        return <RasterHost node={node} onAction={onAction} />;
      case "gis2d-map":
        return <GisMapHost node={node} onAction={onAction} />;
      case "puzzle2d-board":
        return <Puzzle2dBoardHost node={node} onAction={onAction} />;
      case "icon-render":
        return <IconRenderHost node={node} onAction={onAction} />;
      case "note-canvas":
        return <NoteCanvasHost node={node} onAction={onAction} />;
      case "virtualFileSystem":
        return <VirtualFileSystemHost node={node} onAction={onAction} />;
      default:
        return <p className="text-muted-foreground text-xs">Unknown component: {node.componentKind}</p>;
    }
  })();
  return <Suspense fallback={<ComponentSceneFallback />}>{host}</Suspense>;
}

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly onAction: (action: ActionDescriptor) => void;
};
//#endregion UiInterpreterContext

//#region ActionDispatch
function dispatchUiAction(onAction: UiInterpreterContext["onAction"], descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function resolveDeclarativeControlIcon(iconId: string, size: number | "tiny" | "small" | "base" | "large" = "small"): ReactNode {
  const iconName = iconId in ICONS ? (iconId as IconName) : "circle-dot";
  return <Icon icon={iconName} size={size} />;
}
//#endregion ActionDispatch

//#region RenderUiControl
/** @emoji 🎛 Renders a declarative control node with ui-react primitives. */
export function renderUiControl(control: UiControlNode, onAction: UiInterpreterContext["onAction"]): ReactElement {
  switch (control.type) {
    case "input": {
      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => {
        const value = control.inputKind === "number" ? Number(raw) : raw;
        dispatchUiAction(onAction, control.onChange, { value });
      };
      if (control.inputKind === "longText") {
        return (
          <Textarea
            id={control.id}
            className="min-h-[4.5rem] w-full min-w-0"
            value={control.value}
            placeholder={control.placeholder}
            onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)}
            onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
          />
        );
      }
      const inputType = control.inputKind === "number" ? "number" : control.inputKind === "date" ? "date" : control.inputKind === "color" ? "color" : control.inputKind === "file" ? "file" : "text";
      return (
        <Input
          id={control.id}
          type={inputType}
          className="h-medium w-full min-w-0"
          value={control.inputKind === "file" ? undefined : control.value}
          placeholder={control.placeholder}
          min={control.min}
          max={control.max}
          step={control.step}
          accept={control.inputKind === "file" ? control.accept : undefined}
          onChange={
            commitOnBlur
              ? undefined
              : (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
          }
          onBlur={
            commitOnBlur
              ? (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
              : undefined
          }
        />
      );
    }
    case "select":
      return (
        <Select value={control.value || undefined} onValueChange={(value) => dispatchUiAction(onAction, control.onChange, { value })}>
          <SelectTrigger id={control.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={control.placeholder ?? "Select"} />
          </SelectTrigger>
          <SelectContent>
            {control.items.map((item, index) => (
              <SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "toggle":
      return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveDeclarativeControlIcon(control.iconId)} onPressedChange={(pressed) => dispatchUiAction(onAction, control.onChange, { pressed })} />;
    case "vec3": {
      const tuple = control.value;
      const mixed = tuple == null || !Array.isArray(tuple) || tuple.length < 3;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-1">
          {axes.map((axis, index) => (
            <Input
              key={`${control.id}.${axis}`}
              id={`${control.id}.${axis}`}
              type="number"
              className="h-medium w-full min-w-0"
              value={mixed ? "" : String(tuple[index] ?? 0)}
              placeholder={mixed ? "—" : axis}
              disabled={mixed}
              onChange={(event) => {
                if (mixed) return;
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0];
                next[index] = parsed;
                dispatchUiAction(onAction, control.onChange, { value: next });
              }}
            />
          ))}
        </div>
      );
    }
    case "keyValue":
      return (
        <dl className="grid grid-cols-[auto_1fr] gap-x-2 gap-y-1 text-xs">
          {control.entries.map((entry) => (
            <div key={entry.label} className="contents">
              <dt className="text-muted-foreground">{entry.label}</dt>
              <dd className="tabular-nums">{entry.value}</dd>
            </div>
          ))}
        </dl>
      );
    case "slider": {
      const slider = (
        <Slider
          id={control.id}
          className="w-full min-w-0"
          max={control.max}
          min={control.min}
          step={control.step}
          value={[control.value]}
          onValueChange={(values) => dispatchUiAction(onAction, control.onChange, { value: values[0] ?? control.value })}
        />
      );
      if (!control.unit) return slider;
      return (
        <div className="flex min-w-0 w-full items-center gap-single">
          {slider}
          <span className="text-muted-foreground shrink-0 text-xs tabular-nums">
            {control.value} {control.unit}
          </span>
        </div>
      );
    }
    case "numberStepper":
      return (
        <div className="flex min-w-0 w-full items-center gap-1">
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchUiAction(onAction, control.onDelta, { delta: -control.step })} type="button" variant="outline">
            −
          </Button>
          <Input
            className="h-medium min-w-0 flex-1 font-mono text-xs"
            id={control.id}
            onChange={(event) => {
              const parsed = Number(event.target.value);
              if (Number.isFinite(parsed)) {
                dispatchUiAction(onAction, control.onAbsolute, { value: parsed });
              }
            }}
            placeholder={control.uniform ? undefined : "Mixed"}
            type="number"
            value={control.uniform && Number.isFinite(control.value) ? String(control.value) : ""}
          />
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchUiAction(onAction, control.onDelta, { delta: control.step })} type="button" variant="outline">
            +
          </Button>
        </div>
      );
    case "ring":
      return <Ring id={control.id} onOrbChange={(_orbId, _oldT, newT) => dispatchUiAction(onAction, control.onChange, { t: newT })} orbs={[{ disabled: control.disabled, id: control.orbId, selected: true, t: control.t }]} />;
    case "iconSelect":
      return (
        <IconSelector
          classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
          id={control.id}
          onChange={(next) => dispatchUiAction(onAction, control.onChange, { value: next })}
          uniform={control.uniform}
          value={control.value}
        />
      );
    case "button":
      return <Button id={control.id} text={control.label} icon={resolveDeclarativeControlIcon(control.iconId)} disabled={control.disabled} onClick={() => onAction(control.action)} />;
  }
}
//#endregion RenderUiControl

//#region UiTreePanel
function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], onAction: UiInterpreterContext["onAction"]): TreeDataItem[] {
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    description: item.description,
    icon: item.iconId ? renderControlIcon(item.iconId, 12) : undefined,
    control: item.control ? renderUiControl(item.control, onAction) : undefined,
    defaultOpen: item.defaultOpen,
    isSelected: item.selected,
    isHidden: item.isHidden,
    draggable: item.draggable,
    dragData: item.dragData,
    className: item.draggable || item.dragData ? "cursor-grab active:cursor-grabbing" : undefined,
    items: item.items?.length ? uiTreeItemsToTreeData(item.items, onAction) : undefined,
    onClick: item.action ? () => dispatchUiAction(onAction, item.action!, {}) : undefined,
    onPointerEnter: item.hoverAction ? () => dispatchUiAction(onAction, item.hoverAction!, {}) : undefined,
    onPointerLeave: item.unhoverAction ? () => dispatchUiAction(onAction, item.unhoverAction!, {}) : undefined,
    actions: item.actions?.map((action) => ({
      kind: "button" as const,
      icon: renderControlIcon(action.iconId, 12),
      title: action.label,
      revealOnHover: action.revealOnHover,
      onClick: () => dispatchUiAction(onAction, action.action, {}),
    })),
  }));
}

/** @emoji 🌲 Maps a declarative {@link UiTreeNode} to a {@link TreePanelConfig}. */
export function uiTreeNodeToTreePanelConfig(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreePanelConfig {
  const sections: TreeDataSection[] = treeNode.sections.map((section: UiTreeSectionNode) => ({
    id: section.id,
    label: section.label ?? "",
    defaultOpen: section.defaultOpen,
    items: uiTreeItemsToTreeData(section.items, onAction),
  }));
  return {
    sections,
    selectedIds: treeNode.selectedIds as string[] | undefined,
    highlightedIds: treeNode.highlightedIds,
    onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchUiAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
  };
}

function treeDragPayloadMime(treeNode: UiTreeNode): string | undefined {
  for (const section of treeNode.sections) {
    const visit = (items: readonly UiTreeItemNode[]): string | undefined => {
      for (const item of items) {
        const mime = item.dragData ? Object.keys(item.dragData)[0] : undefined;
        if (mime) return mime;
        const nested = item.items?.length ? visit(item.items) : undefined;
        if (nested) return nested;
      }
      return undefined;
    };
    const mime = visit(section.items);
    if (mime) return mime;
  }
  return undefined;
}

/** @emoji 🖱️ Builds the drag/drop controller for a declarative tree — palette source when items carry drag payloads, drop dispatch when the tree declares a drop action. */
export function declarativeTreeDragController(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreeDragAndDropController | undefined {
  const mime = treeDragPayloadMime(treeNode);
  const source = mime ? catalogueTreeDragController(mime) : undefined;
  const dropAction = treeNode.dropAction;
  if (!dropAction) return source;
  return {
    ...(source ?? {}),
    handleDrop: ({ data, target, dropPosition }) => {
      const encoded = Object.entries(data).find(([kind, value]) => kind.startsWith("application/x-semio-") && value.trim())?.[1];
      if (!encoded) return;
      let payload: Record<string, unknown>;
      try {
        payload = JSON.parse(encoded) as Record<string, unknown>;
      } catch {
        return;
      }
      dispatchUiAction(onAction, dropAction, { ...payload, targetId: target.id, dropPosition: dropPosition ?? "inside" });
    },
  };
}

function DeclarativeTreePanel({ treeNode, onAction }: { readonly treeNode: UiTreeNode; readonly onAction: UiInterpreterContext["onAction"] }) {
  const config = uiTreeNodeToTreePanelConfig(treeNode, onAction);
  const dragController = declarativeTreeDragController(treeNode, onAction);
  return (
    <Tree
      className="min-h-0 min-w-0 flex-1 overflow-auto"
      sections={config.sections}
      selectionMode={config.selectedIds?.length ? "multiple" : "single"}
      showLines
      selectedIds={config.selectedIds}
      highlightedIds={config.highlightedIds}
      onSelectionChange={config.onSelectionChange}
      dragAndDropController={dragController}
    />
  );
}
//#endregion UiTreePanel

//#region VirtualFileSystemHost
function VirtualFileSystemHost({ node, onAction }: { readonly node: Extract<UiNode, { type: "componentScene" }>; readonly onAction: (action: ActionDescriptor) => void }) {
  const scene = node.virtualFileSystem;
  if (!scene) return <div className="semio-vfs-empty">No virtual file system scene</div>;
  const schema = JSON.parse(scene.schemaJson) as Parameters<typeof VirtualFileSystem>[0]["schema"];
  const rows = JSON.parse(scene.rowsJson) as Parameters<typeof VirtualFileSystem>[0]["rows"];
  const selectedRowIds = scene.selectedRowIdsJson ? (JSON.parse(scene.selectedRowIdsJson) as string[]) : undefined;
  return (
    <VirtualFileSystem
      className="min-h-0 flex-1"
      schema={schema}
      rows={rows}
      selectedRowIds={selectedRowIds}
      emptyMessage={scene.emptyMessage}
      dragDrop={scene.dragDropEnabled ? { enabled: true } : undefined}
      onSelectionChange={(ids) =>
        onAction({
          controllerId: node.controllerId,
          action: "selectRows",
          args: { surfaceId: node.surfaceId, ids },
        })
      }
      onRowDoubleClick={(row) => {
        const uri = row.navigateUri;
        if (!uri) return;
        if (uri.startsWith("os://instance/")) {
          onAction({
            controllerId: node.controllerId,
            action: "openInstance",
            args: { surfaceId: node.surfaceId, instanceId: uri.slice("os://instance/".length) },
          });
          return;
        }
        if (uri.startsWith("os://export/")) {
          const [, , , instanceId, , format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "exportMedia",
              args: { surfaceId: node.surfaceId, instanceId, format },
            });
          }
          return;
        }
        if (uri.startsWith("os://import/")) {
          const [, , , instanceId, resourceKind, format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "importMedia",
              args: { surfaceId: node.surfaceId, instanceId, resourceKind, format },
            });
          }
          return;
        }
        if (uri.startsWith("/studios/")) {
          const studioId = uri.split("/")[2];
          if (studioId) {
            onAction({
              controllerId: node.controllerId,
              action: "navigateVirtualFileSystemNode",
              args: { surfaceId: node.surfaceId, studioId },
            });
          }
          return;
        }
        if (uri.startsWith("studio:")) {
          onAction({
            controllerId: node.controllerId,
            action: "navigateVirtualFileSystemNode",
            args: { surfaceId: node.surfaceId, studioId: uri.slice("studio:".length) },
          });
        }
      }}
    />
  );
}
//#endregion VirtualFileSystemHost

//#region InterpretUiNode
function uiNodeKey(node: UiNode, index: number): string {
  if ("id" in node && typeof node.id === "string" && node.id) return node.id;
  return `${node.type}:${index}`;
}

/** @emoji 🌳 Interprets a declarative {@link UiNode} tree into ui-react components. */
export function interpretUiNode(node: UiNode, context: UiInterpreterContext): ReactNode {
  switch (node.type) {
    case "stack": {
      const activate = node.activate;
      const dropAction = node.dropAction;
      return (
        <div
          className={cn(
            `semio-ui-stack semio-ui-stack--${node.direction}`,
            activate && "border-border bg-panel cursor-pointer rounded-md border",
            node.selected && "ring-primary border-primary ring-1",
          )}
          data-ui-stack={node.id}
          role={activate ? "button" : undefined}
          onClick={
            activate
              ? (event) => {
                  event.stopPropagation();
                  dispatchUiAction(context.onAction, activate, {});
                }
              : undefined
          }
          onDragOver={
            dropAction
              ? (event) => {
                  event.preventDefault();
                  event.dataTransfer.dropEffect = "copy";
                }
              : undefined
          }
          onDrop={
            dropAction
              ? (event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
                  if (!encoded?.trim()) return;
                  try {
                    dispatchUiAction(context.onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
                  } catch {
                    return;
                  }
                }
              : undefined
          }
          style={{
            display: "flex",
            flexDirection: node.direction === "horizontal" ? "row" : "column",
            gap: node.gap === "none" ? 0 : node.gap === "tight" ? "0.25rem" : node.gap === "relaxed" ? "1rem" : "0.5rem",
            padding: node.padding === "none" ? 0 : "0.5rem",
            minHeight: 0,
            minWidth: 0,
            flex: 1,
          }}
        >
          {node.children.map((child, index) => (
            <div key={uiNodeKey(child, index)} className="min-h-0 min-w-0 flex-1">
              {interpretUiNode(child, context)}
            </div>
          ))}
        </div>
      );
    }
    case "text":
      return <p className={node.emphasize ? "font-semibold" : "text-sm"}>{node.value}</p>;
    case "button":
      return <Button id={node.id} text={node.label} icon={resolveDeclarativeControlIcon(node.iconId)} disabled={node.disabled} onClick={() => context.onAction(node.action)} />;
    case "separator":
      return <hr className="border-border" />;
    case "image":
      return <img id={node.id} src={node.src} alt={node.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-image={node.id} />;
    case "input":
      return renderUiControl(node, context.onAction);
    case "select":
      return renderUiControl(node, context.onAction);
    case "toggle":
      return renderUiControl(node, context.onAction);
    case "vec3":
      return renderUiControl(node, context.onAction);
    case "keyValue":
      return renderUiControl(node, context.onAction);
    case "slider":
      return renderUiControl(node, context.onAction);
    case "numberStepper":
      return renderUiControl(node, context.onAction);
    case "ring":
      return renderUiControl(node, context.onAction);
    case "iconSelect":
      return renderUiControl(node, context.onAction);
    case "field":
      return (
        <Field id={node.id} label={node.label} description={node.description} required={node.required} error={node.error}>
          {interpretUiNode(node.child, context)}
        </Field>
      );
    case "section":
      return (
        <Section id={node.id} title={node.label}>
          {node.children.map((child, index) => (
            <div key={uiNodeKey(child, index)}>{interpretUiNode(child, context)}</div>
          ))}
        </Section>
      );
    case "tree":
      return <DeclarativeTreePanel treeNode={node} onAction={context.onAction} />;
    case "componentScene":
      return renderComponentSceneHost(node, context.onAction);
    case "externalSlot":
      return <p className="text-muted-foreground text-xs">Extension unavailable: {node.pluginId}</p>;
  }
}
//#endregion InterpretUiNode
