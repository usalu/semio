import {ifc4x3Catalog as catalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
let aggregate=0,numeric=0;
for(const [from,a]of Object.entries(catalog.entities))for(const [to,b]of Object.entries(catalog.entities)){
 if(a.attributes.length===0||b.attributes.length===0||a.attributes.length===b.attributes.length)continue;
 const map=b.attributes.map(next=>a.attributes.findIndex(old=>old.name===next.name&&JSON.stringify(old.typeExpr)===JSON.stringify(next.typeExpr)));
 if(map.some((index,position)=>index<0&&!b.attributes[position]!.optional))continue;
 const kind=b.attributes.some((item,index)=>map[index]>=0&&item.typeExpr.kind==='aggregate')?'aggregate':b.attributes.some((item,index)=>map[index]>=0&&/REAL|INTEGER|MEASURE/.test(item.typeExpr.name??''))?'numeric':null;
 if(kind==='aggregate'&&aggregate++<5||kind==='numeric'&&numeric++<5)console.log(JSON.stringify({from,to,map,attributes:b.attributes}));
}
