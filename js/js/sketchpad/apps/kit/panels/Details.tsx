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
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.name"
              value={kit.name}
              onLazyChange={(value) => kitStore.change({ name: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.name")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.name")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.name")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.version"
              value={kit.version || ""}
              placeholder={t("semio.sketchpad.app.kit.versionPlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ version: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.version")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.version")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.version")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Textarea
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.description"
              value={kit.description || ""}
              placeholder={t("semio.sketchpad.app.kit.descriptionPlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ description: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.description")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.description")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.description")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.icon"
              value={kit.icon || ""}
              placeholder={t("semio.sketchpad.app.kit.iconPlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ icon: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.icon")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.icon")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.icon")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.image"
              value={kit.image || ""}
              placeholder={t("semio.sketchpad.app.kit.imagePlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ image: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.image")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.image")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.image")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.homepage"
              value={kit.homepage || ""}
              placeholder={t("semio.sketchpad.app.kit.homepagePlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ homepage: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.homepage")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.homepage")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.homepage")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.license"
              value={kit.license || ""}
              placeholder={t("semio.sketchpad.app.kit.licensePlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ license: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.license")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.license")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.kit.license")}
              showLabel
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
          <Input id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.type.panel.details.section.type.variant" value={type.variant || ""} placeholder={t("semio.sketchpad.app.type.variantPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.type.panel.details.section.type.description" value={type.description || ""} placeholder={t("semio.sketchpad.app.type.descriptionPlaceholder.label")} readOnly showLabel />
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
          <Input id="semio.sketchpad.app.design.panel.details.section.design.name" value={design.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.variant" value={design.variant || ""} placeholder={t("semio.sketchpad.app.design.variantPlaceholder")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.design.panel.details.section.design.description" value={design.description || ""} placeholder={t("semio.sketchpad.app.design.descriptionPlaceholder")} readOnly showLabel />
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

export const FileSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const kit = useKit() as Kit;
  const selection = kitApp?.selection;
  const selectedFiles = selection?.files || [];

  if (selectedFiles.length === 0) return null;

  const files = selectedFiles
    .map((filePath) => {
      return kit.files?.find((f) => f.path === filePath);
    })
    .filter(Boolean);

  if (files.length === 0) return null;

  const formatFileSize = (bytes?: number) => {
    if (!bytes) return "0 KB";
    return `${(bytes / 1024).toFixed(1)} KB`;
  };

  const formatDate = (date?: Date) => {
    if (!date) return "";
    const parsedDate = date instanceof Date ? date : new Date(date);
    if (isNaN(parsedDate.getTime())) return "";
    return parsedDate.toLocaleDateString();
  };

  return (
    <>
      {files.map((file) => (
        <TreeItem key={file!.guid}>
          <TreeContent>
            <div className="space-y-2">
              <div>
                <label className="text-xs text-muted-foreground">{t("semio.file.path")}</label>
                <p className="text-sm">{file!.path}</p>
              </div>
              <div>
                <label className="text-xs text-muted-foreground">{t("semio.file.size")}</label>
                <p className="text-sm">{formatFileSize(file!.size)}</p>
              </div>
              {file!.createdAt && (
                <div>
                  <label className="text-xs text-muted-foreground">{t("semio.file.created")}</label>
                  <p className="text-sm">{formatDate(file!.createdAt)}</p>
                </div>
              )}
              {file!.updatedAt && (
                <div>
                  <label className="text-xs text-muted-foreground">{t("semio.file.updated")}</label>
                  <p className="text-sm">{formatDate(file!.updatedAt)}</p>
                </div>
              )}
            </div>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

export const FolderSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const kit = useKit() as Kit;
  const kitStore = useKitStore() as any;
  const { startTransaction, finalizeTransaction, abortTransaction } = useKitAppCommands();
  const selection = kitApp?.selection;
  const selectedFolders = selection?.folders || [];

  if (selectedFolders.length === 0) return null;

  const folders = selectedFolders
    .map((folderGuid) => {
      return kit.folders?.find((f) => f.guid === folderGuid);
    })
    .filter(Boolean);

  if (folders.length === 0) return null;
  if (folders.length > 1) return null; // Show only single folder

  const folder = folders[0]!;

  const formatDate = (date?: Date) => {
    if (!date) return "";
    const parsedDate = date instanceof Date ? date : new Date(date);
    if (isNaN(parsedDate.getTime())) return "";
    return parsedDate.toLocaleDateString();
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.folder.name"
            value={folder.name}
            onLazyChange={(value) => {
              const folderStore = (kitStore as any).folder(folder.guid);
              folderStore.change({ name: value });
            }}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      {folder.description && (
        <TreeItem>
          <TreeContent>
            <Textarea
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.folder.description"
              value={folder.description || ""}
              placeholder={t("semio.sketchpad.app.folder.descriptionPlaceholder.label")}
              onLazyChange={(value) => {
                const folderStore = (kitStore as any).folder(folder.guid);
                folderStore.change({ description: value });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
      {folder.createdAt && (
        <TreeItem>
          <TreeContent>
            <div>
              <label className="text-xs text-muted-foreground">{t("semio.folder.created")}</label>
              <p className="text-sm">{formatDate(folder.createdAt)}</p>
            </div>
          </TreeContent>
        </TreeItem>
      )}
      {folder.updatedAt && (
        <TreeItem>
          <TreeContent>
            <div>
              <label className="text-xs text-muted-foreground">{t("semio.folder.updated")}</label>
              <p className="text-sm">{formatDate(folder.updatedAt)}</p>
            </div>
          </TreeContent>
        </TreeItem>
      )}
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
  const kinds: string[] = [];
  if (typesCount > 0) kinds.push(t("semio.sketchpad.app.kit.types.multipleTitle", { count: typesCount }));
  if (designsCount > 0) kinds.push(t("semio.sketchpad.app.kit.designs.multipleTitle", { count: designsCount }));
  if (qualitiesCount > 0) kinds.push(t("semio.sketchpad.app.kit.qualities.multipleTitle", { count: qualitiesCount }));
  if (filesCount > 0) kinds.push(t("semio.sketchpad.app.kit.files.multipleTitle", { count: filesCount }));
  if (authorsCount > 0) kinds.push(t("semio.sketchpad.app.kit.authors.multipleTitle", { count: authorsCount }));
  if (kinds.length <= 1) return null;
  return (
    <TreeItem>
      <TreeContent>
        <p className="text-sm text-muted-foreground">{kinds.join(", ")}</p>
      </TreeContent>
    </TreeItem>
  );
};
