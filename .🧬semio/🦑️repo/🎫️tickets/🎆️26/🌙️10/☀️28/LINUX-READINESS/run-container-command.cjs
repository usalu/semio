const fs=require("node:fs"),path=require("node:path"),cp=require("node:child_process");
const [name,...command]=process.argv.slice(2);
if(!name||!command.length)throw Error("Log name and owned container command required");
const ticket=path.relative(process.cwd(),__dirname).split(path.sep).join("/"),log=path.join(__dirname,"🗑️generated",name+".log"),fd=fs.openSync(log,"w"),args=["tools","exec","docker","--","exec","--env","SEMIO_LANGUAGE=en","--env","SEMIO_WORKSPACE_ROOT=/workspaces/semio","--env","SEMIO_TEST_ARTIFACT_DIR=/workspaces/semio/"+ticket+"/🗑️generated","--workdir","/workspaces/semio","semio-linux-readiness-source",...command],result=cp.spawnSync("/home/deck/.local/bin/repo",args,{env:{...process.env,SEMIO_TOOL_DOCKER:"/usr/bin/podman",SEMIO_LANGUAGE:"en"},stdio:["ignore",fd,fd]});
fs.closeSync(fd);
console.log("[DEBUG] "+JSON.stringify({status:result.status,error:result.error?.message,log}));
if(result.status)process.exitCode=result.status;
