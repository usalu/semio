import {spawnSync} from "node:child_process";
const sh=spawnSync("sh",["-c","command -v bun; bun --version; command -v repo; env | grep -E '^(BUN_RUNTIME_TRANSPILER_CACHE_PATH|SEMIO_LAUNCHER)' "],{encoding:"utf8"});
console.log(JSON.stringify({bun:Bun.version,exec:process.execPath,nested:sh.stdout.trim().split("\n")}));
