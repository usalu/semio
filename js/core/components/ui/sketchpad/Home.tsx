import { FC, useEffect } from "react";
import { useKits, useSketchpadCommands } from "../../../store";
import { Button } from "../Button";
import { Input } from "../Input";
import { TreeContent, TreeItem } from "../Tree";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";

const Home: FC = ({}) => {
  const kits = useKits();
  const { createKit } = useSketchpadCommands();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    addSection("chat", {
      id: "home-chat",
      label: "Welcome",
      order: 0,
      defaultOpen: true,
      content: (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">Welcome to Semio! Start by creating a kit or opening an existing one.</p>
          </TreeContent>
        </TreeItem>
      ),
    });

    return () => {
      removeSection("chat", "home-chat");
    };
  }, [addSection, removeSection]);

  useEffect(() => {
    addSection("settings", {
      id: "home-settings",
      label: "Home",
      order: 0,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>
            <TreeContent>
              <Input label="Kit Name" value="Metabolism" disabled />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input label="Version" value="1.0.0" disabled />
            </TreeContent>
          </TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "home-settings");
    };
  }, [addSection, removeSection]);

  const onCreateKit = async () => {
    await createKit({ name: "New Kit", version: "1.0.0" });
  };

  return (
    <div>
      <Button onClick={onCreateKit}>Create Kit</Button>
      {kits.map((kit) => (
        <div key={kit.name}>{kit.name}</div>
      ))}
    </div>
  );
};

export default Home;
