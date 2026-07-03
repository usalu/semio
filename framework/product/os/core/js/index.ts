// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/framework-os-core` — os CQRS store, programs, resources, media graph, dev JSON backbone. */
// #endregion 🧲Header

import {
	createDocumentVcsEnvelope,
	createDocumentVcsId,
	type DocumentVcsEnvelope,
	DocumentVcsStore,
	materializeDocumentProjection,
	type Author,
	buildHistoryColumns,
	type HistoryColumn,
} from "@semio-tech/vcs-core/internal";
import {
	SRESOURCES_DESCRIPTOR_IDS,
	SRESOURCES_MANIFEST_DOCUMENT,
	type SResourcesDescriptorKindId,
} from "@semio-tech/graph-manifest";
import type { ComponentKind, PlatformDefinition, PluginContext, AppDefinition } from "@semio-tech/framework-platform-core";
import {
	VirtualFileSystemController,
	virtualFileSystemDescriptorValues,
	type VirtualFileSystemNodeRecord,
	type VirtualFileSystemSchemaModel,
	type VirtualFileSystemScope,
} from "@semio-tech/framework-platform-core";
import type { CommandBus } from "@semio-tech/framework-core";

//#region 🔖Schemas
export const OS_STUDIO_SCHEMA = "s.studio" as const;
export const OS_MEDIA_GRAPH_SCHEMA = "s.media-graph" as const;

export type OsResourceKindId = SResourcesDescriptorKindId;
export { SRESOURCES_DESCRIPTOR_IDS as OS_RESOURCE_KIND_IDS };

export interface OsBackboneRef {
	readonly kind: "dev" | "local" | "remote";
	readonly uri: string;
}

export interface OsConflict {
	readonly kind: "os-conflict";
	readonly uri: string;
	readonly message: string;
	readonly localRevision?: string;
	readonly remoteRevision?: string;
}

export interface OsSourceDocument {
	readonly format: string;
	readonly vcsJson?: string;
	readonly inline?: string;
	readonly payloadRef?: string;
}

export interface OsAppInstance {
	readonly id: string;
	readonly programId: string;
	readonly appId: string;
	readonly label: string;
	readonly yields: OsResourceKindId;
	readonly sourceDocument: OsSourceDocument;
}

export interface OsMediaPort {
	readonly id: string;
	readonly resourceKind: OsResourceKindId;
	readonly direction: "in" | "out";
}

export interface OsPortSpec {
	readonly id: string;
	readonly label: string;
	readonly resourceKind: OsResourceKindId;
	readonly required?: boolean;
}

export interface OsMediaGraphNode {
	readonly id: string;
	readonly instanceId: string;
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly inputs: readonly OsMediaPort[];
	readonly outputs: readonly OsMediaPort[];
}

export interface OsMediaGraphEdge {
	readonly id: string;
	readonly sourceNodeId: string;
	readonly sourcePortId: string;
	readonly targetNodeId: string;
	readonly targetPortId: string;
}

export interface OsMediaGraph {
	readonly schema: typeof OS_MEDIA_GRAPH_SCHEMA;
	readonly nodes: readonly OsMediaGraphNode[];
	readonly edges: readonly OsMediaGraphEdge[];
}

export type OsParameterType = "numeric" | "categorical" | "toggle" | "text";

export interface OsParameterFieldSpec {
	readonly fieldPath: string;
	readonly label: string;
	readonly type: OsParameterType;
}

export interface OsParameterBase {
	readonly id: string;
	readonly name: string;
}

export interface OsNumericParameter extends OsParameterBase {
	readonly type: "numeric";
	readonly value: number;
	readonly min?: number;
	readonly max?: number;
	readonly step?: number;
}

export interface OsCategoricalParameter extends OsParameterBase {
	readonly type: "categorical";
	readonly value: string;
	readonly options: readonly string[];
}

export interface OsToggleParameter extends OsParameterBase {
	readonly type: "toggle";
	readonly value: boolean;
}

export interface OsTextParameter extends OsParameterBase {
	readonly type: "text";
	readonly value: string;
}

export type OsParameter = OsNumericParameter | OsCategoricalParameter | OsToggleParameter | OsTextParameter;

export interface OsParameterFieldBinding {
	readonly parameterId: string;
	readonly instanceId: string;
	readonly fieldPath: string;
}

export interface OsProjection {
	readonly programs: readonly string[];
	readonly activeProgramId: string | null;
	readonly activeAlternativeId: string | null;
	readonly appInstances: readonly OsAppInstance[];
	readonly mediaGraph: OsMediaGraph;
	readonly parameters: readonly OsParameter[];
	readonly parameterBindings: readonly OsParameterFieldBinding[];
}

export interface OsOperation {
	readonly op:
		| "spawnAppInstance"
		| "removeAppInstance"
		| "connectMediaPorts"
		| "disconnectMediaEdge"
		| "moveMediaNode"
		| "patchAppInstance"
		| "patchAppSource"
		| "applyAppOperation"
		| "setStudioName"
		| "setActiveProgram"
		| "createAlternative"
		| "switchAlternative"
		| "addParameter"
		| "removeParameter"
		| "patchParameter"
		| "bindParameterField"
		| "unbindParameterField"
		| "syncParameterPorts";
	readonly payload: Record<string, unknown>;
}

export interface OsChange {
	readonly id: string;
	readonly forwards: readonly OsOperation[];
	readonly backwards: readonly OsOperation[];
	readonly description?: string;
	readonly savedAt?: string;
}

export interface OsCheckpoint {
	readonly id: string;
	readonly changeIds: readonly string[];
	readonly parentId?: string;
	readonly authors?: readonly Author[];
	readonly message?: string;
	readonly savedAt: string;
}

export interface OsAlternative {
	readonly id: string;
	readonly name: string;
	readonly checkpointIds: readonly string[];
}

export interface OsVcs {
	readonly initialProjection: OsProjection;
	readonly operations: readonly OsChange[];
	readonly checkpoints: readonly OsCheckpoint[];
	readonly alternatives: readonly OsAlternative[];
}

export interface OsDocument {
	readonly schema: typeof OS_STUDIO_SCHEMA;
	readonly id: string;
	readonly name: string;
	readonly vcs: OsVcs;
	readonly backbone?: OsBackboneRef;
}
//#endregion 🔖Schemas

//#region 🔖ProgramRegistry
export interface OsAppRegistration {
	readonly id: string;
	readonly label: string;
	readonly controllerId: string;
	readonly inputs: readonly OsPortSpec[];
	readonly outputs: readonly OsPortSpec[];
	readonly sourceFormat: string;
	readonly componentKind: ComponentKind;
	readonly defaultModeId?: string;
	readonly parameterFields?: readonly OsParameterFieldSpec[];
}

export function osAppPrimaryOutputKind(registration: Pick<OsAppRegistration, "outputs">): OsResourceKindId {
	return registration.outputs[0]?.resourceKind ?? "graph.dag";
}

export function osOutPort(resourceKind: OsResourceKindId, id = "out", label = "Out"): OsPortSpec {
	return { id, label, resourceKind };
}

export function osInPort(resourceKind: OsResourceKindId, id: string, label: string, required = false): OsPortSpec {
	return { id, label, resourceKind, required };
}

export function mediaPortIdForSpec(instanceId: string, specId: string, direction: "in" | "out"): string {
	return `${instanceId}:${specId}:${direction}`;
}

export function mediaPortSpecId(portId: string): string | null {
	const parts = portId.split(":");
	if (parts.length < 3) return null;
	return parts.slice(1, -1).join(":");
}

export const OS_PARAMETER_PORT_PREFIX = "param." as const;

/** @emoji 🎛️ Builds the media graph input port id for a bound studio parameter. */
export function parameterPortId(instanceId: string, parameterId: string): string {
	return mediaPortIdForSpec(instanceId, `${OS_PARAMETER_PORT_PREFIX}${parameterId}`, "in");
}

/** @emoji 🎛️ Returns whether a media port id denotes a studio parameter input channel. */
export function isParameterPortId(portId: string): boolean {
	const specId = mediaPortSpecId(portId);
	return specId?.startsWith(OS_PARAMETER_PORT_PREFIX) ?? false;
}

/** @emoji 🎛️ Extracts the studio parameter id from a parameter input port id. */
export function parameterIdFromPortId(portId: string): string | null {
	const specId = mediaPortSpecId(portId);
	if (!specId?.startsWith(OS_PARAMETER_PORT_PREFIX)) return null;
	return specId.slice(OS_PARAMETER_PORT_PREFIX.length);
}

/** @emoji 🎛️ Reads the runtime value from a studio parameter definition. */
export function osParameterValue(parameter: OsParameter): unknown {
	return parameter.value;
}

/** @emoji 🎛️ Returns whether a parameter type can drive a bindable field type. */
export function osParameterTypesCompatible(parameterType: OsParameterType, fieldType: OsParameterType): boolean {
	return parameterType === fieldType;
}

/** @emoji 🎛️ Creates a default studio parameter of the given type. */
export function createDefaultOsParameter(type: OsParameterType = "numeric", name = "Parameter", id?: string): OsParameter {
	const parameterId = id ?? `param-${Date.now()}`;
	const base = { id: parameterId, name };
	switch (type) {
		case "numeric":
			return { ...base, type, value: 0, min: 0, max: 100, step: 1 };
		case "categorical":
			return { ...base, type, value: "Option A", options: ["Option A", "Option B"] };
		case "toggle":
			return { ...base, type, value: false };
		case "text":
			return { ...base, type, value: "" };
	}
}

function clampNumericValue(value: number, min?: number, max?: number, step?: number): number {
	let next = value;
	if (Number.isFinite(min)) next = Math.max(next, min!);
	if (Number.isFinite(max)) next = Math.min(next, max!);
	if (Number.isFinite(step) && step! > 0) {
		const anchor = Number.isFinite(min) ? min! : 0;
		next = anchor + Math.round((next - anchor) / step!) * step!;
		if (Number.isFinite(min)) next = Math.max(next, min!);
		if (Number.isFinite(max)) next = Math.min(next, max!);
	}
	return next;
}

/** @emoji 🎛️ Applies a partial patch to a studio parameter, enforcing type constraints. */
export function patchOsParameter(parameter: OsParameter, patch: Record<string, unknown>): OsParameter {
	const name = typeof patch.name === "string" ? patch.name : parameter.name;
	if (patch.type === "numeric" || parameter.type === "numeric" && patch.type === undefined) {
		const current = parameter.type === "numeric" ? parameter : createDefaultOsParameter("numeric", name);
		const min = patch.min !== undefined ? Number(patch.min) : current.min;
		const max = patch.max !== undefined ? Number(patch.max) : current.max;
		const step = patch.step !== undefined ? Number(patch.step) : current.step;
		const rawValue = patch.value !== undefined ? Number(patch.value) : current.value;
		return {
			id: parameter.id,
			name,
			type: "numeric",
			min: Number.isFinite(min) ? min : undefined,
			max: Number.isFinite(max) ? max : undefined,
			step: Number.isFinite(step) ? step : undefined,
			value: clampNumericValue(rawValue, Number.isFinite(min) ? min : undefined, Number.isFinite(max) ? max : undefined, Number.isFinite(step) ? step : undefined),
		};
	}
	if (patch.type === "categorical" || parameter.type === "categorical" && patch.type === undefined) {
		const current = parameter.type === "categorical" ? parameter : createDefaultOsParameter("categorical", name);
		const options = Array.isArray(patch.options) ? patch.options.map((entry) => String(entry)) : [...current.options];
		const uniqueOptions = options.length ? options : ["Option A"];
		const value = typeof patch.value === "string" && uniqueOptions.includes(patch.value) ? patch.value : uniqueOptions.includes(current.value) ? current.value : uniqueOptions[0]!;
		return { id: parameter.id, name, type: "categorical", options: uniqueOptions, value };
	}
	if (patch.type === "toggle" || parameter.type === "toggle" && patch.type === undefined) {
		const current = parameter.type === "toggle" ? parameter : createDefaultOsParameter("toggle", name);
		const value = typeof patch.value === "boolean" ? patch.value : current.value;
		return { id: parameter.id, name, type: "toggle", value };
	}
	const current = parameter.type === "text" ? parameter : createDefaultOsParameter("text", name);
	const value = typeof patch.value === "string" ? patch.value : current.value;
	return { id: parameter.id, name, type: "text", value };
}

function jsonPointerSegments(pointer: string): readonly string[] {
	if (pointer.startsWith("/")) return pointer.slice(1).split("/").filter(Boolean);
	return pointer.split(".").filter(Boolean);
}

/** @emoji 🎛️ Deep-sets a JSON-pointer path on a plain object projection. */
export function setJsonPointerValue(root: Record<string, unknown>, pointer: string, value: unknown): void {
	const segments = jsonPointerSegments(pointer);
	if (!segments.length) return;
	let current: Record<string, unknown> = root;
	for (let index = 0; index < segments.length - 1; index += 1) {
		const segment = segments[index]!;
		const next = current[segment];
		if (typeof next !== "object" || next === null || Array.isArray(next)) current[segment] = {};
		current = current[segment] as Record<string, unknown>;
	}
	current[segments[segments.length - 1]!] = value;
}

/** @emoji 🎛️ Applies bound studio parameter values onto an app projection via JSON pointers. */
export function applyParameterValuesToProjection<T>(
	projection: T,
	bindings: readonly OsParameterFieldBinding[],
	parameters: readonly OsParameter[],
	instanceId: string,
): T {
	const instanceBindings = bindings.filter((binding) => binding.instanceId === instanceId);
	if (!instanceBindings.length) return projection;
	const clone = structuredClone(projection) as Record<string, unknown>;
	for (const binding of instanceBindings) {
		const parameter = parameters.find((entry) => entry.id === binding.parameterId);
		if (!parameter) continue;
		setJsonPointerValue(clone, binding.fieldPath, osParameterValue(parameter));
	}
	return clone as T;
}

function syncMediaNodeParameterPorts(
	node: OsMediaGraphNode,
	bindings: readonly OsParameterFieldBinding[],
): OsMediaGraphNode {
	const instanceBindings = bindings.filter((binding) => binding.instanceId === node.instanceId);
	const baseInputs = node.inputs.filter((port) => !isParameterPortId(port.id));
	const parameterInputs: OsMediaPort[] = instanceBindings.map((binding) => ({
		id: parameterPortId(node.instanceId, binding.parameterId),
		resourceKind: "parameter.value",
		direction: "in",
	}));
	const inputs = [...baseInputs, ...parameterInputs];
	const portCount = Math.max(inputs.length, node.outputs.length, 1);
	return { ...node, inputs, height: 56 + portCount * 18 };
}

function syncMediaGraphParameterPorts(graph: OsMediaGraph, bindings: readonly OsParameterFieldBinding[]): OsMediaGraph {
	return {
		schema: OS_MEDIA_GRAPH_SCHEMA,
		nodes: graph.nodes.map((node) => syncMediaNodeParameterPorts(node, bindings)),
		edges: graph.edges.map((edge) => ({ ...edge })),
	};
}

/** @emoji 🎛️ Resolves bound parameter values for an app instance as a field-path map. */
export function resolveParameterValuesForInstance(
	bindings: readonly OsParameterFieldBinding[],
	parameters: readonly OsParameter[],
	instanceId: string,
): Readonly<Record<string, unknown>> {
	const values: Record<string, unknown> = {};
	for (const binding of bindings.filter((entry) => entry.instanceId === instanceId)) {
		const parameter = parameters.find((entry) => entry.id === binding.parameterId);
		if (!parameter) continue;
		values[binding.fieldPath] = osParameterValue(parameter);
	}
	return values;
}

export interface OsProgramDefinition extends PlatformDefinition {
	readonly apps: readonly (OsAppRegistration & { readonly modes: readonly { readonly id: string; readonly label: string }[] })[];
}

export interface OsResourceDescriptor {
	readonly kind: OsResourceKindId;
	readonly name: string;
	readonly sourceFormat: string;
	readonly componentKind: ComponentKind;
	readonly dimension: string;
}

function descriptorPresentation(kind: OsResourceKindId): OsResourceDescriptor {
	const row = SRESOURCES_MANIFEST_DOCUMENT.descriptorKinds?.find((entry) => entry.id === kind);
	const presentation = (row?.presentation ?? {}) as Record<string, string>;
	return {
		kind,
		name: row?.name ?? kind,
		sourceFormat: presentation.sourceFormat ?? kind,
		componentKind: (presentation.componentKind ?? "panel") as ComponentKind,
		dimension: presentation.dimension ?? "unknown",
	};
}

export function listOsResourceDescriptors(): readonly OsResourceDescriptor[] {
	return SRESOURCES_DESCRIPTOR_IDS.map(descriptorPresentation);
}

export function osResourceDescriptor(kind: OsResourceKindId): OsResourceDescriptor {
	return descriptorPresentation(kind);
}

export function resourcesCompatible(left: OsResourceKindId, right: OsResourceKindId): boolean {
	return left === right;
}

const osBuiltinProgramRegistry: OsProgramDefinition[] = [];

/** @emoji 📚 Registers a built-in os program (e.g. s.system) prepended to {@link listOsPrograms}. */
export function registerOsBuiltinProgram(program: OsProgramDefinition): void {
	if (osBuiltinProgramRegistry.some((entry) => entry.id === program.id)) return;
	osBuiltinProgramRegistry.push(program);
}

export function osBaselineResource(
	resourceKind: OsResourceKindId,
	sourceFormat: string,
	componentKind: ComponentKind,
	modes: readonly { readonly id: string; readonly label: string }[] = [{ id: "edit", label: "Edit" }],
): Omit<OsAppRegistration, "id" | "label" | "controllerId"> & { readonly modes: readonly { readonly id: string; readonly label: string }[] } {
	return { inputs: [], outputs: [osOutPort(resourceKind)], sourceFormat, componentKind, modes };
}

export type OsAppResourceSpec = Omit<OsAppRegistration, "id" | "label" | "controllerId"> & { readonly modes: readonly { readonly id: string; readonly label: string }[] };

const osProgramExtensionRegistry = new Map<string, OsProgramDefinition>();

/** @emoji 📚 Registers a fully materialized os program definition. */
export function registerOsProgramDefinition(program: OsProgramDefinition): void {
	osProgramExtensionRegistry.set(program.id, program);
}

/** @emoji 🧩 Merges a technology {@link PlatformDefinition} into the os program registry with port metadata. */
export function mergeOsProgramDefinition(
	programId: string,
	definition: PlatformDefinition,
	resourceByAppId: Readonly<Record<string, OsAppResourceSpec>>,
): void {
	const resources = resourceByAppId;
	const fallbackResource = Object.values(resources)[0];
	if (!fallbackResource) throw new Error(`mergeOsProgramDefinition requires resourceByAppId for ${programId}`);
	registerOsProgramDefinition({
		id: programId,
		name: definition.name,
		apiVersion: definition.apiVersion ?? "1",
		apps: definition.apps.map((app) => {
			const resource = resources[app.id] ?? fallbackResource;
			return {
				id: app.id,
				label: app.label,
				controllerId: app.controllerId,
				inputs: resource.inputs,
				outputs: resource.outputs,
				sourceFormat: resource.sourceFormat,
				componentKind: resource.componentKind,
				parameterFields: resource.parameterFields,
				modes: app.modes.length > 0 ? app.modes : resource.modes,
				defaultModeId: app.defaultModeId ?? resource.defaultModeId,
			};
		}),
		createPlatformApi: () => ({}),
	});
}

export function osExtensionRegistrySize(): number {
	return osProgramExtensionRegistry.size;
}

/** @emoji 🌱 Seeds the extension registry from a resource map for tests and offline tooling. */
export function seedOsProgramRegistryFromResourceMap(resourceByProgram: Readonly<Record<string, Readonly<Record<string, OsAppResourceSpec>>>>): void {
	for (const [programId, resources] of Object.entries(resourceByProgram)) {
		if (osProgramExtensionRegistry.has(programId)) continue;
		mergeOsProgramDefinition(programId, {
			id: programId,
			name: programId
				.split(".")
				.map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
				.join(" "),
			apiVersion: "1",
			apps: Object.entries(resources).map(([appId, resource]) => ({
				id: appId,
				label: appId.charAt(0).toUpperCase() + appId.slice(1),
				controllerId: `${programId.replace(/\./g, "-")}-play`,
				modes: resource.modes,
			})),
			createPlatformApi: () => ({}),
		}, resources);
	}
}

export function listOsPrograms(): readonly OsProgramDefinition[] {
	return [...osBuiltinProgramRegistry, ...osProgramExtensionRegistry.values()];
}

export function osProgramById(programId: string): OsProgramDefinition | undefined {
	return listOsPrograms().find((program) => program.id === programId);
}

export function osAppRegistration(programId: string, appId: string): OsAppRegistration | undefined {
	const program = osProgramById(programId);
	return program?.apps.find((app) => app.id === appId);
}

/** @emoji 🧩 Resolves the {@link AppDefinition} backing an embedded os app instance. */
export function resolveOsAppDefinition(instance: OsAppInstance): AppDefinition | undefined {
	const registration = osAppRegistration(instance.programId, instance.appId);
	if (!registration) return undefined;
	const program = osProgramById(instance.programId);
	const app = program?.apps.find((entry) => entry.id === instance.appId);
	return {
		id: registration.id,
		label: registration.label,
		controllerId: registration.controllerId,
		modes: app?.modes ?? [{ id: "edit", label: "Edit" }],
		defaultModeId: app?.defaultModeId ?? registration.defaultModeId,
	};
}
//#endregion 🔖ProgramRegistry

//#region 🔖AppVcsRegistry
export interface AppMaterializeContext {
	readonly resolveFixtureJson?: (slug: string) => string | null;
	readonly graph?: OsMediaGraph;
	readonly instances?: readonly OsAppInstance[];
	readonly inputBindings?: Readonly<Record<string, unknown>>;
	readonly parameterValues?: Readonly<Record<string, unknown>>;
	readonly outputPortId?: string;
}

export interface AppVcsHandler<TProjection = unknown, TOp = unknown> {
	readonly format: string;
	readonly createEnvelope: (id: string) => DocumentVcsEnvelope<TProjection, TOp>;
	readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	readonly backwardsOp?: (projection: TProjection, operation: TOp) => readonly TOp[];
	readonly serializeEnvelope: (envelope: DocumentVcsEnvelope<TProjection, TOp>) => string;
	readonly deserializeEnvelope: (json: string) => DocumentVcsEnvelope<TProjection, TOp>;
	readonly materializeProjection: (source: OsSourceDocument, context?: AppMaterializeContext) => TProjection;
	readonly applyInputBindings?: (projection: TProjection, inputBindings: Readonly<Record<string, unknown>>, context: AppMaterializeContext) => TProjection;
	readonly projectOutput?: (projection: TProjection, outputPortId: string, context: AppMaterializeContext) => unknown;
}

const appVcsHandlers = new Map<string, AppVcsHandler>();

/** @emoji 📚 Registers a technology document VCS handler for s applyAppOperation dispatch. */
export function registerAppVcsHandler<TProjection, TOp>(handler: AppVcsHandler<TProjection, TOp>): void {
	appVcsHandlers.set(handler.format, handler as AppVcsHandler);
}

export function appVcsHandlerForFormat(format: string): AppVcsHandler | undefined {
	return appVcsHandlers.get(format);
}

/** @emoji 📦 Typed VCS handler factory for technologies without a dedicated core export yet. */
export function createTypedAppVcsHandler<TProjection, TOp>(
	format: string,
	schema: string,
	empty: () => TProjection,
	applyOp: (projection: TProjection, operation: TOp) => TProjection,
	backwardsOp?: (projection: TProjection, operation: TOp) => readonly TOp[],
	hooks?: {
		readonly applyInputBindings?: AppVcsHandler<TProjection, TOp>["applyInputBindings"];
		readonly projectOutput?: AppVcsHandler<TProjection, TOp>["projectOutput"];
	},
): AppVcsHandler<TProjection, TOp> {
	const applyInputBindings = hooks?.applyInputBindings;
	const projectOutput = hooks?.projectOutput;
	return {
		format,
		createEnvelope: (id) => createDocumentVcsEnvelope<TProjection, TOp>(schema, id, empty()),
		applyOp,
		serializeEnvelope: (envelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json) => JSON.parse(json) as DocumentVcsEnvelope<TProjection, TOp>,
		materializeProjection: (source, context) => {
			const mergeInputs = (projection: TProjection) => {
				if (!context?.inputBindings || !applyInputBindings) return projection;
				return applyInputBindings(projection, context.inputBindings, context);
			};
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as DocumentVcsEnvelope<TProjection, TOp>;
				const appliedIds = envelope.vcs.edits.map((edit) => edit.id);
				return mergeInputs(materializeDocumentProjection(envelope, appliedIds, applyOp));
			}
			if (source.inline) return mergeInputs(JSON.parse(source.inline) as TProjection);
			return mergeInputs(empty());
		},
		...(backwardsOp ? { backwardsOp } : {}),
		...(applyInputBindings ? { applyInputBindings } : {}),
		...(projectOutput ? { projectOutput } : {}),
	};
}

type ShootingAsset = { readonly id: string; readonly name: string; readonly url: string; readonly format: "glb" };
type ShootingFixture = {
	readonly schema: "shooting.fixture";
	readonly assets: readonly ShootingAsset[];
	readonly camera: { readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly zoom: number };
	readonly savedCameras: readonly unknown[];
	readonly scene: Record<string, unknown>;
	readonly shots: readonly unknown[];
	readonly activeShotId?: string;
	readonly activeAssetId?: string;
};
type ShootingOp =
	| { readonly op: "addAsset"; readonly asset: ShootingAsset }
	| { readonly op: "removeAsset"; readonly assetId: string }
	| { readonly op: "setActiveAsset"; readonly assetId: string };

function defaultShootingFixture(): ShootingFixture {
	return {
		schema: "shooting.fixture",
		assets: [{ id: "base", name: "Base", url: "/mesh/base.glb", format: "glb" }],
		camera: { position: [420, -420, 320], target: [0, 0, 40], zoom: 1 },
		savedCameras: [],
		scene: {},
		shots: [{ id: "overview-svg", label: "Overview", width: 256, height: 256, format: "svg" }],
		activeShotId: "overview-svg",
		activeAssetId: "base",
	};
}

function applyShootingOp(fixture: ShootingFixture, op: ShootingOp): ShootingFixture {
	switch (op.op) {
		case "addAsset":
			return { ...fixture, assets: [...fixture.assets, op.asset] };
		case "removeAsset":
			return { ...fixture, assets: fixture.assets.filter((asset) => asset.id !== op.assetId) };
		case "setActiveAsset":
			return { ...fixture, activeAssetId: op.assetId };
	}
}

/** @emoji 📸 S app VCS handler for shooting scene documents. */
export function createShootingAppVcsHandler() {
	return createTypedAppVcsHandler<ShootingFixture, ShootingOp>(
		"shooting.scene",
		"shooting.fixture",
		defaultShootingFixture,
		applyShootingOp,
		undefined,
		{
			applyInputBindings: (fixture, inputBindings) => {
				const mesh = inputBindings.mesh as { readonly url?: string } | undefined;
				if (!mesh?.url) return fixture;
				const activeId = fixture.activeAssetId ?? fixture.assets[0]?.id;
				if (!activeId) return fixture;
				return {
					...fixture,
					assets: fixture.assets.map((asset) => (asset.id === activeId ? { ...asset, url: mesh.url! } : asset)),
				};
			},
		},
	);
}

/** @emoji 🌊 S app VCS handler for flow documents. */
export function createFlowDocumentAppVcsHandler() {
	return createTypedAppVcsHandler("flow.document", "flow.document", () => ({ flow: {}, tree: {} }), (doc, op) => {
		if (op.op === "setFlow") return { ...doc, flow: op.flow };
		return { ...doc, tree: op.tree };
	});
}

/** @emoji 🌳 S app VCS handler for DAG documents. */
export function createFlowDagAppVcsHandler() {
	type DagDoc = { readonly nodes: readonly unknown[]; readonly edges: readonly unknown[] };
	type DagOp = { readonly op: "setNodes"; readonly nodes: readonly unknown[] } | { readonly op: "setEdges"; readonly edges: readonly unknown[] };
	return createTypedAppVcsHandler<DagDoc, DagOp>("flow.dag", "flow.dag", () => ({ nodes: [], edges: [] }), (doc, op) => {
		if (op.op === "setNodes") return { ...doc, nodes: op.nodes };
		return { ...doc, edges: op.edges };
	});
}

/** @emoji 📏 S app VCS handler for procedural 2d documents. */
export function createProcedural2dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("procedural.2d", "procedural.2d", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 📐 S app VCS handler for procedural 3d documents. */
export function createProcedural3dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("procedural.3d", "procedural.3d", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 🔺 S app VCS handler for trinity graph documents. */
export function createTrinityGraphAppVcsHandler() {
	type Doc = { readonly nodes: readonly unknown[] };
	type Op = { readonly op: "setNodes"; readonly nodes: readonly unknown[] };
	return createTypedAppVcsHandler<Doc, Op>("trinity.graph", "trinity.graph", () => ({ nodes: [] }), (doc, op) => ({ nodes: op.nodes }));
}

/** @emoji 🗺️ S app VCS handler for GIS map documents. */
export function createGisMapAppVcsHandler() {
	type Doc = { readonly layers: readonly unknown[] };
	type Op = { readonly op: "setLayers"; readonly layers: readonly unknown[] };
	return createTypedAppVcsHandler<Doc, Op>("gis.map", "gis.map", () => ({ layers: [] }), (doc, op) => ({ layers: op.layers }));
}

/** @emoji 📽 S app VCS handler for presentation deck documents. */
export function createPresentationDeckAppVcsHandler() {
	type Tile = { readonly id: string; readonly name: string };
	type Doc = { readonly schema: string; readonly tiles: readonly Tile[] };
	type Op = { readonly op: "addTile"; readonly tile: Tile } | { readonly op: "removeTile"; readonly tileId: string };
	return createTypedAppVcsHandler<Doc, Op>(
		"presentation.deck",
		"presentation.deck",
		() => ({ schema: "presentation.deck", tiles: [] }),
		(doc, op) => {
			if (op.op === "addTile") return { ...doc, tiles: [...doc.tiles, op.tile] };
			return { ...doc, tiles: doc.tiles.filter((tile) => tile.id !== op.tileId) };
		},
	);
}

/** @emoji 🩻 S app VCS handler for puzzle 2d documents. */
export function createPuzzle2dAppVcsHandler() {
	type Doc = { readonly nodes: readonly string[] };
	type Op = { readonly op: "addNode"; readonly nodeId: string } | { readonly op: "removeNode"; readonly nodeId: string };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.2d", "puzzle.2d", () => ({ nodes: [] }), (doc, op) => {
		if (op.op === "addNode") return { ...doc, nodes: [...doc.nodes, op.nodeId] };
		return { ...doc, nodes: doc.nodes.filter((id) => id !== op.nodeId) };
	});
}

/** @emoji 🏙️ S app VCS handler for puzzle 3d documents. */
export function createPuzzle3dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.3d", "puzzle.3d", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 📜 S app VCS handler for sequence documents. */
export function createSequenceAppVcsHandler() {
	type Doc = { readonly schema: string; readonly steps: readonly unknown[]; readonly edges: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"sequence.fixture",
		"sequence.fixture",
		() => ({ schema: "sequence.fixture", steps: [], edges: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}

/** @emoji 📄 S app VCS handler for layout documents. */
export function createLayoutAppVcsHandler() {
	type Doc = { readonly schema: string; readonly pages: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"layout.fixture",
		"layout.fixture",
		() => ({ schema: "layout.fixture", pages: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}

/** @emoji ⚙️ S app VCS handler for imperative documents. */
export function createImperativeAppVcsHandler() {
	type Doc = { readonly schema: string; readonly path: { readonly steps: readonly unknown[] } };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"imperative.document",
		"imperative.document",
		() => ({ schema: "imperative.document", path: { steps: [] } }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}

/** @emoji 🔷 S app VCS handler for lowpoly fixtures. */
export function createLowpolyAppVcsHandler() {
	type Doc = { readonly schema: string; readonly objects: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"lowpoly.fixture",
		"lowpoly.fixture",
		() => ({ schema: "lowpoly.fixture", objects: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}

/** @emoji 🗄️ S app VCS handler for vcs demo documents. */
export function createVcsDemoAppVcsHandler() {
	type Doc = { readonly schema: string; readonly title: string; readonly counter: number };
	type Op = { readonly op: "setDocument"; readonly document: Doc } | { readonly op: "setCounter"; readonly counter: number };
	return createTypedAppVcsHandler<Doc, Op>(
		"vcs.demo",
		"vcs.demo",
		() => ({ schema: "vcs.demo", title: "VCS Demo", counter: 0 }),
		(doc, op) => {
			if (op.op === "setDocument") return op.document;
			return { ...doc, counter: op.counter };
		},
	);
}

/** @emoji 📚 Registers a catalogue.kinds VCS handler backed by a bundle factory. */
export function createCatalogueKindsAppVcsHandler(bundle: () => unknown) {
	return createTypedAppVcsHandler("catalogue.kinds", "catalogue.kinds", bundle, (doc) => doc);
}

/** @emoji 👯 S app VCS handler for puzzle 5d documents. */
export function createPuzzle5dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.5d", "puzzle.5d", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

registerAppVcsHandler(createFlowDocumentAppVcsHandler());
registerAppVcsHandler(createFlowDagAppVcsHandler());
registerAppVcsHandler(createProcedural2dAppVcsHandler());
registerAppVcsHandler(createProcedural3dAppVcsHandler());
registerAppVcsHandler(createShootingAppVcsHandler());
registerAppVcsHandler(createTrinityGraphAppVcsHandler());
registerAppVcsHandler(createGisMapAppVcsHandler());
registerAppVcsHandler(createPresentationDeckAppVcsHandler());
registerAppVcsHandler(createPuzzle2dAppVcsHandler());
registerAppVcsHandler(createPuzzle3dAppVcsHandler());
registerAppVcsHandler(createPuzzle5dAppVcsHandler());
registerAppVcsHandler(
	createTypedAppVcsHandler<
		{ readonly schema: string; readonly id: string; readonly nodes: readonly { readonly id: string; readonly label: string }[] },
		| { readonly op: "addNode"; readonly node: { readonly id: string; readonly label: string } }
		| { readonly op: "removeNode"; readonly nodeId: string }
	>("cad.scene", "cad.scene", () => ({ schema: "cad.scene", id: "cad", nodes: [] }), (doc, op) => {
		if (op.op === "addNode") return { ...doc, nodes: [...doc.nodes, op.node] };
		return { ...doc, nodes: doc.nodes.filter((node) => node.id !== op.nodeId) };
	}),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.design",
		"compose.design",
		() => ({ id: "design" }),
		(doc, op) => ({ id: op.id }),
	),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.type",
		"compose.type",
		() => ({ id: "type" }),
		(doc, op) => ({ id: op.id }),
	),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.kit",
		"compose.kit",
		() => ({ id: "kit" }),
		(doc, op) => ({ id: op.id }),
	),
);

export function resolvePayloadRef(payloadRef: string): string | null {
	if (payloadRef.startsWith("fixture:")) return payloadRef.slice("fixture:".length);
	if (payloadRef.startsWith("upstream:")) return payloadRef.slice("upstream:".length);
	return null;
}

let osFixtureJsonResolver: ((slug: string) => string | null) | null = null;

/** @emoji 📎 Registers bundled fixture JSON lookup for payloadRef materialization. */
export function registerOsFixtureJsonResolver(resolver: (slug: string) => string | null): void {
	osFixtureJsonResolver = resolver;
}

function resolveSourceDocument(source: OsSourceDocument, context?: AppMaterializeContext): OsSourceDocument {
	const resolveFixture = context?.resolveFixtureJson ?? osFixtureJsonResolver ?? undefined;
	if (source.payloadRef?.startsWith("fixture:") && resolveFixture) {
		const slug = resolvePayloadRef(source.payloadRef);
		const json = slug ? resolveFixture(slug) : null;
		if (json) return { ...source, inline: json };
	}
	if (source.payloadRef?.startsWith("upstream:") && context?.instances) {
		const upstreamId = source.payloadRef.slice("upstream:".length);
		const upstream = context.instances.find((entry) => entry.id === upstreamId);
		if (upstream) return upstream.sourceDocument;
	}
	return source;
}

export function createAppSourceDocument(
	format: string,
	instanceId: string,
	options?: { readonly payloadRef?: string; readonly inline?: string },
): OsSourceDocument {
	const handler = appVcsHandlerForFormat(format);
	const envelope = handler?.createEnvelope(instanceId);
	return {
		format,
		vcsJson: envelope ? handler!.serializeEnvelope(envelope) : undefined,
		inline: options?.inline,
		payloadRef: options?.payloadRef,
	};
}

export function materializeAppInstanceProjection(instance: OsAppInstance, context?: AppMaterializeContext): unknown {
	const source = resolveSourceDocument(instance.sourceDocument, context);
	const handler = appVcsHandlerForFormat(source.format);
	if (!handler) {
		if (source.inline) return JSON.parse(source.inline);
		return null;
	}
	let projection = handler.materializeProjection(source, context);
	if (context?.parameterValues && Object.keys(context.parameterValues).length > 0) {
		const clone = structuredClone(projection) as Record<string, unknown>;
		for (const [fieldPath, value] of Object.entries(context.parameterValues)) setJsonPointerValue(clone, fieldPath, value);
		projection = clone;
	}
	if (context?.outputPortId && handler.projectOutput) return handler.projectOutput(projection, context.outputPortId, context);
	return projection;
}

export function applyAppOperationToSource(
	source: OsSourceDocument,
	forwards: readonly unknown[],
	backwards: readonly unknown[],
): OsSourceDocument {
	const handler = appVcsHandlerForFormat(source.format);
	if (!handler) throw new Error(`no app VCS handler for ${source.format}`);
	const envelope = source.vcsJson ? handler.deserializeEnvelope(source.vcsJson) : handler.createEnvelope(createDocumentVcsId("app"));
	const store = new DocumentVcsStore({
		envelope,
		applyOp: handler.applyOp as (projection: unknown, operation: unknown) => unknown,
		backwardsOp: (handler.backwardsOp ?? ((projection, _operation) => [{ op: "setDocument", document: projection }])) as (
			projection: unknown,
			operation: unknown,
		) => readonly unknown[],
		diffOp: (_projection, operation) => operation,
	});
	store.dispatch({ kind: "apply", operations: forwards as never[] });
	return { ...source, vcsJson: handler.serializeEnvelope(store.getEnvelope()) };
}
//#endregion 🔖AppVcsRegistry

//#region 🔖MediaGraphEngine
export interface OsMediaGraphValidation {
	readonly ok: boolean;
	readonly errors: readonly string[];
}

export function emptyMediaGraph(): OsMediaGraph {
	return { schema: OS_MEDIA_GRAPH_SCHEMA, nodes: [], edges: [] };
}

export function validateMediaGraph(graph: OsMediaGraph): OsMediaGraphValidation {
	const errors: string[] = [];
	const nodeById = new Map(graph.nodes.map((node) => [node.id, node]));
	const adjacency = new Map<string, string[]>();
	for (const edge of graph.edges) {
		const source = nodeById.get(edge.sourceNodeId);
		const target = nodeById.get(edge.targetNodeId);
		if (!source) errors.push(`missing source node ${edge.sourceNodeId}`);
		if (!target) errors.push(`missing target node ${edge.targetNodeId}`);
		const outPort = source?.outputs.find((port) => port.id === edge.sourcePortId);
		const inPort = target?.inputs.find((port) => port.id === edge.targetPortId);
		if (outPort && inPort && !resourcesCompatible(outPort.resourceKind, inPort.resourceKind)) {
			errors.push(`incompatible resources ${outPort.resourceKind} → ${inPort.resourceKind}`);
		}
		const next = adjacency.get(edge.sourceNodeId) ?? [];
		next.push(edge.targetNodeId);
		adjacency.set(edge.sourceNodeId, next);
	}
	const visiting = new Set<string>();
	const visited = new Set<string>();
	function dfs(nodeId: string): boolean {
		if (visiting.has(nodeId)) return false;
		if (visited.has(nodeId)) return true;
		visiting.add(nodeId);
		for (const child of adjacency.get(nodeId) ?? []) {
			if (!dfs(child)) return false;
		}
		visiting.delete(nodeId);
		visited.add(nodeId);
		return true;
	}
	for (const node of graph.nodes) {
		if (!dfs(node.id)) errors.push(`cycle detected at ${node.id}`);
	}
	return { ok: errors.length === 0, errors };
}

export function mediaGraphNodeForInstance(instance: OsAppInstance, position: { readonly x: number; readonly y: number }): OsMediaGraphNode {
	const registration = osAppRegistration(instance.programId, instance.appId);
	const inputs = (registration?.inputs ?? []).map((spec) => ({
		id: mediaPortIdForSpec(instance.id, spec.id, "in"),
		resourceKind: spec.resourceKind,
		direction: "in" as const,
	}));
	const outputs = (registration?.outputs ?? [osOutPort("graph.dag")]).map((spec) => ({
		id: mediaPortIdForSpec(instance.id, spec.id, "out"),
		resourceKind: spec.resourceKind,
		direction: "out" as const,
	}));
	const portCount = Math.max(inputs.length, outputs.length, 1);
	return {
		id: `node-${instance.id}`,
		instanceId: instance.id,
		x: position.x,
		y: position.y,
		width: 180,
		height: 56 + portCount * 18,
		inputs,
		outputs,
	};
}

export interface OsMediaInputBinding {
	readonly inputPortId: string;
	readonly inputSpecId: string;
	readonly upstreamInstanceId: string;
	readonly upstreamPortId: string;
	readonly resourceKind: OsResourceKindId;
}

export function resolveInputBindingsForInstance(
	graph: OsMediaGraph,
	instances: readonly OsAppInstance[],
	targetInstanceId: string,
): readonly OsMediaInputBinding[] {
	const node = graph.nodes.find((entry) => entry.instanceId === targetInstanceId);
	if (!node) return [];
	const bindings: OsMediaInputBinding[] = [];
	for (const edge of graph.edges.filter((entry) => entry.targetNodeId === node.id)) {
		const inPort = node.inputs.find((port) => port.id === edge.targetPortId);
		if (!inPort) continue;
		const specId = mediaPortSpecId(inPort.id);
		if (!specId) continue;
		const sourceNode = graph.nodes.find((entry) => entry.id === edge.sourceNodeId);
		if (!sourceNode) continue;
		const upstream = instances.find((entry) => entry.id === sourceNode.instanceId);
		if (!upstream) continue;
		bindings.push({
			inputPortId: inPort.id,
			inputSpecId: specId,
			upstreamInstanceId: upstream.id,
			upstreamPortId: edge.sourcePortId,
			resourceKind: inPort.resourceKind,
		});
	}
	return bindings;
}

export function resolveUpstreamResourceHandle(
	graph: OsMediaGraph,
	instances: readonly OsAppInstance[],
	targetInstanceId: string,
	inputPortId?: string,
): string | null {
	const bindings = resolveInputBindingsForInstance(graph, instances, targetInstanceId);
	if (inputPortId) {
		const binding = bindings.find((entry) => entry.inputPortId === inputPortId);
		return binding?.upstreamInstanceId ?? null;
	}
	return bindings[0]?.upstreamInstanceId ?? null;
}

export function appInstanceResourceProjection(
	graph: OsMediaGraph,
	instances: readonly OsAppInstance[],
	instanceId: string,
	options?: {
		readonly outputPortId?: string;
		readonly context?: Omit<AppMaterializeContext, "graph" | "instances" | "parameterValues">;
		readonly parameters?: readonly OsParameter[];
		readonly parameterBindings?: readonly OsParameterFieldBinding[];
	},
): {
	readonly kind: OsResourceKindId;
	readonly projection: unknown;
	readonly upstreamInstanceId: string | null;
	readonly upstreamProjection: unknown | null;
	readonly inputBindings: Readonly<Record<string, unknown>>;
	readonly outputProjections: Readonly<Record<string, unknown>>;
} | null {
	const instance = instances.find((entry) => entry.id === instanceId);
	if (!instance) return null;
	const registration = osAppRegistration(instance.programId, instance.appId);
	const bindings = resolveInputBindingsForInstance(graph, instances, instanceId);
	const inputProjections: Record<string, unknown> = {};
	for (const binding of bindings) {
		const upstreamSpecId = mediaPortSpecId(binding.upstreamPortId);
		const upstreamProjection = appInstanceResourceProjection(graph, instances, binding.upstreamInstanceId, {
			outputPortId: upstreamSpecId ?? undefined,
			context: options?.context,
			parameters: options?.parameters,
			parameterBindings: options?.parameterBindings,
		});
		if (upstreamProjection) inputProjections[binding.inputSpecId] = upstreamProjection.projection;
	}
	const materializeContext: AppMaterializeContext = {
		...options?.context,
		graph,
		instances,
		inputBindings: inputProjections,
		parameterValues: resolveParameterValuesForInstance(options?.parameterBindings ?? [], options?.parameters ?? [], instanceId),
		outputPortId: options?.outputPortId,
	};
	let projection = materializeAppInstanceProjection(instance, materializeContext);
	const upstreamInstanceId = bindings[0]?.upstreamInstanceId ?? null;
	let upstreamProjection: unknown | null = null;
	if (upstreamInstanceId) {
		const upstreamBundle = appInstanceResourceProjection(graph, instances, upstreamInstanceId, {
			context: options?.context,
			parameters: options?.parameters,
			parameterBindings: options?.parameterBindings,
		});
		upstreamProjection = upstreamBundle?.projection ?? null;
		if (!instance.sourceDocument.inline && !instance.sourceDocument.vcsJson && projection == null) {
			projection = upstreamProjection;
		}
	}
	const outputProjections: Record<string, unknown> = {};
	for (const spec of registration?.outputs ?? []) {
		outputProjections[spec.id] = materializeAppInstanceProjection(instance, {
			...materializeContext,
			outputPortId: spec.id,
		});
	}
	const outputPortId = options?.outputPortId ?? registration?.outputs[0]?.id ?? "out";
	const kind = registration?.outputs.find((spec) => spec.id === outputPortId)?.resourceKind ?? instance.yields;
	return {
		kind,
		projection: options?.outputPortId ? outputProjections[outputPortId] : projection,
		upstreamInstanceId,
		upstreamProjection,
		inputBindings: inputProjections,
		outputProjections,
	};
}

/** @emoji 🌉 Converts an S media graph into a DAG fixture for the WASM canvas. */
export function osMediaGraphToDagFixture(
	graph: OsMediaGraph,
	instances: readonly OsAppInstance[],
	camera: { readonly x: number; readonly y: number; readonly zoom: number } = { x: 0, y: 0, zoom: 1 },
	parameters: readonly OsParameter[] = [],
): {
	readonly schema: "dag.fixture";
	readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
	readonly nodes: readonly Record<string, unknown>[];
	readonly edges: readonly { readonly id: string; readonly source: string; readonly target: string }[];
} {
	const instanceById = new Map(instances.map((instance) => [instance.id, instance]));
	const parameterById = new Map(parameters.map((parameter) => [parameter.id, parameter]));
	const nodes = graph.nodes.map((node) => {
		const instance = instanceById.get(node.instanceId);
		const registration = instance ? osAppRegistration(instance.programId, instance.appId) : undefined;
		return {
			id: node.id,
			name: instance?.label ?? node.instanceId,
			abbreviation: instance?.appId.slice(0, 3) ?? "app",
			icon: `emoji:${registration?.componentKind ?? "s"}`,
			x: node.x + node.width / 2,
			y: node.y + node.height / 2,
			width: node.width,
			height: node.height,
			kind: "appInstance",
			operatorKind: instance?.programId ?? "app",
			instanceId: node.instanceId,
			programId: instance?.programId ?? "",
			appId: instance?.appId ?? "",
			inputs: node.inputs.map((port) => {
				const parameterId = parameterIdFromPortId(port.id);
				const parameter = parameterId ? parameterById.get(parameterId) : undefined;
				return {
					id: port.id,
					label: parameter?.name ?? mediaPortSpecId(port.id) ?? port.id,
					resourceKind: port.resourceKind,
				};
			}),
			outputs: node.outputs.map((port) => ({
				id: port.id,
				label: mediaPortSpecId(port.id) ?? port.id,
				resourceKind: port.resourceKind,
			})),
		};
	});
	const edges = graph.edges.map((edge) => ({
		id: edge.id,
		source: `${edge.sourceNodeId}:${edge.sourcePortId}`,
		target: `${edge.targetNodeId}:${edge.targetPortId}`,
	}));
	return { schema: "dag.fixture", camera, nodes, edges };
}

export function osMediaGraphToDagFixtureJson(
	graph: OsMediaGraph,
	instances: readonly OsAppInstance[],
	camera?: { readonly x: number; readonly y: number; readonly zoom: number },
	parameters?: readonly OsParameter[],
): string {
	return JSON.stringify(osMediaGraphToDagFixture(graph, instances, camera, parameters));
}

/** @emoji 🔁 Applies structural DAG fixture edits back onto the studio media graph. */
export function applyDagFixtureJsonToOsMediaGraph(
	graph: OsMediaGraph,
	fixtureJson: string,
	dispatch: (command: OsCommand) => void,
): void {
	const fixture = JSON.parse(fixtureJson) as {
		readonly nodes?: readonly {
			readonly id: string;
			readonly x?: number;
			readonly y?: number;
			readonly width?: number;
			readonly height?: number;
		}[];
		readonly edges?: readonly { readonly id: string; readonly source: string; readonly target: string }[];
	};
	for (const dagNode of fixture.nodes ?? []) {
		const node = graph.nodes.find((entry) => entry.id === dagNode.id);
		if (!node || dagNode.x == null || dagNode.y == null) continue;
		const width = dagNode.width ?? node.width;
		const height = dagNode.height ?? node.height;
		const x = dagNode.x - width / 2;
		const y = dagNode.y - height / 2;
		if (Math.abs(node.x - x) > 0.5 || Math.abs(node.y - y) > 0.5) {
			dispatch({ kind: "moveMediaNode", nodeId: node.id, x, y });
		}
	}
	const edgeKey = (source: string, target: string) => `${source}→${target}`;
	const beforeKeys = new Set(graph.edges.map((edge) => edgeKey(`${edge.sourceNodeId}:${edge.sourcePortId}`, `${edge.targetNodeId}:${edge.targetPortId}`)));
	const afterKeys = new Set((fixture.edges ?? []).map((edge) => edgeKey(edge.source, edge.target)));
	for (const edge of fixture.edges ?? []) {
		if (beforeKeys.has(edgeKey(edge.source, edge.target))) continue;
		const [sourceNodeId, ...sourcePortParts] = edge.source.split(":");
		const [targetNodeId, ...targetPortParts] = edge.target.split(":");
		const sourcePortId = sourcePortParts.join(":");
		const targetPortId = targetPortParts.join(":");
		if (!sourceNodeId || !targetNodeId || !sourcePortId || !targetPortId) continue;
		dispatch({ kind: "connectMediaPorts", sourceNodeId, sourcePortId, targetNodeId, targetPortId });
	}
	for (const edge of graph.edges) {
		const key = edgeKey(`${edge.sourceNodeId}:${edge.sourcePortId}`, `${edge.targetNodeId}:${edge.targetPortId}`);
		if (!afterKeys.has(key)) dispatch({ kind: "disconnectMediaEdge", edgeId: edge.id });
	}
}
//#endregion 🔖MediaGraphEngine

//#region 🔖Projection
let sIdCounter = 0;

export function createOsId(prefix = "s"): string {
	sIdCounter += 1;
	return `${prefix}-${sIdCounter}`;
}

export function defaultOsProjection(): OsProjection {
	return { programs: [], activeProgramId: null, activeAlternativeId: null, appInstances: [], mediaGraph: emptyMediaGraph(), parameters: [], parameterBindings: [] };
}

export function createEmptyOsDocument(id = "studio", name = "Studio"): OsDocument {
	return {
		schema: OS_STUDIO_SCHEMA,
		id,
		name,
		vcs: {
			initialProjection: defaultOsProjection(),
			operations: [],
			checkpoints: [],
			alternatives: [],
		},
		backbone: { kind: "dev", uri: "dev://studio.json" },
	};
}

function cloneProjection(projection: OsProjection): OsProjection {
	return {
		programs: [...projection.programs],
		activeProgramId: projection.activeProgramId,
		activeAlternativeId: projection.activeAlternativeId,
		appInstances: projection.appInstances.map((instance) => ({
			...instance,
			sourceDocument: { ...instance.sourceDocument },
		})),
		parameters: projection.parameters.map((parameter) => ({ ...parameter, ...(parameter.type === "categorical" ? { options: [...parameter.options] } : {}) })),
		parameterBindings: projection.parameterBindings.map((binding) => ({ ...binding })),
		mediaGraph: {
			schema: OS_MEDIA_GRAPH_SCHEMA,
			nodes: projection.mediaGraph.nodes.map((node) => ({
				...node,
				inputs: node.inputs.map((port) => ({ ...port })),
				outputs: node.outputs.map((port) => ({ ...port })),
			})),
			edges: projection.mediaGraph.edges.map((edge) => ({ ...edge })),
		},
	};
}

function applyOsOperation(projection: OsProjection, operation: OsOperation): OsProjection {
	const next = cloneProjection(projection);
	switch (operation.op) {
		case "setStudioName":
			return next;
		case "setActiveProgram":
			return { ...next, activeProgramId: (operation.payload.programId as string | null) ?? null };
		case "setActiveAlternative":
			return { ...next, activeAlternativeId: (operation.payload.alternativeId as string | null) ?? null };
		case "createAlternative":
			return next;
		case "applyAppOperation": {
			const instanceId = String(operation.payload.instanceId);
			const nextSource = operation.payload.nextSource as OsSourceDocument;
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId ? { ...instance, sourceDocument: { ...nextSource } } : instance,
			);
			return next;
		}
		case "spawnAppInstance": {
			const instance = operation.payload.instance as OsAppInstance;
			if (!next.programs.includes(instance.programId)) next.programs.push(instance.programId);
			next.appInstances.push(instance);
			const position = (operation.payload.position as { x: number; y: number }) ?? { x: 0, y: 0 };
			const node = syncMediaNodeParameterPorts(mediaGraphNodeForInstance(instance, position), next.parameterBindings);
			next.mediaGraph.nodes.push(node);
			return next;
		}
		case "removeAppInstance": {
			const instanceId = String(operation.payload.instanceId);
			next.appInstances = next.appInstances.filter((instance) => instance.id !== instanceId);
			next.parameterBindings = next.parameterBindings.filter((binding) => binding.instanceId !== instanceId);
			const nodeId = next.mediaGraph.nodes.find((node) => node.instanceId === instanceId)?.id;
			next.mediaGraph.nodes = next.mediaGraph.nodes.filter((node) => node.instanceId !== instanceId);
			if (nodeId) next.mediaGraph.edges = next.mediaGraph.edges.filter((edge) => edge.sourceNodeId !== nodeId && edge.targetNodeId !== nodeId);
			return next;
		}
		case "connectMediaPorts": {
			const edge = operation.payload.edge as OsMediaGraphEdge;
			next.mediaGraph.edges.push(edge);
			return next;
		}
		case "disconnectMediaEdge": {
			const edgeId = String(operation.payload.edgeId);
			next.mediaGraph.edges = next.mediaGraph.edges.filter((edge) => edge.id !== edgeId);
			return next;
		}
		case "moveMediaNode": {
			const nodeId = String(operation.payload.nodeId);
			const x = Number(operation.payload.x);
			const y = Number(operation.payload.y);
			next.mediaGraph.nodes = next.mediaGraph.nodes.map((node) => (node.id === nodeId ? { ...node, x, y } : node));
			return next;
		}
		case "patchAppSource": {
			const instanceId = String(operation.payload.instanceId);
			const inline = String(operation.payload.inline ?? "");
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId ? { ...instance, sourceDocument: { ...instance.sourceDocument, inline } } : instance,
			);
			return next;
		}
		case "patchAppInstance": {
			const instanceId = String(operation.payload.instanceId);
			const label = typeof operation.payload.label === "string" ? operation.payload.label : undefined;
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId && label !== undefined ? { ...instance, label } : instance,
			);
			return next;
		}
		case "addParameter": {
			const parameter = operation.payload.parameter as OsParameter;
			next.parameters = [...next.parameters, parameter];
			return next;
		}
		case "removeParameter": {
			const parameterId = String(operation.payload.parameterId);
			next.parameters = next.parameters.filter((parameter) => parameter.id !== parameterId);
			next.parameterBindings = next.parameterBindings.filter((binding) => binding.parameterId !== parameterId);
			next.mediaGraph = syncMediaGraphParameterPorts(next.mediaGraph, next.parameterBindings);
			return next;
		}
		case "patchParameter": {
			const parameterId = String(operation.payload.parameterId);
			const nextParameter = operation.payload.parameter as OsParameter;
			next.parameters = next.parameters.map((parameter) => (parameter.id === parameterId ? nextParameter : parameter));
			return next;
		}
		case "bindParameterField": {
			const binding = operation.payload.binding as OsParameterFieldBinding;
			next.parameterBindings = [
				...next.parameterBindings.filter(
					(entry) => !(entry.instanceId === binding.instanceId && entry.fieldPath === binding.fieldPath),
				),
				binding,
			];
			next.mediaGraph = syncMediaGraphParameterPorts(next.mediaGraph, next.parameterBindings);
			return next;
		}
		case "unbindParameterField": {
			const instanceId = String(operation.payload.instanceId);
			const fieldPath = String(operation.payload.fieldPath);
			next.parameterBindings = next.parameterBindings.filter(
				(binding) => !(binding.instanceId === instanceId && binding.fieldPath === fieldPath),
			);
			next.mediaGraph = syncMediaGraphParameterPorts(next.mediaGraph, next.parameterBindings);
			return next;
		}
		case "syncParameterPorts": {
			next.mediaGraph = syncMediaGraphParameterPorts(next.mediaGraph, next.parameterBindings);
			return next;
		}
		default:
			return next;
	}
}

export function materializeOsProjection(document: OsDocument, appliedChangeIds: readonly string[] = []): OsProjection {
	let projection = normalizeOsProjection(cloneProjection(document.vcs.initialProjection));
	for (const changeId of appliedChangeIds) {
		const change = document.vcs.operations.find((entry) => entry.id === changeId);
		if (!change) continue;
		for (const operation of change.forwards) projection = applyOsOperation(projection, operation);
	}
	return { ...projection, activeAlternativeId: projection.activeAlternativeId ?? null };
}

function normalizeOsProjection(projection: OsProjection): OsProjection {
	return {
		...projection,
		parameters: projection.parameters ?? [],
		parameterBindings: projection.parameterBindings ?? [],
	};
}

export function parseOsDocument(raw: unknown): OsDocument {
	const value = raw as Partial<OsDocument>;
	if (value.schema !== OS_STUDIO_SCHEMA) throw new Error(`expected schema ${OS_STUDIO_SCHEMA}`);
	if (!value.id || !value.name || !value.vcs?.initialProjection) throw new Error("studio document requires id, name, and vcs.initialProjection");
	return {
		...(value as OsDocument),
		vcs: {
			...value.vcs,
			initialProjection: normalizeOsProjection(value.vcs.initialProjection as OsProjection),
		},
	};
}

export function osDocumentToJson(document: OsDocument): string {
	return JSON.stringify(document, null, 2);
}

export function osDocumentFromJson(json: string): OsDocument {
	return parseOsDocument(JSON.parse(json));
}
//#endregion 🔖Projection

//#region 🔖MediaExport
export type OsMediaExportFormat = "svg" | "png" | "obj" | "glb";

export interface OsMediaExportResult {
	readonly data: string | Uint8Array;
	readonly mimeType: string;
	readonly fileName: string;
}

export type OsMediaExportHandler = (sourceDocument: unknown) => Promise<OsMediaExportResult>;

const osMediaExportHandlers = new Map<string, OsMediaExportHandler>();

function osMediaExportKey(resourceKind: OsResourceKindId, format: OsMediaExportFormat): string {
	return `${resourceKind}:${format}`;
}

/** @emoji 💾 Registers an export handler for a media resource kind and format. */
export function registerOsMediaExportHandler(
	resourceKind: OsResourceKindId,
	format: OsMediaExportFormat,
	handler: OsMediaExportHandler,
): void {
	osMediaExportHandlers.set(osMediaExportKey(resourceKind, format), handler);
}

export function requiredOsMediaExportFormats(dimension: string): readonly OsMediaExportFormat[] {
	if (dimension === "2d") return ["svg", "png"];
	if (dimension === "3d" || dimension === "5d") return ["glb", "obj"];
	return [];
}

export function assertOsMediaExportCoverage(): void {
	const missing: string[] = [];
	for (const descriptor of listOsResourceDescriptors()) {
		for (const format of requiredOsMediaExportFormats(descriptor.dimension)) {
			if (!osMediaExportHandlers.has(osMediaExportKey(descriptor.kind, format))) {
				missing.push(`${descriptor.kind}:${format}`);
			}
		}
	}
	if (missing.length) {
		throw new Error(`missing os media export handlers: ${missing.join(", ")}`);
	}
}

export async function exportOsAppInstanceMedia(
	instance: OsAppInstance,
	sourceDocument: unknown,
	format: OsMediaExportFormat,
): Promise<OsMediaExportResult> {
	const handler = osMediaExportHandlers.get(osMediaExportKey(instance.yields, format));
	if (!handler) {
		throw new Error(`no export handler for ${instance.yields}:${format}`);
	}
	return handler(sourceDocument);
}

export function osMediaExportExtensionForFormat(format: OsMediaExportFormat): string {
	switch (format) {
		case "svg":
			return "svg";
		case "png":
			return "png";
		case "obj":
			return "obj";
		case "glb":
			return "glb";
	}
}

export const OS_MEDIA_GRAPH_VFS_ROOT_ID = "os-media-graph-root" as const;

export const OS_MEDIA_GRAPH_VFS_SCHEMA: VirtualFileSystemSchemaModel = {
	descriptorKinds: {
		text: { id: "text", name: "Text", presentation: "text" },
	},
	fileNodeKinds: {
		root: {
			id: "root",
			name: "Root",
			descriptors: [{ id: "binding", descriptorKindId: "text", label: "Binding" }],
		},
		instance: {
			id: "instance",
			name: "App Instance",
			descriptors: [{ id: "binding", descriptorKindId: "text", label: "Binding" }],
		},
		folder: {
			id: "folder",
			name: "Folder",
			descriptors: [{ id: "binding", descriptorKindId: "text", label: "Binding" }],
		},
		source: {
			id: "source",
			name: "Source",
			descriptors: [{ id: "binding", descriptorKindId: "text", label: "Binding" }],
		},
		export: {
			id: "export",
			name: "Export",
			descriptors: [
				{ id: "binding", descriptorKindId: "text", label: "Binding" },
				{ id: "format", descriptorKindId: "text", label: "Format" },
			],
		},
		input: {
			id: "input",
			name: "Input Port",
			descriptors: [{ id: "binding", descriptorKindId: "text", label: "Binding" }],
		},
	},
	descriptorColumnIds: ["binding", "format"],
};

function osMediaGraphVfsInstanceId(nodeId: string): string | null {
	const match = /^inst:([^:]+)(?::|$)/.exec(nodeId);
	return match?.[1] ?? null;
}

function osMediaGraphVfsInstanceFolderId(instanceId: string): string {
	return `inst:${instanceId}`;
}

function osMediaGraphVfsSourceId(instanceId: string): string {
	return `inst:${instanceId}:source`;
}

function osMediaGraphVfsInputsFolderId(instanceId: string): string {
	return `inst:${instanceId}:inputs`;
}

function osMediaGraphVfsOutputsFolderId(instanceId: string): string {
	return `inst:${instanceId}:outputs`;
}

function osMediaGraphVfsInputPortId(instanceId: string, portSpecId: string): string {
	return `inst:${instanceId}:input:${portSpecId}`;
}

function osMediaGraphVfsExportId(instanceId: string, portSpecId: string, format: OsMediaExportFormat): string {
	return `inst:${instanceId}:export:${portSpecId}:${format}`;
}

/** @emoji 📁 Bidirectional VFS projection of the OS media graph backed by {@link OsStore}. */
export class OsMediaGraphVirtualFileSystemController extends VirtualFileSystemController {
	private readonly storeProvider: () => OsStore;
	private readonly onOpenInstance?: (instanceId: string) => void;
	private readonly onExport?: (instanceId: string, portSpecId: string, format: OsMediaExportFormat) => void | Promise<void>;
	private readonly onSpawnApp?: (programId: string, appId: string) => void;

	constructor(
		id: string,
		commandBus: CommandBus,
		hostNotify: () => void,
		options: {
			readonly store: () => OsStore;
			readonly onOpenInstance?: (instanceId: string) => void;
			readonly onExport?: (instanceId: string, portSpecId: string, format: OsMediaExportFormat) => void | Promise<void>;
			readonly onSpawnApp?: (programId: string, appId: string) => void;
		},
	) {
		super(id, commandBus, hostNotify);
		this.storeProvider = options.store;
		this.onOpenInstance = options.onOpenInstance;
		this.onExport = options.onExport;
		this.onSpawnApp = options.onSpawnApp;
	}

	invalidateMediaGraphVirtualFileSystem(scope: VirtualFileSystemScope): void {
		this.invalidateVirtualFileSystemScope(scope);
	}

	protected override getSchema(_scope: VirtualFileSystemScope): VirtualFileSystemSchemaModel {
		return OS_MEDIA_GRAPH_VFS_SCHEMA;
	}

	protected override getRoot(_scope: VirtualFileSystemScope): VirtualFileSystemNodeRecord {
		return {
			id: OS_MEDIA_GRAPH_VFS_ROOT_ID,
			fileNodeKindId: "root",
			name: "Media Graph",
			path: "/",
			parentId: null,
			hasChildren: true,
			icon: "folder",
		};
	}

	protected override loadChildren(parentId: string, _scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[] {
		const projection = this.storeProvider().projection();
		if (parentId === OS_MEDIA_GRAPH_VFS_ROOT_ID) {
			return projection.appInstances.map((instance) => {
				const registration = osAppRegistration(instance.programId, instance.appId);
				return {
					id: osMediaGraphVfsInstanceFolderId(instance.id),
					fileNodeKindId: "instance",
					name: `${instance.label} (${instance.programId}.${instance.appId})`,
					path: `/${instance.label}`,
					parentId: OS_MEDIA_GRAPH_VFS_ROOT_ID,
					hasChildren: true,
					icon: registration?.componentKind ?? "component",
					canDrag: true,
					descriptorValues: virtualFileSystemDescriptorValues(OS_MEDIA_GRAPH_VFS_SCHEMA, "instance", {
						path: `/${instance.label}`,
						textByDescriptorId: { binding: instance.yields },
					}),
				};
			});
		}
		const instanceId = osMediaGraphVfsInstanceId(parentId);
		if (!instanceId) return [];
		const instance = projection.appInstances.find((entry) => entry.id === instanceId);
		if (!instance) return [];
		const registration = osAppRegistration(instance.programId, instance.appId);
		const graphNode = projection.mediaGraph.nodes.find((node) => node.instanceId === instanceId);
		if (parentId === osMediaGraphVfsInstanceFolderId(instanceId)) {
			return [
				{
					id: osMediaGraphVfsSourceId(instanceId),
					fileNodeKindId: "source",
					name: "source.json",
					path: `/${instance.label}/source.json`,
					parentId,
					hasChildren: false,
					icon: "json",
					navigateUri: `os://instance/${instanceId}`,
					descriptorValues: virtualFileSystemDescriptorValues(OS_MEDIA_GRAPH_VFS_SCHEMA, "source", {
						path: `/${instance.label}/source.json`,
						textByDescriptorId: { binding: registration?.sourceFormat ?? instance.yields },
					}),
				},
				{
					id: osMediaGraphVfsInputsFolderId(instanceId),
					fileNodeKindId: "folder",
					name: "inputs",
					path: `/${instance.label}/inputs`,
					parentId,
					hasChildren:
						(registration?.inputs.length ?? 0) > 0 ||
						projection.parameterBindings.some((binding) => binding.instanceId === instanceId),
					icon: "folder-input",
				},
				{
					id: osMediaGraphVfsOutputsFolderId(instanceId),
					fileNodeKindId: "folder",
					name: "outputs",
					path: `/${instance.label}/outputs`,
					parentId,
					hasChildren: (registration?.outputs.length ?? 0) > 0,
					icon: "folder-output",
				},
			];
		}
		if (parentId === osMediaGraphVfsInputsFolderId(instanceId)) {
			const bindings = resolveInputBindingsForInstance(projection.mediaGraph, projection.appInstances, instanceId);
			const parameterById = new Map(projection.parameters.map((parameter) => [parameter.id, parameter]));
			const resourceRows = (registration?.inputs ?? []).map((spec) => {
				const binding = bindings.find((entry) => entry.inputSpecId === spec.id);
				return {
					id: osMediaGraphVfsInputPortId(instanceId, spec.id),
					fileNodeKindId: "input",
					name: spec.id,
					path: `/${instance.label}/inputs/${spec.id}`,
					parentId,
					hasChildren: false,
					icon: "plug",
					canDrag: true,
					descriptorValues: virtualFileSystemDescriptorValues(OS_MEDIA_GRAPH_VFS_SCHEMA, "input", {
						path: `/${instance.label}/inputs/${spec.id}`,
						textByDescriptorId: {
							binding: binding
								? `← ${binding.upstreamInstanceId}:${mediaPortSpecId(binding.upstreamPortId) ?? binding.upstreamPortId}`
								: spec.resourceKind,
						},
					}),
				};
			});
			const parameterRows = projection.parameterBindings
				.filter((binding) => binding.instanceId === instanceId)
				.map((binding) => {
					const parameter = parameterById.get(binding.parameterId);
					return {
						id: osMediaGraphVfsInputPortId(instanceId, `param.${binding.parameterId}`),
						fileNodeKindId: "input",
						name: parameter?.name ?? binding.fieldPath,
						path: `/${instance.label}/inputs/param.${binding.parameterId}`,
						parentId,
						hasChildren: false,
						icon: "sliders-horizontal",
						descriptorValues: virtualFileSystemDescriptorValues(OS_MEDIA_GRAPH_VFS_SCHEMA, "input", {
							path: `/${instance.label}/inputs/param.${binding.parameterId}`,
							textByDescriptorId: { binding: parameter?.name ?? binding.parameterId },
						}),
					};
				});
			return [...resourceRows, ...parameterRows];
		}
		if (parentId === osMediaGraphVfsOutputsFolderId(instanceId)) {
			const descriptor = osResourceDescriptor(instance.yields);
			const formats = requiredOsMediaExportFormats(descriptor.dimension);
			const rows: VirtualFileSystemNodeRecord[] = [];
			for (const spec of registration?.outputs ?? []) {
				for (const format of formats) {
					const ext = osMediaExportExtensionForFormat(format);
					rows.push({
						id: osMediaGraphVfsExportId(instanceId, spec.id, format),
						fileNodeKindId: "export",
						name: `${spec.id}.${ext}`,
						path: `/${instance.label}/outputs/${spec.id}.${ext}`,
						parentId,
						hasChildren: false,
						icon: ext,
						navigateUri: `os://export/${instanceId}/${spec.id}/${format}`,
						descriptorValues: virtualFileSystemDescriptorValues(OS_MEDIA_GRAPH_VFS_SCHEMA, "export", {
							path: `/${instance.label}/outputs/${spec.id}.${ext}`,
							textByDescriptorId: { binding: spec.resourceKind, format },
						}),
					});
				}
			}
			return rows;
		}
		if (graphNode && parentId.startsWith(`inst:${instanceId}:`)) return [];
		return [];
	}

	protected override createNode(parentId: string, name: string, _fileNodeKindId: string, scope: VirtualFileSystemScope): boolean {
		if (parentId !== OS_MEDIA_GRAPH_VFS_ROOT_ID) return false;
		const [programId, appId] = name.split(".").map((part) => part.trim());
		if (!programId || !appId) return false;
		if (!osAppRegistration(programId, appId)) return false;
		if (this.onSpawnApp) {
			this.onSpawnApp(programId, appId);
		} else {
			this.storeProvider().dispatch({ kind: "spawnAppInstance", programId, appId, label: name });
		}
		this.invalidateMediaGraphVirtualFileSystem(scope);
		return true;
	}

	protected override renameNode(nodeId: string, name: string, _scope: VirtualFileSystemScope): boolean {
		const instanceId = osMediaGraphVfsInstanceId(nodeId);
		if (!instanceId || nodeId !== osMediaGraphVfsInstanceFolderId(instanceId)) return false;
		this.storeProvider().dispatch({ kind: "patchAppInstances", instanceIds: [instanceId], field: "label", value: name });
		return true;
	}

	protected override deleteNode(nodeId: string, scope: VirtualFileSystemScope): boolean {
		const instanceId = osMediaGraphVfsInstanceId(nodeId);
		if (!instanceId) return false;
		if (nodeId === osMediaGraphVfsInstanceFolderId(instanceId)) {
			this.storeProvider().dispatch({ kind: "removeAppInstance", instanceId });
			this.invalidateMediaGraphVirtualFileSystem(scope);
			return true;
		}
		const inputMatch = /^inst:([^:]+):input:([^:]+)$/.exec(nodeId);
		if (inputMatch) {
			const [, targetInstanceId, portSpecId] = inputMatch;
			const projection = this.storeProvider().projection();
			const graphNode = projection.mediaGraph.nodes.find((node) => node.instanceId === targetInstanceId);
			if (!graphNode) return false;
			const targetPortId = mediaPortIdForSpec(targetInstanceId!, portSpecId!, "in");
			const edge = projection.mediaGraph.edges.find((entry) => entry.targetPortId === targetPortId);
			if (edge) this.storeProvider().dispatch({ kind: "disconnectMediaEdge", edgeId: edge.id });
			this.invalidateMediaGraphVirtualFileSystem(scope);
			return true;
		}
		return false;
	}

	protected override moveNodePersisted(activeId: string, overId: string, scope: VirtualFileSystemScope): boolean {
		const inputMatch = /^inst:([^:]+):input:([^:]+)$/.exec(activeId);
		const exportMatch = /^inst:([^:]+):export:([^:]+):([a-z]+)$/.exec(overId);
		if (!inputMatch || !exportMatch) return false;
		const [, targetInstanceId, targetPortSpecId] = inputMatch;
		const [, sourceInstanceId, sourcePortSpecId] = exportMatch;
		if (targetInstanceId === sourceInstanceId) return false;
		const projection = this.storeProvider().projection();
		const sourceNode = projection.mediaGraph.nodes.find((node) => node.instanceId === sourceInstanceId);
		const targetNode = projection.mediaGraph.nodes.find((node) => node.instanceId === targetInstanceId);
		if (!sourceNode || !targetNode) return false;
		this.storeProvider().dispatch({
			kind: "connectMediaPorts",
			sourceNodeId: sourceNode.id,
			sourcePortId: mediaPortIdForSpec(sourceInstanceId!, sourcePortSpecId!, "out"),
			targetNodeId: targetNode.id,
			targetPortId: mediaPortIdForSpec(targetInstanceId!, targetPortSpecId!, "in"),
		});
		this.invalidateMediaGraphVirtualFileSystem(scope);
		return true;
	}

	protected override runVirtualFileSystemCommand(command: string, args?: unknown): boolean {
		if (command === "navigateVirtualFileSystemNode") {
			const payload = (args ?? {}) as { nodeId?: string };
			if (!payload.nodeId) return true;
			if (payload.nodeId.endsWith(":source")) {
				const instanceId = osMediaGraphVfsInstanceId(payload.nodeId);
				if (instanceId) this.onOpenInstance?.(instanceId);
				return true;
			}
			const exportMatch = /^inst:([^:]+):export:([^:]+):([a-z]+)$/.exec(payload.nodeId);
			if (exportMatch) {
				const [, instanceId, portSpecId, format] = exportMatch;
				void this.onExport?.(instanceId!, portSpecId!, format as OsMediaExportFormat);
				return true;
			}
		}
		return super.runVirtualFileSystemCommand(command, args);
	}
}

export const OS_STORAGE_VFS_ROOT_ID = "os-storage-root";

export interface OsStorageNodeRecord {
	readonly id: string;
	readonly parentId: string | null;
	readonly name: string;
	readonly kind: "folder" | "document";
	readonly documentId?: string;
}

/** @emoji 🗄️ VFS controller for OS storage nodes (hub REST or dev localStorage namespace). */
export class OsStorageVirtualFileSystemController extends VirtualFileSystemController {
	private readonly hubOrigin: string | null;
	private readonly onOpenDocument?: (documentId: string) => void;
	private cachedNodes: OsStorageNodeRecord[] = [];

	constructor(
		id: string,
		commandBus: CommandBus,
		hostNotify: () => void,
		options?: { readonly hubOrigin?: string | null; readonly onOpenDocument?: (documentId: string) => void },
	) {
		super(id, commandBus, hostNotify);
		this.hubOrigin = options?.hubOrigin ?? null;
		this.onOpenDocument = options?.onOpenDocument;
	}

	protected override loadChildren(parentId: string, _scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[] {
		const nodes = this.resolveNodes();
		return nodes
			.filter((node) => node.parentId === (parentId === OS_STORAGE_VFS_ROOT_ID ? null : parentId))
			.map((node) => ({
				id: node.id,
				parentId: node.parentId ?? OS_STORAGE_VFS_ROOT_ID,
				name: node.name,
				fileNodeKindId: node.kind === "folder" ? "folder" : "source",
			}));
	}

	protected override runVirtualFileSystemCommand(command: string, args?: unknown): boolean {
		if (command === "navigateVirtualFileSystemNode") {
			const payload = (args ?? {}) as { nodeId?: string };
			const node = this.resolveNodes().find((entry) => entry.id === payload.nodeId);
			if (node?.documentId) {
				this.onOpenDocument?.(node.documentId);
				return true;
			}
		}
		return super.runVirtualFileSystemCommand(command, args);
	}

	private resolveNodes(): readonly OsStorageNodeRecord[] {
		if (this.hubOrigin) return this.cachedNodes;
		if (typeof localStorage === "undefined") return [];
		const nodes: OsStorageNodeRecord[] = [
			{ id: "storage:documents", parentId: null, name: "Documents", kind: "folder" },
		];
		for (const key of Object.keys(localStorage)) {
			if (!key.startsWith("s:backbone:")) continue;
			const uri = key.slice("s:backbone:".length);
			const id = `storage:doc:${uri}`;
			nodes.push({ id, parentId: "storage:documents", name: uri, kind: "document", documentId: uri });
		}
		return nodes;
	}

	async refreshFromHub(): Promise<void> {
		if (!this.hubOrigin) return;
		const response = await fetch(`${this.hubOrigin}/nodes`);
		if (!response.ok) throw new Error(`storage nodes fetch failed: ${response.status}`);
		this.cachedNodes = (await response.json()) as OsStorageNodeRecord[];
		this.invalidateVirtualFileSystem();
	}
}
//#endregion 🔖MediaExport

//#region 🔖OsStore
export type OsCommand =
	| { readonly kind: "spawnAppInstance"; readonly programId: string; readonly appId: string; readonly label?: string; readonly sourceInline?: string; readonly payloadRef?: string; readonly position?: { readonly x: number; readonly y: number } }
	| { readonly kind: "removeAppInstance"; readonly instanceId: string }
	| { readonly kind: "connectMediaPorts"; readonly sourceNodeId: string; readonly sourcePortId: string; readonly targetNodeId: string; readonly targetPortId: string }
	| { readonly kind: "disconnectMediaEdge"; readonly edgeId: string }
	| { readonly kind: "moveMediaNode"; readonly nodeId: string; readonly x: number; readonly y: number }
	| { readonly kind: "patchAppInstances"; readonly instanceIds: readonly string[]; readonly field: "label"; readonly value?: string }
	| { readonly kind: "patchAppSource"; readonly instanceId: string; readonly inline: string }
	| { readonly kind: "applyAppOperation"; readonly instanceId: string; readonly forwards: readonly unknown[]; readonly backwards: readonly unknown[] }
	| { readonly kind: "openProgram"; readonly programId: string }
	| { readonly kind: "setStudioName"; readonly name: string }
	| { readonly kind: "commitCheckpoint"; readonly message?: string; readonly authors?: readonly Author[] }
	| { readonly kind: "checkoutCheckpoint"; readonly checkpointId: string }
	| { readonly kind: "createAlternative"; readonly name: string }
	| { readonly kind: "switchAlternative"; readonly alternativeId: string }
	| { readonly kind: "addParameter"; readonly type?: OsParameterType; readonly name?: string }
	| { readonly kind: "removeParameter"; readonly parameterId: string }
	| { readonly kind: "patchParameter"; readonly parameterId: string; readonly patch: Record<string, unknown> }
	| { readonly kind: "bindParameterField"; readonly instanceId: string; readonly fieldPath: string; readonly parameterId: string }
	| { readonly kind: "unbindParameterField"; readonly instanceId: string; readonly fieldPath: string }
	| { readonly kind: "undo" }
	| { readonly kind: "redo" };

export interface PresencePeer {
	readonly clientId: string;
	readonly name: string;
	readonly selection?: readonly string[];
}

export class OsStore {
	private document: OsDocument;
	private appliedChangeIds: string[] = [];
	private redoChangeIds: string[] = [];
	private listeners = new Set<() => void>();
	private generation = 0;
	private projectionSnapshot: OsProjection | undefined;
	private projectionSnapshotGeneration = -1;
	private onAfterMutation?: () => void;
	private presencePeers = new Map<string, PresencePeer>();

	constructor(document: OsDocument, options?: { readonly onAfterMutation?: () => void }) {
		this.document = document;
		this.onAfterMutation = options?.onAfterMutation;
	}

	setOnAfterMutation(listener: (() => void) | undefined): void {
		this.onAfterMutation = listener;
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	getDocument(): OsDocument {
		return this.document;
	}

	getPresencePeers(): readonly PresencePeer[] {
		return [...this.presencePeers.values()];
	}

	setPresencePeers(peers: readonly PresencePeer[]): void {
		this.presencePeers = new Map(peers.map((peer) => [peer.clientId, peer]));
		this.generation += 1;
		for (const listener of this.listeners) listener();
	}

	projection(): OsProjection {
		if (this.projectionSnapshotGeneration === this.generation && this.projectionSnapshot) {
			return this.projectionSnapshot;
		}
		this.projectionSnapshot = materializeOsProjection(this.document, this.appliedChangeIds);
		this.projectionSnapshotGeneration = this.generation;
		return this.projectionSnapshot;
	}

	mediaGraphSnapshot(): OsMediaGraph {
		return this.projection().mediaGraph;
	}

	appInstanceResource(instanceId: string): OsResourceKindId | null {
		const instance = this.projection().appInstances.find((entry) => entry.id === instanceId);
		if (!instance) return null;
		return osAppRegistration(instance.programId, instance.appId) ? osAppPrimaryOutputKind(osAppRegistration(instance.programId, instance.appId)!) : null;
	}

	/** @emoji 📡 Applies an inbound remote change without touching the undo stack. */
	applyRemoteChange(change: OsChange): void {
		if (this.document.vcs.operations.some((entry) => entry.id === change.id)) return;
		this.document = {
			...this.document,
			vcs: { ...this.document.vcs, operations: [...this.document.vcs.operations, change] },
		};
		this.appliedChangeIds.push(change.id);
		this.redoChangeIds = [];
		this.generation += 1;
		for (const listener of this.listeners) listener();
	}

	dispatch(command: OsCommand): void {
		if (command.kind === "undo") {
			const last = this.appliedChangeIds.pop();
			if (!last) return;
			this.redoChangeIds.push(last);
			this.generation += 1;
			for (const listener of this.listeners) listener();
			return;
		}
		if (command.kind === "redo") {
			const next = this.redoChangeIds.pop();
			if (!next) return;
			this.appliedChangeIds.push(next);
			this.generation += 1;
			for (const listener of this.listeners) listener();
			return;
		}
		if (command.kind === "commitCheckpoint") {
			const parentId = this.document.vcs.checkpoints.at(-1)?.id;
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					checkpoints: [
						...this.document.vcs.checkpoints,
						{
							id: createOsId("checkpoint"),
							changeIds: [...this.appliedChangeIds],
							parentId,
							authors: command.authors ? [...command.authors] : [],
							message: command.message,
							savedAt: new Date().toISOString(),
						},
					],
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "checkoutCheckpoint") {
			const checkpoint = this.document.vcs.checkpoints.find((entry) => entry.id === command.checkpointId);
			if (!checkpoint) return;
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "createAlternative") {
			let checkpoint = this.document.vcs.checkpoints.at(-1);
			if (!checkpoint) {
				checkpoint = {
					id: createOsId("checkpoint"),
					changeIds: [...this.appliedChangeIds],
					message: "auto",
					savedAt: new Date().toISOString(),
				};
				this.document = {
					...this.document,
					vcs: { ...this.document.vcs, checkpoints: [...this.document.vcs.checkpoints, checkpoint] },
				};
			}
			const alternative: OsAlternative = {
				id: createOsId("alt"),
				name: command.name,
				checkpointIds: [checkpoint.id],
			};
			this.document = {
				...this.document,
				vcs: { ...this.document.vcs, alternatives: [...this.document.vcs.alternatives, alternative] },
			};
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					initialProjection: {
						...this.document.vcs.initialProjection,
						activeAlternativeId: alternative.id,
					},
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "switchAlternative") {
			const alternative = this.document.vcs.alternatives.find((entry) => entry.id === command.alternativeId);
			if (!alternative) return;
			const checkpointId = alternative.checkpointIds.at(-1);
			const checkpoint = this.document.vcs.checkpoints.find((entry) => entry.id === checkpointId);
			if (!checkpoint) return;
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					initialProjection: {
						...this.document.vcs.initialProjection,
						activeAlternativeId: alternative.id,
					},
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "setStudioName") {
			this.document = { ...this.document, name: command.name };
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		const change = this.commandToChange(command);
		if (!change) return;
		this.document = {
			...this.document,
			vcs: { ...this.document.vcs, operations: [...this.document.vcs.operations, change] },
		};
		this.appliedChangeIds.push(change.id);
		this.redoChangeIds = [];
		this.generation += 1;
		for (const listener of this.listeners) listener();
		this.onAfterMutation?.();
	}

	private commandToChange(command: OsCommand): OsChange | null {
		switch (command.kind) {
			case "undo":
			case "redo":
			case "commitCheckpoint":
			case "checkoutCheckpoint":
			case "createAlternative":
			case "switchAlternative":
			case "setStudioName":
				return null;
			case "openProgram":
				return {
					id: createOsId("change"),
					forwards: [{ op: "setActiveProgram", payload: { programId: command.programId } }],
					backwards: [{ op: "setActiveProgram", payload: { programId: this.projection().activeProgramId } }],
				};
			case "spawnAppInstance": {
				const registration = osAppRegistration(command.programId, command.appId);
				if (!registration) throw new Error(`unknown app ${command.programId}/${command.appId}`);
				const instance: OsAppInstance = {
					id: createOsId("app"),
					programId: command.programId,
					appId: command.appId,
					label: command.label ?? registration.label,
					yields: osAppPrimaryOutputKind(registration),
					sourceDocument: createAppSourceDocument(registration.sourceFormat, createOsId("app-doc"), {
						inline: command.sourceInline,
						payloadRef: command.payloadRef,
					}),
				};
				const spawn: OsOperation = { op: "spawnAppInstance", payload: { instance, position: command.position ?? { x: 40, y: 40 } } };
				const remove: OsOperation = { op: "removeAppInstance", payload: { instanceId: instance.id } };
				return { id: createOsId("change"), forwards: [spawn], backwards: [remove] };
			}
			case "removeAppInstance": {
				const projection = this.projection();
				const instance = projection.appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const node = projection.mediaGraph.nodes.find((entry) => entry.instanceId === instance.id);
				const connectedEdges = projection.mediaGraph.edges.filter((edge) => edge.sourceNodeId === node?.id || edge.targetNodeId === node?.id);
				const forwards: OsOperation[] = [{ op: "removeAppInstance", payload: { instanceId: instance.id } }];
				const backwards: OsOperation[] = [
					{ op: "spawnAppInstance", payload: { instance, position: node ? { x: node.x, y: node.y } : { x: 0, y: 0 } } },
					...connectedEdges.map((edge) => ({ op: "connectMediaPorts" as const, payload: { edge } })),
				];
				return { id: createOsId("change"), forwards, backwards };
			}
			case "connectMediaPorts": {
				const edge: OsMediaGraphEdge = {
					id: createOsId("edge"),
					sourceNodeId: command.sourceNodeId,
					sourcePortId: command.sourcePortId,
					targetNodeId: command.targetNodeId,
					targetPortId: command.targetPortId,
				};
				const validation = validateMediaGraph({
					...this.projection().mediaGraph,
					edges: [...this.projection().mediaGraph.edges, edge],
				});
				if (!validation.ok) throw new Error(validation.errors.join("; "));
				return {
					id: createOsId("change"),
					forwards: [{ op: "connectMediaPorts", payload: { edge } }],
					backwards: [{ op: "disconnectMediaEdge", payload: { edgeId: edge.id } }],
				};
			}
			case "disconnectMediaEdge":
				return {
					id: createOsId("change"),
					forwards: [{ op: "disconnectMediaEdge", payload: { edgeId: command.edgeId } }],
					backwards: [{ op: "connectMediaPorts", payload: { edge: this.projection().mediaGraph.edges.find((edge) => edge.id === command.edgeId) } }],
				};
			case "moveMediaNode": {
				const node = this.projection().mediaGraph.nodes.find((entry) => entry.id === command.nodeId);
				if (!node) return null;
				return {
					id: createOsId("change"),
					forwards: [{ op: "moveMediaNode", payload: { nodeId: command.nodeId, x: command.x, y: command.y } }],
					backwards: [{ op: "moveMediaNode", payload: { nodeId: command.nodeId, x: node.x, y: node.y } }],
				};
			}
			case "patchAppSource": {
				const instance = this.projection().appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const previous = instance.sourceDocument.inline ?? "";
				return {
					id: createOsId("change"),
					forwards: [{ op: "patchAppSource", payload: { instanceId: command.instanceId, inline: command.inline } }],
					backwards: [{ op: "patchAppSource", payload: { instanceId: command.instanceId, inline: previous } }],
				};
			}
			case "patchAppInstances": {
				if (command.field !== "label" || typeof command.value !== "string") return null;
				const projection = this.projection();
				const forwards: OsOperation[] = [];
				const backwards: OsOperation[] = [];
				for (const instanceId of command.instanceIds) {
					const instance = projection.appInstances.find((entry) => entry.id === instanceId);
					if (!instance) continue;
					forwards.push({ op: "patchAppInstance", payload: { instanceId, label: command.value } });
					backwards.push({ op: "patchAppInstance", payload: { instanceId, label: instance.label } });
				}
				if (!forwards.length) return null;
				return { id: createOsId("change"), forwards, backwards };
			}
			case "applyAppOperation": {
				const instance = this.projection().appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const previous = { ...instance.sourceDocument };
				const nextSource = applyAppOperationToSource(instance.sourceDocument, command.forwards, command.backwards);
				return {
					id: createOsId("change"),
					forwards: [{ op: "applyAppOperation", payload: { instanceId: command.instanceId, nextSource } }],
					backwards: [{ op: "applyAppOperation", payload: { instanceId: command.instanceId, nextSource: previous } }],
				};
			}
			case "addParameter": {
				const parameter = createDefaultOsParameter(command.type ?? "numeric", command.name ?? "Parameter", createOsId("param"));
				return {
					id: createOsId("change"),
					forwards: [{ op: "addParameter", payload: { parameter } }],
					backwards: [{ op: "removeParameter", payload: { parameterId: parameter.id } }],
				};
			}
			case "removeParameter": {
				const projection = this.projection();
				const parameter = projection.parameters.find((entry) => entry.id === command.parameterId);
				if (!parameter) return null;
				const removedBindings = projection.parameterBindings.filter((binding) => binding.parameterId === command.parameterId);
				const forwards: OsOperation[] = [{ op: "removeParameter", payload: { parameterId: command.parameterId } }];
				const backwards: OsOperation[] = [
					{ op: "addParameter", payload: { parameter } },
					...removedBindings.map((binding) => ({ op: "bindParameterField" as const, payload: { binding } })),
				];
				return { id: createOsId("change"), forwards, backwards };
			}
			case "patchParameter": {
				const projection = this.projection();
				const current = projection.parameters.find((entry) => entry.id === command.parameterId);
				if (!current) return null;
				const next = patchOsParameter(current, command.patch);
				return {
					id: createOsId("change"),
					forwards: [{ op: "patchParameter", payload: { parameterId: command.parameterId, parameter: next } }],
					backwards: [{ op: "patchParameter", payload: { parameterId: command.parameterId, parameter: current } }],
				};
			}
			case "bindParameterField": {
				const projection = this.projection();
				const parameter = projection.parameters.find((entry) => entry.id === command.parameterId);
				if (!parameter) return null;
				const binding: OsParameterFieldBinding = {
					parameterId: command.parameterId,
					instanceId: command.instanceId,
					fieldPath: command.fieldPath,
				};
				const previous = projection.parameterBindings.find(
					(entry) => entry.instanceId === command.instanceId && entry.fieldPath === command.fieldPath,
				);
				const forwards: OsOperation[] = [{ op: "bindParameterField", payload: { binding } }];
				const backwards: OsOperation[] = previous
					? [{ op: "bindParameterField", payload: { binding: previous } }]
					: [{ op: "unbindParameterField", payload: { instanceId: command.instanceId, fieldPath: command.fieldPath } }];
				return { id: createOsId("change"), forwards, backwards };
			}
			case "unbindParameterField": {
				const projection = this.projection();
				const previous = projection.parameterBindings.find(
					(entry) => entry.instanceId === command.instanceId && entry.fieldPath === command.fieldPath,
				);
				if (!previous) return null;
				return {
					id: createOsId("change"),
					forwards: [{ op: "unbindParameterField", payload: { instanceId: command.instanceId, fieldPath: command.fieldPath } }],
					backwards: [{ op: "bindParameterField", payload: { binding: previous } }],
				};
			}
		}
	}
}
//#endregion 🔖OsStore

/** @emoji 🌳 Builds branch-lane history columns for studio VCS checkpoints. */
export function buildOsHistoryColumns(document: OsDocument): readonly HistoryColumn[] {
	const envelope: DocumentVcsEnvelope<OsProjection, OsOperation> = {
		schema: document.schema,
		id: document.id,
		vcs: {
			initialProjection: document.vcs.initialProjection,
			edits: [],
			changes: document.vcs.operations.map((operation) => ({
				id: operation.id,
				editIds: [operation.id],
				description: operation.description,
				savedAt: operation.savedAt ?? new Date().toISOString(),
			})),
			checkpoints: document.vcs.checkpoints.map((checkpoint) => ({
				id: checkpoint.id,
				changeIds: [...checkpoint.changeIds],
				parentId: checkpoint.parentId,
				authors: checkpoint.authors ? [...checkpoint.authors] : [],
				message: checkpoint.message,
				timestamp: checkpoint.savedAt,
			})),
			alternatives: document.vcs.alternatives.map((alternative) => ({
				id: alternative.id,
				name: alternative.name,
				checkpointIds: [...alternative.checkpointIds],
			})),
		},
		backbone: document.backbone,
	};
	return buildHistoryColumns(envelope);
}

//#region 🔖OsBackbone
export type OsBackboneKind = "dev" | "local" | "remote";

export interface OsBackbone {
	readonly kind: OsBackboneKind;
	attach(uri: string): void;
	detach?(): void;
	status(): { readonly attachedUri: string | null; readonly kind: OsBackboneKind; readonly conflict?: OsConflict | null };
	loadAttached(): OsDocument | null;
	sync(document: OsDocument): string;
	subscribe?(listener: (document: OsDocument) => void): () => void;
}

export interface PendingOsOp {
	readonly changeId: string;
	readonly change: OsChange;
	readonly baseVersion: number;
}

//#region 🔖DevJsonBackbone
export interface OsBackbonePort {
	readonly read: (uri: string) => string | null;
	readonly write: (uri: string, json: string) => void;
}

const defaultBrowserBackbonePort: OsBackbonePort = {
	read(uri) {
		if (typeof localStorage === "undefined") return null;
		return localStorage.getItem(`s:backbone:${uri}`);
	},
	write(uri, json) {
		if (typeof localStorage === "undefined") return;
		localStorage.setItem(`s:backbone:${uri}`, json);
	},
};

export class DevJsonBackbone implements OsBackbone {
	readonly kind = "dev" as const;
	private uri: string | null = null;
	private readonly port: OsBackbonePort;
	private channel: BroadcastChannel | null = null;
	private readonly remoteListeners = new Set<(document: OsDocument) => void>();

	constructor(port: OsBackbonePort = defaultBrowserBackbonePort) {
		this.port = port;
	}

	attach(uri: string): void {
		this.uri = uri;
		if (typeof BroadcastChannel === "undefined") return;
		this.channel?.close();
		this.channel = new BroadcastChannel(`os-backbone:${uri}`);
		this.channel.onmessage = (event: MessageEvent<string>) => {
			const json = typeof event.data === "string" ? event.data : null;
			if (!json) return;
			try {
				const document = this.load(json);
				for (const listener of this.remoteListeners) listener(document);
			} catch {
				return;
			}
		};
	}

	detach(): void {
		this.channel?.close();
		this.channel = null;
		this.uri = null;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "dev" } {
		return { attachedUri: this.uri, kind: "dev" };
	}

	subscribe(listener: (document: OsDocument) => void): () => void {
		this.remoteListeners.add(listener);
		return () => this.remoteListeners.delete(listener);
	}

	load(json: string): OsDocument {
		return osDocumentFromJson(json);
	}

	loadAttached(): OsDocument | null {
		if (!this.uri) return null;
		const json = this.port.read(this.uri);
		if (!json) return null;
		return this.load(json);
	}

	sync(document: OsDocument): string {
		const synced =
			this.uri != null ? { ...document, backbone: { kind: "dev" as const, uri: this.uri } } : document;
		const json = osDocumentToJson(synced);
		if (this.uri) {
			this.port.write(this.uri, json);
			this.channel?.postMessage(json);
		}
		return json;
	}
}

/** @emoji 💾 Local-first backbone stub (`local://`) mirroring dev JSON port shape. */
export class LocalJsonBackbone {
	private uri: string | null = null;
	private readonly port: OsBackbonePort;

	constructor(port: OsBackbonePort = defaultBrowserBackbonePort) {
		this.port = port;
	}

	attach(uri: string): void {
		if (!uri.startsWith("local://")) throw new Error(`expected local:// uri, got ${uri}`);
		this.uri = uri;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "local" } {
		return { attachedUri: this.uri, kind: "local" };
	}

	loadAttached(): OsDocument | null {
		if (!this.uri) return null;
		const json = this.port.read(this.uri);
		if (!json) return null;
		return osDocumentFromJson(json);
	}

	sync(document: OsDocument): string {
		const synced =
			this.uri != null ? { ...document, backbone: { kind: "local" as const, uri: this.uri } } : document;
		const json = osDocumentToJson(synced);
		if (this.uri) this.port.write(this.uri, json);
		return json;
	}
}

/** @emoji 🌐 Remote OS backbone (`remote://`) with REST push, WS pull, and conflict surfacing. */
export class RemoteOsBackbone implements OsBackbone {
	readonly kind = "remote" as const;
	private uri: string | null = null;
	private hubOrigin: string | null = null;
	private documentId: string | null = null;
	private cachedDocument: OsDocument | null = null;
	private version = 0;
	private lastConflict: OsConflict | null = null;
	private ws: WebSocket | null = null;
	private readonly remoteListeners = new Set<(document: OsDocument) => void>();
	private readonly pendingOps: PendingOsOp[] = [];
	private lastSyncedOpCount = 0;

	attach(uri: string): void {
		if (!uri.startsWith("remote://")) throw new Error(`expected remote:// uri, got ${uri}`);
		this.uri = uri;
		const parsed = new URL(uri.replace("remote://", "http://"));
		this.hubOrigin = `${parsed.protocol}//${parsed.host}`;
		this.documentId = parsed.pathname.replace(/^\//, "") || "default";
	}

	detach(): void {
		this.ws?.close();
		this.ws = null;
		this.uri = null;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "remote"; readonly conflict: OsConflict | null } {
		return { attachedUri: this.uri, kind: "remote", conflict: this.lastConflict };
	}

	subscribe(listener: (document: OsDocument) => void): () => void {
		this.remoteListeners.add(listener);
		return () => this.remoteListeners.delete(listener);
	}

	/** @emoji 📥 Loads snapshot + version from hub and opens the op stream. */
	async connect(): Promise<OsDocument | null> {
		if (!this.hubOrigin || !this.documentId) return null;
		const response = await fetch(`${this.hubOrigin}/documents/${encodeURIComponent(this.documentId)}`);
		if (!response.ok) throw new Error(`remote load failed: ${response.status}`);
		const payload = (await response.json()) as { snapshot: OsDocument; version: number };
		this.cachedDocument = payload.snapshot;
		this.version = payload.version;
		this.lastSyncedOpCount = payload.snapshot.vcs.operations.length;
		this.openWebSocket();
		return this.cachedDocument;
	}

	loadAttached(): OsDocument | null {
		return this.cachedDocument;
	}

	sync(document: OsDocument): string {
		if (!this.hubOrigin || !this.documentId) return osDocumentToJson(document);
		const synced = { ...document, backbone: { kind: "remote" as const, uri: this.uri! } };
		const json = osDocumentToJson(synced);
		this.cachedDocument = synced;
		const newOps = synced.vcs.operations.slice(this.lastSyncedOpCount);
		for (const change of newOps) {
			this.pendingOps.push({ changeId: change.id, change, baseVersion: this.version });
		}
		this.lastSyncedOpCount = synced.vcs.operations.length;
		void this.flushPendingOps();
		return json;
	}

	getPendingOps(): readonly PendingOsOp[] {
		return this.pendingOps;
	}

	clearConflict(): void {
		this.lastConflict = null;
	}

	private openWebSocket(): void {
		if (!this.hubOrigin || !this.documentId || typeof WebSocket === "undefined") return;
		this.ws?.close();
		const wsUrl = `${this.hubOrigin.replace(/^http/u, "ws")}/documents/${encodeURIComponent(this.documentId)}/ws`;
		this.ws = new WebSocket(wsUrl);
		this.ws.onmessage = (event) => {
			try {
				const message = JSON.parse(String(event.data)) as { kind: string; change?: OsChange; version?: number };
				if (message.kind === "op" && message.change) {
					this.version = message.version ?? this.version + 1;
					if (this.cachedDocument) {
						this.cachedDocument = {
							...this.cachedDocument,
							vcs: { ...this.cachedDocument.vcs, operations: [...this.cachedDocument.vcs.operations, message.change] },
						};
						this.lastSyncedOpCount = this.cachedDocument.vcs.operations.length;
						for (const listener of this.remoteListeners) listener(this.cachedDocument);
					}
				}
			} catch {
				return;
			}
		};
	}

	private async flushPendingOps(): Promise<void> {
		if (!this.hubOrigin || !this.documentId) return;
		while (this.pendingOps.length > 0) {
			const pending = this.pendingOps[0]!;
			const response = await fetch(`${this.hubOrigin}/documents/${encodeURIComponent(this.documentId)}/ops`, {
				method: "POST",
				headers: { "content-type": "application/json" },
				body: JSON.stringify({ version: pending.baseVersion, change: pending.change }),
			});
			if (response.status === 409) {
				this.lastConflict = {
					kind: "os-conflict",
					uri: this.uri ?? "remote://unknown",
					message: "remote version conflict",
					remoteRevision: String(pending.baseVersion),
				};
				return;
			}
			if (!response.ok) throw new Error(`remote push failed: ${response.status}`);
			const payload = (await response.json()) as { version: number };
			this.version = payload.version;
			this.pendingOps.shift();
			this.lastConflict = null;
		}
	}
}
//#endregion 🔖DevJsonBackbone

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("os-core", () => {
		it("spawns and removes app instances", () => {
			mergeOsProgramDefinition("draw", {
				id: "draw",
				name: "Draw",
				apiVersion: "1",
				apps: [{ id: "draw", label: "Draw", controllerId: "draw-play", modes: [{ id: "edit", label: "Edit" }] }],
				createPlatformApi: () => ({}),
			}, {
				draw: { inputs: [], outputs: [osOutPort("2d.drawing")], sourceFormat: "draw.document", componentKind: "draw", modes: [{ id: "edit", label: "Edit" }] },
			});
			const store = new OsStore(createEmptyOsDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw" });
			expect(store.projection().appInstances).toHaveLength(1);
			store.dispatch({ kind: "undo" });
			expect(store.projection().appInstances).toHaveLength(0);
		});

		it("validates media graph cycles", () => {
			const graph = emptyMediaGraph();
			expect(validateMediaGraph(graph).ok).toBe(true);
		});

		it("assertOsMediaExportCoverage lists missing handlers", () => {
			expect(() => assertOsMediaExportCoverage()).toThrow(/missing os media export handlers/);
		});

		it("resolves app definitions for embedded instances", () => {
			mergeOsProgramDefinition("writer", {
				id: "writer",
				name: "Writer",
				apiVersion: "1",
				apps: [{ id: "writer", label: "Writer", controllerId: "writer-play", modes: [{ id: "edit", label: "Edit" }] }],
				createPlatformApi: () => ({}),
			}, {
				writer: { inputs: [], outputs: [osOutPort("writer.document")], sourceFormat: "writer.document", componentKind: "panel", modes: [{ id: "edit", label: "Edit" }] },
			});
			const store = new OsStore(createEmptyOsDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "writer", appId: "writer" });
			const instance = store.projection().appInstances[0]!;
			expect(resolveOsAppDefinition(instance)?.controllerId).toBe("writer-play");
		});

		it("buildOsHistoryColumns maps checkpoint authors", () => {
			const store = new OsStore(createEmptyOsDocument());
			store.dispatch({ kind: "commitCheckpoint", message: "root", authors: [{ id: "dev", name: "Dev" }] });
			const columns = buildOsHistoryColumns(store.getDocument());
			expect(columns.length).toBe(1);
			expect(columns[0]?.authors[0]?.name).toBe("Dev");
		});

		it("adds patches and removes studio parameters with undo", () => {
			const store = new OsStore(createEmptyOsDocument());
			store.dispatch({ kind: "addParameter", type: "numeric", name: "Zoom" });
			const parameter = store.projection().parameters[0]!;
			expect(parameter.name).toBe("Zoom");
			store.dispatch({ kind: "patchParameter", parameterId: parameter.id, patch: { value: 12, max: 10 } });
			const patched = store.projection().parameters[0]!;
			expect(patched.type).toBe("numeric");
			if (patched.type === "numeric") expect(patched.value).toBe(10);
			store.dispatch({ kind: "undo" });
			expect(store.projection().parameters[0]?.name).toBe("Zoom");
			store.dispatch({ kind: "removeParameter", parameterId: parameter.id });
			expect(store.projection().parameters).toHaveLength(0);
		});

		it("binds parameter fields to app input channels", () => {
			mergeOsProgramDefinition(
				"raster",
				{
					id: "raster",
					name: "Raster",
					apiVersion: "1",
					apps: [{ id: "raster", label: "Raster", controllerId: "raster-play", modes: [{ id: "edit", label: "Edit" }] }],
					createPlatformApi: () => ({}),
				},
				{
					raster: {
						inputs: [],
						outputs: [osOutPort("2d.raster")],
						sourceFormat: "raster.document",
						componentKind: "raster",
						modes: [{ id: "edit", label: "Edit" }],
						parameterFields: [{ fieldPath: "/brushSize", label: "Brush size", type: "numeric" }],
					},
				},
			);
			const store = new OsStore(createEmptyOsDocument());
			store.dispatch({ kind: "addParameter", type: "numeric", name: "Brush" });
			store.dispatch({ kind: "spawnAppInstance", programId: "raster", appId: "raster" });
			const projection = store.projection();
			const parameter = projection.parameters[0]!;
			const instance = projection.appInstances[0]!;
			store.dispatch({
				kind: "bindParameterField",
				instanceId: instance.id,
				fieldPath: "/brushSize",
				parameterId: parameter.id,
			});
			const node = store.projection().mediaGraph.nodes.find((entry) => entry.instanceId === instance.id);
			expect(node?.inputs.some((port) => isParameterPortId(port.id))).toBe(true);
			const values = resolveParameterValuesForInstance(store.projection().parameterBindings, store.projection().parameters, instance.id);
			expect(values["/brushSize"]).toBe(0);
		});

		it("applies json pointer parameter overrides during materialization", () => {
			const projection = {
				brushSize: 8,
			};
			const overridden = applyParameterValuesToProjection(
				projection,
				[{ parameterId: "p1", instanceId: "i1", fieldPath: "/brushSize" }],
				[{ id: "p1", name: "Brush", type: "numeric", value: 42 }],
				"i1",
			);
			expect((overridden as { brushSize: number }).brushSize).toBe(42);
		});
	});
}
// #endregion 🧪Tests
