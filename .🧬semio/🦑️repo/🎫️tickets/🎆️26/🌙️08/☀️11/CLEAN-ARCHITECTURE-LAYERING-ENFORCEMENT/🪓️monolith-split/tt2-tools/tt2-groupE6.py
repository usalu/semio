from tt2_edit import *
edit(DRAW+'../../../../../🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts'.replace('../../../../../','')[0:0]+ROOT[0:0] if False else ROOT+'✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts',[
 ('let state={pages:law.lanes.map(','let state:Record<string,any>={pages:law.lanes.map('),
])
edit(ANY+'✏️editor/🎮️commands/🩹️patch-layer/🧪️tests/🔬️unit/🟦️.ts',[
 ('type FixtureLayer={readonly id:string;readonly kind:string;readonly children?:readonly FixtureLayer[]};','type FixtureLayer={readonly id:string;readonly kind:string;readonly children?:readonly (FixtureLayer|string)[]};'),
 ('if(layer.kind==="group")visit(layer.children??[]);','if(layer.kind==="group")visit((layer.children??[]).filter((child):child is FixtureLayer=>typeof child!=="string"));'),
])
