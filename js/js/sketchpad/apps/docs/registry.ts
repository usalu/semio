// #region Header

// registry.ts

// 2025 Ueli Saluz

// #endregion

import { FileTreeNode } from "../../elements/aggregation/Tree";
import { getAllMDXFiles, getAllSections, getMDXFilesBySection, SectionInfo } from "./mdx-loader";

export interface DocsPage {
  title: string;
  description?: string;
  icon?: string;
  path: string;
  section: string;
  order?: number;
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
        } else if (hasPage) {
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
