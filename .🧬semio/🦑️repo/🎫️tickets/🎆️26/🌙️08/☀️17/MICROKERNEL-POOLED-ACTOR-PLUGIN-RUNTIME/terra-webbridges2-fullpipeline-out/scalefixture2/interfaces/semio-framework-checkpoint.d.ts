/** @module Interface semio:framework/checkpoint@1.0.0 **/
export function checkpoint(): Promise<Uint8Array>;
export function restore(state: Uint8Array): Promise<void>;
export type PluginError = import('./semio-framework-types.js').PluginError;
