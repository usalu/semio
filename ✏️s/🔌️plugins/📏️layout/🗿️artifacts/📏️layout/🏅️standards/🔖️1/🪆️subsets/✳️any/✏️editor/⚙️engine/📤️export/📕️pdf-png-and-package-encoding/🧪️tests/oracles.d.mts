import type {PdfRasterSource,PdfRasterPlane} from '../🖼️raster/🟦️.ts';
/** 🔮️ Independent PDF and image observations use owned argument and result types. */
export function rasterPlane(source:PdfRasterSource,plane:PdfRasterPlane):Promise<Uint8Array>;
export function unicodeText(unicode:string,encoding:string,encoded:string):Promise<string>;
export function markedText(unicode:string,encoding:string,encoded:string,scope:string):Promise<{text:string;tags:string[]}>;
export function fontFamilyHex(family:string):string;
