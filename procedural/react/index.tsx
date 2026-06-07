// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔧 `@procedural/react` — flow-based brep editor with R3F viewport. */
// #endregion 🧲Header

// #region 🔌Adapters
import { reactHostPort, sceneHostPort } from "@ui/react";
import {
	brepjsGeometryKernel,
	BrepjsGeometryKernel,
	ensureBrepWasmLoaded,
	isRenderableMeshTransfer,
	meshTransferToGeometryData,
	type BrepjsGeometryKernel as BrepKernelType,
	type SolidRef,
	type Vec3,
} from "@geometry/brep/js";
import {
	FlowCanvas,
	FlowExtensionHost,
	createFlowEvalBridge,
	type CatalogueSection,
	type FlowExtensionEntry,
	type FlowFixtureV1,
	type FlowModuleCommandV1,
	type FlowModuleManifestV1,
	type FlowModuleNeuronKindV1,
	type FlowReorganizeRequest,
} from "@flow/react";
import {
	WorldCameraInvalidator,
	WorldCanvas,
	WorldOrbitGated,
} from "@infinite/world/r3f";
import { useEffect, useRef, useState, type ReactNode } from "react";

const THREE = sceneHostPort.three;
// #endregion 🔌Adapters

// #region 🔖BrepWasmBridge
if (!import.meta.env.VITEST) {
	await ensureBrepWasmLoaded();
}
// #endregion 🔖BrepWasmBridge

// #region 🔖BrepFlowModule
const BREP_FLOW_KINDS: readonly FlowModuleNeuronKindV1[] = [
	{ id: "brep.box", module: "brep", name: "Box", summary: "Axis-aligned box solid", inputs: ["cornerA", "cornerB", "height"], outputs: ["brep"] },
	{ id: "brep.sphere", module: "brep", name: "Sphere", summary: "Sphere solid", inputs: ["center", "radius"], outputs: ["brep"] },
	{ id: "brep.cylinder", module: "brep", name: "Cylinder", summary: "Cylinder solid", inputs: ["base", "axis", "radius", "height"], outputs: ["brep"] },
	{ id: "brep.extrude", module: "brep", name: "Extrude", summary: "Extrude a solid", inputs: ["brep", "direction", "distance"], outputs: ["brep"] },
	{ id: "brep.translate", module: "brep", name: "Translate", summary: "Translate a solid", inputs: ["brep", "offset"], outputs: ["brep"] },
	{ id: "brep.union", module: "brep", name: "Union", summary: "Boolean union", inputs: ["a", "b"], outputs: ["brep"] },
];

const BREP_MODULE_MANIFEST: FlowModuleManifestV1 = {
	schema: "flow.module/v1",
	id: "brep",
	name: "Brep",
	version: "0.1.0",
	activationEvents: ["onStartup"],
	contributes: { neuronKinds: BREP_FLOW_KINDS, widgets: [], commands: [], settings: [] },
};

function parseVec3(input: unknown, fallback: Vec3 = [0, 0, 0]): Vec3 {
	if (!Array.isArray(input) || input.length < 3) return fallback;
	return [Number(input[0]) || 0, Number(input[1]) || 0, Number(input[2]) || 0];
}

function parseSolidRef(input: Record<string, unknown>, key: string): SolidRef | null {
	const raw = input[key];
	return typeof raw === "string" && raw.length > 0 ? (raw as SolidRef) : null;
}

/** @emoji 🧊 Synchronous brep neuron evaluation (requires pre-initialized WASM). */
export function evaluateBrepFlowKind(kindId: string, inputJson: string, kernel: BrepKernelType): string {
	const input = JSON.parse(inputJson) as Record<string, unknown>;
	try {
		if (kindId === "brep.box") {
			const solid = kernel.createBoxFromCornersSync({
				cornerA: parseVec3(input.cornerA, [0, 0, 0]),
				cornerB: parseVec3(input.cornerB, [1, 1, 0]),
				height: Number(input.height) || 1,
			});
			return JSON.stringify({ brep: String(solid) });
		}
		if (kindId === "brep.sphere") {
			const solid = kernel.createSphereSync(parseVec3(input.center), Number(input.radius) || 1);
			return JSON.stringify({ brep: String(solid) });
		}
		if (kindId === "brep.cylinder") {
			const solid = kernel.createCylinderSync(
				parseVec3(input.base),
				parseVec3(input.axis, [0, 0, 1]),
				Number(input.radius) || 1,
				Number(input.height) || 1,
			);
			return JSON.stringify({ brep: String(solid) });
		}
		if (kindId === "brep.extrude") {
			const src = parseSolidRef(input, "brep");
			if (!src) return JSON.stringify({ error: "missing brep input" });
			const solid = kernel.extrudeSolidSync(src, parseVec3(input.direction, [0, 0, 1]), Number(input.distance) || 1);
			return JSON.stringify({ brep: String(solid) });
		}
		if (kindId === "brep.translate") {
			const src = parseSolidRef(input, "brep");
			if (!src) return JSON.stringify({ error: "missing brep input" });
			const solid = kernel.translateSolidSync(src, parseVec3(input.offset));
			return JSON.stringify({ brep: String(solid) });
		}
		if (kindId === "brep.union") {
			const a = parseSolidRef(input, "a");
			const b = parseSolidRef(input, "b");
			if (!a || !b) return JSON.stringify({ error: "missing union inputs" });
			const solid = kernel.fuseSolidsSync([a, b]);
			return JSON.stringify({ brep: String(solid) });
		}
		return JSON.stringify({ error: `unknown brep kind: ${kindId}` });
	} catch (e) {
		return JSON.stringify({ error: e instanceof Error ? e.message : String(e) });
	}
}

/** @emoji 🔌 Flow extension host with brep JS module + default flow modules. */
export class ProceduralExtensionHost extends FlowExtensionHost {
	private brepKernel: BrepKernelType;
	private brepReady = false;

	constructor(kernel: BrepKernelType = new BrepjsGeometryKernel()) {
		super();
		this.brepKernel = kernel;
		this.brepReady = import.meta.env.VITEST === true;
	}

	getBrepKernel(): BrepKernelType {
		return this.brepKernel;
	}

	async activateDefaults(): Promise<void> {
		await ensureBrepWasmLoaded();
		this.brepReady = true;
		await super.activateDefaults();
	}

	override evaluate(kindId: string, inputJson: string): string {
		if (kindId.startsWith("brep.")) {
			if (!this.brepReady) return JSON.stringify({ error: "brep wasm not ready" });
			return evaluateBrepFlowKind(kindId, inputJson, this.brepKernel);
		}
		return super.evaluate(kindId, inputJson);
	}

	override catalogueSections(): CatalogueSection[] {
		const sections = [...super.catalogueSections()];
		sections.push({
			id: "brep",
			title: "Brep",
			items: BREP_FLOW_KINDS.map((kind) => ({
				kind: "neuron",
				neuronKind: kind.id,
				name: kind.name,
				summary: kind.summary,
			})),
		});
		return sections;
	}

	override kindInfosJson(): string {
		const kinds = JSON.parse(super.kindInfosJson()) as FlowModuleNeuronKindV1[];
		return JSON.stringify([...kinds, ...BREP_FLOW_KINDS]);
	}

	override listEntries(): readonly FlowExtensionEntry[] {
		return [
			...super.listEntries(),
			{ id: "brep", manifest: BREP_MODULE_MANIFEST, active: this.brepReady },
		];
	}
}

export const proceduralExtensionHost = new ProceduralExtensionHost();
// #endregion 🔖BrepFlowModule

// #region 🔖Fixture
export const PROCEDURAL_DEFAULT_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [
		{ kind: "neuron", id: "box", neuronKind: "brep.box" },
		{ kind: "outputPreview", id: "preview" },
	],
	synapses: [{ id: "s1", from: "box", to: "preview", fromPort: "out", toPort: "in" }],
};

export function proceduralFixtureToJson(fixture: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE): string {
	return JSON.stringify(fixture);
}
// #endregion 🔖Fixture

// #region 🔖BrepViewport
export interface BrepViewportProps {
	readonly solidId: string | null;
	readonly kernel?: BrepKernelType;
	readonly tolerance?: number;
	readonly className?: string;
}

function BrepMesh({ solidId, kernel, tolerance }: { readonly solidId: string; readonly kernel: BrepKernelType; readonly tolerance: number }): ReactNode {
	const [geo, setGeo] = useState<THREE.BufferGeometry | null>(null);
	const solid = solidId as SolidRef;

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			await ensureBrepWasmLoaded();
			const mesh = await kernel.tessellate(solid, tolerance);
			if (cancelled || !isRenderableMeshTransfer(mesh)) {
				setGeo(null);
				return;
			}
			const data = meshTransferToGeometryData(mesh);
			const geometry = new THREE.BufferGeometry();
			geometry.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
			geometry.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
			geometry.setIndex(new THREE.BufferAttribute(data.index, 1));
			for (const g of data.faceGroups) geometry.addGroup(g.start, g.count, 0);
			setGeo(geometry);
		})();
		return () => {
			cancelled = true;
		};
	}, [kernel, solid, tolerance]);

	if (!geo) return null;
	return (
		<mesh geometry={geo}>
			<meshStandardMaterial color="#6b9bd1" metalness={0.1} roughness={0.6} transparent opacity={0.85} side={THREE.DoubleSide} />
		</mesh>
	);
}

export function BrepViewport({ solidId, kernel = brepjsGeometryKernel, tolerance = 0.02, className }: BrepViewportProps): ReactNode {
	return (
		<div className={className ?? "relative h-full w-full bg-zinc-900"}>
			<WorldCanvas frameloop="demand" cameraPosition={[8, 8, 6]} background="#18181b">
				<WorldCameraInvalidator />
				<ambientLight intensity={0.45} />
				<directionalLight position={[12, 18, 10]} intensity={1.1} />
				<WorldOrbitGated />
				{solidId ? <BrepMesh solidId={solidId} kernel={kernel} tolerance={tolerance} /> : null}
			</WorldCanvas>
		</div>
	);
}
// #endregion 🔖BrepViewport

// #region 🔖ProceduralEditor
export interface ProceduralEditorProps {
	readonly fixtureJson?: string;
	readonly className?: string;
	readonly extensionHost?: ProceduralExtensionHost;
	readonly reorganize?: FlowReorganizeRequest;
	readonly extensionRevision?: number;
	readonly onPreviewText?: (text: string) => void;
	readonly onFixtureChange?: (fixtureJson: string) => void;
}

function extractBrepSolidId(outputsJson: string): string | null {
	try {
		const outputs = JSON.parse(outputsJson) as Record<string, Record<string, unknown>>;
		for (const dict of Object.values(outputs)) {
			if (typeof dict.brep === "string" && dict.brep.length > 0) return dict.brep;
		}
	} catch {
		/* ignore */
	}
	return null;
}

export function ProceduralEditor({
	fixtureJson,
	className,
	extensionHost = proceduralExtensionHost,
	reorganize,
	extensionRevision = 0,
	onPreviewText,
	onFixtureChange,
}: ProceduralEditorProps): ReactNode {
	const [brepSolidId, setBrepSolidId] = useState<string | null>(null);
	const hostRef = useRef(extensionHost);

	useEffect(() => {
		hostRef.current = extensionHost;
		void extensionHost.activateDefaults();
	}, [extensionHost]);

	const onEvalOutputs = reactHostPort.useCallback((outputsJson: string) => {
		const id = extractBrepSolidId(outputsJson);
		if (id) {
			console.log(`[DEBUG] procedural brep output: ${id}`);
			setBrepSolidId(id);
		}
	}, []);

	return (
		<div className={className ?? "flex h-full w-full min-h-0"}>
			<div className="min-w-0 flex-1 border-r border-zinc-700">
				<FlowCanvas
					fixtureJson={fixtureJson}
					fixtureDragDrop
					reorganize={reorganize}
					extensionRevision={extensionRevision}
					extensionHost={extensionHost}
					onPreviewText={onPreviewText}
					onEvalOutputs={onEvalOutputs}
					onFixtureChange={onFixtureChange}
					className="h-full w-full"
				/>
			</div>
			<div className="min-w-0 flex-1">
				<BrepViewport solidId={brepSolidId} kernel={extensionHost.getBrepKernel()} />
			</div>
		</div>
	);
}

export { createFlowEvalBridge, type FlowModuleCommandV1, type FlowReorganizeRequest };
// #endregion 🔖ProceduralEditor

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;
	const { ProceduralExtensionHost, evaluateBrepFlowKind } = await import("./index.tsx");
	const { BrepjsGeometryKernel, ensureBrepWasmLoaded } = await import("@geometry/brep/js");

	describe("@procedural/react", () => {
		const kernel = new BrepjsGeometryKernel();

		beforeAll(async () => {
			await ensureBrepWasmLoaded();
		});

		it("brep.box evaluates to solid id", () => {
			const out = evaluateBrepFlowKind(
				"brep.box",
				JSON.stringify({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }),
				kernel,
			);
			const parsed = JSON.parse(out) as { brep?: string };
			expect(parsed.brep).toBeTruthy();
		});

		it("procedural host includes brep catalogue section", async () => {
			const host = new ProceduralExtensionHost(kernel);
			await host.activateDefaults();
			const sections = host.catalogueSections();
			expect(sections.some((s) => s.id === "brep")).toBe(true);
		});
	});
}
// #endregion 🧪Tests
