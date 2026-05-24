// #region 🧲Header
/** @emoji 🧱 Maps {@link @elements/framework} shell specs to React window/panel/toolbar registrations. */
// #endregion 🧲Header

import {
	CommandBus,
	getDeclarativeSidePanelBodyFactory,
	getDeclarativeWindowBodyFactory,
	type ShellAppTools,
	type ShellFooterItem,
	type ShellSidePanelBodyViewContext,
	type ShellSideTabSpec,
	type ShellToolItem,
	type ShellWindowBodyViewContext,
	type ShellWindowMeasure,
	WorkbenchWindowKind,
} from "@elements/framework";
import type { LucideIcon } from "lucide-react";
import * as React from "react";

import {
	APP_TOOL_CATEGORY_ORDER,
	type AppTools,
	type FooterItem,
	type SidePanelTabConfig,
	type UIWindowKindDefinition,
	type UIWindowMeasure,
	type UIToolbarItem,
} from "./shell-chrome-types.tsx";

import { UiRenderer } from "./ui-declarative-renderer.tsx";
import { useApp } from "./workbench-app-context.tsx";

const elementIconNodes = new Map<string, React.ReactNode>();

/** @emoji 🖼 Registers a static icon node resolved by `iconId` for toolbars, footers, and tabs. */
export function registerElementIcon(iconId: string, node: React.ReactNode): void {
	elementIconNodes.set(iconId, node);
}

/** @emoji 🔍 Returns a registered element icon node for navbar/search rows. */
export function resolveElementIcon(iconId: string): React.ReactNode | undefined {
	return elementIconNodes.get(iconId);
}

const shellTabIcons = new Map<string, LucideIcon>();

/** @emoji 🖼 Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerShellTabIcon(iconId: string, Icon: LucideIcon): void {
	shellTabIcons.set(iconId, Icon);
}

const windowBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji 🪟 Binds a `bodyKey` from {@link WorkbenchWindowKind} to a React window body component. */
export function registerWindowBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	windowBodyByKey.set(bodyKey, Component);
}

const sidePanelBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji 📑 Binds a `bodyKey` from {@link ShellSideTabSpec} to a React panel body component. */
export function registerSidePanelBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	sidePanelBodyByKey.set(bodyKey, Component);
}

const declarativeWindowBodyComponents = new Map<string, React.FC>();

function getDeclarativeWindowBodyComponent(windowKindId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${windowKindId}`;
	let component = declarativeWindowBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeWindowBody() {
			const { workbench, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => workbench.subscribe(listener),
				() => workbench.generation,
				() => 0,
			);
			const ctx: ShellWindowBodyViewContext = {
				workbench,
				windowKindId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getDeclarativeWindowBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={workbench.commandBus} />;
		};
		declarativeWindowBodyComponents.set(cacheKey, component);
	}
	return component;
}

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${tabId}`;
	let component = declarativeSidePanelBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeSidePanelBody() {
			const { workbench, activeModeId } = useApp();
			const generation = React.useSyncExternalStore(
				(listener) => workbench.subscribe(listener),
				() => workbench.generation,
				() => 0,
			);
			const ctx: ShellSidePanelBodyViewContext = {
				workbench,
				windowKindId: tabId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getDeclarativeSidePanelBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={workbench.commandBus} />;
		};
		declarativeSidePanelBodyComponents.set(cacheKey, component);
	}
	return component;
}

function shellMeasuresToGolden(measures: readonly ShellWindowMeasure[], bus: CommandBus): UIWindowMeasure[] | undefined {
	if (!measures.length) return undefined;
	return measures.map((measure) => {
		if (measure.kind === "select") {
			return {
				id: measure.id,
				kind: "select",
				label: measure.label,
				value: measure.value,
				items: measure.items.map((item) => ({ id: item.id, value: item.value, label: item.label })),
				onValueChange: (value: string) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), value }),
			};
		}
		return { id: measure.id, kind: "display", content: null };
	});
}

/** @emoji 🪟 Converts framework window kinds into golden-layout window definitions. */
export function shellWindowKindsToGolden(windowKinds: readonly WorkbenchWindowKind[], bus: CommandBus): UIWindowKindDefinition[] {
	const goldenMeasures = (wk: WorkbenchWindowKind) => shellMeasuresToGolden(wk.measures, bus);
	return windowKinds.map((wk) => {
		const declarativeFactory = getDeclarativeWindowBodyFactory(wk.bodyKey);
		if (declarativeFactory) {
			return { id: wk.id, label: wk.label, component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey), measures: goldenMeasures(wk) };
		}
		const Body =
			windowBodyByKey.get(wk.bodyKey) ??
			(() => (
				<div className="p-2 text-xs text-muted-foreground">
					Missing window body &quot;{wk.bodyKey}&quot;
				</div>
			));
		return { id: wk.id, label: wk.label, component: Body as React.ComponentType, measures: goldenMeasures(wk) };
	});
}

function shellTabIconComponent(iconId: string): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		const node = elementIconNodes.get(iconId);
		if (node) {
			return (
				<span className="inline-flex items-center justify-center" style={{ width: size, height: size }}>
					{node}
				</span>
			);
		}
		const Lucide = shellTabIcons.get(iconId);
		return Lucide ? <Lucide size={size} /> : <span style={{ display: "inline-block", width: size }} data-missing-icon={iconId} />;
	};
}

/** @emoji 📑 Converts framework side tabs into panel tab configs. */
export function shellSideTabsToPanelTabs(tabs: readonly ShellSideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
	void bus;
	return tabs.map((tab, orderIndex) => {
		const declarativeFactory = getDeclarativeSidePanelBodyFactory(tab.bodyKey);
		const Body = declarativeFactory
			? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey)
			: (sidePanelBodyByKey.get(tab.bodyKey) ?? (() => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>));
		return {
			id: tab.id,
			icon: shellTabIconComponent(tab.iconId),
			order: tab.order ?? orderIndex,
			tree: { sections: [{ id: `${tab.id}.body`, content: <Body /> }] },
		};
	});
}

/** @emoji 👣 Converts framework footer items into React footer rows. */
export function shellFooterToFooterItems(items: readonly ShellFooterItem[], bus: CommandBus): FooterItem[] {
	return items.map((item) => ({
		id: item.id,
		text: item.text,
		order: item.order,
		className: item.className,
		disabled: item.disabled,
		icon: item.iconId ? elementIconNodes.get(item.iconId) : undefined,
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	}));
}

function shellToolToToolbarItem(item: ShellToolItem, bus: CommandBus): UIToolbarItem {
	if (item.kind === "separator") {
		return { id: item.id, kind: "separator", order: item.order };
	}
	const iconNode = item.iconId ? elementIconNodes.get(item.iconId) : undefined;
	if (item.kind === "toggle") {
		return {
			id: item.id,
			kind: "toggle",
			icon: iconNode,
			label: item.label,
			text: item.text,
			order: item.order,
			pressed: item.pressed,
			onPressedChange: (pressed: boolean) => {
				if (item.controllerId && item.command) bus.dispatch(item.controllerId, item.command, { ...((item.args as object | undefined) ?? {}), pressed });
			},
		};
	}
	return {
		id: item.id,
		icon: iconNode,
		label: item.label,
		text: item.text,
		order: item.order,
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	};
}

/** @emoji 🧰 Converts framework toolbar maps into React toolbar items. */
export function shellToolsToAppTools(tools: ShellAppTools | undefined, bus: CommandBus): AppTools | undefined {
	if (!tools) return undefined;
	const merged: AppTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const list = tools[category];
		if (!list?.length) continue;
		merged[category] = list.map((entry) => shellToolToToolbarItem(entry, bus));
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 🔀 Merges config rows by `id` (extension overrides base). */
export function mergeConfigEntries<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}
