// #region Header

// registry.ts

// 2025 Ueli Saluz

// #endregion

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
}

export const docsRegistry = new DocsRegistry();
