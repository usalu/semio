/** WASM facade — parse/print delegates to the plugin Rust crate. */
export function parseDsl(text: string): unknown {
  throw new Error("wire to plugin WASM");
}
export function printDsl(value: unknown): string {
  throw new Error("wire to plugin WASM");
}
