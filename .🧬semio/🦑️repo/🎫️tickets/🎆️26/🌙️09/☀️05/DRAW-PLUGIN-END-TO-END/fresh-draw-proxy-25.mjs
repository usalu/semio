/** 🔀️ Serves a verified ticket-local Draw materialization through the existing Vite shell. */
import { createServer, request as httpRequest } from 'node:http';
import { connect } from 'node:net';
import { createReadStream, existsSync, statSync } from 'node:fs';
import { resolve, join, basename } from 'node:path';
const [artifactRoot, portArg = '16064', upstreamArg = 'http://127.0.0.1:6064'] = process.argv.slice(2);
if (!artifactRoot) throw Error('Expected materialized root');
const root=resolve(artifactRoot), upstream=new URL(upstreamArg), port=Number(portArg);
if (upstream.protocol !== 'http:' || !Number.isInteger(port) || port < 1 || port > 65535) throw Error('Expected HTTP upstream and valid port');
const prefix='/🔌️plugin-modules/🖍️draw/', vendor='/🔌️plugin-modules/vendor/';
const moduleRoot=join(root,'module'), vendorRoot=join(root,'vendor'), descriptor=join(moduleRoot,'🔣️.json');
for (const file of [join(moduleRoot,'semio_hub_draw_component.js'),descriptor]) if (!existsSync(file)) throw Error('Missing verified input '+file);
const server=createServer((req,res)=>{
  const url=new URL(req.url,'http://localhost'), path=decodeURIComponent(url.pathname);
  let file;
  if (path===prefix+'🔣️.json') file=descriptor;
  else if (path.startsWith(prefix)) {
    const name=path.slice(prefix.length);
    if (name===basename(name) && (name==='🟨️.js' || name==='🌉️bridge.js' || name==='semio_hub_draw_component.js' || /^semio_hub_draw_component\.core\d*\.wasm$/.test(name))) file=join(moduleRoot,name);
  } else if (path.startsWith(vendor)) {
    const relative=path.slice(vendor.length);
    if (relative.split('/').every(part=>part && part!=='.' && part!=='..')) file=join(vendorRoot,relative);
  }
  if (file) {
    if (!existsSync(file) || !statSync(file).isFile()) {res.writeHead(404);res.end();return;}
    const size=statSync(file).size;
    res.writeHead(200,{'content-type':file.endsWith('.wasm')?'application/wasm':file.endsWith('.json')?'application/json':'text/javascript','content-length':size,'cache-control':'no-store'});
    console.log('[DEBUG] fresh asset',req.method,path,size);
    if (req.method==='HEAD') res.end(); else createReadStream(file).pipe(res);
    return;
  }
  const proxy=httpRequest({hostname:upstream.hostname,port:upstream.port||80,path:req.url,method:req.method,headers:{...req.headers,host:upstream.host}},response=>{res.writeHead(response.statusCode,response.headers);response.pipe(res);});
  proxy.on('error',error=>{console.error('[DEBUG] proxy error',error.message);if(!res.headersSent)res.writeHead(502);res.end();});req.pipe(proxy);
});
server.on('upgrade',(req,socket,head)=>{
  const remote=connect(Number(upstream.port||80),upstream.hostname,()=>{
    const headers={...req.headers,host:upstream.host};
    remote.write(`${req.method} ${req.url} HTTP/${req.httpVersion}\r\n`+Object.entries(headers).map(([k,v])=>`${k}: ${Array.isArray(v)?v.join(', '):v}`).join('\r\n')+'\r\n\r\n');
    if(head.length)remote.write(head);socket.pipe(remote).pipe(socket);
  });
  remote.on('error',()=>socket.destroy());socket.on('error',()=>remote.destroy());
});
server.listen(port,'127.0.0.1',()=>console.log(`[DEBUG] Draw proxy listening http://127.0.0.1:${port} upstream ${upstream.href}`));
