// #region Header

// Navbar.tsx

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

export interface NavbarItem {
  id: string;
  content: ReactNode;
  onClick?: () => void;
  className?: string;
  order?: number;
}

export interface NavbarProps {
  leftItems?: NavbarItem[];
  centerItems?: NavbarItem[];
  rightItems?: NavbarItem[];
  className?: string;
  height?: number;
  isExpanded?: boolean;
}

const Navbar: FC<NavbarProps> = ({ leftItems = [], centerItems = [], rightItems = [], className = "", height = 48, isExpanded = false }) => {
  const sortedLeft = [...leftItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  const sortedCenter = [...centerItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  const sortedRight = [...rightItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <nav className={`bg-base border-b flex items-center justify-between px-2 ${className}`} style={{ height: `${height}px`, transition: "height 150ms" }}>
      <div className="flex items-center gap-1 min-w-0">
        {sortedLeft.map((item) => (
          <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
            {item.content}
          </div>
        ))}
      </div>
      <div className="flex items-center gap-1 min-w-0 justify-center flex-1">
        {sortedCenter.map((item) => (
          <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
            {item.content}
          </div>
        ))}
      </div>
      <div className="flex items-center gap-1 min-w-0">
        {sortedRight.map((item) => (
          <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
            {item.content}
          </div>
        ))}
      </div>
    </nav>
  );
};

export default Navbar;
