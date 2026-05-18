// #region 🧲Header
// 💻 elements/client/lib/scene/play/index.tsx — Scene play harness: Nakagin fixture, relocate toolbar, selection, `@elements/ui` shell.
// #endregion 🧲Header

// #region 📥Imports
import { useGLTF } from "@react-three/drei";
import { Suspense, useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";

import {
	Button,
	LevelProvider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	UI,
	createStackLayout,
	getLevelBgClass,
	type UIAppConfig,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";

import sceneFixtureJson from "../fixtures/nakagin-capsule-tower.scene.json";
import {
	Scene,
	SceneObject,
	SceneTie,
	SceneVortex,
	parseSceneFixtureV1,
	sceneBlockedVortexFullIdsFromTies,
	type SceneFixtureV1,
	type SceneKindCatalogBundle,
	type SceneKindCompatEntry,
	type SceneLodKind,
	type SceneRelocateMode,
} from "../react/index";
import "./globals.css";
// #endregion 📥Imports

// #region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly SceneKindCompatEntry[] {
	if (!meta || typeof meta !== "object") return [];
	const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: SceneKindCompatEntry[] = [];
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
			e.specificity === "tie"
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

function parseKindCatalogs(meta: Record<string, unknown> | undefined): SceneKindCatalogBundle | undefined {
	const kc = meta?.kindCatalogs;
	if (!kc || typeof kc !== "object") return undefined;
	return kc as SceneKindCatalogBundle;
}
// #endregion 🧾Meta

// #region 🎬ScenePlay
function ScenePlayBody({ fixture }: { fixture: SceneFixtureV1 }) {
	const [relocateMode, setRelocateMode] = useState<SceneRelocateMode>("translate");
	const [selectedId, setSelectedId] = useState<string | null>(null);
	const [proximityCount, setProximityCount] = useState(0);
	const [connectCount, setConnectCount] = useState(0);
	const [indirectCount, setIndirectCount] = useState(0);
	const [sceneLodTag, setSceneLodTag] = useState<SceneLodKind>("normal");
	const kindCompatibility = useMemo(() => parseKindCompatibility(fixture.meta), [fixture.meta]);
	const kindCatalogs = useMemo(() => parseKindCatalogs(fixture.meta), [fixture.meta]);
	const blockedVortexFullIds = useMemo(
		() => sceneBlockedVortexFullIdsFromTies(fixture.ties),
		[fixture.ties],
	);

	useEffect(() => {
		const urls = [...new Set(fixture.objects.map((o) => o.meshUrl))];
		for (const u of urls) {
			useGLTF.preload(u);
		}
	}, [fixture.objects]);

	useEffect(() => {
		const w = window as unknown as { __scenePlaySelect?: (id: string) => void };
		w.__scenePlaySelect = (id: string) => {
			setSelectedId(id);
		};
		return () => {
			delete w.__scenePlaySelect;
		};
	}, []);

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
					<span data-e2e-scene-lod>{sceneLodTag}</span>
					<span data-e2e-proximity-count>{proximityCount}</span>
					<span data-e2e-connect-count>{connectCount}</span>
					<span data-e2e-indirect-count>{indirectCount}</span>
				</div>
			</div>
			<div className="relative min-h-0 flex-1">
				<Suspense fallback={<div className="p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
					<Scene
						className="absolute inset-0"
						camera={fixture.camera}
						kindCatalogs={kindCatalogs}
						kindCompatibility={kindCompatibility}
						blockedVortexFullIds={blockedVortexFullIds}
						proximityRadius={24}
						relocateMode={relocateMode}
						showLodGrid
						gridSnapEnabled
						onLodChange={setSceneLodTag}
						onSelect={onSelect}
						onConnect={onConnect}
						onIndirectConnect={onIndirectConnect}
						onProximityConnect={onProximityConnect}
					>
						{fixture.objects.map((o) => (
							<SceneObject
								key={o.id}
								id={o.id}
								meshUrl={o.meshUrl}
								origin={o.origin}
								orientation={o.orientation}
								scale={o.scale}
								objectKind={o.objectKind}
								label={o.label}
								selected={selectedId === o.id}
								relocate={relocateMode}
							>
								{o.vortices.map((v) => (
									<SceneVortex key={v.id} objectId={o.id} objectKind={o.objectKind} {...v} />
								))}
							</SceneObject>
						))}
						{fixture.ties.map((t) => (
							<SceneTie key={t.id} {...t} />
						))}
					</Scene>
				</Suspense>
			</div>
		</div>
	);
}

function MainWindow() {
	const fixture = useMemo(() => parseSceneFixtureV1(sceneFixtureJson as unknown), []);
	if (!fixture) {
		return <div className="p-4 text-destructive">Invalid scene fixture</div>;
	}
	return <ScenePlayBody fixture={fixture} />;
}

function ScenePlayApp() {
	const apps = useMemo<UIAppConfig[]>(
		() => [
			{
				id: "elements-scene-play",
				label: "Scene play",
				windowKinds: [{ id: "scene-main", label: "Scene", component: MainWindow }],
				defaultLayout: createStackLayout(["scene-main"], ["Scene"]),
			},
		],
		[],
	);
	return (
		<LevelProvider>
			<UI apps={apps} defaultAppId="elements-scene-play" className={getLevelBgClass(0)} />
		</LevelProvider>
	);
}
// #endregion 🎬ScenePlay

// #region 🚀Mount
const rootEl = document.getElementById("root");
if (rootEl) {
	createRoot(rootEl).render(<ScenePlayApp />);
}
// #endregion 🚀Mount

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("scene play fixture hook", () => {
		it("parses nakagin fixture", () => {
			const f = parseSceneFixtureV1(sceneFixtureJson as unknown);
			expect(f?.ties.length).toBeGreaterThan(0);
			expect(f?.objects.length).toBeGreaterThan(0);
		});
	});
}
