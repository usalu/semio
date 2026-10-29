const fs=require("fs"),cp=require("child_process"),path=require("path"),net=require("net");
const output=path.join(__dirname,"🗑️generated"),proof={budgetMilliseconds:20000};
const delay=ms=>new Promise(resolve=>setTimeout(resolve,ms));
const run=(command,args,environment,timeout=5000)=>new Promise(resolve=>{const child=cp.spawn(command,args,{env:{...process.env,...environment},stdio:["ignore","pipe","pipe"]});let stdout="",stderr="",timedOut=false;child.stdout.on("data",chunk=>stdout+=chunk);child.stderr.on("data",chunk=>stderr+=chunk);const timer=setTimeout(()=>{timedOut=true;child.kill("SIGTERM");},timeout);child.on("error",error=>{clearTimeout(timer);resolve({error:error.message});});child.on("close",(code,signal)=>{clearTimeout(timer);resolve({code,signal,timedOut,stdout,stderr});});});
(async()=>{
 let x,display,deadline,timedOut=false;const fd=fs.openSync(path.join(output,"headless-graphics-xvfb.log"),"w");
 try{
  const executable="/usr/bin/Xvfb";if(!fs.existsSync(executable))throw Error("Xvfb absent");
  x=cp.spawn(executable,["-displayfd","3","-screen","0","1280x720x24","-nolisten","tcp"],{stdio:["ignore",fd,fd,"pipe"]});
  deadline=setTimeout(()=>{timedOut=true;x.kill("SIGTERM");},20000);
  display=await new Promise((resolve,reject)=>{let text="";const timer=setTimeout(()=>reject(Error("Xvfb startup exceeded 5s")),5000);x.stdio[3].on("data",b=>{text+=b;if(text.includes("\n")){clearTimeout(timer);resolve(":"+text.trim());}});x.once("error",reject);x.once("exit",(code,signal)=>reject(Error("Xvfb exited before display "+code+" "+signal)));});
  proof.xvfb={pid:x.pid,display,argv:x.spawnargs};
  const socket="/tmp/.X11-unix/X"+display.slice(1);
  proof.x11=await new Promise((resolve,reject)=>{const client=net.createConnection(socket);const timer=setTimeout(()=>{client.destroy();reject(Error("X11 handshake timeout"));},3000);client.once("connect",()=>{const hello=Buffer.alloc(12);hello[0]=108;hello.writeUInt16LE(11,2);client.write(hello);});client.once("data",data=>{clearTimeout(timer);client.destroy();if(data[0]!==1)reject(Error("X11 setup rejected "+data[0]));else resolve({accepted:true,protocolMajor:data.readUInt16LE(2),socket});});client.once("error",reject);});
  const python=["/usr/bin/python3",...(process.env.PATH||"").split(":").map(dir=>path.join(dir,"python3"))].find(f=>fs.existsSync(f));
  proof.python=python??null;
  if(python)proof.egl=await run(python,[path.join(__dirname,"headless-egl.py")],{DISPLAY:display});
  const requireFrom=require("module").createRequire("/workspaces/semio/package.json");
  try{const config=JSON.parse(fs.readFileSync("/workspaces/semio/.devcontainer/devcontainer.json","utf8"));const configured=config.containerEnv.PLAYWRIGHT_BROWSERS_PATH.replace("${containerWorkspaceFolder}","/workspaces/semio");process.env.PLAYWRIGHT_BROWSERS_PATH=configured;proof.configuredBrowserRoot=configured;const playwright=requireFrom("playwright");const executablePath=playwright.chromium.executablePath();proof.chromium={executablePath,present:fs.existsSync(executablePath),version:requireFrom("playwright/package.json").version};}catch(error){proof.chromium={error:error.message};}
 }catch(error){proof.error=error.stack;process.exitCode=1;}
 finally{
  clearTimeout(deadline);
  if(x){const ended=new Promise(resolve=>x.once("close",(code,signal)=>resolve({code,signal})));x.kill("SIGTERM");let result=await Promise.race([ended,delay(3000).then(()=>null)]);if(!result){x.kill("SIGKILL");result=await ended;}proof.cleanup={...result,timedOut,alive:fs.existsSync("/proc/"+x.pid),socketStillExists:display?fs.existsSync("/tmp/.X11-unix/X"+display.slice(1)):null};}
  fs.closeSync(fd);fs.writeFileSync(path.join(output,"headless-graphics-proof.json"),JSON.stringify(proof,null,2)+"\n");console.log(JSON.stringify(proof,null,2));
 }
})();
