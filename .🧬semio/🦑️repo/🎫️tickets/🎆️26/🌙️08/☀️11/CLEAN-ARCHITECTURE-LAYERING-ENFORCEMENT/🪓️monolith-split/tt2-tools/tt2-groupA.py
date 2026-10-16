from tt2_edit import *
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
# export-document
edit(ANY+'✏️editor/🎮️commands/📤️export-document/🧪️tests/🔬️unit/🟦️.ts',[
 ('const lift=(value:any):any=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;\n',''),
 ('import {binary64} from "@semio-tech/framework/value/ieee754";\n','import {binary64} from "@semio-tech/framework/value/ieee754";\nimport {drawingArtifactFixture,drawingLayersFixture} from '+FX+';\n'),
 ('return{row,document:{...structuredClone(row.document),layers:lift(row.document.layers),artboard:{width:binary64(2),height:binary64(1)}}};','return{row,document:{...drawingArtifactFixture(row.document),artboard:{width:binary64(2),height:binary64(1)}}};'),
 ('document.layers=lift(sample.layers);','document.layers=drawingLayersFixture(sample.layers);'),
])
# edit-selection
edit(ANY+'✏️editor/🎮️commands/🎛️edit-selection/🧪️tests/🔬️unit/🟦️.ts',[
 ('expect(result).toEqual(item.segments);','expect<unknown>(result).toEqual(item.segments);'),
 ('if ("bounds" in item) expect(output.map(layer => [layer.x,layer.y,layer.width,layer.height])).toEqual(item.bounds);','if ("bounds" in item) expect<unknown>(output.map(layer => [layer.x,layer.y,layer.width,layer.height])).toEqual(item.bounds);'),
 ('if ("rootCount" in item) expect(output.length).toBe(item.rootCount);','if ("rootCount" in item) expect<unknown>(output.length).toBe(item.rootCount);'),
 ('if ("order" in item) expect(output.map(layer => layer.id)).toEqual(item.order);','if ("order" in item) expect<unknown>(output.map(layer => layer.id)).toEqual(item.order);'),
 ('expect(actual).toEqual(row.deltas);expect(row.bounds).toEqual(saved);','expect<unknown>(actual).toEqual(row.deltas);expect(row.bounds).toEqual(saved);'),
 ('expect(layer.visible).toBe(!row.hidden!.includes(id));','expect(layer.visible).toBe(!new Set<string>(row.hidden!).has(id));'),
])
# add-layer
edit(ANY+'✏️editor/🎮️commands/➕️add-layer/🧪️tests/🔬️unit/🟦️.ts',[
 ('const before={id:"new",name:"Untitled",attributes:{fill:null,stroke:null} as unknown};','const before:{id:string;name:string;attributes:unknown}={id:"new",name:"Untitled",attributes:{fill:null,stroke:null}};'),
 ('expect({...before,attributes:initialAppearance(entry.kind)}).toEqual(expected);','expect<typeof before>({...before,attributes:initialAppearance(entry.kind)}).toEqual(expected);'),
])
