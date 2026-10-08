import type { InventoryProject } from "./⚡️caching/📇️inventory/🟦️.ts";

export type GeneratedNodes = readonly (readonly [string, { readonly projects: Readonly<Record<string, InventoryProject>> }])[];
export interface ProjectNodeProvider { readonly name: string; readonly createNodesV2: readonly [string, (files: readonly string[], options: unknown, context: { readonly workspaceRoot: string }) => Promise<GeneratedNodes>]; readonly createDependencies: typeof createDependencies }
export function createDependencies(...args: readonly unknown[]): Promise<unknown>;
export function dependencyResolutionAuthority(...args: readonly unknown[]): Promise<string>;
export function nativeSourceWatchPlanV1(root: string, workspaceRoot: string): Promise<unknown>;
export function nativeOwnerExecutionRoute(target: unknown): unknown;
export const libraryBootstrap: Promise<void>;
export const cacheInternals: Readonly<Record<string, unknown>>;
declare const plugin: ProjectNodeProvider;
export default plugin;
