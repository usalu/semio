from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts');s=p.read_text(encoding='utf-8')
s=s.replace('depth:number;context:string};const stack:Frame[]=[];', 'depth:number;context:string;allowed?:readonly string[]};const stack:Frame[]=[];')
needle=' const put=(source:unknown,set:'
insert=r''' const fields=(source:object,context:string,parent:string):readonly string[]|undefined=>{if(!admit||Array.isArray(source))return undefined;const has=(key:string)=>Object.hasOwn(source,key);if(has("kind")){const kind=(source as {kind:unknown}).kind;return kind==="unset"||kind==="derived"?["kind"]:["kind","value"];}if(has("mutation"))return["mutation","id","index","name","entity","complex","values","value","path"];if(context==="header")return["fileDescription","fileName","fileSchema"];if(context==="entities"||context==="args")return["removed","modified","added"];if(context==="diff")return["name","args","complex"];if(has("bits"))return["bits"];if(has("items"))return["name","items"];if(has("entity"))return["index","entity"];if(has("diff"))return["id","diff"];if(has("id"))return has("name")?["id","name","args","complex"]:["id","index"];if(has("args")||parent==="complex")return["name","args"];if(has("index"))return["index","value"];if(context==="")return has("schema")?["schema","header","entities"]:["fileDescription","fileName","fileSchema","entities"];return[];};
'''
s=s.replace(needle,insert+needle)
s=s.replace('stack.push({source,target,keys:keys(source),depth,context});','stack.push({source,target,keys:keys(source),depth,context,allowed:fields(source,context,parent)});')
s=s.replace('const key=next.value;put((frame.source as Record<string,unknown>)[key]', 'const key=next.value;if(frame.allowed&&!frame.allowed.includes(key))fail("invalidValue","captured IFC data carries an unknown field");put((frame.source as Record<string,unknown>)[key]')
p.write_text(s,encoding='utf-8',newline='\n')
