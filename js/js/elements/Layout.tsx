// #region Header

// Layout.tsx

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
import BottomPanel, { BottomPanelProps } from "./panels/BottomPanel";
import LeftPanel, { LeftPanelProps } from "./panels/LeftPanel";
import MiddlePanel, { MiddlePanelProps } from "./panels/MiddlePanel";
import RightPanel, { RightPanelProps } from "./panels/RightPanel";

export interface LayoutProps {
  navbar?: ReactNode;
  footer?: ReactNode;
  leftPanel?: LeftPanelProps;
  middlePanel?: MiddlePanelProps;
  rightPanel?: RightPanelProps;
  bottomPanel?: BottomPanelProps;
  canvas: ReactNode;
  className?: string;
}

const Layout: FC<LayoutProps> = ({ navbar, footer, leftPanel, middlePanel, rightPanel, bottomPanel, canvas, className = "" }) => (
  <div className={`flex flex-col h-screen w-screen overflow-hidden ${className}`}>
    {navbar && <div className="flex-shrink-0">{navbar}</div>}
    <div className="flex flex-1 min-h-0">
      {leftPanel && leftPanel.visible && <LeftPanel {...leftPanel} />}
      <div className="flex flex-col flex-1 min-w-0">
        <div className="flex flex-1 min-h-0">
          {middlePanel && middlePanel.visible && <MiddlePanel {...middlePanel} />}
          <div className="flex-1 min-w-0 min-h-0">{canvas}</div>
          {rightPanel && rightPanel.visible && <RightPanel {...rightPanel} />}
        </div>
        {bottomPanel && bottomPanel.visible && <BottomPanel {...bottomPanel} />}
      </div>
    </div>
    {footer && <div className="flex-shrink-0">{footer}</div>}
  </div>
);

export default Layout;
