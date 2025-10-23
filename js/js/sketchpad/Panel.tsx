// #region Header

// Panel.tsx

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

import { FC, ReactNode } from "react";
import BasePanel from "../elements/panels/Panel";
import { PanelKey, usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";
import { useActiveInteraction, useIsMobile } from "./store";

type ResizeSide = "left" | "right";

interface PanelProps extends ResizablePanelProps {
  panelId: PanelKey;
  resizeSide?: ResizeSide;
  zIndex?: 20 | 30;
  showBackground?: boolean;
  minWidth?: number;
  maxWidth?: number;
  scopeWrapper?: FC<{ children: ReactNode }>;
  emptyMessage?: string;
  additionalSections?: ReactNode;
  footer?: ReactNode;
  hideActiveInteractionOpacity?: (activeInteraction: string | null) => boolean;
}

const Panel: FC<PanelProps> = ({
  panelId,
  visible,
  onWidthChange,
  width,
  resizeSide = "right",
  zIndex = 20,
  showBackground = true,
  minWidth = 150,
  maxWidth = 500,
  scopeWrapper: ScopeWrapper,
  emptyMessage,
  additionalSections,
  footer,
  hideActiveInteractionOpacity,
}) => {
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();
  const sections = usePanelSections(panelId);
  const shouldHideOpacity = hideActiveInteractionOpacity ? hideActiveInteractionOpacity(activeInteraction ?? null) : activeInteraction && !activeInteraction.startsWith(`${panelId}-`);
  const wrappedSections = ScopeWrapper
    ? sections.map((section) => ({
        ...section,
        content:
          typeof section.content === "function" ? (
            (() => {
              const ContentFn = section.content as () => ReactNode;
              return () => <ScopeWrapper>{ContentFn()}</ScopeWrapper>;
            })()
          ) : (
            <ScopeWrapper>{section.content}</ScopeWrapper>
          ),
      }))
    : sections;
  return (
    <BasePanel
      visible={visible}
      size={width}
      onSizeChange={onWidthChange}
      resizeSide={resizeSide}
      zIndex={zIndex}
      showBackground={showBackground}
      minSize={minWidth}
      maxSize={maxWidth}
      sections={wrappedSections}
      emptyMessage={emptyMessage}
      additionalContent={additionalSections}
      footer={footer}
      opacity={shouldHideOpacity ? 0.1 : 1}
      className={isMobile ? "p-2" : "p-1"}
    />
  );
};

export default Panel;
