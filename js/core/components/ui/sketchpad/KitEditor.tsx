import { FC, useState } from "react";
import { Author, Design, DesignId, KitId } from "../../../semio";
import { DesignEditorId, useKitCommands, useSketchpadCommands } from "../../../store";

import { Tambour } from "@semio/assets";
import { Button } from "../Button";

const KitEditor: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const { createDesign, createType, createAuthor } = useKitCommands();
  const { createDesignEditor, setActiveDesignEditor } = useSketchpadCommands();

  const onPopulateKit = async () => {
    const defaultKitId: KitId = { name: "Metabolism", version: "r25.07-1" };
    const author: Author = { name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };
    const design: Design = {
      name: "Nakagin Capsule Tower",
      unit: "m",
      variant: "",
      view: "",
      pieces: [
        {
          id_: "p1",
          center: { x: 0, y: 0 },
          type: { name: "Tambour", variant: "" },
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
    await setActiveDesignEditor({ kit: defaultKitId, design: defaultDesignId } as DesignEditorId);
    setIsImporting(false);
  };

  if (isImporting) return null;

  return <Button onClick={onPopulateKit}>Populate Kit</Button>;
};

export default KitEditor;
