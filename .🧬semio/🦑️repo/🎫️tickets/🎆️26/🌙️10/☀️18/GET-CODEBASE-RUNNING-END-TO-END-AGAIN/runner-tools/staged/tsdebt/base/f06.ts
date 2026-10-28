/** 🧱️ Contract: the shared build-dir lease admits same-profile compilers concurrently BY DESIGN, so exclusion of one unit's writers is Cargo's own fine-grain unit lock; this law runs the repository's pinned Cargo concurrently on one shared build directory and proves, with an independent Python interval oracle, that two build-script executions of the same unit never overlap. Everything a non-Cargo reader takes from that directory afterwards (observation, retention) is therefore NOT protected and is re-validated as a concurrent edit by the churn policy. */
import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),root=resolve(here,"../../../../../../../..");
const toolchains=join(root,".cache/b/t/m/r/toolchains"),nightly=existsSync(toolchains)?readdirSync(toolchains).filter(name=>name.startsWith("nightly-")).sort().at(-1):undefined;
const cargo=nightly?join(toolchains,nightly,"bin/cargo"):undefined;
const files={
 "Cargo.toml":'[package]\nname = "churn_probe"\nversion = "0.0.0"\nedition = "2021"\nbuild = "build.rs"\n\n[workspace]\n',
 ".cargo/config.toml":'[build]\ntarget-dir = "t"\nbuild-dir = "b"\n\n[unstable]\nfine-grain-locking = true\nbuild-dir-new-layout = true\nchecksum-freshness = true\n',
 "src/main.rs":'fn main() { print!("{}", include_str!(concat!(env!("OUT_DIR"), "/probe.txt"))); }\n',
 "build.rs":`use std::{env, fs, io::Write, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};
fn note(log: &str, what: &str, variant: &str) {
    let mut file = fs::OpenOptions::new().create(true).append(true).open(log).unwrap();
    writeln!(file, "{} {} {} {}", what, std::process::id(), variant, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()).unwrap();
}
fn main() {
    println!("cargo:rerun-if-env-changed=CHURN_VARIANT");
    let log = env::var("CHURN_LOG").unwrap();
    let variant = env::var("CHURN_VARIANT").unwrap_or_default();
    let target = format!("{}/probe.txt", env::var("OUT_DIR").unwrap());
    note(&log, "start", &variant);
    fs::write(&target, "").unwrap();
    for part in 0..6 {
        let mut file = fs::OpenOptions::new().append(true).open(&target).unwrap();
        write!(file, "{}:{} ", variant, part).unwrap();
        sleep(Duration::from_millis(400));
    }
    note(&log, "end", &variant);
}
`};

test.skipIf(!cargo||!existsSync(cargo)||!python)("concurrent pinned Cargo runs on one shared build directory never overlap in one unit, as judged by an independent interval oracle",async()=>{
 const work=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??realpathSync(process.env.TMPDIR??"/tmp"),"ue-")));
 for(const [name,text] of Object.entries(files)){mkdirSync(dirname(join(work,name)),{recursive:true});writeFileSync(join(work,name),text);}
 const log=join(work,"log.txt");writeFileSync(log,"");
 const variants=["a","b","a","b"],runs=variants.map((variant,index)=>new Promise<number>(accept=>{setTimeout(()=>{const child=spawn(cargo,["-Zunstable-options","-C",work,"build"],{env:{...process.env,PATH:join(dirname(cargo))+":"+(process.env.PATH??""),RUSTUP_TOOLCHAIN:"",CHURN_LOG:log,CHURN_VARIANT:variant},stdio:"ignore"});child.once("close",code=>accept(code??1));},index*700);}));
 expect(await Promise.all(runs)).toEqual([0,0,0,0]);
 const final=join(work,"final.txt");writeFileSync(final,Bun.spawnSync([join(work,"t/debug/churn_probe")]).stdout);
 const run=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),log,String(variants.length),final]),report=JSON.parse(run.stdout.toString().split("\n")[0]!);
 expect(run.exitCode,run.stdout.toString()).toBe(0);
 expect(report.overlaps).toBe(0);expect(report.unfinished).toBe(0);expect(report.runs).toBeGreaterThanOrEqual(2);
 expect(report.finalVariants.length).toBe(1);expect(report.finalParts).toBe(6);
 const overlapping=join(work,"overlap.txt");writeFileSync(overlapping,"start 1 a 100\nstart 2 b 150\nend 1 a 200\nend 2 b 250\n");
 expect(Bun.spawnSync([python,"-I",join(here,"🐍️.py"),overlapping,"2"]).exitCode).toBe(1);
 const own=readFileSync(log,"utf8").trim().split("\n");expect(own.length).toBe(report.runs*2);
},120000);
