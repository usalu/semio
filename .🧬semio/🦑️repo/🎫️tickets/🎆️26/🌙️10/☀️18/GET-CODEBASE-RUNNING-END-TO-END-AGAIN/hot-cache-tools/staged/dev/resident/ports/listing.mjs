/** 📋 Membership listing in frontier shape: the native parallel scan when a watcher binary exists, a portable walk otherwise. */
import {spawn} from "node:child_process";
import {lstat,readdir} from "node:fs/promises";

/** 📋 Returns `{directories:[path,mtimeMs,ino][],members:[path,mode][]}` of the tree under `root` without excluded paths. */
export async function scanListingV1({root,binary,exclude,filter,threads=8}){
 if(binary){
  const args=["scan","--root",root,"--threads",String(threads)];
  for(const item of exclude.rooted)args.push("--exclude",item);
  for(const item of exclude.names)args.push("--exclude-name",item);
  const bytes=await new Promise((resolve,reject)=>{
   const child=spawn(binary,args,{stdio:["ignore","pipe","inherit"]}),chunks=[];
   child.stdout.on("data",chunk=>chunks.push(chunk));child.once("error",reject);
   child.once("exit",code=>code===0?resolve(Buffer.concat(chunks)):reject(new Error("scan exited with "+code)));
  });
  const frame=JSON.parse(bytes.toString("utf8"));
  return{directories:frame.directories,members:frame.members};
 }
 const directories=[],members=[];
 const walk=async rel=>{
  const absolute=rel===""?root:root+"/"+rel,status=await lstat(absolute);
  directories.push([rel,status.mtimeMs,status.ino]);
  const entries=await readdir(absolute,{withFileTypes:true}),subdirectories=[];
  await Promise.all(entries.map(async entry=>{
   const child=rel===""?entry.name:rel+"/"+entry.name;
   if(filter.excluded(child))return;
   if(entry.isDirectory())subdirectories.push(child);
   else if(entry.isFile()){const s=await lstat(root+"/"+child);members.push([child,s.mode]);}
  }));
  await Promise.all(subdirectories.map(walk));
 };
 await walk("");
 return{directories,members};
}
