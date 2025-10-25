import { PageFrontmatter } from "../../../elements/docs/Page";

export interface MDXModule {
    default: React.ComponentType;
    frontmatter?: PageFrontmatter;
}

export interface MDXFileInfo {
    path: string;
    section: string;
    title: string;
    description?: string;
    order?: number;
    module?: MDXModule;
}

const mdxModules = import.meta.glob<MDXModule>("../../docs/**/*.mdx", { eager: false });

export async function loadMDXFile(path: string): Promise<MDXModule | null> {
    const cleanPath = path.replace(/^docs\//, "");
    const possibleKeys = Object.keys(mdxModules).filter(key => {
        const keyPath = key.replace("../../docs/", "").replace(".mdx", "");
        return keyPath === cleanPath || keyPath === `${cleanPath}/index`;
    });
    if (possibleKeys.length > 0) {
        const modulePath = possibleKeys[0];
        try {
            const module = await mdxModules[modulePath]();
            return module;
        } catch (error) {
            console.error(`[ORIGIN] Failed to load MDX file: ${path}`, error);
            return null;
        }
    }
    return null;
}

function pathToSection(filePath: string): string {
    const parts = filePath.replace("../../docs/", "").split("/");
    return parts[0] || "general";
}

function pathToTitle(filePath: string): string {
    const parts = filePath.replace("../../docs/", "").replace(".mdx", "").split("/");
    const fileName = parts[parts.length - 1];
    if (fileName === "index") return parts[parts.length - 2] || "Home";
    return fileName
        .split("-")
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(" ");
}

export function getAllMDXFiles(): MDXFileInfo[] {
    return Object.keys(mdxModules).map(filePath => {
        const cleanPath = filePath.replace("../../docs/", "").replace(".mdx", "");
        const fullPath = `docs/${cleanPath}`;
        return {
            path: fullPath,
            section: pathToSection(filePath),
            title: pathToTitle(filePath),
            order: 0,
        };
    });
}

export function getMDXFilesBySection(section: string): MDXFileInfo[] {
    return getAllMDXFiles().filter(file => file.section === section);
}

export function getAllSections(): string[] {
    const sections = new Set<string>();
    Object.keys(mdxModules).forEach(filePath => {
        sections.add(pathToSection(filePath));
    });
    return Array.from(sections).sort();
}

