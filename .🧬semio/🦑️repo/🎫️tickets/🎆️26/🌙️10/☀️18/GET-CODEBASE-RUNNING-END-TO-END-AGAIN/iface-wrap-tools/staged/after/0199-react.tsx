/** 🗄️ The virtual-file-system surface host: its navigation action and the host that renders the file tree.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { useContext, useMemo, useState } from "../../⚛️react-port/🟦️.ts";
import {
  ContextMenuController,
  VirtualFileSystem,
  buildVirtualFileSystemSceneRows,
  useLabel,
  type VirtualFileSystemNode
} from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
import { useMapContextMenuSpecs, openSurfaceContextMenu, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🖱️surface-menu/🟦️.ts";
import { parseSceneJsonField, wireLabel, WindowInstanceIdContext } from "../🌉️scene-host-contract/🟦️.ts";

//#region 🔖️UiInterpreter
//#region VirtualFileSystemHost
export function virtualFileSystemNavigation(row: Pick<VirtualFileSystemNode, "navigateUri">): { readonly action: string; readonly args: Readonly<Record<string, string>> } | null {
  const uri = row.navigateUri;
  if (!uri) return null;
  if (uri.startsWith("os://instance/")) {
    const instanceId = uri.slice("os://instance/".length);
    return instanceId ? { action: "openInstance", args: { instanceId } } : null;
  }
  if (uri.startsWith("os://export/")) {
    const [instanceId, format, ...rest] = uri.slice("os://export/".length).split("/");
    return instanceId && format && rest.length === 0 ? { action: "exportMedia", args: { instanceId, format } } : null;
  }
  if (uri.startsWith("/spaces/")) {
    const [spaceId, ...rest] = uri.slice("/spaces/".length).split("/");
    return spaceId && rest.length === 0 ? { action: "navigateVirtualFileSystemNode", args: { spaceId } } : null;
  }
  if (uri.startsWith("studio:")) {
    const spaceId = uri.slice("studio:".length);
    return spaceId ? { action: "navigateVirtualFileSystemNode", args: { spaceId } } : null;
  }
  return null;
}

export function VirtualFileSystemHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.virtualFileSystem;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.file");
  const dispatch = (action: string, args?: Record<string, unknown>): void => {
    onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
  };
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const rawRows = useMemo(() => scene ? parseSceneJsonField<VirtualFileSystemNode[]>(scene.rowsJson) : [], [scene?.rowsJson]);
  const [expandedRowIds, setExpandedRowIds] = useState<ReadonlySet<string>>(() => new Set(rawRows.filter((row) => row.hasChildren).map((row) => row.id)));
  const rows = useMemo(() => buildVirtualFileSystemSceneRows(rawRows, expandedRowIds), [expandedRowIds, rawRows]);
  if (!scene) return <div className="semio-vfs-empty">{emptySceneLabel}</div>;
  const schema = parseSceneJsonField<Parameters<typeof VirtualFileSystem>[0]["schema"]>(scene.schemaJson);
  const selectedRowIds = scene.selectedRowIdsJson ? parseSceneJsonField<string[]>(scene.selectedRowIdsJson) : undefined;
  return (
    <>
      <VirtualFileSystem
        className="min-h-0 flex-1"
        schema={schema}
        rows={rows}
        selectedRowIds={selectedRowIds}
        emptyMessage={scene.emptyMessage !== undefined ? wireLabel(scene.emptyMessage) : undefined}
        dragDrop={scene.dragDropEnabled ? { enabled: true } : undefined}
        onToggleExpand={(rowId) => setExpandedRowIds((current) => {
          const next = new Set(current);
          if (next.has(rowId)) next.delete(rowId);
          else next.add(rowId);
          return next;
        })}
        onRowDoubleClick={(row) => {
          const navigation = virtualFileSystemNavigation(row);
          if (navigation) dispatch(navigation.action, { ...navigation.args });
        }}
        onSelectionChange={(ids) => onAction({ controllerId: node.controllerId, action: "selectRows", args: { surfaceId: node.surfaceId, ids } })}
        onRowContextMenu={(row, index, event) => {
          if (!requestContextMenu) return;
          event.preventDefault();
          event.stopPropagation();
          const rowId = String(row.id ?? index);
          void (async () => {
            const menu = await openSurfaceContextMenu(
              requestContextMenu,
              {
                menu: { id: "virtualFileSystem", args: null },
                surface: { surfaceId: node.surfaceId, kind: "virtualFileSystem", hits: [{ domain: "row", id: rowId }], selection: selectedRowIds && selectedRowIds.length > 0 ? [{ domain: "row", ids: selectedRowIds }] : [] },
                windowInstanceId: windowInstanceId ?? undefined,
                point: { x: event.clientX, y: event.clientY },
              },
              mapContextMenu,
              shellContextMenuFallback,
            );
            setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
          })();
        }}
      />
      <ContextMenuController title={contextMenuTitleLabel} open={contextMenu != null} position={contextMenu ?? { x: 0, y: 0 }} items={contextMenu?.items ?? []} onOpenChange={(open) => { if (!open) setContextMenu(null); }} />
    </>
  );
}
//#endregion VirtualFileSystemHost
//#endregion 🔖️UiInterpreter
