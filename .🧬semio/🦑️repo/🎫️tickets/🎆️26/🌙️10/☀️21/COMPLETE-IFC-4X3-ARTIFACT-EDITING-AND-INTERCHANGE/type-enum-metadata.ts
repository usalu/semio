import {ifc4x3Catalog as c} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
for(const name of ['IFCCOMPOSITECURVESEGMENT','IFCBSPLINECURVE','IFCBSPLINECURVEWITHKNOTS','IFCDERIVEDUNIT','IFCSENSORTYPE'])console.log(JSON.stringify({name,direct:c.entities[name]!.attributes,supertype:c.entities[name]!.supertype,all:c.entities[name]!.allAttributes.map(a=>[a.name,a.optional])}));
for(const name of ['IFCDERIVEDUNITENUM','IFCSENSORTYPEENUM'])console.log(JSON.stringify({name,members:c.types[name]!.members}));
