from tt2_edit import *
FX='"../../../../../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
LIFT='const lift=(value:any):any=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;\n'
B=ANY+'🚪️io/📤️export/🧵️serializers/🗿️artifacts/'
edit(B+'📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🟦️.ts',[
 (LIFT,''),
 ('import {binary64} from "@semio-tech/framework/value/ieee754";\n','import {drawingArtifactFixture,drawingLayersFixture} from '+FX+';\n'),
 ('document={...structuredClone(source.document),layers:lift(style?.layers??source.document.layers)}','document={...drawingArtifactFixture(source.document),layers:drawingLayersFixture(style?.layers??source.document.layers)}'),
 ('prepareDocumentVector({...structuredClone(source.document),layers:lift(source.document.layers)},','prepareDocumentVector(drawingArtifactFixture(source.document),',2),
 ('document={...structuredClone(source.document),layers:lift(style.layers)}','document={...drawingArtifactFixture(source.document),layers:drawingLayersFixture(style.layers)}'),
])
edit(B+'🎨️svg/🔖️1.1/✳️any/🧵️write/🧪️tests/🟦️.ts',[
 (LIFT,''),
 ('import {binary64} from "@semio-tech/framework/value/ieee754";\n','import {drawingArtifactFixture,drawingLayersFixture} from '+FX+';\n'),
 ('document={...structuredClone(source.document),layers:lift(style?.layers??source.document.layers)}','document={...drawingArtifactFixture(source.document),layers:drawingLayersFixture(style?.layers??source.document.layers)}'),
 ('prepareDocumentVector({...structuredClone(source.document),layers:lift(source.document.layers)},','prepareDocumentVector(drawingArtifactFixture(source.document),',2),
])
