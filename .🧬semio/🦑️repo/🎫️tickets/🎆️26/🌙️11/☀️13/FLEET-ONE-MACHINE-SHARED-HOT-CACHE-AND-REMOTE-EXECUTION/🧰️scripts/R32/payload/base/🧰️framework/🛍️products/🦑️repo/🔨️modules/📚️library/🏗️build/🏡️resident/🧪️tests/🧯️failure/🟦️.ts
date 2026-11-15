/** 🧯️ An EPERM of the resident service below a macOS-protected folder is named as the missing file access consent in both languages, with a reference and the log; every other failure keeps its former refusal. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {accessDeniedOfV1,deniedPlaceOfV1,failureDetailV1,failureRefusalV1,heavyRefusalOfV1,serviceFactsV1,wantedRefusalOfV1} from "../../🏛️service/🧯️failure/🟨️.mjs";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const perm=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"}),home="/Users/worker",logs:string[]=[];
const refuse=(error:unknown,options:{root:string;platform?:string})=>failureRefusalV1(error,{log:(event,detail)=>{logs.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",...options});

test("EPERM below Documents, Desktop or Downloads of the user on macOS is the access consent, anywhere else it is internal",()=>{
  for(const folder of ["Documents","Desktop","Downloads"])expect(accessDeniedOfV1(perm,{root:`${home}/${folder}/semio/deep`,platform:"darwin",home})).toBe(folder);
  for(const root of [`${home}/semio`,"/Users/Shared/semio",`${home}/DocumentsOther/semio`,"/Users/other/Documents/semio"])expect(accessDeniedOfV1(perm,{root,platform:"darwin",home}),root).toBeUndefined();
  expect(accessDeniedOfV1(perm,{root:`${home}/Documents/semio`,platform:"linux",home})).toBeUndefined();
  expect(accessDeniedOfV1(new Error("Relative filesystem fstatat failed (ENOENT)"),{root:`${home}/Documents/semio`,platform:"darwin",home})).toBeUndefined();
  expect(accessDeniedOfV1(new Error("Relative filesystem fstatat failed (EPERM)"),{root:`${home}/Documents/semio`,platform:"darwin",home})).toBe("Documents");
});

test("the refusal carries folder, reference and log, keeps the retry advice of the caller and the log holds the failure under the same reference",()=>{
  const refusal=refuse(perm,{root:`${home}/Documents/semio`});
  expect(refusal).toMatchObject({code:"consent",key:"access-denied",retry:"later",fields:{folder:"Documents",log:"/log"}});
  expect(refusal.fields.identity).toContain("semio/launcher/bin/repo");
  expect(logs.at(-1)).toContain(refusal.fields.reference);
  expect(refuse(perm,{root:"/Users/Shared/semio"})).toMatchObject({code:"internal",key:"internal",retry:"later"});
  expect(refuse(Object.assign(new Error("database or disk is full"),{code:"ENOSPC"}),{root:`${home}/Documents/semio`}).code).toBe("disk");
});

test("the text exists in English and German with the same placeholders and names the settings, the identity and the alternative",()=>{
  const texts=schema.$defs?.Texts ?? (schema as any).definitions?.Texts;
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["access-denied"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all) ?? find(texts);
  const holes=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(table.de["access-denied"])).toEqual(holes(table.en["access-denied"]));
  expect(table.en["access-denied"]).toContain("Files & Folders");
  expect(table.de["access-denied"]).toContain("Dateien & Ordner");
  expect(table.en["access-denied"]).toContain("{identity}");
});

test("the graph-wanted refusal keeps its numbers through values, message text and cause, and every other failure is not one",()=>{
  const values={wanted:703,arrived:0,seconds:902,missing:703,local:192},fields={wanted:"703",arrived:"0",seconds:"902",missing:"703",local:"192",heavy:""};
  const message="unserved build.graph.wanted-expired "+JSON.stringify(values);
  expect(wantedRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.wanted-expired",values}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error(message))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error("outer",{cause:new Error(message)}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error("secret"))).toBeUndefined();
  expect(refuse(new Error(message),{root:"/Users/Shared/semio"})).toEqual({code:"unserved",key:"host-graph-wanted-expired",fields,retry:"later"});
});

test("a graph-wanted refusal that waited for heavy projects keeps the sentence naming them through values and message text",()=>{
  const heavy=", among them the too heavy project(s) workspace (about 6200 MB) (this machine can give about 4790 MB per project)",values={wanted:2,arrived:0,seconds:300,missing:2,local:192,heavy},message="unserved build.graph.wanted-expired "+JSON.stringify(values);
  const fields={wanted:"2",arrived:"0",seconds:"300",missing:"2",local:"192",heavy};
  expect(wantedRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.wanted-expired",values}))).toEqual(fields);
  expect(wantedRefusalOfV1(new Error(message))).toEqual(fields);
  const shown=refuse(new Error(message),{root:"/Users/Shared/semio"});
  expect(shown.fields).toEqual(fields);
  expect(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8").match(/"host-graph-wanted-expired": "[^"]*\{heavy\}/gu)).toHaveLength(2);
});

test("a graph read that stopped projects too heavy for the machine keeps its words through values, message text and cause, and its refusal is retried later in both languages",()=>{
  const values={named:"@semio-tech/s-dev-composition (about 28000 MB)",limit:2396,megabytes:14000},message="unserved build.graph.owner-too-heavy "+JSON.stringify(values),fields={named:values.named,limit:"2396"};
  expect(heavyRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.owner-too-heavy",values}))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error(message))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error("outer",{cause:new Error(message)}))).toEqual(fields);
  expect(heavyRefusalOfV1(new Error("secret"))).toBeUndefined();expect(wantedRefusalOfV1(new Error(message))).toBeUndefined();
  expect(refuse(new Error(message),{root:"/Users/Shared/semio"})).toEqual({code:"unserved",key:"host-graph-owner-too-heavy",fields,retry:"later"});
  const text=readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8");
  expect(text.match(/"host-graph-owner-too-heavy": "[^"]*\{named\}[^"]*\{limit\}/gu)).toHaveLength(2);
});

test("every EPERM in the service log carries a where line with the refused root and entry, the code, and the route, pid and parent of the service; an EPERM that names nothing says so",()=>{
  const bare=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"}),named=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:`${home}/Documents/semio`,entry:".cache"}),lines:string[]=[];
  expect(deniedPlaceOfV1(named)).toBe(`root=${home}/Documents/semio entry=.cache code=EPERM`);
  expect(deniedPlaceOfV1(new Error("outer",{cause:named}))).toBe(deniedPlaceOfV1(named));
  expect(deniedPlaceOfV1(bare)).toBeUndefined();expect(deniedPlaceOfV1("text")).toBeUndefined();
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"launchd"},pid:11,ppid:1})).toBe("route=launchd pid=11 ppid=1");
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:12,ppid:300})).toBe("route=direct pid=12 ppid=300");
  expect(serviceFactsV1({environment:{},pid:13,ppid:2})).toBe("route=foreground pid=13 ppid=2");
  expect(serviceFactsV1({environment:{SEMIO_SERVICE_ROUTE:"cron"},pid:13,ppid:2})).toBe("route=foreground pid=13 ppid=2");
  const record=(error:unknown,root=`${home}/Documents/semio`)=>failureRefusalV1(error,{log:(event,detail)=>{lines.push(event+" "+detail);},logPath:"/log",home,platform:"darwin",root,environment:{SEMIO_SERVICE_ROUTE:"direct"},pid:4242,ppid:77});
  const one=record(named),two=record(bare);
  expect(lines.at(-2)).toContain(`where: root=${home}/Documents/semio entry=.cache code=EPERM route=direct pid=4242 ppid=77`);
  expect(lines.at(-2)!.startsWith("request-failed "+one.fields.reference+" ")).toBe(true);expect(two.fields.reference).toMatch(/^[0-9a-f]{8}$/u);
  expect(lines.at(-1)).toContain("where: root=? entry=? code=EPERM route=direct pid=4242 ppid=77");
  expect(one.fields).toMatchObject({place:`${home}/Documents/semio/.cache`,route:"direct",pid:"4242",ppid:"77"});
  expect(two.fields).toMatchObject({place:"?",route:"direct",pid:"4242",ppid:"77"});
  const outside=record(named,"/Users/Shared/semio");
  expect(outside.code).toBe("internal");expect(lines.at(-1)).toContain("where: root=");expect(lines.at(-1)).toContain("route=direct pid=4242 ppid=77");
  record(Object.assign(new Error("Relative filesystem fstatat failed (ENOENT)"),{code:"ENOENT"}));
  expect(lines.at(-1)).not.toContain("where:");
});

test("the refusal text names the place and the route of the service in both languages with the same placeholders",()=>{
  const all=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
  const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["access-denied"]?node:Object.values(node).map(find).find(Boolean)):undefined;
  const table=find(all);
  for(const language of ["en","de"])for(const name of ["place","route","pid","ppid","reference","log","folder","identity"])expect(table[language]["access-denied"],language+name).toContain("{"+name+"}");
});

test("a failure whose stack lost its message is logged with its name, message, code and project",()=>{
  const lost=Object.assign(new Error("Observation leaf changed before delivery"),{code:"BUILD_INFERENCE_LEAF_CHANGED",semioProject:"libs/alpha",stack:"Error\n    at leafChangedV1 (x.mjs:1:1)"}),lines:string[]=[];
  const detail=failureDetailV1(lost);
  expect(detail.split("\n")[0]).toBe("Error: Observation leaf changed before delivery code=BUILD_INFERENCE_LEAF_CHANGED project=libs/alpha");expect(detail).toContain("at leafChangedV1");
  const intact=new Error("plain");expect(failureDetailV1(intact)).toBe(intact.stack);expect(failureDetailV1("text")).toBe("text");
  failureRefusalV1(lost,{log:(event,line)=>{lines.push(event+" "+line);},logPath:"/log"});
  expect(lines[0]).toContain("BUILD_INFERENCE_LEAF_CHANGED");expect(lines[0]).toContain("project=libs/alpha");
});
