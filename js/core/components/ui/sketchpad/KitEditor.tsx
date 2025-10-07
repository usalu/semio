import { FC, useEffect } from "react";
import { Author, Design } from "../../../semio";
import { useKit, useKitCommands, useSketchpadCommands } from "../../../store";

import { Tambour } from "@semio/assets";
import { Button } from "../Button";
import { Input } from "../Input";
import { TreeItem } from "../Tree";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";

const KitEditor: FC = () => {
  const kit = useKit();
  const { createDesign, createType, createAuthor } = useKitCommands();
  const { createDesignEditor } = useSketchpadCommands();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    addSection("details", {
      id: "kit-editor-details",
      label: "Kit",
      order: 0,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>
            <Input label="Kit Name" value={kit.name} />
          </TreeItem>
          <TreeItem>
            <Input label="Version" value="1.0.0" />
          </TreeItem>
          <TreeItem>
            <Button onClick={onPopulateKit}>Populate Kit</Button>
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

  return <div></div>;
};

export default KitEditor;
