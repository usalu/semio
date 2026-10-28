import { fileURLToPath } from "node:url";
import fs from "node:fs";import {spawnSync} from "node:child_process";import path from "node:path";
const ticket=path.dirname(fileURLToPath(import.meta.url)),output=path.join(ticket,"🗑️generated");
const args=["tools","exec","--light","uv","--","run","--no-sync","--project","🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python","--python","3.12","--group","test","python",...process.argv.slice(2)];
const result=spawnSync("repo",args,{stdio:"inherit",shell:process.platform==="win32",env:{...process.env,PYTHONUTF8:"1",PYTHONIOENCODING:"utf-8",PYTHONDONTWRITEBYTECODE:"1",SEMIO_TEACHING_OUTPUT_ROOT:output,UV_PROJECT_ENVIRONMENT:path.resolve(".🧬semio/🎓️teaching/⚡️cache/python")}});process.exitCode=result.status??1;
