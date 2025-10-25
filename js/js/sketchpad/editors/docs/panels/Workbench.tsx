// #region Header

// Workbench.tsx

// 2025 Ueli Saluz

// #endregion

import { BookOpen, CheckCircle, FileText, Folder, GraduationCap, Lightbulb, Puzzle, Rocket, Star } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";
import { DocsPage, docsRegistry } from "../registry";
import { useDocs, useDocsCommands } from "../store";

const sectionIcons: Record<string, any> = {
  "getting-started": Rocket,
  tutorials: GraduationCap,
  integrations: Puzzle,
  manuals: BookOpen,
  theory: Lightbulb,
  showcases: Star,
};

interface PageTreeNode {
  page?: DocsPage;
  children: Map<string, PageTreeNode>;
  name: string;
}

function buildTree(pages: DocsPage[]): PageTreeNode {
  const root: PageTreeNode = { children: new Map(), name: "root" };
  for (const page of pages) {
    const pathParts = page.path.replace("docs/", "").split("/");
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

function renderTreeNode(node: PageTreeNode, navigate: (path: string) => void, selectPage: (section: string, page: string) => void, section: string): JSX.Element[] {
  const items: JSX.Element[] = [];
  if (node.page) {
    items.push(
      <TreeItem key={node.page.path}>
        <TreeContent>
          <div
            className="flex items-center gap-2 cursor-pointer hover:text-foreground transition-colors text-sm"
            onClick={() => {
              selectPage(section, node.page!.path);
              navigate(`/${node.page!.path}`);
            }}
          >
            <FileText className="w-3 h-3" />
            <span>{node.page.title}</span>
          </div>
        </TreeContent>
      </TreeItem>,
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
          <TreeSection key={name} label={folderLabel} icon={Folder} defaultOpen={false}>
            {renderTreeNode(childNode, navigate, selectPage, section)}
          </TreeSection>,
        );
      } else {
        items.push(...renderTreeNode(childNode, navigate, selectPage, section));
      }
    });
  }
  return items;
}

interface WorkbenchProps {}

const Workbench: FC<WorkbenchProps> = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const docsState = useDocs();
  const { selectPage } = useDocsCommands();

  const sections = docsRegistry.getAllSections();

  return (
    <>
      {sections.map((section) => {
        const sectionState = docsState.sectionStates[section.id] || { isExpanded: false };
        const Icon = sectionIcons[section.id] || Folder;
        const pages = docsRegistry.getPagesBySection(section.id);
        const tree = buildTree(pages);

        return (
          <TreeSection key={section.id} label={`${section.emoji} ${section.label}`} defaultOpen={sectionState.isExpanded} icon={Icon}>
            {renderTreeNode(tree, navigate, selectPage, section.id)}
            {pages.length === 0 && (
              <TreeItem>
                <TreeContent>
                  <p className="text-sm text-muted-foreground">{t(`docs.sections.${section.id}.description`, section.description)}</p>
                </TreeContent>
              </TreeItem>
            )}
            {sectionState.progress !== undefined && (
              <TreeItem>
                <TreeContent>
                  <div className="flex items-center gap-2 mt-2">
                    <div className="flex-1 bg-accent h-2 rounded-full overflow-hidden">
                      <div className="bg-primary h-full transition-all" style={{ width: `${sectionState.progress}%` }} />
                    </div>
                    <span className="text-xs">{Math.round(sectionState.progress)}%</span>
                  </div>
                </TreeContent>
              </TreeItem>
            )}
            {sectionState.completedPages && sectionState.completedPages.length > 0 && (
              <TreeItem>
                <TreeContent>
                  <div className="flex items-center gap-1 mt-2 text-xs">
                    <CheckCircle className="w-3 h-3" />
                    <span>
                      {sectionState.completedPages.length} {t("docs.pagesCompleted")}
                    </span>
                  </div>
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
