import { useMemo } from "react";
import { Button, Icon, Input, Table, useLabel, type TableColumn } from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import type { ActionDescriptor, ComponentSceneHostProps } from "@semio-tech/framework-core";

//#region TableHost
//#region Types
type TableColumnRecord = { readonly id: string; readonly label: string; readonly sortable?: boolean };
type TableCellButton = { readonly iconId: string; readonly label?: string; readonly action: ActionDescriptor; readonly revealOnHover?: boolean };
type TableCellRecord =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "number"; readonly value: number }
  | { readonly kind: "stepper"; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly action: ActionDescriptor }
  | { readonly kind: "buttons"; readonly buttons: readonly TableCellButton[] };
type TableRowRecord = Record<string, unknown> & { readonly id?: string; readonly _drag?: Record<string, unknown> };
//#endregion Types

//#region Helpers
function isTableCellRecord(value: unknown): value is TableCellRecord {
  return typeof value === "object" && value !== null && "kind" in value;
}

function dispatchCellAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function resolveTableCellIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

function renderTableCell(cell: TableCellRecord, onAction: (action: ActionDescriptor) => void): React.ReactNode {
  switch (cell.kind) {
    case "text":
      return cell.value;
    case "number":
      return String(cell.value);
    case "stepper":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: -cell.step })} disabled={cell.value <= cell.min} type="button" variant="outline">
            −
          </Button>
          <Input className="h-medium w-14 min-w-0 text-center font-mono text-xs" readOnly value={String(cell.value)} />
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: cell.step })} disabled={cell.value >= cell.max} type="button" variant="outline">
            +
          </Button>
        </div>
      );
    case "buttons":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          {cell.buttons.map((button, index) => (
            <Button key={index} className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, button.action, {})} title={button.label} type="button" variant="outline">
              <Icon icon={resolveTableCellIcon(button.iconId)} size="small" />
            </Button>
          ))}
        </div>
      );
  }
}
//#endregion Helpers

//#region Component
export function TableHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.table;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const columns = useMemo(() => {
    if (!scene) return [] as TableColumnRecord[];
    try {
      return JSON.parse(scene.columnsJson) as TableColumnRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const rows = useMemo(() => {
    if (!scene) return [] as TableRowRecord[];
    try {
      return JSON.parse(scene.rowsJson) as TableRowRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const selectedRows = useMemo(() => {
    if (!scene?.selectionJson) return undefined;
    try {
      const parsed = JSON.parse(scene.selectionJson) as { readonly selectedIds?: readonly string[] };
      return new Set(parsed.selectedIds ?? []);
    } catch {
      return undefined;
    }
  }, [scene]);
  const sort = useMemo(() => {
    if (!scene?.sortJson) return undefined;
    try {
      return JSON.parse(scene.sortJson) as { readonly columnId?: string; readonly direction?: "asc" | "desc" };
    } catch {
      return undefined;
    }
  }, [scene]);
  const tableColumns = useMemo<TableColumn<TableRowRecord>[]>(
    () =>
      columns.map((column) => ({
        id: column.id,
        header: column.label,
        sortable: column.sortable,
        accessor: (row) => {
          const value = row[column.id];
          if (isTableCellRecord(value)) return renderTableCell(value, onAction);
          return String(value ?? "");
        },
      })),
    [columns, onAction],
  );

  if (!scene) return <div className="semio-table-empty">{emptySceneLabel}</div>;

  const rowDragMime = scene.rowDragMime;
  const dropAction = scene.dropAction;

  return (
    <div
      className="semio-table-host h-full min-h-0 w-full"
      data-surface-id={node.surfaceId}
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
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchCellAction(onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      <Table
        className="h-full w-full"
        columns={tableColumns}
        data={rows}
        emptyMessage="No rows"
        getRowId={(row, index) => String(row.id ?? row.programId ?? index)}
        selectedRows={selectedRows}
        sortColumn={sort?.columnId}
        sortDirection={sort?.direction}
        onSort={(columnId, direction) =>
          onAction({
            controllerId: node.controllerId,
            action: "sortTable",
            args: { surfaceId: node.surfaceId, columnId, direction },
          })
        }
        rowDragProps={
          rowDragMime
            ? (row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: (event) => {
                        event.dataTransfer.setData(rowDragMime, JSON.stringify(row._drag));
                        event.dataTransfer.effectAllowed = "copy";
                      },
                    }
                  : {}
            : undefined
        }
        onRowClick={(row) =>
          onAction({
            controllerId: node.controllerId,
            action: "selectRow",
            args: { surfaceId: node.surfaceId, row },
          })
        }
      />
    </div>
  );
}
//#endregion Component
//#endregion TableHost
