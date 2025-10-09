import { FC, ReactNode, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";

import { ScrollArea } from "@semio/js/components/ui/ScrollArea";
import { Tree, TreeItem, TreeSection } from "@semio/js/components/ui/Tree";
import { DesignScopeProvider, KitScopeProvider, TypeScopeProvider } from "../../../store";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

interface DetailsProps extends ResizablePanelProps {}

const ScopedContent: FC<{ children: ReactNode }> = ({ children }) => {
  const { kit, design, type } = useParams();

  if (design && kit) {
    return (
      <KitScopeProvider guid={kit}>
        <DesignScopeProvider guid={design}>{children}</DesignScopeProvider>
      </KitScopeProvider>
    );
  }
  if (type && kit) {
    return (
      <KitScopeProvider guid={kit}>
        <TypeScopeProvider guid={type}>{children}</TypeScopeProvider>
      </KitScopeProvider>
    );
  }
  if (kit) {
    return <KitScopeProvider guid={kit}>{children}</KitScopeProvider>;
  }

  return <>{children}</>;
};

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const sections = usePanelSections("details");

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX);
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
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1 overflow-hidden min-w-0">
          <ScopedContent>
            <Tree className="min-w-0 overflow-hidden">
              {sortedSections.map((section) => (
                <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                  {typeof section.content === "function" ? section.content() : section.content}
                </TreeSection>
              ))}

              {sortedSections.length === 0 && (
                <TreeSection label={t("details.noSelection")} defaultOpen={true}>
                  <TreeItem>
                    <p className="text-sm text-muted-foreground">{t("details.noSelectionMessage")}</p>
                  </TreeItem>
                </TreeSection>
              )}
            </Tree>
          </ScopedContent>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Details;
