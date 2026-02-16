// #region 🔖Header

// [👤semio📚js🗃️sketchpad💻docstsx](semiorepo://file/SEMIO/JS/SKETCHPAD/DOCS.TSX)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Documentation viewer app with workbench and detail panels.

// #endregion 🔖Header

// #region 🔖Imports

// [👤semio📚js🗃️sketchpad💻docstsx🔖imports](semiorepo://section/semio/js/sketchpad/docs.tsx/imports)
// External and internal module imports MUST be declared here.

import { MDXProvider as BaseMDXProvider } from "@mdx-js/react";
import { FC, ReactNode, Suspense, createContext, useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { useLocation, useNavigate, useParams } from "react-router";
import { useLabel } from "../i18n";
import type { SketchpadStore } from "./Sketchpad";
import {
  Canvas,
  LayoutCanvas,
  PlainAppStore,
  createDefaultLayout,
  registerDocsAppStoreFactory,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useFocus,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSettings,
  useSketchpadCommands,
} from "./Sketchpad";
import { Aside, Tabs as BaseTabs, FileTree, FileTreeNode, Page, PageFrontmatter, PageNavigation, TabsContent, TabsList, TabsTrigger, TreeItem, TreeStateProvider } from "./elements";
import { PanelKind, createPanelDefinition, parseWindowLayout, registerAppPlugin, registerDocsRegistry, stringifyWindowLayout, type AppConfig, type AppEdit, type AppPlugin, type AppWindowConfig, type PanelVisibility } from "./shared";

// #endregion 🔖Imports

// #region 🔖MDX Loader

// [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader](semiorepo://section/semio/js/sketchpad/docs.tsx/mdx-loader)
// MDX file loading and section discovery utilities MUST be declared here.

/**
 * MDX module with default component and optional frontmatter.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️mdxmodule](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/mdxmodule)
 **/
export interface MDXModule {
  default: React.ComponentType;
  frontmatter?: PageFrontmatter;
}

/**
 * Frontmatter metadata for a docs section index page.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️sectionfrontmatter](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/sectionfrontmatter)
 **/
export interface SectionFrontmatter {
  title?: string;
  description?: string;
  icon?: string;
  order?: number;
  sidebar?: {
    label?: string;
  };
}

/**
 * File info for a loaded MDX file including path, section, and frontmatter.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️mdxfileinfo](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/mdxfileinfo)
 **/
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

/**
 * Metadata for a docs section including label, icon, and sort order.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️sectioninfo](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/sectioninfo)
 **/
export interface SectionInfo {
  id: string;
  label: string;
  description?: string;
  icon?: string;
  order: number;
}

const mdxModules = import.meta.glob<MDXModule>("./pages/**/*.mdx", { eager: true });

/**
 * Loads an MDX file by path from the eager-loaded module map.
 *
 * The path MUST be relative to the pages directory without extension.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️loadmdxfile](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/loadmdxfile)
 **/
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

/**
 * Returns all MDX files with resolved frontmatter and section info.
 *
 * Index files for sections MUST be excluded from the flat list.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️getallmdxfiles](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/getallmdxfiles)
 **/
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

/**
 * Returns MDX files filtered by section and sorted by order.
 *
 * The section parameter MUST match a top-level page directory.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️getmdxfilesbysection](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/getmdxfilesbysection)
 **/
export function getMDXFilesBySection(section: string): MDXFileInfo[] {
  return getAllMDXFiles()
    .filter((file) => file.section === section)
    .sort((a, b) => (a.order ?? 999) - (b.order ?? 999));
}

/**
 * Returns all docs sections discovered from the MDX file structure.
 *
 * Sections MUST be sorted by their order field.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxloader🛠️getallsections](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20loader/getallsections)
 **/
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

// #endregion 🔖MDX Loader

// #region 🔖MDX Provider

// [🔖semio/js/sketchpad/Docs.tsx#MDX Provider](semiorepo://section/semio/js/sketchpad/Docs.tsx/MDX-PROVIDER)
// MDX rendering context and heading components MUST be declared here.

// #region 🔖SectionTree

// [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🔖sectiontree](semiorepo://section/semio/js/sketchpad/docs.tsx/mdx-provider/section-tree)
// Section tree navigation component MUST render docs file hierarchy.

/**
 * Props for the SectionTree navigation component.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🔖sectiontree🛠️sectiontreeprops](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/sectiontree/sectiontreeprops)
 **/
export interface SectionTreeProps {
  title?: string;
  section?: string;
}

/**
 * Section tree component rendering a navigable file tree for a docs section.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🔖sectiontree🪨sectiontree](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/sectiontree/sectiontree)
 **/
export const SectionTree: React.FC<SectionTreeProps> = ({ title, section }) => {
  const location = useLocation();
  const navigate = useNavigate();

  const currentSection =
    section ||
    (() => {
      const path = location.pathname.replace(/^\/docs\//, "");
      const parts = path.split("/");
      return parts[0];
    })();

  const currentPath = location.pathname.replace(/^\//, "");
  const tree = docsRegistry.getSectionTree(currentSection);

  const handleNavigate = (path: string) => {
    navigate(`/${path}`);
  };

  return <FileTree title={title} nodes={tree} currentPath={currentPath} onNavigate={handleNavigate} as="div" />;
};

// #endregion 🔖SectionTree

/**
 * Node representing a heading with ID, text, level, and optional children.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🛠️headingnode](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/headingnode)
 **/
export interface HeadingNode {
  id: string;
  text: string;
  level: number;
  children?: HeadingNode[];
}

const headingsState = {
  headings: new Map<string, HeadingNode>(),
  listeners: new Set<() => void>(),
  cachedArray: [] as HeadingNode[],
  subscribe(listener: () => void) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  },
  notify() {
    this.cachedArray = Array.from(this.headings.values());
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
    return this.cachedArray;
  },
};

const subscribeHeadings = (callback: () => void) => headingsState.subscribe(callback);
const getHeadingsSnapshot = () => headingsState.getAll();

/**
 * Hook providing heading registration and retrieval via external store.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🪨useheadings](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/useheadings)
 **/
export const useHeadings = () => {
  const headings = useSyncExternalStore(subscribeHeadings, getHeadingsSnapshot);

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
    <tr className="border-b border-element" {...props}>
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

/**
 * Context provider supplying heading state to descendant components.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🪨headingsprovider](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/headingsprovider)
 **/
export const HeadingsProvider: FC<HeadingsProviderProps> = ({ children }) => {
  const { headings, registerHeading, clearHeadings } = useHeadings();
  return <HeadingsContext.Provider value={{ headings, registerHeading, clearHeadings }}>{children}</HeadingsContext.Provider>;
};

interface MDXProviderProps {
  children: ReactNode;
}

/**
 * MDX component provider wrapping children with custom heading and element renderers.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖mdxprovider🪨mdxprovider](semiorepo://definition/semio/js/sketchpad/docs.tsx/mdx%20provider/mdxprovider)
 **/
export const MDXProvider: FC<MDXProviderProps> = ({ children }) => {
  const components = useMemo(() => createComponents(), []);
  return <BaseMDXProvider components={components}>{children}</BaseMDXProvider>;
};

// #endregion 🔖MDX Provider

// #region 🔖Registry

// [👤semio📚js🗃️sketchpad💻docstsx🔖registry](semiorepo://section/semio/js/sketchpad/docs.tsx/registry)
// Docs registry MUST provide page and section lookup for navigation.

/**
 * Metadata for a docs page including path, section, and ordering.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖registry🛠️docspage](semiorepo://definition/semio/js/sketchpad/docs.tsx/registry/docspage)
 **/
export interface DocsPage {
  title: string;
  description?: string;
  icon?: string;
  path: string;
  section: string;
  order?: number;
  concepts?: string[];
}

/**
 * Extended section info for docs registry lookups.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖registry🛠️docssection](semiorepo://definition/semio/js/sketchpad/docs.tsx/registry/docssection)
 **/
export interface DocsSection extends SectionInfo { }

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

  private getOrderedPages(): DocsPage[] {
    const sections = this.getAllSections();
    const pages: DocsPage[] = [];

    const rootPage = this.getAllPages().find((p) => p.path === "docs/index");
    if (rootPage) {
      pages.push(rootPage);
    }

    sections.forEach((section) => {
      const sectionPages = this.getPagesBySection(section.id);
      pages.push(...sectionPages);
    });

    return pages;
  }

  getPreviousPage(currentPath: string): DocsPage | undefined {
    const orderedPages = this.getOrderedPages();
    const currentIndex = orderedPages.findIndex((p) => p.path === currentPath);
    if (currentIndex > 0) {
      return orderedPages[currentIndex - 1];
    }
    return undefined;
  }

  getNextPage(currentPath: string): DocsPage | undefined {
    const orderedPages = this.getOrderedPages();
    const currentIndex = orderedPages.findIndex((p) => p.path === currentPath);
    if (currentIndex >= 0 && currentIndex < orderedPages.length - 1) {
      return orderedPages[currentIndex + 1];
    }
    return undefined;
  }

  getSectionTree(sectionId: string): FileTreeNode[] {
    const sectionPages = this.getPagesBySection(sectionId);

    interface TreeNode {
      page?: DocsPage;
      children: Map<string, TreeNode>;
      name: string;
    }

    const root: TreeNode = { children: new Map(), name: "root" };
    for (const page of sectionPages) {
      const pathParts = page.path.replace("docs/", "").replace(`${sectionId}/`, "").split("/");
      let current = root;

      const isIndexFile = pathParts[pathParts.length - 1] === "index";
      const partsToTraverse = isIndexFile ? pathParts.slice(0, -1) : pathParts;

      for (let i = 0; i < partsToTraverse.length; i++) {
        const part = partsToTraverse[i];
        if (!current.children.has(part)) {
          current.children.set(part, { children: new Map(), name: part });
        }
        current = current.children.get(part)!;
      }

      current.page = page;
    }

    const convertToFileTree = (node: TreeNode): FileTreeNode[] => {
      const items: FileTreeNode[] = [];

      Array.from(node.children.entries()).forEach(([name, childNode]) => {
        const hasChildren = childNode.children.size > 0;
        const hasPage = !!childNode.page;

        if (hasChildren) {
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
          items.push({
            title: childNode.page.title,
            path: childNode.page.path,
            icon: childNode.page.icon,
            isFolder: false,
            children: [],
          });
        }
      });

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

/**
 * Singleton docs registry instance for page and section lookups.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖registry🪨docsregistry](semiorepo://definition/semio/js/sketchpad/docs.tsx/registry/docsregistry)
 **/
export const docsRegistry = new DocsRegistry();

// #endregion 🔖Registry

// #region 🔖Store

// [👤semio📚js🗃️sketchpad💻docstsx🔖store](semiorepo://section/semio/js/sketchpad/docs.tsx/store)
// Docs app section state MUST be declared here.

/**
 * Persisted state for a docs section including expansion and progress.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖store🛠️docssectionstate](semiorepo://definition/semio/js/sketchpad/docs.tsx/store/docssectionstate)
 **/
export interface DocsSectionState {
  isExpanded: boolean;
  progress?: number;
  completedPages?: string[];
}

// #endregion 🔖Store

// #region 🔖Types

// [👤semio📚js🗃️sketchpad💻docstsx🔖types](semiorepo://section/semio/js/sketchpad/docs.tsx/types)
// Docs app state, selection, and diff type definitions MUST be declared here.

/**
 * Current selection state of the docs app.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappselection](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappselection)
 **/
export interface DocsAppSelection {
  section?: string;
  page?: string;
}

/**
 * Diff for docs app selection section and page changes.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappselectiondiff](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappselectiondiff)
 **/
export interface DocsAppSelectionDiff {
  section?: { prev?: string; next?: string };
  page?: { prev?: string; next?: string };
}

/**
 * Section-level state for expansion, progress, and completed pages.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappsectionstate](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappsectionstate)
 **/
export interface DocsAppSectionState {
  isExpanded?: boolean;
  progress?: number;
  completedPages?: string[];
}

/**
 * Complete state of the docs app including panels, selection, and section states.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappstate](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappstate)
 **/
export interface DocsAppState {
  panelVisibility: PanelVisibility;
  selection?: DocsAppSelection;
  sectionStates?: Record<string, DocsAppSectionState>;
}

/**
 * Partial state diff for updating the docs app.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappdiff](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappdiff)
 **/
export interface DocsAppDiff {
  panelVisibility?: Partial<PanelVisibility>;
  selection?: DocsAppSelectionDiff;
  sectionStatesDiff?: Record<string, Partial<DocsAppSectionState>>;
}

/**
 * Edit record for undo and redo in the docs app.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docsappedit](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docsappedit)
 **/
export interface DocsAppEdit extends AppEdit<DocsAppSelectionDiff> { }

/**
 * Context passed to docs app command handlers.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docscommandcontext](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docscommandcontext)
 **/
export interface DocsCommandContext {
  docs: DocsAppState;
  origin?: string;
}

/**
 * Result returned by docs app command handlers.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖types🛠️docscommandresult](semiorepo://definition/semio/js/sketchpad/docs.tsx/types/docscommandresult)
 **/
export interface DocsCommandResult {
  diff?: DocsAppDiff;
}

// #endregion 🔖Types

// #region 🔖Docs App Store

// [👤semio📚js🗃️sketchpad💻docstsx🔖docsappstore](semiorepo://section/semio/js/sketchpad/docs.tsx/docs-app-store)
// Docs app store MUST extend PlainAppStore with docs-specific state management.

/**
 * Store managing docs app state including selection, sections, and commands.
 *
 * The store MUST apply diffs immutably and record edits for undo and redo.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖docsappstore🛠️docsappstore](semiorepo://definition/semio/js/sketchpad/docs.tsx/docs%20app%20store/docsappstore)
 **/
export class DocsAppStore extends PlainAppStore<DocsAppState, DocsAppDiff, DocsAppSelectionDiff, DocsAppEdit, DocsCommandContext, DocsCommandResult> {
  constructor(_parent: SketchpadStore) {
    const defaultState: DocsAppState = {
      panelVisibility: { toolbar: false, workbench: false, details: false, chat: false, settings: false },
      selection: undefined,
      sectionStates: {},
    };
    super(defaultState);
  }

  protected getSelection(): DocsAppSelection | undefined {
    return this.state.selection;
  }

  protected inverseSelectionDiff(selection: DocsAppSelection | undefined, diff: DocsAppSelectionDiff): DocsAppSelectionDiff {
    const inverseDiff: DocsAppSelectionDiff = {};
    if (diff.section) {
      inverseDiff.section = { prev: diff.section.next, next: diff.section.prev };
    }
    if (diff.page) {
      inverseDiff.page = { prev: diff.page.next, next: diff.page.prev };
    }
    return inverseDiff;
  }

  protected applySelectionDiff(selectionDiff: DocsAppSelectionDiff): void {
    const currentSelection = this.state.selection || {};
    const newSelection: DocsAppSelection = { ...currentSelection };

    if (selectionDiff.section?.next !== undefined) {
      newSelection.section = selectionDiff.section.next;
    }
    if (selectionDiff.page?.next !== undefined) {
      newSelection.page = selectionDiff.page.next;
    }

    this.state = { ...this.state, selection: newSelection };
    this.notify();
  }

  change(diff: DocsAppDiff): void {
    const newState = { ...this.state };

    if (diff.panelVisibility !== undefined) {
      newState.panelVisibility = { ...newState.panelVisibility, ...diff.panelVisibility };
    }
    if (diff.selection) {
      this.applySelectionDiff(diff.selection);
      return;
    }
    if (diff.sectionStatesDiff) {
      newState.sectionStates = { ...(newState.sectionStates || {}) };
      Object.entries(diff.sectionStatesDiff).forEach(([section, stateUpdate]) => {
        newState.sectionStates![section] = { ...(newState.sectionStates![section] || {}), ...stateUpdate };
      });
    }

    this.state = newState;
    this.notify();
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

// #endregion 🔖Docs App Store

// #region 🔖Commands

// [👤semio📚js🗃️sketchpad💻docstsx🔖commands](semiorepo://section/semio/js/sketchpad/docs.tsx/commands)
// Docs app command handlers MUST modify state through diff objects.

/**
 * Command handlers for docs app page selection, section toggling, and progress tracking.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖commands🪨docscommands](semiorepo://definition/semio/js/sketchpad/docs.tsx/commands/docscommands)
 **/
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
  registerDocsAppStoreFactory((parent) => {
    const store = new DocsAppStore(parent);
    Object.entries(docsCommands).forEach(([commandId, command]) => {
      store.registerCommand(commandId, command as any);
    });
    return store;
  });
}

// #region 🔖Docs App Plugin Registration

// [👤semio📚js🗃️sketchpad💻docstsx🔖commands🔖docsapppluginregistration](semiorepo://section/semio/js/sketchpad/docs.tsx/commands/docs-app-plugin-registration)
// Plugin registration MUST initialize docs app context and registry.

const docsAppPlugin: AppPlugin = {
  id: "docs",
  namespace: "DOCS",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): DocsAppState => ({
      panelVisibility: { toolbar: false, workbench: false, details: false, chat: false, settings: false },
      selection: undefined,
      sectionStates: {},
    }),
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(docsAppPlugin);
  registerDocsRegistry(docsRegistry);
}

// #endregion 🔖Docs App Plugin Registration

// #endregion 🔖Commands

// #region 🔖Canvas

// [🔖semio/js/sketchpad/Docs.tsx#Canvas](semiorepo://section/semio/js/sketchpad/Docs.tsx/CANVAS)
// Canvas components MUST render the docs app visual content.

// #region 🔖Windows

// [🔖semio/js/sketchpad/Docs.tsx#Windows](semiorepo://section/semio/js/sketchpad/Docs.tsx/WINDOWS)
// Window components MUST provide windowed views within the canvas.

// #region 🔖Page

// [👤semio📚js🗃️sketchpad💻docstsx🔖canvas🔖windows🔖page](semiorepo://section/semio/js/sketchpad/docs.tsx/canvas/windows/page)
// Page window MUST render MDX content with navigation and heading extraction.

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

  const currentPath = useMemo(() => {
    const path = location.pathname.replace(/^\//, "");
    return path;
  }, [location.pathname]);

  const isIndexPage = useMemo(() => {
    return currentPath.endsWith("/index") || currentPath.split("/").pop() === currentPath.split("/")[0];
  }, [currentPath]);

  const treeData = useMemo(() => {
    if (!isIndexPage) return null;

    const path = location.pathname.replace(/^\//, "");
    const parts = path.split("/").filter(Boolean);
    const section = parts[0];

    const pages = docsRegistry.getPagesBySection(section);
    if (pages.length === 0) return null;

    const root: TreeNode = { name: "root", children: new Map() };

    pages.forEach((page) => {
      const pageParts = page.path.replace(`${section}/`, "").split("/");
      let current = root;

      pageParts.forEach((part, index) => {
        if (!current.children.has(part)) {
          current.children.set(part, {
            name: part,
            children: new Map(),
          });
        }
        current = current.children.get(part)!;

        if (index === pageParts.length - 1 && part === "index") {
          const parent = pageParts.length === 1 ? root : pageParts.slice(0, -1).reduce((node, p) => node.children.get(p)!, root);
          parent.page = page;
        } else if (index === pageParts.length - 1) {
          current.page = page;
        }
      });
    });

    return root.children.size > 0 ? root : null;
  }, [isIndexPage, location.pathname]);

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

          const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
          if (prevItemsRef.current !== itemsKey) {
            prevItemsRef.current = itemsKey;
            setFocusItems(items);
          }

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

    const timer = setTimeout(extractHeadings, 100);
    return () => clearTimeout(timer);
    // eslint-disable-next-line react-hooks/exhaustive-deps -- setFocusItems is stable via useCallback
  }, [MDXContent, registerHeading]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps -- setOnFocusItem is stable via useCallback
  }, []);

  const renderTreeNode = (node: TreeNode): React.ReactNode[] => {
    const items: React.ReactNode[] = [];

    node.children.forEach((childNode, name) => {
      const hasChildren = childNode.children.size > 0;
      const hasPage = !!childNode.page;

      if (hasChildren) {
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
        {treeData && (
          <TreeStateProvider>
            <div className="not-prose my-8 p-6 rounded-lg border border-element bg-card">
              <h3 className="text-lg font-semibold mb-4">In this section</h3>
              <div className="flex flex-col gap-single">{renderTreeNode(treeData)}</div>
            </div>
          </TreeStateProvider>
        )}
      </Page>
    </div>
  );
};

// #endregion 🔖Page

// #endregion 🔖Windows

// #endregion 🔖Canvas

// #region 🔖Footer

// [👤semio📚js🗃️sketchpad💻docstsx🔖footer](semiorepo://section/semio/js/sketchpad/docs.tsx/footer)
// Footer component MUST manage docs app footer items.

/**
 * Footer component for the docs app registering footer items.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖footer🪨docsappfooter](semiorepo://definition/semio/js/sketchpad/docs.tsx/footer/docsappfooter)
 **/
export const DocsAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();

  useEffect(() => {
    if (appType !== "docs") return;

    // TODO: Add docs-specific footer items here

    return () => { };
  }, [appType, addFooterItem, removeFooterItem]);

  return null;
};

// #endregion 🔖Footer

// #region 🔖Panels

// [👤semio📚js🗃️sketchpad💻docstsx🔖panels](semiorepo://section/semio/js/sketchpad/docs.tsx/panels)
// Panel components MUST render sidebar content for the docs app.

const Workbench: FC = () => {
  const navigate = useNavigate();
  const sections = docsRegistry.getAllSections();
  return (
    <TreeStateProvider>
      <div className="flex flex-col gap-single">
        {sections.map((section) => (
          <TreeItem key={section.id} label={section.label} icon={section.icon ? <span className="text-sm">{section.icon}</span> : undefined} onClick={() => navigate(`/docs/${section.id}`)} />
        ))}
      </div>
    </TreeStateProvider>
  );
};

const Overview: FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const pathParts = location.pathname.replace(/^\//, "").split("/").filter(Boolean);
  const section = pathParts[1] || "index";
  const pages = section === "index" ? docsRegistry.getAllPages() : docsRegistry.getPagesBySection(section);
  return (
    <TreeStateProvider>
      <div className="flex flex-col gap-single">
        {pages.map((page) => (
          <TreeItem key={page.path} label={page.title} icon={page.icon ? <span className="text-sm">{page.icon}</span> : undefined} onClick={() => navigate(`/${page.path}`)} />
        ))}
      </div>
    </TreeStateProvider>
  );
};

const Details: FC = () => {
  const { headings } = useHeadings();
  const handleClick = useCallback((id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: "smooth", block: "start" });
  }, []);

  return (
    <div className="flex flex-col gap-single">
      {headings.map((h) => (
        <button key={h.id} className="text-left text-sm px-single py-tiny hover:bg-hover-panel" onClick={() => handleClick(h.id)} style={{ paddingLeft: `${Math.max(0, (h.level - 1) * 12)}px` }}>
          {h.text}
        </button>
      ))}
    </div>
  );
};

const Settings: FC = () => {
  return <div className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.panel.settings.placeholder")}</div>;
};

// #endregion 🔖Panels

// #region 🔖App

// [👤semio📚js🗃️sketchpad💻docstsx🔖app](semiorepo://section/semio/js/sketchpad/docs.tsx/app)
// Docs app root component MUST compose MDX routing, panel sections, and layout.

/**
 * Window kind identifiers for docs app layout.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖app🛠️docsappwindowkind](semiorepo://definition/semio/js/sketchpad/docs.tsx/app/docsappwindowkind)
 **/
export enum DocsAppWindowKind {
  Page = "page",
}

const App: FC = () => {
  const { "*": routePath } = useParams();
  const location = useLocation();
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  useFocus();
  const settings = useSettings();
  const sketchpadCommands = useSketchpadCommands();
  const [mdxModule, setMdxModule] = useState<MDXModule | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const docsPath = useMemo(() => {
    const raw = (routePath ?? "").trim();
    if (raw) return raw;
    const fromLocation = location.pathname.replace(/^\//, "").replace(/^docs\/?/, "");
    return fromLocation.trim() || "index";
  }, [routePath, location.pathname]);

  const defaultLayout = useMemo(() => createDefaultLayout([DocsAppWindowKind.Page], "row", [100], ["page"]), []);
  const storedWindowLayout = useMemo(() => parseWindowLayout(settings?.apps?.docs?.windowLayout), [settings]);
  const windowLayout = useMemo(() => storedWindowLayout || defaultLayout, [storedWindowLayout, defaultLayout]);
  const lastLayoutRef = useRef<any>(null);

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

    addSection("toolbar", {
      id: "semio.sketchpad.app.docs.toolbar.empty",
      specificity: 20,
      order: 0,
      toolbarPlaceholder: true,
      content: () => null,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.docs.docs");
      removeSection("workbench", "semio.sketchpad.app.docs.overview");
      removeSection("details", "semio.sketchpad.app.docs.page");
      removeSection("settings", "semio.sketchpad.app.docs.settings");
      removeSection("toolbar", "semio.sketchpad.app.docs.toolbar.empty");
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
        const module = await loadMDXFile(docsPath);
        if (module) {
          setMdxModule(module);
        } else {
          setError(`Failed to load ${docsPath}`);
        }
      } catch (err) {
        setError((err as Error).message);
      } finally {
        setLoading(false);
      }
    };

    loadContent();
  }, [docsPath, appType]);

  const windowConfig: AppWindowConfig = useMemo(
    () => ({
      windowKinds: [
        {
          id: DocsAppWindowKind.Page,
          label: "page",
          component: () => {
            if (loading) return <PageCanvas frontmatter={{ title: "Loading...", description: "" }} />;
            if (error || !mdxModule) return <PageCanvas frontmatter={{ title: "Error", description: error || "Content not found" }} />;
            return <PageCanvas MDXContent={mdxModule.default} frontmatter={mdxModule.frontmatter} />;
          },
        },
      ],
      defaultLayout,
    }),
    [defaultLayout, error, loading, mdxModule],
  );

  const handleLayoutChange = useCallback(
    (layout: any) => {
      const next = stringifyWindowLayout(layout);
      if (!next) return;
      const prev = stringifyWindowLayout(lastLayoutRef.current);
      if (next === prev) return;
      lastLayoutRef.current = layout;
      sketchpadCommands.setState("semio.sketchpad.app.docs.windowLayout", {
        settings: {
          apps: {
            docs: {
              windowLayout: next,
            },
          },
        },
      });
    },
    [sketchpadCommands],
  );

  return (
    <HeadingsProvider>
      <Canvas>
        <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
      </Canvas>
    </HeadingsProvider>
  );
};

export default App;

// #endregion 🔖App

// #region 🔖Config

// [👤semio📚js🗃️sketchpad💻docstsx🔖config](semiorepo://section/semio/js/sketchpad/docs.tsx/config)
// Docs app route, panel, and path matching configuration MUST be exported.

/**
 * Docs app configuration for routing, panels, and path matching.
 *
 *  * [👤semio📚js🗃️sketchpad💻docstsx🔖config🪨config](semiorepo://definition/semio/js/sketchpad/docs.tsx/config/config)
 **/
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

// #endregion 🔖Config
