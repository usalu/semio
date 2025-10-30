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
      <TreeItem label={t("semio.sketchpad.app.quality.key")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.key" value={quality.key ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.name")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.name" value={quality.name ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.description")}>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.quality.panel.details.description" value={quality.description ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.formula")}>
        <TreeContent>
          <Textarea
            id="semio.sketchpad.app.quality.panel.details.formula"
            value={quality.formula ?? ""}
            onChange={(e) => updateFormula("semio.sketchpad.app.quality.panel.details.formula", e.target.value)}
            className="w-full font-mono text-xs"
            rows={5}
            placeholder={t("semio.sketchpad.app.quality.formulaPlaceholder")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.defaultSiUnit")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultSiUnit" value={quality.defaultSiUnit ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.defaultImperialUnit")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultImperialUnit" value={quality.defaultImperialUnit ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.kind")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.kind" type="number" value={quality.kind?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.canScale")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.canScale" type="checkbox" checked={quality.canScale ?? false} disabled className="h-4 w-4" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.defaultValue")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.defaultValue" type="number" value={quality.defaultValue?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.min")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.min" type="number" value={quality.min?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.isMinExcluded")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.isMinExcluded" type="checkbox" checked={quality.isMinExcluded ?? false} disabled className="h-4 w-4" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.max")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.max" type="number" value={quality.max?.toString() ?? ""} readOnly className="w-full" showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("semio.sketchpad.app.quality.isMaxExcluded")}>
        <TreeContent>
          <Input id="semio.sketchpad.app.quality.panel.details.isMaxExcluded" type="checkbox" checked={quality.isMaxExcluded ?? false} disabled className="h-4 w-4" showLabel />
        </TreeContent>
      </TreeItem>
    </>
  );
};
