/* tslint:disable */
/* eslint-disable */

export function semioRendererBoot(canvas: HTMLCanvasElement, plugins: any, plugin_filter: string): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly semioRendererBoot: (a: any, b: any, c: number, d: number) => any;
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke___wasm_bindgen_b686e425c74afaa1___JsValue__core_d7816cce2541b107___result__Result_____wasm_bindgen_b686e425c74afaa1___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke___js_sys_f7175f2838118a8d___Function_fn_wasm_bindgen_b686e425c74afaa1___JsValue_____wasm_bindgen_b686e425c74afaa1___sys__Undefined___js_sys_f7175f2838118a8d___Function_fn_wasm_bindgen_b686e425c74afaa1___JsValue_____wasm_bindgen_b686e425c74afaa1___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke___wasm_bindgen_b686e425c74afaa1___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke___web_sys_bc78574726119a6d___features__gen_MouseEvent__MouseEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke___web_sys_bc78574726119a6d___features__gen_MouseEvent__MouseEvent______true__3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_b686e425c74afaa1___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
