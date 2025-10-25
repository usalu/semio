// #region Header

// Editor.tsx

// 2025 Ueli Saluz

// #endregion

import { FC } from "react";
import { useParams } from "react-router";
import { Canvas, Window } from "../../Canvas";
import PageCanvas from "./canvas/Page";

const DocsEditor: FC = () => {
  const params = useParams();
  const pathParts = params["*"]?.split("/").filter(Boolean) || [];
  const section = pathParts[0];
  const pagePath = pathParts.slice(1).join("/");

  return (
    <Canvas>
      <Window id="page">
        <PageCanvas
          frontmatter={{
            title: `${section || "Docs"} ${pagePath ? `/ ${pagePath}` : ""}`,
            description: "Documentation is being migrated to Sketchpad",
          }}
          content={`
            <div class="p-4">
              <h2>Section: ${section || "none"}</h2>
              <h3>Page: ${pagePath || "index"}</h3>
              <p>The docs are being migrated from Astro Starlight to an integrated Sketchpad system.</p>
              <p>Path: docs/${pathParts.join("/")}</p>
              <h4>Next Steps:</h4>
              <ol>
                <li>Set up MDX processing pipeline</li>
                <li>Load MDX files from file system</li>
                <li>Wire up Workbench and Details panels</li>
                <li>Implement search and navigation</li>
                <li>Migrate existing docs</li>
              </ol>
            </div>
          `}
        />
      </Window>
    </Canvas>
  );
};

const DocsEditorWithProvider: FC = () => {
  return <DocsEditor />;
};

export default DocsEditorWithProvider;
