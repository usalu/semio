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
import { FC, useMemo } from "react";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Design, Guid, Piece, Type } from "../../../../semio";
import { useDesign, useDesignScope, useIsInDesignScope, useType } from "../../../kits/store";
import { useActiveInteraction, useSketchpadCommands } from "../../../store";
import type { DesignEditorSelection } from "../store";
import { useDesignEditorCommandsSafe, useDesignEditorCommands, useDesignEditorHover, useDesignEditorHoverSafe, useDesignEditorIsTypeTransitiveHovered, useDesignEditorSelection, useDesignEditorSelectionSafe } from "../store";

interface TypeAvatarProps {
  typeId?: Guid;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  // Always call useType unconditionally, even if typeId is undefined
  const typeFromStore = useType(undefined, typeId || "") as Type | null;
  const type = typeProp || typeFromStore;
  const { setActiveInteraction } = useSketchpadCommands();
  const { hoverType, clearHover } = useDesignEditorCommands();
  const activeInteraction = useActiveInteraction();
  const isInDesignScope = useIsInDesignScope();
  let design: Design | null = null;
  try {
    design = useDesign() as Design | null;
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error) || error.message.indexOf("DesignScopeProvider") === -1) throw error;
  }
  let selection: DesignEditorSelection | undefined;
  try {
    selection = useDesignEditorSelection();
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error)) throw error;
  }

  // Call hooks unconditionally, even if type is null (will use empty string as fallback)
  let isHovered = false;
  try {
    isHovered = useDesignEditorIsTypeTransitiveHovered(undefined, type?.guid || "");
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error)) throw error;
  }

  const interactionId = type ? `type-${type.name}-${type.variant || ""}` : "type-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { type },
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

  const isSelected = useMemo(() => {
    if (!type || !selection?.pieces?.length || !design?.pieces?.length) return false;
    return selection.pieces.some((pieceId) => {
      const piece = design.pieces?.find((p) => p.guid === pieceId);
      if (!piece) return false;
      const pieceType = piece.type as Type | string | undefined;
      if (!pieceType) return false;
      if (typeof pieceType === "string") {
        return pieceType === type.guid || pieceType === type.name || pieceType === type.variant;
      }
      return pieceType.guid === type.guid;
    });
  }, [type, selection?.pieces, design?.pieces]);

  // Early return AFTER all hooks have been called
  if (!type) {
    return null;
  }

  const isActiveSelection = isSelected;

  const displayVariant = type.variant || type.name;
  const avatar = (
    <Avatar
      className={`cursor-grab active:cursor-grabbing select-none border-[color:var(--border-color)] ${isActiveSelection ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isActiveSelection ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>
        {displayVariant.substring(0, 2).toUpperCase()}
      </AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div
        ref={setNodeRef}
        {...enhancedListeners}
        {...attributes}
        onPointerEnter={() => {
          if (isInDesignScope) hoverType(type.guid);
        }}
        onPointerLeave={() => {
          if (isInDesignScope) clearHover();
        }}
      >
        {avatar}
      </div>
    );
  }

  return (
    <div
      ref={setNodeRef}
      {...enhancedListeners}
      {...attributes}
      onPointerEnter={() => {
        if (isInDesignScope) hoverType(type.guid);
      }}
      onPointerLeave={() => {
        if (isInDesignScope) clearHover();
      }}
    >
      <HoverCard openDelay={500}>
        <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
        <HoverCardContent className="w-80">
          <div className="space-y-1">
            {type.variant ? (
              <>
                <h4 className="text-sm font-semibold">{type.variant}</h4>
                <p className="text-sm">{type.description || "No description available."}</p>
              </>
            ) : (
              <p className="text-sm">{type.description || "No description available."}</p>
            )}
          </div>
        </HoverCardContent>
      </HoverCard>
    </div>
  );
};

interface DesignAvatarProps {
  designId?: Guid;
  design?: Design;
  showHoverCard?: boolean;
  isActive?: boolean;
}

export const DesignAvatar: FC<DesignAvatarProps> = ({ designId, design: designProp, showHoverCard = false, isActive = false }) => {
  const isInDesignScope = useIsInDesignScope();
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const designFromStore = designId && !designProp ? (useDesign(undefined, designId) as Design | null) : null;
  const design = designProp || designFromStore;

  const designScope = useDesignScope();
  const currentDesignFromScope = designScope ? (useDesign() as Design | null) : null;

  const selectionFromScope = useDesignEditorSelectionSafe();
  const hoverFromScope = useDesignEditorHoverSafe();
  const commandsFromScope = useDesignEditorCommandsSafe();

  const isHovered = hoverFromScope?.designs?.includes(design?.guid || "") ?? false;

  const interactionId = design ? `design-${design.name}-${design.variant || ""}-${design.view || ""}` : "design-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { design },
    disabled: isActive,
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      if (!isActive) {
        setActiveInteraction(interactionId);
        listeners?.onPointerDown?.(e);
      }
    },
  };

  const isSelectedDesign = useMemo(() => {
    if (!design) return false;
    if (selectionFromScope?.pieces?.some((pieceId: string) => currentDesignFromScope?.pieces?.find((piece: Piece) => piece.guid === pieceId && piece.design === design.guid))) return true;
    return selectionFromScope?.pieces?.some((pieceId: string) => currentDesignFromScope?.pieces?.find((piece: Piece) => piece.guid === pieceId && piece.design === design.guid)) ?? false;
  }, [design, selectionFromScope?.pieces, currentDesignFromScope?.pieces]);

  // Early return AFTER all hooks have been called
  if (!design) {
    return null;
  }

  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

  const displayVariant = design.variant || design.name;
  const avatar = (
    <Avatar
      className={`select-none ${isActive ? "cursor-default" : "cursor-grab active:cursor-grabbing"} border-[color:var(--border-color)] ${isSelectedDesign ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : isActive ? 0.5 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isSelectedDesign ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>
        {displayVariant.substring(0, 2).toUpperCase()}
      </AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div
        ref={setNodeRef}
        {...enhancedListeners}
        {...attributes}
        onPointerEnter={() => {
          if (!isActive && isInDesignScope) commandsFromScope.hoverDesign(design.guid);
        }}
        onPointerLeave={() => {
          if (!isActive && isInDesignScope) commandsFromScope.clearHover();
        }}
      >
        {avatar}
      </div>
    );
  }

  return (
    <div
      ref={setNodeRef}
      {...enhancedListeners}
      {...attributes}
      onPointerEnter={() => {
        if (!isActive && isInDesignScope) commandsFromScope.hoverDesign(design.guid);
      }}
      onPointerLeave={() => {
        if (!isActive && isInDesignScope) commandsFromScope.clearHover();
      }}
    >
      <HoverCard openDelay={500}>
        <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
        <HoverCardContent className="w-80">
          <div className="space-y-1">
            {!isDefault && (
              <h4 className="text-sm font-semibold">
                {design.variant || design.name}
                {design.view && design.view !== "Default" && ` (${design.view})`}
              </h4>
            )}
            <p className="text-sm">{design.description || "No description available."}</p>
          </div>
        </HoverCardContent>
      </HoverCard>
    </div>
  );
};
