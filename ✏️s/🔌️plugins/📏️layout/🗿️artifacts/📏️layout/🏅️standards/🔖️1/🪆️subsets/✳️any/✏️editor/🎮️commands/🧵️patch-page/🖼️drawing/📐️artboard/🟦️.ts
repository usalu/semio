import{binary64,binary64Value}from'@semio-tech/framework/value/ieee754';
import type{DrawingArtboard}from'../../../../../../../../../../../../🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts';
/** 📐️ The real typed Drawing artboard contributes the untouched axis of a page field command. */
export function drawingArtboardForPageField(field:string,value:string,before:DrawingArtboard):DrawingArtboard{
 if(!/^[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?$/.test(value))throw Error('Page Drawing dimension is not a finite positive number');const next=Number(value),width=field==='pageDrawingWidth'?next:binary64Value(before.width),height=field==='pageDrawingHeight'?next:binary64Value(before.height);if(!['pageDrawingWidth','pageDrawingHeight'].includes(field)||!Number.isFinite(width)||width<=0||!Number.isFinite(height)||height<=0)throw Error('Page Drawing dimension requires the original positive artboard');return{width:binary64(width),height:binary64(height)};
}
