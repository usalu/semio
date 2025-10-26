// #region Header

// Workbench.tsx

// 2025 Ueli Saluz

// #endregion

import { FileText, Folder } from "lucide-react";
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
    for (let i = 0; i < pathParts.length; i++) {
      const part = pathParts[i];
      if (!current.children.has(part)) {
        current.children.set(part, { children: new Map(), name: part });
      }
      current = current.children.get(part)!;
      if (i === pathParts.length - 1) {
        current.page = page;
      }
    }
  }
  return root;
}

function renderTreeNode(node: PageTreeNode, navigate: (path: string) => void, selectPage: (section: string, page: string) => void, section: string, currentPath?: string): React.ReactElement[] {
  const items: React.ReactElement[] = [];
  if (node.page) {
    const isCurrentPage = !!(currentPath && node.page.path === `docs/${currentPath}`);
    items.push(
      <TreeItem
        key={node.page.path}
        label={node.page.title}
        icon={<FileText className="w-3 h-3" />}
        isHighlighted={isCurrentPage}
        onClick={() => {
          selectPage(section, node.page!.path);
          navigate(`/${node.page!.path}`);
        }}
      />,
    );
  }
  if (node.children.size > 0) {
    Array.from(node.children.entries()).forEach(([name, childNode]) => {
      if (childNode.children.size > 0 && !childNode.page) {
        const folderLabel = name
          .split("-")
          .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
          .join(" ");
        items.push(
          <TreeItem key={name} label={folderLabel} icon={<Folder className="w-3 h-3" />} defaultOpen={false}>
            {renderTreeNode(childNode, navigate, selectPage, section, currentPath)}
          </TreeItem>,
        );
      } else {
        items.push(...renderTreeNode(childNode, navigate, selectPage, section, currentPath));
      }
    });
  }
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
        const displayLabel = section.emoji ? `${section.emoji} ${section.label}` : section.label;

        return (
          <TreeSection key={section.id} label={displayLabel} defaultOpen={true}>
            {renderTreeNode(tree, navigate, () => {}, section.id, currentPath)}
            {pages.length === 0 && (
              <TreeItem>
                <TreeContent>
                  <p className="text-sm text-muted-foreground">{section.description || t("docs.noPages")}</p>
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
