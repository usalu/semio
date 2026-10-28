import {ifc4x3TemplateCatalog} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/🟦️.ts';
import {ifc4x3Catalog} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const enums=new Set([...ifc4x3Catalog.types.IFCUNITENUM!.members,...ifc4x3Catalog.types.IFCDERIVEDUNITENUM!.members]);
for(const t of ifc4x3TemplateCatalog().templates)for(const e of t.entries){let n=e.dataType.toUpperCase(), seen=new Set(); let found=false;while(!seen.has(n)){seen.add(n);if(enums.has(n.replace(/^IFC/,'').replace(/MEASURE$/,'UNIT'))||['IFCTHERMALCONDUCTIVITYMEASURE','IFCMONETARYMEASURE','IFCRATIOMEASURE','IFCCOUNTMEASURE','IFCNUMERICMEASURE'].includes(n)){found=true;break;}const x=ifc4x3Catalog.types[n]?.typeExpr;if(x?.kind!=='named')break;n=x.name!;}if(!found&&e.dataType.includes('Measure'))console.log(t.name,e.name,e.dataType);}

