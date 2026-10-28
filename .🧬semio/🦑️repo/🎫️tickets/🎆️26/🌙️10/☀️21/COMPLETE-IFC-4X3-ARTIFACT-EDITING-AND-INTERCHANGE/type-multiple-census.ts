import {ifc4x3Catalog as catalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
let maximum=0,names:string[]=[];
for(const [name,entity]of Object.entries(catalog.entities)){const count=entity.attributes.filter(attribute=>!attribute.optional&&!attribute.derived&&attribute.typeExpr.kind==='named'&&catalog.entities[attribute.typeExpr.name??'']).length;if(count>maximum){maximum=count;names=[name];}else if(count===maximum)names.push(name);}
console.log(JSON.stringify({schema:'IFC4X3_ADD2',maximum,names}));
