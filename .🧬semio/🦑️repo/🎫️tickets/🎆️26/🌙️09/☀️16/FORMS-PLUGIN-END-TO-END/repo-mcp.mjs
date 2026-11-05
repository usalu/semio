const child = Bun.spawn(["repo", "mcp"], {stdin:"pipe",stdout:"pipe",stderr:"inherit"});
const reader = child.stdout.getReader();
let buffer = "", sequence = 0;
async function request(method, params) {
 const id = ++sequence;
 child.stdin.write(JSON.stringify({jsonrpc:"2.0",id,method,params})+"\n");
 for (;;) {
  const end = buffer.indexOf("\n");
  if (end < 0) { const next = await reader.read(); if(next.done) throw Error("Repo MCP ended"); buffer += new TextDecoder().decode(next.value); continue; }
  const line = buffer.slice(0,end); buffer = buffer.slice(end+1);
  if (!line) continue;
  const response = JSON.parse(line);
  if (response.id === id) return response;
 }
}
try {
 await request("initialize",{protocolVersion:"2024-11-05",capabilities:{},clientInfo:{name:"codex",version:"1"}});
 child.stdin.write(JSON.stringify({jsonrpc:"2.0",method:"notifications/initialized"})+"\n");
 const method = process.argv[2], params = JSON.parse(process.argv[3] || "{}");
 console.log(JSON.stringify(await request(method,params)));
} finally { child.kill(); process.exit(0); }
