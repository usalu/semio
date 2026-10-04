import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const product='C:/git/semio/🧰️framework/🛍️products/📓️print/',temporary=import.meta.dir+'/source.tmp';
function edit(path:string,change:(source:string)=>string){const file=product+path;writeFileSync(temporary,change(readFileSync(file,'utf8')));renameSync(temporary,file);}
edit('🧬️schema/💡️inferences/🖼️render/🟦️.ts',source=>{
  source=source.replace('translateCommands(path.commands,col+swatch/2,row+swatch/2)','translateCommands(path.commands,col+swatch/2,height-row-swatch/2)');
  const start=source.indexOf('function legendGuideItems'),end=source.indexOf('function scalesOf',start),part=source.slice(start,end).replace('  return o.style===undefined?items:items.map(item=>({...item,tikzStyle:String(o.style)}));',`  const projected=items.map(item=>item.kind==="rect"?{...item,y:height-item.y-item.height}:item.kind==="circle"?{...item,cy:height-item.cy}:item.kind==="line"?{...item,y1:height-item.y1,y2:height-item.y2}:item.kind==="text"?{...item,y:height-item.y}:item);
  return o.style===undefined?projected:projected.map(item=>({...item,tikzStyle:String(o.style)}));`);
  return source.slice(0,start)+part+source.slice(end);
});
edit('🧪️tests/🧬️native-chart-grammar/🟦️.ts',source=>source.replace('[first.x,first.y,Number(o.length??24),first.height]','[first.x,legendVectors.height-first.y-first.height,Number(o.length??24),first.height]').replace('[item.x,item.y+.5]','[item.x,legendVectors.height-item.y+.5]').replace('[item.cx,item.cy,item.r]','[item.cx,legendVectors.height-item.cy,item.r]').replace('[item.x-swatch-labelGap,item.y-swatch/2]','[item.x-swatch-labelGap,legendVectors.height-item.y-swatch/2]'));
edit('🧪️tests/🎬️render-scene/🔣️legend.json',source=>source.replaceAll('"positions"','"nativeOrigins"'));
edit('🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts',source=>source.replaceAll('positions','nativeOrigins'));
