// #region 🧲Header
// 💻 .storybook/stories/elements/ui/UI.stories.tsx — Storybook harness for {@link PlaygroundView} + {@link ProductRuntime}.
// #endregion 🧲Header

import {
	Controller,
	ProductRuntime,
	AppRuntime,
	WindowKindRuntime,
	createDefaultLayout,
	type CommandBus,
	type FindItem,
	type SearchItemSpec,
	type AppTools,
} from "@elements/playground";
import {
	PlaygroundView,
	PureSidePanelTabDefinition,
	StaticTreePanelDefinition,
	registerTabIcon,
	registerWindowBody,
	type SidePanelTabConfig,
} from "@elements/playground/react";
import { Tree } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { BarChart, File, FileText, FolderOpen, Info, Layers, Redo, Save, Scissors, Settings, Undo } from "lucide-react";
import * as React from "react";
import { expect, userEvent, within } from "storybook/test";

const STORY_CTRL = "story-ui";

class StoryUiController extends Controller {
	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(STORY_CTRL, commandBus, hostNotify);
	}

	override run(_command: string, _args?: unknown): void {}
}

const ExplorerTree: React.FC = () => (
	<Tree
		sections={[
			{
				id: "explorer.src",
				label: "src",
				icon: <FolderOpen size={14} />,
				defaultOpen: true,
				items: [
					{ id: "explorer.src.index", label: "index.ts", icon: <File size={14} /> },
					{ id: "explorer.src.app", label: "app.tsx", icon: <File size={14} /> },
				],
			},
		]}
	/>
);

const PropertiesTree: React.FC = () => (
	<Tree
		sections={[
			{
				id: "properties.element",
				label: "Element",
				icon: <Info size={14} />,
				defaultOpen: true,
				items: [{ id: "properties.element.id", label: "id: editor-1" }],
			},
		]}
	/>
);

const MetricsTree: React.FC = () => (
	<Tree
		sections={[
			{
				id: "metrics.performance",
				label: "Performance",
				icon: <BarChart size={14} />,
				defaultOpen: true,
				items: [{ id: "metrics.performance.fps", label: "FPS: 60" }],
			},
		]}
	/>
);

let storyChromeReady = false;

function ensureStoryWorkbenchChrome(): void {
	if (storyChromeReady) return;
	storyChromeReady = true;
	registerTabIcon("story.icon.file-text", FileText);
	registerTabIcon("story.icon.bar-chart", BarChart);
	registerTabIcon("story.icon.layers", Layers);
	registerTabIcon("story.icon.settings", Settings);
	registerTabIcon("story.icon.info", Info);
	registerTabIcon("story.icon.undo", Undo);
	registerTabIcon("story.icon.redo", Redo);
	registerTabIcon("story.icon.scissors", Scissors);
	registerTabIcon("story.icon.save", Save);
	registerWindowBody("story.body.editor", () => (
		<div className="flex h-full items-center justify-center bg-window">
			<h2 className="text-xl font-bold">Editor Window</h2>
		</div>
	));
	registerWindowBody("story.body.preview", () => (
		<div className="flex h-full items-center justify-center bg-panel">
			<h2 className="text-xl font-bold">Preview Window</h2>
		</div>
	));
	registerWindowBody("story.body.stats", () => (
		<div className="flex h-full items-center justify-center bg-window">
			<h2 className="text-xl font-bold">Statistics</h2>
		</div>
	));
}

class StoryExplorerTabDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "explorer",
			icon: Layers,
			order: 0,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "story.explorer.section",
						label: "Explorer",
						defaultOpen: true,
						items: [{ id: "story.explorer.body", label: "Files", description: <ExplorerTree /> }],
					},
				],
			}),
		};
	}
}

class StorySettingsTabDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "settings",
			icon: Settings,
			order: 1,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "story.settings.section",
						label: "Settings",
						defaultOpen: true,
						items: [{ id: "story.settings.body", label: "Settings", description: <div className="p-2">Settings content.</div> }],
					},
				],
			}),
		};
	}
}

class StoryMetricsTabDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "metrics",
			icon: BarChart,
			order: 0,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "story.metrics.section",
						label: "Metrics",
						defaultOpen: true,
						items: [{ id: "story.metrics.body", label: "Metrics", description: <MetricsTree /> }],
					},
				],
			}),
		};
	}
}

class StoryPropertiesTabDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "properties",
			icon: Info,
			order: 0,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "story.properties.section",
						label: "Properties",
						defaultOpen: true,
						items: [{ id: "story.properties.body", label: "Properties", description: <PropertiesTree /> }],
					},
				],
			}),
		};
	}
}

function storyPanelTabs(): { readonly workbench: readonly SidePanelTabConfig[]; readonly details: readonly SidePanelTabConfig[] } {
	return {
		workbench: [new StoryExplorerTabDefinition().resolveTab(), new StorySettingsTabDefinition().resolveTab()],
		details: [new StoryPropertiesTabDefinition().resolveTab()],
	};
}

function storyDashboardPanelTabs(): { readonly workbench: readonly SidePanelTabConfig[] } {
	return {
		workbench: [new StoryMetricsTabDefinition().resolveTab()],
	};
}

const editorFindItems: FindItem[] = [
	{ id: "f1", label: "function handleClick", description: "Line 42", category: "Functions" },
	{ id: "f2", label: "function renderEditor", description: "Line 87", category: "Functions" },
];

const storySearchRows: SearchItemSpec[] = [
	{ id: "s1", label: "index.ts", description: "Entry", category: "Files", iconId: "story.icon.file-text", controllerId: STORY_CTRL, command: "noop" },
	{ id: "s2", label: "Button.tsx", description: "Component", category: "Components", iconId: "story.icon.file-text", controllerId: STORY_CTRL, command: "noop" },
];

function buildTwoAppRuntime(): ProductRuntime {
	ensureStoryWorkbenchChrome();
	const runtime = new ProductRuntime();
	const ctrl = new StoryUiController(runtime.commandBus, () => runtime.notify());
	const editorTools: AppTools = {
		actions: [
			{ id: "undo", kind: "button", iconId: "story.icon.undo", label: "Undo", order: 0, controllerId: STORY_CTRL, command: "noop" },
			{ id: "redo", kind: "button", iconId: "story.icon.redo", label: "Redo", order: 1, controllerId: STORY_CTRL, command: "noop" },
			{ id: "save", kind: "button", iconId: "story.icon.save", label: "Save", order: 5, controllerId: STORY_CTRL, command: "noop" },
		],
	};
	const editorApp = new AppRuntime(
		"editor",
		"Editor",
		"story.icon.file-text",
		ctrl,
		createDefaultLayout(["editor", "preview"], "row", [60, 40]) as never,
		[
			new WindowKindRuntime("editor", "Editor", "story.body.editor"),
			new WindowKindRuntime("preview", "Preview", "story.body.preview"),
		],
	);
	editorApp.leftTabs = [];
	editorApp.rightTabs = [];
	editorApp.tools = editorTools;
	editorApp.findItems = editorFindItems;
	editorApp.onFindSelect = (itemId) => console.log("Find selected:", itemId);
	editorApp.footerItems = [
		{ id: "status", text: "Ready", order: 0 },
		{ id: "line", text: "Ln 42, Col 8", order: 1 },
	];
	const dashboardApp = new AppRuntime(
		"dashboard",
		"Dashboard",
		"story.icon.bar-chart",
		ctrl,
		createDefaultLayout(["stats"]) as never,
		[new WindowKindRuntime("stats", "Statistics", "story.body.stats")],
	);
	dashboardApp.leftTabs = [];
	dashboardApp.rightTabs = [];
	dashboardApp.footerItems = [{ id: "last-updated", text: "Updated 2m ago", order: 0 }];
	runtime.addApp(editorApp);
	runtime.addApp(dashboardApp);
	runtime.searchItems = storySearchRows;
	runtime.globalFooterItems = [{ id: "version", text: "v1.0.0", order: 100 }];
	return runtime;
}

const meta = {
	title: "elements/react/UI",
	component: PlaygroundView,
	parameters: { layout: "fullscreen" },
	tags: ["autodocs"],
	render: () => (
		<PlaygroundView
			runtime={buildTwoAppRuntime()}
			augmentPanelTabs={storyPanelTabs()}
			initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
		/>
	),
} satisfies Meta<typeof PlaygroundView>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Mobile: Story = {
	render: () => (
		<PlaygroundView
			runtime={buildTwoAppRuntime()}
			augmentPanelTabs={storyPanelTabs()}
			mobile
			initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
		/>
	),
	parameters: {
		viewport: { defaultViewport: "mobile1" },
		layout: "fullscreen",
	},
	decorators: [
		(Story) => (
			<div style={{ width: "375px", height: "667px", overflow: "hidden", border: "1px solid var(--border-color)" }}>
				<Story />
			</div>
		),
	],
	play: async ({ canvasElement }) => {
		const documentBody = canvasElement.ownerDocument.body;
		const mobilePanelToggle = canvasElement.ownerDocument.getElementById("ui.panelToggle.workbench");
		expect(mobilePanelToggle).toBeTruthy();
		await userEvent.click(mobilePanelToggle!);
		expect(documentBody.querySelector('[data-panel="mobilePanel"]')).toBeTruthy();
		expect(within(documentBody).getByText("src")).toBeTruthy();
	},
};

export const FullFeatured: Story = {
	render: () => {
		const runtime = buildTwoAppRuntime();
		runtime.globalTools = {
			actions: [{ id: "global-save", kind: "button", iconId: "story.icon.save", label: "Save All", order: 100, controllerId: STORY_CTRL, command: "noop" }],
		};
		return (
			<PlaygroundView
				runtime={runtime}
				augmentPanelTabs={{
					workbench: [...storyPanelTabs().workbench, ...storyDashboardPanelTabs().workbench],
					details: storyPanelTabs().details,
				}}
				initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
			/>
		);
	},
	play: async ({ canvasElement }) => {
		const documentBody = canvasElement.ownerDocument.body;
		const workbenchToggle = canvasElement.ownerDocument.getElementById("ui.panelToggle.workbench");
		const searchToggle = canvasElement.ownerDocument.getElementById("ui.search.toggle");
		const findToggle = canvasElement.ownerDocument.getElementById("ui.find.toggle");
		expect(workbenchToggle).toBeTruthy();
		expect(searchToggle).toBeTruthy();
		expect(findToggle).toBeTruthy();
		await userEvent.click(workbenchToggle!);
		expect(documentBody.querySelector('[data-panel="leftSidePanel"]')).toBeTruthy();
		expect(within(documentBody).getByText("src")).toBeTruthy();
		await userEvent.click(searchToggle!);
		expect(canvasElement.ownerDocument.getElementById("ui.search.input")).toBeTruthy();
		await userEvent.click(searchToggle!);
		await userEvent.click(findToggle!);
		expect(canvasElement.ownerDocument.getElementById("ui.find.input")).toBeTruthy();
	},
};
