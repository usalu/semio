import {join} from "node:path";
import {CargoController} from "../🎛️control/🟦️.ts";
import {cargoManifestDocument,type CargoDiscoveryOperation} from "../📁️physical/🟦️.ts";
import {cargoPatternMatches} from "../🔎️pattern/🟦️.ts";
import {cargoRepositoryPackage,parseCargoWorkspaceContribution} from "../🟦️.ts";
import type {CargoTomlTable} from "../📖️toml/🟦️.ts";
const isTable=(value:unknown):value is CargoTomlTable=>Boolean(value)&&typeof value==="object"&&!Array.isArray(value);

/** 📐️ Resolves authored membership from the same admitted physical declaration. */
export async function cargoWorkspaceDeclaresMember(root:string,manifest:string,directory:string,operation:CargoDiscoveryOperation):Promise<boolean>{const document=await cargoManifestDocument(root,manifest,operation);if(!isTable(document.workspace))throw Error("Cargo membership requires a workspace declaration");const workspace=document.workspace,contribution=await parseCargoWorkspaceContribution({schemaVersion:1,members:workspace.members,exclude:workspace.exclude??[]},operation);let member=false;for(const pattern of contribution.members)if(await cargoPatternMatches(pattern,directory,operation)){member=true;break;}if(!member)return false;for(const pattern of contribution.exclude)if(await cargoPatternMatches(pattern,directory,operation))return false;return true;}

/** 🎛️ Admits each literal argument and binds exact current selectors to one physical owner. */
export async function selectedCargoArguments(root:string,args:readonly string[],operation:CargoDiscoveryOperation):Promise<readonly string[]>{
 const control=new CargoController(operation,operation.workspace);await control.step("cargo-arguments-owner","",1,128);const copied:string[]=[],names:string[]=[];operation.workspace.partialRecords.push(copied,names);let explicit=false;
 for(let index=0;index<args.length;index++){const argument=args[index]!;if(argument.length>4096)throw Error("Cargo argument exceeds finite owned scope");for(const character of argument){await control.step("cargo-argument",argument);if(character==="\0")throw Error("Cargo argument contains NUL");}await control.step("cargo-argument-owner",argument,1,16);copied.push(argument);if(argument==="--manifest-path"){if(!args[index+1])throw Error("Cargo manifest selector lacks a path");explicit=true;}else if(argument.startsWith("--manifest-path=")){if(argument.length===16)throw Error("Cargo manifest selector lacks a path");explicit=true;}}
 if(explicit)return copied;if(!copied.length)throw Error("Cargo command is absent");for(let index=1;index<copied.length;index++){await control.step("cargo-selector",copied[index]!);const argument=copied[index]!;if(argument==="--")break;if(argument==="-p"||argument==="--package"){const value=copied[++index];if(!value)throw Error("Cargo package selector lacks a name");await control.step("cargo-selector-owner",value,1,16);names.push(value);}else if(argument.startsWith("--package=")){if(argument.length===10)throw Error("Cargo package selector lacks a name");await control.step("cargo-selector-owner",argument,1,32+argument.length*2);names.push(argument.slice(10));}}
 if(!names.length)return copied;let workspace:string|undefined,manifest:string|undefined;for(const name of names){const row=await cargoRepositoryPackage(root,name,operation);if(workspace!==undefined&&workspace!==row.workspace)throw Error("Cargo command selects different physical workspaces");workspace=row.workspace;manifest??=row.manifest;}await control.step("cargo-selected-arguments-owner",manifest!,1,64+root.length*2+manifest!.length*2);const selected:string[]=[copied[0]!,"--manifest-path",join(root,manifest!)];operation.workspace.partialRecords.push(selected);for(let index=1;index<copied.length;index++){await control.step("cargo-selected-argument",copied[index]!,1,16);selected.push(copied[index]!);}return selected;
}


export type CargoMembershipFact=Readonly<{directory:string;declared:boolean}>;

/** 🧾️ Captures a bounded first-party membership fact from the exact admitted physical declaration. */
export async function admitCargoMembership(root:string,manifest:string,directory:string,operation:CargoDiscoveryOperation):Promise<CargoMembershipFact>{if(!directory||directory.length>4096||directory.includes("\0")||directory.includes("\\")||directory.startsWith("/")||directory.split("/").includes(".."))throw Error("Cargo membership requires a finite owned directory");const declared=await cargoWorkspaceDeclaresMember(root,manifest,directory,operation);await new CargoController(operation,operation.workspace).step("membership-fact-owner",manifest,directory.length+1,64+directory.length*2);const fact={directory,declared};operation.workspace.partialRecords.push(fact);return fact;}
