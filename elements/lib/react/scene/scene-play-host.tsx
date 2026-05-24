// #region ­ƒº▓Header
// ­ƒÆ╗ elements/client/lib/system/renderer/react/scene/scene-play-host.tsx ÔÇö Host outside play bundle: scene play React tree and mount.
// #endregion ­ƒº▓Header

import { useGLTF } from "@react-three/drei";
import type { UiScene3DHostSurfaceNode } from "@elements/framework";
import {
	LevelProvider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	applyElementsSurfaceChrome,
	getLevelBgClass,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type FooterItem,
} from "@elements/ui";
import { Expertise, Workbench, registerDeclarativeWindowBody } from "@elements/framework";
import { WorkbenchView, mountReactApp, registerUiScene3DSurfaceHost, useApp } from "@elements/framework-react";
import * as React from "react";

import nakaginSceneFixtureJson from "./play/fixtures/nakagin-capsule-tower.scene.json";
import "./play/globals.css";
import {
	LS_DEVICE,
	LS_EXPERTISE,
	LS_THEME,
	PLAY_APP_ID,
	SCENE_PLAY_BODY_KEY,
	SCENE_PLAY_CONTROLLER_ID,
	SCENE_PLAY_SCENE_SURFACE_ID,
	ScenePlayShellController,
	buildScenePlayDeclarativeBody,
	buildScenePlayWorkbenchApp,
	parseKindCatalogs,
	parseKindCompatibility,
	parseStoredDevice,
	parseStoredExpertise,
	parseStoredTheme,
	type ScenePlaySnapshot,
} from "./play/index.ts";
import {
	Canvas3D,
	SceneObjectStateContext,
	SceneAttractions,
	SceneObjectStateProvider,
	SceneObjects,
	ScenePlayTestBridge,
	blockedVortexFullIdsFromAttractions,
	parseFixtureV1,
	type CanvasProps,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type LodKind,
	type SceneObjectStateContextValue,
	type RelocateMode,
} from "./index.tsx";

function useScenePlaySnapshot(): ScenePlaySnapshot {
	const { workbench } = useApp();
	const generation = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void generation;
	const ctrl = workbench.getActiveApp()?.controller as ScenePlayShellController | undefined;
	return (
		ctrl?.getSnapshot() ?? {
			fixture: null,
			lodProps: { automaticLod: true },
			lodTag: "normal",
			relocateMode: "translate",
			selectedId: null,
			proximityCount: 0,
			connectCount: 0,
			indirectCount: 0,
		}
	);
}

function ScenePlaySceneSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
	const { workbench } = useApp();
	const bus = workbench.commandBus;
	if (node.controllerId !== SCENE_PLAY_CONTROLLER_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid scene viewport binding</div>;
	}
	const snap = useScenePlaySnapshot();
	if (!snap.fixture) {
		return <div className="p-4 text-destructive">Invalid scene fixture</div>;
	}
	const kindCompatibility = parseKindCompatibility(snap.fixture.meta);
	const kindCatalogs = parseKindCatalogs(snap.fixture.meta);
	const blockedVortexFullIds = blockedVortexFullIdsFromAttractions(snap.fixture.attractions);
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<SceneObjectStateProvider fixture={snap.fixture} onConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteConnect")}>
				<PlaySceneCanvas
					fixture={snap.fixture}
					kindCatalogs={kindCatalogs}
					kindCompatibility={kindCompatibility}
					blockedVortexFullIds={blockedVortexFullIds}
					lodProps={snap.lodProps}
					relocateMode={snap.relocateMode}
					selectedId={snap.selectedId}
					setSelectedId={(id) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
					onSelect={(selection) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteSelection", selection)}
					onIndirectConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteIndirect")}
					onProximityConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteProximity")}
					onLodChange={(lod) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
				/>
			</SceneObjectStateProvider>
		</div>
	);
}

let scenePlayChromeRegistered = false;

function registerScenePlayChrome(): void {
	if (scenePlayChromeRegistered) return;
	scenePlayChromeRegistered = true;
	registerUiScene3DSurfaceHost(SCENE_PLAY_SCENE_SURFACE_ID, ScenePlaySceneSurfaceHost);
	registerDeclarativeWindowBody(SCENE_PLAY_BODY_KEY, buildScenePlayDeclarativeBody);
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
	readonly selectedId: string | null;
	readonly setSelectedId: (id: string | null) => void;
	readonly onSelect: (snap: { objectIds: readonly string[] }) => void;
	readonly onIndirectConnect: () => void;
	readonly onProximityConnect: () => void;
	readonly onLodChange: (lod: LodKind) => void;
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
				onLodChange={this.props.onLodChange}
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

interface PlayInnerState {
	readonly theme: ElementsSurfaceTheme;
	readonly device: ElementsSurfaceDevice;
	readonly expertise: Expertise;
}

class PlayInner extends React.Component<{}, PlayInnerState> {
	state: PlayInnerState = {
		theme: readTheme(),
		device: readDevice(),
		expertise: readExpertise(),
	};

	private cleanupSurfaceChrome: (() => void) | null = null;

	private sceneWorkbench: Workbench | null = null;

	componentDidMount(): void {
		registerScenePlayChrome();
		this.applySurfaceChrome();
		this.persistState();
		const fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
		if (fixture) {
			const urls = [...new Set(fixture.objects.map((object) => object.meshUrl))];
			for (const url of urls) {
				useGLTF.preload(url);
			}
		}
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
		if (!this.sceneWorkbench) {
			const wb = new Workbench();
			const ctrl = new ScenePlayShellController(wb.commandBus, () => wb.notify());
			wb.addApp(buildScenePlayWorkbenchApp(ctrl));
			this.sceneWorkbench = wb;
		}
		const workbench = this.sceneWorkbench;
		return (
			<WorkbenchView
				workbench={workbench}
				defaultAppId={PLAY_APP_ID}
				extraFooterItems={surfaceFooterItems}
				mobile={this.state.device === "mobile"}
			/>
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

/** @emoji ­ƒÜÇ Vite host entry: mounts scene play into `#root`. */
export function mountScenePlay(): void {
	mountReactApp(createScenePlayElement());
}
