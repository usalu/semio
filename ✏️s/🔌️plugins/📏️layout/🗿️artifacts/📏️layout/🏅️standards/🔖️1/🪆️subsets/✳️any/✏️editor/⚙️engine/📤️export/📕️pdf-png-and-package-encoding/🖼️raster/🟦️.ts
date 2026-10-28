import {OutputWork} from '../../📤️output/🟦️.ts';
export interface PdfRasterSource { width: number; height: number; rgba: Uint8Array }
export type PdfRasterPlane = 'rgb' | 'alpha';

/** 📏️ Validates one retained image using constant work without copying its pixels. */
export function pdfRasterLength(source: PdfRasterSource, plane: PdfRasterPlane): number {
  if (!Number.isInteger(source.width) || !Number.isInteger(source.height) || source.width <= 0 || source.height <= 0 || source.width > 0xffffffff || source.height > 0xffffffff) throw new Error('layout-export-pdf-raster-dimensions');
  const pixels = source.width * source.height;
  if (!Number.isSafeInteger(pixels * 4) || source.rgba.length !== pixels * 4) throw new Error('layout-export-pdf-raster-bytes');
  if (plane !== 'rgb' && plane !== 'alpha') throw new Error('layout-export-pdf-raster-plane');
  return pixels * (plane === 'rgb' ? 3 : 1);
}
/** 🖼️ Extracts one admitted plane chunk into caller-owned backing without a transient byte owner. */
export function pdfRasterChunk(source:PdfRasterSource,plane:PdfRasterPlane,cursor:number,target:Uint8Array,work:OutputWork):number{const length=pdfRasterLength(source,plane);if(!Number.isSafeInteger(cursor)||cursor<0||cursor>length||target.length===0||target.length>4096)throw Error('layout-export-pdf-raster-turn');const take=Math.min(target.length,length-cursor);if(take===0)return 0;work.copy(take+24,1);for(let index=0;index<take;index++){const at=cursor+index;target[index]=source.rgba[plane==='rgb'?Math.floor(at/3)*4+at%3:at*4+3]!;}return take;}
