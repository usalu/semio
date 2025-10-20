// #region Header

// Tools.tsx

// Tool components for the design editor toolbar.
// Provides selection and lasso tools with dropdown variants.

// #endregion

import { ChevronDown, Lasso, MousePointer2, Square } from "lucide-react";
import { FC } from "react";
import { Button } from "../../../elements/input/Button";
import { ToggleGroup, ToggleGroupItem } from "../../../elements/input/ToggleGroup";
import { Popover, PopoverContent, PopoverTrigger } from "../../../elements/Popover";
import { ToolType, useIsInDesignScope } from "../../store";
import { useDesignEditor, useDesignEditorCommands } from "./store";

export const SelectionTools: FC = () => {
  const activeTool = useDesignEditor((s) => s.activeTool);
  const { setActiveTool } = useDesignEditorCommands();

  const isSelectionTool = activeTool === ToolType.SELECTION_NORMAL || activeTool === ToolType.SELECTION_ADDITIVE || activeTool === ToolType.SELECTION_SUBTRACTIVE;

  const getSelectionIcon = () => {
    switch (activeTool) {
      case ToolType.SELECTION_NORMAL:
      case ToolType.SELECTION_ADDITIVE:
      case ToolType.SELECTION_SUBTRACTIVE:
        return <MousePointer2 className="h-4 w-4" />;
      default:
        return <MousePointer2 className="h-4 w-4" />;
    }
  };

  const getSelectionLabel = () => {
    switch (activeTool) {
      case ToolType.SELECTION_NORMAL:
        return "Select";
      case ToolType.SELECTION_ADDITIVE:
        return "Add to Selection";
      case ToolType.SELECTION_SUBTRACTIVE:
        return "Remove from Selection";
      default:
        return "Select";
    }
  };

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button level="panel" variant={isSelectionTool ? "default" : "ghost"} className="gap-1 h-8 px-2">
          {getSelectionIcon()}
          <span className="text-xs">{getSelectionLabel()}</span>
          <ChevronDown className="h-3 w-3 opacity-50" />
        </Button>
      </PopoverTrigger>
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
  );
};

export const LassoTools: FC = () => {
  const activeTool = useDesignEditor((s) => s.activeTool);
  const { setActiveTool } = useDesignEditorCommands();

  const isLassoTool = activeTool === ToolType.LASSO_RECTANGULAR || activeTool === ToolType.LASSO_FREEFORM;

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
        return "Rectangle";
      case ToolType.LASSO_FREEFORM:
        return "Freeform";
      default:
        return "Lasso";
    }
  };

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button level="panel" variant={isLassoTool ? "default" : "ghost"} className="gap-1 h-8 px-2">
          {getLassoIcon()}
          <span className="text-xs">{getLassoLabel()}</span>
          <ChevronDown className="h-3 w-3 opacity-50" />
        </Button>
      </PopoverTrigger>
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
  );
};

export const ToolsToggleGroup: FC = () => {
  const isInDesignScope = useIsInDesignScope();

  if (!isInDesignScope) {
    return null;
  }

  return <ToolsToggleGroupInternal />;
};

const ToolsToggleGroupInternal: FC = () => {
  const activeTool = useDesignEditor((s) => s.activeTool);
  const { setActiveTool } = useDesignEditorCommands();

  const isSelectionTool = activeTool === ToolType.SELECTION_NORMAL || activeTool === ToolType.SELECTION_ADDITIVE || activeTool === ToolType.SELECTION_SUBTRACTIVE;
  const isLassoTool = activeTool === ToolType.LASSO_RECTANGULAR || activeTool === ToolType.LASSO_FREEFORM;

  const handleValueChange = (value: string) => {
    if (value === "selection") {
      setActiveTool(ToolType.SELECTION_NORMAL);
    } else if (value === "lasso") {
      setActiveTool(ToolType.LASSO_RECTANGULAR);
    }
  };

  const currentValue = isSelectionTool ? "selection" : isLassoTool ? "lasso" : "selection";

  return (
    <ToggleGroup type="single" value={currentValue} onValueChange={handleValueChange} level="panel">
      <ToggleGroupItem value="selection" tooltip="Selection Tool" className="gap-1 px-2">
        <MousePointer2 className="h-4 w-4" />
      </ToggleGroupItem>
      <ToggleGroupItem value="lasso" tooltip="Lasso Tool" className="gap-1 px-2">
        <Square className="h-4 w-4" />
      </ToggleGroupItem>
    </ToggleGroup>
  );
};
