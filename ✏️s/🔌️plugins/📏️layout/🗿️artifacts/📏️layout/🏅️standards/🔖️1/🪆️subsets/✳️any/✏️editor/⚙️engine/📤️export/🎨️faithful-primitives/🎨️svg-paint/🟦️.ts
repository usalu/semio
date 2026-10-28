/** 🎨️ Layout SVG presentation attributes preserve the model's finite binary32 sRGB channels. */
export type SvgPaintSlot='fill'|'stroke';
export type SvgColor=readonly[number,number,number,number];
function decimal(value:number):string{const text=String(value);if(!text.includes('e'))return text;const[coefficient,exponent]=text.split('e'),digits=coefficient!.replace('.',''),zeroes=-Number(exponent)-1;return'0.'+'0'.repeat(zeroes)+digits;}
export function svgPaintAttributes(slot:SvgPaintSlot,color:SvgColor|null):string{
 if(slot!=='fill'&&slot!=='stroke')throw Error('layout-export-svg-paint-slot');
 if(color===null)return slot+'="none"';
 if(color.length!==4||!color.every(value=>Number.isFinite(value)&&Number.isFinite(Math.fround(value))))throw Error('layout-export-svg-paint-color');
 const red=Math.max(0,Math.min(1,Math.fround(color[0]))),green=Math.max(0,Math.min(1,Math.fround(color[1]))),blue=Math.max(0,Math.min(1,Math.fround(color[2]))),alpha=Math.max(0,Math.min(1,Math.fround(color[3])));
 return slot+'="rgb('+decimal(red*100)+'%,'+decimal(green*100)+'%,'+decimal(blue*100)+'%)" '+slot+'-opacity="'+decimal(alpha)+'"';
}
