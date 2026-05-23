// #region 🧲Header
// 💻 elements/client/lib/scene/play/index.tsx — Scene play harness: Nakagin fixture, LOD toolbar, relocate modes, and Playwright hooks (fixtures live only under play/).
// #endregion 🧲Header

import { useGLTF } from "@react-three/drei";
import {
	App,
	Button,
	Expertise,
	LevelProvider,
	mountReactApp,
	PureAppDefinition,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	createStackLayout,
	getLevelBgClass,
	useElementsSurfaceChrome,
	type AppConfig,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type FooterItem,
	type UIWindowKindDefinition,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";
import {
	Suspense,
	createContext,
	memo,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useState,
	type ReactElement,
} from "react";

import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import "./globals.css";
import {
	Canvas3D,
	LOD_MODE_AUTOMATIC,
	SceneAttractions,
	SceneObjectStateProvider,
	SceneObjects,
	ScenePlayTestBridge,
	blockedVortexFullIdsFromAttractions,
	isLodKind,
	lodAutomaticSelectLabel,
	lodCanvasProps,
	parseFixtureV1,
	useSceneObjectConnect,
	useSceneObjectRelocate,
	type CanvasProps,
	type FixtureV1,
	type KindCatalogBundle,
	type KindCompatEntry,
	type LodKind,
	type LodModeKind,
	type RelocateMode,
} from "../index.tsx";

//#region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly KindCompatEntry[] {
	if (!meta || typeof meta !== "object") return [];
	const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: KindCompatEntry[] = [];
	for (const entry of arr) {
		if (!entry || typeof entry !== "object") continue;
		const e = entry as Record<string, unknown>;
		const source = typeof e.source === "string" ? e.source.trim() : "";
		const target = typeof e.target === "string" ? e.target.trim() : "";
		if (!source || !target) continue;
		const specificity =
			e.specificity === "general" ||
			e.specificity === "node" ||
			e.specificity === "edge" ||
			e.specificity === "handle" ||
			e.specificity === "wire" ||
			e.specificity === "object" ||
			e.specificity === "attraction"
				? e.specificity
				: undefined;
		out.push({
			source,
			target,
			...(e.bidirectional === true ? { bidirectional: true } : {}),
			...(e.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
		});
	}
	return out;
}

class ScenePlayDefinition extends PureAppDefinition {
	constructor(private readonly lodMode: LodModeKind, private readonly setLodMode: (mode: LodModeKind) => void) {
		super();
	}

	resolveConfig(): AppConfig {
		return {
			id: PLAY_APP_ID,
			label: "Scene play",
			windowKinds: windowKindsWithLodMeasures(this.lodMode, this.setLodMode),
			defaultLayout: createStackLayout(["scene-main"], ["Scene"]),
		};
	}
}

function parseKindCatalogs(meta: Record<string, unknown> | undefined): KindCatalogBundle | undefined {
	const kc = meta?.kindCatalogs;
	if (!kc || typeof kc !== "object") return undefined;
	return kc as KindCatalogBundle;
}
//#endregion 🧾Meta

//#region 🖥️Surface
const LS_THEME = "elements.board-play.surface.theme";
const LS_DEVICE = "elements.board-play.surface.device";
const LS_EXPERTISE = "elements.board-play.surface.expertise";

function parseStoredTheme(raw: string | null): ElementsSurfaceTheme {
	if (raw === "light" || raw === "dark" || raw === "system") return raw;
	return "system";
}

function parseStoredDevice(raw: string | null): ElementsSurfaceDevice {
	if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
	return "desktop";
}

function parseStoredExpertise(raw: string | null): Expertise {
	if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
	return Expertise.NORMAL;
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

function PlaySurfaceFooter(props: {
	theme: ElementsSurfaceTheme;
	device: ElementsSurfaceDevice;
	expertise: Expertise;
	onTheme: (v: ElementsSurfaceTheme) => void;
	onDevice: (v: ElementsSurfaceDevice) => void;
	onExpertise: (v: Expertise) => void;
}): ReactElement {
	const { theme, device, expertise, onDevice, onExpertise, onTheme } = props;
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
//#endregion 🖥️Surface

//#region 🎬Play
const PLAY_LOD_TIERS: LodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function playLodTierMenuLabel(tier: LodKind): string {
	return tier.charAt(0).toUpperCase() + tier.slice(1);
}

const PlayLodContext = createContext<Pick<CanvasProps, "automaticLod" | "lod">>({ automaticLod: true });

const PlayLodDisplayContext = createContext<LodKind>("normal");

const PlayRuntimeContext = createContext<{
	readonly setEffectiveLod: (lod: LodKind) => void;
} | null>(null);

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
						{
							id: "automatic",
							label: lodAutomaticSelectLabel("normal"),
							value: LOD_MODE_AUTOMATIC,
						},
						...PLAY_LOD_TIERS.map((tier) => ({
							id: tier,
							label: playLodTierMenuLabel(tier),
							value: tier,
						})),
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

function PlayBody({
	fixture,
	lodProps,
}: {
	fixture: FixtureV1;
	lodProps: Pick<CanvasProps, "automaticLod" | "lod">;
}) {
	const [relocateMode, setRelocateMode] = useState<RelocateMode>("translate");
	const [selectedId, setSelectedId] = useState<string | null>(null);
	const [proximityCount, setProximityCount] = useState(0);
	const [connectCount, setConnectCount] = useState(0);
	const [indirectCount, setIndirectCount] = useState(0);
	const lodTag = useContext(PlayLodDisplayContext);
	const runtime = useContext(PlayRuntimeContext);
	const kindCompatibility = useMemo(() => parseKindCompatibility(fixture.meta), [fixture.meta]);
	const kindCatalogs = useMemo(() => parseKindCatalogs(fixture.meta), [fixture.meta]);
	const blockedVortexFullIds = useMemo(
		() => blockedVortexFullIdsFromAttractions(fixture.attractions),
		[fixture.attractions],
	);

	useEffect(() => {
		const urls = [...new Set(fixture.objects.map((o) => o.meshUrl))];
		for (const u of urls) {
			useGLTF.preload(u);
		}
	}, [fixture.objects]);

	const onSelect = useCallback((snap: { objectIds: readonly string[] }) => {
		setSelectedId(snap.objectIds[0] ?? null);
	}, []);

	const onProximityConnect = useCallback(() => {
		setProximityCount((c) => c + 1);
	}, []);

	const onConnect = useCallback(() => {
		setConnectCount((c) => c + 1);
	}, []);

	const onIndirectConnect = useCallback(() => {
		setIndirectCount((c) => c + 1);
	}, []);

	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<ToolbarZone>
					<ToolbarGroup>
						<ToolbarItem>
							<Button
								variant={relocateMode === "translate" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("translate")}
							>
								<Move3d className="mr-1 size-4" />
								Translate
							</Button>
						</ToolbarItem>
						<ToolbarItem>
							<Button
								variant={relocateMode === "rotate" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("rotate")}
							>
								<Rotate3d className="mr-1 size-4" />
								Rotate
							</Button>
						</ToolbarItem>
						<ToolbarItem>
							<Button
								variant={relocateMode === "scale" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("scale")}
							>
								<Scaling className="mr-1 size-4" />
								Scale
							</Button>
						</ToolbarItem>
					</ToolbarGroup>
				</ToolbarZone>
				<div className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
					<span data-e2e-selected>{selectedId ?? "—"}</span>
					<span data-e2e-scene-lod>{lodTag}</span>
					<span data-e2e-proximity-count>{proximityCount}</span>
					<span data-e2e-connect-count>{connectCount}</span>
					<span data-e2e-indirect-count>{indirectCount}</span>
				</div>
			</div>
			<div className="relative min-h-0 flex-1">
				<Suspense fallback={<div className="p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
					<SceneObjectStateProvider fixture={fixture} onConnect={onConnect}>
						<PlaySceneCanvas
							fixture={fixture}
							kindCatalogs={kindCatalogs}
							kindCompatibility={kindCompatibility}
							blockedVortexFullIds={blockedVortexFullIds}
							lodProps={lodProps}
							relocateMode={relocateMode}
							runtime={runtime}
							selectedId={selectedId}
							setSelectedId={setSelectedId}
							onSelect={onSelect}
							onIndirectConnect={onIndirectConnect}
							onProximityConnect={onProximityConnect}
						/>
					</SceneObjectStateProvider>
				</Suspense>
			</div>
		</div>
	);
}

const PlaySceneCanvasContent = memo(function PlaySceneCanvasContent(props: {
	readonly selectedId: string | null;
	readonly relocateMode: RelocateMode;
	readonly setSelectedId: (id: string | null) => void;
}) {
	return (
		<>
			<ScenePlayTestBridge setSelectedId={props.setSelectedId} />
			<SceneObjects selectedObjectId={props.selectedId} relocate={props.relocateMode} />
			<SceneAttractions />
		</>
	);
});

function PlaySceneCanvas(props: {
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
}) {
	const onRelocate = useSceneObjectRelocate();
	const onConnect = useSceneObjectConnect();
	const sceneChildren = useMemo(
		() => (
			<PlaySceneCanvasContent
				relocateMode={props.relocateMode}
				selectedId={props.selectedId}
				setSelectedId={props.setSelectedId}
			/>
		),
		[props.relocateMode, props.selectedId, props.setSelectedId],
	);
	return (
		<Canvas3D
			className="absolute inset-0"
			camera={props.fixture.camera}
			domain={props.fixture.domain}
			kindCatalogs={props.kindCatalogs}
			kindCompatibility={props.kindCompatibility}
			blockedVortexFullIds={props.blockedVortexFullIds}
			proximityRadius={24}
			relocateMode={props.relocateMode}
			showLodGrid
			gridSnapEnabled
			{...props.lodProps}
			onLodChange={props.runtime?.setEffectiveLod}
			onSelect={props.onSelect}
			onConnect={onConnect}
			onIndirectConnect={props.onIndirectConnect}
			onProximityConnect={props.onProximityConnect}
			onRelocate={onRelocate}
		>
			{sceneChildren}
		</Canvas3D>
	);
}

function MainWindow() {
	const fixture = useMemo(() => parseFixtureV1(nakaginSceneFixtureJson as unknown), []);
	const lodProps = useContext(PlayLodContext);
	if (!fixture) {
		return <div className="p-4 text-destructive">Invalid scene fixture</div>;
	}
	return <PlayBody fixture={fixture} lodProps={lodProps} />;
}

const PLAY_APP_ID = "elements-scene-play";

function PlayInner(): ReactElement {
	const [theme, setTheme] = useState<ElementsSurfaceTheme>(readTheme);
	const [device, setDevice] = useState<ElementsSurfaceDevice>(readDevice);
	const [expertise, setExpertise] = useState<Expertise>(readExpertise);
	const { mobile } = useElementsSurfaceChrome({ theme, device, expertise });

	useEffect(() => {
		try {
			localStorage.setItem(LS_THEME, theme);
		} catch {
			/* ignore */
		}
	}, [theme]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_DEVICE, device);
		} catch {
			/* ignore */
		}
	}, [device]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_EXPERTISE, expertise);
		} catch {
			/* ignore */
		}
	}, [expertise]);

	const surfaceFooterItems = useMemo<FooterItem[]>(
		() => [
			{
				content: (
					<PlaySurfaceFooter
						device={device}
						expertise={expertise}
						onDevice={setDevice}
						onExpertise={setExpertise}
						onTheme={setTheme}
						theme={theme}
					/>
				),
				id: "scene-play-surface",
				order: 0,
			},
		],
		[device, expertise, theme],
	);

	const [lodMode, setLodMode] = useState<LodModeKind>(LOD_MODE_AUTOMATIC);
	const [lodTag, setLodTag] = useState<LodKind>("normal");
	const lodProps = useMemo(() => lodCanvasProps(lodMode), [lodMode]);
	const runtime = useMemo(
		() => ({
			setEffectiveLod: (lod: LodKind) => {
				setLodTag((prev) => (prev === lod ? prev : lod));
			},
		}),
		[],
	);

	const apps = useMemo(() => [new ScenePlayDefinition(lodMode, setLodMode)], [lodMode]);

	return (
		<PlayLodDisplayContext.Provider value={lodTag}>
			<PlayLodContext.Provider value={lodProps}>
				<PlayRuntimeContext.Provider value={runtime}>
					<App apps={apps} defaultAppId={PLAY_APP_ID} footerItems={surfaceFooterItems} mobile={mobile} />
				</PlayRuntimeContext.Provider>
			</PlayLodContext.Provider>
		</PlayLodDisplayContext.Provider>
	);
}

function PlayApp(): ReactElement {
	return (
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<PlayInner />
			</div>
		</LevelProvider>
	);
}
//#endregion 🎬Play

mountReactApp(<PlayApp />);

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("scene play fixture", () => {
		it("parses nakagin fixture", () => {
			const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
			expect(f?.domain).toBe("architecture");
			expect(f?.attractions.length).toBeGreaterThan(0);
			expect(f?.objects.length).toBeGreaterThan(0);
		});
	});
}
//#endregion 🧪Tests
