import { nativeSystemFromProviderV1 } from "../../../../../../../../🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts";
const t0 = performance.now();
const s = nativeSystemFromProviderV1();
console.log("load ms", (performance.now() - t0).toFixed(1));
const lib = s.dlopen("/usr/lib/libSystem.B.dylib", { getpid: { args: [], returns: "i32" }, getppid: { args: [], returns: "i32" }, strlen: { args: ["cstring"], returns: "u64" } });
console.log(lib.symbols.getpid(), process.pid, lib.symbols.getppid(), process.ppid, lib.symbols.strlen("héllo"));
