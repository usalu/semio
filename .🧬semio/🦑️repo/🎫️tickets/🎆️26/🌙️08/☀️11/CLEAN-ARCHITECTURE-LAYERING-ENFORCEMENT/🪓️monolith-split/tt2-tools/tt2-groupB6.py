from tt2_edit import *
S=ANY+'🧬️schema/🎬️scene/'
FX='"../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(S+'👁️view/🧪️tests/🔬️unit/🟦️.ts',[
 ('import {binary64} from "@semio-tech/framework/value/ieee754";\n','import {binary64} from "@semio-tech/framework/value/ieee754";\nimport {admitted} from '+FX+';\n'),
 ('expect(records).toEqual(sample.expected);','expect<unknown>(records).toEqual(sample.expected);'),
 ('const ajv=new Ajv({strict:true});const validate=ajv.compile(schema);\n for(const row of boundCases){expect(validate([row.record])).toBe(true);const node=row.record as PreparedSceneNode,bounds=','const ajv=new Ajv({strict:true});const validate=ajv.compile<PreparedSceneNode[]>(schema);\n for(const row of boundCases){expect(validate([row.record])).toBe(true);const node=admitted(validate,[row.record],"prepared scene node")[0]!,bounds='),
 ('let previous:[number,number]=[0,0];for(const segment of node.segments)','let previous:readonly [number,number]=[0,0];for(const segment of node.segments)'),
 ('const hidden={...boundCases[0]!.record,visible:false} as PreparedSceneNode;','const hidden:PreparedSceneNode={...admitted(validate,[boundCases[0]!.record],"prepared scene node")[0]!,visible:false};'),
 ('const source=structuredClone(row.document)as any;\n const visit=(layer:any)=>{if(sample.locked.includes(layer.id))layer.locked=true;if(sample.hidden.includes(layer.id))layer.visible=false;if(sample.zeroOpacity.includes(layer.id))layer.opacity=0;','const source=structuredClone(row.document)as any,flagged={locked:new Set<string>(sample.locked),hidden:new Set<string>(sample.hidden),zeroOpacity:new Set<string>(sample.zeroOpacity)};\n const visit=(layer:any)=>{if(flagged.locked.has(layer.id))layer.locked=true;if(flagged.hidden.has(layer.id))layer.visible=false;if(flagged.zeroOpacity.has(layer.id))layer.opacity=0;'),
 ('expect(bounds,sample.name).toEqual(sample.expected);expect(plan).toEqual(before);','expect<unknown>(bounds,sample.name).toEqual(sample.expected);expect(plan).toEqual(before);'),
])
