import { FC } from "react";
import { useTranslation } from "react-i18next";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
import { Input } from "../../../../elements/input/Input";
import { Textarea } from "../../../../elements/input/Textarea";
import { Kit } from "../../../../semio";
import { useIsInKitScope, useKit, useKitStore } from "../../../store";
import { useKitEditorCommands } from "../store";

export const KitDetails: FC = () => {
  const isInKitScope = useIsInKitScope();
  if (!isInKitScope) return null;
  return <KitDetailsForm />;
};

const KitDetailsForm: FC = () => {
  const { t } = useTranslation();

  try {
    const kit = useKit() as Kit;

    if (!kit) {
      return (
        <TreeSection label={t("kit.title")} defaultOpen={true}>
          <TreeItem>
            <TreeContent>
              <p className="text-sm text-muted-foreground">{t("kit.notAvailable")}</p>
            </TreeContent>
          </TreeItem>
        </TreeSection>
      );
    }

    const kitStore = useKitStore() as any;
    const { startTransaction, finalizeTransaction, abortTransaction } = useKitEditorCommands();

    return (
      <TreeSection label={t("kit.title")} defaultOpen={true}>
        <TreeItem>
          <TreeContent>
            <Input lazy label={t("kit.name")} value={kit.name} onLazyChange={(value) => kitStore.change({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              label={t("kit.version")}
              value={kit.version || ""}
              placeholder={t("kit.versionPlaceholder")}
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
              label={t("kit.description")}
              value={kit.description || ""}
              placeholder={t("kit.descriptionPlaceholder")}
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
              label={t("kit.icon")}
              value={kit.icon || ""}
              placeholder={t("kit.iconPlaceholder")}
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
              label={t("kit.image")}
              value={kit.image || ""}
              placeholder={t("kit.imagePlaceholder")}
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
              label={t("kit.homepage")}
              value={kit.homepage || ""}
              placeholder={t("kit.homepagePlaceholder")}
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
              label={t("kit.license")}
              value={kit.license || ""}
              placeholder={t("kit.licensePlaceholder")}
              onLazyChange={(value) => kitStore.change({ license: value })}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
            />
          </TreeContent>
        </TreeItem>
      </TreeSection>
    );
  } catch (error) {
    console.error("Error rendering kit details:", error);
    return (
      <TreeSection label={t("kit.title")} defaultOpen={true}>
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("kit.notFound")}</p>
          </TreeContent>
        </TreeItem>
      </TreeSection>
    );
  }
};
