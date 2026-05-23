// #region 🧲Header
/** @emoji 🔧 Hexagonal CAD engine: pure plugin commands, runtime registry, Topologic wasm kernel adapter, and R3F `CadCanvas` wiring. */
// #endregion 🧲Header

import { OrbitControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import {
	createContext,
	useContext,
	useEffect,
	useLayoutEffect,
	useMemo,
	useRef,
	useState,
	type ReactElement,
	type ReactNode,
} from "react";
import { CylinderGeometry, DoubleSide, Group, Mesh, MeshBasicMaterial } from "three";

import { TopologicSceneGraph } from "../react/index.tsx";
import {
	ensureTopologicWasmLoaded,
	parseTopologicFixtureV1,
	type TopologicEntity,
	type TopologicFixtureV1,
} from "../wasm/index.ts";

//#region 🔖Core
export interface Point3D {
	readonly x: number;
	readonly y: number;
	readonly z: number;
}

export interface IPreviewMesh {
	setPosition(p: Point3D): void;
	setScale(x: number, y: number, z: number): void;
	setVisible(visible: boolean): void;
	dispose(): void;
}

export interface IGraphicsEngine {
	createCylinderPreview(color: string, opacity: number): IPreviewMesh;
}

export interface ICADEngine {
	createCylinder(center: Point3D, radius: number, height: number): unknown;
	createBox(min: Point3D, max: Point3D): unknown;
	createSphere(center: Point3D, radius: number): unknown;
}

export interface IHostContext {
	readonly graphics: IGraphicsEngine;
	readonly cad: ICADEngine;
	readonly commitGeometry: (geometryPayload: unknown) => void;
}

export type CadEventKind = "commandActivated" | "commandDeactivated" | "geometryCommitted";

export interface CadEngineEvent {
	readonly kind: CadEventKind;
	readonly commandId: string | null;
	readonly payload?: unknown;
}

export function distancePoint3D(a: Point3D, b: Point3D): number {
	const dx = a.x - b.x;
	const dy = a.y - b.y;
	const dz = a.z - b.z;
	return Math.sqrt(dx * dx + dy * dy + dz * dz);
}
//#endregion 🔖Core

//#region 🔖Commands
export interface ICommand {
	readonly id: string;
	activate(context: IHostContext): void;
	deactivate(): void;
	onPointerMove(point: Point3D): void;
	onPointerClick(point: Point3D): void;
}

export abstract class BaseCommand implements ICommand {
	abstract readonly id: string;
	protected context: IHostContext | null = null;

	activate(context: IHostContext): void {
		this.context = context;
	}

	deactivate(): void {
		this.context = null;
	}

	abstract onPointerMove(point: Point3D): void;
	abstract onPointerClick(point: Point3D): void;
}
//#endregion 🔖Commands

//#region 🔖Plugins
export interface ICadPlugin {
	readonly id: string;
	readonly commands: readonly ICommand[];
	readonly activate?: (host: IHostContext) => void;
	readonly deactivate?: () => void;
}

enum CylinderStep {
	GetCenter = 0,
	GetRadius = 1,
	GetHeight = 2,
}

export class CylinderCommand extends BaseCommand {
	readonly id = "elements.geometry.cad.primitives.cylinder";

	private preview: IPreviewMesh | null = null;
	private step = CylinderStep.GetCenter;
	private centerPt: Point3D | null = null;
	private radius = 0;

	override activate(context: IHostContext): void {
		super.activate(context);
		this.preview = context.graphics.createCylinderPreview("#ef4444", 0.45);
		this.preview.setVisible(false);
		this.step = CylinderStep.GetCenter;
		this.centerPt = null;
		this.radius = 0;
	}

	override deactivate(): void {
		this.preview?.dispose();
		this.preview = null;
		this.centerPt = null;
		super.deactivate();
	}

	onPointerMove(point: Point3D): void {
		if (!this.preview || !this.centerPt) return;
		if (this.step === CylinderStep.GetRadius) {
			this.radius = distancePoint3D(this.centerPt, point);
			this.preview.setVisible(true);
			this.preview.setPosition(this.centerPt);
			this.preview.setScale(this.radius, 0.01, this.radius);
		} else if (this.step === CylinderStep.GetHeight) {
			const height = point.y - this.centerPt.y;
			this.preview.setPosition({ x: this.centerPt.x, y: this.centerPt.y + height / 2, z: this.centerPt.z });
			this.preview.setScale(this.radius, Math.abs(height) || 0.001, this.radius);
		}
	}

	onPointerClick(point: Point3D): void {
		if (this.step === CylinderStep.GetCenter) {
			this.centerPt = { x: point.x, y: point.y, z: point.z };
			this.step = CylinderStep.GetRadius;
		} else if (this.step === CylinderStep.GetRadius && this.centerPt) {
			this.radius = distancePoint3D(this.centerPt, point);
			this.step = CylinderStep.GetHeight;
		} else if (this.step === CylinderStep.GetHeight && this.centerPt && this.context) {
			const height = point.y - this.centerPt.y;
			this.context.commitGeometry(this.context.cad.createCylinder(this.centerPt, this.radius, height));
		}
	}
}

enum BoxStep {
	GetMin = 0,
	GetMax = 1,
}

export class BoxCommand extends BaseCommand {
	readonly id = "elements.geometry.cad.primitives.box";

	private preview: IPreviewMesh | null = null;
	private step = BoxStep.GetMin;
	private minPt: Point3D | null = null;

	override activate(context: IHostContext): void {
		super.activate(context);
		this.preview = context.graphics.createCylinderPreview("#3b82f6", 0.25);
		this.preview.setVisible(false);
		this.step = BoxStep.GetMin;
		this.minPt = null;
	}

	override deactivate(): void {
		this.preview?.dispose();
		this.preview = null;
		this.minPt = null;
		super.deactivate();
	}

	onPointerMove(point: Point3D): void {
		if (!this.preview || !this.minPt || this.step !== BoxStep.GetMax) return;
		const minX = Math.min(this.minPt.x, point.x);
		const maxX = Math.max(this.minPt.x, point.x);
		const minY = Math.min(this.minPt.y, point.y);
		const maxY = Math.max(this.minPt.y, point.y);
		const minZ = Math.min(this.minPt.z, point.z);
		const maxZ = Math.max(this.minPt.z, point.z);
		this.preview.setVisible(true);
		this.preview.setPosition({ x: (minX + maxX) / 2, y: (minY + maxY) / 2, z: (minZ + maxZ) / 2 });
		this.preview.setScale(Math.max(maxX - minX, 0.001) / 2, Math.max(maxY - minY, 0.001) / 2, Math.max(maxZ - minZ, 0.001) / 2);
	}

	onPointerClick(point: Point3D): void {
		if (this.step === BoxStep.GetMin) {
			this.minPt = { x: point.x, y: point.y, z: point.z };
			this.step = BoxStep.GetMax;
		} else if (this.step === BoxStep.GetMax && this.minPt && this.context) {
			const minX = Math.min(this.minPt.x, point.x);
			const maxX = Math.max(this.minPt.x, point.x);
			const minY = Math.min(this.minPt.y, point.y);
			const maxY = Math.max(this.minPt.y, point.y);
			const minZ = Math.min(this.minPt.z, point.z);
			const maxZ = Math.max(this.minPt.z, point.z);
			this.context.commitGeometry(this.context.cad.createBox({ x: minX, y: minY, z: minZ }, { x: maxX, y: maxY, z: maxZ }));
		}
	}
}

enum SphereStep {
	GetCenter = 0,
	GetRadius = 1,
}

export class SphereCommand extends BaseCommand {
	readonly id = "elements.geometry.cad.primitives.sphere";

	private preview: IPreviewMesh | null = null;
	private step = SphereStep.GetCenter;
	private centerPt: Point3D | null = null;

	override activate(context: IHostContext): void {
		super.activate(context);
		this.preview = context.graphics.createCylinderPreview("#a855f7", 0.35);
		this.preview.setVisible(false);
		this.step = SphereStep.GetCenter;
		this.centerPt = null;
	}

	override deactivate(): void {
		this.preview?.dispose();
		this.preview = null;
		this.centerPt = null;
		super.deactivate();
	}

	onPointerMove(point: Point3D): void {
		if (!this.preview || !this.centerPt || this.step !== SphereStep.GetRadius) return;
		const r = distancePoint3D(this.centerPt, point);
		this.preview.setVisible(true);
		this.preview.setPosition(this.centerPt);
		this.preview.setScale(r, r, r);
	}

	onPointerClick(point: Point3D): void {
		if (this.step === SphereStep.GetCenter) {
			this.centerPt = { x: point.x, y: point.y, z: point.z };
			this.step = SphereStep.GetRadius;
		} else if (this.step === SphereStep.GetRadius && this.centerPt && this.context) {
			const r = distancePoint3D(this.centerPt, point);
			this.context.commitGeometry(this.context.cad.createSphere(this.centerPt, r));
		}
	}
}

export function createPrimitivesPlugin(): ICadPlugin {
	return {
		id: "elements.geometry.cad.primitives",
		commands: [new CylinderCommand(), new BoxCommand(), new SphereCommand()],
	};
}

export const primitivesPlugin: ICadPlugin = createPrimitivesPlugin();
//#endregion 🔖Plugins

//#region 🔖Registry
export class CadEngine {
	private readonly plugins = new Map<string, ICadPlugin>();
	private readonly commands = new Map<string, ICommand>();
	private host: IHostContext | null = null;
	private activeCommand: ICommand | null = null;
	private activeCommandId: string | null = null;
	private readonly listeners = new Set<(event: CadEngineEvent) => void>();

	subscribe(listener: (event: CadEngineEvent) => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private emit(event: CadEngineEvent): void {
		for (const listener of this.listeners) listener(event);
	}

	setHostContext(host: IHostContext): void {
		const wrapped: IHostContext = {
			graphics: host.graphics,
			cad: host.cad,
			commitGeometry: (payload) => {
				this.emit({ kind: "geometryCommitted", commandId: this.activeCommandId, payload });
				host.commitGeometry(payload);
				this.deactivateCommand();
			},
		};
		this.host = wrapped;
		for (const plugin of this.plugins.values()) {
			plugin.activate?.(wrapped);
		}
	}

	clearHostContext(): void {
		this.deactivateCommand();
		for (const plugin of this.plugins.values()) {
			plugin.deactivate?.();
		}
		this.host = null;
	}

	getHostContext(): IHostContext | null {
		return this.host;
	}

	registerPlugin(plugin: ICadPlugin): void {
		if (this.plugins.has(plugin.id)) this.unregisterPlugin(plugin.id);
		this.plugins.set(plugin.id, plugin);
		for (const command of plugin.commands) {
			this.commands.set(command.id, command);
		}
		if (this.host) plugin.activate?.(this.host);
	}

	unregisterPlugin(pluginId: string): void {
		const plugin = this.plugins.get(pluginId);
		if (!plugin) return;
		for (const command of plugin.commands) {
			if (this.activeCommand === command) this.deactivateCommand();
			this.commands.delete(command.id);
		}
		plugin.deactivate?.();
		this.plugins.delete(pluginId);
	}

	listCommands(): readonly { id: string; pluginId: string }[] {
		const out: { id: string; pluginId: string }[] = [];
		for (const plugin of this.plugins.values()) {
			for (const command of plugin.commands) {
				out.push({ id: command.id, pluginId: plugin.id });
			}
		}
		return out;
	}

	getActiveCommandId(): string | null {
		return this.activeCommandId;
	}

	activateCommand(commandId: string): void {
		const next = this.commands.get(commandId);
		if (!next) throw new Error(`Unknown CAD command: ${commandId}`);
		if (this.activeCommand === next) return;
		this.deactivateCommand();
		this.activeCommand = next;
		this.activeCommandId = commandId;
		next.activate(this.getHostOrThrow());
		this.emit({ kind: "commandActivated", commandId });
	}

	deactivateCommand(): void {
		if (this.activeCommand) {
			this.activeCommand.deactivate();
			this.emit({ kind: "commandDeactivated", commandId: this.activeCommandId });
		}
		this.activeCommand = null;
		this.activeCommandId = null;
	}

	onPointerMove(point: Point3D): void {
		this.activeCommand?.onPointerMove(point);
	}

	onPointerClick(point: Point3D): void {
		this.activeCommand?.onPointerClick(point);
	}

	private getHostOrThrow(): IHostContext {
		if (!this.host) throw new Error("CAD host context is not ready");
		return this.host;
	}
}
//#endregion 🔖Registry

//#region 🔖TopologicAdapter

const SCHEMA = "elements.geometry.topologic.fixture/v1" as const;

function surfaceFace(id: string, label: string, vertices: readonly [number, number, number][], triangles: readonly number[]): TopologicEntity {
	return {
		id,
		kind: "face",
		label,
		wires: [],
		surface: { vertices, triangles },
	};
}

function buildCylinderFixture(center: Point3D, radius: number, height: number, suffix: string): unknown {
	const h = Math.abs(height) || 1e-6;
	const y0 = center.y - h / 2;
	const y1 = center.y + h / 2;
	const n = 20;
	const topologies: TopologicEntity[] = [];
	const faceIds: string[] = [];
	const p = suffix;
	const pushFace = (id: string, label: string, verts: [number, number, number][], tri: number[]) => {
		const fe = surfaceFace(`${p}${id}`, label, verts, tri);
		topologies.push(fe);
		faceIds.push(fe.id);
	};
	for (let i = 0; i < n; i += 1) {
		const a0 = (Math.PI * 2 * i) / n;
		const a1 = (Math.PI * 2 * (i + 1)) / n;
		const x0 = center.x + radius * Math.cos(a0);
		const z0 = center.z + radius * Math.sin(a0);
		const x1 = center.x + radius * Math.cos(a1);
		const z1 = center.z + radius * Math.sin(a1);
		const verts: [number, number, number][] = [
			[x0, y0, z0],
			[x1, y0, z1],
			[x1, y1, z1],
			[x0, y1, z0],
		];
		pushFace(`side.${i}`, `Side ${i}`, verts, [0, 1, 2, 0, 2, 3]);
	}
	const bottomRing: [number, number, number][] = [];
	const topRing: [number, number, number][] = [];
	for (let i = 0; i < n; i += 1) {
		const a = (Math.PI * 2 * i) / n;
		bottomRing.push([center.x + radius * Math.cos(a), y0, center.z + radius * Math.sin(a)]);
		topRing.push([center.x + radius * Math.cos(a), y1, center.z + radius * Math.sin(a)]);
	}
	const bottomCenter: [number, number, number] = [center.x, y0, center.z];
	const topCenter: [number, number, number] = [center.x, y1, center.z];
	const bottomVerts = [bottomCenter, ...bottomRing];
	const topVerts = [topCenter, ...topRing];
	const bottomTri: number[] = [];
	const topTri: number[] = [];
	for (let i = 0; i < n; i += 1) {
		const j = (i + 1) % n;
		bottomTri.push(0, i + 1, j + 1);
		topTri.push(0, j + 1, i + 1);
	}
	pushFace("cap.bottom", "Bottom", bottomVerts, bottomTri);
	pushFace("cap.top", "Top", topVerts, topTri);
	const shellId = `${p}shell`;
	const cellId = `${p}cell`;
	const rootId = `${p}root`;
	topologies.push({ id: shellId, kind: "shell", label: "Cylinder shell", faces: faceIds });
	topologies.push({ id: cellId, kind: "cell", label: "Cylinder cell", shells: [shellId] });
	topologies.push({ id: rootId, kind: "topology", label: "CAD primitive", members: [cellId] });
	return {
		schema: SCHEMA,
		label: "CAD cylinder",
		roots: [rootId],
		topologies,
	};
}

function buildBoxFixture(min: Point3D, max: Point3D, suffix: string): unknown {
	const p = suffix;
	const x0 = Math.min(min.x, max.x);
	const x1 = Math.max(min.x, max.x);
	const y0 = Math.min(min.y, max.y);
	const y1 = Math.max(min.y, max.y);
	const z0 = Math.min(min.z, max.z);
	const z1 = Math.max(min.z, max.z);
	const faces: { id: string; label: string; v: [number, number, number][]; t: number[] }[] = [
		{ id: "f0", label: "Z-", v: [[x0, y0, z0], [x1, y0, z0], [x1, y1, z0], [x0, y1, z0]], t: [0, 1, 2, 0, 2, 3] },
		{ id: "f1", label: "Z+", v: [[x0, y0, z1], [x0, y1, z1], [x1, y1, z1], [x1, y0, z1]], t: [0, 1, 2, 0, 2, 3] },
		{ id: "f2", label: "Y-", v: [[x0, y0, z0], [x0, y0, z1], [x1, y0, z1], [x1, y0, z0]], t: [0, 1, 2, 0, 2, 3] },
		{ id: "f3", label: "Y+", v: [[x0, y1, z0], [x1, y1, z0], [x1, y1, z1], [x0, y1, z1]], t: [0, 1, 2, 0, 2, 3] },
		{ id: "f4", label: "X-", v: [[x0, y0, z0], [x0, y1, z0], [x0, y1, z1], [x0, y0, z1]], t: [0, 1, 2, 0, 2, 3] },
		{ id: "f5", label: "X+", v: [[x1, y0, z0], [x1, y0, z1], [x1, y1, z1], [x1, y1, z0]], t: [0, 1, 2, 0, 2, 3] },
	];
	const topologies: TopologicEntity[] = [];
	const faceIds: string[] = [];
	for (const f of faces) {
		const fe = surfaceFace(`${p}${f.id}`, f.label, f.v, f.t);
		topologies.push(fe);
		faceIds.push(fe.id);
	}
	const shellId = `${p}shell`;
	const cellId = `${p}cell`;
	const rootId = `${p}root`;
	topologies.push({ id: shellId, kind: "shell", label: "Box shell", faces: faceIds });
	topologies.push({ id: cellId, kind: "cell", label: "Box cell", shells: [shellId] });
	topologies.push({ id: rootId, kind: "topology", label: "CAD primitive", members: [cellId] });
	return { schema: SCHEMA, label: "CAD box", roots: [rootId], topologies };
}

function buildSphereFixture(center: Point3D, radius: number, suffix: string): unknown {
	const p = suffix;
	const lat = 10;
	const lon = 16;
	const verts: [number, number, number][] = [];
	for (let iy = 0; iy <= lat; iy += 1) {
		const v = iy / lat;
		const phi = v * Math.PI;
		for (let ix = 0; ix <= lon; ix += 1) {
			const u = ix / lon;
			const theta = u * Math.PI * 2;
			const sinP = Math.sin(phi);
			verts.push([center.x + radius * sinP * Math.cos(theta), center.y + radius * Math.cos(phi), center.z + radius * sinP * Math.sin(theta)]);
		}
	}
	const topologies: TopologicEntity[] = [];
	const faceIds: string[] = [];
	let fi = 0;
	for (let iy = 0; iy < lat; iy += 1) {
		for (let ix = 0; ix < lon; ix += 1) {
			const i0 = iy * (lon + 1) + ix;
			const i1 = i0 + 1;
			const i2 = i0 + lon + 2;
			const i3 = i0 + lon + 1;
			const quad = [verts[i0]!, verts[i1]!, verts[i2]!, verts[i3]!];
			const fe = surfaceFace(`${p}sphere.f.${fi}`, `Sphere ${fi}`, quad, [0, 1, 2, 0, 2, 3]);
			topologies.push(fe);
			faceIds.push(fe.id);
			fi += 1;
		}
	}
	const shellId = `${p}shell`;
	const cellId = `${p}cell`;
	const rootId = `${p}root`;
	topologies.push({ id: shellId, kind: "shell", label: "Sphere shell", faces: faceIds });
	topologies.push({ id: cellId, kind: "cell", label: "Sphere cell", shells: [shellId] });
	topologies.push({ id: rootId, kind: "topology", label: "CAD primitive", members: [cellId] });
	return { schema: SCHEMA, label: "CAD sphere", roots: [rootId], topologies };
}

let fixtureSerial = 0;

export class TopologicCadEngine implements ICADEngine {
	createCylinder(center: Point3D, radius: number, height: number): unknown {
		const suffix = `cad${fixtureSerial++}_`;
		const raw = buildCylinderFixture(center, radius, height, suffix);
		return parseTopologicFixtureV1(raw);
	}

	createBox(min: Point3D, max: Point3D): unknown {
		const suffix = `cad${fixtureSerial++}_`;
		return parseTopologicFixtureV1(buildBoxFixture(min, max, suffix));
	}

	createSphere(center: Point3D, radius: number): unknown {
		const suffix = `cad${fixtureSerial++}_`;
		return parseTopologicFixtureV1(buildSphereFixture(center, Math.max(radius, 1e-6), suffix));
	}
}

export function mergeCadFixtures(base: TopologicFixtureV1, piece: TopologicFixtureV1, idPrefix: string): TopologicFixtureV1 {
	const remap = new Map<string, string>();
	for (const entity of piece.topologies) {
		remap.set(entity.id, `${idPrefix}${entity.id}`);
	}
	const remapId = (id: string) => remap.get(id) ?? `${idPrefix}${id}`;
	const mapEntity = (entity: TopologicEntity): TopologicEntity => {
		const id = remapId(entity.id);
		switch (entity.kind) {
			case "topology":
				return { ...entity, id, members: entity.members.map(remapId) };
			case "vertex":
				return { ...entity, id };
			case "edge":
				return { ...entity, id, vertices: [remapId(entity.vertices[0]), remapId(entity.vertices[1])] as [string, string] };
			case "wire":
				return { ...entity, id, edges: entity.edges.map(remapId) };
			case "face":
				return { ...entity, id, wires: entity.wires.map(remapId) };
			case "shell":
				return { ...entity, id, faces: entity.faces.map(remapId) };
			case "cell":
				return { ...entity, id, shells: entity.shells.map(remapId) };
			case "cellComplex":
				return { ...entity, id, cells: entity.cells.map(remapId) };
			case "cluster":
				return { ...entity, id, topologies: entity.topologies.map(remapId) };
			default:
				return { ...entity, id };
		}
	};
	const newTopologies = piece.topologies.map(mapEntity);
	const newRoots = piece.roots.map(remapId);
	return {
		...base,
		roots: [...base.roots, ...newRoots],
		topologies: [...base.topologies, ...newTopologies],
	};
}
//#endregion 🔖TopologicAdapter

//#region 🔖R3FAdapter

export class ThreePreviewMesh implements IPreviewMesh {
	constructor(
		private readonly mesh: Mesh,
		private readonly layer: Group,
	) {
		this.layer.add(this.mesh);
	}

	setPosition(p: Point3D): void {
		this.mesh.position.set(p.x, p.y, p.z);
	}

	setScale(x: number, y: number, z: number): void {
		this.mesh.scale.set(x, y, z);
	}

	setVisible(visible: boolean): void {
		this.mesh.visible = visible;
	}

	dispose(): void {
		this.layer.remove(this.mesh);
		this.mesh.geometry.dispose();
		(this.mesh.material as MeshBasicMaterial).dispose();
	}
}

export class ThreeGraphicsEngine implements IGraphicsEngine {
	constructor(private readonly previewLayer: Group) {}

	createCylinderPreview(color: string, opacity: number): IPreviewMesh {
		const geo = new CylinderGeometry(1, 1, 1, 32);
		const mat = new MeshBasicMaterial({ color, wireframe: true, transparent: true, opacity });
		const mesh = new Mesh(geo, mat);
		return new ThreePreviewMesh(mesh, this.previewLayer);
	}
}
//#endregion 🔖R3FAdapter

//#region 🔖CanvasUI

const CadEngineContext = createContext<CadEngine | null>(null);

export function useCadEngine(): CadEngine {
	const engine = useContext(CadEngineContext);
	if (!engine) throw new Error("CadEngineContext missing");
	return engine;
}

export function useActiveCommand(): string | null {
	const engine = useCadEngine();
	const [id, setId] = useState(engine.getActiveCommandId());
	useEffect(() => {
		const unsub = engine.subscribe((event) => {
			if (event.kind === "commandActivated" || event.kind === "commandDeactivated") setId(engine.getActiveCommandId());
		});
		return unsub;
	}, [engine]);
	return id;
}

export function useCadCommit(onCommit: (payload: unknown) => void): void {
	const engine = useCadEngine();
	useEffect(() => {
		const unsub = engine.subscribe((event) => {
			if (event.kind === "geometryCommitted" && event.payload !== undefined) onCommit(event.payload);
		});
		return unsub;
	}, [engine, onCommit]);
}

export function useRegisterPlugin(plugin: ICadPlugin): void {
	const engine = useCadEngine();
	useEffect(() => {
		engine.registerPlugin(plugin);
		return () => engine.unregisterPlugin(plugin.id);
	}, [engine, plugin]);
}

function CadPointerSurface(props: { engine: CadEngine }): ReactElement {
	return (
		<mesh
			rotation={[-Math.PI / 2, 0, 0]}
			position={[0, 0, 0]}
			renderOrder={999}
			onPointerMove={(event) => {
				event.stopPropagation();
				const p = event.point;
				props.engine.onPointerMove({ x: p.x, y: p.y, z: p.z });
			}}
			onPointerDown={(event) => {
				if (event.button !== 0) return;
				event.stopPropagation();
				const p = event.point;
				props.engine.onPointerClick({ x: p.x, y: p.y, z: p.z });
			}}
		>
			<planeGeometry args={[500, 500]} />
			<meshBasicMaterial visible={false} side={DoubleSide} depthWrite={false} />
		</mesh>
	);
}

function CadRuntime(props: { engine: CadEngine; onHostReady: (ready: boolean) => void }): ReactElement {
	const previewRoot = useRef<Group>(null);
	const { engine, onHostReady } = props;

	useLayoutEffect(() => {
		const root = previewRoot.current;
		if (!root) return;
		let cancelled = false;
		void ensureTopologicWasmLoaded().then(() => {
			if (cancelled) return;
			const graphics = new ThreeGraphicsEngine(root);
			const cad = new TopologicCadEngine();
			engine.setHostContext({
				graphics,
				cad,
				commitGeometry: () => {},
			});
			onHostReady(true);
		});
		return () => {
			cancelled = true;
			engine.clearHostContext();
			onHostReady(false);
		};
	}, [engine, onHostReady]);

	return <group ref={previewRoot} />;
}

export interface CadCanvasProps {
	readonly engine?: CadEngine;
	readonly plugins?: readonly ICadPlugin[];
	readonly className?: string;
	readonly children?: ReactNode;
}

export function CadCanvas(props: CadCanvasProps): ReactElement {
	const engine = useMemo(() => props.engine ?? new CadEngine(), [props.engine]);
	const pluginsList = useMemo(() => props.plugins ?? [primitivesPlugin], [props.plugins]);
	const [fixture, setFixture] = useState<TopologicFixtureV1 | null>(null);
	const mergeCounter = useRef(0);
	const [hostReady, setHostReady] = useState(false);

	useEffect(() => {
		for (const plugin of pluginsList) {
			engine.registerPlugin(plugin);
		}
		return () => {
			for (const plugin of pluginsList) {
				engine.unregisterPlugin(plugin.id);
			}
		};
	}, [engine, pluginsList]);

	useEffect(() => {
		const unsub = engine.subscribe((event) => {
			if (event.kind !== "geometryCommitted") return;
			const parsed = event.payload as TopologicFixtureV1 | null;
			if (!parsed || parsed.schema !== SCHEMA) return;
			setFixture((current) => {
				if (!current) return parsed;
				const prefix = `m${mergeCounter.current++}_`;
				return mergeCadFixtures(current, parsed, prefix);
			});
		});
		return unsub;
	}, [engine]);

	return (
		<CadEngineContext.Provider value={engine}>
			<div className={props.className ?? "relative h-full w-full"}>
				<Canvas className="h-full w-full" camera={{ position: [12, 10, 12], fov: 50 }}>
					<color attach="background" args={["#0f172a"]} />
					{!fixture ? (
						<>
							<ambientLight intensity={0.6} />
							<directionalLight position={[8, 14, 6]} intensity={0.9} />
							<OrbitControls makeDefault />
						</>
					) : null}
					<CadRuntime engine={engine} onHostReady={setHostReady} />
					{fixture ? <TopologicSceneGraph fixture={fixture} /> : null}
					{hostReady ? <CadPointerSurface engine={engine} /> : null}
					{props.children}
				</Canvas>
			</div>
		</CadEngineContext.Provider>
	);
}
//#endregion 🔖CanvasUI

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("cad engine", () => {
		it("keeps pure regions free of three, r3f, and wasm import paths", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const path = join(dirname(fileURLToPath(import.meta.url)), "index.tsx");
			const source = readFileSync(path, "utf8");
			const start = source.indexOf("//#region 🔖Core");
			const end = source.indexOf("//#region 🔖TopologicAdapter");
			expect(start).toBeGreaterThanOrEqual(0);
			expect(end).toBeGreaterThan(start);
			const head = source.slice(start, end);
			expect(head).not.toMatch(/from ['"]three['"]/);
			expect(head).not.toMatch(/from ['"]@react-three\/fiber['"]/);
			expect(head).not.toMatch(/from ['"]@react-three\/drei['"]/);
			expect(head).not.toMatch(/from ['"]\.\.\/wasm\//);
			expect(head).not.toMatch(/from ['"]\.\.\/react\//);
		});

		it("merges prefixed fixture ids without collisions", () => {
			const base = {
				schema: SCHEMA,
				roots: ["a"],
				topologies: [{ id: "a", kind: "topology" as const, members: ["b"] }, { id: "b", kind: "vertex" as const, point: [0, 0, 0] as const }],
			} as TopologicFixtureV1;
			const piece = {
				schema: SCHEMA,
				roots: ["r"],
				topologies: [{ id: "r", kind: "topology" as const, members: ["v"] }, { id: "v", kind: "vertex" as const, point: [1, 1, 1] as const }],
			} as TopologicFixtureV1;
			const merged = mergeCadFixtures(base, piece, "p_");
			expect(merged.roots).toEqual(["a", "p_r"]);
			expect(merged.topologies).toHaveLength(4);
			const root = merged.topologies.find((e) => e.id === "p_r");
			expect(root?.kind).toBe("topology");
			if (root?.kind === "topology") expect(root.members).toEqual(["p_v"]);
		});

		it("runs cylinder command against mock adapters and commits once", () => {
			const commits: unknown[] = [];
			const graphics: IGraphicsEngine = {
				createCylinderPreview: () => ({
					setPosition: () => {},
					setScale: () => {},
					setVisible: () => {},
					dispose: () => {},
				}),
			};
			const cad: ICADEngine = {
				createCylinder: () => ({ kind: "mock" }),
				createBox: () => null,
				createSphere: () => null,
			};
			const engine = new CadEngine();
			engine.setHostContext({
				graphics,
				cad,
				commitGeometry: (payload) => {
					commits.push(payload);
				},
			});
			engine.registerPlugin(createPrimitivesPlugin());
			engine.activateCommand("elements.geometry.cad.primitives.cylinder");
			engine.onPointerClick({ x: 0, y: 0, z: 0 });
			engine.onPointerClick({ x: 2, y: 0, z: 0 });
			engine.onPointerClick({ x: 0, y: 3, z: 0 });
			expect(commits).toHaveLength(1);
		});

		it("parses a wasm-built cylinder fixture and builds a render packet", async () => {
			const { buildTopologicRenderPacketV1 } = await import("../wasm/index.ts");
			await ensureTopologicWasmLoaded();
			const engine = new TopologicCadEngine();
			const parsed = engine.createCylinder({ x: 0, y: 1, z: 0 }, 1.5, 2) as TopologicFixtureV1 | null;
			expect(parsed?.schema).toBe(SCHEMA);
			const packet = buildTopologicRenderPacketV1(parsed!);
			expect(packet?.entries?.length ?? 0).toBeGreaterThan(0);
		});
	});
}
//#endregion 🧪Tests
