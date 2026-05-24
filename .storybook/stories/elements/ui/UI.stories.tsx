// #region 🧲Header
// 💻 .storybook/stories/elements/ui/UI.stories.tsx — Storybook harness for {@link WorkbenchView} + {@link Workbench}.
// #endregion 🧲Header

import {
	Controller,
	Workbench,
	WorkbenchApp,
	WorkbenchWindowKind,
	createDefaultLayout,
	type CommandBus,
	type ShellFindItem,
	type ShellSearchItemSpec,
	type ShellSideTabSpec,
	type ShellToolItem,
} from "@elements/framework";
import {
	WorkbenchView,
	registerElementIcon,
	registerSidePanelBody,
	registerWindowBody,
	type AppTools,
} from "@elements/framework-react";
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
	registerElementIcon("story.icon.file-text", <FileText size={16} />);
	registerElementIcon("story.icon.bar-chart", <BarChart size={16} />);
	registerElementIcon("story.icon.layers", <Layers size={16} />);
	registerElementIcon("story.icon.settings", <Settings size={16} />);
	registerElementIcon("story.icon.info", <Info size={16} />);
	registerElementIcon("story.icon.undo", <Undo size={14} />);
	registerElementIcon("story.icon.redo", <Redo size={14} />);
	registerElementIcon("story.icon.scissors", <Scissors size={14} />);
	registerElementIcon("story.icon.save", <Save size={14} />);
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
	registerSidePanelBody("story.panel.explorer", ExplorerTree);
	registerSidePanelBody("story.panel.properties", PropertiesTree);
	registerSidePanelBody("story.panel.metrics", MetricsTree);
	registerSidePanelBody("story.panel.settings", () => <div className="p-2">Settings content.</div>);
}

const editorFindItems: ShellFindItem[] = [
	{ id: "f1", label: "function handleClick", description: "Line 42", category: "Functions" },
	{ id: "f2", label: "function renderEditor", description: "Line 87", category: "Functions" },
];

const storySearchRows: ShellSearchItemSpec[] = [
	{ id: "s1", label: "index.ts", description: "Entry", category: "Files", iconId: "story.icon.file-text", controllerId: STORY_CTRL, command: "noop" },
	{ id: "s2", label: "Button.tsx", description: "Component", category: "Components", iconId: "story.icon.file-text", controllerId: STORY_CTRL, command: "noop" },
];

function shellToolsFromAppTools(tools: AppTools | undefined): Record<string, readonly ShellToolItem[]> | undefined {
	if (!tools) return undefined;
	const out: Record<string, readonly ShellToolItem[]> = {};
	for (const [category, list] of Object.entries(tools)) {
		out[category] = (list as { id: string; kind?: "separator"; icon?: React.ReactNode; label?: string; onClick?: () => void; order?: number }[]).map((item) => {
			if (item.kind === "separator") return { id: item.id, kind: "separator" as const, order: item.order };
			const iconKey = `story.tool.icon.${item.id}`;
			if (item.icon) registerElementIcon(iconKey, item.icon as React.ReactElement);
			return {
				id: item.id,
				kind: "button" as const,
				iconId: iconKey,
				label: item.label,
				order: item.order,
				controllerId: STORY_CTRL,
				command: "noop",
			};
		});
	}
	return out;
}

function buildTwoAppWorkbench(): Workbench {
	ensureStoryWorkbenchChrome();
	const wb = new Workbench();
	const ctrl = new StoryUiController(wb.commandBus, () => wb.notify());
	const editorTabsLeft: ShellSideTabSpec[] = [
		{ id: "explorer", iconId: "story.icon.layers", order: 0, bodyKey: "story.panel.explorer" },
		{ id: "settings", iconId: "story.icon.settings", order: 1, bodyKey: "story.panel.settings" },
	];
	const editorTabsRight: ShellSideTabSpec[] = [{ id: "properties", iconId: "story.icon.info", order: 0, bodyKey: "story.panel.properties" }];
	const editorTools = shellToolsFromAppTools({
		actions: [
			{ id: "undo", icon: <Undo size={14} />, label: "Undo", onClick: () => {}, order: 0 },
			{ id: "redo", icon: <Redo size={14} />, label: "Redo", onClick: () => {}, order: 1 },
			{ id: "save", icon: <Save size={14} />, label: "Save", onClick: () => {}, order: 5 },
		],
	} as AppTools);
	const editorApp = new WorkbenchApp(
		"editor",
		"Editor",
		"story.icon.file-text",
		ctrl,
		createDefaultLayout(["editor", "preview"], "row", [60, 40]) as never,
		[
			new WorkbenchWindowKind("editor", "Editor", "story.body.editor"),
			new WorkbenchWindowKind("preview", "Preview", "story.body.preview"),
		],
	);
	editorApp.leftTabs = editorTabsLeft;
	editorApp.rightTabs = editorTabsRight;
	editorApp.tools = editorTools ?? {};
	editorApp.findItems = editorFindItems;
	editorApp.onFindSelect = (itemId) => console.log("Find selected:", itemId);
	editorApp.footerItems = [
		{ id: "status", text: "Ready", order: 0 },
		{ id: "line", text: "Ln 42, Col 8", order: 1 },
	];
	const dashboardApp = new WorkbenchApp(
		"dashboard",
		"Dashboard",
		"story.icon.bar-chart",
		ctrl,
		createDefaultLayout(["stats"]) as never,
		[new WorkbenchWindowKind("stats", "Statistics", "story.body.stats")],
	);
	dashboardApp.leftTabs = [{ id: "metrics", iconId: "story.icon.bar-chart", order: 0, bodyKey: "story.panel.metrics" }];
	dashboardApp.footerItems = [{ id: "last-updated", text: "Updated 2m ago", order: 0 }];
	wb.addApp(editorApp);
	wb.addApp(dashboardApp);
	wb.searchItems = storySearchRows;
	wb.globalFooterItems = [{ id: "version", text: "v1.0.0", order: 100 }];
	return wb;
}

const meta = {
	title: "elements/react/UI",
	component: WorkbenchView,
	parameters: { layout: "fullscreen" },
	tags: ["autodocs"],
	render: () => <WorkbenchView workbench={buildTwoAppWorkbench()} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />,
} satisfies Meta<typeof WorkbenchView>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Mobile: Story = {
	render: () => <WorkbenchView workbench={buildTwoAppWorkbench()} mobile initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />,
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
		const wb = buildTwoAppWorkbench();
		wb.globalTools = shellToolsFromAppTools({
			actions: [{ id: "global-save", icon: <Save size={14} />, label: "Save All", onClick: () => {}, order: 100 }],
		} as AppTools);
		return <WorkbenchView workbench={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />;
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
