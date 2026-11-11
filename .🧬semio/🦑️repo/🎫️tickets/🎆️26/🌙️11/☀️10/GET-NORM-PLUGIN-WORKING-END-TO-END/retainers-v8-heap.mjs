import {readFileSync} from "node:fs";
const buf=readFileSync(process.argv[2]);
const head=buf.toString("latin1",0,1500),nodeCount=Number(/"node_count":(\d+)/.exec(head)[1]),edgeCount=Number(/"edge_count":(\d+)/.exec(head)[1]);
const parseInts=(from,to,stride,want)=>{const out=want.map(()=>null);return out;};
const a=buf.indexOf('"nodes":[')+9,b=buf.indexOf('"edges":['),c=b+9,d=buf.indexOf('"strings":[');
const T=new Uint8Array(nodeCount),Nm=new Int32Array(nodeCount),Sz=new Int32Array(nodeCount),Ec=new Int32Array(nodeCount);
{let i=a,n=0,f=0,num=0;for(;i<b;i++){const ch=buf[i];if(ch===44||ch===93){if(f===0)T[n]=num;else if(f===1)Nm[n]=num;else if(f===3)Sz[n]=num;else if(f===4)Ec[n]=num;f++;num=0;if(f===7){f=0;n++;}}else if(ch>=48&&ch<=57)num=num*10+ch-48;}}
const ET=new Uint8Array(edgeCount),EN=new Int32Array(edgeCount),ETo=new Int32Array(edgeCount);
{let i=c,n=0,f=0,num=0;const end=d;for(;i<end&&n<edgeCount;i++){const ch=buf[i];if(ch===44||ch===93){if(f===0)ET[n]=num;else if(f===1)EN[n]=num;else ETo[n]=num/7|0;f++;num=0;if(f===3){f=0;n++;}}else if(ch>=48&&ch<=57)num=num*10+ch-48;}}
const strings=JSON.parse(buf.toString("utf8",buf.indexOf('"strings":[')+10,buf.lastIndexOf("]")+1));
const eStart=new Int32Array(nodeCount+1);for(let i=0;i<nodeCount;i++)eStart[i+1]=eStart[i]+Ec[i];
const par=new Int32Array(nodeCount).fill(-1),pe=new Int32Array(nodeCount).fill(-1),q=new Int32Array(nodeCount);let h=0,t=0;q[t++]=0;par[0]=0;
while(h<t){const u=q[h++];for(let e=eStart[u];e<eStart[u+1];e++){if(ET[e]===6)continue;const v=ETo[e];if(par[v]<0){par[v]=u;pe[v]=e;q[t++]=v;}}}
const types=["hidden","array","string","object","code","closure","regexp","number","native","synthetic","concatenated string","sliced string","symbol","bigint","object shape"];
const edgeLabel=e=>ET[e]===1||ET[e]===0?"["+EN[e]+"]":strings[EN[e]];
const nodeLabel=u=>types[T[u]]+":"+(T[u]===3||T[u]===5?strings[Nm[u]]:JSON.stringify(strings[Nm[u]].slice(0,40)));
const path=u=>{const o=[];for(let k=0;k<40&&u!==0&&par[u]>=0;k++){o.push(edgeLabel(pe[u])+" -> "+nodeLabel(u));u=par[u];}return o.reverse().join("\n   ");};
// subtree sizes on the BFS tree
const sub=new Float64Array(nodeCount),cnt=new Int32Array(nodeCount);for(let i=t-1;i>0;i--){const u=q[i];sub[u]+=Sz[u];cnt[u]++;const p=par[u];sub[p]+=sub[u];cnt[p]+=cnt[u];}

const target=process.argv[3],idx=strings.indexOf(target),all=[];for(let u=0;u<nodeCount;u++)if(Nm[u]===idx&&(T[u]===2||T[u]===10)&&par[u]>=0)all.push(u);
console.log("copies",all.length);const kinds=new Map();
for(const u of all){const p=pe[u],owner=par[u];const k=nodeLabel(owner)+" ."+edgeLabel(p);kinds.set(k,(kinds.get(k)??0)+1);}
console.log([...kinds].sort((a,b)=>b[1]-a[1]).slice(0,12).map(x=>x.join("  x")).join("\n"));
for(let j=0;j<all.length;j+=Math.floor(all.length/5)||1)console.log("--- sample\n   "+path(all[j]).split("\n").slice(-9).join("\n   "));
