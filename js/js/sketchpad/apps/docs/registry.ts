// #region Header

// registry.ts

// 2025 Ueli Saluz

// #endregion

import { FileTreeNode } from "../../elements/aggregation/Tree";
import { getAllMDXFiles, getAllSections, getMDXFilesBySection, SectionInfo } from "./mdx-loader";

export interface DocsPage {
  title: string;
  description?: string;
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
    const allPages = this.getAllPages();
    const sectionPages = allPages.filter((p) => p.path.startsWith(`docs/${sectionId}/`));

    // Group pages by their directory structure
    const tree = new Map<string, FileTreeNode>();

    sectionPages.forEach((page) => {
      const pathParts = page.path.replace(`docs/${sectionId}/`, "").split("/");

      // Build path hierarchy
      let currentPath = `docs/${sectionId}`;
      for (let i = 0; i < pathParts.length; i++) {
        const part = pathParts[i];
        const isLast = i === pathParts.length - 1;
        currentPath = `${currentPath}/${part}`;

        if (!tree.has(currentPath)) {
          tree.set(currentPath, {
            title: isLast ? page.title : this.pathPartToTitle(part),
            path: currentPath,
            isFolder: !isLast,
            children: [],
          });
        }
      }
    });

    // Build parent-child relationships
    const rootNodes: FileTreeNode[] = [];
    const nodes = Array.from(tree.values());

    nodes.forEach((node) => {
      const pathParts = node.path.split("/");
      if (pathParts.length === 3) {
        // Direct child of section
        rootNodes.push(node);
      } else {
        // Find parent
        const parentPath = pathParts.slice(0, -1).join("/");
        const parent = tree.get(parentPath);
        if (parent && parent.children) {
          parent.children.push(node);
        }
      }
    });

    // Sort by order
    const sortNodes = (nodes: FileTreeNode[]): FileTreeNode[] => {
      return nodes
        .map((node) => ({
          ...node,
          children: node.children ? sortNodes(node.children) : undefined,
        }))
        .sort((a, b) => {
          const pageA = allPages.find((p) => p.path === a.path);
          const pageB = allPages.find((p) => p.path === b.path);
          return (pageA?.order ?? 999) - (pageB?.order ?? 999);
        });
    };

    return sortNodes(rootNodes);
  }

  private pathPartToTitle(part: string): string {
    return part
      .split("-")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  }
}

export const docsRegistry = new DocsRegistry();
