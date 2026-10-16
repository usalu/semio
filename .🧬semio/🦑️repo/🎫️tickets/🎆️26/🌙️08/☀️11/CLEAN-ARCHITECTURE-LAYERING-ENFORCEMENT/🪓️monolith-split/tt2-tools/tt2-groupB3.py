from tt2_edit import *
FX='"../../../../../../🧪️tests/🧫️fixtures/🟦️.ts"'
edit(ANY+'🧬️schema/🎬️scene/📷️raster/🧪️tests/🔬️unit/🖼️assets/🟦️.ts',[
 ('import {RasterSceneJob,rasterizeScene,type RasterSceneInput} from "../../../🟦️.ts";\n','import {RasterSceneJob,rasterizeScene,type RasterSceneInput} from "../../../🟦️.ts";\nimport {admitted,hostile} from '+FX+';\n'),
 ('const ajv=new Ajv({strict:true}).addMetaSchema(draft7);','type RasterSceneFixture=Omit<RasterSceneInput,"assets">&{readonly assets:readonly {readonly id:string;readonly image:{readonly width:number;readonly height:number;readonly pixels:readonly number[]}}[]};\nconst ajv=new Ajv({strict:true}).addMetaSchema(draft7);'),
 ('const validate=ajv.compile(schema),validateProgress','const validate=ajv.compile<RasterSceneFixture>(schema),validateProgress'),
 ('const complete=(value:RasterSceneInput,grant=4096)=>','const sceneInput=(value:unknown):RasterSceneInput=>{const fixture=admitted(validate,value,"raster scene input");return {...fixture,assets:fixture.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))};};\nconst complete=(value:RasterSceneInput,grant=4096)=>'),
 ('const value={...row.input,assets:row.input.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,before=structuredClone(value);\n for(const grant','const value=sceneInput(row.input),before=structuredClone(value);\n for(const grant'),
 ('if("expectedAllocatedPixels" in row)expect(result.progress.pixels).toBe(row.expectedAllocatedPixels);','if("expectedAllocatedPixels" in row)expect<unknown>(result.progress.pixels).toBe(row.expectedAllocatedPixels);'),
 ('new RasterSceneJob(row.input as RasterSceneInput)','new RasterSceneJob(hostile<RasterSceneInput>(row.input))'),
 ('const original=cases[5]!.input,value={...original,assets:original.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,probe=','const value=sceneInput(cases[5]!.input),probe='),
 ('const unused={...cases[0]!.input,nodes:[],assets:[{id:"é😀",image:{width:1,height:1,pixels:new Uint8Array([255,0,0,255])}}],maxSourceBytes:4} as RasterSceneInput;','const unused:RasterSceneInput={...sceneInput(cases[0]!.input),nodes:[],assets:[{id:"é😀",image:{width:1,height:1,pixels:new Uint8Array([255,0,0,255])}}],maxSourceBytes:4};'),
 ('const original=cases[cases.length-1]!.input,base={...original,assets:original.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput;','const base=sceneInput(cases[cases.length-1]!.input);'),
 ('const row=cases[31]!,value={...row.input,assets:row.input.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,before=','const row=cases[31]!,value=sceneInput(row.input),before='),
])
