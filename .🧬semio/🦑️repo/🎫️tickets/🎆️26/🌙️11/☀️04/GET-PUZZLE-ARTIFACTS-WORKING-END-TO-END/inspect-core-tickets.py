from pathlib import Path
import json,subprocess,threading,queue,time
ticket=Path(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END")
generated=ticket/"🗑️generated"
stderr=(generated/"core-ticket-mcp.stderr").open("w")
process=subprocess.Popen(["repo","mcp"],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=stderr,text=True,encoding="utf-8")
messages=queue.Queue()
def reader():
 for line in process.stdout:
  messages.put(line)
threading.Thread(target=reader,daemon=True).start()
def request(identity,method,params):
 process.stdin.write(json.dumps({"jsonrpc":"2.0","id":identity,"method":method,"params":params},ensure_ascii=False)+"\n")
 process.stdin.flush()
 end=time.monotonic()+30
 while True:
  line=messages.get(timeout=max(0.01,end-time.monotonic()))
  try:value=json.loads(line)
  except json.JSONDecodeError:continue
  if value.get("id")==identity:
   if "error" in value:raise RuntimeError(value["error"])
   return value["result"]
try:
 request(1,"initialize",{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"codex","version":"1.0"}})
 process.stdin.write(json.dumps({"jsonrpc":"2.0","method":"notifications/initialized","params":{}})+"\n");process.stdin.flush()
 result=request(2,"resources/read",{"uri":"repo://tickets"})
 (generated/"core-ticket-index.json").write_text(json.dumps(result,ensure_ascii=False))
 text="\n".join(row.get("text","") for row in result.get("contents",[]))
 selected=[line for line in text.splitlines() if any(word in line.lower() for word in ["original","interpreter","core","funded","wasm","bytecode","host"])]
 report="# Shared Runtime Ticket Discovery\n\nThe connected repo://tickets resource was empty; a read-only local repo mcp resource request used the actual workspace. No ticket, goal or process was mutated. Matching index lines are routing evidence, not proof that a runtime feature works.\n\n"+("\n".join(selected[:100]) if selected else "No matching entries in the actual resource index.")+"\n"
 (ticket/"shared-runtime-ticket-discovery.md").write_text(report)
 print("resource bytes",len(text.encode()),"matching lines",len(selected))
 print("\n".join(selected[:40]))
except Exception as error:
 (ticket/"shared-runtime-ticket-discovery.md").write_text("# Shared Runtime Ticket Discovery\n\nThe bounded local MCP request refused: "+str(error)+"\nNo ticket or goal was mutated. The owned MCP process closes in finally.\n")
 raise
finally:
 process.terminate()
 try:process.wait(timeout=5)
 except subprocess.TimeoutExpired:process.kill();process.wait()
 stderr.close()
