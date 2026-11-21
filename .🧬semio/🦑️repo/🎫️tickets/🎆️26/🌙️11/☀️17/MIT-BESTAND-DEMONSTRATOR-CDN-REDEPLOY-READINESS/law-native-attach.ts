import { plugin } from "bun";
import { readFile } from "node:fs/promises";

plugin({name:"ticket-owned-law-image-attachment",setup(build){
  build.onLoad({filter:/🧪️testing[/]📥️invocation[/]🟦️[.]ts$/u},async({path})=>{
    const code=await readFile(path,"utf8");
    const launch='process.execPath,["test","--timeout"';
    if(!code.includes(launch))throw Error("Unknown isolated test invocation");
    return {contents:code.replace(launch,'process.execPath,["test","--preload",'+JSON.stringify(import.meta.path)+',"--timeout"'),loader:"ts"};
  });
  build.onLoad({filter:/🍎️image[/]👤️owners[/]🟦️[.]ts$/u}, async ({path})=>{
    const code=await readFile(path,"utf8");
    const program="nativeImageAttachProgramV1:string=protocol.attachProgram";
    const args="return Object.freeze([...protocol.attachArguments,protocol.attachMountPointFlag,root,image]);";
    if(!code.includes(program)||!code.includes(args))throw Error("Unknown native attach protocol");
    const replacement=`const dir=realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR!);
      const imagePath=realpathSync(image);
      if(!imagePath.startsWith(dir+"/i-")||!/^i-[a-zA-Z0-9]{6}\\/i\\/fixed\\.dmg$/.test(imagePath.slice(dir.length+1))||realpathSync(root)!==imagePath.slice(0,-9)+"m")throw Error("Expected exact private compiler law image: "+root+" "+imagePath);
      return ["attach","-nobrowse","-owners","on","-mountpoint",root,image];`;
    return {contents:code.replace(program,'nativeImageAttachProgramV1:string="/usr/bin/hdiutil"').replace(args,replacement),loader:"ts"};
  });
}});
