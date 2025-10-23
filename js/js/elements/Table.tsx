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

import { ReactElement, ReactNode } from "react";
import { ScrollArea } from "./aggregation/ScrollArea";

export interface TableColumn<T = unknown> {
  id: string;
  header: ReactNode;
  accessor: (row: T) => ReactNode;
  width?: string;
  className?: string;
}

export interface TableProps<T = unknown> {
  columns: TableColumn<T>[];
  data: T[];
  onRowClick?: (row: T, index: number) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
  rowClassName?: (row: T, index: number) => string;
  emptyMessage?: string;
  className?: string;
}

const Table = <T,>({ columns, data, onRowClick, onRowDoubleClick, rowClassName, emptyMessage = "No data", className = "" }: TableProps<T>) => (
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
        {data.length === 0 ? (
          <tr>
            <td colSpan={columns.length} className="p-4 text-center text-muted-foreground">
              {emptyMessage}
            </td>
          </tr>
        ) : (
          data.map((row, index) => (
            <tr key={index} className={`border-b hover:bg-panel cursor-pointer ${rowClassName ? rowClassName(row, index) : ""}`} onClick={() => onRowClick?.(row, index)} onDoubleClick={() => onRowDoubleClick?.(row, index)}>
              {columns.map((column) => (
                <td key={column.id} className={`p-2 text-sm ${column.className || ""}`}>
                  {column.accessor(row)}
                </td>
              ))}
            </tr>
          ))
        )}
      </tbody>
    </table>
  </ScrollArea>
);

export default Table as <T>(props: TableProps<T>) => ReactElement;
