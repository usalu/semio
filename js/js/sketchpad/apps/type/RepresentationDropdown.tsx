// #region Header

// RepresentationDropdown.tsx

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

import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../elements/input/Select";
import { Kit, Representation, Type } from "../../../semio";
import { useKit } from "../../kits/store";
import { useTypeAppCommands, useTypeAppSelectedRepresentationGuid } from "./store";

export const RepresentationDropdown: FC = () => {
  const { t } = useTranslation();
  const params = useParams();
  const kit = useKit() as Kit | undefined;
  const typeGuid = params.type;
  
  // Get type from kit directly instead of using useType() which requires TypeScopeProvider
  const type = useMemo(() => {
    if (!kit || !typeGuid) return undefined;
    return kit.types?.find((t: Type) => t.guid === typeGuid);
  }, [kit, typeGuid]);
  
  const selectedRepresentationGuid = useTypeAppSelectedRepresentationGuid();
  const { setSelectedRepresentation } = useTypeAppCommands();

  const representations = useMemo(() => type?.representations ?? [], [type]);

  const currentValue = useMemo(() => {
    if (!selectedRepresentationGuid && representations.length > 0) {
      return representations[0].guid;
    }
    return selectedRepresentationGuid ?? "";
  }, [selectedRepresentationGuid, representations]);

  const handleValueChange = (value: string) => {
    if (setSelectedRepresentation) {
      setSelectedRepresentation(value);
    }
  };

  if (representations.length === 0) {
    return <div className="text-xs text-disabled">{t("semio.sketchpad.app.type.noRepresentations")}</div>;
  }

  return (
    <Select value={currentValue} onValueChange={handleValueChange}>
      <SelectTrigger className="h-full border-0 rounded-none text-xs min-w-[200px]">
        <SelectValue placeholder={t("semio.sketchpad.app.type.selectRepresentation")} />
      </SelectTrigger>
      <SelectContent>
        {representations.map((representation: Representation) => (
          <SelectItem key={representation.guid} value={representation.guid} className="text-xs">
            {representation.description || representation.tags?.join(", ") || representation.guid.substring(0, 8)}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};
