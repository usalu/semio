import { useDraggable } from "@dnd-kit/core";
import { FC, useState } from "react";

import { Avatar, AvatarFallback } from "@semio/js/components/ui/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@semio/js/components/ui/HoverCard";
import { ScrollArea } from "@semio/js/components/ui/ScrollArea";
import { Tree, TreeSection } from "@semio/js/components/ui/Tree";
import { Design, Type } from "../../../semio";
import { DesignId, TypeId, useIsMobile } from "../../../store";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

interface TypeAvatarProps {
  typeId?: TypeId;
  type?: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ typeId, type: typeProp, showHoverCard = false }) => {
  const type = typeProp;
  
  if (!type) {
    console.warn("[ORIGIN] TypeAvatar requires a type prop, received:", { typeId, typeProp });
    return null;
  }
  
  const dragId = `type-${type.name}-${type.variant || ""}`;
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: dragId,
    data: { type },
  });

  const displayVariant = type.variant || type.name;
  const avatar = (
    <Avatar className="cursor-grab active:cursor-grabbing select-none">
      <AvatarFallback className="select-none">{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...listeners} {...attributes}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...listeners} {...attributes}>
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
  designId?: DesignId;
  design?: Design;
  showHoverCard?: boolean;
  isActive?: boolean;
}

export const DesignAvatar: FC<DesignAvatarProps> = ({ designId, design: designProp, showHoverCard = false, isActive = false }) => {
  const design = designProp;
  
  if (!design) {
    console.warn("[ORIGIN] DesignAvatar requires a design prop");
    return null;
  }
  
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `design-${design.name}-${design.variant || ""}-${design.view || ""}`,
    data: { design },
    disabled: isActive,
  });

  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

  const displayVariant = design.variant || design.name;
  const avatar = (
    <Avatar className={`select-none ${isActive ? "cursor-default opacity-50" : "cursor-grab active:cursor-grabbing"}`}>
      <AvatarFallback className="select-none">{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return (
      <div ref={setNodeRef} {...listeners} {...attributes}>
        {avatar}
      </div>
    );
  }

  return (
    <div ref={setNodeRef} {...listeners} {...attributes}>
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

interface WorkbenchProps extends ResizablePanelProps { }

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();

  const sections = usePanelSections("workbench");

  if (!visible) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth + (e.clientX - startX);
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  const sortedSections = sections.sort((a, b) => (a.order || 0) - (b.order || 0));

  return (
    <div
      className={`h-full z-20 bg-panel text-foreground border
                ${isResizing || isResizeHovered ? "border-r-primary" : "border-r"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className={isMobile ? "p-2" : "p-1"}>
          <Tree>
            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {typeof section.content === "function" ? section.content() : section.content}
              </TreeSection>
            ))}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Workbench;
