/** 🎭 Stand-in for the build CLI that the bench tests run as a real process: it answers `--help`, `run <target>` and `service start|stop` with delays, marker lines, memory and failures taken from `BENCH_STUB_*` variables. */
import {appendFileSync,existsSync,readFileSync,rmSync,writeFileSync} from "node:fs";

const env=process.env,sleep=(milliseconds:number)=>new Promise<void>(accept=>setTimeout(accept,milliseconds)),number=(name:string,fallback:number)=>Number(env[name]??fallback);
const rest=process.argv.slice(3),tool=rest.find(argument=>argument.startsWith("--tool="))?.slice(7)??"neutral",words=rest.filter(argument=>!argument.startsWith("--tool=")&&!argument.startsWith("--service="));
if(env.BENCH_STUB_LOG)appendFileSync(env.BENCH_STUB_LOG,JSON.stringify(rest)+"\n");
if(words[0]==="--help"){await sleep(number("BENCH_STUB_HELP_MS",20));console.log("usage: repo <command>");process.exit(0);}
if(words[0]==="service"){
 if(words[1]==="start")writeFileSync(env.BENCH_STUB_SERVICE!,"1");else rmSync(env.BENCH_STUB_SERVICE!,{force:true});
 process.exit(0);
}
if(words[0]==="run"){
 const watch=env.BENCH_STUB_WATCH;
 if(watch&&existsSync(watch)&&readFileSync(watch,"utf8").includes(env.BENCH_STUB_FAIL_TOKEN??"@@fail@@")){console.error("error: build failed");process.exit(1);}
 const prelude=number("BENCH_STUB_PRELUDE_MS",0),start=number("BENCH_STUB_START_MS",0);
 if(prelude>0){await sleep(prelude);console.error("[script] loading");}
 await sleep(Math.max(0,start-prelude));
 console.error(tool==="nx"?`> nx run ${words[1]}`:tool==="bazel"?"INFO: Analyzing target":"[script] running");
 const keep=number("BENCH_STUB_ALLOC_MB",0)>0?Buffer.alloc(number("BENCH_STUB_ALLOC_MB",0)*1048576,1):undefined;
 await sleep(number("BENCH_STUB_RUN_MS",0));
 console.log(keep?`done ${keep.length}`:"done");
 process.exit(0);
}
process.exit(2);
