from tt2_edit import *
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(ANY+'✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🔬️unit/🟦️.ts',[
 ('import type {DrawingArtifact} from "../../../../../🧬️schema/🟦️.ts";\n',''),
 ('import {binary64} from "@semio-tech/framework/value/ieee754";\n\nfunction lift(value:unknown):unknown {\n if(typeof value==="number")return binary64(value);\n if(Array.isArray(value))return value.map(lift);\n return value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;\n}\n','import {drawingArtifactFixture} from '+FX+';\ntype FixtureLayer={readonly id:string;readonly kind:string;readonly children?:readonly FixtureLayer[]};\n'),
 ('const document={...fixture.document,layers:lift(fixture.document.layers)} as DrawingArtifact;','const document=drawingArtifactFixture(fixture.document);',3),
 ('const visit=(layers:typeof fixture.document.layers):void=>{for(const layer of layers){if(row.ids.includes(layer.id)||row.ids.includes("drawing-play-layers."+layer.kind+"."+layer.id))draft.push(layer.id);if("children" in layer&&layer.kind==="group")visit(layer.children as typeof fixture.document.layers);}};','const visit=(layers:readonly FixtureLayer[]):void=>{for(const layer of layers){if(row.ids.includes(layer.id)||row.ids.includes("drawing-play-layers."+layer.kind+"."+layer.id))draft.push(layer.id);if(layer.kind==="group")visit(layer.children??[]);}};'),
])
