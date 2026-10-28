/** 📚️ Canonical built-in font identities are independent of UI text-system registration. */
import catalog from "./📦️catalog/🔣️.json";
export type LayoutFontResourceReader=(path:string)=>Uint8Array|null;
export function layoutFontCount():number{return catalog.fonts.length;}
export function layoutFontSource(index:number,read:LayoutFontResourceReader):Uint8Array {if(!Number.isSafeInteger(index)||index<0||index>=catalog.fonts.length)throw Error("Layout font index is unavailable");const asset=catalog.fonts[index]!;const source=read(asset.path);if(!source||source.byteLength!==asset.size)throw Error("Layout builtin font source is unavailable");return source;}
