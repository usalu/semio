from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_input.ts')
s=f.read_text(encoding='utf8').rstrip()
s=s.replace('StagedFormDiagnostic,StagedValue','StagedFormDiagnostic,StagedFormPageBinding,StagedValue').replace('import {stagedFormToken}', 'import {stagedFormToken,stagedFormPage}')
a=s.index("type InputKind=");b=s.index('\nfunction integer(',a)
s=s[:a]+'''type InputKind='reference'|'string'|'integer'|'enum';
interface InputDomain {kind:InputKind;members?:readonly string[]}
function* inputKind(expression:IfcTypeExpression):IfcEditSteps<InputDomain>{let current=expression;for(let depth=0;depth<64;depth++){yield tick();if(current.kind==='named'&&catalog.entities[current.name??''])return{kind:'reference'};if(current.kind==='named'&&current.name==='STRING')return{kind:'string'};if(current.kind==='named'&&current.name==='INTEGER')return{kind:'integer'};if(current.kind==='named'&&['BOOLEAN','LOGICAL'].includes(current.name??''))return{kind:'enum',members:current.name==='BOOLEAN'?['T','F']:['T','F','U']};const declaration=current.kind==='named'?catalog.types[current.name??'']:undefined;if(declaration?.kind==='enum')return{kind:'enum',members:declaration.members};if(declaration?.kind!=='defined'||!declaration.typeExpr)fail('type.input-control');current=declaration.typeExpr;}return fail('type.input-depth');}
interface Required extends InputDomain{index:number;name:string}
''' +s[b:]
s=s.replace('missing:{index:number;name:string;kind:InputKind}[];required:{index:number;name:string;kind:InputKind}[];', 'missing:Required[];required:Required[];')
s=s.replace("if(request.token.action!=='setSnapshotValue')fail('staging.action');", "if(request.token.action!=='setSnapshotValue')fail('staging.action');if(request.page!==undefined&&(!stagedFormPage(request.page)||request.page.fieldOffset!=='0'||request.page.diagnosticOffset!=='0'))fail('type.input-page');")
s=s.replace("if(!Number.isSafeInteger(position)||!entity||!entity.complex?.length)fail('type.input-source');", "if(!Number.isSafeInteger(position)||!entity)fail('type.input-source');")
a="if(!old||!next||old.attributes.length!==entity.args.length||old.attributes.length===0||next.attributes.length===0||old.attributes.length===next.attributes.length)fail('type.input-shape');"
b="if(!old||!next||old.attributes.length!==entity.args.length||old.attributes.length===0||next.attributes.length===0)fail('type.input-shape');const repair=name===entity.name.toUpperCase()&&old.attributes.length===next.attributes.length;if(repair?!entity.complex?.length&&(old.supertype!==null||old.allAttributes.length!==old.attributes.length):!entity.complex?.length||old.attributes.length===next.attributes.length)fail('type.input-shape');"
assert a in s;s=s.replace(a,b).replace("['reference','string','integer'].includes(payload.kind)", "['reference','string','integer','enum'].includes(payload.kind)")
a="const kind=yield* inputKind(attribute.typeExpr);required.push({index,name:attribute.name,kind});if(payload&&payload.kind!==kind)fail('type.input-kind');"
b="const domain=yield* inputKind(attribute.typeExpr),{kind}=domain,field={index,name:attribute.name,...domain};required.push(field);if(payload&&payload.kind!==kind)fail('type.input-kind');"
assert a in s;s=s.replace(a,b).replace('missing.push({index,name:attribute.name,kind});','missing.push(field);')
a="try{args.push(kind==='reference'?{kind,value:decimal(payload.value,true)}:kind==='integer'?{kind,value:integer(payload.value)}:{kind,value:payload.value as string});}"
b="try{if(kind==='enum'){let accepted=false;for(const member of domain.members!){yield tick();accepted ||= member===payload.value;}if(!accepted)fail('type.input-member');}args.push(kind==='reference'?{kind,value:decimal(payload.value,true)}:kind==='integer'?{kind,value:integer(payload.value)}:{kind,value:payload.value as string});}"
assert a in s;s=s.replace(a,b)
s=s.replace(' if(missing.length===0){',' if(repair&&required.length===0)fail(\'type.input-no-change\');\n if(missing.length===0){')
a=s.index('/** 🧭️ All required');b=s.index('/** 🔀️ Exact caller',a)
s=s[:a]+'''/** 🧭️ All required typed fields retain their original target order and caller payloads. */
export async function typeInputForm(source:IfcSnapshot,revision:string,request:StagedFormRequest,control:StagedFormControl):Promise<{response:StagedFormResponse;bindings:StagedFormBinding[];pageBindings:StagedFormPageBinding[]}>{
 const opts=options(control);return runIfcEditAsync((function*():IfcEditSteps<{response:StagedFormResponse;bindings:StagedFormBinding[];pageBindings:StagedFormPageBinding[]}>{
 const prepared=yield* prepare(source,revision,request,opts,true),fields:StagedFormField[]=[],diagnostics:StagedFormDiagnostic[]=[],response:StagedFormResponse={token:prepared.request.token,fields,diagnostics,ready:prepared.missing.length===0&&!prepared.invalid},bindings:StagedFormBinding[]=[],pageBindings:StagedFormPageBinding[]=[];
 if(prepared.required.length>16)fail('type.input-content');
 const marker=object(prepared.request.args.value)!,rows=marker.$ifcInputs as StagedValue[]|undefined,coordinates:{path:string[];offset:string}[]=[];
 const address=(index:number)=>['entities',String(prepared.position),'args',String(index)];
 for(const field of prepared.required){yield tick();if(field.members&&field.members.length>32)for(let offset=0;offset<field.members.length;offset+=31){yield tick();coordinates.push({path:address(field.index),offset:String(offset)});}}
 const requested=prepared.request.page?.choice;let active=0;
 if(requested){active=coordinates.findIndex(page=>page.offset===requested.offset&&JSON.stringify(page.path)===JSON.stringify(requested.path));if(active<0)fail('type.input-page');}
 const current=coordinates[active];for(const index of [active-1,active+1]){yield tick();const choice=coordinates[index];if(choice)pageBindings.push({token:prepared.request.token,page:{fieldOffset:'0',diagnosticOffset:'0',choice}});}
 for(const field of prepared.required){
  yield tick();const found=rows?.findIndex(row=>object(row)?.index===String(field.index))??-1,insertion=rows?.findIndex(row=>Number(object(row)?.index)>field.index)??-1,row=found<0?(insertion<0?(rows?.length??0):insertion):found,payloadPath=['value','$ifcInputs',String(row),'value'],path=[...payloadPath,'value'],draft={index:String(field.index),value:{kind:field.kind,value:''}},fixed:StagedFormBinding['edits']=found<0?[{source:'fixed',edit:{kind:'set',path:['value','$ifcInputs'],value:[...(rows??[]).slice(0,row),draft,...(rows??[]).slice(row)]}}]:[];
  if(field.members){
   const offset=current&&JSON.stringify(current.path)===JSON.stringify(address(field.index))?Number(current.offset):0,words=field.members.length>32?field.members.slice(offset,offset+31):[...field.members],selected=found<0?undefined:object(object(rows![found])?.value)?.value;
   if(typeof selected==='string'&&field.members.includes(selected)&&!words.includes(selected))words.push(selected);
   const choices=[];for(const word of words){yield tick();choices.push({id:word,label:word==='T'?label('True','Wahr'):word==='F'?label('False','Falsch'):word==='U'?label('Unknown','Unbekannt'):label(word,word),value:{kind:'enum',value:word}});}
   fields.push({kind:'choice',path:payloadPath,label:label(field.name,field.name),required:true,provenance:'declaration',choices});
   bindings.push({token:prepared.request.token,layout:'replace',edits:[...fixed,{source:'choice',kind:'set',path:payloadPath,options:choices.map(choice=>({key:choice.id,value:choice.value}))}]});
  }else{
   bindings.push({token:prepared.request.token,layout:found<0?'replace':'preserve',edits:[...fixed,{source:'text',path}]});
   fields.push({kind:'text',path,label:label(field.name+' '+(field.kind==='reference'?'reference':field.kind==='integer'?'integer':'text'),field.kind==='reference'&&field.name==='Profile'?'Profilreferenz':field.name+' '+(field.kind==='reference'?'Referenz':field.kind==='integer'?'Ganzzahl':'Text')),required:true,format:field.kind==='reference'?'decimal-u64':field.kind==='integer'?'decimal-i64':'text'});
  }
  if(!response.ready)diagnostics.push({path:field.members?payloadPath:path,code:'type.input-required',label:label('Supply the declared value','Deklarierten Wert angeben')});
 }
 return{response,bindings,pageBindings};})(),opts);
}
''' +s[b:]
s=s.replace("result:IfcMutation[]=[{mutation:'setEntityName',id:entity.id,name:prepared.name}];", "result:IfcMutation[]=[];if(prepared.name!==entity.name.toUpperCase())result.push({mutation:'setEntityName',id:entity.id,name:prepared.name});")
b=s.encode();old=f.stat().st_size
with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
