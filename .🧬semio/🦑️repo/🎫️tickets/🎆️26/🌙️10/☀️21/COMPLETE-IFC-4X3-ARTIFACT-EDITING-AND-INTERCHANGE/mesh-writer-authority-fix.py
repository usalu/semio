from pathlib import Path
g=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry')
for name,profile_name,dimensions in [('rectangle.ts','IFCRECTANGLEPROFILEDEF',"x=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','XDim')),y=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCRECTANGLEPROFILEDEF','YDim')),depth=ifcNumber(yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','Depth'))"),('circle.ts','IFCCIRCLEPROFILEDEF',"radius=ifcNumber(yield* ifcGeometryFieldSteps(graph,profile,'IFCCIRCLEPROFILEDEF','Radius')),depth=ifcNumber(yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','Depth')),radiusMetres=radius*scale")]:
 p=g/name;s=p.read_text(encoding='utf8')
 if name=='rectangle.ts':s=s.replace("import {rectanglePoint", "import {ifcGeometryEntity,ifcGeometryFieldSteps} from './source.ts';\nimport {rectanglePoint")
 else:s=s.replace('import {ifcGeometryFieldSteps}', 'import {ifcGeometryEntity,ifcGeometryFieldSteps}')
 start=s.index(' yield;',s.index('export function* ifc'+('Rectangle'if name=='rectangle.ts'else'Circle')+'ExtrusionWriteSteps'))
 end=s.index(' if(x<=0'if name=='rectangle.ts'else' if(radius<=0',start)
 s=s[:start]+f''' yield;ifcGeometryEntity(solid,['IFCEXTRUDEDAREASOLID']);
 const profile=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','SweptArea'));ifcGeometryEntity(profile,['{profile_name}']);
 const profileType=yield* ifcGeometryFieldSteps(graph,profile,'IFCPROFILEDEF','ProfileType');
 if(profileType.kind!=='enum'||profileType.value!=='AREA')throw new IfcStepError('geometry.profile','An exact AREA profile is required');
 const position=(value:IfcValue,name:string):IfcMatrix=>{{if(value.kind==='unset')return ifcIdentity();if(graph.ref(value).name!==name)throw new IfcStepError('geometry.placement','Sweep placement must match its declared dimension');return graph.axis(value);}};
 const profileFrame=position(yield* ifcGeometryFieldSteps(graph,profile,'IFCPARAMETERIZEDPROFILEDEF','Position'),'IFCAXIS2PLACEMENT2D'),solidFrame=position(yield* ifcGeometryFieldSteps(graph,solid,'IFCSWEPTAREASOLID','Position'),'IFCAXIS2PLACEMENT3D'),frame=ifcMultiply(world,solidFrame);
 const directionValue=yield* ifcGeometryFieldSteps(graph,solid,'IFCEXTRUDEDAREASOLID','ExtrudedDirection'),direction=graph.ref(directionValue);ifcGeometryEntity(direction,['IFCDIRECTION']);
 if(ifcAggregate(yield* ifcGeometryFieldSteps(graph,direction,'IFCDIRECTION','DirectionRatios')).length!==3)throw new IfcStepError('geometry.direction','An explicit three-dimensional extrusion direction is required');
 const vector=ifcNormalizeDirection(graph.vector(directionValue)),{dimensions};
''' + s[end:]
 p.write_text(s,encoding='utf8')
