from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_input.ts')
s=p.read_text(encoding='utf8').replace("value.$ifcType.length>256)","value.$ifcType.length>256||!/^[_A-Za-z][_A-Za-z0-9]*$/u.test(value.$ifcType))")
a=s.index('/** 🧭️'); b=s.index('/** 🔀️',a)
s=s[:a]+'''/** 🧭️ All required reference fields retain their original target order and caller payloads. */
export async function typeReferenceInputForm(source:IfcSnapshot,revision:string,request:StagedFormRequest,control:StagedFormControl):Promise<{response:StagedFormResponse;bindings:StagedFormBinding[]}>{
 const opts=options(control),prepared=await runIfcEditAsync(prepare(source,revision,request,opts,true),opts),response:StagedFormResponse={token:prepared.request.token,fields:[],diagnostics:[],ready:prepared.missing.length===0&&!prepared.invalid},bindings:StagedFormBinding[]=[];
 if(prepared.required.length>16)fail('type.input-content');
 const marker=object(prepared.request.args.value)!,rows=marker.$ifcInputs as StagedValue[]|undefined;
 for(const field of prepared.required){
  const found=rows?.findIndex(row=>object(row)?.index===String(field.index))??-1,insertion=rows?.findIndex(row=>Number(object(row)?.index)>field.index)??-1,row=found<0?(insertion<0?(rows?.length??0):insertion):found,path=['value','$ifcInputs',String(row),'value','value'],draft={index:String(field.index),value:{kind:'reference',value:''}};
  bindings.push({token:prepared.request.token,layout:found<0?'replace':'preserve',edits:found<0?[{source:'fixed',edit:{kind:'set',path:['value','$ifcInputs'],value:[...(rows??[]).slice(0,row),draft,...(rows??[]).slice(row)]}},{source:'text',path}]:[{source:'text',path}]});
  response.fields.push({kind:'text',path,label:label(field.name+' reference',field.name==='Profile'?'Profilreferenz':field.name+' Referenz'),required:true,format:'decimal-u64'});
  if(!response.ready)response.diagnostics.push({path,code:'type.input-required',label:label('Supply the declared reference','Deklarierte Referenz angeben')});
 }
 return{response,bindings};
}
''' + s[b:]
with p.open('r+b')as f:
 f.write(s.encode()); f.write(b' '*max(0,p.stat().st_size-len(s.encode())))
p=p.with_name('type_input.test.ts');s=p.read_text(encoding='utf8').replace('.binding!', '.bindings[0]!').replace('.binding)', '.bindings[0])');data=s.encode()
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,p.stat().st_size-len(data)))
