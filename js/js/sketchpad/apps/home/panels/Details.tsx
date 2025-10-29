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

import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem } from "../../../../elements/aggregation/Tree";
import { Input } from "../../../../elements/input/Input";
import { Textarea } from "../../../../elements/input/Textarea";
import { KitShallow } from "../../../../semio";
import { useSketchpadStore } from "../../../store";
import { HomeState, useHome } from "../store";

export const KitSection: FC = () => {
  const { t } = useTranslation();
  const home = useHome() as HomeState;
  const selection = home?.selection;
  const selectedKits = selection?.kits || [];
  if (selectedKits.length === 0) return null;
  if (selectedKits.length === 1) return <SingleKitSection kitId={selectedKits[0]} />;
  return <MultipleKitsSection kitIds={selectedKits} />;
};

const SingleKitSection: FC<{ kitId: string }> = ({ kitId }) => {
  const { t } = useTranslation();
  const sketchpadStore = useSketchpadStore();
  const kitShallow = sketchpadStore.kitShallows().find((k) => k.guid === kitId);
  if (!kitShallow) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.kit.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.name")} value={kitShallow.name} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.version")} value={kitShallow.version || ""} placeholder={t("semio.kit.versionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea label={t("semio.kit.description")} value={kitShallow.description || ""} placeholder={t("semio.kit.descriptionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.icon")} value={kitShallow.icon || ""} placeholder={t("semio.kit.iconPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.image")} value={kitShallow.image || ""} placeholder={t("semio.kit.imagePlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleKitsSection: FC<{ kitIds: string[] }> = ({ kitIds }) => {
  const { t } = useTranslation();
  const sketchpadStore = useSketchpadStore();
  const kits = kitIds.map((id) => sketchpadStore.kitShallows().find((k) => k.guid === id)).filter((k) => k !== undefined) as KitShallow[];

  // Helper function to get common value or undefined if different
  const getCommonValue = <T,>(getter: (kit: KitShallow) => T): T | undefined => {
    if (kits.length === 0) return undefined;
    const firstValue = getter(kits[0]);
    const allSame = kits.every((kit) => getter(kit) === firstValue);
    return allSame ? firstValue : undefined;
  };

  const commonName = getCommonValue((k) => k.name);
  const commonVersion = getCommonValue((k) => k.version);
  const commonDescription = getCommonValue((k) => k.description);
  const commonIcon = getCommonValue((k) => k.icon);
  const commonImage = getCommonValue((k) => k.image);

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.name")} value={commonName || ""} placeholder={commonName === undefined ? t("semio.common.mixedValues") : undefined} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.version")} value={commonVersion || ""} placeholder={commonVersion === undefined ? t("semio.common.mixedValues") : t("semio.kit.versionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea label={t("semio.kit.description")} value={commonDescription || ""} placeholder={commonDescription === undefined ? t("semio.common.mixedValues") : t("semio.kit.descriptionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.icon")} value={commonIcon || ""} placeholder={commonIcon === undefined ? t("semio.common.mixedValues") : t("semio.kit.iconPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.kit.image")} value={commonImage || ""} placeholder={commonImage === undefined ? t("semio.common.mixedValues") : t("semio.kit.imagePlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
    </>
  );
};
