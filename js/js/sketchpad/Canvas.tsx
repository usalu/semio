// #region Header

// Canvas.tsx

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

import { FC, Fragment, memo, ReactNode, useMemo } from "react";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../elements/aggregation/Resizable";
import BaseCanvas, { useCanvasContext as useBaseCanvasContext } from "../elements/Canvas";
import BaseWindow, { WindowConfig as BaseWindowConfig } from "../elements/windows/Window";
import GoldenLayoutCanvas from "./GoldenLayoutCanvas";

export { GoldenLayoutCanvas };

export enum WindowLayout {
  SINGLE = "single",
  HORIZONTAL_SPLIT = "horizontal-split",
  VERTICAL_SPLIT = "vertical-split",
  GRID = "grid",
}

export interface WindowConfig extends BaseWindowConfig {
  defaultSize?: number;
}

export interface WindowTypeDefinition {
  id: string;
  label: string;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
}

export interface AppWindowConfig {
  windowTypes: WindowTypeDefinition[];
  defaultLayout: any;
}

export const Canvas = BaseCanvas;
export const useCanvasContext = useBaseCanvasContext;
export const Window = BaseWindow;

interface HorizontalWindowsProps {
  windows: WindowConfig[];
  handleClassName?: string;
}

export const HorizontalWindows: FC<HorizontalWindowsProps> = memo(({ windows, handleClassName = "border-r" }) => {
  const { fullscreenWindow } = useCanvasContext();
  const visibleWindows = useMemo(() => (fullscreenWindow ? windows.filter((w) => w.id === fullscreenWindow) : windows), [fullscreenWindow, windows]);

  if (visibleWindows.length === 0) return null;
  if (visibleWindows.length === 1) {
    const window = visibleWindows[0];
    return <Window {...window} />;
  }

  return (
    <ResizablePanelGroup direction="horizontal">
      {visibleWindows.map((window, index) => (
        <Fragment key={window.id}>
          <ResizablePanel defaultSize={window.defaultSize ?? 100 / visibleWindows.length} className={fullscreenWindow && fullscreenWindow !== window.id ? "hidden" : "block"}>
            <Window {...window} isVisible={!fullscreenWindow || fullscreenWindow === window.id} />
          </ResizablePanel>
          {index < visibleWindows.length - 1 && <ResizableHandle className={`${handleClassName} ${fullscreenWindow ? "hidden" : "block"}`} />}
        </Fragment>
      ))}
    </ResizablePanelGroup>
  );
});

HorizontalWindows.displayName = "HorizontalWindows";

interface VerticalWindowsProps {
  windows: WindowConfig[];
  handleClassName?: string;
}

export const VerticalWindows: FC<VerticalWindowsProps> = memo(({ windows, handleClassName = "border-b" }) => {
  const { fullscreenWindow } = useCanvasContext();
  const visibleWindows = useMemo(() => (fullscreenWindow ? windows.filter((w) => w.id === fullscreenWindow) : windows), [fullscreenWindow, windows]);

  if (visibleWindows.length === 0) return null;
  if (visibleWindows.length === 1) {
    const window = visibleWindows[0];
    return <Window {...window} />;
  }

  return (
    <ResizablePanelGroup direction="vertical">
      {visibleWindows.map((window, index) => (
        <Fragment key={window.id}>
          <ResizablePanel defaultSize={window.defaultSize ?? 100 / visibleWindows.length} className={fullscreenWindow && fullscreenWindow !== window.id ? "hidden" : "block"}>
            <Window {...window} isVisible={!fullscreenWindow || fullscreenWindow === window.id} />
          </ResizablePanel>
          {index < visibleWindows.length - 1 && <ResizableHandle className={`${handleClassName} ${fullscreenWindow ? "hidden" : "block"}`} />}
        </Fragment>
      ))}
    </ResizablePanelGroup>
  );
});

VerticalWindows.displayName = "VerticalWindows";
