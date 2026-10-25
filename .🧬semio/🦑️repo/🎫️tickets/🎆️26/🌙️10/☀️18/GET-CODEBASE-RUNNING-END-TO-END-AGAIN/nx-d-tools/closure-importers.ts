const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/";
const a=await import(L+"🟦️.ts"),snap=await import(L+"📦️snapshot/🟦️.ts");
const result=await a.captureNxAuthorizerSourcesV1(root,process.argv[2]!,new AbortController().signal);
const snapshot=snap.snapshotNxAuthorizerSourcesV1(result);
const target=process.argv[3]!,out:string[]=[];
for(const row of snapshot.files){const text=Buffer.from(row.base64,"base64").toString("utf8");for(const line of text.split("\n"))if(line.includes(target)&&/(import|export|require|from)/u.test(line))out.push(row.path.slice(-110)+" :: "+line.trim().slice(0,220));}
console.log(out.join("\n"));
