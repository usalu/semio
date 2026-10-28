import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗻️ The declared type rules; the schema file is their single source. */
export const CHANGE_FILE_SYSTEM_POLICY_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{version:1;linux:{exact:string[];prefixes:string[]};darwin:{exact:string[];prefixes:string[]};darwinFirmlinkPrefix:string;darwinLocalFlag:string;darwinLocalTypes:string[];windowsNetworkDriveType:string;windowsUncPrefixes:string[];probeTimeoutMilliseconds:number}>);
/** 🗻️ A file system that does not report edits made on its other side: its type name and the mount point (or share) it was found at. */
export type ChangeRemoteFileSystemV1=Readonly<{type:string;mount:string}>;
/** 🧾️ What the host says about the file system of a root: the mount table of its platform, or the drive type of its Windows drive. */
export type ChangeFileSystemFactsV1=Readonly<{platform:string;root:string;mountinfo?:string;mountTable?:string;driveType?:string}>;
/** 🔌️ Everything the facts need from the host: the platform, a text file and the output of one program; both answer nothing where they fail. */
export type ChangeFileSystemPortsV1=Readonly<{platform:string;readText(path:string):Promise<string|undefined>;run(program:string,args:readonly string[]):Promise<string|undefined>}>;

const within=(root:string,mount:string):boolean=>mount==="/"?root.startsWith("/"):root===mount||root.startsWith(mount+"/");
const matches=(rule:{exact:readonly string[];prefixes:readonly string[]},type:string):boolean=>{const name=type.toLowerCase();return rule.exact.includes(name)||rule.prefixes.some(prefix=>name.startsWith(prefix));};
const unescapeMount=(value:string):string=>value.replace(/\\([0-7]{3})/gu,(_all,octal:string)=>String.fromCharCode(parseInt(octal,8)));

/** 🔎️ The mount a root lies on in a Linux `/proc/self/mountinfo` table: the longest mount point that contains the root, the later row winning a tie, as the kernel stacks mounts. */
export function linuxMountOfV1(mountinfo:string,root:string):Readonly<{mount:string;type:string}>|undefined{
 let best:{mount:string;type:string}|undefined;
 for(const line of mountinfo.split("\n")){
  const dash=line.indexOf(" - ");if(dash<0)continue;
  const before=line.slice(0,dash).split(" "),after=line.slice(dash+3).split(" ");
  if(before.length<5||after.length<1||!after[0])continue;
  const mount=unescapeMount(before[4]!);
  if(within(root,mount)&&(best===undefined||mount.length>=best.mount.length))best={mount,type:after[0]};
 }
 return best;
}

const unfirmlink=(path:string):string=>{const prefix=CHANGE_FILE_SYSTEM_POLICY_V1.darwinFirmlinkPrefix;return path===prefix?"/":path.startsWith(prefix+"/")?path.slice(prefix.length):path;};

/** 🔎️ The mount a root lies on in the output of macOS `mount`: `source on /point (type, flag, flag)`; the longest containing mount point wins, and the data volume's firmlink prefix is the same place as the plain path (`/Users` is `/System/Volumes/Data/Users`), a later row winning a tie as the volume group lists the data volume after the system volume. */
export function darwinMountOfV1(table:string,root:string):Readonly<{mount:string;type:string;flags:readonly string[]}>|undefined{
 let best:{mount:string;type:string;flags:string[]}|undefined,bestLength=-1;
 const place=unfirmlink(root);
 for(const line of table.split("\n")){
  const match=/^(.+?) on (.+) \(([^()]*)\)$/u.exec(line.trimEnd());if(!match)continue;
  const mount=match[2]!,parts=match[3]!.split(",").map(part=>part.trim()).filter(Boolean),alias=unfirmlink(mount);
  if(parts.length===0||!within(place,alias))continue;
  if(best===undefined||alias.length>=bestLength){best={mount,type:parts[0]!,flags:parts.slice(1)};bestLength=alias.length;}
 }
 return best;
}

/** ⚖️ Decides from the facts whether the file system of a root is one whose far-side edits no watcher reports; `undefined` where it is a local one or where nothing can be said (then the watcher's own canary decides). */
export function changeFileSystemVerdictV1(facts:ChangeFileSystemFactsV1):ChangeRemoteFileSystemV1|undefined{
 const policy=CHANGE_FILE_SYSTEM_POLICY_V1;
 if(facts.platform==="linux"){
  const found=facts.mountinfo===undefined?undefined:linuxMountOfV1(facts.mountinfo,facts.root);
  return found&&matches(policy.linux,found.type)?{type:found.type,mount:found.mount}:undefined;
 }
 if(facts.platform==="darwin"){
  const found=facts.mountTable===undefined?undefined:darwinMountOfV1(facts.mountTable,facts.root);
  if(!found)return undefined;
  const local=found.flags.includes(policy.darwinLocalFlag),known=policy.darwinLocalTypes.includes(found.type.toLowerCase());
  return matches(policy.darwin,found.type)||(!local&&!known)?{type:found.type,mount:found.mount}:undefined;
 }
 if(facts.platform==="win32"){
  const unc=policy.windowsUncPrefixes.find(prefix=>facts.root.startsWith(prefix));
  if(unc!==undefined)return {type:"unc",mount:facts.root.slice(0,unc.length)+facts.root.slice(unc.length).split(/[\\/]/u).slice(0,2).join(unc[0]!)};
  return facts.driveType?.trim()===policy.windowsNetworkDriveType?{type:"network-drive",mount:facts.root.slice(0,2)}:undefined;
 }
 return undefined;
}

/** 🗻️ Asks the host about the file system of a root and decides; a failing question is an unknown, never a refusal of its own. */
export async function inspectChangeFileSystemV1(root:string,ports:ChangeFileSystemPortsV1):Promise<ChangeRemoteFileSystemV1|undefined>{
 const {platform}=ports;
 if(platform==="linux")return changeFileSystemVerdictV1({platform,root,...await optional("mountinfo",ports.readText("/proc/self/mountinfo"))});
 if(platform==="darwin")return changeFileSystemVerdictV1({platform,root,...await optional("mountTable",ports.run("/sbin/mount",[]))});
 if(platform==="win32"){
  if(CHANGE_FILE_SYSTEM_POLICY_V1.windowsUncPrefixes.some(prefix=>root.startsWith(prefix)))return changeFileSystemVerdictV1({platform,root});
  const drive=/^([A-Za-z]):/u.exec(root);
  if(!drive)return undefined;
  return changeFileSystemVerdictV1({platform,root,...await optional("driveType",ports.run("powershell.exe",["-NoProfile","-NonInteractive","-Command",`(Get-CimInstance Win32_LogicalDisk -Filter "DeviceID='${drive[1]!.toUpperCase()}:'").DriveType`]))});
 }
 return undefined;
}
async function optional<K extends "mountinfo"|"mountTable"|"driveType">(key:K,answer:Promise<string|undefined>):Promise<Partial<Record<K,string>>>{
 try{const text=await answer;return text===undefined?{}:{[key]:text} as Partial<Record<K,string>>;}catch{return {};}
}
