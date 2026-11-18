/** 📌️ Admits the closed pointer to the newest generation. */
export function readGenerationPointerV1(bytes:Uint8Array):Readonly<Record<string,unknown>>;
/** 📜️ Admits one closed immutable generation manifest. */
export function readGenerationManifestV1(bytes:Uint8Array):Readonly<Record<string,any>>;
/** 🧩️ Admits one closed owner fragment. */
export function readGenerationFragmentV1(value:unknown):any;
