import { PageFrontmatter } from "../../../elements/windows/Page";

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
  if (frontmatter?.sidebar?.label) return frontmatter.sidebar.label;
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
