// #region Header

// Editor.tsx

// Generalized, reusable editor component system.
// Provides base editor components and patterns that can be extended by specific editors.

// #endregion

import { FC, ReactNode } from "react";

export enum WindowType {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
}

export interface EditorWindowProps {
  type: WindowType;
  children: ReactNode;
  className?: string;
}

export const EditorWindow: FC<EditorWindowProps> = ({ type, children, className = "" }) => <div className={`editor-window editor-window-${type} relative h-full w-full ${className}`}>{children}</div>;
