import { useCallback, useMemo, useRef, useState, Suspense } from "react";
import { BufferAttribute, BufferGeometry, MeshStandardMaterial, Quaternion } from "three";
import { useLoader } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import {
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WorldCanvas,
	WorldLayerStack,
	WorldLodBridge,
} from "@semio-tech/infinite-world-r3f";
import {
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	SelectionMarquee,
	type SelectionMarqueeMethod,
	type SelectionMarqueePoint,
} from "@semio-tech/ui-react";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

//#region WorldSceneParsing
type WorldCameraRecord = {
	readonly position?: readonly [number, number, number];
	readonly target?: readonly [number, number, number];
	readonly fov?: number;
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
};

type WorldMeshRecord = {
	readonly id: string;
	readonly data?: {
		readonly positions: readonly number[];
		readonly normals: readonly number[];
		readonly indices: readonly number[];
	};
	readonly url?: string;
};

type WorldInstanceRecord = {
	readonly id: string;
	readonly meshId?: string;
	readonly position?: readonly [number, number, number];
	readonly rotation?: readonly [number, number, number, number];
	readonly scale?: readonly [number, number, number];
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
	readonly color?: string;
	readonly selected?: boolean;
	readonly hovered?: boolean;
};

type WorldSelectionRecord = {
	readonly method?: SelectionMarqueeMethod;
	readonly ids?: readonly string[];
};

function parseCamera(cameraJson: string): { readonly position: [number, number, number]; readonly fov: number } {
	try {
		const parsed = JSON.parse(cameraJson) as WorldCameraRecord;
		if (parsed.position) {
			return {
				position: [parsed.position[0], parsed.position[1], parsed.position[2]],
				fov: parsed.fov ?? 45,
			};
		}
		return {
			position: [parsed.x ?? 4, parsed.y ?? -4, parsed.z ?? 3],
			fov: parsed.fov ?? 45,
		};
	} catch {
		return { position: [4, -4, 3], fov: 45 };
	}
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
	try {
		const parsed = JSON.parse(meshesJson);
		return Array.isArray(parsed) ? (parsed as WorldMeshRecord[]) : [];
	} catch {
		return [];
	}
}

function parseInstances(instancesJson: string): WorldInstanceRecord[] {
	try {
		const parsed = JSON.parse(instancesJson);
		return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
	} catch {
		return [];
	}
}

function parseSelection(selectionJson: string): WorldSelectionRecord {
	try {
		return JSON.parse(selectionJson) as WorldSelectionRecord;
	} catch {
		return { method: "rectangle", ids: [] };
	}
}

function geometryFromMesh(mesh: NonNullable<WorldMeshRecord["data"]>) {
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
	geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
	if (mesh.indices.length > 0) geometry.setIndex([...mesh.indices]);
	return geometry;
}

function GlbInstanceMesh({ url, color }: { readonly url: string; readonly color: string }) {
	const gltf = useLoader(GLTFLoader, url);
	const scene = useMemo(() => {
		const cloned = gltf.scene.clone(true);
		cloned.traverse((child) => {
			if ("isMesh" in child && (child as { isMesh?: boolean }).isMesh) {
				const mesh = child as import("three").Mesh;
				mesh.material = new MeshStandardMaterial({ color });
			}
		});
		return cloned;
	}, [gltf.scene, color]);
	return <primitive object={scene} />;
}
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({
	instances,
	meshes,
	onInstancePointerDown,
	onInstancePointerMove,
}: {
	readonly instances: readonly WorldInstanceRecord[];
	readonly meshes: readonly WorldMeshRecord[];
	readonly onInstancePointerDown: (id: string, event: { shiftKey: boolean; ctrlKey: boolean }) => void;
	readonly onInstancePointerMove: (id: string | null) => void;
}) {
	const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
	const geometries = useMemo(() => {
		const map = new Map<string, BufferGeometry>();
		for (const mesh of meshes) {
			if (mesh.data) map.set(mesh.id, geometryFromMesh(mesh.data));
		}
		return map;
	}, [meshes]);

	return (
		<WorldLayerStack>
			<group>
				{instances.map((instance, index) => {
					const meshId = instance.meshId ?? "box";
					const meshRecord = meshById.get(meshId);
					const geometry = geometries.get(meshId);
					const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
					const scale = instance.scale ?? [1, 1, 1];
					const rotation = instance.rotation;
					const quaternion = rotation
						? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3])
						: undefined;
					const color =
						instance.color ??
						(instance.selected ? "#60a5fa" : instance.hovered ? "#fbbf24" : "#94a3b8");
					return (
						<group
							key={instance.id}
							position={position as [number, number, number]}
							scale={scale as [number, number, number]}
							quaternion={quaternion}
							onPointerDown={(event) => {
								event.stopPropagation();
								onInstancePointerDown(instance.id, event);
							}}
							onPointerMove={(event) => {
								event.stopPropagation();
								onInstancePointerMove(instance.id);
							}}
							onPointerOut={() => onInstancePointerMove(null)}
						>
							{meshRecord?.url ? (
								<Suspense fallback={null}>
									<GlbInstanceMesh url={meshRecord.url} color={color} />
								</Suspense>
							) : geometry ? (
								<mesh geometry={geometry}>
									<meshStandardMaterial color={color} />
								</mesh>
							) : (
								<mesh>
									<boxGeometry args={[1, 1, 1]} />
									<meshStandardMaterial color={color} />
								</mesh>
							)}
						</group>
					);
				})}
			</group>
		</WorldLayerStack>
	);
}
//#endregion WorldInstancesLayer

//#region World3dHost
export function World3dHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.world3d;
	const camera = useMemo(() => parseCamera(scene?.cameraJson ?? "{}"), [scene?.cameraJson]);
	const meshes = useMemo(() => parseMeshes(scene?.meshesJson ?? "[]"), [scene?.meshesJson]);
	const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
	const selection = useMemo(() => parseSelection(scene?.selectionJson ?? "{}"), [scene?.selectionJson]);
	const hostRef = useRef<HTMLDivElement | null>(null);
	const lodRef = useRef(DEFAULT_MANUAL_LOD);
	const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
	const [marqueeActive, setMarqueeActive] = useState(false);

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({
				controllerId: node.controllerId,
				command,
				args: { surfaceId: node.surfaceId, ...args },
			});
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const mergeModeToArg = (mode: ReturnType<typeof marqueeModeFromModifiers>) => {
		if (mode === "additive") return "add";
		if (mode === "subtractive" || mode === "invertive") return "toggle";
		return "replace";
	};

	const handleInstancePointerDown = useCallback(
		(id: string, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
			dispatch("worldSelect", {
				ids: [id],
				merge: mergeModeToArg(marqueeModeFromModifiers(event)),
			});
		},
		[dispatch],
	);

	const handleInstancePointerMove = useCallback(
		(id: string | null) => {
			dispatch("worldHover", { id });
		},
		[dispatch],
	);

	const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
		const rect = hostRef.current?.getBoundingClientRect();
		if (!rect) return { x: event.clientX, y: event.clientY };
		return { x: event.clientX - rect.left, y: event.clientY - rect.top };
	}, []);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (event.button !== 0 || event.target !== hostRef.current) return;
			setMarqueeActive(true);
			setMarqueePath([toLocalPoint(event)]);
		},
		[toLocalPoint],
	);

	const handlePointerMove = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!marqueeActive) return;
			setMarqueePath((path) => [...path, toLocalPoint(event)]);
		},
		[marqueeActive, toLocalPoint],
	);

	const handlePointerUp = useCallback(() => {
		setMarqueeActive(false);
		setMarqueePath([]);
	}, []);

	if (!scene) return <div className="semio-world-3d-empty">No world scene</div>;

	const method = selection.method ?? "rectangle";
	const marqueeStart = marqueePath[0];
	const marqueeEnd = marqueePath[marqueePath.length - 1];
	const marqueeCoverage =
		marqueeStart && marqueeEnd
			? marqueeCoverageFromGesture({
					method,
					startX: marqueeStart.x,
					endX: marqueeEnd.x,
					path: marqueePath,
				})
			: "full";

	return (
		<div
			ref={hostRef}
			className="semio-world-3d-host relative h-full min-h-[24rem] w-full"
			data-surface-id={node.surfaceId}
			onPointerDown={handlePointerDown}
			onPointerMove={handlePointerMove}
			onPointerUp={handlePointerUp}
		>
			<WorldCanvas className="h-full w-full" cameraUp={[0, 0, 1]} cameraPosition={camera.position} cameraFov={camera.fov}>
				<WorldLodBridge
					lodRef={lodRef}
					distanceReference={100}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled={false}
					showLodGrid
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
					gridDatum={[0, 0, 0]}
				>
					<ambientLight intensity={0.65} />
					<directionalLight intensity={0.85} position={[4, 6, 8]} />
					<WorldInstancesLayer
						instances={instances}
						meshes={meshes}
						onInstancePointerDown={handleInstancePointerDown}
						onInstancePointerMove={handleInstancePointerMove}
					/>
				</WorldLodBridge>
			</WorldCanvas>
			{marqueeActive && marqueePath.length > 1 && marqueeStart && marqueeEnd ? (
				method === "lasso" ? (
					<SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
				) : (
					<SelectionMarquee
						coverage={marqueeCoverage}
						shape="rect"
						rect={{
							x: Math.min(marqueeStart.x, marqueeEnd.x),
							y: Math.min(marqueeStart.y, marqueeEnd.y),
							width: Math.abs(marqueeEnd.x - marqueeStart.x),
							height: Math.abs(marqueeEnd.y - marqueeStart.y),
						}}
					/>
				)
			) : null}
		</div>
	);
}
//#endregion World3dHost
