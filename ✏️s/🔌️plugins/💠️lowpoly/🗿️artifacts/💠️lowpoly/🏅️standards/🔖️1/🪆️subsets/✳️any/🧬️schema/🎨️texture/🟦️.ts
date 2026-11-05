import schema from"./🔣️.json";
const layout=schema.definitions.Layout.const;
export const PAINT_TEXTURE_WIDTH=layout.width;
export const PAINT_TEXTURE_HEIGHT=layout.height;
export const PAINT_TEXTURE_BYTES=layout.explicitBytes;
/** 🎨️ A document paint buffer has a declared canvas extent; malformed content never becomes a new canvas. */
export class PaintTextureError extends Error{
 readonly code="lowpoly.paint.invalid-texture";
 constructor(readonly actualBytes:number){super("Lowpoly paint pixels must be empty or exactly "+PAINT_TEXTURE_BYTES+" RGBA bytes");this.name="PaintTextureError";}
}
/** 🛂️ Validates decoded canonical bytes without modifying, padding, cropping or materializing them. */
export function validatePaintPixels(pixels:Uint8Array):void{if(pixels.byteLength!==0&&pixels.byteLength!==PAINT_TEXTURE_BYTES)throw new PaintTextureError(pixels.byteLength);}
