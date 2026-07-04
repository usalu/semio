import { useMemo, useSyncExternalStore, type ReactNode } from "react";
import {
	Button,
	Icon,
	Input,
	type SidePanelTabConfig,
	type TreeDataItem,
	type TreePanelConfig,
} from "@semio-tech/ui-react";
import {
	createNamedLayout,
	type NamedLayout,
	type WindowLayout,
} from "@semio-tech/framework-core";

//#region DisplayPanel
export type DisplayHostApi = {
	readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
	readonly namedLayouts: readonly NamedLayout[];
	readonly userLayouts: readonly NamedLayout[];
	readonly saveCurrentLayout: (label: string) => void;
	readonly applyNamedLayout: (layoutId: string) => void;
	readonly deleteUserLayout: (layoutId: string) => void;
};

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";

let displayLayoutSaveLabel = "";

function groupNamedLayoutsToTreeItems(
	layouts: readonly NamedLayout[],
	onApply: (layoutId: string) => void,
	onDeleteUser?: (layoutId: string) => void,
): TreeDataItem[] {
	const root: TreeDataItem[] = [];
	const folderByKey = new Map<string, TreeDataItem>();
	const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
		id: `framework.display.layout.${entry.id}`,
		label: entry.label,
		onClick: () => onApply(entry.id),
		...(entry.origin === "user" && onDeleteUser
			? {
					actions: [
						{
							id: `framework.display.delete.${entry.id}`,
							icon: <Icon icon="trash-2" size="small" />,
							onClick: () => onDeleteUser(entry.id),
						},
					],
				}
			: {}),
	});
	for (const entry of layouts) {
		if (!entry.groupPath?.length) {
			root.push(layoutLeaf(entry));
			continue;
		}
		let siblings = root;
		let pathKey = "";
		for (let index = 0; index < entry.groupPath.length; index += 1) {
			const segment = entry.groupPath[index]!;
			pathKey = pathKey ? `${pathKey}/${segment}` : segment;
			let folder = folderByKey.get(pathKey);
			if (!folder) {
				folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
				folderByKey.set(pathKey, folder);
				siblings.push(folder);
			}
			const folderItems = folder.items ?? (folder.items = []);
			if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
			else siblings = folderItems;
		}
	}
	return root;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
	return {
		sections: host.windowKinds.length
			? host.windowKinds.map((kind) => ({
					id: `framework.display.windows.${kind.id}`,
					label: kind.label,
					defaultOpen: false,
					items: [{ id: `framework.display.windows.${kind.id}.kind`, label: kind.label }],
				}))
			: [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
	};
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
	const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
	const userLayouts = host.userLayouts;
	const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
	const userItems = userLayouts.length
		? [
				{
					id: "framework.display.layout.group.saved",
					label: "Saved",
					defaultOpen: false,
					items: groupNamedLayoutsToTreeItems(userLayouts, (layoutId) => host.applyNamedLayout(layoutId), (layoutId) => host.deleteUserLayout(layoutId)),
				},
			]
		: [];
	return {
		sections: [
			{
				id: "framework.display.layout.save",
				label: "Save layout",
				defaultOpen: false,
				items: [
					{
						id: "framework.display.layout.save.label",
						label: "Name",
						control: (
							<Input
								id="framework.display.save-label"
								defaultValue={displayLayoutSaveLabel}
								onChange={(event) => {
									displayLayoutSaveLabel = event.target.value;
								}}
								placeholder="Layout name"
							/>
						),
					},
					{
						id: "framework.display.layout.save.action",
						label: "Save",
						control: (
							<Button
								id="framework.display.save"
								size="sm"
								text="Save current layout"
								disabled={!displayLayoutSaveLabel.trim()}
								onClick={() => {
									const label = displayLayoutSaveLabel.trim();
									if (!label) return;
									host.saveCurrentLayout(label);
									displayLayoutSaveLabel = "";
								}}
							/>
						),
					},
				],
			},
			{
				id: "framework.display.layout.list",
				label: "Layouts",
				defaultOpen: true,
				items: [...builtinItems, ...userItems],
			},
		],
	};
}

function shellTabIcon(iconId: string): React.FC<{ size?: number }> {
	return function ShellTabIcon({ size = 16 }: { size?: number }) {
		return <Icon icon="display" size={size} />;
	};
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): SidePanelTabConfig[] {
	return [
		{
			id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
			icon: shellTabIcon("framework.display.windows"),
			name: "Windows",
			order: -100,
			tree: {
				resolveTree: () => {
					const host = getHost();
					return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
				},
			},
		},
		{
			id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
			icon: shellTabIcon("framework.display.layout"),
			name: "Layout",
			order: -99,
			tree: {
				resolveTree: () => {
					const host = getHost();
					return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
				},
			},
		},
	];
}
//#endregion DisplayPanel

//#region SettingsPanel
export type SettingsHostApi = {
	readonly compact: boolean;
	readonly setCompact: (compact: boolean) => void;
	readonly expertise: string;
	readonly setExpertise: (expertise: string) => void;
	readonly theme: string;
	readonly setTheme: (theme: string) => void;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
	return {
		sections: [
			{
				id: "framework.settings.general",
				label: "General",
				defaultOpen: true,
				items: [
					{
						id: "framework.settings.theme",
						label: "Theme",
						control: (
							<select
								id="framework.settings.theme"
								className="h-small w-full rounded border border-border bg-background px-2 text-sm"
								value={host.theme}
								onChange={(event) => host.setTheme(event.target.value)}
							>
								<option value="system">System</option>
								<option value="light">Light</option>
								<option value="dark">Dark</option>
							</select>
						),
					},
					{
						id: "framework.settings.compact",
						label: "Compact UI",
						control: (
							<input
								id="framework.settings.compact"
								type="checkbox"
								checked={host.compact}
								onChange={(event) => host.setCompact(event.target.checked)}
							/>
						),
					},
					{
						id: "framework.settings.expertise",
						label: "Expertise",
						control: (
							<select
								id="framework.settings.expertise"
								className="h-small w-full rounded border border-border bg-background px-2 text-sm"
								value={host.expertise}
								onChange={(event) => host.setExpertise(event.target.value)}
							>
								<option value="beginner">Beginner</option>
								<option value="normal">Normal</option>
								<option value="expert">Expert</option>
							</select>
						),
					},
				],
			},
		],
	};
}

export function createFrameworkSettingsPanelTab(getHost: () => SettingsHostApi | null): SidePanelTabConfig {
	return {
		id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
		icon: shellTabIcon("framework.settings.general"),
		name: "Settings",
		order: -98,
		tree: {
			resolveTree: () => {
				const host = getHost();
				return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: "Settings unavailable" }] }] };
			},
		},
	};
}

export function useNamedLayoutHost(options: {
	readonly appId: string;
	readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
	readonly builtinLayouts: readonly NamedLayout[];
	readonly currentLayout: WindowLayout | undefined;
	readonly onApplyLayout: (layout: WindowLayout) => void;
	readonly namedLayoutStore: { getSnapshot: () => readonly NamedLayout[]; save: (layout: NamedLayout) => void; remove: (layoutId: string) => void; subscribe: (listener: () => void) => () => void };
}): DisplayHostApi {
	const userLayouts = useSyncExternalStore(
		(listener) => options.namedLayoutStore.subscribe(listener),
		() => options.namedLayoutStore.getSnapshot(),
		() => options.namedLayoutStore.getSnapshot(),
	);
	return useMemo(
		(): DisplayHostApi => ({
			windowKinds: options.windowKinds,
			namedLayouts: options.builtinLayouts,
			userLayouts,
			saveCurrentLayout: (label) => {
				if (!options.currentLayout) return;
				const id = `user-${Date.now()}`;
				options.namedLayoutStore.save(createNamedLayout(id, label, options.currentLayout, "user"));
			},
			applyNamedLayout: (layoutId) => {
				const layout = [...options.builtinLayouts, ...userLayouts].find((entry) => entry.id === layoutId);
				if (layout) options.onApplyLayout(layout.layout);
			},
			deleteUserLayout: (layoutId) => options.namedLayoutStore.remove(layoutId),
		}),
		[options, userLayouts],
	);
}
//#endregion SettingsPanel
