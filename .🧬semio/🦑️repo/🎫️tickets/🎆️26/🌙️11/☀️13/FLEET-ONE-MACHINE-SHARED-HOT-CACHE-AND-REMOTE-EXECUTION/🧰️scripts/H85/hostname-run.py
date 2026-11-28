import os,sys,time,shutil
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
topo=M+"/🤝️shared-worktree/🗺️topology/⌨️command/🟦️.ts";fs=M+"/📚️library/🏗️build/🚢️fleet-status/⌨️command/🟦️.ts"
a=open(topo,encoding="utf8").read();b=open(fs,encoding="utf8").read()
lines=a.split("\n")
last=max(i for i,l in enumerate(lines[:60]) if l.startswith("import"))
lines.insert(last+1,'/** 🖥️ Runs an interactive command with the terminal of this process (the operating system tools of `repo fleet hostname`, which may ask for a password); the status commands of the fleet never use it. */\nconst interactiveRunV1=(command:string,commandArguments:readonly string[]):Promise<number>=>new Promise(done=>{const child=spawn(command,[...commandArguments],{stdio:"inherit"});child.once("error",()=>done(1));child.once("close",code=>done(code??1));});')
a="\n".join(lines)
x='if(context.confirm===undefined||context.run===undefined)throw Error("hostname needs'
assert a.count(x)==1;a=a.replace(x,'if(context.confirm===undefined)throw Error("hostname needs')
y="run:context.run,...context.platform===undefined"
assert a.count(y)==1;a=a.replace(y,"run:context.run??interactiveRunV1,...context.platform===undefined")
i='import {spawnSync} from "node:child_process";\n';assert b.count(i)==1;b=b.replace(i,"")
r='  const run=own.run??(async(command:string,commandArguments:readonly string[]):Promise<number>=>spawnSync(command,[...commandArguments],{stdio:"inherit"}).status??1);\n'
assert b.count(r)==1;b=b.replace(r,"")
c="environment,confirm,run})";assert b.count(c)==1;b=b.replace(c,"environment,confirm,...own.run===undefined?{}:{run:own.run}})")
os.makedirs(T+"/landing.lock.topology") if not os.path.exists(T+"/landing.lock.topology") else sys.exit("busy topology")
try:
    os.makedirs(T+"/landing.lock.fleet-status")
except FileExistsError:
    shutil.rmtree(T+"/landing.lock.topology");sys.exit("busy fleet-status")
try:
    for p,t in((topo,a),(fs,b)):
        open(p+".h85.tmp","w",encoding="utf8").write(t);os.replace(p+".h85.tmp",p)
    print("landed",time.strftime("%H:%M:%S"))
finally:
    shutil.rmtree(T+"/landing.lock.topology",ignore_errors=True);shutil.rmtree(T+"/landing.lock.fleet-status",ignore_errors=True)
