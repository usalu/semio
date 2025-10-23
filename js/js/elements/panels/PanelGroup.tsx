// #region Header

// PanelGroup.tsx

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

export interface PanelGroupProps {
  children: ReactNode;
  className?: string;
  position?: "left" | "right" | "middle" | "bottom";
}

const PanelGroup: FC<PanelGroupProps> = ({ children, className = "", position = "middle" }) => {
  const baseClass = "flex";
  const positionClass =
    position === "left" || position === "right" || position === "middle"
      ? "flex-col"
      : "flex-row";
  return (
    <div className={`${baseClass} ${positionClass} ${className}`}>
      {children}
    </div>
  );
};

export default PanelGroup;
