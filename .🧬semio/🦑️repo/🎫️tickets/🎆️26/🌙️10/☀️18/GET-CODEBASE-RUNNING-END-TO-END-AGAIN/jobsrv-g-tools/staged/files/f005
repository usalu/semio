import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";

/** 🔬 The third-party oracle for everything about job-token pools: the `jobserver` crate, the code Cargo, rustc and `cc` run to find the pool. The probe asks it for a client the way rustc does (pipes must be pipes) and counts the tokens that are free (`try_acquire`, or `available` where a pipe does not support it, as on macOS), so its verdict says whether a pool is reachable and how many extra slots it offers. */
const MAIN=`use jobserver::Client;
fn text(value: Option<String>) -> String { match value { Some(value) => format!("{:?}", value), None => "null".to_string() } }
fn main() {
    let from = unsafe { Client::from_env_ext(true) };
    let variable = from.var.as_ref().map(|(name, _)| name.to_string());
    let pool = match &from.client {
        Ok(client) => {
            let mut held = Vec::new();
            let mut unsupported = false;
            while held.len() < 256 { match client.try_acquire() { Ok(Some(token)) => held.push(token), Ok(None) => break, Err(_) => { unsupported = true; break } } }
            let tokens = if unsupported { client.available().map(|count| count as i64).unwrap_or(-1) } else { held.len() as i64 };
            format!("{{\\"client\\":true,\\"tokens\\":{}}}", tokens)
        }
        Err(error) => format!("{{\\"client\\":false,\\"kind\\":\\"{:?}\\"}}", error.kind()),
    };
    println!("{{\\"variable\\":{},\\"numJobs\\":{},\\"pool\\":{}}}", text(variable), text(std::env::var("NUM_JOBS").ok()), pool);
}
`;

/** 🌍️ The environment of a Cargo run in a scratch directory: no repository configuration, no wrapper and no pool of a surrounding build leaks in. */
export function plainCargoEnvironmentV1(extra:Readonly<Record<string,string>>={}):NodeJS.ProcessEnv{
 const environment:NodeJS.ProcessEnv={...process.env};
 for(const name of["RUSTC_WRAPPER","RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","MAKEFLAGS","CARGO_MAKEFLAGS","MFLAGS","CARGO_BUILD_JOBS","NUM_JOBS","SEMIO_UNIT_CACHE","SEMIO_UNIT_CACHE_ROOT","SEMIO_UNIT_CACHE_JOBSERVER_FDS"])delete environment[name];
 return{...environment,...extra};
}

/** 🛠️ Runs Cargo offline when the registry cache has what the manifest needs and online otherwise; returns the finished process. */
export function cargoWithRegistryV1(cargo:string,args:readonly string[],options:Readonly<{cwd:string;env:NodeJS.ProcessEnv}>):ReturnType<typeof spawnSync>{
 const offline=spawnSync(cargo,[...args,"--offline"],{...options,encoding:"utf8",maxBuffer:1<<26});
 return offline.status===0?offline:spawnSync(cargo,[...args],{...options,encoding:"utf8",maxBuffer:1<<26});
}

/** 🧱️ Builds the probe once into `directory` and answers the executable. */
export function buildJobserverProbeV1(directory:string,cargo:string):string{
 mkdirSync(join(directory,"src"),{recursive:true});
 writeFileSync(join(directory,"Cargo.toml"),'[package]\nname = "jprobe"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n[dependencies]\njobserver = "0.1"\n');
 writeFileSync(join(directory,"src","main.rs"),MAIN);
 const run=cargoWithRegistryV1(cargo,["build"],{cwd:directory,env:plainCargoEnvironmentV1({CARGO_TARGET_DIR:join(directory,"t"),CARGO_INCREMENTAL:"0"})});
 const binary=join(directory,"t","debug","jprobe");
 if(run.status!==0||!existsSync(binary))throw Error("the jobserver probe did not build: "+String(run.stderr));
 return binary;
}

/** 📋️ What the probe printed. */
export type ProbeVerdictV1=Readonly<{variable:string|null;numJobs:string|null;pool:Readonly<{client:true;tokens:number}|{client:false;kind:string}>}>;
