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
      id: "docs-navigation",
      label: t("docs.docs", "Docs"),
      order: 1,
      defaultOpen: true,
      content: WorkbenchWrapper,
    });

    addSection("details", {
      id: "docs-toc",
      label: t("docs.page", "Page"),
      order: 1,
      defaultOpen: true,
      content: DetailsWrapper,
    });

    addSection("settings", {
      id: "docs-settings",
      label: t("docs.settings", "Settings"),
      order: 1,
      defaultOpen: true,
      content: SettingsWrapper,
    });

    return () => {
      removeSection("workbench", "docs-navigation");
      removeSection("details", "docs-toc");
      removeSection("settings", "docs-settings");
    };
  }, [appType, addSection, removeSection, t, fullPath]);

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
