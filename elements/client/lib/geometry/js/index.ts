import createTopologicKernelModule from "../wasm/generated/topologic-kernel.js";

export interface TopologicJsBindings {
	readonly parseFixture: (raw: unknown) => unknown;
	readonly vertexPoint: (fixture: unknown, id: string) => unknown;
	readonly edgeCurve: (fixture: unknown, id: string) => unknown;
	readonly updateFixtureTransform: (fixture: unknown, entityId: string, transform: unknown) => unknown;
}

type TopologicKernelModule = TopologicJsBindings;

let bindings: TopologicJsBindings | null = null;
let bindingsPromise: Promise<TopologicJsBindings> | null = null;

export function getTopologicJsBindings(): TopologicJsBindings {
	if (!bindings) {
		throw new Error("Topologic wasm module is not loaded yet. Call ensureTopologicJsBindingsLoaded() before using synchronous geometry helpers.");
	}
	return bindings;
}

export async function ensureTopologicJsBindingsLoaded(): Promise<TopologicJsBindings> {
	bindingsPromise ??= Promise.resolve(createTopologicKernelModule() as TopologicKernelModule | Promise<TopologicKernelModule>).then((module) => {
		bindings = module;
		return module;
	});
	return bindingsPromise;
}