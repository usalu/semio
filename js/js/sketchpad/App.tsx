// #region Header

// App.tsx

// Generalized, reusable app component system.
// Provides base app components and patterns that can be extended by specific apps.

// #endregion

import { FC, ReactNode } from "react";

export enum WindowType {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
}

export interface AppWindowProps {
  type: WindowType;
  children: ReactNode;
  className?: string;
}

export const AppWindow: FC<AppWindowProps> = ({ type, children, className = "" }) => <div className={`app-window app-window-${type} relative h-full w-full ${className}`}>{children}</div>;
