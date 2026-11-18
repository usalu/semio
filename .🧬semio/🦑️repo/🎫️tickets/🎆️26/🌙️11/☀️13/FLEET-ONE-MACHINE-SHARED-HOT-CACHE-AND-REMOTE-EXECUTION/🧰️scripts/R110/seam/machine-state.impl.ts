import {isAbsolute,join} from "node:path";

/** 🏠️ The variable that moves the machine state: an absolute directory that stands for `<worktree>/.cache/f`, so several machines can share one checkout (the rehearsal of a fleet) and every file a machine keeps for itself (identity, facts, pool, ready, graph state) stays apart. */
export const MACHINE_STATE_ENVIRONMENT_V1="SEMIO_MACHINE_STATE_DIRECTORY";

/** 🗂️ The directory in which a machine keeps what is its own: the declared one, else `<worktree>/.cache/f`. A relative or empty declaration is ignored. */
export function machineStateDirectoryV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const named=environment[MACHINE_STATE_ENVIRONMENT_V1];
 return named!==undefined&&named!==""&&isAbsolute(named)?named:join(root,".cache","f");
}

/** 🧭️ A path below the worktree; one that starts with `.cache/f` (what the policies of the build declare) is moved to the machine state directory. */
export function machineStatePathV1(root:string,relative:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const text=relative.replaceAll("\\","/");
 if(text===".cache/f")return machineStateDirectoryV1(root,environment);
 return text.startsWith(".cache/f/")?join(machineStateDirectoryV1(root,environment),...text.slice(9).split("/")):join(root,...text.split("/"));
}
