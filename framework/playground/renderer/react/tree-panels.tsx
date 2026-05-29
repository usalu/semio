// #region 🧲Header
/** @emoji 🌲 Playground tree panel definitions (no puzzle hosts — safe for circular-import-free use). */
// #endregion 🧲Header

import type {
	SidePanelTabConfig,
	SidePanelTabDefinition,
	TreeDataItem,
	TreeDataSection,
	TreePanelConfig,
	TreePanelDefinition,
} from "@ui/react";

//#region 🔖TreePanels
/** @emoji 🌲 Enforces playground panels: each section needs `items` and/or `content` (no JSON-only fallbacks). */
export function enforcePlaygroundTreePanel(config: TreePanelConfig): void {
	if (!config.sections?.length) {
		throw new Error("Playground tree panel must declare at least one section.");
	}
	for (const section of config.sections) {
		const hasItems = Boolean(section.items?.length);
		const hasContent = section.content != null;
		if (!hasItems && !hasContent) {
			throw new Error(`Playground tree section "${section.id}" must declare items or content.`);
		}
	}
}

/** @emoji 📑 Abstract side-panel tab resolved to a {@link SidePanelTabConfig} tree. */
export abstract class PureSidePanelTabDefinition implements SidePanelTabDefinition {
	abstract resolveTab(): SidePanelTabConfig;
}

/** @emoji 🌲 Static tree panel: sections + items only. */
export class StaticTreePanelDefinition implements TreePanelDefinition {
	constructor(private readonly config: TreePanelConfig) {
		enforcePlaygroundTreePanel(config);
	}

	resolveTree(): TreePanelConfig {
		return this.config;
	}
}

/** @emoji 🌲 Tree panel that rebuilds sections on every {@link TreePanelDefinition.resolveTree} call. */
export class CallbackTreePanelDefinition implements TreePanelDefinition {
	constructor(private readonly buildSections: () => TreeDataSection[]) {}

	resolveTree(): TreePanelConfig {
		const config: TreePanelConfig = { sections: this.buildSections() };
		enforcePlaygroundTreePanel(config);
		return config;
	}
}

/** @emoji 🌲 Factory for a static {@link StaticTreePanelDefinition}. */
export function playgroundStaticTreePanel(config: TreePanelConfig): StaticTreePanelDefinition {
	return new StaticTreePanelDefinition(config);
}

/** @emoji 🌲 Single tree body for a side-panel tab (no duplicate section title; the tab is the panel name). */
export function playgroundTreePanelRootItems(sectionId: string, items: TreeDataItem[]): TreeDataSection[] {
	if (!items.length) {
		throw new Error("playgroundTreePanelRootItems requires at least one root item.");
	}
	return [{ id: sectionId, defaultOpen: true, items }];
}
//#endregion 🔖TreePanels
