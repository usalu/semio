// #region 🧲Header
/** @emoji 🧱 Playground stub for sketchpad docs MDX (avoids MDX glob in shared playground bundles). */
// #endregion 🧲Header

export type SketchpadDocPage = { readonly path: string; readonly title: string };
export type SketchpadDocSection = { readonly id: string; readonly label: string; readonly pages: readonly SketchpadDocPage[] };

export type SketchpadMdxModule = {
	readonly default: unknown;
	readonly frontmatter?: Readonly<Record<string, unknown>>;
};

/** @emoji 🔍 Playground docs route resolver stub. */
export function sketchpadResolveMdxModuleKey(_docsPath: string): string | null {
	return null;
}

/** @emoji 📥 Playground docs loader stub. */
export async function sketchpadLoadMdxModule(_docsPath: string): Promise<SketchpadMdxModule | null> {
	return null;
}

/** @emoji 🏷️ Playground docs title stub. */
export function sketchpadMdxTitle(_module: SketchpadMdxModule | null, docsPath: string): string {
	return docsPath;
}

/** @emoji 📚 Playground docs registry stub. */
export function sketchpadBuildDocsRegistry(): readonly SketchpadDocSection[] {
	return [];
}
