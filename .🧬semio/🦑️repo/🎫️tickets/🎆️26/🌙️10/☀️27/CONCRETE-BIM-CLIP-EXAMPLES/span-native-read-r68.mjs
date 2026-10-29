/** 🧵 Read native empty IFC text through an independent engine. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
export async function verifyEmptyProperties(ticket,samples){
 const config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));
 const output=[];
 try{for(const sample of samples){
 const id=api.OpenModel(new TextEncoder().encode(sample.text));
 try{
 const property=api.GetLine(id,Number(sample.property.slice(1)));
 if(property.type!==library.IFCPROPERTYSINGLEVALUE||property.Name?.value!=='SharedOwnerTags'||property.NominalValue?.value!=='')throw Error('Native empty text changed');
 output.push({language:sample.language,name:property.Name.value,value:property.NominalValue.value,quoted:sample.quoted,text_sha256:sample.text_sha256});
 }finally{api.CloseModel(id);}
 }}finally{api.Dispose();}return output;
}
