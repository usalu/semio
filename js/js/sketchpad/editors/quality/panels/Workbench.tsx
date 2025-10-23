// #region Header

// Workbench.tsx

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

import { useDraggable } from "@dnd-kit/core";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeSection } from "../../../../elements/aggregation/Tree";
import { useActiveInteraction, useSketchpadCommands } from "../../../store";

interface FunctionNodeProps {
  name: string;
  type: "function" | "quality" | "variable" | "unit" | "value";
  label: string;
}

const FunctionNode: FC<FunctionNodeProps> = ({ name, type, label }) => {
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const interactionId = `formula-${type}-${name}`;

  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { name, type },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction(interactionId);
      listeners?.onPointerDown?.(e);
    },
  };

  return (
    <div
      ref={setNodeRef}
      {...enhancedListeners}
      {...attributes}
      className="border border-foreground bg-base p-1 text-xs hover:bg-hover-base cursor-grab active:cursor-grabbing"
      style={{ opacity: shouldFade ? 0.3 : 1, transition: "opacity 150ms" }}
    >
      {label}
    </div>
  );
};

export const QualityWorkbench: FC = () => {
  const { t } = useTranslation();

  return (
    <>
      <TreeSection label={t("quality.numericFunctions")} defaultOpen={true}>
        <TreeContent>
          <div className="grid grid-cols-2 gap-1 p-1">
            <FunctionNode name="Add" type="function" label={t("quality.add")} />
            <FunctionNode name="Subtract" type="function" label={t("quality.subtract")} />
            <FunctionNode name="Multiply" type="function" label={t("quality.multiply")} />
            <FunctionNode name="Divide" type="function" label={t("quality.divide")} />
          </div>
        </TreeContent>
      </TreeSection>
      <TreeSection label={t("quality.branchingFunctions")} defaultOpen={true}>
        <TreeContent>
          <div className="grid grid-cols-2 gap-1 p-1">
            <FunctionNode name="If" type="function" label={t("quality.if")} />
            <FunctionNode name="Switch" type="function" label={t("quality.switch")} />
          </div>
        </TreeContent>
      </TreeSection>
      <TreeSection label={t("quality.dataStructures")} defaultOpen={true}>
        <TreeContent>
          <div className="grid grid-cols-2 gap-1 p-1">
            <FunctionNode name="List" type="function" label={t("quality.list")} />
            <FunctionNode name="Dictionary" type="function" label={t("quality.dictionary")} />
          </div>
        </TreeContent>
      </TreeSection>
    </>
  );
};
