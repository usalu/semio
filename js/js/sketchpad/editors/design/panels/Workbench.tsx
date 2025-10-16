// #region Header

// Workbench.tsx

// #endregion

import { useDraggable } from "@dnd-kit/core";
import { FC, useMemo } from "react";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Design, Guid, Type } from "../../../../semio";
import { useActiveInteraction, useDesign, useDesignEditorHover, useSketchpadCommands, useType } from "../../../store";

interface TypeAvatarProps {
  typeId?: Guid;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  const type = typeProp || (typeId ? (useType(undefined, typeId) as Type) : null);
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();
  const design = useDesign() as Design | null;
  const hover = useDesignEditorHover();
  const hoveredPiece = useMemo(() => {
    if (!hover?.piece || !design?.pieces) return undefined;
    return design.pieces.find((piece) => piece.guid === hover.piece);
  }, [hover?.piece, design]);
  const isHovered = useMemo(() => {
    if (!hoveredPiece || hoveredPiece.design) return false;
    const hoveredPieceType = hoveredPiece.type as Type | string | undefined;
    if (!hoveredPieceType) return false;
    if (typeof hoveredPieceType === "string") {
      return hoveredPieceType === type.guid || hoveredPieceType === type.name || hoveredPieceType === type.variant;
    }
    return hoveredPieceType.guid === type.guid;
  }, [hoveredPiece, type.guid, type.name, type.variant]);

  if (!type) {
    console.warn("[ORIGIN] TypeAvatar requires either a type or typeId prop");
    return null;
  }

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

  const displayVariant = type.variant || type.name;
  const avatar = (
    <Avatar className={`cursor-grab active:cursor-grabbing select-none ${isHovered ? "border-[var(--hover-base)]" : ""}`} style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
      <AvatarFallback className={`select-none ${isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...enhancedListeners} {...attributes}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...enhancedListeners} {...attributes}>
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
  const activeInteraction = useActiveInteraction();
  const currentDesign = useDesign() as Design | null;
  const hover = useDesignEditorHover();
  const hoveredPiece = useMemo(() => {
    if (!hover?.piece || !currentDesign?.pieces) return undefined;
    return currentDesign.pieces.find((piece) => piece.guid === hover.piece);
  }, [hover?.piece, currentDesign]);
  const isHovered = useMemo(() => {
    if (!design) return false;
    if (hoveredPiece?.design === design.guid) return true;
    if (hover?.port?.designPiece === design.guid) return true;
    return false;
  }, [design, hoveredPiece?.design, hover?.port?.designPiece]);

  if (!design) {
    console.warn("[ORIGIN] DesignAvatar requires either a design or designId prop");
    return null;
  }

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

  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

  const displayVariant = design.variant || design.name;
  const avatar = (
    <Avatar className={`select-none ${isActive ? "cursor-default" : "cursor-grab active:cursor-grabbing"} ${isHovered ? "border-[var(--hover-base)]" : ""}`} style={{ opacity: shouldFade ? 0 : isActive ? 0.5 : 1, transition: "opacity 150ms" }}>
      <AvatarFallback className={`select-none ${isHovered ? "bg-[var(--hover-base)] text-foreground" : "bg-muted"}`}>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...enhancedListeners} {...attributes}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...enhancedListeners} {...attributes}>
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
