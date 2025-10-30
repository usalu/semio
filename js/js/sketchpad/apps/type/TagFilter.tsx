// #region Header

// TagFilter.tsx

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

import { X } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { Button } from "../../../elements/input/Button";
import { getAllTagsFromRepresentations, getAvailableTagsForRepresentations } from "../../../semio";
import { useSketchpadStore, useSync } from "../../store";
import { useTypeAppCommands, useTypeAppSelectedRepresentationTags } from "./store";

export const TagFilter: FC = () => {
  const { t } = useTranslation();
  const params = useParams();

  const kitGuid = params.kit;
  const typeGuid = params.type;

  const typeAppId = useMemo(() => {
    if (!kitGuid || !typeGuid) return undefined;
    return { kit: kitGuid, type: typeGuid };
  }, [kitGuid, typeGuid]);

  const sketchpadStore = useSketchpadStore();
  const typeStore = useMemo(() => {
    if (!kitGuid || !typeGuid) return null;
    if (!sketchpadStore.hasKit(kitGuid)) return null;
    const kitStore = sketchpadStore.kit(kitGuid);
    if (!kitStore.hasType(typeGuid)) return null;
    return kitStore.type(typeGuid);
  }, [sketchpadStore, kitGuid, typeGuid]);

  const type = useSync(typeStore ?? null, (t) => t, true);
  const selectedTags = useTypeAppSelectedRepresentationTags();
  const { addRepresentationTag, removeRepresentationTag, clearRepresentationTags } = useTypeAppCommands(typeAppId);

  const representations = useMemo(() => type?.representations ?? [], [type]);

  const allTags = useMemo(() => getAllTagsFromRepresentations(representations), [representations]);

  const availableTags = useMemo(() => getAvailableTagsForRepresentations(representations, selectedTags), [representations, selectedTags]);

  if (allTags.length === 0) {
    return <div className="text-xs text-disabled px-2">{t("semio.sketchpad.app.type.noTags")}</div>;
  }

  return (
    <div className="flex items-center gap-1 px-2 h-full flex-wrap">
      {selectedTags.map((tag) => (
        <Button
          key={tag}
          id={`semio.sketchpad.app.type.footer.tagFilter.selected.${tag}`}
          variant="secondary"
          className="h-6 gap-1 text-xs px-2"
          onClick={() => removeRepresentationTag("semio.sketchpad.app.type.footer.tagFilter.remove", tag)}
        >
          {tag}
          <X className="h-3 w-3" />
        </Button>
      ))}
      {selectedTags.length > 0 && (
        <Button
          id="semio.sketchpad.app.type.footer.tagFilter.clear"
          variant="ghost"
          className="h-6 text-xs px-2"
          onClick={() => clearRepresentationTags("semio.sketchpad.app.type.footer.tagFilter.clear")}
        >
          {t("semio.sketchpad.app.type.clearTags")}
        </Button>
      )}
      {availableTags.length > 0 && (
        <>
          {selectedTags.length > 0 && <div className="w-px h-4 bg-border" />}
          {availableTags.map((tag) => (
            <Button
              key={tag}
              id={`semio.sketchpad.app.type.footer.tagFilter.available.${tag}`}
              variant="ghost"
              className="h-6 text-xs px-2"
              onClick={() => addRepresentationTag("semio.sketchpad.app.type.footer.tagFilter.add", tag)}
            >
              {tag}
            </Button>
          ))}
        </>
      )}
    </div>
  );
};
