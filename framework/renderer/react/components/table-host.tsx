import { useMemo } from "react";
import { Table, type TableColumn } from "@semio-tech/ui-react";
import type { ActionDescriptor, UiComponentSceneNode } from "../os-shell.tsx";

//#region TableHost
type TableColumnRecord = { readonly id: string; readonly label: string };
type TableRowRecord = Record<string, unknown>;

export function TableHost({ node, onAction }: { readonly node: UiComponentSceneNode; readonly onAction: (action: ActionDescriptor) => void }) {
  const scene = node.table;
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
  const tableColumns = useMemo<TableColumn<TableRowRecord>[]>(
    () =>
      columns.map((column) => ({
        id: column.id,
        header: column.label,
        accessor: (row) => String(row[column.id] ?? ""),
      })),
    [columns],
  );

  if (!scene) return <div className="semio-table-empty">No table scene</div>;

  return (
    <div className="semio-table-host h-full min-h-0 w-full" data-surface-id={node.surfaceId}>
      <Table
        className="h-full w-full"
        columns={tableColumns}
        data={rows}
        emptyMessage="No rows"
        getRowId={(row, index) => String(row.id ?? row.programId ?? index)}
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
//#endregion TableHost
