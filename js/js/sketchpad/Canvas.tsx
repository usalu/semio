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

import { FC, ReactNode, createContext, useContext, useState, useCallback, useMemo, Fragment, memo } from "react";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../elements/aggregation/Resizable";

export enum WindowLayout {
  SINGLE = "single",
  HORIZONTAL_SPLIT = "horizontal-split",
  VERTICAL_SPLIT = "vertical-split",
  GRID = "grid",
}

export interface WindowConfig {
  id: string;
  children: ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
}

interface CanvasContextValue {
  fullscreenWindow: string | null;
  setFullscreenWindow: (windowId: string | null) => void;
  toggleFullscreenWindow: (windowId: string) => void;
}

const CanvasContext = createContext<CanvasContextValue | null>(null);

export const useCanvasContext = () => {
  const context = useContext(CanvasContext);
  if (!context) throw new Error("useCanvasContext must be used within Canvas");
  return context;
};

interface CanvasProps {
  children: ReactNode;
  className?: string;
}

export const Canvas: FC<CanvasProps> = ({ children, className = "" }) => {
  const [fullscreenWindow, setFullscreenWindow] = useState<string | null>(null);

  const toggleFullscreenWindow = useCallback((windowId: string) => {
    setFullscreenWindow((current) => (current === windowId ? null : windowId));
  }, []);

  const contextValue = useMemo(
    () => ({ fullscreenWindow, setFullscreenWindow, toggleFullscreenWindow }),
    [fullscreenWindow, toggleFullscreenWindow]
  );

  return (
    <CanvasContext.Provider value={contextValue}>
      <div className={`relative h-full w-full ${className}`}>{children}</div>
    </CanvasContext.Provider>
  );
};

interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

export const Window: FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true }) => {
  if (!isVisible) return null;
  return (
    <div className={`relative h-full w-full ${className}`} onDoubleClick={onDoubleClick}>
      {children}
    </div>
  );
};

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
