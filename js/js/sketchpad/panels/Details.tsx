// #region Header

// Details.tsx

// Generalized details panel that displays context-specific information based on the active editor and selection.
// Editors can register details sections dynamically that update based on user selections.
//
// Architecture:
// - Wraps content in scope providers (Kit, Design, Type) based on route parameters
// - Editors register sections via `useAddPanelSection("details", {...})`
// - Common pattern: editors register different sections based on selection state
//   (e.g., design editor shows "Design" when nothing selected, "Pieces" when pieces selected)
// - Sections are sorted by order and rendered as TreeSections
// - Shows a placeholder message when no sections are registered

// #endregion

import { FC, ReactNode, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";

import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeContent, TreeItem, TreeSection } from "../../elements/aggregation/Tree";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { DesignScopeProvider, KitScopeProvider, TypeScopeProvider, useActiveInteraction, useIsMobile } from "../store";

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
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();

  const sections = usePanelSections("details");

  if (!visible) return null;

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
      className={`h-full z-20 bg-panel text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px`, opacity: activeInteraction && !activeInteraction.startsWith("details-") ? 0.1 : 1, transition: "opacity 150ms" }}
    >
      <ScrollArea className="h-full">
        <div className={`${isMobile ? "p-2" : "p-1"} overflow-hidden min-w-0`}>
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
                    <TreeContent>
                      <p className="text-sm text-muted-foreground">{t("details.noSelectionMessage")}</p>
                    </TreeContent>
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
