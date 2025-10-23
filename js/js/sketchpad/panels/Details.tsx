// #region Header

// Details.tsx

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
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { TreeContent, TreeItem, TreeSection } from "../../elements/aggregation/Tree";
import { usePanelSections } from "../Navbar";
import Panel from "../Panel";
import { ResizablePanelProps } from "../Sketchpad";
import { DesignScopeProvider, KitScopeProvider, QualityScopeProvider, TypeScopeProvider, useNavigation } from "../store";

interface DetailsProps extends ResizablePanelProps {}

const ScopedContent: FC<{ children: ReactNode }> = ({ children }) => {
  const params = useParams();
  const navigation = useNavigation();
  const match = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kit = params.kit ?? match?.[1];
  const design = params.design ?? (match?.[2] === "designs" ? match?.[3] : undefined);
  const type = params.type ?? (match?.[2] === "types" ? match?.[3] : undefined);
  const quality = params.quality ?? (match?.[2] === "qualities" ? match?.[3] : undefined);
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
  if (quality && kit) {
    return (
      <KitScopeProvider guid={kit}>
        <QualityScopeProvider guid={quality}>{children}</QualityScopeProvider>
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
  const sections = usePanelSections("details");

  return (
    <Panel
      panelId="details"
      visible={visible}
      onWidthChange={onWidthChange}
      width={width}
      resizeSide="left"
      scopeWrapper={ScopedContent}
      additionalSections={
        sections.length === 0 ? (
          <TreeSection label={t("details.noSelection")} defaultOpen={true}>
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">{t("details.noSelectionMessage")}</p>
              </TreeContent>
            </TreeItem>
          </TreeSection>
        ) : undefined
      }
    />
  );
};

export default Details;
