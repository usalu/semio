import {readFileSync,writeFileSync} from 'node:fs';
const home='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details',previous=JSON.parse(readFileSync(home+'/🧪️tests/constituent-remap.json','utf8'));
const unset={kind:'unset'},ref=(value:string)=>({kind:'reference',value}),str=(value:string)=>({kind:'string',value});
const sources:Record<string,any>={};
for(const key of ['axis3','axis2','single','bounded','invalid-unit','sphere','missing']){const old=previous.sources[key],part=old.complex.at(-1);sources[key]={name:part.name,args:part.args,complex:[{name:old.name,args:old.args},...old.complex.slice(0,-1)]};}
sources['single-unset']={...sources.single,args:[unset,unset]};
sources.material={name:'IFCMATERIALPROFILE',args:[str('名称🌍'),str('exact description'),ref('6'),ref('7'),{kind:'integer',value:'9223372036854775807'},str('category/🌍')],complex:[{name:'IFCMATERIALDEFINITION',args:[]}]};
sources.constituent={name:'IFCMATERIALCONSTITUENT',args:[str('名称🌍'),str('exact description'),ref('6'),unset,str('category/🌍')],complex:[{name:'IFCMATERIALDEFINITION',args:[]}]};
const real=(bits:string)=>({kind:'real',value:{bits}});
sources.offsets={name:'IFCMATERIALLAYERWITHOFFSETS',args:[{kind:'enum',value:'AXIS2'},{kind:'aggregate',value:[real('9223372036854775808'),real('4611686018427387904')]}],complex:[{name:'IFCMATERIALDEFINITION',args:[]},{name:'IFCMATERIALLAYER',args:[ref('6'),real('4611686018427387904'),unset,str('layer'),unset,unset,unset]}]};
sources.box={name:'IFCBOUNDINGBOX',args:[ref('3'),real('4611686018427387904'),real('4611686018427387904'),real('4611686018427387904')],complex:[{name:'IFCREPRESENTATIONITEM',args:[]},{name:'IFCGEOMETRICREPRESENTATIONITEM',args:[]}]};
const cases=[
{id:'axis-shrink',source:'axis3',target:'IFCAXIS2PLACEMENT2D',map:[1],args:[ref('5')],mutations:3},
{id:'axis-grow',source:'axis2',target:'IfcAxis2Placement3D',map:[null,0],args:[unset,ref('5')],mutations:3},
{id:'property-grow',source:'single',target:'IFCPROPERTYBOUNDEDVALUE',map:[null,null,1,null],args:[unset,unset,ref('9007199254740993'),unset],mutations:5},
{id:'property-shrink',source:'bounded',target:'IFCPROPERTYSINGLEVALUE',map:[null,2],args:[unset,ref('9007199254740993')],mutations:5},
{id:'sparse-existing-unset',source:'single-unset',target:'IFCPROPERTYBOUNDEDVALUE',map:[null,null,1,null],args:[unset,unset,unset,unset],mutations:3},
{id:'material-in-place-and-move',source:'material',target:'IFCMATERIALCONSTITUENT',map:[0,1,2,null,5],args:[...sources.material.args.slice(0,3),unset,sources.material.args[5]],mutations:4},
{id:'aggregate-offset-move',source:'offsets',target:'IFCMATERIALPROFILEWITHOFFSETS',map:[1],args:[sources.offsets.args[1]],mutations:3},
{id:'numeric-dimension-move',source:'box',target:'IFCRECTANGLEPROFILEDEF',map:[1,2],args:[sources.box.args[1],sources.box.args[2]],mutations:5},
{id:'maximum-id',entityId:'18446744073709551615',source:'single',target:'IFCPROPERTYBOUNDEDVALUE',map:[null,null,1,null],args:[unset,unset,ref('9007199254740993'),unset],mutations:5},
{id:'required-profile-refusal',source:'constituent',target:'IFCMATERIALPROFILE',phase:'required',admitted:false},
{id:'required-radius-refusal',source:'sphere',target:'IFCELLIPSE',phase:'required',admitted:false},
{id:'required-enumeration-refusal',source:'single',target:'IFCPROPERTYENUMERATION',phase:'required',admitted:false},
{id:'invalid-compatible-domain-refusal',source:'invalid-unit',target:'IFCPROPERTYBOUNDEDVALUE',phase:'domain',admitted:false},
{id:'missing-original-slot',source:'missing',target:'IFCPROPERTYBOUNDEDVALUE',retained:false,admitted:false}
].map(row=>({path:'/entities/0',retained:true,admitted:true,...row}));
previous.tail.push({id:'6',name:'IFCMATERIAL',args:[str('steel'),unset,unset],complex:[]},{id:'7',name:'IFCRECTANGLEPROFILEDEF',args:[{kind:'enum',value:'AREA'},unset,unset,{kind:'real',value:{bits:'4611686018427387904'}},{kind:'real',value:{bits:'4611686018427387904'}}],complex:[]});
writeFileSync(home+'/🧪️tests/primary-remap.json',JSON.stringify({schema:'IFC4X3_ADD2',nativeExecution:false,operation:'ordinal-preserving-compatible-direct-attribute-remap',sources,tail:previous.tail,cases},null,2)+'\n');
