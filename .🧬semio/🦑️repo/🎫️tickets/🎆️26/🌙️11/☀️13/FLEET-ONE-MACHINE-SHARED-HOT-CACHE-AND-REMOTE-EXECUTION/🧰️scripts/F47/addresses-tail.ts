/** 🪞️ Where this machine keeps the last published address set of an origin that it read from the store: `<root>/.cache/f/addresses/<origin>.json`, so that a store that is unmounted or stalled does not take the way to the server with it. */
export function fleetAddressMirrorPathV1(root:string,origin:string):string{
 return join(root,".cache","f","addresses",`${origin}.json`);
}

const mirrorAddresses=async(root:string,set:FleetAddressSetV1):Promise<void>=>{
 try{
  const path=fleetAddressMirrorPathV1(root,set.origin),text=JSON.stringify(set,null,1)+"\n";
  if(await readFile(path,"utf8").catch(()=>undefined)===text)return;
  await writeAtomic(path,text);
 }catch{}
};

const readMirror=async(root:string,origin:string):Promise<FleetAddressSetV1|undefined>=>{
 try{const set=requireFleetAddressSetV1(JSON.parse(await readFile(fleetAddressMirrorPathV1(root,origin),"utf8")));return set.origin===origin?set:undefined;}catch{return undefined;}
};

/** 📛️ The roles of a machine and the port of each (0 for a role that listens on none), which its DNS-SD advertisement carries as TXT records. */
export type FleetServiceRolesV1=Readonly<Record<string,number>>;

/** 📡️ One DNS-SD advertisement: the instance (the origin id), the service type, the port and the TXT records (`name`, `group`, `roles` and one `<role>=<port>` per role). */
export type FleetServiceRecordV1=Readonly<{instance:string;type:string;port:number;txt:readonly string[]}>;

/** 🏗️ Builds the DNS-SD advertisement of a machine from its origin, its group and its roles; the service port is the first non-zero role port in role order. */
export function fleetServiceRecordV1(input:Readonly<{origin:string;group:string;roles:FleetServiceRolesV1}>):FleetServiceRecordV1{
 const entries=Object.entries(input.roles).sort(([left],[right])=>left.localeCompare(right));
 if(entries.length===0||entries.some(([role,port])=>!/^[a-z][a-z0-9-]{0,31}$/u.test(role)||!Number.isInteger(port)||port<0||port>65535))throw Error("Declared fleet roles with ports required");
 return {instance:input.origin,type:FLEET_SERVICE_TYPE_V1,port:entries.find(([,port])=>port>0)?.[1]??0,txt:[`name=${fleetNameV1(input.origin)}`,`group=${input.group}`,`roles=${entries.map(([role])=>role).join(",")}`,...entries.map(([role,port])=>`${role}=${port}`)]};
}

/** 🛠️ The platform tool and arguments that advertise a record without elevated rights: `dns-sd -R` (Bonjour on macOS, `dns-sd.exe` on Windows) or `avahi-publish -s` (Linux). */
export function fleetAdvertiseCommandV1(record:FleetServiceRecordV1,platform:NodeJS.Platform=process.platform):Readonly<{command:string;args:readonly string[]}>{
 return platform==="linux"?{command:"avahi-publish",args:["-s",record.instance,record.type,String(record.port),...record.txt]}:{command:platform==="win32"?"dns-sd.exe":"dns-sd",args:["-R",record.instance,record.type,"local",String(record.port),...record.txt]};
}

const runTool=(command:string,args:readonly string[],signal:AbortSignal,onStart:()=>void):Promise<number>=>new Promise<number>(resolve=>{
 try{
  const child=spawnProcess(command,[...args],{stdio:"ignore",windowsHide:true,signal});
  child.once("spawn",onStart);
  child.once("error",error=>resolve((error as NodeJS.ErrnoException).code==="ENOENT"?-1:-2));
  child.once("exit",code=>resolve(code??0));
 }catch{resolve(-1);}
});

/** 📣️ Keeps the DNS-SD advertisement of this machine alive through the platform tool and says so in the chosen language: when the tool is missing (no avahi, no Bonjour) it reports once that the machine relies on the addresses published in the store, and looks again every 10 minutes; when the tool ends it starts it again. It ends with the signal and never throws. */
export async function advertiseFleetServiceV1(input:Readonly<{record:FleetServiceRecordV1;signal:AbortSignal;language:FleetAddressLanguageV1;report:(line:string)=>void;platform?:NodeJS.Platform;run?:(command:string,args:readonly string[],signal:AbortSignal,onStart:()=>void)=>Promise<number>;wait?:(milliseconds:number,signal:AbortSignal)=>Promise<void>}>):Promise<void>{
 const tool=fleetAdvertiseCommandV1(input.record,input.platform),run=input.run??runTool,wait=input.wait??((milliseconds:number,signal:AbortSignal)=>new Promise<void>(resolve=>{const timer=setTimeout(resolve,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});}));
 let warned=false;
 while(!input.signal.aborted){
  const code=await run(tool.command,tool.args,input.signal,()=>input.report(fleetAddressTextV1(input.language,"mdns.advertising",{instance:input.record.instance,type:input.record.type,port:input.record.port,tool:tool.command})));
  if(input.signal.aborted)return;
  if(code===-1){if(!warned)input.report(fleetAddressTextV1(input.language,"mdns.unavailable",{tool:tool.command}));warned=true;await wait(600_000,input.signal);continue;}
  input.report(fleetAddressTextV1(input.language,"mdns.ended",{tool:tool.command,code,seconds:10}));
  await wait(10_000,input.signal);
 }
}

/** 🔎️ What service discovery found for one instance: its addresses (loopback and link-local left out), its service port and its TXT records. */
export type FleetDiscoveryV1=Readonly<{addresses:readonly string[];port:number|null;txt:Readonly<Record<string,string>>}>;

const textRecords=(line:string):Record<string,string>=>Object.fromEntries(line.trim().split(/\s+/u).filter(token=>/^[A-Za-z0-9_-]+=/u.test(token)).map(token=>{const at=token.indexOf("=");return [token.slice(0,at),token.slice(at+1)];}));

/** 🧩️ Reads the answer of `dns-sd -L <instance> <type> local`: the target host, the port and the TXT records. */
export function parseDnsSdLookupV1(text:string):Readonly<{host:string;port:number;txt:Readonly<Record<string,string>>}>|undefined{
 const lines=text.split("\n");
 for(let index=0;index<lines.length;index++){
  const match=/can be reached at (\S+?)\.?:(\d+)/u.exec(lines[index]!);
  if(match!==null)return {host:match[1]!,port:Number(match[2]),txt:textRecords(lines[index+1]??"")};
 }
 return undefined;
}

const usable=(address:string):boolean=>(quad(address)!==undefined||groups(address)!==undefined)&&!fleetLoopbackV1(address)&&!fleetLinkLocalV1(address);

/** 🧩️ Reads the answer of `dns-sd -G v4v6 <host>`: every added address, once. */
export function parseDnsSdAddressesV1(text:string):readonly string[]{
 const found:string[]=[];
 for(const line of text.split("\n")){
  const tokens=line.trim().split(/\s+/u);
  if(!tokens.includes("Add"))continue;
  const address=tokens.map(token=>token.split("%")[0]!).find(token=>usable(token));
  if(address!==undefined&&!found.includes(address))found.push(address);
 }
 return found;
}

/** 🧩️ Reads the answer of `avahi-browse -rtp <type>` for one instance. */
export function parseAvahiBrowseV1(text:string,instance:string):FleetDiscoveryV1|undefined{
 const addresses:string[]=[];
 let port:number|null=null,txt:Record<string,string>={};
 for(const line of text.split("\n")){
  const fields=line.split(";");
  if(fields[0]!=="="||fields[3]!==instance)continue;
  const address=(fields[7]??"").split("%")[0]!;
  if(usable(address)&&!addresses.includes(address))addresses.push(address);
  port=Number(fields[8]);
  txt=textRecords((fields[9]??"").replaceAll('"',""));
 }
 return addresses.length===0?undefined:{addresses,port,txt};
}

const collectTool=(command:string,args:readonly string[],milliseconds:number):Promise<string>=>new Promise<string>(resolve=>{
 let out="";
 try{
  const child=spawnProcess(command,[...args],{stdio:["ignore","pipe","ignore"],windowsHide:true});
  const timer=setTimeout(()=>{child.kill();},milliseconds);
  child.stdout!.on("data",chunk=>{out+=String(chunk);});
  child.once("error",()=>{clearTimeout(timer);resolve(out);});
  child.once("close",()=>{clearTimeout(timer);resolve(out);});
 }catch{resolve(out);}
});

/** 🔭️ Finds one machine by its instance name (the origin id) through the platform's service discovery without elevated rights: `avahi-browse` on Linux, `dns-sd -L` and `dns-sd -G` on macOS and Windows. `undefined` when the tool is missing, mDNS is blocked or the machine does not advertise; the caller then relies on the addresses published in the store. */
export async function discoverFleetServiceV1(input:Readonly<{instance:string;platform?:NodeJS.Platform;timeoutMs?:number;run?:(command:string,args:readonly string[],milliseconds:number)=>Promise<string>}>):Promise<FleetDiscoveryV1|undefined>{
 const platform=input.platform??process.platform,run=input.run??collectTool,limit=input.timeoutMs??1500;
 if(platform==="linux")return parseAvahiBrowseV1(await run("avahi-browse",["-rtp",FLEET_SERVICE_TYPE_V1],limit),input.instance);
 const command=platform==="win32"?"dns-sd.exe":"dns-sd",found=parseDnsSdLookupV1(await run(command,["-L",input.instance,FLEET_SERVICE_TYPE_V1,"local"],limit));
 if(found===undefined)return undefined;
 const addresses=parseDnsSdAddressesV1(await run(command,["-G","v4v6",found.host],limit));
 return addresses.length===0?undefined:{addresses,port:found.port,txt:found.txt};
}

/** 🔌️ A probe that counts an endpoint as answering when a TCP connection to its host and port opens (for services that do not speak HTTP, such as the NFS port of the store). */
export function fleetTcpProbeV1(endpoint:string,signal:AbortSignal):Promise<boolean>{
 return new Promise<boolean>(resolve=>{
  let url:URL;
  try{url=new URL(endpoint);}catch{resolve(false);return;}
  const socket=createConnection({host:url.hostname.replace(/^\[|\]$/gu,""),port:Number(url.port)});
  const done=(ok:boolean):void=>{signal.removeEventListener("abort",abort);socket.destroy();resolve(ok);},abort=():void=>done(false);
  signal.addEventListener("abort",abort,{once:true});
  socket.once("connect",()=>done(true));
  socket.once("error",()=>done(false));
 });
}

/** 🧭️ THE resolver of every client of a fleet service: finds one service of one machine, named by its origin id, in one call and with one failover policy. Order: the endpoint that answered last time, then the addresses the machine published in the store (a copy of them kept on this machine serves while the store is unreadable), same subnet first; then DNS-SD discovery of the instance. A published set that is stale puts discovery before its addresses, because the machine may have moved to another network. Each attempt has its own time limit. The answer and the path that found it (`via`) are remembered and reported in the chosen language; `undefined` when nothing answers, so the caller falls back to local work. */
export async function resolveFleetEndpointV1(input:Readonly<{root:string;storeRoot:string;origin:string;service:string;port:number;local?:readonly FleetAddressV1[];probe:(endpoint:string,signal:AbortSignal)=>Promise<boolean>;discover?:(instance:string)=>Promise<readonly string[]>;timeoutMs?:number;signal?:AbortSignal;now?:number;report?:(line:string)=>void;language?:FleetAddressLanguageV1}>):Promise<Readonly<FleetConnectionV1&{via:FleetResolutionPathV1}>|undefined>{
 const key=`${input.origin}/${input.service}`,local=input.local??collectFleetAddressesV1(networkInterfaces()),read=await readFleetAddressSetV1(input.storeRoot,input.origin,input.now);
 if(read!==undefined)await mirrorAddresses(input.root,read.set);
 const mirrored=read===undefined?await readMirror(input.root,input.origin):undefined,set=read?.set??mirrored,fresh=read?.fresh??false,lastGood=await readFleetLastGoodV1(input.root,key);
 const known=fleetEndpointCandidatesV1({set,local,port:input.port}),scheme=lastGood===undefined||!lastGood.startsWith("https")?"http":"https";
 const discover=input.discover??(async(instance:string):Promise<readonly string[]>=>(await discoverFleetServiceV1({instance}))?.addresses??[]);
 const discovered=async():Promise<readonly string[]>=>{try{return (await discover(input.origin)).map(address=>`${scheme}://${fleetUrlHostV1(address)}:${input.port}`);}catch{return [];}};
 const attempts:{endpoint:string;ok:boolean;milliseconds:number}[]=[],tried=new Set<string>();
 const stage=async(candidates:readonly string[],via:FleetResolutionPathV1):Promise<Readonly<FleetConnectionV1&{via:FleetResolutionPathV1}>|undefined>=>{
  const fresh_=candidates.filter(endpoint=>!tried.has(endpoint));
  for(const endpoint of fresh_)tried.add(endpoint);
  if(fresh_.length===0)return undefined;
  const found=await connectFleetEndpointV1({candidates:fresh_,probe:input.probe,...input.timeoutMs===undefined?{}:{timeoutMs:input.timeoutMs},...input.signal===undefined?{}:{signal:input.signal},...input.report===undefined?{}:{report:input.report},...input.language===undefined?{}:{language:input.language}});
  if(found===undefined){for(const endpoint of fresh_)attempts.push({endpoint,ok:false,milliseconds:input.timeoutMs??1500});return undefined;}
  attempts.push(...found.attempts);
  return {endpoint:found.endpoint,attempts:[...attempts],via};
 };
 const stored=mirrored!==undefined?"mirror":"published";
 let found=lastGood===undefined?undefined:await stage([lastGood],"last-good");
 if(found===undefined&&fresh)found=await stage(known,stored);
 if(found===undefined)found=await stage(await discovered(),"dns-sd");
 if(found===undefined&&!fresh)found=await stage(known,stored);
 if(found===undefined){
  if(input.report!==undefined&&input.language!==undefined)input.report(fleetAddressTextV1(input.language,"addresses.unreachable",{origin:input.origin,count:tried.size}));
  return undefined;
 }
 await writeFleetLastGoodV1(input.root,key,found.endpoint);
 if(input.report!==undefined&&input.language!==undefined)input.report(fleetAddressTextV1(input.language,"addresses.resolved",{origin:input.origin,endpoint:found.endpoint,via:found.via}));
 return found;
}
