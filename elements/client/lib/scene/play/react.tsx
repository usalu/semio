// #region 🧲Header
// 💻 elements/client/lib/scene/play/react.tsx — React runtime for scene play: surface controls, LOD providers, scene canvas, and shell mounting.
// #endregion 🧲Header

import { useGLTF } from "@react-three/drei";
import {
	applyElementsSurfaceChrome,
	Button,
	Controller,
	Expertise,
	LevelProvider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	Workbench,
	WorkbenchApp,
	WorkbenchWindowKind,
	WorkbenchView,
	createStackLayout,
	getLevelBgClass,
	type CommandBus,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type FooterItem,
	type UIWindowKindDefinition,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";
import * as React from "react";

import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import "./globals.css";
import {
	LS_DEVICE,
	LS_EXPERTISE,
	LS_THEME,
	PLAY_APP_ID,
	PLAY_LOD_TIERS,
	parseKindCatalogs,
	parseKindCompatibility,
	parseStoredDevice,
	parseStoredExpertise,
	parseStoredTheme,
	playLodTierMenuLabel,
} from "./index.ts";
import {
	Canvas3D,
	LOD_MODE_AUTOMATIC,
	SceneObjectStateContext,
	SceneAttractions,
	SceneObjectStateProvider,
	SceneObjects,
	ScenePlayTestBridge,
	blockedVortexFullIdsFromAttractions,
	isLodKind,
	lodAutomaticSelectLabel,
	lodCanvasProps,
	parseFixtureV1,
	type CanvasProps,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type LodKind,
	type LodModeKind,
	type SceneObjectStateContextValue,
	type RelocateMode,
} from "../index.tsx";

const PlayLodContext = React.createContext<Pick<CanvasProps, "automaticLod" | "lod">>({ automaticLod: true });

const PlayLodDisplayContext = React.createContext<LodKind>("normal");

const PlayRuntimeContext = React.createContext<{
	readonly setEffectiveLod: (lod: LodKind) => void;
} | null>(null);

const SCENE_PLAY_SHELL_CONTROLLER_ID = "scene-play";

class ScenePlayShellController extends Controller {
	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SCENE_PLAY_SHELL_CONTROLLER_ID, commandBus, hostNotify);
	}

	override run(_command: string, _args?: unknown): void {}
}

function buildScenePlayWorkbenchApp(controller: ScenePlayShellController): WorkbenchApp {
	return new WorkbenchApp(
		PLAY_APP_ID,
		"Scene play",
		undefined,
		controller,
		createStackLayout(["scene-main"], ["Scene"]) as never,
		[new WorkbenchWindowKind("scene-main", "Scene", "elements.scene.placeholder")],
	);
}

function readTheme(): ElementsSurfaceTheme {
	if (typeof localStorage === "undefined") return "system";
	try {
		return parseStoredTheme(localStorage.getItem(LS_THEME));
	} catch {
		return "system";
	}
}

function readDevice(): ElementsSurfaceDevice {
	if (typeof localStorage === "undefined") return "desktop";
	try {
		return parseStoredDevice(localStorage.getItem(LS_DEVICE));
	} catch {
		return "desktop";
	}
}

function readExpertise(): Expertise {
	if (typeof localStorage === "undefined") return Expertise.NORMAL;
	try {
		return parseStoredExpertise(localStorage.getItem(LS_EXPERTISE));
	} catch {
		return Expertise.NORMAL;
	}
}

class PlaySurfaceFooter extends React.Component<{
	theme: ElementsSurfaceTheme;
	device: ElementsSurfaceDevice;
	expertise: Expertise;
	onTheme: (v: ElementsSurfaceTheme) => void;
	onDevice: (v: ElementsSurfaceDevice) => void;
	onExpertise: (v: Expertise) => void;
}> {
	render(): React.ReactElement {
		const { theme, device, expertise, onDevice, onExpertise, onTheme } = this.props;
		return (
			<div className="flex min-w-0 flex-wrap items-center gap-double px-single py-tiny">
				<span className="shrink-0 text-xs text-muted-foreground">Theme</span>
				<Select onValueChange={(v) => onTheme(v as ElementsSurfaceTheme)} value={theme}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-theme" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="system">System</SelectItem>
						<SelectItem value="light">Light</SelectItem>
						<SelectItem value="dark">Dark</SelectItem>
					</SelectContent>
				</Select>
				<span className="shrink-0 text-xs text-muted-foreground">Device</span>
				<Select onValueChange={(v) => onDevice(v as ElementsSurfaceDevice)} value={device}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-device" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="desktop">Desktop</SelectItem>
						<SelectItem value="tablet">Tablet</SelectItem>
						<SelectItem value="mobile">Mobile</SelectItem>
					</SelectContent>
				</Select>
				<span className="shrink-0 text-xs text-muted-foreground">Expertise</span>
				<Select onValueChange={(v) => onExpertise(v as Expertise)} value={expertise}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-expertise" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value={Expertise.BEGINNER}>Beginner</SelectItem>
						<SelectItem value={Expertise.NORMAL}>Normal</SelectItem>
						<SelectItem value={Expertise.EXPERT}>Expert</SelectItem>
					</SelectContent>
				</Select>
			</div>
		);
	}
}

function windowKindsWithLodMeasures(
	lodMode: LodModeKind,
	setLodMode: (mode: LodModeKind) => void,
): UIWindowKindDefinition[] {
	return [
		{
			id: "scene-main",
			label: "Scene",
			component: MainWindow,
			measures: [
				{
					id: "scene-main-lod",
					items: [
						{ id: "automatic", label: lodAutomaticSelectLabel("normal"), value: LOD_MODE_AUTOMATIC },
						...PLAY_LOD_TIERS.map((tier) => ({ id: tier, label: playLodTierMenuLabel(tier), value: tier })),
					],
					kind: "select",
					label: "LOD",
					onValueChange: (value) => {
						if (value === LOD_MODE_AUTOMATIC || isLodKind(value)) {
							setLodMode(value as LodModeKind);
						}
					},
					value: lodMode,
				},
			],
		},
	];
}

interface PlayBodyProps {
	readonly fixture: FixtureV1;
	readonly lodProps: Pick<CanvasProps, "automaticLod" | "lod">;
	readonly lodTag: LodKind;
	readonly runtime: { readonly setEffectiveLod: (lod: LodKind) => void } | null;
}

interface PlayBodyState {
	readonly relocateMode: RelocateMode;
	readonly selectedId: string | null;
	readonly proximityCount: number;
	readonly connectCount: number;
	readonly indirectCount: number;
}

class PlayBody extends React.Component<PlayBodyProps, PlayBodyState> {
	state: PlayBodyState = {
		relocateMode: "translate",
		selectedId: null,
		proximityCount: 0,
		connectCount: 0,
		indirectCount: 0,
	};

	componentDidMount(): void {
		this.preloadMeshes();
	}

	componentDidUpdate(prevProps: Readonly<PlayBodyProps>): void {
		if (prevProps.fixture.objects !== this.props.fixture.objects) {
			this.preloadMeshes();
		}
	}

	private preloadMeshes(): void {
		const urls = [...new Set(this.props.fixture.objects.map((object) => object.meshUrl))];
		for (const url of urls) {
			useGLTF.preload(url);
		}
	}

	private readonly onSelect = (snap: { objectIds: readonly string[] }): void => {
		this.setState({ selectedId: snap.objectIds[0] ?? null });
	};

	private readonly onProximityConnect = (): void => {
		this.setState((current) => ({ proximityCount: current.proximityCount + 1 }));
	};

	private readonly onConnect = (): void => {
		this.setState((current) => ({ connectCount: current.connectCount + 1 }));
	};

	private readonly onIndirectConnect = (): void => {
		this.setState((current) => ({ indirectCount: current.indirectCount + 1 }));
	};

	render(): React.ReactElement {
		const { fixture, lodProps, lodTag, runtime } = this.props;
		const kindCompatibility = parseKindCompatibility(fixture.meta);
		const kindCatalogs = parseKindCatalogs(fixture.meta);
		const blockedVortexFullIds = blockedVortexFullIdsFromAttractions(fixture.attractions);
		return (
			<div className="flex h-full w-full flex-col">
				<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
					<ToolbarZone>
						<ToolbarGroup>
							<ToolbarItem>
								<Button variant={this.state.relocateMode === "translate" ? "default" : "outline"} size="sm" onClick={() => this.setState({ relocateMode: "translate" })}>
									<Move3d className="mr-1 size-4" />
									Translate
								</Button>
							</ToolbarItem>
							<ToolbarItem>
								<Button variant={this.state.relocateMode === "rotate" ? "default" : "outline"} size="sm" onClick={() => this.setState({ relocateMode: "rotate" })}>
									<Rotate3d className="mr-1 size-4" />
									Rotate
								</Button>
							</ToolbarItem>
							<ToolbarItem>
								<Button variant={this.state.relocateMode === "scale" ? "default" : "outline"} size="sm" onClick={() => this.setState({ relocateMode: "scale" })}>
									<Scaling className="mr-1 size-4" />
									Scale
								</Button>
							</ToolbarItem>
						</ToolbarGroup>
					</ToolbarZone>
					<div className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
						<span data-e2e-selected>{this.state.selectedId ?? "—"}</span>
						<span data-e2e-scene-lod>{lodTag}</span>
						<span data-e2e-proximity-count>{this.state.proximityCount}</span>
						<span data-e2e-connect-count>{this.state.connectCount}</span>
						<span data-e2e-indirect-count>{this.state.indirectCount}</span>
					</div>
				</div>
				<div className="relative min-h-0 flex-1">
					<SceneObjectStateProvider fixture={fixture} onConnect={this.onConnect}>
						<PlaySceneCanvas
							fixture={fixture}
							kindCatalogs={kindCatalogs}
							kindCompatibility={kindCompatibility}
							blockedVortexFullIds={blockedVortexFullIds}
							lodProps={lodProps}
							relocateMode={this.state.relocateMode}
							runtime={runtime}
							selectedId={this.state.selectedId}
							setSelectedId={(selectedId) => this.setState({ selectedId })}
							onSelect={this.onSelect}
							onIndirectConnect={this.onIndirectConnect}
							onProximityConnect={this.onProximityConnect}
						/>
					</SceneObjectStateProvider>
				</div>
			</div>
		);
	}
}

class PlayBodyHost extends React.Component<{ readonly fixture: FixtureV1; readonly lodProps: Pick<CanvasProps, "automaticLod" | "lod">; }> {
	render(): React.ReactElement {
		return (
			<PlayLodDisplayContext.Consumer>
				{(lodTag) => <PlayRuntimeContext.Consumer>{(runtime) => <PlayBody fixture={this.props.fixture} lodProps={this.props.lodProps} lodTag={lodTag} runtime={runtime} />}</PlayRuntimeContext.Consumer>}
			</PlayLodDisplayContext.Consumer>
		);
	}
}

class PlaySceneCanvasContent extends React.Component<{
	readonly selectedId: string | null;
	readonly relocateMode: RelocateMode;
	readonly setSelectedId: (id: string | null) => void;
}> {
	render(): React.ReactElement {
		return (
			<>
				<ScenePlayTestBridge setSelectedId={this.props.setSelectedId} />
				<React.Suspense fallback={null}>
					<SceneObjects selectedObjectId={this.props.selectedId} relocate={this.props.relocateMode} />
					<SceneAttractions />
				</React.Suspense>
			</>
		);
	}
}

class PlaySceneCanvas extends React.Component<{
	readonly fixture: FixtureV1;
	readonly kindCatalogs: KindCatalogBundle | undefined;
	readonly kindCompatibility: readonly KindCompatEntry[];
	readonly blockedVortexFullIds: ReadonlySet<string>;
	readonly lodProps: Pick<CanvasProps, "automaticLod" | "lod">;
	readonly relocateMode: RelocateMode;
	readonly runtime: { readonly setEffectiveLod: (lod: LodKind) => void } | null | undefined;
	readonly selectedId: string | null;
	readonly setSelectedId: (id: string | null) => void;
	readonly onSelect: (snap: { objectIds: readonly string[] }) => void;
	readonly onIndirectConnect: () => void;
	readonly onProximityConnect: () => void;
}> {
	static contextType = SceneObjectStateContext;
	declare context: React.ContextType<typeof SceneObjectStateContext>;

	render(): React.ReactElement {
		const state = this.context as SceneObjectStateContextValue | null;
		if (!state) {
			throw new Error("SceneObjectStateProvider missing");
		}
		return (
			<Canvas3D
				className="absolute inset-0"
				camera={this.props.fixture.camera}
				domain={this.props.fixture.domain}
				kindCatalogs={this.props.kindCatalogs}
				kindCompatibility={this.props.kindCompatibility}
				blockedVortexFullIds={this.props.blockedVortexFullIds}
				proximityRadius={24}
				relocateMode={this.props.relocateMode}
				showLodGrid
				gridSnapEnabled
				{...this.props.lodProps}
				onLodChange={this.props.runtime?.setEffectiveLod}
				onSelect={this.props.onSelect}
				onConnect={state.handleConnect}
				onIndirectConnect={this.props.onIndirectConnect}
				onProximityConnect={this.props.onProximityConnect}
				onRelocate={state.handleRelocate}
			>
				<PlaySceneCanvasContent relocateMode={this.props.relocateMode} selectedId={this.props.selectedId} setSelectedId={this.props.setSelectedId} />
			</Canvas3D>
		);
	}
}

class MainWindow extends React.Component {
	static contextType = PlayLodContext;
	declare context: React.ContextType<typeof PlayLodContext>;
	private readonly fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);

	render(): React.ReactElement {
		if (!this.fixture) {
			return <div className="p-4 text-destructive">Invalid scene fixture</div>;
		}
		return <PlayBodyHost fixture={this.fixture} lodProps={this.context} />;
	}
}

interface PlayInnerState {
	readonly theme: ElementsSurfaceTheme;
	readonly device: ElementsSurfaceDevice;
	readonly expertise: Expertise;
	readonly lodMode: LodModeKind;
	readonly lodTag: LodKind;
}

class PlayInner extends React.Component<{}, PlayInnerState> {
	state: PlayInnerState = {
		theme: readTheme(),
		device: readDevice(),
		expertise: readExpertise(),
		lodMode: LOD_MODE_AUTOMATIC,
		lodTag: "normal",
	};

	private cleanupSurfaceChrome: (() => void) | null = null;

	private boardWorkbench: Workbench | null = null;

	private readonly runtime = {
		setEffectiveLod: (lod: LodKind) => {
			this.setState((current) => ({ lodTag: current.lodTag === lod ? current.lodTag : lod }));
		},
	};

	componentDidMount(): void {
		this.applySurfaceChrome();
		this.persistState();
	}

	componentDidUpdate(_prevProps: {}, prevState: Readonly<PlayInnerState>): void {
		if (prevState.theme !== this.state.theme || prevState.device !== this.state.device || prevState.expertise !== this.state.expertise) {
			this.applySurfaceChrome();
			this.persistState();
		}
	}

	componentWillUnmount(): void {
		this.cleanupSurfaceChrome?.();
	}

	private applySurfaceChrome(): void {
		this.cleanupSurfaceChrome?.();
		this.cleanupSurfaceChrome = applyElementsSurfaceChrome({
			theme: this.state.theme,
			device: this.state.device,
			expertise: this.state.expertise,
		});
	}

	private persistState(): void {
		try {
			localStorage.setItem(LS_THEME, this.state.theme);
			localStorage.setItem(LS_DEVICE, this.state.device);
			localStorage.setItem(LS_EXPERTISE, this.state.expertise);
		} catch {}
	}

	render(): React.ReactElement {
		const surfaceFooterItems: FooterItem[] = [
			{
				content: <PlaySurfaceFooter device={this.state.device} expertise={this.state.expertise} onDevice={(device) => this.setState({ device })} onExpertise={(expertise) => this.setState({ expertise })} onTheme={(theme) => this.setState({ theme })} theme={this.state.theme} />,
				id: "scene-play-surface",
				order: 0,
			},
		];
		const lodProps = lodCanvasProps(this.state.lodMode);
		const windowKinds = windowKindsWithLodMeasures(this.state.lodMode, (lodMode) => this.setState({ lodMode }));
		if (!this.boardWorkbench) {
			const wb = new Workbench();
			const ctrl = new ScenePlayShellController(wb.commandBus, () => wb.notify());
			wb.addApp(buildScenePlayWorkbenchApp(ctrl));
			this.boardWorkbench = wb;
		}
		const workbench = this.boardWorkbench;
		return (
			<PlayLodDisplayContext.Provider value={this.state.lodTag}>
				<PlayLodContext.Provider value={lodProps}>
					<PlayRuntimeContext.Provider value={this.runtime}>
						<WorkbenchView
							workbench={workbench}
							defaultAppId={PLAY_APP_ID}
							extraFooterItems={surfaceFooterItems}
							mobile={this.state.device === "mobile"}
							resolvedWindowKindsOverride={windowKinds}
						/>
					</PlayRuntimeContext.Provider>
				</PlayLodContext.Provider>
			</PlayLodDisplayContext.Provider>
		);
	}
}

class PlayApp extends React.Component {
	render(): React.ReactElement {
		return (
			<LevelProvider level="window">
				<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
					<PlayInner />
				</div>
			</LevelProvider>
		);
	}
}

export function createScenePlayElement(): React.ReactElement {
	return <PlayApp />;
}