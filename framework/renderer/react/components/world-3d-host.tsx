import { useCallback, useMemo } from "react";
import { WorldCanvas, WorldLayerStack, WorldLodGridHelper } from "@semio-tech/infinite-world-r3f";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

//#region WorldSceneParsing
type WorldCameraRecord = {
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
	readonly fov?: number;
};

type WorldInstanceRecord = {
	readonly id?: string;
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
	readonly scale?: number;
	readonly color?: string;
	readonly label?: string;
};

function parseCamera(cameraJson: string): { readonly position: [number, number, number]; readonly fov: number } {
	try {
		const parsed = JSON.parse(cameraJson) as WorldCameraRecord;
		return {
			position: [parsed.x ?? 0, parsed.y ?? 0, parsed.z ?? 5],
			fov: parsed.fov ?? 45,
		};
	} catch {
		return { position: [0, 0, 5], fov: 45 };
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
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({ instances }: { readonly instances: readonly WorldInstanceRecord[] }) {
	return (
		<WorldLayerStack>
			<WorldLodGridHelper />
			<group>
				{instances.map((instance, index) => {
					const scale = instance.scale ?? 1;
					const color = instance.color ?? `hsl(${(index * 53) % 360} 60% 55%)`;
					return (
						<mesh key={instance.id ?? `instance-${index}`} position={[instance.x ?? index, instance.y ?? 0, instance.z ?? 0]} scale={scale}>
							<boxGeometry args={[1, 1, 1]} />
							<meshStandardMaterial color={color} />
						</mesh>
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
	const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
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

	if (!scene) return <div className="semio-world-3d-empty">No world scene</div>;

	return (
		<div className="semio-world-3d-host h-full min-h-[24rem] w-full" data-surface-id={node.surfaceId}>
			<WorldCanvas
				className="h-full w-full"
				cameraUp={[0, 0, 1]}
				cameraPosition={camera.position}
				cameraFov={camera.fov}
				onPointerDown={() => dispatch("worldPointerDown")}
			>
				<ambientLight intensity={0.65} />
				<directionalLight intensity={0.85} position={[4, 6, 8]} />
				<WorldInstancesLayer instances={instances} />
			</WorldCanvas>
		</div>
	);
}
//#endregion World3dHost
