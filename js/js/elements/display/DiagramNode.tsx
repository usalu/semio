// #region Header

// DiagramNode.tsx

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

import { FC, ReactNode } from "react";
import { Handle, Position } from "@xyflow/react";

export interface DiagramNodeProps {
  /** Content to display inside the node */
  content: ReactNode;
  /** Whether the node is selected */
  selected?: boolean;
  /** Whether the node is hovered */
  hovered?: boolean;
  /** Whether the node is a placeholder (dashed border) */
  isPlaceholder?: boolean;
  /** Whether to show top handle for incoming connections */
  showTopHandle?: boolean;
  /** Whether to show bottom handle for outgoing connections */
  showBottomHandle?: boolean;
  /** Custom className */
  className?: string;
  /** Mouse enter handler */
  onMouseEnter?: () => void;
  /** Mouse leave handler */
  onMouseLeave?: () => void;
  /** Click handler */
  onClick?: () => void;
}

/**
 * Generalized diagram node component with consistent styling
 * All diagram nodes are circular with borders, consistent hover/selection colors and cursors
 */
export const DiagramNode: FC<DiagramNodeProps> = ({ content, selected = false, hovered = false, isPlaceholder = false, showTopHandle = false, showBottomHandle = false, className = "", onMouseEnter, onMouseLeave, onClick }) => {
  return (
    <div
      className={`
        relative flex items-center justify-center
        w-12 h-12 rounded-full
        ${isPlaceholder ? "border-2 border-dashed" : "border-2 border-solid"}
        ${selected ? "ring-2 ring-[color:var(--active-base)]" : ""}
        ${hovered ? "ring-2 ring-[color:var(--hover-base)]" : ""}
        ${isPlaceholder ? "border-[color:var(--disabled-base)] bg-[color:var(--disabled-panel)]" : "border-[color:var(--foreground-panel)] bg-[color:var(--background-panel)]"}
        transition-all duration-150
        ${onClick ? "cursor-selectable" : "cursor-default"}
        ${className}
      `}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      onClick={onClick}
    >
      {showTopHandle && <Handle type="target" position={Position.Top} className="w-2 h-2 !bg-[color:var(--foreground-panel)] !border-[color:var(--background-panel)]" />}

      <div className="text-sm font-medium text-[color:var(--foreground-panel)] truncate px-2">{content}</div>

      {showBottomHandle && <Handle type="source" position={Position.Bottom} className="w-2 h-2 !bg-[color:var(--foreground-panel)] !border-[color:var(--background-panel)]" />}
    </div>
  );
};

/**
 * Placeholder node for showing drop targets
 */
export const PlaceholderDiagramNode: FC<{ label?: string; onClick?: () => void }> = ({ label = "+ Drop here", onClick }) => (
  <DiagramNode content={label} isPlaceholder showTopHandle onClick={onClick} className="hover:border-[color:var(--hover-base)] hover:bg-[color:var(--hover-panel)]" />
);
