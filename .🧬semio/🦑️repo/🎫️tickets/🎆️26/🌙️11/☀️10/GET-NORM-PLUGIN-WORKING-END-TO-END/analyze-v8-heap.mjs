import {readFileSync} from "node:fs";
const buf=readFileSync(process.argv[2]);
const a=buf.indexOf('"nodes":[')+9,b=buf.indexOf('"edges":[');
const nodeCount=Number(/"node_count":(\d+)/.exec(buf.toString("latin1",0,1200))[1]);
const T=new Uint8Array(nodeCount),N=new Int32Array(nodeCount),Sz=new Int32Array(nodeCount);
let i=a,n=0,field=0,num=0;
for(;i<b;i++){const c=buf[i];if(c===44){if(field===0)T[n]=num;else if(field===1)N[n]=num;else if(field===3)Sz[n]=num;field++;num=0;if(field===7){field=0;n++;}}else num=num*10+c-48;}

const s0=buf.indexOf('"strings":[')+10;const strings=JSON.parse(buf.toString("utf8",s0,buf.lastIndexOf("]")+1));
const types=["hidden","array","string","object","code","closure","regexp","number","native","synthetic","concatenated string","sliced string","symbol","bigint","object shape"];
const count=new Map(),bytes=new Map();let sb=0,sc=0;const ty=new Map();
for(let k=0;k<nodeCount;k++){const t=T[k];if(t===2||t===10||t===11){sc++;sb+=Sz[k];ty.set(types[t],(ty.get(types[t])??0)+1);count.set(N[k],(count.get(N[k])??0)+1);}}
console.log("nodes",nodeCount,"strings",sc,"string bytes MB",Math.round(sb/1e6),Object.fromEntries(ty),"distinct values",count.size);
console.log("TOP DUPLICATED");for(const[k,v]of[...count].sort((x,y)=>y[1]-x[1]).slice(0,30))console.log(v,strings[k].length,JSON.stringify(strings[k].slice(0,120)));
console.log("TOP BY TOTAL CHARS");for(const[k,v]of[...count].map(([k,v])=>[k,v*strings[k].length]).sort((x,y)=>y[1]-x[1]).slice(0,15))console.log(v,count.get(k),JSON.stringify(strings[k].slice(0,120)));
const oc=new Map();for(let k=0;k<nodeCount;k++)if(T[k]===3)oc.set(N[k],(oc.get(N[k])??0)+1);console.log("OBJECT CTORS");for(const[k,v]of[...oc].sort((x,y)=>y[1]-x[1]).slice(0,8))console.log(v,strings[k]);
