p='📜️script.ts'
s=open(p,encoding='utf8',newline='').read()
old='''    const mode = a === "repo" ? "repo" : "default";
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    if (mode === "repo") {
      runCmd('''
new='''    if (a !== undefined && a !== "repo") {
      console.error(`[dev] unknown mcp route ${JSON.stringify(segments.join(" "))}: expected stdio, repo or nothing (the inspector).`);
      process.exit(1);
    }
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    if (a === "repo") {
      runCmd('''
assert s.count(old)==1
open(p,'w',encoding='utf8',newline='').write(s.replace(old,new)) if False else None
s=s.replace(old,new)
open(p,'w',encoding='utf8',newline='').write(s)
