import {compareNativeRecords as edit} from "./typing-edit-oracle";
import {compareNativeRecords as commit} from "./typing-original-oracle";
const kind=process.argv[2],path=process.argv[3];
if((kind!=="edit"&&kind!=="commit")||!path)throw Error("requires edit|commit and an actual native log");
const compare=kind==="edit"?edit:commit,prefix=kind==="edit"?"[DEBUG] [original-typing-edit] ":"[DEBUG] [original-typing] ";
const text=await Bun.file(path).text(),count=compare(text);
const records=text.split(/\r?\n/).filter(line=>line.includes(prefix));
if(records.length<2)throw Error("actual native corpus needs distinct records");
function refuse(name:string,lines:string[],reason:string):string{try{compare(lines.join("\n"));}catch(error){if(!String(error).includes(reason))throw error;return name;}throw Error("hostile native record mutation was accepted: "+name);}
const replacement=[...records];replacement[1]=records[0];
const first=JSON.parse(records[0].slice(records[0].indexOf(prefix)+prefix.length));
const unknown={...first,name:first.name+"-unknown"},changed={...first,result:{...first.result,unexpectedField:true}};
const rejected=[
 refuse("duplicate-replaces-omitted-case",replacement,"duplicate native"),
 refuse("extra-known-duplicate",[...records,records[0]],"duplicate native"),
 refuse("omitted-case",records.slice(1),"missing"),
 refuse("unknown-case",[prefix+JSON.stringify(unknown),...records.slice(1)],"unknown native"),
 refuse("changed-actual-output",[prefix+JSON.stringify(changed),...records.slice(1)],kind==="edit"?"native edit differs":"native typing output differs"),
];
console.log(JSON.stringify({kind,actualNativeRecords:count,nativeCompared:true,hostileRejected:rejected}));
