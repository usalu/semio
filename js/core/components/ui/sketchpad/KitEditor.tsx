import { FC, useEffect, useState } from "react";
import { Design, DesignId } from "../../../semio";
import { DesignEditorId, DesignScopeProvider, useKit, useKitCommands, useSketchpadCommands } from "../../../store";
import DesignEditor from "./DesignEditor";

import { default as Tambour } from "../../../../../assets/semio/type_tambour.json";

const KitEditor: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const { createDesign, createType } = useKitCommands();
  const { createDesignEditor, setActiveDesignEditor } = useSketchpadCommands();
  const kit = useKit();

  const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };
  const design: Design = {
    name: "Nakagin Capsule Tower",
    variant: "",
    view: "",
    pieces: [
      {
        id_: "p1",
        center: { x: 0, y: 0 },
        type: { name: "Tambour", variant: "" },
        plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
        hidden: false,
        locked: false,
        attributes: [],
      },
    ],
    connections: [],
  };

  useEffect(() => {
    let mounted = true;
    (async () => {
      await createType(Tambour);
      await createDesign(design);
      await createDesignEditor({ kit, design: defaultDesignId } as DesignEditorId);
      await setActiveDesignEditor({ kit, design: defaultDesignId } as DesignEditorId);
      setIsImporting(false);
    })();
    return () => {
      mounted = false;
    };
  }, []); // TODO add store to dependencies after debugging

  if (isImporting) return null;

  return (
    <DesignScopeProvider id={defaultDesignId}>
      <DesignEditor />
    </DesignScopeProvider>
  );
};

export default KitEditor;
