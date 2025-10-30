// #region Header

// Workbench.tsx

// 2025 Ueli Saluz

// #endregion

import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
import { DocsPage, docsRegistry } from "../registry";

interface PageTreeNode {
  page?: DocsPage;
  children: Map<string, PageTreeNode>;
  name: string;
}

function buildTree(pages: DocsPage[], sectionId: string): PageTreeNode {
  const root: PageTreeNode = { children: new Map(), name: "root" };
  for (const page of pages) {
    const pathParts = page.path.replace("docs/", "").replace(`${sectionId}/`, "").split("/");
    let current = root;

    // Check if the last part is "index" - if so, assign page to parent folder
    const isIndexFile = pathParts[pathParts.length - 1] === "index";
    const partsToTraverse = isIndexFile ? pathParts.slice(0, -1) : pathParts;

    for (let i = 0; i < partsToTraverse.length; i++) {
      const part = partsToTraverse[i];
      if (!current.children.has(part)) {
        current.children.set(part, { children: new Map(), name: part });
      }
      current = current.children.get(part)!;
    }

    // Assign the page to the current node (either the file node or the folder node for index files)
    current.page = page;
  }
  return root;
}

function renderTreeNode(node: PageTreeNode, navigate: (path: string) => void, selectPage: (section: string, page: string) => void, section: string, currentPath?: string): React.ReactElement[] {
  const items: React.ReactElement[] = [];

  // Process each child node
  Array.from(node.children.entries()).forEach(([name, childNode]) => {
    const hasChildren = childNode.children.size > 0;
    const hasPage = !!childNode.page;

    if (hasChildren) {
      // This is a folder (with or without an index page)
      const folderLabel = childNode.page?.title || name
        .split("-")
        .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
        .join(" ");

      const isCurrentPage = !!(childNode.page && currentPath && childNode.page.path === `docs/${currentPath}`);

      items.push(
        <TreeItem
          key={childNode.page?.path || name}
          label={folderLabel}
          icon={<span className="text-sm">📁</span>}
          defaultOpen={false}
          isHighlighted={isCurrentPage}
          onClick={childNode.page ? () => {
            selectPage(section, childNode.page!.path);
            navigate(`/${childNode.page!.path}`);
          } : undefined}
        >
          {renderTreeNode(childNode, navigate, selectPage, section, currentPath)}
        </TreeItem>,
      );
    } else if (hasPage) {
      // This is a leaf page (no children)
      const isCurrentPage = !!(currentPath && childNode.page.path === `docs/${currentPath}`);
      items.push(
        <TreeItem
          key={childNode.page.path}
          label={childNode.page.title}
          isHighlighted={isCurrentPage}
          onClick={() => {
            selectPage(section, childNode.page!.path);
            navigate(`/${childNode.page!.path}`);
          }}
        />,
      );
    }
  });

  return items;
}

interface WorkbenchProps {
  currentPath?: string;
}

const Workbench: FC<WorkbenchProps> = ({ currentPath }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const sections = docsRegistry.getAllSections();

  return (
    <>
      {sections.map((section) => {
        const pages = docsRegistry.getPagesBySection(section.id);
        const tree = buildTree(pages, section.id);

        return (
          <TreeSection
            key={section.id}
            label={section.label}
            icon={section.icon ? <span aria-hidden="true">{section.icon}</span> : undefined}
            defaultOpen={true}
          >
            {renderTreeNode(tree, navigate, () => {}, section.id, currentPath)}
            {pages.length === 0 && (
              <TreeItem>
                <TreeContent>
                  <p className="text-sm text-muted-foreground">{section.description || t("semio.sketchpad.app.docs.noPages")}</p>
                </TreeContent>
              </TreeItem>
            )}
          </TreeSection>
        );
      })}
    </>
  );
};

export default Workbench;
