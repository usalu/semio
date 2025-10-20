// #region Header

// Toolbar.tsx

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

import { FC } from "react";
import { usePanelSections } from "../Navbar";
import { useActiveInteraction, useIsMobile } from "../store";

interface ToolbarProps {
  visible: boolean;
  leftOffset?: number;
  rightOffset?: number;
}

const Toolbar: FC<ToolbarProps> = ({ visible, leftOffset = 0, rightOffset = 0 }) => {
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();
  const sections = usePanelSections("toolbar");
  if (!visible) return null;
  const sortedSections = sections.sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <div
      className="z-20 bg-transparent text-foreground h-9"
      style={{
        marginLeft: `${leftOffset}px`,
        marginRight: `${rightOffset}px`,
        opacity: activeInteraction ? 0.1 : 1,
        transition: "opacity 150ms",
      }}
    >
      <div className="h-full flex items-center justify-center">
        {sortedSections.length === 0 ? (
          <div className="text-muted-foreground text-xs">No tools</div>
        ) : (
          <div className={`inline-flex items-center gap-1 ${isMobile ? "px-1" : "px-1"}`}>
            {sortedSections.map((section) => {
              const content = typeof section.content === "function" ? section.content() : section.content;
              return <div key={section.id}>{content}</div>;
            })}
          </div>
        )}
      </div>
    </div>
  );
};

export default Toolbar;
