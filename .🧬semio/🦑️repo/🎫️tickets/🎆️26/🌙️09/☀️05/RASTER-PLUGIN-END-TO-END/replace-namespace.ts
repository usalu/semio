import{readFileSync,writeFileSync}from"node:fs";
const path=process.argv[2]!,from=process.argv[3]!,to=process.argv[4]!;
const before=readFileSync(path,"utf8"),after=before.replaceAll(from,to);
if(after!==before)writeFileSync(path,after);
console.log(JSON.stringify({path,replacements:before.split(from).length-1,changed:after!==before}));
