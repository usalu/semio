"""🧮️ Apply the focused original-controlled EXPRESS population traversal change."""
from pathlib import Path

path = Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/population.ts")
source = path.read_text(encoding="utf-8")

def replace(before, after):
    """🧷️ Require the exact current authored source before changing it."""
    global source
    assert source.count(before) == 1, before[:180]
    source = source.replace(before, after)

replace(" readonly entities=new Map<bigint,ExpressValue>();",
" readonly entities=new Map<bigint,ExpressValue>();\n readonly #populations=new Map<string,ExpressValue[]>();")
replace("this.entities.set(entity.id,{kind:'entity',identity:entity.id,types,fields:new Map(),groups:new Map()});",
"const value:ExpressValue={kind:'entity',identity:entity.id,types,fields:new Map(),groups:new Map()};this.entities.set(entity.id,value);for(const type of types){await this.evaluator.tick();if(!this.#populations.has(type))this.#populations.set(type,[]);this.#populations.get(type)!.push(value);}")
replace(" scope(value:ExpressValue):ExpressScope{return new Map([['SELF',value]]);}",
""" scope(value:ExpressValue):ExpressScope{return new Map([['SELF',value]]);}
 /** 👥 Project source-ordered subtype cohorts without lending the retained index array. */
 async population(name:string):Promise<ExpressValue>{
  await this.evaluator.tick();const values:ExpressValue[]=[];
  for(const value of this.#populations.get(name.toUpperCase())??[]){await this.evaluator.tick();values.push(value);}
  return expressAggregate(values,'SET');
 }
 /** 🔗 Share original-controlled inverse identity matching with scalar cardinality. */
 async inverseValues(value:ExpressValue,inverse:ExpressProgram['inverse'][number]):Promise<ExpressValue[]>{
  await this.evaluator.tick();const matching:ExpressValue[]=[],seen=new Set<ExpressValue>();
  for(const edge of typeof value.identity==='bigint'?this.incoming.get(value.identity)??[]:[]){await this.evaluator.tick();if(edge.attribute===inverse.attribute&&(edge.entity.types??[]).includes(inverse.type)&&!seen.has(edge.entity)){seen.add(edge.entity);matching.push(edge.entity);}}
  return matching;
 }""")
replace("if(inverse){const matching:ExpressValue[]=[];for(const edge of typeof actual.identity==='bigint'?this.incoming.get(actual.identity)??[]:[]){await this.evaluator.tick();if(edge.attribute===inverse.attribute&&(edge.entity.types??[]).includes(inverse.type)&&!matching.includes(edge.entity))matching.push(edge.entity);}return inverse.aggregate===false?matching[0]??unknown:expressAggregate(matching,'SET');}",
"if(inverse){const matching=await this.inverseValues(actual,inverse);return inverse.aggregate===false?matching[0]??unknown:expressAggregate(matching,'SET');}")
replace("const inverseValue=await context.attribute(value,inverse.name),size=BigInt(inverse.aggregate===false?(typeof value.identity==='bigint'?[...new Set((context.incoming.get(value.identity)??[]).filter(edge=>edge.attribute===inverse.attribute&&(edge.entity.types??[]).includes(inverse.type)).map(edge=>edge.entity))].length:0):aggregateItems(inverseValue).length);",
"const size=BigInt((await context.inverseValues(value,inverse)).length);")
replace("scope.set(population,expressAggregate([...context.entities.values()].filter(value=>(value.types??[]).includes(population)),'SET'));",
"scope.set(population,await context.population(population));")
path.write_text(source, encoding="utf-8")
print("[DEBUG] Focused population traversal change applied")
