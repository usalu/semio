// #region Header

// Tools.tsx

// Type editor-specific toolbar tools that are registered to the toolbar panel.
// Rendered within the toolbar when the type editor is active.

// #endregion

import { Crosshair, MousePointer2 } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { TooltipProvider } from "../../../elements/display/Tooltip";
import { ToggleGroup, ToggleGroupItem } from "../../../elements/input/ToggleGroup";
import { ToolType } from "../../store";
import { useTypeEditor, useTypeEditorCommands } from "./store";

export const ToolsToggleGroup: FC = () => {
  const { t } = useTranslation();
  const { kit, type } = useParams();
  const editor = useTypeEditor((s) => s, kit && type ? { kit, type } : undefined);
  const { setActiveTool } = useTypeEditorCommands(kit && type ? { kit, type } : undefined);

  console.log("[ORIGIN] ToolsToggleGroup render", { kit, type, editor, activeTool: editor?.activeTool });

  if (!editor) return null;

  const activeTool = editor.activeTool ?? ToolType.PORT;

  const handleValueChange = (value: string) => {
    console.log("[ORIGIN] handleValueChange", { value, currentActiveTool: activeTool });
    if (value) {
      setActiveTool(value as ToolType);
    }
  };

  return (
    <TooltipProvider>
      <ToggleGroup type="single" level="panel" value={activeTool} onValueChange={handleValueChange}>
        <ToggleGroupItem value={ToolType.SELECTION_NORMAL} className="h-8 w-8 p-0" tooltip={t("tools.selection")}>
          <MousePointer2 className="h-4 w-4" />
        </ToggleGroupItem>
        <ToggleGroupItem value={ToolType.PORT} className="h-8 w-8 p-0" tooltip={t("tools.port")}>
          <Crosshair className="h-4 w-4" />
        </ToggleGroupItem>
      </ToggleGroup>
    </TooltipProvider>
  );
};
