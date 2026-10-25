import{spawnSync}from"node:child_process";

const nested=spawnSync("sh",["-c","bun --version"],{encoding:"utf8",cwd:"/tmp"}).stdout.trim();
console.log(`entry=${process.versions.bun} nested(cwd=/tmp, sh -c 'bun --version')=${nested}`);
