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
import { Quality } from "../../../../semio";
import { useQuality } from "../../../kits/store";
import { useQualityAppCommands } from "../store";

export const QualityDetails: FC = () => {
  const { t } = useTranslation();
  const quality = useQuality(undefined, undefined, true) as Quality | undefined;
  const { updateFormula } = useQualityAppCommands();

  if (!quality) return null;

  return (
    <>
      <TreeItem label={t("semio.quality.key")}>
        <TreeContent>
          <Input value={quality.key ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.name")}>
        <TreeContent>
          <Input value={quality.name ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.description")}>
        <TreeContent>
          <Textarea value={quality.description ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.formula")}>
        <TreeContent>
          <Textarea value={quality.formula ?? ""} onChange={(e) => updateFormula(e.target.value)} className="w-full font-mono text-xs" rows={5} placeholder={t("semio.quality.formulaPlaceholder")} />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.defaultSiUnit")}>
        <TreeContent>
          <Input value={quality.defaultSiUnit ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.defaultImperialUnit")}>
        <TreeContent>
          <Input value={quality.defaultImperialUnit ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.kind")}>
        <TreeContent>
          <Input type="number" value={quality.kind?.toString() ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.canScale")}>
        <TreeContent>
          <Input type="checkbox" checked={quality.canScale ?? false} disabled className="h-4 w-4" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.defaultValue")}>
        <TreeContent>
          <Input type="number" value={quality.defaultValue?.toString() ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.min")}>
        <TreeContent>
          <Input type="number" value={quality.min?.toString() ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.isMinExcluded")}>
        <TreeContent>
          <Input type="checkbox" checked={quality.isMinExcluded ?? false} disabled className="h-4 w-4" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.max")}>
        <TreeContent>
          <Input type="number" value={quality.max?.toString() ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.quality.isMaxExcluded")}>
        <TreeContent>
          <Input type="checkbox" checked={quality.isMaxExcluded ?? false} disabled className="h-4 w-4" />
        </TreeContent>
      </TreeItem>
    </>
  );
};
