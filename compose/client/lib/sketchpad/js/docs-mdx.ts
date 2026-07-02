// #region 🧲Header
/** @emoji 📄 Sketchpad docs MDX page bundle (isolated from main index for playground graph hygiene). */
// #endregion 🧲Header

export type SketchpadDocPage = { readonly path: string; readonly title: string };
export type SketchpadDocSection = { readonly id: string; readonly label: string; readonly pages: readonly SketchpadDocPage[] };

/** @emoji 📄 Lazy-loaded MDX module shape from the sketchpad pages bundle. */
export type SketchpadMdxModule = {
	readonly default: unknown;
	readonly frontmatter?: Readonly<Record<string, unknown>>;
};

function sketchpadTitleFromDocPath(relativePath: string): string {
	return relativePath
		.split("/")
		.filter(Boolean)
		.map((segment) => segment.replace(/-/g, " ").replace(/\b\w/g, (char) => char.toUpperCase()))
		.join(" ");
}

const SKETCHPAD_MDX_MODULE_LOADERS = import.meta.glob<SketchpadMdxModule>("./page/**/*.mdx");
const SKETCHPAD_MDX_MODULE_PATHS = Object.keys(SKETCHPAD_MDX_MODULE_LOADERS);

/** @emoji 🔍 Resolves a docs route path to a Vite MDX module key. */
export function sketchpadResolveMdxModuleKey(docsPath: string): string | null {
	const clean = docsPath.replace(/^\/+/, "").replace(/\.mdx$/, "");
	const matches = SKETCHPAD_MDX_MODULE_PATHS.filter((key) => {
		const keyPath = key.replace(/^\.\/page\//, "").replace(/\.mdx$/, "");
		return keyPath === clean || keyPath === `${clean}/index`;
	});
	return matches[0] ?? null;
}

/** @emoji 📥 Loads an MDX page module for a docs route (`getting-started/index`, …). */
export async function sketchpadLoadMdxModule(docsPath: string): Promise<SketchpadMdxModule | null> {
	const moduleKey = sketchpadResolveMdxModuleKey(docsPath);
	if (!moduleKey) return null;
	try {
		return await SKETCHPAD_MDX_MODULE_LOADERS[moduleKey]!();
	} catch {
		return null;
	}
}

/** @emoji 🏷️ Reads a display title from MDX frontmatter or route path. */
export function sketchpadMdxTitle(module: SketchpadMdxModule | null, docsPath: string): string {
	const frontmatter = module?.frontmatter;
	if (frontmatter && typeof frontmatter["title"] === "string" && frontmatter["title"].length > 0) {
		return frontmatter["title"];
	}
	return sketchpadTitleFromDocPath(docsPath);
}

/** @emoji 📚 Builds the sketchpad docs tree from bundled MDX pages (Vite glob). */
export function sketchpadBuildDocsRegistry(): readonly SketchpadDocSection[] {
	const sectionMap = new Map<string, SketchpadDocPage[]>();
	for (const modulePath of SKETCHPAD_MDX_MODULE_PATHS) {
		const relative = modulePath.replace(/^\.\/pages\//, "").replace(/\.mdx$/, "");
		const sectionId = relative.split("/")[0] ?? "root";
		const pages = sectionMap.get(sectionId) ?? [];
		pages.push({ path: relative, title: sketchpadTitleFromDocPath(relative) });
		sectionMap.set(sectionId, pages);
	}
	if (sectionMap.size === 0) {
		return [
			{
				id: "getting-started",
				label: "Getting started",
				pages: [
					{ path: "getting-started/index", title: "Getting started" },
					{ path: "getting-started/installation", title: "Installation" },
				],
			},
		];
	}
	return [...sectionMap.entries()]
		.map(([id, pages]) => ({
			id,
			label: sketchpadTitleFromDocPath(id),
			pages: pages.sort((left, right) => left.path.localeCompare(right.path)),
		}))
		.sort((left, right) => left.label.localeCompare(right.label));
}
