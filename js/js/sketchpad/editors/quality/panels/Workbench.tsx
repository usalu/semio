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
import { DraggableAvatar } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Guid, Kit, Quality } from "../../../../semio";
import { useKit, useQuality } from "../../../kits/store";
import { useActiveInteraction, useSketchpadCommands } from "../../../store";
import { formulaFunctions } from "../functions";

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
  const shouldFade = !!(activeInteraction && !isInteracting);

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction(interactionId);
      listeners?.onPointerDown?.(e);
    },
  };

  // Get function initials (first letter of name)
  const initials = name.substring(0, 2).toUpperCase();
  const fn = formulaFunctions[name];

  return (
    <HoverCard openDelay={500}>
      <HoverCardTrigger asChild>
        <div>
          <DraggableAvatar content={initials} shouldFade={shouldFade} title={label} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />
        </div>
      </HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          <h4 className="text-sm font-semibold">{label}</h4>
          {fn?.description && <p className="text-sm">{fn.description}</p>}
        </div>
      </HoverCardContent>
    </HoverCard>
  );
};

interface QualityAvatarProps {
  qualityId?: Guid;
  quality?: Quality;
  showHoverCard?: boolean;
}

export const QualityAvatar: FC<QualityAvatarProps> = ({ qualityId, quality: qualityProp, showHoverCard = false }) => {
  const qualityFromStore = qualityId && !qualityProp ? (useQuality(undefined, qualityId) as Quality | null) : null;
  const quality = qualityProp || qualityFromStore;
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const interactionId = quality ? `quality-${quality.key}` : "quality-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { quality, type: "quality" },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = !!(activeInteraction && !isInteracting);

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction(interactionId);
      listeners?.onPointerDown?.(e);
    },
  };

  if (!quality) {
    return null;
  }

  const displayName = quality.name || quality.key || "Q";
  const initials =
    displayName
      .split(".")
      .map((part) => part[0])
      .filter(Boolean)
      .join("")
      .substring(0, 2)
      .toUpperCase() || "Q";

  if (!showHoverCard) {
    return <DraggableAvatar content={initials} shouldFade={shouldFade} title={quality.name || quality.key} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />;
  }

  return (
    <HoverCard openDelay={500}>
      <HoverCardTrigger asChild>
        <div>
          <DraggableAvatar content={initials} shouldFade={shouldFade} title={quality.name || quality.key} dragRef={setNodeRef} dragListeners={enhancedListeners} dragAttributes={attributes} />
        </div>
      </HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          <h4 className="text-sm font-semibold">{quality.name}</h4>
          <p className="text-xs text-muted-foreground">{quality.key}</p>
          {quality.description && <p className="text-sm">{quality.description}</p>}
          {quality.formula && (
            <div className="text-xs text-muted-foreground mt-2">
              <span className="font-mono">{quality.formula}</span>
            </div>
          )}
        </div>
      </HoverCardContent>
    </HoverCard>
  );
};

export const QualityWorkbench: FC = () => {
  const { t } = useTranslation();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const qualities = kit?.qualities || [];

  console.log("[ORIGIN] QualityWorkbench - kit:", kit);
  console.log("[ORIGIN] QualityWorkbench - qualities:", qualities);

  return (
    <>
      <TreeSection label={t("quality.numericFunctions")} defaultOpen={true}>
        <TreeContent>
          <div className="flex flex-wrap gap-1 p-1">
            <FunctionNode name="Add" type="function" label={t("quality.add")} />
            <FunctionNode name="Subtract" type="function" label={t("quality.subtract")} />
            <FunctionNode name="Multiply" type="function" label={t("quality.multiply")} />
            <FunctionNode name="Divide" type="function" label={t("quality.divide")} />
          </div>
        </TreeContent>
      </TreeSection>
      <TreeSection label={t("quality.branchingFunctions")} defaultOpen={true}>
        <TreeContent>
          <div className="flex flex-wrap gap-1 p-1">
            <FunctionNode name="If" type="function" label={t("quality.if")} />
            <FunctionNode name="Switch" type="function" label={t("quality.switch")} />
          </div>
        </TreeContent>
      </TreeSection>
      <TreeSection label={t("quality.dataStructures")} defaultOpen={true}>
        <TreeContent>
          <div className="flex flex-wrap gap-1 p-1">
            <FunctionNode name="List" type="function" label={t("quality.list")} />
            <FunctionNode name="Dictionary" type="function" label={t("quality.dictionary")} />
          </div>
        </TreeContent>
      </TreeSection>
      <TreeSection label={t("quality.qualities")} defaultOpen={true}>
        <TreeContent>{qualities.length === 0 ? <div className="text-sm text-muted-foreground p-2">{t("quality.noQualities")}</div> : <QualityTree qualities={qualities} />}</TreeContent>
      </TreeSection>
    </>
  );
};

/**
 * Build nested quality tree structure based on quality keys
 */
interface QualityTreeNode {
  key: string;
  qualities: Quality[];
  children: Map<string, QualityTreeNode>;
}

const buildQualityTree = (qualities: Quality[]): Map<string, QualityTreeNode> => {
  const root = new Map<string, QualityTreeNode>();

  qualities.forEach((quality) => {
    if (!quality.key) return;

    const parts = quality.key.split(".");
    let currentLevel = root;

    parts.forEach((part, index) => {
      if (!currentLevel.has(part)) {
        currentLevel.set(part, {
          key: parts.slice(0, index + 1).join("."),
          qualities: [],
          children: new Map(),
        });
      }

      const node = currentLevel.get(part)!;

      // If this is the last part, add the quality to this node
      if (index === parts.length - 1) {
        node.qualities.push(quality);
      }

      currentLevel = node.children;
    });
  });

  return root;
};

/**
 * Render quality tree recursively
 */
const QualityTree: FC<{ qualities: Quality[] }> = ({ qualities }) => {
  const tree = buildQualityTree(qualities);

  const renderNode = (key: string, node: QualityTreeNode, level: number = 0) => {
    const hasChildren = node.children.size > 0;
    const hasQualities = node.qualities.length > 0;

    if (hasChildren) {
      // Render as a tree section with children
      return (
        <TreeSection key={key} label={key} defaultOpen={level < 2}>
          <TreeContent>
            {hasQualities && (
              <div className="flex flex-wrap gap-1 p-1">
                {node.qualities.map((quality) => (
                  <QualityAvatar key={quality.guid} quality={quality} showHoverCard={true} />
                ))}
              </div>
            )}
            {Array.from(node.children.entries()).map(([childKey, childNode]) => renderNode(childKey, childNode, level + 1))}
          </TreeContent>
        </TreeSection>
      );
    } else if (hasQualities) {
      // Render as a flat list of quality avatars
      return (
        <div key={key} className="flex flex-wrap gap-1 p-1">
          {node.qualities.map((quality) => (
            <QualityAvatar key={quality.guid} quality={quality} showHoverCard={true} />
          ))}
        </div>
      );
    }

    return <></>;
  };

  return <>{Array.from(tree.entries()).map(([key, node]) => renderNode(key, node))}</>;
};
