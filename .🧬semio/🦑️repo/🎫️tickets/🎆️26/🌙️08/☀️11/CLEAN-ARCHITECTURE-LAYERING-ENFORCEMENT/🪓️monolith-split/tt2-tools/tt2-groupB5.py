from tt2_edit import *
UNI='import {unicodeSourceUrl} from "@semio-tech/framework/2d/text/font/shape/unicode";\n'
CAT='import {builtinFontLocations} from "@semio-tech/framework/2d/text/font/catalog";\n'
FONT_OLD='builtinFontLocations().map(source=>({family:source.family,id:source.id,bytes:new Uint8Array(readFileSync(source.url))})));'
FONT_NEW='builtinFontLocations().map(source=>({family:source.family,id:source.id,bytes:new Uint8Array(readFileSync(source.url))})),new Uint8Array(readFileSync(unicodeSourceUrl)));'
S=ANY+'🧬️schema/🎬️scene/'
edit(S+'🎨️paint/📋️prepare/🧪️tests/🔬️unit/🟦️.ts',[
 (CAT,CAT+UNI),
 ('producer.admitFontSources('+FONT_OLD,'producer.admitFontSources('+FONT_NEW),
 ('expect(output.geometry.map((g,index)=>({id:output.plan.nodes[index]!.id,kind:g.paint.kind}))).toEqual(row.expected);','expect<unknown>(output.geometry.map((g,index)=>({id:output.plan.nodes[index]!.id,kind:g.paint.kind}))).toEqual(row.expected);'),
])
edit(S+'📋️prepare/🧪️tests/🔬️unit/🟦️.ts',[
 (CAT,CAT+UNI),
 ('job.admitFontSources('+FONT_OLD,'job.admitFontSources('+FONT_NEW),
 ('let sum=0,previous=[0,0],origin=[0,0];','let sum=0,previous:readonly number[]=[0,0],origin:readonly number[]=[0,0];'),
 ('paths="paths" in oracle?oracle.paths:[oracle]','paths="paths" in oracle?oracle.paths!:[oracle]'),
])
edit(S+'🎨️paint/🧪️tests/🔬️unit/🟦️.ts',[
 ('expect(regions.bounds,row.name).toEqual(row.expected.bounds);','expect<unknown>(regions.bounds,row.name).toEqual(row.expected.bounds);'),
 ('if("bounds"in row)expect(regions.bounds).toEqual(row.bounds);','if("bounds"in row)expect<unknown>(regions.bounds).toEqual(row.bounds);'),
])
