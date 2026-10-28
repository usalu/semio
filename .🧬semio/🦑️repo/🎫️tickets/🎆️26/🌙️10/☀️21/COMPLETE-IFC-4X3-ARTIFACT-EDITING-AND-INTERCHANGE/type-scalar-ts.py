from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_input.ts');s=p.read_text(encoding='utf8')
s=s.replace("import type {IfcMutation}","import type {IfcTypeExpression} from '../📚️catalog/🟦️.ts';\nimport type {IfcMutation}")
a=s.index('interface Prepared');s=s[:a]+'''type InputKind='reference'|'string'|'integer';
function* inputKind(expression:IfcTypeExpression):IfcEditSteps<InputKind>{let current=expression;for(let depth=0;depth<64;depth++){yield tick();if(current.kind==='named'&&catalog.entities[current.name??''])return'reference';if(current.kind==='primitive'&&current.name==='STRING')return'string';if(current.kind==='primitive'&&current.name==='INTEGER')return'integer';const declaration=current.kind==='named'?catalog.types[current.name??'']:undefined;if(declaration?.kind!=='defined'||!declaration.typeExpr)fail('type.input-control');current=declaration.typeExpr;}return fail('type.input-depth');}
function integer(value:StagedValue|undefined):bigint{if(typeof value!=='string'||value.length>20||!/^(0|-?[1-9][0-9]*)$/u.test(value))return fail('type.input-integer');const result=BigInt(value);if(result< -9223372036854775808n||result>9223372036854775807n)return fail('type.input-range');return result;}
''' + s[a:]
s=s.replace('name:string;reference:boolean','name:string;kind:InputKind')
s=s.replace("payload.kind!=='reference'", "!['reference','string','integer'].includes(String(payload.kind))")
s=s.replace("  const reference=attribute.typeExpr.kind==='named'&&Boolean(catalog.entities[attribute.typeExpr.name??'']);if(!reference)fail('type.input-reference-control');required.push({index,name:attribute.name,reference});", "  const kind=yield* inputKind(attribute.typeExpr);required.push({index,name:attribute.name,kind});if(payload&&payload.kind!==kind)fail('type.input-kind');")
s=s.replace('missing.push({index,name:attribute.name,reference})','missing.push({index,name:attribute.name,kind})')
s=s.replace("try{args.push({kind:'reference',value:decimal(payload.value,true)});}", "try{args.push(kind==='reference'?{kind,value:decimal(payload.value,true)}:kind==='integer'?{kind,value:integer(payload.value)}:{kind,value:payload.value as string});}")
s=s.replace('typeReferenceInputForm','typeInputForm').replace('typeReferenceInputMutations','typeInputMutations')
s=s.replace("value:{kind:'reference',value:''}","value:{kind:field.kind,value:''}")
s=s.replace("label:label(field.name+' reference',field.name==='Profile'?'Profilreferenz':field.name+' Referenz'),required:true,format:'decimal-u64'", "label:label(field.name+' '+(field.kind==='reference'?'reference':field.kind==='integer'?'integer':'text'),field.kind==='reference'&&field.name==='Profile'?'Profilreferenz':field.name+' '+(field.kind==='reference'?'Referenz':field.kind==='integer'?'Ganzzahl':'Text')),required:true,format:field.kind==='reference'?'decimal-u64':field.kind==='integer'?'decimal-i64':'text'")
s=s.replace("Supply the declared reference','Deklarierte Referenz angeben", "Supply the declared value','Deklarierten Wert angeben")
data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
for name in ['type_input.test.ts','type_multiple_input.test.ts']:
 p=p.with_name(name);s=p.read_text(encoding='utf8').replace('typeReferenceInputForm','typeInputForm').replace('typeReferenceInputMutations','typeInputMutations');data=s.encode();old=p.stat().st_size
 with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
