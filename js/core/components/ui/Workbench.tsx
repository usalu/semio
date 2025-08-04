import { useDraggable } from "@dnd-kit/core";
import { FC, useState } from "react";

import { Design, Type, useDesignEditor } from "@semio/js";
import { Avatar, AvatarFallback } from "@semio/js/components/ui/Avatar";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@semio/js/components/ui/HoverCard";
import { ScrollArea } from "@semio/js/components/ui/ScrollArea";
import { Tree, TreeItem, TreeSection } from "@semio/js/components/ui/Tree";
import { ResizablePanelProps } from "./DesignEditor";

interface TypeAvatarProps {
  type: Type;
  showHoverCard?: boolean;
}

export const TypeAvatar: FC<TypeAvatarProps> = ({ type, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `type-${type.name}-${type.variant || ""}`,
  });

  const displayVariant = type.variant || type.name;
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return avatar;
  }

  return (
    <HoverCard>
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
  );
};

interface DesignAvatarProps {
  design: Design;
  showHoverCard?: boolean;
}

export const DesignAvatar: FC<DesignAvatarProps> = ({ design, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `design-${design.name}-${design.variant || ""}-${design.view || ""}`,
  });

  // Determine if this is the default variant and view
  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === "Default");

  const displayVariant = design.variant || design.name;
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  );

  if (!showHoverCard) {
    return avatar;
  }

  return (
    <HoverCard>
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
  );
};

interface WorkbenchProps extends ResizablePanelProps {}

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null;
  const { kit } = useDesignEditor();
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

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

  if (!kit?.types || !kit?.designs) return null;

  const typesByName = kit.types.reduce(
    (acc, type) => {
      acc[type.name] = acc[type.name] || [];
      acc[type.name].push(type);
      return acc;
    },
    {} as Record<string, Type[]>,
  );

  const designsByName = kit.designs.reduce(
    (acc, design) => {
      const nameKey = design.name;
      acc[nameKey] = acc[nameKey] || [];
      acc[nameKey].push(design);
      return acc;
    },
    {} as Record<string, Design[]>,
  );

  return (
    <div
      className={`absolute top-4 left-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? "border-r-primary" : "border-r"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            <TreeSection label="Types" defaultOpen={true}>
              {Object.entries(typesByName).map(([name, variants]) => (
                <TreeItem key={name} label={name} defaultOpen={false}>
                  <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                    {variants.map((type) => (
                      <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                    ))}
                  </div>
                </TreeItem>
              ))}
            </TreeSection>
            <TreeSection label="Designs" defaultOpen={true}>
              {Object.entries(designsByName).map(([name, designs]) => (
                <TreeItem key={name} label={name} defaultOpen={false}>
                  <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                    {designs.map((design) => (
                      <DesignAvatar key={`${design.name}-${design.variant}-${design.view}`} design={design} showHoverCard={true} />
                    ))}
                  </div>
                </TreeItem>
              ))}
            </TreeSection>
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Workbench;
