import{statSync,readdirSync,appendFileSync}from"node:fs";
const out=process.argv[2],tag=process.argv[3],root="/Users/ueli/Documents/semio";let last="";
const t=()=>{const r=[];for(const p of[root+"/.cache/b/.reset.json",root+"/.cache/b/n/.reset.json",root+"/.cache/b/n/m/.reset.json"]){try{statSync(p);r.push("ok")}catch(e){r.push(e.code)}}try{readdirSync(root);r.push("ls-ok")}catch(e){r.push("ls-"+e.code)}return r.join(",")};
const end=Date.now()+Number(process.argv[4]??900)*1000;
while(Date.now()<end){const now=t();if(now!==last){appendFileSync(out,tag+" "+new Date().toISOString()+" pid="+process.pid+" "+now+"\n");last=now;}await new Promise(r=>setTimeout(r,250));}
appendFileSync(out,tag+" end "+last+"\n");
