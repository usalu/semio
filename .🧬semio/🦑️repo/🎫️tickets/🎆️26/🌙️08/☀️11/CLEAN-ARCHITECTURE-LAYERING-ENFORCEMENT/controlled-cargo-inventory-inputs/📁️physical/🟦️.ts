import {open,lstat,opendir,type FileHandle} from "node:fs/promises";
import type {Dir,Dirent} from "node:fs";
import {constants} from "node:fs";
import {resolve,relative,isAbsolute,join} from "node:path";
import {CargoController,type CargoControl,type CargoRetirementOperation} from "../🎛️control/🟦️.ts";
import {CargoTomlWorkspace,parseCargoTomlChunks,type CargoTomlTable} from "../📖️toml/🟦️.ts";

export type CargoPhysicalEntry=Readonly<{name:string;kind:"file"|"directory"|"symlink"|"other"}>;
export class CargoDiscoveryWorkspace {
 completed=0;ownedBytes=0;retirementIndex=0;state:"active"|"prepared"|"published"|"retiring"|"retired"="active";
 private rootInput:string|null=null;private rootAbsolute:string|null=null;
 readonly manifests=new Map<string,CargoManifestOwner>();readonly manifestOwners:CargoManifestOwner[]=[];
 readonly directories:CargoDirectoryOwner[]=[];readonly directoryIndex=new Map<string,CargoDirectoryOwner>();
 readonly partialRecords:unknown[]=[];
 async bind(root:string,control:CargoController):Promise<void>{if(root.length>4096)throw Error("Cargo root exceeds finite owned scope");if(this.rootInput===root){await control.step("repository-root",root);return;}await control.step("repository-root-owner",root,root.length+1,32+root.length*2);const full=resolve(root);if(this.rootAbsolute!==null&&this.rootAbsolute!==full)throw Error("Cargo discovery workspace cannot mix physical roots");this.rootInput=root;this.rootAbsolute=full;}
}
export interface CargoDiscoveryOperation extends CargoControl {readonly workspace:CargoDiscoveryWorkspace;}

export class CargoManifestOwner {
 readonly pages:string[]=[];readonly characters:string[][]=[];readonly parser:CargoTomlWorkspace;
 private handle:FileHandle|null=null;private buffer:Uint8Array|null=null;private pending:number[]=[];
 state:"opening"|"reading"|"parsing"|"source-complete"|"complete"|"refused"="opening";document:CargoTomlTable|null=null;
 constructor(readonly path:string,workspace:CargoDiscoveryWorkspace){this.parser=new CargoTomlWorkspace(workspace);}
 async read(full:string,identity:Readonly<{dev:number;ino:number}>,control:CargoController):Promise<void>{
  this.handle=await open(full,constants.O_RDONLY|(constants.O_NOFOLLOW??0));await control.step("file-identity",this.path);const actual=await this.handle.stat();if(!actual.isFile()||actual.dev!==identity.dev||actual.ino!==identity.ino)throw Error("Cargo manifest changed during physical admission");await control.step("read-buffer",this.path,1,4096);this.buffer=new Uint8Array(4096);this.state="reading";let readBytes=0,characters:string[]=[];await control.step("text-page",this.path,1,64);this.characters.push(characters);
  while(true){await control.step("read",this.path);const result=await this.handle.read(this.buffer,0,this.buffer.length,null);if(!result.bytesRead)break;readBytes+=result.bytesRead;for(let index=0;index<result.bytesRead;index++){await control.step("utf8",this.path,1,16);const byte=this.buffer[index]!;this.pending.push(byte);const head=this.pending[0]!,width=head<128?1:head>=194&&head<=223?2:head>=224&&head<=239?3:head>=240&&head<=244?4:0;if(!width)throw Error("Cargo manifest has invalid UTF8");if(this.pending.length<width)continue;let point=head&(width===1?127:width===2?31:width===3?15:7);for(let tail=1;tail<width;tail++){const value=this.pending[tail]!;if(value<128||value>191)throw Error("Cargo manifest has invalid UTF8 continuation");point=(point<<6)|(value&63);}if(width===2&&point<128||width===3&&point<2048||width===4&&point<65536||point>0x10ffff||point>=0xd800&&point<=0xdfff)throw Error("Cargo manifest has invalid UTF8 scalar");await control.step("source-character",this.path,1,20);characters.push(String.fromCodePoint(point));this.pending=[];if(characters.length===1024){await control.step("source-page",this.path,1,16+characters.length*4);this.pages.push(characters.join(""));await control.step("text-page",this.path,1,64);characters=[];this.characters.push(characters);}}}
  if(this.pending.length)throw Error("Cargo manifest has truncated UTF8");if(characters.length){await control.step("source-page",this.path,1,16+characters.length*4);this.pages.push(characters.join(""));}await control.step("read-consistency",this.path);const final=await this.handle.stat();if(readBytes!==actual.size||final.size!==actual.size||final.mtimeMs!==actual.mtimeMs||final.ctimeMs!==actual.ctimeMs)throw Error("Cargo manifest changed during bounded read");await control.step("file-close",this.path);await this.handle.close();this.handle=null;
 }
 async close(control:CargoController):Promise<void>{if(this.handle){await control.step("retire-file",this.path);await this.handle.close();this.handle=null;}}
}

export class CargoDirectoryOwner {
 private handle:Dir|null=null;private pending:Dirent|null=null;private readonly entries:CargoPhysicalEntry[]=[];complete=false;
 constructor(readonly path:string){}
 async admit(full:string,control:CargoController):Promise<void>{await control.step("directory-open",this.path,1,256);this.handle=await opendir(full,{bufferSize:1});}
 async at(index:number,control:CargoController):Promise<CargoPhysicalEntry|null>{control.check();if(index<this.entries.length){await control.step("directory-entry-retained",this.path);return this.entries[index]!;}if(index!==this.entries.length)throw Error("Cargo directory cursor skips an admitted entry");if(this.complete)return null;if(!this.handle)throw Error("Cargo directory admission is incomplete");if(!this.pending){await control.step("directory-entry",this.path);this.pending=await this.handle.read();}if(!this.pending){await this.close(control);this.complete=true;return null;}const entry=this.pending;await control.step("directory-entry-owner",this.path,1,64+entry.name.length*2);const row:CargoPhysicalEntry={name:entry.name,kind:entry.isSymbolicLink()?"symlink":entry.isFile()?"file":entry.isDirectory()?"directory":"other"};this.entries.push(row);this.pending=null;return row;}
 async close(control:CargoController):Promise<void>{if(this.handle){await control.step("retire-directory",this.path);await this.handle.close();this.handle=null;}if(this.pending){await control.step("retire-directory-entry",this.path);this.pending=null;}}
}

export class CargoDirectoryCursor {
 private index=0;
 constructor(private readonly owner:CargoDirectoryOwner){}
 async next(control:CargoController):Promise<CargoPhysicalEntry|null>{const entry=await this.owner.at(this.index,control);if(entry)this.index++;return entry;}
}

/** 📁️ Checks physical ancestry with admitted system calls and an owned continuation. */
export async function cargoPhysicalPath(root:string,path:string,operation:CargoDiscoveryOperation,kind:"file"|"directory"):Promise<Readonly<{full:string;dev:number;ino:number}>>{
 const control=new CargoController(operation,operation.workspace);await operation.workspace.bind(root,control);if(path.length>4096)throw Error("Cargo physical path exceeds finite owned scope");await control.step("path",path,path.length+1,32+path.length*2);const full=resolve(root,path),rel=relative(resolve(root),full).replaceAll("\\","/");if(rel.startsWith("../")||isAbsolute(rel))throw Error("Cargo physical path escapes repository");const parts=rel.split("/").filter(Boolean);await control.step("path-parts",path,parts.length,32+rel.length*2);let current=resolve(root),identity=await lstat(current);if(identity.isSymbolicLink()||!identity.isDirectory())throw Error("Cargo repository root is not a physical directory");for(const part of parts){await control.step("path-ancestor",path);current=join(current,part);identity=await lstat(current);if(identity.isSymbolicLink())throw Error("Cargo physical ancestry contains a symlink");}if(kind==="file"?!identity.isFile():!identity.isDirectory())throw Error("Cargo physical path has the wrong kind");return{full,dev:identity.dev,ino:identity.ino};
}

/** 📄️ Reads and parses one admitted manifest, retaining every partial owner on refusal. */
export async function cargoManifestDocument(root:string,path:string,operation:CargoDiscoveryOperation):Promise<CargoTomlTable>{
 const control=new CargoController(operation,operation.workspace);await operation.workspace.bind(root,control);await control.step("manifest-cache",path);if(operation.workspace.state!=="active")throw Error("Cargo discovery workspace is retiring");const previous=operation.workspace.manifests.get(path);if(previous){if(previous.state!=="complete"||!previous.document)throw Error("Cargo manifest owner is incomplete");return previous.document;}const identity=await cargoPhysicalPath(root,path,operation,"file");await control.step("manifest-owner",path,1,256+path.length*2);const owner=new CargoManifestOwner(path,operation.workspace);operation.workspace.manifests.set(path,owner);operation.workspace.manifestOwners.push(owner);
 try{await owner.read(identity.full,identity,control);owner.state="parsing";owner.document=await parseCargoTomlChunks(owner.pages,owner.parser,operation,path);owner.state="complete";return owner.document;}catch(error){owner.state="refused";throw error;}
}

/** 🗂️ Opens a caller-retained physical directory cursor without bulk enumeration. */
export async function cargoPhysicalDirectory(root:string,path:string,operation:CargoDiscoveryOperation):Promise<CargoDirectoryCursor>{const control=new CargoController(operation,operation.workspace);await operation.workspace.bind(root,control);if(path.length>4096)throw Error("Cargo physical path exceeds finite owned scope");await control.step("directory-cache",path,path.length+1);if(operation.workspace.state!=="active")throw Error("Cargo discovery workspace is retiring");const key=resolve(root,path);let owner=operation.workspace.directoryIndex.get(key);if(!owner){const identity=await cargoPhysicalPath(root,path,operation,"directory");await control.step("directory-owner",path,1,128+path.length*2);owner=new CargoDirectoryOwner(path);operation.workspace.directories.push(owner);operation.workspace.directoryIndex.set(key,owner);await owner.admit(identity.full,control);}await control.step("directory-cursor-owner",path,1,48);const cursor=new CargoDirectoryCursor(owner);operation.workspace.partialRecords.push(cursor);return cursor;}

/** ♻️ Closes retained system owners under a fresh explicit retirement control. */
export async function closeCargoDiscoveryOwners(workspace:CargoDiscoveryWorkspace,operation:CargoRetirementOperation):Promise<void>{const control=new CargoController(operation,operation.accounting);workspace.state="retiring";const manifests=workspace.manifestOwners;for(;workspace.retirementIndex<manifests.length+workspace.directories.length;workspace.retirementIndex++){const owner=workspace.retirementIndex<manifests.length?manifests[workspace.retirementIndex]!:workspace.directories[workspace.retirementIndex-manifests.length]!;await owner.close(control);}await control.finish("system-owners-closed","");}
