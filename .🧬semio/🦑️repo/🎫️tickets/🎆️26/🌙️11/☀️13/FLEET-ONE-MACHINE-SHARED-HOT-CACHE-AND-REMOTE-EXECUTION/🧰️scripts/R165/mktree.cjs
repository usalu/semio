const fs=require("node:fs"),path=require("node:path");
const root=process.argv[2],fan=+process.argv[3];
fs.mkdirSync(root,{recursive:true});
fs.writeFileSync(path.join(root,"package.json"),JSON.stringify({name:"r",semio:{workspace:{schemaVersion:1,members:["pkgs/**"],owners:["*/package.json"]}}}));
fs.mkdirSync(path.join(root,"pkgs"));let n=0;
for(let a=0;a<fan;a++)for(let b=0;b<fan;b++){const d=path.join(root,"pkgs","a"+a,"b"+b);fs.mkdirSync(d,{recursive:true});n+=2;for(let c=0;c<fan;c++){fs.mkdirSync(path.join(d,"c"+c));n++;}if((a+b)%5===0)fs.writeFileSync(path.join(d,"package.json"),JSON.stringify({name:"p"+a+"-"+b}));}
console.log("dirs",n);
