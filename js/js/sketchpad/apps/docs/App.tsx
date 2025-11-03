// #region Header

// App.tsx

// 2025 Ueli Saluz

// #endregion

import { FC, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { Canvas, Window } from "../../Canvas";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useAppType } from "../../store";
import PageCanvas from "./canvas/Page";
import { loadMDXFile, MDXModule } from "./mdx-loader";
import DocsDetails from "./panels/Details";
import DocsSettings from "./panels/Settings";
import DocsWorkbench from "./panels/Workbench";

const DocsApp: FC = () => {
  const { t } = useTranslation();
  const params = useParams();
  const pathParts = params["*"]?.split("/").filter(Boolean) || [];
  const fullPath = pathParts.join("/") || "index";
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  const [mdxModule, setMdxModule] = useState<MDXModule | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (appType !== "docs") return;

    const WorkbenchWrapper = () => <DocsWorkbench currentPath={fullPath} />;
    const DetailsWrapper = () => <DocsDetails />;
    const SettingsWrapper = () => <DocsSettings />;

    addSection("workbench", {
      id: "semio.sketchpad.app.docs.docs",
      order: 1,
      content: WorkbenchWrapper,
    });

    addSection("details", {
      id: "semio.sketchpad.app.docs.page",
      order: 1,
      content: DetailsWrapper,
    });

    addSection("settings", {
      id: "semio.sketchpad.app.docs.settings",
      order: 1,
      content: SettingsWrapper,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.docs.docs");
      removeSection("details", "semio.sketchpad.app.docs.page");
      removeSection("settings", "semio.sketchpad.app.docs.settings");
    };
  }, [appType, addSection, removeSection, fullPath]);

  useEffect(() => {
    const loadContent = async () => {
      setLoading(true);
      setError(null);
      try {
        const module = await loadMDXFile(fullPath);
        if (module) {
          setMdxModule(module);
        } else {
          setError(`Failed to load ${fullPath}`);
        }
      } catch (err) {
        setError((err as Error).message);
      } finally {
        setLoading(false);
      }
    };

    loadContent();
  }, [fullPath]);

  if (loading) {
    return (
      <Canvas>
        <Window id="page">
          <PageCanvas frontmatter={{ title: "Loading...", description: "" }} />
        </Window>
      </Canvas>
    );
  }

  if (error || !mdxModule) {
    return (
      <Canvas>
        <Window id="page">
          <PageCanvas frontmatter={{ title: "Error", description: error || "Content not found" }} />
        </Window>
      </Canvas>
    );
  }

  return (
    <Canvas>
      <Window id="page">
        <PageCanvas MDXContent={mdxModule.default} frontmatter={mdxModule.frontmatter} />
      </Window>
    </Canvas>
  );
};

export default DocsApp;
