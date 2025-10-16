// #region Header

// Workbench.tsx

// #endregion

import { useDraggable } from "@dnd-kit/core";
import { FC } from "react";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../elements/display/HoverCard";
import { Design, Guid, Type } from "../../../../semio";
import { useActiveInteraction, useDesign, useSketchpadCommands, useType } from "../../../store";

interface TypeAvatarProps {
  typeId?: Guid;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  const type = typeProp || (typeId ? (useType(undefined, typeId) as Type) : null);
  const { setActiveInteraction } = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

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
    <Avatar className="cursor-grab active:cursor-grabbing select-none" style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
      <AvatarFallback className="select-none">{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
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
    <Avatar className={`select-none ${isActive ? "cursor-default opacity-50" : "cursor-grab active:cursor-grabbing"}`} style={{ opacity: shouldFade ? 0 : isActive ? 0.5 : 1, transition: "opacity 150ms" }}>
      <AvatarFallback className="select-none">{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
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
