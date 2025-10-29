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

import { FC, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../elements/input/Select";
import { Representation } from "../../../semio";
import { useSketchpadStore } from "../../store";
import { useTypeAppCommands } from "./store";

export const RepresentationDropdown: FC = () => {
  const { t } = useTranslation();
  const params = useParams();
  const sketchpadStore = useSketchpadStore();

  const kitGuid = params.kit;
  const typeGuid = params.type;

  // Create TypeAppId to pass to hooks that need it
  const typeAppId = useMemo(() => {
    if (!kitGuid || !typeGuid) return undefined;
    return { kit: kitGuid, type: typeGuid };
  }, [kitGuid, typeGuid]);

  // Get type and selectedRepresentationGuid from sketchpad store directly
  // Use a state selector to reactively track changes
  const type = useMemo(() => {
    if (!kitGuid || !typeGuid) return undefined;
    if (!sketchpadStore.hasKit(kitGuid)) return undefined;
    const kitStore = sketchpadStore.kit(kitGuid);
    if (!kitStore || !kitStore.hasType(typeGuid)) return undefined;
    const typeStore = kitStore.type(typeGuid);
    return typeStore?.snapshot();
  }, [sketchpadStore, kitGuid, typeGuid]);

  const selectedRepresentationGuid = useMemo(() => {
    if (!kitGuid || !typeGuid) return undefined;
    const typeAppStore = sketchpadStore.typeApp(kitGuid, typeGuid);
    return typeAppStore?.snapshot()?.selectedRepresentationGuid;
  }, [sketchpadStore, kitGuid, typeGuid]);

  // Subscribe to type changes to force re-render when representations change
  const [, forceUpdate] = useState({});
  useEffect(() => {
    if (!kitGuid || !typeGuid) return;
    if (!sketchpadStore.hasKit(kitGuid)) return;
    const kitStore = sketchpadStore.kit(kitGuid);
    if (!kitStore || !kitStore.hasType(typeGuid)) return;
    const typeStore = kitStore.type(typeGuid);
    if (!typeStore) return;

    // Use onChangedDeep to catch nested changes like representations array modifications
    const unsubscribe = typeStore.onChangedDeep(() => {
      forceUpdate({});
    });

    return () => {
      unsubscribe();
    };
  }, [sketchpadStore, kitGuid, typeGuid]);

  const { setSelectedRepresentation } = useTypeAppCommands(typeAppId);

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
