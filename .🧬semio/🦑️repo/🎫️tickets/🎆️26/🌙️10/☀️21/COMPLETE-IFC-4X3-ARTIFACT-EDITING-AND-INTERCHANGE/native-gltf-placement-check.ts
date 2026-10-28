import Ajv from 'ajv/dist/2020';
import cases from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/native-gltf-placement-cases.json';
const validate=new Ajv({strict:true}).compile({
 type:'object',required:['schema','rule','baseEntities','entitiesPerPrimitive','identity','cases'],additionalProperties:false,
 properties:{schema:{const:'IFC4X3_ADD2'},rule:{const:'IfcProduct.PlacementForShapeRepresentation'},baseEntities:{const:7},entitiesPerPrimitive:{const:5},identity:{const:{placementRelTo:'unset',relativePlacement:'context.WorldCoordinateSystem'}},
  cases:{type:'array',minItems:1,items:{type:'object',required:['primitives','entities'],additionalProperties:false,properties:{primitives:{type:'integer',minimum:1},entities:{type:'integer',minimum:1}}}}
 }
});
if(!validate(cases))throw Error(JSON.stringify(validate.errors));
for(const row of cases.cases)if(BigInt(row.primitives)*BigInt(cases.entitiesPerPrimitive)+BigInt(cases.baseEntities)!==BigInt(row.entities))throw Error('entity census mismatch');
console.log(JSON.stringify({neutralCases:cases.cases.length,ajv:true,bigintCensus:true,nativeExecuted:false}));
