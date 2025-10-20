// #region Header

// Toolbar.tsx

// Type editor-specific toolbar tools that are registered to the toolbar panel.

// #endregion

import { Crosshair } from "lucide-react";
import { FC } from "react";
import { ToggleGroup, ToggleGroupItem } from "../../../elements/input/ToggleGroup";
import { useIsInTypeScope } from "../../kits/store";
import { ToolType } from "../../store";
import { useTypeEditorActiveTool, useTypeEditorCommands } from "./store";

export const ToolsToggleGroup: FC = () => {
  const isInTypeScope = useIsInTypeScope();

  if (!isInTypeScope) {
    return null;
  }

  return <ToolsToggleGroupInternal />;
};

const ToolsToggleGroupInternal: FC = () => {
  const activeTool = useTypeEditorActiveTool();
  const { setActiveTool } = useTypeEditorCommands();

  const handleValueChange = (value: string) => {
    if (value) {
      setActiveTool(value as ToolType);
    } else {
      setActiveTool(ToolType.SELECTION_NORMAL);
    }
  };

  return (
    <ToggleGroup type="single" level="panel" value={activeTool || ""} onValueChange={handleValueChange}>
      <ToggleGroupItem value={ToolType.PORT} tooltip="Port Tool" className="gap-1 px-2">
        <Crosshair className="h-4 w-4" />
        <span className="text-xs">Port</span>
      </ToggleGroupItem>
    </ToggleGroup>
  );
};
