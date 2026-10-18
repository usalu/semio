/** 🪢️ Declares the owned revisioned component graph API while component modules stay untrusted DATA. */
export interface ComponentRosterEntryV1 { readonly name: string; readonly path: string; readonly needs: readonly string[] }
export interface ComponentRosterV1 { readonly version: 1; readonly components: readonly { readonly name: string; readonly path: string; readonly needs: readonly string[] }[] }
export interface RevisionIdentityV1 { readonly url: string; readonly revision: string; bind(revision: string): void }
export interface ComponentModuleV1 { readonly component: string; wire(registry: Readonly<Record<string, unknown>>): void }
export type ComponentModulesV1 = Readonly<Record<string, ComponentModuleV1>>;
export interface WiredRevisionV1 { readonly modules: ComponentModulesV1 }
export type ComponentRegisterV1 = (analysis: Readonly<{ id: string; revision: string; analyze: (path: string, bytes: Uint8Array) => unknown }>) => unknown;
export function readComponentRoster(manifest: unknown): readonly ComponentRosterEntryV1[];
export function registryKey(name: string): string;
export function createRevisionIdentity(url: string): RevisionIdentityV1;
export function importRevisionedComponents(options: Readonly<{ roster: readonly ComponentRosterEntryV1[]; revision: string; load: (specifier: string) => Promise<ComponentModuleV1> }>): Promise<ComponentModulesV1>;
export function wireRevisionedComponents(options: Readonly<{ roster: readonly ComponentRosterEntryV1[]; modules: ComponentModulesV1; services: unknown; revisionIdentity: RevisionIdentityV1; register: ComponentRegisterV1 }>): WiredRevisionV1;
export function createRevisionLoader(options: Readonly<{ roster: readonly ComponentRosterEntryV1[]; load: (specifier: string) => Promise<ComponentModuleV1>; services: unknown; register: ComponentRegisterV1; url: string }>): Readonly<{ adopt(revision: string, scope: WiredRevisionV1): void; load(revision: string): Promise<WiredRevisionV1> }>;
