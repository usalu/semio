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
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
import { Input } from "../../../../elements/input/Input";
import { Textarea } from "../../../../elements/input/Textarea";
import { Quality } from "../../../../semio";
import { useQuality } from "../../../kits/store";
import { useQualityEditorCommands } from "../store";

export const QualityDetails: FC = () => {
  const { t } = useTranslation();
  const quality = useQuality() as Quality | undefined;
  const { updateFormula } = useQualityEditorCommands();

  if (!quality) return null;

  return (
    <TreeSection label={t("quality.title")} defaultOpen={true}>
      <TreeItem label={t("quality.key")}>
        <TreeContent>
          <Input value={quality.key ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("quality.name")}>
        <TreeContent>
          <Input value={quality.name ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("quality.description")}>
        <TreeContent>
          <Textarea value={quality.description ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("quality.formula")}>
        <TreeContent>
          <Textarea value={quality.formula ?? ""} onChange={(e) => updateFormula(e.target.value)} className="w-full font-mono text-xs" rows={5} placeholder={t("quality.formulaPlaceholder")} />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("quality.defaultSiUnit")}>
        <TreeContent>
          <Input value={quality.defaultSiUnit ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("quality.defaultImperialUnit")}>
        <TreeContent>
          <Input value={quality.defaultImperialUnit ?? ""} readOnly className="w-full" />
        </TreeContent>
      </TreeItem>
    </TreeSection>
  );
};
