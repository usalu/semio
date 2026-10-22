import {spawnSync} from "node:child_process";
import {PackageManagerRegistryV1,type HostPortV1,type PackageManagerV1} from "../🟦️.ts";
import {bunPackageManagerV1} from "../🍞️bun/🟦️.ts";
import {npmPackageManagerV1} from "../📦️npm/🟦️.ts";
import {cargoPackageManagerV1} from "../🦀️cargo/🟦️.ts";
import {goPackageManagerV1} from "../🐹️go/🟦️.ts";
import {uvPackageManagerV1} from "../🐍️uv/🟦️.ts";
import {dotnetPackageManagerV1} from "../🔷️dotnet/🟦️.ts";

/** 📚️ Lists every built-in adapter; this is the only module that names them, so a new manager is one import and one line here. */
export const builtInPackageManagersV1:readonly PackageManagerV1[]=Object.freeze([bunPackageManagerV1,npmPackageManagerV1,cargoPackageManagerV1,goPackageManagerV1,uvPackageManagerV1,dotnetPackageManagerV1]);
/** 🗃️ The registry of the built-in adapters. */
export const packageManagerRegistryV1:PackageManagerRegistryV1=builtInPackageManagersV1.reduce((registry,manager)=>registry.register(manager),new PackageManagerRegistryV1());
/** 🥾️ Selects the manager of one ecosystem from the policy table, overridable per ecosystem through the neutral environment variable. */
export const selectPackageManagerV1=(ecosystem:string,environment:Readonly<Record<string,string|undefined>>=process.env):PackageManagerV1=>packageManagerRegistryV1.select(ecosystem,environment);
/** 🔬️ Asks the tool itself for its version through a child process. */
/** 🖥️ Describes the running host for keys and runtime identity; the version map is the process's own, so embedded managers answer without a probe. */
export const spawnProbeV1:NonNullable<HostPortV1["probe"]>=(executable,args)=>{const result=spawnSync(executable,[...args],{encoding:"utf8"});return result.status===0?result.stdout:undefined;};
export function currentHostV1(probe:HostPortV1["probe"]=spawnProbeV1):HostPortV1{return Object.freeze({platform:process.platform,arch:process.arch,versions:process.versions,probe});}
