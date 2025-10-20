// #region Header

// Workbench.tsx

// #endregion

import { useDraggable } from "@dnd-kit/core";
import { FC, useMemo } from "react";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Design, Guid, Type } from "../../../../semio";
import { useDesign, useType } from "../../../kits/store";
import { useActiveInteraction, useSketchpadCommands } from "../../../store";
import { useDesignEditorCommands, useDesignEditorHover, useDesignEditorIsTypeTransitiveHovered, useDesignEditorSelection } from "../store";

interface TypeAvatarProps {
  typeId?: Guid;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  const type = typeProp || (typeId ? (useType(undefined, typeId) as Type) : null);
  const { setActiveInteraction } = useSketchpadCommands();
  const { hoverType, clearHover } = useDesignEditorCommands();
  const activeInteraction = useActiveInteraction();
  const design = useDesign() as Design | null;
  const selection = useDesignEditorSelection();

  if (!type) {
    console.warn("[ORIGIN] TypeAvatar requires either a type or typeId prop");
    return null;
  }

  const isHovered = useDesignEditorIsTypeTransitiveHovered(undefined, type.guid);

  const interactionId = `type-${type.name}-${type.variant || ""}`;
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: interactionId,
    data: { type },
  });

  const isInteracting = activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const enhancedListeners = {
    ...listeners,
    onPointerDown: (e: React.PointerEvent) => {
      console.log("[ORIGIN] TypeAvatar onPointerDown, setting interaction:", interactionId);
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

  const isActiveSelection = isSelected;

  const displayVariant = type.variant || type.name;
  const avatar = (
    <Avatar
      className={`cursor-grab active:cursor-grabbing select-none border-[color:var(--border-color)] ${isActiveSelection ? "ring-1 ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isActiveSelection ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>
        {displayVariant.substring(0, 2).toUpperCase()}
      </AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...enhancedListeners} {...attributes} onPointerEnter={() => hoverType(type.guid)} onPointerLeave={() => clearHover()}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...enhancedListeners} {...attributes} onPointerEnter={() => hoverType(type.guid)} onPointerLeave={() => clearHover()}>
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
  const design = designProp || (designId ? (useDesign(undefined, designId) as Design) : null);
  const { setActiveInteraction } = useSketchpadCommands();
  const { hoverDesign, clearHover } = useDesignEditorCommands();
  const activeInteraction = useActiveInteraction();
  const currentDesign = useDesign() as Design | null;
  const selection = useDesignEditorSelection();
  const hover = useDesignEditorHover();

  if (!design) {
    console.warn("[ORIGIN] DesignAvatar requires either a design or designId prop");
    return null;
  }

  const isHovered = hover?.designs?.includes(design.guid) ?? false;

  const interactionId = `design-${design.name}-${design.variant || ""}-${design.view || ""}`;
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
    if (selection?.pieces?.some((pieceId) => currentDesign?.pieces?.find((piece) => piece.guid === pieceId && piece.design === design.guid))) return true;
    return selection?.design === design.guid;
  }, [design, selection?.pieces, selection?.design, currentDesign?.pieces]);

  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

  const displayVariant = design.variant || design.name;
  const avatar = (
    <Avatar
      className={`select-none ${isActive ? "cursor-default" : "cursor-grab active:cursor-grabbing"} border-[color:var(--border-color)] ${isSelectedDesign ? "ring-1 ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-[color:var(--hover-base)]" : ""}`}
      style={{ opacity: shouldFade ? 0 : isActive ? 0.5 : 1, transition: "opacity 150ms" }}
    >
      <AvatarFallback className={`select-none ${isSelectedDesign ? "bg-[var(--active-base)] text-[var(--active-foreground)]" : isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>
        {displayVariant.substring(0, 2).toUpperCase()}
      </AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...enhancedListeners} {...attributes} onPointerEnter={() => !isActive && hoverDesign(design.guid)} onPointerLeave={() => !isActive && clearHover()}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...enhancedListeners} {...attributes} onPointerEnter={() => !isActive && hoverDesign(design.guid)} onPointerLeave={() => !isActive && clearHover()}>
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
