// #region Header

// TypeEditor.tsx

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

import { FC, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../../elements/aggregation/Tree";
import { Input } from "../../../elements/input/Input";
import { Textarea } from "../../../elements/input/Textarea";
import { Type } from "../../../semio";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, useEditorType, useIsInTypeScope, useKitCommands, useType, useTypeEditorCommands } from "../../store";
import TypeScene from "./Scene";

const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();

  if (!isInTypeScope) {
    return null;
  }

  return <TypeDetailsForm />;
};

const TypeDetailsForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  return (
    <TreeSection label={t("type.title")} defaultOpen={true}>
      <TreeItem>
        <TreeContent>
          <Input lazy label={t("type.name")} value={type.name} onLazyChange={(value) => updateTypeField({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            label={t("type.description")}
            value={type.description || ""}
            placeholder={t("type.descriptionPlaceholder")}
            onLazyChange={(value) => updateTypeField({ description: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.icon")}
            value={type.icon || ""}
            placeholder={t("type.iconPlaceholder")}
            onLazyChange={(value) => updateTypeField({ icon: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.image")}
            value={type.image || ""}
            placeholder={t("type.imagePlaceholder")}
            onLazyChange={(value) => updateTypeField({ image: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.variant")}
            value={type.variant || ""}
            placeholder={t("type.variantPlaceholder")}
            onLazyChange={(value) => updateTypeField({ variant: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input lazy label={t("type.unit")} value={type.unit} onLazyChange={(value) => updateTypeField({ unit: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
          </TreeContent>
        </TreeItem>
      )}
    </TreeSection>
  );
};

const Editor: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

  useEffect(() => {
    if (editorType !== EditorType.TYPE) return;

    addSection("details", {
      id: "type",
      label: "Type",
      order: 0,
      defaultOpen: true,
      content: () => <TypeDetails />,
    });
    return () => {
      removeSection("details", "type");
    };
  }, [addSection, removeSection, editorType]);

  return <TypeScene />;
};

export default Editor;
