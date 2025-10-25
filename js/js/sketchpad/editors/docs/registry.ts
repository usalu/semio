// #region Header

// registry.ts

// 2025 Ueli Saluz

// #endregion

import { getAllMDXFiles, getMDXFilesBySection } from "./mdx-loader";

export interface DocsPage {
  title: string;
  description?: string;
  path: string;
  section: string;
  order?: number;
}

export interface DocsSection {
  id: string;
  label: string;
  emoji: string;
  description: string;
  order: number;
}

class DocsRegistry {
  private sections: Map<string, DocsSection> = new Map();

  constructor() {
    this.registerSection({ id: "getting-started", label: "Getting Started", emoji: "🚀", description: "Get started with Semio", order: 1 });
    this.registerSection({ id: "tutorials", label: "Tutorials", emoji: "📝", description: "Step-by-step tutorials", order: 2 });
    this.registerSection({ id: "integrations", label: "Integrations", emoji: "🔀", description: "Integration guides", order: 3 });
    this.registerSection({ id: "manuals", label: "Manuals", emoji: "📖", description: "Reference manuals", order: 4 });
    this.registerSection({ id: "theory", label: "Theory", emoji: "📚", description: "Theoretical concepts", order: 5 });
    this.registerSection({ id: "showcases", label: "Showcases", emoji: "🌟", description: "Real-world examples", order: 6 });
  }

  registerSection(section: DocsSection): void {
    this.sections.set(section.id, section);
  }

  getAllSections(): DocsSection[] {
    return Array.from(this.sections.values()).sort((a, b) => a.order - b.order);
  }

  getAllPages(): DocsPage[] {
    return getAllMDXFiles();
  }

  getPagesBySection(sectionId: string): DocsPage[] {
    return getMDXFilesBySection(sectionId);
  }

  getPage(path: string): DocsPage | undefined {
    return this.getAllPages().find(p => p.path === path);
  }

  getSection(id: string): DocsSection | undefined {
    return this.sections.get(id);
  }
}

export const docsRegistry = new DocsRegistry();
