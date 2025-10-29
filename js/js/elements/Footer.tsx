// #region Header

// Footer.tsx

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
import { Tooltip, TooltipContent, TooltipTrigger } from "./display/Tooltip";

export interface FooterItem {
  id: string;
  content: ReactNode;
  i18n?: string;
  order?: number;
  onClick?: () => void;
  className?: string;
}

export interface FooterProps {
  items?: FooterItem[];
  className?: string;
  height?: number;
  isVisible?: boolean;
}

const Footer: FC<FooterProps> = ({ items = [], className = "", height = 20, isVisible = true }) => {
  const sortedItems = [...items].sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <footer className={`bg-base border-t flex items-center transition-transform duration-200 ${isVisible ? "translate-y-0" : "translate-y-full"} ${className}`} style={{ height: `${height}px` }}>
      {sortedItems.map((item, index) => (
        <div key={item.id} className="flex items-center h-full">
          {index > 0 && <div className="h-full w-px bg-border" />}
          {item.i18n ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <div className={`flex items-center h-full px-2 text-xs cursor-pointer ${item.className || ""}`} onClick={item.onClick}>
                  {item.content}
                </div>
              </TooltipTrigger>
              <TooltipContent>{item.i18n}</TooltipContent>
            </Tooltip>
          ) : (
            <div className={`flex items-center h-full px-2 text-xs ${item.onClick ? "cursor-pointer" : ""} ${item.className || ""}`} onClick={item.onClick}>
              {item.content}
            </div>
          )}
        </div>
      ))}
    </footer>
  );
};

export default Footer;
