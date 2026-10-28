// [mem3] Streaming V8 heap snapshot analyzer (not a library file): self sizes by (type,name), string families, dominator tree retained sizes with owner chains. usage: bun heapan.ts <file.heapsnapshot> [topN]. Peak memory ~ (nodes*28 + edges*16 + strings) bytes; aborts above 1.2 GB RSS.
import{openSync,readSync,closeSync,statSync}from"node:fs";
import{StringDecoder}from"node:string_decoder";
const path=process.argv[2]!,TOP=Number(process.argv[3]??"40"),fd=openSync(path,"r"),size=statSync(path).size,CH=1<<23,buf=Buffer.allocUnsafe(CH);
const head=(()=>{const n=readSync(fd,buf,0,1<<16,0);return buf.toString("latin1",0,n);})();
const nodeCount=Number(/"node_count":(\d+)/.exec(head)![1]),edgeCount=Number(/"edge_count":(\d+)/.exec(head)![1]);
const typesMatch=/"node_types":\[\[([^\]]*)\]/.exec(head)![1]!.replace(/"/g,"").split(",");
const NF=7,nodes=new Uint32Array(nodeCount*NF),edges=new Uint32Array(edgeCount*3);
let pos=0,state=0,target:Uint32Array|null=null,ti=0,num=0,innum=false;
const strings:string[]=[];let decoder=new StringDecoder("utf8"),sbuf="",sin=false;
function scanStringsChunk(text:string):void{
 sbuf+=text;let i=sin?1:0;
 while(true){
  if(!sin){const q=sbuf.indexOf('"',i);if(q<0){sbuf="";return;}sin=true;i=q+1;}
  let j=i;
  while(true){const q=sbuf.indexOf('"',j);if(q<0){sbuf=sbuf.slice(i-1);i=1;return;}let b=0,k=q-1;while(k>=i&&sbuf[k]==="\\"){b++;k--;}if(b%2===0){strings.push(JSON.parse(sbuf.slice(i-1,q+1)));sin=false;i=q+1;break;}j=q+1;}
 }
}
function memoryGuard():void{if(process.memoryUsage().rss>1.2e9)throw Error("[DEBUG] heapan RSS above 1.2 GB");}
let carry="";
for(;;){
 const n=readSync(fd,buf,0,CH,pos);if(n<=0)break;pos+=n;
 if(state===3){scanStringsChunk(decoder.write(buf.subarray(0,n)));continue;}
 let from=0;
 while(from<n){
  if(state===0||state===1||state===2){
   if(target===null){
    const text=carry+buf.toString("latin1",from,n),key=state===0?'"nodes":[':state===1?'"edges":[':'"strings":[',at=text.indexOf(key);
    if(at<0){carry=text.slice(-16);from=n;break;}
    const consumed=at+key.length-carry.length;carry="";from+=consumed;
    if(state===2){state=3;decoder=new StringDecoder("utf8");scanStringsChunk(decoder.write(buf.subarray(from,n)));from=n;break;}
    target=state===0?nodes:edges;ti=0;num=0;innum=false;
   }
   let i=from;
   for(;i<n;i++){const c=buf[i]!;if(c>=48&&c<=57){num=num*10+(c-48);innum=true;}else{if(innum){target![ti++]=num;num=0;innum=false;}if(c===93){target=null;state++;i++;break;}}}
   from=i;
  }else break;
 }
 memoryGuard();
}
closeSync(fd);
console.log(`nodes ${nodeCount} edges ${edgeCount} strings ${strings.length} rss ${Math.round(process.memoryUsage().rss/1048576)} MB`);
const mb=(n:number)=>(n/1048576).toFixed(1);
let total=0;const agg=new Map<string,[number,number]>();
for(let i=0;i<nodeCount;i++){const t=nodes[i*NF]!,name=nodes[i*NF+1]!,self=nodes[i*NF+3]!;total+=self;const tn=typesMatch[t]!,key=tn==="string"||tn==="concatenated string"||tn==="sliced string"?tn:tn+":"+strings[name];const row=agg.get(key)??[0,0];row[0]+=self;row[1]++;agg.set(key,row);}
console.log(`total self ${mb(total)} MB`);
console.log("== self size by type:name");
for(const[key,[s,c]]of[...agg].sort((a,b)=>b[1][0]-a[1][0]).slice(0,TOP))console.log(`${mb(s).padStart(8)} MB ${String(c).padStart(9)}  ${key.slice(0,100)}`);
const buckets=new Map<string,[number,number]>();
for(let i=0;i<nodeCount;i++){const tn=typesMatch[nodes[i*NF]!]!;if(tn!=="string"&&tn!=="concatenated string"&&tn!=="sliced string")continue;const s=strings[nodes[i*NF+1]!]!;const fam=s.length>600?"len>600":s.length>150?"len 150-600":s.length>40?"len 40-150":"len<=40";const sh=s.slice(0,18).replace(/[0-9a-f]{8,}/g,"H").replace(/\d+/g,"#");const key=fam+" | "+sh;const row=buckets.get(key)??[0,0];row[0]+=nodes[i*NF+3]!;row[1]++;buckets.set(key,row);}
console.log("== string families (len bucket | 18-char prefix shape), by self size");
for(const[key,[s,c]]of[...buckets].sort((a,b)=>b[1][0]-a[1][0]).slice(0,TOP))console.log(`${mb(s).padStart(8)} MB ${String(c).padStart(9)}  ${JSON.stringify(key.slice(0,80))}`);
const SR=nodeCount,N1=nodeCount+1,first=new Uint32Array(N1+1);for(let i=0;i<nodeCount;i++)first[i+1]=first[i]!+nodes[i*NF+4]!;first[N1]=first[nodeCount]!;
const indeg=new Uint32Array(N1);for(let e=0;e<edgeCount;e++)if(edges[e*3]!==6)indeg[edges[e*3+2]!/NF]++;
const kids=(v:number):Iterable<number>=>{const out:number[]=[];if(v===SR){out.push(0,...orphans);return out;}for(let e=first[v]!;e<first[v+1]!;e++){if(edges[e*3]===6)continue;out.push(edges[e*3+2]!/NF);}return out;};
const orphans:number[]=[];
{const z=new Map<string,[number,number]>();for(let v=0;v<nodeCount;v++)if(indeg[v]===0){const k=typesMatch[nodes[v*NF]!]!;const r=z.get(k)??[0,0];r[0]+=nodes[v*NF+3]!;r[1]++;z.set(k,r);}console.log("== zero in-degree nodes by type (edges to them were not recorded)");for(const[k,[sz,c]]of[...z].sort((x,y)=>y[1][0]-x[1][0]))console.log(`${mb(sz).padStart(8)} MB ${String(c).padStart(9)} ${k}`);}
{const visited=new Uint8Array(N1),stack=new Int32Array(N1);const mark=(r:number)=>{let sp=0;stack[sp++]=r;visited[r]=1;while(sp>0){const v=stack[--sp]!;for(let e=first[v]!;e<first[v+1]!;e++){if(edges[e*3]===6)continue;const w=edges[e*3+2]!/NF;if(!visited[w]){visited[w]=1;stack[sp++]=w;}}}};
 mark(0);let unreachable=0;for(let v=0;v<nodeCount;v++)if(!visited[v]&&indeg[v]===0){orphans.push(v);mark(v);}for(let v=0;v<nodeCount;v++)if(!visited[v]){orphans.push(v);mark(v);}
 for(let v=0;v<nodeCount;v++)if(!visited[v])unreachable++;console.log(`orphan roots ${orphans.length} (zero-indegree first), unreachable after ${unreachable}`);}
const order=new Int32Array(N1).fill(-1),postorder=new Int32Array(N1);let np=0;
{const visited=new Uint8Array(N1),stack=new Int32Array(N1+1),ptr=new Uint32Array(N1),oi=new Uint32Array(1);let sp=0;stack[sp++]=SR;visited[SR]=1;
 const nextChild=(v:number):number=>{if(v===SR){while(oi[0]!<=orphans.length){const k=oi[0]!++;const w=k===0?0:orphans[k-1]!;if(!visited[w])return w;}return-1;}while(ptr[v]!<first[v+1]!){const e=ptr[v]!++;if(edges[e*3]===6)continue;const w=edges[e*3+2]!/NF;if(!visited[w])return w;}return-1;};
 for(let v=0;v<N1;v++)ptr[v]=first[v]!;
 while(sp>0){const v=stack[sp-1]!;const w=nextChild(v);if(w>=0){visited[w]=1;stack[sp++]=w;}else{sp--;postorder[np]=v;order[v]=np;np++;}}}
console.log(`reachable from super root ${np-1} of ${nodeCount}`);
const predCount=new Uint32Array(N1+1);const eachEdge=(f:(v:number,w:number)=>void):void=>{for(let v=0;v<nodeCount;v++){if(order[v]<0)continue;for(let e=first[v]!;e<first[v+1]!;e++){if(edges[e*3]===6)continue;const w=edges[e*3+2]!/NF;if(w!==v&&order[w]!>=0)f(v,w);}}f(SR,0);for(const o of orphans)f(SR,o);};
eachEdge((_v,w)=>{predCount[w+1]++;});
for(let i=0;i<N1;i++)predCount[i+1]+=predCount[i]!;
const preds=new Uint32Array(predCount[N1]!),fill=predCount.slice(0,N1);
eachEdge((v,w)=>{preds[fill[w]!++]=v;});
const idom=new Int32Array(N1).fill(-1);idom[SR]=SR;
const intersect=(a:number,b:number):number=>{while(a!==b){while(order[a]!<order[b]!)a=idom[a]!;while(order[b]!<order[a]!)b=idom[b]!;}return a;};
for(let changed=true,round=0;changed;round++){changed=false;for(let k=np-2;k>=0;k--){const v=postorder[k]!;let nd=-1;for(let p=predCount[v]!;p<predCount[v+1]!;p++){const u=preds[p]!;if(idom[u]===-1)continue;nd=nd<0?u:intersect(u,nd);}if(nd!==idom[v]){idom[v]=nd;changed=true;}}if(round>60)break;}
const retained=new Float64Array(N1);for(let v=0;v<nodeCount;v++)if(order[v]!>=0)retained[v]=nodes[v*NF+3]!;
for(let k=0;k<np-1;k++){const v=postorder[k]!;retained[idom[v]!]+=retained[v]!;}
const label=(v:number):string=>v===SR?"SUPER-ROOT":`${typesMatch[nodes[v*NF]!]}:${String(strings[nodes[v*NF+1]!]).slice(0,60).replace(/\n/g," ")}`;
const edgeName=(p:number,v:number):string=>{if(p===SR)return"orphan";for(let e=first[p]!;e<first[p+1]!;e++)if(edges[e*3+2]!/NF===v){const t=edges[e*3]!;return t===1||t===4?"["+edges[e*3+1]+"]":String(strings[edges[e*3+1]!]).slice(0,40);}return"?";};
const ids=[...Array(nodeCount).keys()].filter(v=>order[v]!>=0).sort((a,b)=>retained[b]!-retained[a]!);
console.log("== top retained (dominator) nodes with owner chains (child <- parent via edge)");
let shown=0;const seenChain=new Set<number>();
for(const v of ids){if(shown>=TOP)break;const chain:string[]=[];let c=v;for(let d=0;d<9&&c!==SR;d++){const p=idom[c]!;chain.push(label(c)+" <-"+edgeName(p,c)+"-");c=p;}
 console.log(`${mb(retained[v]!).padStart(8)} MB  ${chain.join(" ")}`);shown++;}
{const fam=new Map<string,{bytes:number;count:number;sample:string}>();
 for(let v=0;v<nodeCount;v++){if(indeg[v]!==0)continue;const tn=typesMatch[nodes[v*NF]!]!;if(tn==="string"||tn==="concatenated string"||tn==="sliced string")continue;const names:string[]=[],sample:string[]=[];for(let e=first[v]!;e<first[v+1]!&&names.length<8;e++){const t=edges[e*3]!;if(t!==2)continue;const nm=String(strings[edges[e*3+1]!]);names.push(nm);const w=edges[e*3+2]!/NF,wt=typesMatch[nodes[w*NF]!]!;sample.push(nm+"="+(wt==="string"?JSON.stringify(String(strings[nodes[w*NF+1]!]).slice(0,40)):wt));}
  const key=tn+":"+String(strings[nodes[v*NF+1]!])+"{"+names.join(",")+"}";const r=fam.get(key)??{bytes:0,count:0,sample:sample.join(" ").slice(0,200)};r.bytes+=retained[v]!;r.count++;fam.set(key,r);}
 console.log("== zero in-degree object families (rows held by Map/Set/WeakMap values), dominator-retained bytes");
 for(const[k,r]of[...fam].sort((x,y)=>y[1].bytes-x[1].bytes).slice(0,TOP))console.log(`${mb(r.bytes).padStart(8)} MB ${String(r.count).padStart(8)}  ${k.slice(0,110)}\n            e.g. ${r.sample}`);}
{const per=new Map<number,[number,number]>();for(let v=0;v<nodeCount;v++){const tn=typesMatch[nodes[v*NF]!]!;if(tn!=="string")continue;const nm=nodes[v*NF+1]!;const r=per.get(nm)??[0,0];r[0]+=nodes[v*NF+3]!;r[1]++;per.set(nm,r);}
 let dupBytes=0,dupCount=0,distinct=per.size,all=0;const fam=new Map<string,[number,number]>();const top:[number,number,number][]=[];
 for(const[nm,[sz,c]]of per){all+=sz;if(c>1){const one=sz/c;dupBytes+=sz-one;dupCount+=c-1;const key=String(strings[nm]).slice(0,14).replace(/\d+/g,"#");const r=fam.get(key)??[0,0];r[0]+=sz-one;r[1]+=c-1;fam.set(key,r);top.push([sz-one,c,nm]);}}
 console.log(`== duplicate string cells: ${distinct} distinct contents, ${mb(all)} MB in string cells, duplicated bytes ${mb(dupBytes)} MB in ${dupCount} extra cells`);
 for(const[k,[b2,c]]of[...fam].sort((x,y)=>y[1][0]-x[1][0]).slice(0,12))console.log(`${mb(b2).padStart(8)} MB ${String(c).padStart(8)} extra cells  ${JSON.stringify(k)}`);
 top.sort((x,y)=>y[0]-x[0]);for(const[b2,c,nm]of top.slice(0,12))console.log(`${mb(b2).padStart(8)} MB x${c} ${JSON.stringify(String(strings[nm]).slice(0,90))}`);}
{const coll=new Set(["Map","Set","WeakMap","WeakSet"]),cand:number[]=[];for(let v=0;v<nodeCount;v++){const tn=typesMatch[nodes[v*NF]!]!;if(tn==="object"&&coll.has(String(strings[nodes[v*NF+1]!])))cand.push(v);}
 cand.sort((x,y)=>retained[y]!-retained[x]!);const top=cand.slice(0,TOP*2),topSet=new Map(top.map(v=>[v,[] as string[]]));
 for(let v=0;v<nodeCount;v++){for(let e=first[v]!;e<first[v+1]!;e++){const w=edges[e*3+2]!/NF,list=topSet.get(w);if(list&&list.length<3)list.push(label(v).slice(0,50)+"."+(edges[e*3]===1||edges[e*3]===4?"["+edges[e*3+1]+"]":String(strings[edges[e*3+1]!]).slice(0,40)));}}
 console.log("== largest Map/Set/WeakMap/WeakSet by retained bytes of their storage (contents are not graph edges) and their holders");
 for(const v of top)console.log(`${mb(retained[v]!).padStart(8)} MB  ${label(v).slice(0,20)}  held by ${topSet.get(v)!.join(" | ")||"(nothing recorded)"}`);}
{const skip=new Set(["code","object shape","closure","regexp","synthetic"]);const stamp=new Uint32Array(nodeCount);let gen=0;const stack=new Int32Array(nodeCount);
 const reach=(root:number):[number,number]=>{gen++;let sp=0,bytes=0,count=0;stack[sp++]=root;stamp[root]=gen;while(sp>0){const v=stack[--sp]!;bytes+=nodes[v*NF+3]!;count++;for(let e=first[v]!;e<first[v+1]!;e++){const t=edges[e*3]!;if(t===6)continue;const w=edges[e*3+2]!/NF;if(stamp[w]===gen)continue;const tn=typesMatch[nodes[w*NF]!]!;if(skip.has(tn)||String(strings[nodes[w*NF+1]!]).includes("Environment")&&tn==="object")continue;stamp[w]=gen;stack[sp++]=w;}}return[bytes,count];};
 const rows:[string,number,number,string][]=[];
 for(let v=0;v<nodeCount;v++){const nm=String(strings[nodes[v*NF+1]!]);if(!nm.includes("ModuleEnvironment")&&!nm.includes("LexicalEnvironment"))continue;for(let e=first[v]!;e<first[v+1]!;e++){const t=edges[e*3]!;if(t===6||t===1)continue;const w=edges[e*3+2]!/NF,tn=typesMatch[nodes[w*NF]!]!;if(skip.has(tn))continue;const wn=String(strings[nodes[w*NF+1]!]);if(wn.includes("Environment"))continue;const[b,c]=reach(w);if(b>200000)rows.push([String(strings[edges[e*3+1]!]).slice(0,50),b,c,(nm.includes("Module")?"M:":"L:")+tn+":"+wn.slice(0,24)]);}}
 rows.sort((x,y)=>y[1]-x[1]);console.log("== module/lexical environment variables by reachable bytes (not exclusive)");for(const[n,b,c,k]of rows.slice(0,TOP*2))console.log(`${mb(b).padStart(8)} MB ${String(c).padStart(9)} nodes  ${n}  [${k}]`);}
console.log(`rss end ${Math.round(process.memoryUsage().rss/1048576)} MB`);
