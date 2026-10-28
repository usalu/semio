import{join}from"node:path";
import{resolveLanguageV1}from"../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";

/** 📌️ One row of the launcher registry that matters to a guard: the pinned archive of the host. */
export type PinnedDistributionV1=Readonly<{platform:string;arch:string;libc:string;archive:string;bytes:number;sha256:string}>;

/** 🗂️ The part of the launcher registry the guard reads; the Go launcher reads the same file. */
export type RuntimePinRegistryV1=Readonly<{
  selected:string;
  runtimes:Readonly<Record<string,Readonly<{version:string;executable:Readonly<{unix:string;windows:string}>;distributions:readonly PinnedDistributionV1[]}>>>;
  cache:Readonly<{environment:string;runtimeEnvironment:string;toolsPartition:readonly string[];user:readonly string[]}>;
  messages:Readonly<Record<string,Readonly<Record<string,string>>>>;
  languages:readonly string[];
}>;

/** 🖥️ The host in registry terms (platform, architecture, C library) with the operating system's cache directory; the Go launcher derives the same from its runtime. */
export type RuntimePinHostV1=Readonly<{
  root:string;platform:string;arch:string;musl:boolean;
  environment:Readonly<Record<string,string|undefined>>;
  userCacheDirectory:string|undefined;
  installed:(directory:string,executable:string,sha256:string)=>boolean;
}>;

/** 📍️ Where a runtime of the registry lives on this host, with the two stores in lookup order. */
export type PinnedRuntimeV1=Readonly<{name:string;version:string;sha256:string;executable:string;tools:string;user:string|undefined}>;

/** 🧭️ What a guard does when the running Bun is not the pinned one. */
export type GuardDecisionV1="proceed"|"reexec"|"delegate"|"refuse-test"|"refuse-missing"|"refuse-loop";

/** 🎯️ Picks the pinned archive of a host: the first row with the host platform and architecture whose libc is `any` or the host's. */
export function pinnedDistributionV1(rows:readonly PinnedDistributionV1[],platform:string,arch:string,musl:boolean):PinnedDistributionV1|undefined{
  const libc=musl?"musl":"gnu";
  return rows.find(row=>row.platform===platform&&row.arch===arch&&(row.libc==="any"||row.libc===libc));
}

/** 🏪️ Names the runtime of the registry for a host: the `SEMIO_RUNTIME` choice, else the selected one, with its tools partition directory and its user cache directory (`SEMIO_RUNTIME_CACHE`, else the operating system's cache directory). */
export function pinnedRuntimeV1(registry:RuntimePinRegistryV1,host:RuntimePinHostV1):PinnedRuntimeV1{
  const name=host.environment[registry.cache.runtimeEnvironment]||registry.selected,row=registry.runtimes[name];
  if(row===undefined)throw new Error(`${name} is not declared (declared: ${Object.keys(registry.runtimes).sort().join(", ")})`);
  const distribution=pinnedDistributionV1(row.distributions,host.platform,host.arch,host.musl);
  if(distribution===undefined)throw new Error(`${name} ${row.version} has no pinned distribution for ${host.platform}/${host.arch}${host.musl?"/musl":""}`);
  const base=host.environment[registry.cache.environment]||(host.userCacheDirectory===undefined?undefined:join(host.userCacheDirectory,...registry.cache.user));
  return Object.freeze({name,version:row.version,sha256:distribution.sha256,executable:host.platform==="win32"?row.executable.windows:row.executable.unix,tools:join(host.root,...registry.cache.toolsPartition,name,distribution.sha256),user:base===undefined?undefined:join(base,name,distribution.sha256)});
}

/** 🔎️ Finds the installed executable without side effects: the tools partition first, then the user cache; an install counts when its receipt names the pinned archive. */
export function locatePinnedRuntimeV1(pin:PinnedRuntimeV1,host:Pick<RuntimePinHostV1,"installed">):Readonly<{path:string;origin:"tools-partition"|"user-cache"}>|undefined{
  for(const[directory,origin]of[[pin.tools,"tools-partition"],[pin.user,"user-cache"]]as const){
    if(directory!==undefined&&host.installed(directory,pin.executable,pin.sha256))return Object.freeze({path:join(directory,pin.executable),origin});
  }
  return undefined;
}

/** ⚖️ Decides for the running Bun: the pinned one proceeds; another one re-runs the command line under the provisioned pinned Bun or under the `repo` launcher, which provisions it, and is refused when it was already started by a guard, when the test runner's command line cannot be read, or when neither exists. */
export function guardDecisionV1(input:Readonly<{running:string;pin:string;mode:"run"|"test";commandLine:boolean;located:boolean;launcher:boolean;depth:number}>):GuardDecisionV1{
  if(input.running===input.pin)return"proceed";
  if(input.depth>0)return"refuse-loop";
  if(input.mode==="test"&&!input.commandLine)return"refuse-test";
  return input.located?"reexec":input.launcher?"delegate":"refuse-missing";
}

/** 🪟️ Splits a Windows command line into its arguments by the rules of the C runtime (`CommandLineToArgvW`): the program name ends at the first space outside quotes, backslashes count only before a quote, and a quote doubled inside quotes is a literal quote. */
export function splitWindowsCommandLineV1(line:string):readonly string[]{
  const out:string[]=[];let index=0;
  const skip=()=>{while(line[index]===" "||line[index]==="\t")index++;};
  skip();
  if(index<line.length){
    let program="";
    if(line[index]==="\""){index++;while(index<line.length&&line[index]!=="\"")program+=line[index++];index++;}
    else while(index<line.length&&line[index]!==" "&&line[index]!=="\t")program+=line[index++];
    out.push(program);
  }
  for(skip();index<line.length;skip()){
    let argument="",quoted=false;
    while(index<line.length&&(quoted||line[index]!==" "&&line[index]!=="\t")){
      let slashes=0;
      while(line[index]==="\\"){slashes++;index++;}
      if(line[index]==="\""){
        argument+="\\".repeat(Math.floor(slashes/2));
        if(slashes%2===1)argument+="\"";
        else if(quoted&&line[index+1]==="\""){argument+="\"";index++;}
        else quoted=!quoted;
        index++;
      }else{
        argument+="\\".repeat(slashes);
        if(index<line.length&&(quoted||line[index]!==" "&&line[index]!=="\t")){argument+=line[index];index++;}
      }
    }
    out.push(argument);
  }
  return Object.freeze(out);
}

/** 🧩️ Reads the argument vector of a macOS process from the `KERN_PROCARGS2` record the kernel keeps: an argument count, the executable path, padding zeros, then the arguments. */
export function parseProcessArguments2V1(bytes:Uint8Array):readonly string[]|undefined{
  if(bytes.length<8)return undefined;
  const count=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength).getInt32(0,true),decoder=new TextDecoder();
  let index=4;
  const take=():string|undefined=>{const start=index;while(index<bytes.length&&bytes[index]!==0)index++;if(index>=bytes.length)return undefined;const text=decoder.decode(bytes.subarray(start,index));index++;return text;};
  if(take()===undefined)return undefined;
  while(bytes[index]===0)index++;
  const out:string[]=[];
  for(let item=0;item<count;item++){const text=take();if(text===undefined)return undefined;out.push(text);}
  return count>0?Object.freeze(out):undefined;
}

/** 🧵️ Splits the NUL-separated command line of a Linux process (`/proc/<pid>/cmdline`). */
export function parseProcCmdlineV1(bytes:Uint8Array):readonly string[]|undefined{
  const text=new TextDecoder().decode(bytes);
  return text===""?undefined:Object.freeze(text.replace(/\0$/u,"").split("\0"));
}

/** 🗣️ Renders one registry message in the resolved language; with no language chosen every offered language is shown. */
export function guardMessageV1(registry:RuntimePinRegistryV1,key:string,values:Readonly<Record<string,string>>,environment:Readonly<Record<string,string|undefined>>):string{
  const text=registry.messages[key],fill=(template:string)=>template.replace(/\{([A-Za-z]+)\}/gu,(_all,name:string)=>values[name]??`{${name}}`);
  if(text===undefined)throw new Error(`message ${key} is missing in the registry`);
  try{return fill(text[resolveLanguageV1(registry.languages,[],environment,()=>[]).language]!);}
  catch{return registry.languages.map(language=>fill(text[language]!)).join("\n");}
}
