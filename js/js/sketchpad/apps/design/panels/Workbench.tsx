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
import { useTranslation } from "react-i18next";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Design, Guid, Piece, Type } from "../../../../semio";
import { useDesign, useDesignScope, useIsInDesignScope, useKitScope, useType } from "../../../kits/store";
import { useActiveInteraction, useSketchpadCommands } from "../../../store";
import type { DesignAppSelection } from "../store";
import { useDesignAppCommands, useDesignAppHover, useDesignAppIsTypeTransitiveHovered, useDesignAppSelection } from "../store";

interface TypeAvatarProps {
  typeId?: Guid;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  // Always call useType unconditionally, even if typeId is undefined
  const typeFromStore = useType(undefined, typeId || "") as Type | null;
  const type = typeProp || typeFromStore;
  const { t } = useTranslation();
  const { setActiveInteraction, navigateToType } = useSketchpadCommands();
  const { hoverType, clearHover } = useDesignAppCommands();
  const activeInteraction = useActiveInteraction();
  const isInDesignScope = useIsInDesignScope();
  const kitGuid = useKitScope()?.guid;
  let design: Design | null = null;
  try {
    design = useDesign() as Design | null;
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error) || error.message.indexOf("DesignScopeProvider") === -1) throw error;
  }
  let selection: DesignAppSelection | undefined;
  try {
    selection = useDesignAppSelection();
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error)) throw error;
  }

  // Call hooks unconditionally, even if type is null (will use empty string as fallback)
  let isHovered = false;
  try {
    isHovered = useDesignAppIsTypeTransitiveHovered(undefined, type?.guid || "");
  } catch (error) {
    if (isInDesignScope || !(error instanceof Error)) throw error;
  }

  const interactionId = type ? `type-${type.guid}` : "type-unknown";
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { type },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      setActiveInteraction("semio.sketchpad.app.design.panel.workbench.type.drag", interactionId);
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
        return pieceType === type.guid || pieceType === type.name;
      }
      return pieceType.guid === type.guid;
    });
  }, [type, selection?.pieces, design?.pieces]);

  // Early return AFTER all hooks have been called
  if (!type) {
    return null;
  }

  const isActiveSelection = isSelected;

  const displayName = type.name || "??";
  const initials = displayName.substring(0, 2).toUpperCase();
  const avatar = (
    <Avatar
      className={`cursor-grab active:cursor-grabbing select-none border-[color:var(--border-color)] ${isActiveSelection ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isActiveSelection ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>{initials}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div
        ref={setNodeRef}
        {...enhancedListeners}
        {...attributes}
        onDoubleClick={() => {
          setActiveInteraction("semio.sketchpad.app.design.panel.workbench.type.navigate", undefined);
          if (!kitGuid) {
            return;
          }
          navigateToType(kitGuid, type.guid);
        }}
        onPointerEnter={() => {
          if (isInDesignScope) hoverType("semio.sketchpad.app.design.panel.workbench.type.hover", type.guid);
        }}
        onPointerLeave={() => {
          if (isInDesignScope) clearHover("semio.sketchpad.app.design.panel.workbench.type.leave");
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
      onDoubleClick={() => {
        setActiveInteraction("semio.sketchpad.app.design.panel.workbench.type.navigate", undefined);
        if (!kitGuid) {
          return;
        }
        navigateToType(kitGuid, type.guid);
      }}
      onPointerEnter={() => {
        if (isInDesignScope) hoverType("semio.sketchpad.app.design.panel.workbench.type.hover", type.guid);
      }}
      onPointerLeave={() => {
        if (isInDesignScope) clearHover("semio.sketchpad.app.design.panel.workbench.type.leave");
      }}
    >
      <HoverCard openDelay={500}>
        <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
        <HoverCardContent className="w-80">
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">{type.name}</h4>
            {type.isAbstract && <span className="text-xs text-muted-foreground">(Abstract)</span>}
            <p className="text-sm">{type.description || t("semio.sketchpad.common.noDescription")}</p>
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
  const { t } = useTranslation();
  const isInDesignScope = useIsInDesignScope();
  const { setActiveInteraction, navigateToDesign } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const designFromStore = designId && !designProp ? (useDesign(undefined, designId) as Design | null) : null;
  const design = designProp || designFromStore;

  const kitGuid = useKitScope()?.guid;
  const designScope = useDesignScope();
  const currentDesignFromScope = designScope ? (useDesign() as Design | null) : null;

  const selectionFromScope = designScope ? useDesignAppSelection() : undefined;
  const hoverFromScope = designScope ? useDesignAppHover() : undefined;
  const commandsFromScope = designScope ? useDesignAppCommands() : undefined;

  const isHovered = useMemo(() => {
    if (!design) return false;
    if (hoverFromScope?.designs?.includes(design.guid)) return true;
    if (hoverFromScope?.pieces && hoverFromScope.pieces.length > 0 && currentDesignFromScope?.pieces?.length) {
      if (hoverFromScope.pieces.some((pieceId: string) => currentDesignFromScope.pieces?.some((piece: Piece) => piece.guid === pieceId && piece.design === design.guid))) return true;
    }
    if (hoverFromScope?.ports && hoverFromScope.ports.length > 0 && currentDesignFromScope?.pieces?.length) {
      if (
        hoverFromScope.ports.some((port) => {
          const targetId = port.designPiece || port.piece;
          if (!targetId) return false;
          return currentDesignFromScope.pieces?.some((piece: Piece) => piece.guid === targetId && piece.design === design.guid);
        })
      )
        return true;
    }
    if (hoverFromScope?.types && hoverFromScope.types.length > 0 && design.pieces?.length) {
      if (
        hoverFromScope.types.some((typeId: string) =>
          design.pieces?.some((piece: Piece) => {
            const pieceType = piece.type as Type | string | undefined;
            if (!pieceType) return false;
            if (typeof pieceType === "string") return pieceType === typeId;
            return pieceType.guid === typeId;
          }),
        )
      )
        return true;
    }
    return false;
  }, [design, hoverFromScope, currentDesignFromScope]);

  const interactionId = design ? `design-${design.guid}` : "design-unknown";
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
        setActiveInteraction("semio.sketchpad.app.design.panel.workbench.design.drag", interactionId);
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

  const displayName = design.name || "??";
  const initials = displayName.substring(0, 2).toUpperCase();
  const avatar = (
    <Avatar
      className={`select-none ${isActive ? "cursor-default" : "cursor-grab active:cursor-grabbing"} border-[color:var(--border-color)] ${isSelectedDesign ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : isActive ? 0.5 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isSelectedDesign ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>{initials}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div
        ref={setNodeRef}
        {...enhancedListeners}
        {...attributes}
        onDoubleClick={() => {
          setActiveInteraction("semio.sketchpad.app.design.panel.workbench.design.navigate", undefined);
          if (!kitGuid) {
            return;
          }
          navigateToDesign(kitGuid, design.guid);
        }}
        onPointerEnter={() => {
          if (!isActive && isInDesignScope) commandsFromScope.hoverDesign("semio.sketchpad.app.design.panel.workbench.design.hover", design.guid);
        }}
        onPointerLeave={() => {
          if (!isActive && isInDesignScope) commandsFromScope.clearHover("semio.sketchpad.app.design.panel.workbench.design.leave");
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
      onDoubleClick={() => {
        setActiveInteraction("semio.sketchpad.app.design.panel.workbench.design.navigate", undefined);
        if (!kitGuid) {
          return;
        }
        navigateToDesign(kitGuid, design.guid);
      }}
      onPointerEnter={() => {
        if (!isActive && isInDesignScope) commandsFromScope.hoverDesign("semio.sketchpad.app.design.panel.workbench.design.hover", design.guid);
      }}
      onPointerLeave={() => {
        if (!isActive && isInDesignScope) commandsFromScope.clearHover("semio.sketchpad.app.design.panel.workbench.design.leave");
      }}
    >
      <HoverCard openDelay={500}>
        <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
        <HoverCardContent className="w-80">
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">{design.name}</h4>
            {design.isAbstract && <span className="text-xs text-muted-foreground">(Abstract)</span>}
            <p className="text-sm">{design.description || t("semio.sketchpad.common.noDescription")}</p>
          </div>
        </HoverCardContent>
      </HoverCard>
    </div>
  );
};
