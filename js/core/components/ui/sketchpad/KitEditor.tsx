import { FC, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Author, Design } from "../../../semio";
import { useKit, useKitCommands, useSketchpadCommands } from "../../../store";

import { Tambour } from "@semio/assets";
import { Button } from "../Button";
import { Input } from "../Input";
import { ScrollArea } from "../ScrollArea";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import { TreeItem } from "../Tree";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";

const KitEditor: FC = () => {
  const { t } = useTranslation();
  const kit = useKit();
  const { createDesign, createType, createAuthor } = useKitCommands();
  const { createDesignEditor } = useSketchpadCommands();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    addSection("details", {
      id: "kit-editor-details",
      label: t("kit.name"),
      order: 0,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>
            <Input label={t("kit.name")} value={kit.name} />
          </TreeItem>
          <TreeItem>
            <Input label={t("kit.version")} value="1.0.0" />
          </TreeItem>
          <TreeItem>
            <Button onClick={onPopulateKit}>{t("common.create")}</Button>
          </TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("details", "kit-editor-details");
    };
  }, [kit, addSection, removeSection]);

  const onPopulateKit = async () => {
    const author: Author = { guid: "10000000-0000-0000-0000-000000000000", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const design: Design = {
      guid: "50000000-0000-0000-0000-000000000000",
      name: "Nakagin Capsule Tower",
      unit: "m",
      variant: "",
      view: "",
      pieces: [
        {
          guid: "30000000-0000-0000-0000-000000000000",
          id_: "p1",
          center: { x: 0, y: 0 },
          type: "40000000-0000-0000-0000-000000000000",
          plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
          isHidden: false,
          isLocked: false,
          attributes: [],
        },
      ],
      connections: [],
    };
    await createAuthor(author);
    await createType(Tambour);
    await createDesign(design);
    await createDesignEditor({ kit: defaultKitId, design: defaultDesignId } as DesignEditorId);
  };

  return (
    <>
      <ScrollArea orientation="horizontal">
        <ToggleGroup type="multiple" value={kit.name as string[]} onValueChange={(value) => setKitName(value)}>
          <ToggleGroupItem value="designs">{t("kit.designs")}</ToggleGroupItem>
          <ToggleGroupItem value="types">{t("kit.types")}</ToggleGroupItem>
          <ToggleGroupItem value="authors">{t("kit.authors")}</ToggleGroupItem>
        </ToggleGroup>
      </ScrollArea>
      <ScrollArea orientation="horizontal">
        <ToggleGroup type="multiple" value={kit.name as string[]} onValueChange={(value) => setKitName(value)}>
          <ToggleGroupItem value="designs">{t("kit.designs")}</ToggleGroupItem>
          <ToggleGroupItem value="types">{t("kit.types")}</ToggleGroupItem>
          <ToggleGroupItem value="authors">{t("kit.authors")}</ToggleGroupItem>
        </ToggleGroup>
      </ScrollArea>
      <Input />
    </>
  );
};

export default KitEditor;
