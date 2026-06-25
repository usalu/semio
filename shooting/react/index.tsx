// #region 🧲Header
/** @emoji 📸 Shooting React canvases: interactive model viewport and icon shot preview/export. */
// #endregion 🧲Header

import type { CSSProperties, ReactNode, RefObject } from "react";
import type { OrbitCameraProjection, WorldCameraState } from "@semio-tech/infinite-world-r3f";
import {
	GLB_MESH_FRAME_ROTATION_X,
	WorldCanvas,
	WorldLayer,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitProjectionSwitch,
	WorldOrbitViewControls,
	WorldOrbitViewSnapGateProvider,
} from "@semio-tech/infinite-world-r3f";
import type { IconRenderRequest } from "@semio-tech/ui-styling/icon-render-port";
import { cn, iconRenderPort, reactHostPort, sceneHostPort } from "@semio-tech/ui-react";
import type { Object3D } from "three";

const Clone = sceneHostPort.drei.Clone;
const useGLTF = sceneHostPort.drei.useGLTF;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const { Box3, Color, MeshStandardMaterial, Vector3 } = sceneHostPort.three;

//#region 🔖Fixture
export type ShootingAssetFormat = "glb";

export interface ShootingAssetV1 {
	readonly id: string;
	readonly name: string;
	readonly url: string;
	readonly format: ShootingAssetFormat;
}

export interface ShootingCameraV1 {
	readonly position: readonly [number, number, number];
	readonly target: readonly [number, number, number];
	readonly zoom: number;
	readonly up?: readonly [number, number, number];
	readonly projection?: OrbitCameraProjection;
	readonly fov?: number;
}

export interface ShootingSavedCameraV1 {
	readonly id: string;
	readonly label: string;
	readonly camera: ShootingCameraV1;
}

export interface ShootingSunV1 {
	readonly azimuth: number;
	readonly elevation: number;
	readonly intensity: number;
	readonly color: string;
}

export interface ShootingAmbientV1 {
	readonly intensity: number;
	readonly color: string;
}

export interface ShootingShadowV1 {
	readonly enabled: boolean;
	readonly opacity: number;
	readonly softness: number;
}

export interface ShootingMaterialV1 {
	readonly color: string;
	readonly metalness: number;
	readonly roughness: number;
	readonly emissive: string;
	readonly emissiveIntensity: number;
}

export interface ShootingSceneV1 {
	readonly background: string;
	readonly sun: ShootingSunV1;
	readonly ambient: ShootingAmbientV1;
	readonly shadow: ShootingShadowV1;
	readonly material: ShootingMaterialV1;
}

export type ShootingShotFormat = "svg" | "png";

export interface ShootingShotV1 {
	readonly id: string;
	readonly label: string;
	readonly width: number;
	readonly height: number;
	readonly format: ShootingShotFormat;
	readonly background?: string;
	readonly cameraId?: string;
}

export interface ShootingFixtureV1 {
	readonly schema: "shooting.fixture/v1";
	readonly assets: readonly ShootingAssetV1[];
	readonly camera: ShootingCameraV1;
	readonly savedCameras: readonly ShootingSavedCameraV1[];
	readonly scene: ShootingSceneV1;
	readonly shots: readonly ShootingShotV1[];
	readonly activeShotId?: string;
	readonly activeAssetId?: string;
}

export const DEFAULT_SHOOTING_SCENE: ShootingSceneV1 = {
	background: "#e8eaed",
	sun: { azimuth: 45, elevation: 35, intensity: 2.4, color: "#ffffff" },
	ambient: { intensity: 1.15, color: "#ffffff" },
	shadow: { enabled: true, opacity: 0.35, softness: 1 },
	material: { color: "#9aa0ab", metalness: 0, roughness: 1, emissive: "#000000", emissiveIntensity: 0 },
};

export const DEFAULT_SHOOTING_CAMERA: ShootingCameraV1 = {
	position: [420, -420, 320],
	target: [0, 0, 40],
	zoom: 1,
	fov: 50,
};

export const DEFAULT_SHOOTING_FIXTURE: ShootingFixtureV1 = {
	schema: "shooting.fixture/v1",
	assets: [{ id: "base", name: "Base", url: "/mesh/base.glb", format: "glb" }],
	camera: DEFAULT_SHOOTING_CAMERA,
	savedCameras: [],
	scene: DEFAULT_SHOOTING_SCENE,
	shots: [
		{ id: "overview-svg", label: "Overview Svg", width: 256, height: 256, format: "svg" },
		{ id: "overview-png", label: "Overview Png", width: 512, height: 512, format: "png" },
	],
	activeShotId: "overview-svg",
	activeAssetId: "base",
};

export function shootingFixtureToJson(fixture: ShootingFixtureV1): string {
	return JSON.stringify(fixture, null, 2);
}

export function parseShootingFixture(json: string): ShootingFixtureV1 | null {
	try {
		const parsed = JSON.parse(json) as ShootingFixtureV1;
		if (parsed.schema !== "shooting.fixture/v1") return null;
		return parsed;
	} catch {
		return null;
	}
}

export function shootingCameraToWorldState(camera: ShootingCameraV1): WorldCameraState {
	return {
		position: [camera.position[0], camera.position[1], camera.position[2]],
		target: [camera.target[0], camera.target[1], camera.target[2]],
		zoom: camera.zoom,
		...(camera.up ? { up: [camera.up[0], camera.up[1], camera.up[2]] } : {}),
		...(camera.projection ? { projection: camera.projection } : {}),
	};
}

export function worldStateToShootingCamera(state: WorldCameraState, fov?: number): ShootingCameraV1 {
	return {
		position: [state.position[0], state.position[1], state.position[2]],
		target: [state.target[0], state.target[1], state.target[2]],
		zoom: state.zoom,
		...(state.up ? { up: [state.up[0], state.up[1], state.up[2]] } : {}),
		...(state.projection ? { projection: state.projection } : {}),
		...(fov !== undefined ? { fov } : {}),
	};
}

export function resolveActiveShot(fixture: ShootingFixtureV1): ShootingShotV1 | null {
	if (!fixture.shots.length) return null;
	const active = fixture.activeShotId ? fixture.shots.find((shot) => shot.id === fixture.activeShotId) : undefined;
	return active ?? fixture.shots[0] ?? null;
}

export function resolveActiveAsset(fixture: ShootingFixtureV1): ShootingAssetV1 | null {
	if (!fixture.assets.length) return null;
	const active = fixture.activeAssetId ? fixture.assets.find((asset) => asset.id === fixture.activeAssetId) : undefined;
	return active ?? fixture.assets[0] ?? null;
}

export function resolveShotCamera(fixture: ShootingFixtureV1, shot: ShootingShotV1): ShootingCameraV1 {
	if (!shot.cameraId) return fixture.camera;
	const saved = fixture.savedCameras.find((entry) => entry.id === shot.cameraId);
	return saved?.camera ?? fixture.camera;
}

export function applyShootingCameraToFixture(fixture: ShootingFixtureV1, shot: ShootingShotV1, camera: ShootingCameraV1): ShootingFixtureV1 {
	if (!shot.cameraId) return { ...fixture, camera };
	return {
		...fixture,
		savedCameras: fixture.savedCameras.map((entry) => (entry.id === shot.cameraId ? { ...entry, camera } : entry)),
	};
}

export function shootingIconRenderRequest(fixture: ShootingFixtureV1, shot: ShootingShotV1, asset: ShootingAssetV1): IconRenderRequest {
	const camera = resolveShotCamera(fixture, shot);
	return {
		assetUrl: asset.url,
		camera: {
			position: camera.position,
			target: camera.target,
			zoom: camera.zoom,
			fov: camera.fov ?? 50,
			...(camera.up ? { up: camera.up } : {}),
		},
		lights: {
			ambientIntensity: fixture.scene.ambient.intensity,
			ambientColor: fixture.scene.ambient.color,
			sunAzimuth: fixture.scene.sun.azimuth,
			sunElevation: fixture.scene.sun.elevation,
			sunIntensity: fixture.scene.sun.intensity,
			sunColor: fixture.scene.sun.color,
		},
		width: shot.width,
		height: shot.height,
		format: shot.format,
		background: shot.background ?? fixture.scene.background,
		shadowEnabled: fixture.scene.shadow.enabled,
		material: {
			color: fixture.scene.material.color,
			metalness: fixture.scene.material.metalness,
			roughness: fixture.scene.material.roughness,
			emissive: fixture.scene.material.emissive,
			emissiveIntensity: fixture.scene.material.emissiveIntensity,
		},
	};
}
//#endregion 🔖Fixture

//#region 🔖SceneHelpers
export function sunPositionFromAzimuthElevation(azimuthDeg: number, elevationDeg: number, distance = 120): [number, number, number] {
	const az = (azimuthDeg * Math.PI) / 180;
	const el = (elevationDeg * Math.PI) / 180;
	return [Math.cos(el) * Math.cos(az) * distance, Math.cos(el) * Math.sin(az) * distance, Math.sin(el) * distance];
}

function createStyledMaterial(material: ShootingMaterialV1): MeshStandardMaterial {
	const mat = new MeshStandardMaterial({
		color: new Color(material.color),
		metalness: material.metalness,
		roughness: material.roughness,
	});
	mat.emissive.set(material.emissive);
	mat.emissiveIntensity = material.emissiveIntensity;
	return mat;
}

export interface ShootingModelBounds {
	readonly center: readonly [number, number, number];
	readonly radius: number;
}

export function shootingFitCameraFromBounds(
	bounds: ShootingModelBounds,
	camera: Pick<ShootingCameraV1, "position" | "target" | "zoom" | "projection">,
	padding = 1.25,
): Pick<ShootingCameraV1, "position" | "target" | "zoom"> {
	const [cx, cy, cz] = bounds.center;
	const distance = Math.max(bounds.radius * padding, 2);
	const dx = camera.position[0] - camera.target[0];
	const dy = camera.position[1] - camera.target[1];
	const dz = camera.position[2] - camera.target[2];
	const length = Math.hypot(dx, dy, dz);
	const nx = length > 1e-6 ? dx / length : 1;
	const ny = length > 1e-6 ? dy / length : -1;
	const nz = length > 1e-6 ? dz / length : 0.85;
	const norm = Math.hypot(nx, ny, nz) || 1;
	return {
		position: [cx + (nx / norm) * distance, cy + (ny / norm) * distance, cz + (nz / norm) * distance],
		target: [cx, cy, cz],
		zoom: camera.zoom,
	};
}

function ShootingAutoFit({
	meshRef,
	assetKey,
	enabled,
	fitRevision,
	camera,
	projection,
	onCamera,
}: {
	readonly meshRef: RefObject<Object3D | null>;
	readonly assetKey: string;
	readonly enabled: boolean;
	readonly fitRevision: number;
	readonly camera: ShootingCameraV1;
	readonly projection: OrbitCameraProjection;
	readonly onCamera: (camera: ShootingCameraV1) => void;
}): null {
	const { camera: sceneCamera, controls, invalidate } = useThree();
	const appliedKeyRef = reactHostPort.useRef("");
	const targetScratch = reactHostPort.useMemo(() => new Vector3(), []);
	useFrame(() => {
		if (!enabled || !sceneCamera) return;
		const mesh = meshRef.current;
		if (!mesh) return;
		const fitKey = `${fitRevision}:${assetKey}:${mesh.uuid}`;
		if (appliedKeyRef.current === fitKey) return;
		const box = new Box3().setFromObject(mesh);
		if (box.isEmpty()) return;
		const center = box.getCenter(new Vector3());
		const size = box.getSize(new Vector3());
		const radius = Math.max(size.x, size.y, size.z) * 0.5;
		if (radius <= 0) return;
		appliedKeyRef.current = fitKey;
		const fitted = shootingFitCameraFromBounds({ center: [center.x, center.y, center.z], radius }, camera);
		const orbit = controls as { target: Vector3; update?: () => void } | null;
		const target = orbit?.target ?? targetScratch;
		target.set(fitted.target[0], fitted.target[1], fitted.target[2]);
		sceneCamera.position.set(fitted.position[0], fitted.position[1], fitted.position[2]);
		if ("zoom" in sceneCamera) {
			sceneCamera.zoom = fitted.zoom;
		}
		sceneCamera.updateProjectionMatrix();
		if (orbit) {
			orbit.update?.();
		} else {
			sceneCamera.lookAt(target);
		}
		invalidate();
		onCamera({
			...camera,
			position: fitted.position,
			target: fitted.target,
			zoom: fitted.zoom,
			projection,
		});
	});
	return null;
}
//#endregion 🔖SceneHelpers

//#region 🔖ModelCanvas
function ShootingGlbMesh({
	url,
	material,
	shadowEnabled,
	meshRef,
}: {
	readonly url: string;
	readonly material: ShootingMaterialV1;
	readonly shadowEnabled: boolean;
	readonly meshRef?: RefObject<Object3D | null>;
}): ReactNode {
	const gltf = useGLTF(url);
	reactHostPort.useLayoutEffect(() => {
		if (!gltf.scene) return;
		const styled = createStyledMaterial(material);
		gltf.scene.traverse((obj) => {
			if (!("isMesh" in obj) || !(obj as { isMesh?: boolean }).isMesh) return;
			const mesh = obj as { material?: unknown; castShadow?: boolean; receiveShadow?: boolean };
			mesh.material = styled;
			mesh.castShadow = shadowEnabled;
			mesh.receiveShadow = shadowEnabled;
		});
	}, [gltf.scene, material, shadowEnabled]);
	if (!gltf.scene) return null;
	return (
		<group ref={meshRef} rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>
			<Clone object={gltf.scene} />
		</group>
	);
}

export interface ShootingViewportOptions {
	readonly centerModel?: boolean;
	readonly fitRevision?: number;
}

export interface ShootingModelCanvasProps extends ShootingViewportOptions {
	readonly fixture: ShootingFixtureV1;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly onCamera?: (camera: ShootingCameraV1) => void;
}

export function ShootingModelCanvas({
	fixture,
	className,
	style,
	onCamera,
	centerModel = true,
	fitRevision = 0,
}: ShootingModelCanvasProps): ReactNode {
	const asset = resolveActiveAsset(fixture);
	const camera = shootingCameraToWorldState(fixture.camera);
	const projection = fixture.camera.projection ?? "perspective";
	const [cameraSeed, setCameraSeed] = reactHostPort.useState(0);
	const meshRef = reactHostPort.useRef<Object3D | null>(null);
	const sunPos = sunPositionFromAzimuthElevation(fixture.scene.sun.azimuth, fixture.scene.sun.elevation);
	const handleCamera = reactHostPort.useCallback(
		(next: ShootingCameraV1) => {
			onCamera?.(next);
		},
		[onCamera],
	);
	const onProjectionChange = reactHostPort.useCallback((next: OrbitCameraProjection) => {
		handleCamera({ ...fixture.camera, projection: next });
	}, [fixture.camera, handleCamera]);
	if (!asset) {
		return <div className={cn("flex h-full items-center justify-center text-sm opacity-60", className)}>No asset</div>;
	}
	return (
		<div className={cn("absolute inset-0", className)} style={style}>
			<WorldCanvas
				className="h-full w-full"
				background={fixture.scene.background}
				shadows={fixture.scene.shadow.enabled}
				overlay={<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />}
			>
				<WorldOrbitViewSnapGateProvider>
					<WorldOrbitCameraViewRig state={camera} seedKey={`${cameraSeed}`} perspectiveFov={fixture.camera.fov ?? 50} />
					<WorldOrbitGated
						controlsKey={`${cameraSeed}`}
						projection={projection}
						zoom={camera.zoom}
						onCamera={(next) => handleCamera(worldStateToShootingCamera(next, fixture.camera.fov))}
					/>
					<WorldOrbitViewControls
						onCameraChange={(next) => {
							handleCamera(worldStateToShootingCamera(next, fixture.camera.fov));
							setCameraSeed((seed) => seed + 1);
						}}
					/>
					<ShootingAutoFit
						assetKey={asset.url}
						camera={fixture.camera}
						enabled={centerModel}
						fitRevision={fitRevision}
						meshRef={meshRef}
						projection={projection}
						onCamera={handleCamera}
					/>
					<ambientLight color={fixture.scene.ambient.color} intensity={fixture.scene.ambient.intensity} />
					<directionalLight
						color={fixture.scene.sun.color}
						intensity={fixture.scene.sun.intensity}
						position={sunPos}
						castShadow={fixture.scene.shadow.enabled}
					/>
					<WorldLayer order={10} name="shooting.model">
						<ShootingGlbMesh
							url={asset.url}
							material={fixture.scene.material}
							shadowEnabled={fixture.scene.shadow.enabled}
							meshRef={meshRef}
						/>
					</WorldLayer>
				</WorldOrbitViewSnapGateProvider>
			</WorldCanvas>
		</div>
	);
}
//#endregion 🔖ModelCanvas

//#region 🔖IconCanvas
export interface ShootingIconCanvasProps extends ShootingViewportOptions {
	readonly fixture: ShootingFixtureV1;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly onCamera?: (camera: ShootingCameraV1) => void;
}

export function ShootingIconCanvas({
	fixture,
	className,
	style,
	onCamera,
	centerModel = true,
	fitRevision = 0,
}: ShootingIconCanvasProps): ReactNode {
	const shot = resolveActiveShot(fixture);
	const asset = resolveActiveAsset(fixture);
	const shotCamera = shot ? resolveShotCamera(fixture, shot) : null;
	const camera = shotCamera ? shootingCameraToWorldState(shotCamera) : null;
	const projection = shotCamera?.projection ?? "perspective";
	const [cameraSeed, setCameraSeed] = reactHostPort.useState(0);
	const meshRef = reactHostPort.useRef<Object3D | null>(null);
	const sunPos = sunPositionFromAzimuthElevation(fixture.scene.sun.azimuth, fixture.scene.sun.elevation);
	const handleCamera = reactHostPort.useCallback(
		(next: ShootingCameraV1) => {
			onCamera?.(next);
		},
		[onCamera],
	);
	const onProjectionChange = reactHostPort.useCallback(
		(next: OrbitCameraProjection) => {
			if (!shotCamera) return;
			handleCamera({ ...shotCamera, projection: next });
		},
		[handleCamera, shotCamera],
	);
	if (!shot || !asset || !shotCamera || !camera) {
		return <div className={cn("flex h-full items-center justify-center text-sm opacity-60", className)}>No shot</div>;
	}
	const shotBackground = shot.background ?? fixture.scene.background;
	return (
		<div className={cn("absolute inset-0 flex flex-col", className)} style={style}>
			<div className="relative min-h-0 flex-1 p-3">
				<div className="absolute inset-3 flex items-center justify-center">
					<div
						className="relative h-full w-full overflow-hidden shadow-sm ring-1 ring-border/40"
						style={{ aspectRatio: `${shot.width} / ${shot.height}`, maxHeight: "100%", maxWidth: "100%" }}
					>
						<WorldCanvas
							className="h-full w-full"
							background={shotBackground}
							shadows={fixture.scene.shadow.enabled}
							overlay={<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />}
						>
							<WorldOrbitViewSnapGateProvider>
								<WorldOrbitCameraViewRig state={camera} seedKey={`icon-${cameraSeed}`} perspectiveFov={shotCamera.fov ?? 50} />
								<WorldOrbitGated
									controlsKey={`icon-${cameraSeed}`}
									projection={projection}
									zoom={camera.zoom}
									onCamera={(next) => handleCamera(worldStateToShootingCamera(next, shotCamera.fov))}
								/>
								<WorldOrbitViewControls
									onCameraChange={(next) => {
										handleCamera(worldStateToShootingCamera(next, shotCamera.fov));
										setCameraSeed((seed) => seed + 1);
									}}
								/>
								<ShootingAutoFit
									assetKey={asset.url}
									camera={shotCamera}
									enabled={centerModel}
									fitRevision={fitRevision}
									meshRef={meshRef}
									projection={projection}
									onCamera={handleCamera}
								/>
								<ambientLight color={fixture.scene.ambient.color} intensity={fixture.scene.ambient.intensity} />
								<directionalLight
									color={fixture.scene.sun.color}
									intensity={fixture.scene.sun.intensity}
									position={sunPos}
									castShadow={fixture.scene.shadow.enabled}
								/>
								<WorldLayer order={10} name="shooting.icon">
									<ShootingGlbMesh
										url={asset.url}
										material={fixture.scene.material}
										shadowEnabled={fixture.scene.shadow.enabled}
										meshRef={meshRef}
									/>
								</WorldLayer>
							</WorldOrbitViewSnapGateProvider>
						</WorldCanvas>
					</div>
				</div>
			</div>
			<div className="shrink-0 px-3 pb-2 text-center text-xs opacity-60">
				{shot.label} · {shot.width}×{shot.height} · {shot.format.toUpperCase()}
			</div>
		</div>
	);
}

export async function renderShootingShot(fixture: ShootingFixtureV1, shot: ShootingShotV1, asset?: ShootingAssetV1) {
	const resolvedAsset = asset ?? resolveActiveAsset(fixture);
	if (!resolvedAsset) throw new Error("No asset for shot render");
	return iconRenderPort.render(shootingIconRenderRequest(fixture, shot, resolvedAsset));
}
//#endregion 🔖IconCanvas

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/shooting-react", () => {
		it("exports default fixture json", () => {
			expect(shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE)).toContain("shooting.fixture/v1");
		});

		it("parses shooting fixture", () => {
			const json = shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE);
			expect(parseShootingFixture(json)?.activeShotId).toBe("overview-svg");
		});

		it("builds icon render request from fixture", () => {
			const shot = DEFAULT_SHOOTING_FIXTURE.shots[0]!;
			const asset = DEFAULT_SHOOTING_FIXTURE.assets[0]!;
			const request = shootingIconRenderRequest(DEFAULT_SHOOTING_FIXTURE, shot, asset);
			expect(request.format).toBe("svg");
			expect(request.assetUrl).toBe("/mesh/base.glb");
		});

		it("applies shot camera to fixture camera or saved camera", () => {
			const shot = DEFAULT_SHOOTING_FIXTURE.shots[0]!;
			const nextCamera = { ...DEFAULT_SHOOTING_FIXTURE.camera, zoom: 2 };
			expect(applyShootingCameraToFixture(DEFAULT_SHOOTING_FIXTURE, shot, nextCamera).camera.zoom).toBe(2);
			const withSaved = {
				...DEFAULT_SHOOTING_FIXTURE,
				savedCameras: [{ id: "cam-a", label: "A", camera: DEFAULT_SHOOTING_FIXTURE.camera }],
				shots: [{ ...shot, cameraId: "cam-a" }],
			};
			const patched = applyShootingCameraToFixture(withSaved, withSaved.shots[0]!, nextCamera);
			expect(patched.camera.zoom).toBe(1);
			expect(patched.savedCameras[0]?.camera.zoom).toBe(2);
		});

		it("fits camera to model bounds while preserving view direction", () => {
			const fitted = shootingFitCameraFromBounds(
				{ center: [0, 0, 10], radius: 20 },
				DEFAULT_SHOOTING_FIXTURE.camera,
			);
			expect(fitted.target).toEqual([0, 0, 10]);
			expect(fitted.position[0]).toBeGreaterThan(fitted.target[0]);
			expect(fitted.position[1]).toBeLessThan(fitted.target[1]);
		});
	});
}
// #endregion 🧪Tests
