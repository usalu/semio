/** Proxy measurement of the boot-clock change (no Windows host here): the same shape on macOS, `sysctl -n kern.boottime` as a child process (what the shell route costs at least: a process start, a pipe, a parse) against one in-process foreign call (dlopen, buffers, call, close) of `sysctlbyname("kern.boottime")`. usage: bun boot-clock-bench.ts [rounds] */
import {spawnSync} from "node:child_process";
import {dlopen,ptr} from "bun:ffi";
const rounds=Number(process.argv[2]??30),median=(rows:number[])=>[...rows].sort((a,b)=>a-b)[Math.floor(rows.length/2)]!;
const spawnOnce=()=>{const started=performance.now();const out=spawnSync("/usr/sbin/sysctl",["-n","kern.boottime"],{encoding:"utf8"}).stdout;const ms=performance.now()-started;if(!/sec = \d+/.test(out))throw Error(out);return ms;};
const nativeOnce=()=>{const started=performance.now();const library=dlopen("libSystem.B.dylib",{sysctlbyname:{args:["cstring","ptr","ptr","ptr","u64"],returns:"i32"}});const record=Buffer.alloc(16),size=Buffer.alloc(8);size.writeBigUInt64LE(16n);const status=library.symbols.sysctlbyname(Buffer.from("kern.boottime\0"),ptr(record),ptr(size),null,0n);library.close();const ms=performance.now()-started;if(status!==0||record.readBigInt64LE(0)<=0n)throw Error("sysctlbyname "+status);return ms;};
spawnOnce();nativeOnce();
const spawned=Array.from({length:rounds},spawnOnce),native=Array.from({length:rounds},nativeOnce);
const rss=process.memoryUsage().rss/1048576;
console.log(JSON.stringify({rounds,spawnMedianMs:+median(spawned).toFixed(2),spawnMinMs:+Math.min(...spawned).toFixed(2),nativeMedianMs:+median(native).toFixed(3),nativeMaxMs:+Math.max(...native).toFixed(3),ratio:+(median(spawned)/median(native)).toFixed(0),rssMb:+rss.toFixed(0)}));
