// #region Header

// Table.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { FC, ReactElement, ReactNode, useEffect, useRef } from "react";
import { ScrollArea } from "../aggregation/ScrollArea";

export type SortDirection = "asc" | "desc";

export interface TableColumn<T = unknown> {
  id: string;
  header: ReactNode;
  accessor: (row: T) => ReactNode;
  width?: string;
  className?: string;
  sortable?: boolean;
  visible?: boolean | ((data: T[]) => boolean);
}

export interface TableProps<T = unknown> {
  columns: TableColumn<T>[];
  data: T[];
  onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
  rowClassName?: (row: T, index: number) => string;
  rowKey?: (row: T, index: number) => string;
  emptyMessage?: string;
  className?: string;
  sortColumn?: string;
  sortDirection?: SortDirection;
  onSort?: (columnId: string, direction: SortDirection) => void;
  selectedRows?: Set<string> | string[];
  getRowId?: (row: T) => string;
  stickyHeader?: boolean;
  headerClassName?: string;
  rowHeight?: "compact" | "normal" | "comfortable";
  focusedItemId?: string;
  onFocusComplete?: () => void;
}

const Table = <T,>({
  columns,
  data,
  onRowClick,
  onRowDoubleClick,
  rowClassName,
  rowKey,
  emptyMessage = "No data",
  className = "",
  sortColumn,
  sortDirection,
  onSort,
  selectedRows,
  getRowId,
  stickyHeader = true,
  headerClassName = "",
  rowHeight = "normal",
  focusedItemId,
  onFocusComplete,
}: TableProps<T>) => {
  const selectedSet = selectedRows instanceof Set ? selectedRows : new Set(selectedRows || []);
  const scrollAreaRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const rowElements = scrollAreaRef.current.querySelectorAll("tbody tr");
      let focusedIndex = -1;

      data.forEach((row, index) => {
        const rowId = getRowId ? getRowId(row) : rowKey ? rowKey(row, index) : index.toString();
        if (rowId === focusedItemId) {
          focusedIndex = index;
        }
      });

      if (focusedIndex >= 0 && rowElements[focusedIndex]) {
        rowElements[focusedIndex].scrollIntoView({ behavior: "smooth", block: "center" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, data, getRowId, rowKey, onFocusComplete]);

  const rowHeightClass = {
    compact: "py-1",
    normal: "py-1.5",
    comfortable: "py-2",
  }[rowHeight];

  const visibleColumns = columns.filter((col) => {
    if (col.visible === undefined) return true;
    if (typeof col.visible === "boolean") return col.visible;
    return col.visible(data);
  });

  return (
    <ScrollArea ref={scrollAreaRef} className={`h-full w-full ${className}`}>
      <table className="w-full border-collapse">
        <thead className={`bg-base border-b ${stickyHeader ? "sticky top-0 z-10" : ""} ${headerClassName}`}>
          <tr>
            {visibleColumns.map((column) => (
              <th key={column.id} className={`text-left p-1 font-medium ${rowHeightClass} ${column.className || ""}`} style={{ width: column.width }}>
                {column.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.length === 0 ? (
            <tr>
              <td colSpan={visibleColumns.length} className="p-4 text-center text-muted-foreground">
                {emptyMessage}
              </td>
            </tr>
          ) : (
            data.map((row, index) => {
              const key = rowKey ? rowKey(row, index) : index.toString();
              const rowId = getRowId ? getRowId(row) : key;
              const isSelected = selectedSet.has(rowId);
              const baseRowClassName = `border-b ${rowHeightClass} ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`;
              const customRowClassName = rowClassName ? rowClassName(row, index) : "";

              return (
                <tr
                  key={key}
                  className={`${baseRowClassName} ${customRowClassName} ${onRowClick ? "cursor-selectable" : ""}`}
                  onClick={(e) => onRowClick?.(row, index, e)}
                  onDoubleClick={() => onRowDoubleClick?.(row, index)}
                  role={onRowClick ? "button" : undefined}
                  tabIndex={onRowClick ? 0 : undefined}
                >
                  {visibleColumns.map((column) => (
                    <td key={column.id} className={`p-1 ${column.className || ""}`}>
                      {column.accessor(row)}
                    </td>
                  ))}
                </tr>
              );
            })
          )}
        </tbody>
      </table>
    </ScrollArea>
  );
};

export default Table as <T>(props: TableProps<T>) => ReactElement;

export interface TableSkeletonProps {
  columns: TableColumn[];
  rowCount?: number;
  className?: string;
}

export const TableSkeleton: FC<TableSkeletonProps> = ({ columns, rowCount = 5, className = "" }) => (
  <ScrollArea className={`h-full w-full ${className}`}>
    <table className="w-full border-collapse">
      <thead className="bg-panel border-b sticky top-0 z-10">
        <tr>
          {columns.map((column) => (
            <th key={column.id} className={`text-left p-2 text-sm font-medium ${column.className || ""}`} style={{ width: column.width }}>
              {column.header}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {Array.from({ length: rowCount }).map((_, index) => (
          <tr key={index} className="border-b">
            {columns.map((column) => (
              <td key={column.id} className={`p-2 text-sm ${column.className || ""}`}>
                <div className="h-4 bg-muted-foreground/20 rounded animate-pulse" />
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  </ScrollArea>
);
