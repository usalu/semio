// #region Header

// Tools.tsx

// Tool components for the design editor toolbar.
// Provides selection and lasso tools with dropdown variants.

// #endregion

import { Lasso, MousePointer2, Square } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "../../../elements/display/Tooltip";
import { Button } from "../../../elements/input/Button";
import { Popover, PopoverContent, PopoverTrigger } from "../../../elements/Popover";
import { ToolType } from "../../store";
import { useDesignEditor, useDesignEditorCommands } from "./store";

export const ToolsToggleGroup: FC = () => {
  const { t } = useTranslation();
  const { kit, design } = useParams();
  const editor = useDesignEditor((s) => s, kit && design ? { kit, design } : undefined);
  const { setActiveTool } = useDesignEditorCommands(kit && design ? { kit, design } : undefined);

  if (!editor) return null;

  const activeTool = editor.activeTool || ToolType.SELECTION_NORMAL;

  const isSelectionTool = activeTool === ToolType.SELECTION_NORMAL || activeTool === ToolType.SELECTION_ADDITIVE || activeTool === ToolType.SELECTION_SUBTRACTIVE;
  const isLassoTool = activeTool === ToolType.LASSO_RECTANGULAR || activeTool === ToolType.LASSO_FREEFORM;

  const getSelectionIcon = () => <MousePointer2 className="h-4 w-4" />;

  const getSelectionLabel = () => {
    switch (activeTool) {
      case ToolType.SELECTION_NORMAL:
        return "Normal";
      case ToolType.SELECTION_ADDITIVE:
        return "Additive";
      case ToolType.SELECTION_SUBTRACTIVE:
        return "Subtractive";
      default:
        return "Select";
    }
  };

  const getLassoIcon = () => {
    switch (activeTool) {
      case ToolType.LASSO_RECTANGULAR:
        return <Square className="h-4 w-4" />;
      case ToolType.LASSO_FREEFORM:
        return <Lasso className="h-4 w-4" />;
      default:
        return <Square className="h-4 w-4" />;
    }
  };

  const getLassoLabel = () => {
    switch (activeTool) {
      case ToolType.LASSO_RECTANGULAR:
        return "Rectangular";
      case ToolType.LASSO_FREEFORM:
        return "Freeform";
      default:
        return "Lasso";
    }
  };

  return (
    <TooltipProvider>
      <div className="flex items-center gap-2">
        <Popover>
          <Tooltip>
            <TooltipTrigger asChild>
              <PopoverTrigger asChild>
                <Button level="panel" variant={isSelectionTool ? "default" : "ghost"} className="h-8 w-8 p-0">
                  {getSelectionIcon()}
                </Button>
              </PopoverTrigger>
            </TooltipTrigger>
            <TooltipContent>
              <div className="text-xs">
                <div className="font-semibold">Selection Tool</div>
                <div className="text-muted-foreground">Click to select pieces</div>
              </div>
            </TooltipContent>
          </Tooltip>
          <PopoverContent className="w-48 p-2">
            <div className="space-y-1">
              <Button level="temporary" variant="ghost" className="w-full justify-start gap-2 h-8" onClick={() => setActiveTool(ToolType.SELECTION_NORMAL)}>
                <MousePointer2 className="h-4 w-4" />
                <span className="text-xs">Normal (Click)</span>
              </Button>
              <Button level="temporary" variant="ghost" className="w-full justify-start gap-2 h-8" onClick={() => setActiveTool(ToolType.SELECTION_ADDITIVE)}>
                <MousePointer2 className="h-4 w-4" />
                <span className="text-xs">Additive (Shift)</span>
              </Button>
              <Button level="temporary" variant="ghost" className="w-full justify-start gap-2 h-8" onClick={() => setActiveTool(ToolType.SELECTION_SUBTRACTIVE)}>
                <MousePointer2 className="h-4 w-4" />
                <span className="text-xs">Subtractive (Ctrl)</span>
              </Button>
            </div>
          </PopoverContent>
        </Popover>

        <Popover>
          <Tooltip>
            <TooltipTrigger asChild>
              <PopoverTrigger asChild>
                <Button level="panel" variant={isLassoTool ? "default" : "ghost"} className="h-8 w-8 p-0">
                  {getLassoIcon()}
                </Button>
              </PopoverTrigger>
            </TooltipTrigger>
            <TooltipContent>
              <div className="text-xs">
                <div className="font-semibold">Lasso Tool</div>
                <div className="text-muted-foreground">Draw to select multiple pieces</div>
              </div>
            </TooltipContent>
          </Tooltip>
          <PopoverContent className="w-48 p-2">
            <div className="space-y-1">
              <Button level="temporary" variant="ghost" className="w-full justify-start gap-2 h-8" onClick={() => setActiveTool(ToolType.LASSO_RECTANGULAR)}>
                <Square className="h-4 w-4" />
                <span className="text-xs">Rectangular</span>
              </Button>
              <Button level="temporary" variant="ghost" className="w-full justify-start gap-2 h-8" onClick={() => setActiveTool(ToolType.LASSO_FREEFORM)}>
                <Lasso className="h-4 w-4" />
                <span className="text-xs">Freeform</span>
              </Button>
            </div>
          </PopoverContent>
        </Popover>
      </div>
    </TooltipProvider>
  );
};
