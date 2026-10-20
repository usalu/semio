/** 📜️ Reads JSON with comments and trailing commas by blanking both outside strings, so lock files of any manager share one tolerant JSON entry. */
export function parseJsoncV1(text:string):unknown{
 const source=text.charCodeAt(0)===0xFEFF?text.slice(1):text;let out="",pending=false,at=0;
 const flush=(next:string)=>{if(pending&&next!=="}"&&next!=="]")out+=",";pending=false;};
 while(at<source.length){
  const char=source[at]!,pair=source.slice(at,at+2);
  if(pair==="//"){const end=source.indexOf("\n",at);at=end<0?source.length:end;continue;}
  if(pair==="/*"){const end=source.indexOf("*/",at+2);if(end<0)throw Error("Terminated JSONC comment required");at=end+2;continue;}
  if(char===","){pending=true;at++;continue;}
  if(char===" "||char==="\t"||char==="\n"||char==="\r"){at++;continue;}
  flush(char);
  if(char==='"'){let end=at+1;while(end<source.length&&source[end]!=='"')end+=source[end]==="\\"?2:1;if(end>=source.length)throw Error("Terminated JSONC string required");out+=source.slice(at,end+1);at=end+1;continue;}
  out+=char;at++;
 }
 return JSON.parse(out);
}

type TomlTableV1=Record<string,unknown>;
const bareV1=/[A-Za-z0-9_-]/u;
const escapesV1:Readonly<Record<string,string>>={b:"\b",t:"\t",n:"\n",f:"\f",r:"\r",'"':'"',"\\":"\\"};

/** 🧾️ Reads the TOML subset every lock file uses (tables, arrays of tables, dotted keys, strings, numbers, booleans, arrays, inline tables). */
export function parseTomlV1(text:string):TomlTableV1{
 const source=text.charCodeAt(0)===0xFEFF?text.slice(1):text,size=source.length;let at=0;
 const fail=(reason:string):never=>{throw Error("Closed TOML subset required: "+reason+" at offset "+at);};
 const blank=()=>{while(at<size&&(source[at]===" "||source[at]==="\t"))at++;};
 const comment=()=>{if(source[at]==="#")while(at<size&&source[at]!=="\n")at++;};
 const skip=()=>{for(;;){blank();comment();if(source[at]==="\n"||source[at]==="\r"){at++;continue;}return;}};
 const quoted=(quote:string):string=>{
  const multi=source.startsWith(quote.repeat(3),at);at+=multi?3:1;if(multi&&source[at]==="\r")at++;if(multi&&source[at]==="\n")at++;
  let out="";
  for(;;){
   if(at>=size)fail("unterminated string");
   if(multi?source.startsWith(quote.repeat(3),at):source[at]===quote){at+=multi?3:1;return out;}
   const char=source[at]!;
   if(!multi&&char==="\n")fail("newline in string");
   if(quote==='"'&&char==="\\"){
    const next=source[at+1]!;
    if(escapesV1[next]!==undefined){out+=escapesV1[next];at+=2;continue;}
    if(next==="u"||next==="U"){const length=next==="u"?4:8;out+=String.fromCodePoint(parseInt(source.slice(at+2,at+2+length),16));at+=2+length;continue;}
    if(multi&&/\s/u.test(next)){at++;while(at<size&&/\s/u.test(source[at]!))at++;continue;}
    fail("escape");
   }
   out+=char;at++;
  }
 };
 const key=():string[]=>{
  const path:string[]=[];
  for(;;){
   blank();const char=source[at];
   if(char==='"'||char==="'")path.push(quoted(char));
   else{const start=at;while(at<size&&bareV1.test(source[at]!))at++;if(start===at)fail("key");path.push(source.slice(start,at));}
   blank();
   if(source[at]==="."){at++;continue;}
   return path;
  }
 };
 const set=(table:TomlTableV1,path:readonly string[],value:unknown)=>{
  let target=table;
  for(const part of path.slice(0,-1)){const next=target[part];target=next===undefined?(target[part]=Object.create(null) as TomlTableV1):Array.isArray(next)?next[next.length-1] as TomlTableV1:next as TomlTableV1;}
  const last=path[path.length-1]!;
  if(Object.prototype.hasOwnProperty.call(target,last))fail("duplicate key "+last);
  Object.defineProperty(target,last,{value,enumerable:true,writable:true,configurable:true});
 };
 const value=():unknown=>{
  const char=source[at];
  if(char==='"'||char==="'")return quoted(char);
  if(char==="["){
   at++;const items:unknown[]=[];
   for(;;){skip();if(source[at]==="]"){at++;return items;}items.push(value());skip();if(source[at]===","){at++;continue;}if(source[at]!=="]")fail("array");}
  }
  if(char==="{"){
   at++;const table:TomlTableV1=Object.create(null);
   for(;;){blank();if(source[at]==="}"){at++;return table;}const path=key();blank();if(source[at]!=="=")fail("inline table");at++;blank();set(table,path,value());blank();if(source[at]===","){at++;continue;}if(source[at]!=="}")fail("inline table end");}
  }
  const start=at;while(at<size&&!/[\s,\]}#]/u.test(source[at]!))at++;
  const token=source.slice(start,at);
  if(token==="true")return true;
  if(token==="false")return false;
  if(/^[+-]?\d[\d_]*$/u.test(token))return Number(token.replace(/_/gu,""));
  if(/^[+-]?\d[\d_]*(\.\d[\d_]*)?([eE][+-]?\d+)?$/u.test(token))return Number(token.replace(/_/gu,""));
  if(token==="")fail("value");
  return token;
 };
 const root:TomlTableV1=Object.create(null);let current=root;
 for(;;){
  skip();if(at>=size)return root;
  if(source[at]==="["){
   const array=source[at+1]==="[";at+=array?2:1;const path=key();blank();
   if(source.slice(at,at+(array?2:1))!==(array?"]]":"]"))fail("table header");at+=array?2:1;
   let target=root;
   for(const part of path.slice(0,array?-1:undefined)){const next=target[part];target=next===undefined?(target[part]=Object.create(null) as TomlTableV1):Array.isArray(next)?next[next.length-1] as TomlTableV1:next as TomlTableV1;}
   if(array){const last=path[path.length-1]!;const list=(target[last]??=[]) as TomlTableV1[];list.push(current=Object.create(null) as TomlTableV1);}
   else current=target;
   continue;
  }
  const path=key();blank();if(source[at]!=="=")fail("assignment");at++;blank();set(current,path,value());blank();comment();
  if(at<size&&source[at]!=="\n"&&source[at]!=="\r")fail("end of line");
 }
}
