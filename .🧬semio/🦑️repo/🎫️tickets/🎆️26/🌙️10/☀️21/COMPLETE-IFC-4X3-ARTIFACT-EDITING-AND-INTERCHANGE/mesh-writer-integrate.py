from pathlib import Path
g=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry')
def edit(name,change):
 p=g/name;s=p.read_text(encoding='utf-8');s=change(s);p.write_text(s,encoding='utf-8')
def replace(s,old,new):
 assert old in s,old[:100]
 return s.replace(old,new)
def convex(s):
 s=s.replace("type IfcConvexSolid=", "export type IfcConvexSolid=")
 s=s.replace("type IfcConvexSolid=", "type IfcConvexSolid=")
 s=replace(s,"type IfcConvexSolid=", "type IfcConvexSolid=")
 s=s.replace("import{convexProfilePoint", "import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';\nimport{convexProfilePoint")
 s=replace(s," const{shape,boundary,displacement,profile,frame,scale}=source,n=convexProfileSize(boundary),cone=shape==='cone';", " return yield* ifcConvexSolidWriteSteps(source,ifcArrayMeshWriterFactory(maxValues));\n}\n/** 🧭️ Stream the same indexed sweep into a pre-reserved endpoint using fixed precision scratch. */\nexport function* ifcConvexSolidWriteSteps<T>(source:IfcConvexSolid,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n const{shape,boundary,displacement,profile,frame,scale}=source,n=convexProfileSize(boundary),cone=shape==='cone';")
 start=s.index(' const values=cone?')
 end=s.index(' const triangle=',start)
 s=s[:start]+''' const vertexCount=cone?n+1:2*n,triangleCount=cone?2*n-2:4*n-4;
 const point=(ordinal:number):[number,number,number]=>{
  const layer=ordinal<n?0:1,index=ordinal<n?ordinal:ordinal-n,apex=cone&&layer===1,sample=apex?[0,0]:convexProfilePoint(boundary,index)!;
  const base=ifcTransform(profile,[sample[0]!,sample[1]!,0]),local=base.map((value,k)=>value+layer*displacement[k]!)as[number,number,number],result=ifcTransform(frame,local).map(value=>value*scale)as[number,number,number];
  if(result.some(value=>!Number.isFinite(value)))throw new IfcStepError('geometry.range','Convex solid coordinates exceed finite SI range');return result;
 };
 const writer=make(vertexCount,triangleCount);let failed=false,face=0;
 try{
 for(let index=0;index<vertexCount;index++){yield;const p=point(index);writer.point(index,p[0],p[1],p[2]);}
''' + s[end:]
 s=replace(s,'const origin=positions[a]!,edges=[b,c].map(index=>positions[index]!.map((value,k)=>value-origin[k]!));','const origin=point(a),edges=[b,c].map(index=>point(index).map((value,k)=>value-origin[k]!));')
 s=replace(s,'indices.push(a,b,c);','writer.triangle(face++,a,b,c);')
 s=replace(s,' return{positions,indices};',' return writer.finish();\n }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}')
 return s
edit('convex-solid.ts',convex)
def circular(s):
 s=s.replace("import{ifcConvexSolidSteps}","import{ifcConvexSolidWriteSteps}")
 s=s.replace("interface IfcCircularSolid{", "export interface IfcCircularSolid{")
 s=s.replace("interface IfcCircularSolid{", "interface IfcCircularSolid{")
 s=s.replace("import{ifcConvexSolidWriteSteps}","import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';\nimport{ifcConvexSolidWriteSteps}")
 s=replace(s,' const{shape,radius,displacement,profile,frame,scale}=source;'," return yield* ifcCircularSolidWriteSteps(source,policy,ifcArrayMeshWriterFactory(maxValues));\n}\n/** ⭕️ Delegate the admitted circle boundary directly to its original endpoint. */\nexport function* ifcCircularSolidWriteSteps<T>(source:IfcCircularSolid,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n const{shape,radius,displacement,profile,frame,scale}=source;")
 s=s.replace('ifcConvexSolidSteps(', 'ifcConvexSolidWriteSteps(').replace('profile,frame,scale},maxValues)', 'profile,frame,scale},make)')
 return s
edit('circular-solid.ts',circular)
def circle(s):
 s=s.replace('import {ifcCircularSolidSteps}', "import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';\nimport {ifcCircularSolidWriteSteps}")
 s=replace(s,' yield;const profile=graph.ref(solid.args[0]);'," return yield* ifcCircleExtrusionWriteSteps(graph,solid,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));\n}\n/** ⭕️ Author source circle rows directly into the pre-reserved endpoint. */\nexport function* ifcCircleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n yield;const profile=graph.ref(solid.args[0]);")
 s=s.replace('ifcCircularSolidSteps(', 'ifcCircularSolidWriteSteps(').replace('frame,scale},policy,maxValues)', 'frame,scale},policy,make)')
 return s
edit('circle.ts',circle)
def rounded(s):
 s=s.replace("import{ifcConvexSolidSteps}","import{ifcArrayMeshWriterFactory,type IfcMeshWriterFactory}from'./mesh-writer.ts';\nimport{ifcConvexSolidWriteSteps}")
 s=replace(s," ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);", " return yield* ifcRoundedRectangleExtrusionWriteSteps(graph,solid,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));\n}\n/** 🔲️ Keep original rounded source admission while streaming directly into its endpoint. */\nexport function* ifcRoundedRectangleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);")
 s=s.replace('ifcConvexSolidSteps(', 'ifcConvexSolidWriteSteps(').replace('profile:profileFrame,frame,scale},maxValues)', 'profile:profileFrame,frame,scale},make)')
 return s
edit('rounded.ts',rounded)
def rectangle(s):
 s=s.replace('import {rectanglePoint',"import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';\nimport {rectanglePoint")
 s=replace(s,'world:IfcMatrix,scale:number):IfcIoSteps<', 'world:IfcMatrix,scale:number,maxValues:number):IfcIoSteps<')
 s=replace(s,' yield;\n const profile='," return yield* ifcRectangleExtrusionWriteSteps(graph,solid,world,scale,ifcArrayMeshWriterFactory(maxValues));\n}\n/** 🧊️ The rectangular source owns no second positions or index array. */\nexport function* ifcRectangleExtrusionWriteSteps<T>(graph:IfcGeometryGraph,solid:IfcEntity,world:IfcMatrix,scale:number,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n yield;\n const profile=")
 s=replace(s," const positions:IfcTriangleMesh['positions']=[],indices:number[]=[];",' const writer=make(8,12);let failed=false;\n try{')
 s=replace(s,'positions.push(point);', 'writer.point(layer*4+corner,point[0],point[1],point[2]);')
 s=replace(s,'for(const face of BLOCK_TRIANGLES){yield;indices.push(face[0],face[vector[2]>0?1:2],face[vector[2]>0?2:1]);}', 'for(let at=0;at<BLOCK_TRIANGLES.length;at++){yield;const face=BLOCK_TRIANGLES[at]!;writer.triangle(at,face[0],face[vector[2]>0?1:2],face[vector[2]>0?2:1]);}')
 s=replace(s,' return{positions,indices};',' return writer.finish();\n }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}')
 return s
edit('rectangle.ts',rectangle)
def csg(s):
 s=s.replace('import {ifcCircularSolidSteps}',"import {ifcArrayMeshWriterFactory,type IfcMeshWriterFactory} from './mesh-writer.ts';\nimport {ifcCircularSolidWriteSteps}")
 s=replace(s," exact(primitive,['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE']);", " return yield* ifcCsgPrimitiveWriteSteps(graph,primitive,world,scale,policy,ifcArrayMeshWriterFactory(maxValues));\n}\n/** 🧱️ Reserve the endpoint before any primitive coordinate or triangle backing is born. */\nexport function* ifcCsgPrimitiveWriteSteps<T>(graph:IfcGeometryGraph,primitive:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,make:IfcMeshWriterFactory<T>):IfcIoSteps<T>{\n exact(primitive,['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE']);")
 s=s.replace('ifcCircularSolidSteps(', 'ifcCircularSolidWriteSteps(').replace('frame,scale},policy,maxValues)', 'frame,scale},policy,make)')
 start=s.index(' if(!Number.isSafeInteger(maxValues)')
 end=s.index(' for(let at=0;at<8;',start)
 s=s[:start]+" const writer=make(8,12),seen=new Set<string>();let failed=false;\n try{\n"+s[end:]
 s=replace(s,'positions.push(point);','writer.point(at,point[0],point[1],point[2]);')
 s=replace(s,'for(const triangle of BLOCK_TRIANGLES){yield;indices.push(triangle[0],triangle[1],triangle[2]);}yield;return{positions,indices};','for(let at=0;at<BLOCK_TRIANGLES.length;at++){yield;const triangle=BLOCK_TRIANGLES[at]!;writer.triangle(at,triangle[0],triangle[1],triangle[2]);}yield;return writer.finish();\n }catch(error){failed=true;throw error;}finally{try{writer.close();}catch(error){if(!failed)throw error;}}')
 return s
edit('csg.ts',csg)
