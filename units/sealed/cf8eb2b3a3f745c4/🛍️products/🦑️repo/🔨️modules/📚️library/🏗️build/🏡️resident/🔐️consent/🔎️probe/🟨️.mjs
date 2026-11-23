/** 🔎️ The probe job of the launcher consent: `launcher <this file> <checkout>` lists the checkout and the places below it that the build reads through other mounts (`.cache`, `.cache/b`, the graph and tool images, `.cache/f`) with the launcher as responsible process (macOS shows the dialog "repo would like to access files in your Documents folder" now, once); only when every place that exists can be listed it leaves the marker that the launcher holds the consent. It ends by itself and never serves anything; exit status 3 names the place that was refused. */
import {readdirSync} from "node:fs";
import {join} from "node:path";
import {writeLauncherMarkerV1} from "../🟨️.mjs";

const root=process.argv[2];
if(typeof root!=="string"||root.length===0){process.stderr.write("usage: probe <checkout> / Aufruf: probe <Checkout>\n");process.exit(2);}
for(const place of ["",".cache",".cache/b",".cache/b/g",".cache/b/g/m",".cache/b/t",".cache/b/t/m",".cache/f"]){
 try{readdirSync(join(root,place));}
 catch(error){
  if(error?.code==="EPERM"||error?.code==="EACCES"){process.stderr.write("refused: "+join(root,place)+" ("+error.code+") / verweigert: "+join(root,place)+" ("+error.code+")\n");process.exit(3);}
 }
}
writeLauncherMarkerV1(root);
