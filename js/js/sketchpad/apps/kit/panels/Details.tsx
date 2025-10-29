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
import { Design, Kit, Type } from "../../../../semio";
import { useIsInKitScope, useKit, useKitStore } from "../../../store";
import { KitAppState, useKitApp, useKitAppCommands } from "../store";

export const KitSection: FC = () => {
  const isInKitScope = useIsInKitScope();
  if (!isInKitScope) return null;
  return <KitSectionForm />;
};

const KitSectionForm: FC = () => {
  const { t } = useTranslation();
  try {
    const kit = useKit() as Kit;
    if (!kit) {
      return (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.notAvailable")}</p>
          </TreeContent>
        </TreeItem>
      );
    }
    const kitStore = useKitStore() as any;
    const { startTransaction, finalizeTransaction, abortTransaction } = useKitAppCommands();
    return (
      <>
        <TreeItem>
          <TreeContent>
            <Input lazy label={t("semio.sketchpad.app.kit.name")} value={kit.name} onLazyChange={(value) => kitStore.change({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              label={t("semio.sketchpad.app.kit.version")}
              value={kit.version || ""}
              placeholder={t("semio.sketchpad.app.kit.versionPlaceholder")}
              onLazyChange={(value) => kitStore.change({ version: value })}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Textarea
              lazy
              label={t("semio.sketchpad.app.kit.description")}
              value={kit.description || ""}
              placeholder={t("semio.sketchpad.app.kit.descriptionPlaceholder")}
              onLazyChange={(value) => kitStore.change({ description: value })}
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
              label={t("semio.sketchpad.app.kit.icon")}
              value={kit.icon || ""}
              placeholder={t("semio.sketchpad.app.kit.iconPlaceholder")}
              onLazyChange={(value) => kitStore.change({ icon: value })}
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
              label={t("semio.sketchpad.app.kit.image")}
              value={kit.image || ""}
              placeholder={t("semio.sketchpad.app.kit.imagePlaceholder")}
              onLazyChange={(value) => kitStore.change({ image: value })}
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
              label={t("semio.sketchpad.app.kit.homepage")}
              value={kit.homepage || ""}
              placeholder={t("semio.sketchpad.app.kit.homepagePlaceholder")}
              onLazyChange={(value) => kitStore.change({ homepage: value })}
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
              label={t("semio.sketchpad.app.kit.license")}
              value={kit.license || ""}
              placeholder={t("semio.sketchpad.app.kit.licensePlaceholder")}
              onLazyChange={(value) => kitStore.change({ license: value })}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
            />
          </TreeContent>
        </TreeItem>
      </>
    );
  } catch (error) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }
};

export const TypeSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const selectedTypes = selection?.types || [];
  if (selectedTypes.length === 0) return null;
  if (selectedTypes.length === 1) return <SingleTypeSection typeGuid={selectedTypes[0]} />;
  return <MultipleTypesSection typeGuids={selectedTypes} />;
};

const SingleTypeSection: FC<{ typeGuid: string }> = ({ typeGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const type = kit?.types?.find((t) => t.guid === typeGuid);
  if (!type) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.sketchpad.app.type.name")} value={type.name} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.sketchpad.app.type.variant")} value={type.variant || ""} placeholder={t("semio.sketchpad.app.type.variantPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea label={t("semio.sketchpad.app.type.description")} value={type.description || ""} placeholder={t("semio.sketchpad.app.type.descriptionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleTypesSection: FC<{ typeGuids: string[] }> = ({ typeGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const types = typeGuids.map((guid) => kit?.types?.find((t) => t.guid === guid)).filter((t) => t !== undefined) as Type[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.types.multipleSelected", { count: types.length })}</p>
        </TreeContent>
      </TreeItem>
      {types.map((type) => (
        <TreeItem key={type.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{type.name}</p>
            {type.variant && <p className="text-xs text-muted-foreground">{type.variant}</p>}
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

export const DesignSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const selectedDesigns = selection?.designs || [];
  if (selectedDesigns.length === 0) return null;
  if (selectedDesigns.length === 1) return <SingleDesignSection designGuid={selectedDesigns[0]} />;
  return <MultipleDesignsSection designGuids={selectedDesigns} />;
};

const SingleDesignSection: FC<{ designGuid: string }> = ({ designGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const design = kit?.designs?.find((d) => d.guid === designGuid);
  if (!design) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.sketchpad.app.design.name")} value={design.name} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input label={t("semio.sketchpad.app.design.variant")} value={design.variant || ""} placeholder={t("semio.sketchpad.app.design.variantPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea label={t("semio.sketchpad.app.design.description")} value={design.description || ""} placeholder={t("semio.sketchpad.app.design.descriptionPlaceholder")} readOnly />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleDesignsSection: FC<{ designGuids: string[] }> = ({ designGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const designs = designGuids.map((guid) => kit?.designs?.find((d) => d.guid === guid)).filter((d) => d !== undefined) as Design[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.designs.multipleSelected", { count: designs.length })}</p>
        </TreeContent>
      </TreeItem>
      {designs.map((design) => (
        <TreeItem key={design.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{design.name}</p>
            {design.variant && <p className="text-xs text-muted-foreground">{design.variant}</p>}
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

export const MultipleArtifactsSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const typesCount = selection?.types?.length || 0;
  const designsCount = selection?.designs?.length || 0;
  const qualitiesCount = selection?.qualities?.length || 0;
  const filesCount = selection?.files?.length || 0;
  const authorsCount = selection?.authors?.length || 0;
  const totalCount = typesCount + designsCount + qualitiesCount + filesCount + authorsCount;
  const kinds: string[] = [];
  if (typesCount > 0) kinds.push(`${typesCount} ${t("semio.sketchpad.app.kit.types.title")}`);
  if (designsCount > 0) kinds.push(`${designsCount} ${t("semio.sketchpad.app.kit.designs.title")}`);
  if (qualitiesCount > 0) kinds.push(`${qualitiesCount} ${t("semio.qualities.title")}`);
  if (filesCount > 0) kinds.push(`${filesCount} ${t("semio.files.title")}`);
  if (authorsCount > 0) kinds.push(`${authorsCount} ${t("semio.authors.title")}`);
  if (kinds.length <= 1) return null;
  return (
    <TreeItem>
      <TreeContent>
        <p className="text-sm text-muted-foreground">{kinds.join(", ")}</p>
      </TreeContent>
    </TreeItem>
  );
};
