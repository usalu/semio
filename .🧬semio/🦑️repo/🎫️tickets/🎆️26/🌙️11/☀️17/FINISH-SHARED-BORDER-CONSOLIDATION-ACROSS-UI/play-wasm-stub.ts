const none = (): never => { throw new Error("wasm engines are not built in the border review host"); };
export default async function init(): Promise<void> {}
export const initSync = (): void => {};
export const __wbg_init = init;
export const Engine = none;
export const version = (): string => "stub";
