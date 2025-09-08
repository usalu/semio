import { FC, useEffect, useState } from "react";
import { Design, DesignId } from "../../../semio";
import { DesignEditorId, DesignScopeProvider, useKit, useKitCommands, useSketchpadCommands } from "../../../store";
import DesignEditor from "./DesignEditor";

const KitEditor: FC = () => {
  const [isImporting, setIsImporting] = useState<boolean>(true);
  const { createDesign } = useKitCommands();
  const { createDesignEditor, setActiveDesignEditor } = useSketchpadCommands();
  const kit = useKit();

  const defaultDesignId: DesignId = { name: "Nakagin Capsule Tower", variant: "", view: "" };

  useEffect(() => {
    let mounted = true;
    (async () => {
      await createDesign(defaultDesignId as Design);
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
