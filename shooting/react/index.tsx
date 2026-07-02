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
import type { IconRenderRequest, IconRenderShape } from "@semio-tech/ui-styling";
import { cn, iconRenderPort, reactHostPort, sceneHostPort } from "@semio-tech/ui-react";
import type { Object3D } from "three";

const Clone = sceneHostPort.drei.Clone;
const useGLTF = sceneHostPort.drei.useGLTF;
const useFrame = sceneHostPort.fiber.useFrame;
const useThree = sceneHostPort.fiber.useThree;
const { Box3, Color, MeshStandardMaterial, Vector3 } = sceneHostPort.three;

//#region 🔖Fixture
export type ShootingAssetFormat = "glb";

export interface ShootingAsset {
	readonly id: string;
	readonly name: string;
	readonly url: string;
	readonly format: ShootingAssetFormat;
}

export interface ShootingCamera {
	readonly position: readonly [number, number, number];
	readonly target: readonly [number, number, number];
	readonly zoom: number;
	readonly up?: readonly [number, number, number];
	readonly projection?: OrbitCameraProjection;
	readonly fov?: number;
}

export interface ShootingSavedCamera {
	readonly id: string;
	readonly label: string;
	readonly camera: ShootingCamera;
}

export interface ShootingSun {
	readonly azimuth: number;
	readonly elevation: number;
	readonly intensity: number;
	readonly color: string;
}

export interface ShootingAmbient {
	readonly intensity: number;
	readonly color: string;
}

export interface ShootingShadow {
	readonly enabled: boolean;
	readonly opacity: number;
	readonly softness: number;
}

export interface ShootingMaterial {
	readonly color: string;
	readonly metalness: number;
	readonly roughness: number;
	readonly emissive: string;
	readonly emissiveIntensity: number;
}

export interface ShootingScene {
	readonly background: string;
	readonly sun: ShootingSun;
	readonly ambient: ShootingAmbient;
	readonly shadow: ShootingShadow;
	readonly material: ShootingMaterial;
}

export type ShootingShotFormat = "svg" | "png";

export type ShootingShotShape = IconRenderShape;

export interface ShootingShot {
	readonly id: string;
	readonly label: string;
	readonly width: number;
	readonly height: number;
	readonly format: ShootingShotFormat;
	readonly shape?: ShootingShotShape;
	readonly background?: string;
	readonly cameraId?: string;
}

export interface ShootingFixture {
	readonly schema: "shooting.fixture";
	readonly assets: readonly ShootingAsset[];
	readonly camera: ShootingCamera;
	readonly savedCameras: readonly ShootingSavedCamera[];
	readonly scene: ShootingScene;
	readonly shots: readonly ShootingShot[];
	readonly activeShotId?: string;
	readonly activeAssetId?: string;
}

export const DEFAULT_SHOOTING_SCENE: ShootingScene = {
	background: "",
	sun: { azimuth: 45, elevation: 35, intensity: 2.4, color: "#ffffff" },
	ambient: { intensity: 1.15, color: "#ffffff" },
	shadow: { enabled: true, opacity: 0.35, softness: 1 },
	material: { color: "#9aa0ab", metalness: 0, roughness: 1, emissive: "#000000", emissiveIntensity: 0 },
};

export const DEFAULT_SHOOTING_CAMERA: ShootingCamera = {
	position: [420, -420, 320],
	target: [0, 0, 40],
	zoom: 1,
	fov: 50,
};

export const DEFAULT_SHOOTING_FIXTURE: ShootingFixture = {
	schema: "shooting.fixture",
	assets: [{ id: "base", name: "Base", url: "/mesh/base.glb", format: "glb" }],
	camera: DEFAULT_SHOOTING_CAMERA,
	savedCameras: [],
	scene: DEFAULT_SHOOTING_SCENE,
	shots: [
		{ id: "overview-svg", label: "Overview Svg", width: 256, height: 256, format: "svg", shape: "rectangle" },
		{ id: "overview-png", label: "Overview Png", width: 512, height: 512, format: "png", shape: "ellipse" },
	],
	activeShotId: "overview-svg",
	activeAssetId: "base",
};

export function shootingFixtureToJson(fixture: ShootingFixture): string {
	return JSON.stringify(fixture, null, 2);
}

export function parseShootingFixture(json: string): ShootingFixture | null {
	try {
		const parsed = JSON.parse(json) as ShootingFixture;
		if (parsed.schema !== "shooting.fixture") return null;
		return parsed;
	} catch {
		return null;
	}
}

export function shootingCameraToWorldState(camera: ShootingCamera): WorldCameraState {
	return {
		position: [camera.position[0], camera.position[1], camera.position[2]],
		target: [camera.target[0], camera.target[1], camera.target[2]],
		zoom: camera.zoom,
		...(camera.up ? { up: [camera.up[0], camera.up[1], camera.up[2]] } : {}),
		...(camera.projection ? { projection: camera.projection } : {}),
	};
}

export function worldStateToShootingCamera(state: WorldCameraState, fov?: number): ShootingCamera {
	return {
		position: [state.position[0], state.position[1], state.position[2]],
		target: [state.target[0], state.target[1], state.target[2]],
		zoom: state.zoom,
		...(state.up ? { up: [state.up[0], state.up[1], state.up[2]] } : {}),
		...(state.projection ? { projection: state.projection } : {}),
		...(fov !== undefined ? { fov } : {}),
	};
}

export function resolveActiveShot(fixture: ShootingFixture): ShootingShot | null {
	if (!fixture.shots.length) return null;
	const active = fixture.activeShotId ? fixture.shots.find((shot) => shot.id === fixture.activeShotId) : undefined;
	return active ?? fixture.shots[0] ?? null;
}

export function resolveActiveAsset(fixture: ShootingFixture): ShootingAsset | null {
	if (!fixture.assets.length) return null;
	const active = fixture.activeAssetId ? fixture.assets.find((asset) => asset.id === fixture.activeAssetId) : undefined;
	return active ?? fixture.assets[0] ?? null;
}

export function resolveShootingShotShape(shot: ShootingShot): ShootingShotShape {
	return shot.shape ?? "rectangle";
}

export function resolveShotCamera(fixture: ShootingFixture, shot: ShootingShot): ShootingCamera {
	if (!shot.cameraId) return fixture.camera;
	const saved = fixture.savedCameras.find((entry) => entry.id === shot.cameraId);
	return saved?.camera ?? fixture.camera;
}

export function applyShootingCameraToFixture(fixture: ShootingFixture, shot: ShootingShot, camera: ShootingCamera): ShootingFixture {
	if (!shot.cameraId) return { ...fixture, camera };
	return {
		...fixture,
		savedCameras: fixture.savedCameras.map((entry) => (entry.id === shot.cameraId ? { ...entry, camera } : entry)),
	};
}

function mergeShootingScene(scene: ShootingScene, patch: Partial<ShootingScene>): ShootingScene {
	return {
		...scene,
		...patch,
		sun: patch.sun ? { ...scene.sun, ...patch.sun } : scene.sun,
		ambient: patch.ambient ? { ...scene.ambient, ...patch.ambient } : scene.ambient,
		shadow: patch.shadow ? { ...scene.shadow, ...patch.shadow } : scene.shadow,
		material: patch.material ? { ...scene.material, ...patch.material } : scene.material,
	};
}

function patchShootingShotField(shot: ShootingShot, field: string, value: unknown): ShootingShot {
	if (field === "width" || field === "height") {
		const numeric = typeof value === "number" ? value : Number(value);
		if (!Number.isFinite(numeric)) return shot;
		return { ...shot, [field]: Math.round(numeric) };
	}
	if (field === "format" && (value === "svg" || value === "png")) {
		return { ...shot, format: value };
	}
	if (field === "shape" && (value === "rectangle" || value === "ellipse")) {
		return { ...shot, shape: value };
	}
	if (typeof value !== "string") return shot;
	return { ...shot, [field]: value };
}

export type ShootingFixtureEditOp =
	| { readonly op: "setDocument"; readonly document: ShootingFixture }
	| { readonly op: "patchScene"; readonly patch: Partial<ShootingScene> }
	| { readonly op: "patchShots"; readonly shotIds: readonly string[]; readonly field: string; readonly value: unknown }
	| { readonly op: "patchShot"; readonly shotId: string; readonly field: string; readonly value: unknown }
	| { readonly op: "patchAssets"; readonly assetIds: readonly string[]; readonly field: string; readonly value: unknown }
	| { readonly op: "setActiveShot"; readonly shotId: string }
	| { readonly op: "setActiveAsset"; readonly assetId: string }
	| { readonly op: "setCamera"; readonly camera: ShootingCamera }
	| { readonly op: "setShotCamera"; readonly shotId: string; readonly camera: ShootingCamera }
	| { readonly op: "addSavedCamera"; readonly entry: ShootingSavedCamera }
	| { readonly op: "loadSavedCamera"; readonly cameraId: string }
	| { readonly op: "importAsset"; readonly asset: ShootingAsset; readonly setActive: boolean };

/** @emoji 🚪 Applies one semantic shooting fixture edit (CQRS projection applier). */
export function applyShootingFixtureEditOp(fixture: ShootingFixture, op: ShootingFixtureEditOp): ShootingFixture {
	switch (op.op) {
		case "setDocument":
			return op.document;
		case "patchScene":
			return { ...fixture, scene: mergeShootingScene(fixture.scene, op.patch) };
		case "patchShots": {
			const targets = new Set(op.shotIds);
			return {
				...fixture,
				shots: fixture.shots.map((shot) => (targets.has(shot.id) ? patchShootingShotField(shot, op.field, op.value) : shot)),
			};
		}
		case "patchShot":
			return applyShootingFixtureEditOp(fixture, { op: "patchShots", shotIds: [op.shotId], field: op.field, value: op.value });
		case "patchAssets": {
			const targets = new Set(op.assetIds);
			return {
				...fixture,
				assets: fixture.assets.map((asset) => {
					if (!targets.has(asset.id)) return asset;
					if (typeof op.value !== "string") return asset;
					return { ...asset, [op.field]: op.value };
				}),
			};
		}
		case "setActiveShot":
			return { ...fixture, activeShotId: op.shotId };
		case "setActiveAsset":
			return { ...fixture, activeAssetId: op.assetId };
		case "setCamera":
			return { ...fixture, camera: op.camera };
		case "setShotCamera": {
			const shot = fixture.shots.find((entry) => entry.id === op.shotId);
			if (!shot) return fixture;
			return applyShootingCameraToFixture(fixture, shot, op.camera);
		}
		case "addSavedCamera":
			return { ...fixture, savedCameras: [...fixture.savedCameras, op.entry] };
		case "loadSavedCamera": {
			const saved = fixture.savedCameras.find((entry) => entry.id === op.cameraId);
			return saved ? { ...fixture, camera: saved.camera } : fixture;
		}
		case "importAsset":
			return {
				...fixture,
				assets: [...fixture.assets, op.asset],
				...(op.setActive ? { activeAssetId: op.asset.id } : {}),
			};
	}
}

/** @emoji ↩️ Inverts a shooting fixture edit from the pre-apply projection. */
export function backwardsShootingFixtureEditOp(fixture: ShootingFixture, op: ShootingFixtureEditOp): readonly ShootingFixtureEditOp[] {
	switch (op.op) {
		case "setDocument":
			return [{ op: "setDocument", document: fixture }];
		case "setActiveShot":
			return [{ op: "setActiveShot", shotId: fixture.activeShotId ?? fixture.shots[0]?.id ?? op.shotId }];
		case "setActiveAsset":
			return [{ op: "setActiveAsset", assetId: fixture.activeAssetId ?? fixture.assets[0]?.id ?? op.assetId }];
		case "setCamera":
			return [{ op: "setCamera", camera: fixture.camera }];
		case "setShotCamera":
			return [{ op: "setShotCamera", shotId: op.shotId, camera: resolveShotCamera(fixture, fixture.shots.find((shot) => shot.id === op.shotId) ?? fixture.shots[0]!) }];
		case "addSavedCamera":
			return [{ op: "setDocument", document: fixture }];
		case "loadSavedCamera":
			return [{ op: "setCamera", camera: fixture.camera }];
		case "importAsset":
			return [{ op: "setDocument", document: fixture }];
		case "patchScene":
		case "patchShots":
		case "patchShot":
		case "patchAssets":
			return [{ op: "setDocument", document: fixture }];
	}
}

/** @emoji 📊 Returns the shooting fixture edit payload for persistence diffs. */
export function diffShootingFixtureEditOp(_fixture: ShootingFixture, operation: ShootingFixtureEditOp): unknown {
	return operation;
}

export function shootingIconRenderRequest(fixture: ShootingFixture, shot: ShootingShot, asset: ShootingAsset): IconRenderRequest {
	const camera = resolveShotCamera(fixture, shot);
	const background = shot.background ?? fixture.scene.background;
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
		shape: resolveShootingShotShape(shot),
		...(isShootingTransparentBackground(background) ? {} : { background }),
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
export function isShootingTransparentBackground(background: string | undefined): boolean {
	return !background || background === "transparent";
}

export function resolveShootingCanvasBackground(background: string | undefined): string | undefined {
	return isShootingTransparentBackground(background) ? undefined : background;
}

export function shootingCanvasGl(background: string | undefined): { readonly antialias: true; readonly alpha: boolean } {
	return { antialias: true, alpha: isShootingTransparentBackground(background) };
}

export function sunPositionFromAzimuthElevation(azimuthDeg: number, elevationDeg: number, distance = 120): [number, number, number] {
	const az = (azimuthDeg * Math.PI) / 180;
	const el = (elevationDeg * Math.PI) / 180;
	return [Math.cos(el) * Math.cos(az) * distance, Math.cos(el) * Math.sin(az) * distance, Math.sin(el) * distance];
}

function createStyledMaterial(material: ShootingMaterial): MeshStandardMaterial {
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
	camera: Pick<ShootingCamera, "position" | "target" | "zoom" | "projection">,
	padding = 1.25,
): Pick<ShootingCamera, "position" | "target" | "zoom"> {
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
	readonly camera: ShootingCamera;
	readonly projection: OrbitCameraProjection;
	readonly onCamera: (camera: ShootingCamera) => void;
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
	readonly material: ShootingMaterial;
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
	readonly fixture: ShootingFixture;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly onCamera?: (camera: ShootingCamera) => void;
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
		(next: ShootingCamera) => {
			onCamera?.(next);
		},
		[onCamera],
	);
	const shot = resolveActiveShot(fixture);
	const onProjectionChange = reactHostPort.useCallback((next: OrbitCameraProjection) => {
		handleCamera({ ...fixture.camera, projection: next });
	}, [fixture.camera, handleCamera]);
	const modelOverlay = (
		<>
			{shot ? <ShootingShotFrame height={shot.height} shape={resolveShootingShotShape(shot)} width={shot.width} /> : null}
			<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />
		</>
	);
	if (!asset) {
		return <div className={cn("flex h-full items-center justify-center text-sm opacity-60", className)}>No asset</div>;
	}
	return (
		<div className={cn("absolute inset-0", className)} style={style}>
			<WorldCanvas
				className="h-full w-full"
				background={resolveShootingCanvasBackground(fixture.scene.background)}
				gl={shootingCanvasGl(fixture.scene.background)}
				shadows={fixture.scene.shadow.enabled}
				overlay={modelOverlay}
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

//#region 🔖ShotFrame
export function shootingShotFrameStyle(width: number, height: number): CSSProperties {
	const landscape = width >= height;
	return {
		aspectRatio: `${width} / ${height}`,
		maxHeight: "100%",
		maxWidth: "100%",
		width: landscape ? "100%" : "auto",
		height: landscape ? "auto" : "100%",
	};
}

export function shootingShotFrameClass(shape: ShootingShotShape): string {
	return shape === "ellipse" ? "rounded-full" : "rounded-none";
}

export function ShootingShotFrame({
	width,
	height,
	shape = "rectangle",
	className,
	background,
	children,
}: {
	readonly width: number;
	readonly height: number;
	readonly shape?: ShootingShotShape;
	readonly className?: string;
	readonly background?: string;
	readonly children?: ReactNode;
}): ReactNode {
	return (
		<div className={cn("pointer-events-none absolute inset-0 flex items-center justify-center", className)}>
			<div
				className={cn(
					"relative box-border overflow-hidden border-2 border-accent shadow-[0_0_0_1px_color-mix(in_srgb,var(--foreground)_20%,transparent)]",
					shootingShotFrameClass(shape),
				)}
				data-shooting-shot-frame
				data-shooting-shot-shape={shape}
				style={{
					...shootingShotFrameStyle(width, height),
					...(background ? { background } : {}),
				}}
			>
				{children}
				<span className="pointer-events-none absolute bottom-1 right-1 rounded-sm bg-background/80 px-1 font-mono text-[10px] text-muted-foreground">
					{width}×{height} · {shape}
				</span>
			</div>
		</div>
	);
}
//#endregion 🔖ShotFrame

//#region 🔖IconCanvas
export interface ShootingIconCanvasProps {
	readonly fixture: ShootingFixture;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly renderRevision?: number;
}

export function ShootingIconCanvas({ fixture, className, style, renderRevision = 0 }: ShootingIconCanvasProps): ReactNode {
	const shot = resolveActiveShot(fixture);
	const asset = resolveActiveAsset(fixture);
	const [preview, setPreview] = reactHostPort.useState<{ dataUrl: string; label: string } | null>(null);
	const [error, setError] = reactHostPort.useState<string | null>(null);
	reactHostPort.useEffect(() => {
		if (!shot || !asset) {
			setPreview(null);
			return;
		}
		let cancelled = false;
		void iconRenderPort
			.render(shootingIconRenderRequest(fixture, shot, asset))
			.then((result) => {
				if (cancelled) return;
				console.log(`[DEBUG] shooting icon rendered ${shot.id} ${shot.format} ${shot.width}x${shot.height}`);
				setPreview({ dataUrl: result.dataUrl, label: shot.label });
				setError(null);
			})
			.catch((err: unknown) => {
				if (cancelled) return;
				const message = err instanceof Error ? err.message : String(err);
				console.log(`[DEBUG] shooting icon render failed ${shot.id}: ${message}`);
				setError(message);
				setPreview(null);
			});
		return () => {
			cancelled = true;
		};
	}, [fixture, shot, asset, renderRevision]);
	if (!shot || !asset) {
		return <div className={cn("flex h-full items-center justify-center text-sm opacity-60", className)}>No shot</div>;
	}
	const shotBackground = shot.background ?? fixture.scene.background;
	const previewBackground = isShootingTransparentBackground(shotBackground) ? undefined : shotBackground;
	const frame = (content: ReactNode) => (
		<div className={cn("absolute inset-0 flex flex-col", className)} style={style}>
			<div className="relative min-h-0 flex-1">
				<ShootingShotFrame
					background={previewBackground}
					height={shot.height}
					shape={resolveShootingShotShape(shot)}
					width={shot.width}
				>
					{content}
				</ShootingShotFrame>
			</div>
			<div className="shrink-0 px-3 pb-2 text-center text-xs opacity-60">
				{shot.label} · {shot.width}×{shot.height} · {shot.format.toUpperCase()}
			</div>
		</div>
	);
	if (error) {
		return frame(<div className="flex h-full items-center justify-center p-4 text-sm text-red-500">{error}</div>);
	}
	if (!preview) {
		return frame(<div className="flex h-full items-center justify-center text-sm opacity-60">Rendering…</div>);
	}
	return frame(
		<img alt={preview.label} className="block h-full w-full" src={preview.dataUrl} />,
	);
}

export async function renderShootingShot(fixture: ShootingFixture, shot: ShootingShot, asset?: ShootingAsset) {
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
			expect(shootingFixtureToJson(DEFAULT_SHOOTING_FIXTURE)).toContain("shooting.fixture");
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

		it("treats empty background as transparent for canvas and export", () => {
			expect(isShootingTransparentBackground("")).toBe(true);
			expect(resolveShootingCanvasBackground("")).toBeUndefined();
			expect(shootingCanvasGl("").alpha).toBe(true);
			const shot = DEFAULT_SHOOTING_FIXTURE.shots[0]!;
			const asset = DEFAULT_SHOOTING_FIXTURE.assets[0]!;
			expect(shootingIconRenderRequest(DEFAULT_SHOOTING_FIXTURE, shot, asset).background).toBeUndefined();
		});

		it("sizes shot frame from fixed width and height", () => {
			expect(shootingShotFrameStyle(512, 256)).toMatchObject({ aspectRatio: "512 / 256", width: "100%" });
			expect(shootingShotFrameStyle(256, 512)).toMatchObject({ aspectRatio: "256 / 512", height: "100%" });
		});

		it("resolves shot shape and passes it to icon render request", () => {
			const shot = DEFAULT_SHOOTING_FIXTURE.shots[1]!;
			expect(resolveShootingShotShape(shot)).toBe("ellipse");
			expect(shootingShotFrameClass("ellipse")).toBe("rounded-full");
			const asset = DEFAULT_SHOOTING_FIXTURE.assets[0]!;
			expect(shootingIconRenderRequest(DEFAULT_SHOOTING_FIXTURE, shot, asset).shape).toBe("ellipse");
		});
	});
}
// #endregion 🧪Tests
