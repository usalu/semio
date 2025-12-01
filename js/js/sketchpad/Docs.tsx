// #region Header

// Docs.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

// #region Imports

import { MDXProvider as BaseMDXProvider } from "@mdx-js/react";
import { FC, ReactNode, Suspense, createContext, lazy, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate, useParams } from "react-router";
import * as Y from "yjs";
import { useLabel } from "../i18n";
import type { SketchpadStore } from "./Sketchpad";
import { AppStore, Canvas, Window, registerDocsAppStoreFactory, useAddFooterItem, useAddPanelSection, useAppType, useFocus, useFocusSafe, useRemoveFooterItem, useRemovePanelSection } from "./Sketchpad";
import { Aside, Tabs as BaseTabs, FileTreeNode, Page, PageFrontmatter, PageNavigation, TabsContent, TabsList, TabsTrigger, TreeItem, TreeStateProvider } from "./elements";
import { PanelKind, createPanelDefinition, type AppConfig, type AppEdit, type PanelVisibility } from "./shared";

// #endregion Imports

// #region MDX Loader

export interface MDXModule {
  default: React.ComponentType;
  frontmatter?: PageFrontmatter;
}

export interface SectionFrontmatter {
  title?: string;
  description?: string;
  icon?: string;
  order?: number;
  sidebar?: {
    label?: string;
  };
}

export interface MDXFileInfo {
  path: string;
  section: string;
  title: string;
  description?: string;
  icon?: string;
  order?: number;
  concepts?: string[];
  module?: MDXModule;
}

export interface SectionInfo {
  id: string;
  label: string;
  description?: string;
  icon?: string;
  order: number;
}

const mdxModules = import.meta.glob<MDXModule>("./pages/**/*.mdx", { eager: true });

export async function loadMDXFile(path: string): Promise<MDXModule | null> {
  const cleanPath = path.replace(/^docs\//, "");
  const possibleKeys = Object.keys(mdxModules).filter((key) => {
    const keyPath = key.replace("./pages/", "").replace(".mdx", "");
    return keyPath === cleanPath || keyPath === `${cleanPath}/index`;
  });

  if (possibleKeys.length > 0) {
    const modulePath = possibleKeys[0];
    try {
      const module = mdxModules[modulePath];
      return module;
    } catch {
      return null;
    }
  }
  return null;
}

function pathToSection(filePath: string): string {
  const parts = filePath.replace("./pages/", "").split("/");
  return parts[0] || "root";
}

function pathToTitle(filePath: string, frontmatter?: PageFrontmatter): string {
  if (frontmatter?.title) return frontmatter.title;
  const parts = filePath.replace("./pages/", "").replace(".mdx", "").split("/");
  const fileName = parts[parts.length - 1];
  if (fileName === "index") return parts[parts.length - 2] || "Home";
  return fileName
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

export function getAllMDXFiles(): MDXFileInfo[] {
  return Object.keys(mdxModules)
    .filter((filePath) => {
      const parts = filePath.replace("./pages/", "").split("/");
      if (filePath === "./pages/index.mdx") return true;
      if (parts.length === 2 && parts[1] === "index.mdx") return false;
      return true;
    })
    .map((filePath) => {
      const module = mdxModules[filePath];
      const cleanPath = filePath.replace("./pages/", "").replace(".mdx", "");
      const fullPath = `docs/${cleanPath}`;
      const frontmatter = module.frontmatter;
      return {
        path: fullPath,
        section: pathToSection(filePath),
        title: pathToTitle(filePath, frontmatter),
        description: frontmatter?.description,
        icon: frontmatter?.icon,
        order: frontmatter?.order ?? 999,
        concepts: frontmatter?.concepts,
        module,
      };
    });
}

export function getMDXFilesBySection(section: string): MDXFileInfo[] {
  return getAllMDXFiles()
    .filter((file) => file.section === section)
    .sort((a, b) => (a.order ?? 999) - (b.order ?? 999));
}

export function getAllSections(): SectionInfo[] {
  const sectionsMap = new Map<string, SectionInfo>();
  Object.keys(mdxModules).forEach((filePath) => {
    const parts = filePath.replace("./pages/", "").split("/");
    if (parts.length > 1) {
      const sectionId = parts[0];
      if (!sectionsMap.has(sectionId)) {
        const indexPath = `./pages/${sectionId}/index.mdx`;
        const indexModule = mdxModules[indexPath];
        const frontmatter = indexModule?.frontmatter as SectionFrontmatter | undefined;
        sectionsMap.set(sectionId, {
          id: sectionId,
          label:
            frontmatter?.sidebar?.label ||
            frontmatter?.title ||
            sectionId
              .split("-")
              .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
              .join(" "),
          description: frontmatter?.description,
          icon: frontmatter?.icon,
          order: frontmatter?.order ?? 999,
        });
      }
    }
  });
  return Array.from(sectionsMap.values()).sort((a, b) => a.order - b.order);
}

// #endregion MDX Loader

// #region MDX Provider

// Lazy load SectionTree to avoid loading React Router in Storybook
const SectionTree = lazy(() => import("./elements").then((module) => ({ default: module.SectionTree })));

export interface HeadingNode {
  id: string;
  text: string;
  level: number;
  children?: HeadingNode[];
}

// Global headings state with event-based updates
// This allows the MDX content and the Details panel to share heading state
// even though they're rendered in different parts of the component tree
const headingsState = {
  headings: new Map<string, HeadingNode>(),
  listeners: new Set<() => void>(),
  subscribe(listener: () => void) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  },
  notify() {
    this.listeners.forEach((listener) => listener());
  },
  register(heading: HeadingNode) {
    const existing = this.headings.get(heading.id);
    if (!existing || existing.text !== heading.text || existing.level !== heading.level) {
      this.headings.set(heading.id, heading);
      this.notify();
    }
  },
  clear() {
    if (this.headings.size > 0) {
      this.headings.clear();
      this.notify();
    }
  },
  getAll() {
    return Array.from(this.headings.values());
  },
};

export const useHeadings = () => {
  const [headings, setHeadings] = useState<HeadingNode[]>(() => headingsState.getAll());

  useEffect(() => {
    const unsubscribe = headingsState.subscribe(() => {
      setHeadings(headingsState.getAll());
    });
    setHeadings(headingsState.getAll());
    return unsubscribe;
  }, []);

  const registerHeading = useCallback((heading: HeadingNode) => {
    headingsState.register(heading);
  }, []);

  const clearHeadings = useCallback(() => {
    headingsState.clear();
  }, []);

  return { headings, registerHeading, clearHeadings };
};

interface HeadingsContextValue {
  headings: HeadingNode[];
  registerHeading: (heading: HeadingNode) => void;
  clearHeadings: () => void;
}

const HeadingsContext = createContext<HeadingsContextValue | null>(null);

const TabItem: FC<{ label: string; children: ReactNode }> = ({ children }) => <>{children}</>;

const Tabs: FC<{ children: ReactNode }> = ({ children }) => {
  const items = Array.isArray(children) ? children : [children];
  const tabItems = items.filter((child: any) => child?.type === TabItem);
  if (tabItems.length === 0) return <div className="my-4">{children}</div>;
  return (
    <BaseTabs defaultValue={tabItems[0]?.props?.label || "0"} className="my-4">
      <TabsList>
        {tabItems.map((item: any, idx: number) => (
          <TabsTrigger key={idx} value={item.props.label || idx.toString()}>
            {item.props.label}
          </TabsTrigger>
        ))}
      </TabsList>
      {tabItems.map((item: any, idx: number) => (
        <TabsContent key={idx} value={item.props.label || idx.toString()}>
          {item.props.children}
        </TabsContent>
      ))}
    </BaseTabs>
  );
};

const createComponents = () => ({
  Aside,
  Tabs,
  TabItem,
  SectionTree,
  h1: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h1 id={generatedId} className="text-4xl font-bold mb-4 mt-8" {...props}>
        {children}
      </h1>
    );
  },
  h2: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h2 id={generatedId} className="text-3xl font-semibold mb-3 mt-6" {...props}>
        {children}
      </h2>
    );
  },
  h3: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h3 id={generatedId} className="text-2xl font-semibold mb-2 mt-5" {...props}>
        {children}
      </h3>
    );
  },
  h4: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h4 id={generatedId} className="text-xl font-semibold mb-2 mt-4" {...props}>
        {children}
      </h4>
    );
  },
  h5: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h5 id={generatedId} className="text-lg font-semibold mb-1 mt-3" {...props}>
        {children}
      </h5>
    );
  },
  h6: ({ children, id, ...props }: any) => {
    const generatedId =
      id ||
      (typeof children === "string" ? children : "")
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
    return (
      <h6 id={generatedId} className="text-base font-semibold mb-1 mt-2" {...props}>
        {children}
      </h6>
    );
  },
  p: ({ children, ...props }: any) => (
    <p className="mb-4 leading-7" {...props}>
      {children}
    </p>
  ),
  a: ({ children, href, ...props }: any) => (
    <a href={href} className="text-primary hover:underline" {...props}>
      {children}
    </a>
  ),
  ul: ({ children, ...props }: any) => (
    <ul className="list-disc list-inside mb-4 space-y-2" {...props}>
      {children}
    </ul>
  ),
  ol: ({ children, ...props }: any) => (
    <ol className="list-decimal list-inside mb-4 space-y-2" {...props}>
      {children}
    </ol>
  ),
  li: ({ children, ...props }: any) => (
    <li className="ml-4" {...props}>
      {children}
    </li>
  ),
  code: ({ children, className, ...props }: any) => {
    const inline = !className;
    if (inline) {
      return (
        <code className="bg-gray-100 dark:bg-gray-800 px-single.5 py-0.5 rounded text-sm font-mono" {...props}>
          {children}
        </code>
      );
    }
    return (
      <code className={`block bg-gray-100 dark:bg-gray-800 p-small rounded overflow-x-auto font-mono text-sm ${className}`} {...props}>
        {children}
      </code>
    );
  },
  pre: ({ children, ...props }: any) => (
    <pre className="bg-gray-100 dark:bg-gray-800 p-small rounded overflow-x-auto mb-4" {...props}>
      {children}
    </pre>
  ),
  blockquote: ({ children, ...props }: any) => (
    <blockquote className="border-l-4 border-gray-300 dark:border-gray-700 pl-4 italic my-4" {...props}>
      {children}
    </blockquote>
  ),
  hr: (props: any) => <hr className="my-8 border-gray-300 dark:border-gray-700" {...props} />,
  img: ({ src, alt, ...props }: any) => <img src={src} alt={alt} className="max-w-full h-auto rounded my-4" {...props} />,
  table: ({ children, ...props }: any) => (
    <div className="overflow-x-auto my-4">
      <table className="min-w-full border-collapse border border-gray-300 dark:border-gray-700" {...props}>
        {children}
      </table>
    </div>
  ),
  thead: ({ children, ...props }: any) => (
    <thead className="bg-gray-100 dark:bg-gray-800" {...props}>
      {children}
    </thead>
  ),
  tbody: ({ children, ...props }: any) => <tbody {...props}>{children}</tbody>,
  tr: ({ children, ...props }: any) => (
    <tr className="border-b border-gray-300 dark:border-gray-700" {...props}>
      {children}
    </tr>
  ),
  th: ({ children, ...props }: any) => (
    <th className="px-4 py-2 text-left font-semibold" {...props}>
      {children}
    </th>
  ),
  td: ({ children, ...props }: any) => (
    <td className="px-4 py-2" {...props}>
      {children}
    </td>
  ),
});

interface HeadingsProviderProps {
  children: ReactNode;
}

export const HeadingsProvider: FC<HeadingsProviderProps> = ({ children }) => {
  const { headings, registerHeading, clearHeadings } = useHeadings();
  return <HeadingsContext.Provider value={{ headings, registerHeading, clearHeadings }}>{children}</HeadingsContext.Provider>;
};

interface MDXProviderProps {
  children: ReactNode;
}

export const MDXProvider: FC<MDXProviderProps> = ({ children }) => {
  const components = useMemo(() => createComponents(), []);
  return <BaseMDXProvider components={components}>{children}</BaseMDXProvider>;
};

// #endregion MDX Provider

// #region Registry

export interface DocsPage {
  title: string;
  description?: string;
  icon?: string;
  path: string;
  section: string;
  order?: number;
  concepts?: string[];
}

export interface DocsSection extends SectionInfo {}

class DocsRegistry {
  getAllSections(): DocsSection[] {
    return getAllSections();
  }

  getAllPages(): DocsPage[] {
    return getAllMDXFiles();
  }

  getPagesBySection(sectionId: string): DocsPage[] {
    return getMDXFilesBySection(sectionId);
  }

  getPage(path: string): DocsPage | undefined {
    return this.getAllPages().find((p) => p.path === path);
  }

  getSection(id: string): DocsSection | undefined {
    return this.getAllSections().find((s) => s.id === id);
  }

  /**
   * Get all pages ordered by section order then page order
   */
  private getOrderedPages(): DocsPage[] {
    const sections = this.getAllSections();
    const pages: DocsPage[] = [];

    // Add root/index page first if it exists
    const rootPage = this.getAllPages().find((p) => p.path === "docs/index");
    if (rootPage) {
      pages.push(rootPage);
    }

    // Add pages from each section in order
    sections.forEach((section) => {
      const sectionPages = this.getPagesBySection(section.id);
      pages.push(...sectionPages);
    });

    return pages;
  }

  /**
   * Get the previous page in the documentation navigation order
   */
  getPreviousPage(currentPath: string): DocsPage | undefined {
    const orderedPages = this.getOrderedPages();
    const currentIndex = orderedPages.findIndex((p) => p.path === currentPath);
    if (currentIndex > 0) {
      return orderedPages[currentIndex - 1];
    }
    return undefined;
  }

  /**
   * Get the next page in the documentation navigation order
   */
  getNextPage(currentPath: string): DocsPage | undefined {
    const orderedPages = this.getOrderedPages();
    const currentIndex = orderedPages.findIndex((p) => p.path === currentPath);
    if (currentIndex >= 0 && currentIndex < orderedPages.length - 1) {
      return orderedPages[currentIndex + 1];
    }
    return undefined;
  }

  /**
   * Build a tree structure for a section showing all pages and subsections
   */
  getSectionTree(sectionId: string): FileTreeNode[] {
    const sectionPages = this.getPagesBySection(sectionId);

    interface TreeNode {
      page?: DocsPage;
      children: Map<string, TreeNode>;
      name: string;
    }

    // Build tree structure
    const root: TreeNode = { children: new Map(), name: "root" };
    for (const page of sectionPages) {
      const pathParts = page.path.replace("docs/", "").replace(`${sectionId}/`, "").split("/");

      // Check if the last part is "index" - if so, assign page to parent folder
      const isIndexFile = pathParts[pathParts.length - 1] === "index";
      const partsToTraverse = isIndexFile ? pathParts.slice(0, -1) : pathParts;

      let current = root;
      for (let i = 0; i < partsToTraverse.length; i++) {
        const part = partsToTraverse[i];
        if (!current.children.has(part)) {
          current.children.set(part, { children: new Map(), name: part });
        }
        current = current.children.get(part)!;
      }

      current.page = page;
    }

    // Convert to FileTreeNode structure
    const convertToFileTree = (node: TreeNode): FileTreeNode[] => {
      const items: FileTreeNode[] = [];

      Array.from(node.children.entries()).forEach(([name, childNode]) => {
        const hasChildren = childNode.children.size > 0;
        const hasPage = !!childNode.page;

        if (hasChildren) {
          // Folder (with or without index page)
          const folderLabel = childNode.page?.title || this.pathPartToTitle(name);
          const folderPath = childNode.page?.path || `docs/${sectionId}/${name}`;
          const folderIcon = childNode.page?.icon;

          items.push({
            title: folderLabel,
            path: folderPath,
            icon: folderIcon,
            isFolder: true,
            children: convertToFileTree(childNode),
          });
        } else if (hasPage && childNode.page) {
          // Leaf page
          items.push({
            title: childNode.page.title,
            path: childNode.page.path,
            icon: childNode.page.icon,
            isFolder: false,
            children: [],
          });
        }
      });

      // Sort by order
      return items.sort((a, b) => {
        const pageA = sectionPages.find((p) => p.path === a.path);
        const pageB = sectionPages.find((p) => p.path === b.path);
        return (pageA?.order ?? 999) - (pageB?.order ?? 999);
      });
    };

    return convertToFileTree(root);
  }

  private pathPartToTitle(part: string): string {
    return part
      .split("-")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  }
}

export const docsRegistry = new DocsRegistry();

// #endregion Registry

// #region Store

export interface DocsSectionState {
  isExpanded: boolean;
  progress?: number;
  completedPages?: string[];
}

// #endregion Registry

// #region Types

export interface DocsAppSelection {
  section?: string;
  page?: string;
}

export interface DocsAppSelectionDiff {
  section?: { prev?: string; next?: string };
  page?: { prev?: string; next?: string };
}

export interface DocsAppSectionState {
  isExpanded?: boolean;
  progress?: number;
  completedPages?: string[];
}

export interface DocsAppState {
  panelVisibility: PanelVisibility;
  selection?: DocsAppSelection;
  sectionStates?: Record<string, DocsAppSectionState>;
}

export interface DocsAppDiff {
  panelVisibility?: Partial<PanelVisibility>;
  selection?: DocsAppSelectionDiff;
  sectionStatesDiff?: Record<string, Partial<DocsAppSectionState>>;
}

export interface DocsAppEdit extends AppEdit<DocsAppSelectionDiff> {}

export interface DocsCommandContext {
  docs: DocsAppState;
  origin?: string;
}

export interface DocsCommandResult {
  diff?: DocsAppDiff;
}

// #endregion Types

// #region Store

export class DocsAppStore extends AppStore<DocsAppState, DocsAppDiff, DocsAppSelectionDiff, DocsAppEdit, DocsCommandContext, DocsCommandResult> {
  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) {
    super(parent, yMap, transact);

    transact(() => {
      if (!yMap.has("panelVisibility")) {
        const yPanelVisibility = new Y.Map<boolean>();
        yPanelVisibility.set("toolbar", false);
        yPanelVisibility.set("workbench", false);
        yPanelVisibility.set("details", false);
        yPanelVisibility.set("chat", false);
        yPanelVisibility.set("settings", false);
        yMap.set("panelVisibility", yPanelVisibility);
      }
      if (!yMap.has("isTransactionActive")) {
        yMap.set("isTransactionActive", false);
      }
      if (!yMap.has("currentTransactionStack")) {
        yMap.set("currentTransactionStack", new Y.Array<any>());
      }
      if (!yMap.has("pastTransactionsStack")) {
        yMap.set("pastTransactionsStack", new Y.Array<any>());
      }
      if (!yMap.has("redoStack")) {
        yMap.set("redoStack", new Y.Array<any>());
      }
    });
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: false,
        workbench: false,
        details: false,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? false,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? false,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): DocsAppSelection | undefined {
    const ySelection = this.yMap.get("selection") as Y.Map<string> | undefined;
    if (!ySelection) return undefined;
    return {
      section: ySelection.get("section"),
      page: ySelection.get("page"),
    };
  }

  protected hash(state: DocsAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DocsAppState {
    return {
      panelVisibility: this.panelVisibility,
      selection: this.selection,
      sectionStates: {},
    };
  }

  protected applySelectionDiff(selectionDiff: DocsAppSelectionDiff): void {
    let ySelection = this.yMap.get("selection") as Y.Map<string>;
    if (!ySelection) {
      ySelection = new Y.Map<string>();
      this.yMap.set("selection", ySelection);
    }
    if (selectionDiff.section?.next !== undefined) {
      ySelection.set("section", selectionDiff.section.next);
    }
    if (selectionDiff.page?.next !== undefined) {
      ySelection.set("page", selectionDiff.page.next);
    }
  }

  protected inverseSelectionDiff(selection: DocsAppSelection, diff: DocsAppSelectionDiff): DocsAppSelectionDiff {
    const inverseDiff: DocsAppSelectionDiff = {};
    if (diff.section) {
      inverseDiff.section = { prev: diff.section.next, next: diff.section.prev };
    }
    if (diff.page) {
      inverseDiff.page = { prev: diff.page.next, next: diff.page.prev };
    }
    return inverseDiff;
  }

  protected getSelection(): DocsAppSelection | undefined {
    return this.selection;
  }

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) {
      throw new Error(`Command "${command}" not found in docs store`);
    }
    const state = this.snapshot();
    const context: DocsCommandContext = { docs: state, origin };
    const result = await callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
      this.recordEdit(result);
    }
    return result as T;
  }
}

// #endregion Store

// #region Commands

export const docsCommands = {
  "semio.docsApp.selectPage": async (context: DocsCommandContext, section: string, page: string): Promise<DocsCommandResult> => {
    return {
      diff: {
        selection: {
          section: { prev: context.docs.selection?.section, next: section },
          page: { prev: context.docs.selection?.page, next: page },
        },
      },
    };
  },
  "semio.docsApp.toggleSection": async (context: DocsCommandContext, section: string): Promise<DocsCommandResult> => {
    const currentState = context.docs.sectionStates?.[section] || { isExpanded: false };
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { isExpanded: !currentState.isExpanded },
        },
      },
    };
  },
  "semio.docsApp.updateSectionProgress": async (context: DocsCommandContext, section: string, progress: number): Promise<DocsCommandResult> => {
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { progress },
        },
      },
    };
  },
  "semio.docsApp.markPageComplete": async (context: DocsCommandContext, section: string, page: string): Promise<DocsCommandResult> => {
    const currentState = context.docs.sectionStates?.[section] || { isExpanded: false, completedPages: [] };
    const completedPages = currentState.completedPages || [];
    if (!completedPages.includes(page)) {
      completedPages.push(page);
    }
    return {
      diff: {
        sectionStatesDiff: {
          [section]: { completedPages: [...completedPages] },
        },
      },
    };
  },
};

if (typeof window !== "undefined") {
  registerDocsAppStoreFactory((parent, yMap, transact) => {
    const store = new DocsAppStore(parent, yMap, transact);
    Object.entries(docsCommands).forEach(([commandId, command]) => {
      store.registerCommand(commandId, command);
    });
    return store;
  });
}

// #endregion Commands

// #region Navbar

// #endregion Navbar

// #region Canvas

// #region Windows

// #region Page

interface PageCanvasProps {
  MDXContent?: React.ComponentType;
  frontmatter?: any;
}

interface TreeNode {
  name: string;
  page?: DocsPage;
  children: Map<string, TreeNode>;
}

const PageCanvas: FC<PageCanvasProps> = ({ MDXContent, frontmatter }) => {
  const location = useLocation();
  const navigate = useNavigate();
  const { setFocusItems, setOnFocusItem } = useFocus();
  const { clearHeadings, registerHeading } = useHeadings();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const containerRef = useRef<HTMLDivElement>(null);
  const prevItemsRef = useRef<string>("");

  // Get current page path (remove leading slash and "docs/" prefix if present)
  const currentPath = useMemo(() => {
    const path = location.pathname.replace(/^\//, "");
    return path;
  }, [location.pathname]);

  // Detect if this is an index page (folder page)
  const isIndexPage = useMemo(() => {
    return currentPath.endsWith("/index") || currentPath.split("/").pop() === currentPath.split("/")[0];
  }, [currentPath]);

  // Build tree structure for current section/folder
  const treeData = useMemo(() => {
    if (!isIndexPage) return null;

    const path = location.pathname.replace(/^\//, "");
    const parts = path.split("/").filter(Boolean);
    const section = parts[0];

    // Get all pages in this section
    const pages = docsRegistry.getPagesBySection(section);
    if (pages.length === 0) return null;

    // Build tree structure
    const root: TreeNode = { name: "root", children: new Map() };

    pages.forEach((page) => {
      const pageParts = page.path.replace(`${section}/`, "").split("/");
      let currentNode = root;

      pageParts.forEach((part, index) => {
        if (!currentNode.children.has(part)) {
          currentNode.children.set(part, {
            name: part,
            children: new Map(),
          });
        }
        currentNode = currentNode.children.get(part)!;

        // If this is the last part and it's "index", assign to parent
        if (index === pageParts.length - 1 && part === "index") {
          const parent = pageParts.length === 1 ? root : pageParts.slice(0, -1).reduce((node, p) => node.children.get(p)!, root);
          parent.page = page;
        } else if (index === pageParts.length - 1) {
          currentNode.page = page;
        }
      });
    });

    return root.children.size > 0 ? root : null;
  }, [isIndexPage, location.pathname]);

  // Get navigation links
  const navigation = useMemo(() => {
    const prev = docsRegistry.getPreviousPage(currentPath);
    const next = docsRegistry.getNextPage(currentPath);
    return {
      prev: prev
        ? {
            path: prev.path,
            title: prev.title,
            section: prev.section,
          }
        : undefined,
      next: next
        ? {
            path: next.path,
            title: next.title,
            section: next.section,
          }
        : undefined,
    };
  }, [currentPath]);

  // Clear headings when MDXContent changes
  useEffect(() => {
    clearHeadings();
  }, [MDXContent, clearHeadings]);

  useEffect(() => {
    const extractHeadings = () => {
      if (containerRef.current) {
        const headings = containerRef.current.querySelectorAll("h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]");

        if (headings.length > 0) {
          const items = Array.from(headings).map((heading) => ({
            id: heading.id,
            label: heading.textContent || heading.id,
            category: heading.tagName,
          }));

          // Update focus items
          const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
          if (prevItemsRef.current !== itemsKey) {
            prevItemsRef.current = itemsKey;
            setFocusItems(items);
          }

          // Register all headings with HeadingsProvider
          const headingNodes = items.map((item) => ({
            id: item.id,
            text: item.label,
            level: parseInt(item.category.substring(1)),
          }));

          headingNodes.forEach((node) => {
            registerHeading(node);
          });
        }
      }
    };

    // Use setTimeout to wait for MDX to render
    const timer = setTimeout(extractHeadings, 100);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [MDXContent, registerHeading]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Render tree node recursively
  const renderTreeNode = (node: TreeNode): React.ReactNode[] => {
    const items: React.ReactNode[] = [];

    node.children.forEach((childNode, name) => {
      const hasChildren = childNode.children.size > 0;
      const hasPage = !!childNode.page;

      if (hasChildren) {
        // Folder with possible index page
        const folderLabel =
          childNode.page?.title ||
          name
            .split("-")
            .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
            .join(" ");

        const isCurrentPage = !!(childNode.page && currentPath && childNode.page.path === currentPath);
        const folderIcon = childNode.page?.icon;

        items.push(
          <TreeItem
            key={childNode.page?.path || name}
            label={folderLabel}
            icon={folderIcon ? <span className="text-sm">{folderIcon}</span> : undefined}
            isHighlighted={isCurrentPage}
            onClick={
              childNode.page
                ? () => {
                    navigate(`/${childNode.page!.path}`);
                  }
                : undefined
            }
          >
            {renderTreeNode(childNode)}
          </TreeItem>,
        );
      } else if (hasPage && childNode.page) {
        // Leaf page
        const isCurrentPage = !!(currentPath && childNode.page.path === currentPath);
        const pageIcon = childNode.page.icon;

        items.push(
          <TreeItem
            key={childNode.page.path}
            label={childNode.page.title}
            icon={pageIcon ? <span className="text-sm">{pageIcon}</span> : undefined}
            isHighlighted={isCurrentPage}
            onClick={() => {
              navigate(`/${childNode.page!.path}`);
            }}
          />,
        );
      }
    });

    return items;
  };

  return (
    <div ref={containerRef} className="h-full w-full">
      <Page frontmatter={frontmatter} focusedItemId={focusedItemId} onFocusComplete={() => setFocusedItemId(undefined)} footer={<PageNavigation prev={navigation.prev} next={navigation.next} />}>
        <MDXProvider>
          <Suspense fallback={<div className="text-muted-foreground">Loading...</div>}>{MDXContent ? <MDXContent /> : <p className="text-muted-foreground">No content available</p>}</Suspense>
        </MDXProvider>

        {/* Auto-inject section tree on index pages */}
        {treeData && (
          <TreeStateProvider>
            <div className="not-prose my-8 p-6 rounded-lg border border-border bg-card">
              <h3 className="text-lg font-semibold mb-4">In this section</h3>
              <div className="flex flex-col gap-single">{renderTreeNode(treeData)}</div>
            </div>
          </TreeStateProvider>
        )}
      </Page>
    </div>
  );
};

// #endregion Page

// #endregion Windows

// #region Panels

// #region Left

// #region Workbench

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
      const folderLabel =
        childNode.page?.title ||
        name
          .split("-")
          .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
          .join(" ");

      const isCurrentPage = !!(childNode.page && currentPath && childNode.page.path === `docs/${currentPath}`);
      const folderIcon = childNode.page?.icon;

      items.push(
        <TreeItem
          key={childNode.page?.path || name}
          label={folderLabel}
          icon={folderIcon ? <span className="text-sm">{folderIcon}</span> : undefined}
          defaultOpen={false}
          isHighlighted={isCurrentPage}
          onClick={
            childNode.page
              ? () => {
                  selectPage(section, childNode.page!.path);
                  navigate(`/${childNode.page!.path}`);
                }
              : undefined
          }
        >
          {renderTreeNode(childNode, navigate, selectPage, section, currentPath)}
        </TreeItem>,
      );
    } else if (hasPage && childNode.page) {
      // This is a leaf page (no children)
      const isCurrentPage = !!(currentPath && childNode.page.path === `docs/${currentPath}`);
      const pageIcon = childNode.page.icon;

      items.push(
        <TreeItem
          key={childNode.page.path}
          label={childNode.page.title}
          icon={pageIcon ? <span className="text-sm">{pageIcon}</span> : undefined}
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

const Workbench: FC = () => {
  const navigate = useNavigate();
  const params = useParams();
  const pathParts = params["*"]?.split("/").filter(Boolean) || [];
  const currentPath = pathParts.join("/") || "index";
  const sections = docsRegistry.getAllSections();

  return (
    <>
      {sections.map((section) => {
        const pages = docsRegistry.getPagesBySection(section.id);
        const tree = buildTree(pages, section.id);
        const sectionPath = `docs/${section.id}/index`;
        const sectionPage = docsRegistry.getPage(sectionPath);
        const isCurrentPage = !!(currentPath && sectionPath === `docs/${currentPath}`);
        const sectionIcon = section.icon || "📁";

        return (
          <TreeItem
            key={section.id}
            label={section.label}
            icon={<span className="text-sm">{sectionIcon}</span>}
            isHighlighted={isCurrentPage}
            onClick={
              sectionPage
                ? () => {
                    navigate(`/${sectionPath}`);
                  }
                : undefined
            }
          >
            {renderTreeNode(tree, navigate, () => {}, section.id, currentPath)}
          </TreeItem>
        );
      })}
    </>
  );
};

// #endregion Workbench

// #region Overview

interface OverviewProps {}

const Overview: FC<OverviewProps> = () => {
  const { t } = useTranslation();
  const { headings: contextHeadings } = useHeadings();
  const focusContext = useFocusSafe();
  const flatHeadings = contextHeadings;

  if (flatHeadings.length === 0) {
    return (
      <div className="p-single">
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.docs.noHeadings")}</p>
      </div>
    );
  }

  const hierarchicalHeadings = buildHeadingHierarchy(flatHeadings);

  return (
    <div className="p-single">
      <HeadingTree headings={hierarchicalHeadings} onNavigate={undefined} triggerFocus={focusContext?.triggerFocusItem} />
    </div>
  );
};

// #endregion Overview

// #endregion Left

// #region Right

// #region Details

const buildHeadingHierarchy = (flatHeadings: HeadingNode[]): HeadingNode[] => {
  const root: HeadingNode[] = [];
  const stack: HeadingNode[] = [];

  flatHeadings.forEach((heading) => {
    const node: HeadingNode = { ...heading, children: [] };

    // Find the correct parent by going up the stack
    while (stack.length > 0 && stack[stack.length - 1].level >= node.level) {
      stack.pop();
    }

    if (stack.length === 0) {
      // Top-level heading
      root.push(node);
    } else {
      // Add as child to the last item in stack
      const parent = stack[stack.length - 1];
      if (!parent.children) parent.children = [];
      parent.children.push(node);
    }

    stack.push(node);
  });

  return root;
};

const HeadingTree: FC<{ headings: HeadingNode[]; onNavigate?: (id: string) => void; triggerFocus?: (id: string) => void }> = ({ headings, onNavigate, triggerFocus }) => {
  return (
    <>
      {headings.map((heading) => (
        <TreeItem
          key={heading.id}
          label={heading.text}
          defaultOpen={heading.children && heading.children.length > 0}
          onClick={() => {
            if (onNavigate) {
              onNavigate(heading.id);
            } else if (triggerFocus) {
              triggerFocus(heading.id);
            } else {
              const element = document.getElementById(heading.id);
              element?.scrollIntoView({ behavior: "smooth" });
            }
          }}
        >
          {heading.children && heading.children.length > 0 && <HeadingTree headings={heading.children} onNavigate={onNavigate} triggerFocus={triggerFocus} />}
        </TreeItem>
      ))}
    </>
  );
};

const Details: FC = () => {
  const { t } = useTranslation();
  const { headings: contextHeadings } = useHeadings();
  const focusContext = useFocusSafe();
  const flatHeadings = contextHeadings;

  if (flatHeadings.length === 0) {
    return (
      <div className="p-single">
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.docs.noHeadings")}</p>
      </div>
    );
  }

  // Build hierarchical structure from flat list
  const hierarchicalHeadings = buildHeadingHierarchy(flatHeadings);

  return (
    <div className="p-single">
      <HeadingTree headings={hierarchicalHeadings} onNavigate={undefined} triggerFocus={focusContext?.triggerFocusItem} />
    </div>
  );
};

// #endregion Details

// #region Chat

// #endregion Chat

// #region Settings

interface SettingsProps {}

const Settings: FC<SettingsProps> = () => {
  return (
    <div className="p-4">
      <h3 className="text-sm font-semibold mb-2">Documentation Settings</h3>
      <p className="text-xs text-muted-foreground">Settings for documentation display and preferences.</p>
    </div>
  );
};

// #endregion Settings

// #endregion Right

// #endregion Panels

// #region Tools

// #endregion Tools

// #endregion Canvas

// #region Footer

export const DocsAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();

  useEffect(() => {
    if (appType !== "docs") return;

    // TODO: Add docs-specific footer items here

    return () => {
      // Cleanup
    };
  }, [appType, addFooterItem, removeFooterItem]);

  return null;
};

// #endregion Footer

// #region App

const App: FC = () => {
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

    const WorkbenchWrapper = () => <Workbench />;
    const OverviewWrapper = () => <Overview />;
    const DetailsWrapper = () => <Details />;
    const SettingsWrapper = () => <Settings />;

    addSection("workbench", {
      id: "semio.sketchpad.app.docs.docs",
      specificity: 20,
      order: 1,
      content: WorkbenchWrapper,
    });

    addSection("workbench", {
      id: "semio.sketchpad.app.docs.overview",
      specificity: 20,
      order: 2,
      content: OverviewWrapper,
    });

    addSection("details", {
      id: "semio.sketchpad.app.docs.page",
      specificity: 20,
      order: 1,
      content: DetailsWrapper,
    });

    addSection("settings", {
      id: "semio.sketchpad.app.docs.settings",
      specificity: 20,
      order: 1,
      content: SettingsWrapper,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.docs.docs");
      removeSection("workbench", "semio.sketchpad.app.docs.overview");
      removeSection("details", "semio.sketchpad.app.docs.page");
      removeSection("settings", "semio.sketchpad.app.docs.settings");
    };
  }, [appType, addSection, removeSection]);

  useEffect(() => {
    if (appType !== "docs") {
      setLoading(false);
      return;
    }

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
  }, [fullPath, appType]);

  if (loading) {
    return (
      <HeadingsProvider>
        <Canvas>
          <Window id="page" className="h-full w-full">
            <PageCanvas frontmatter={{ title: "Loading...", description: "" }} />
          </Window>
        </Canvas>
      </HeadingsProvider>
    );
  }

  if (error || !mdxModule) {
    return (
      <HeadingsProvider>
        <Canvas>
          <Window id="page" className="h-full w-full">
            <PageCanvas frontmatter={{ title: "Error", description: error || "Content not found" }} />
          </Window>
        </Canvas>
      </HeadingsProvider>
    );
  }

  return (
    <HeadingsProvider>
      <Canvas>
        <Window id="page" className="h-full w-full">
          <PageCanvas MDXContent={mdxModule.default} frontmatter={mdxModule.frontmatter} />
        </Window>
      </Canvas>
    </HeadingsProvider>
  );
};

export default App;

// #endregion App

// #region Config

export const config: AppConfig = {
  id: "docs",
  component: App,
  routeSegments: [{ path: "docs" }, { path: "*" }],
  getPanels: (getLabelFn: (key: string) => string, getHotkeyFn: (id: string) => string) => [
    createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show", getHotkeyFn("semio.sketchpad.navbar.panelToggle.workbench.show"), {
      labelKey: "semio.sketchpad.navbar.panelToggle.workbench.show",
      manualPath: "/docs/manuals/sketchpad#workbench",
    }),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show", getHotkeyFn("semio.sketchpad.navbar.panelToggle.details.show"), {
      labelKey: "semio.sketchpad.navbar.panelToggle.details.show",
      manualPath: "/docs/manuals/sketchpad#details",
    }),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show", getHotkeyFn("semio.sketchpad.navbar.panelToggle.settings.show"), {
      labelKey: "semio.sketchpad.navbar.panelToggle.settings.show",
      manualPath: "/docs/manuals/sketchpad#settings",
    }),
  ],
  matchesPath: (pathParts) => pathParts.length > 0 && pathParts[0] === "docs",
  order: 5,
};

// #endregion Config
