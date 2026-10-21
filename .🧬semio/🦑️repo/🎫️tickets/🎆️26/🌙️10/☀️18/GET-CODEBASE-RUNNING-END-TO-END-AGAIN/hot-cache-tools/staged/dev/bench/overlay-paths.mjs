/** 🧭 Paths of the real library files inside an applied overlay, built from code points so no editor can lose a variation selector. */
import {join} from "node:path";

const name=(codePoint,text="")=>String.fromCodePoint(codePoint)+"️"+text;

/** 🧭 Resolves every module the benchmark imports from the overlay. */
export function overlayPathsV1(overlay){
 const library=join(overlay,name(0x1F9F0,"framework"),name(0x1F6CD,"products"),name(0x1F991,"repo"),name(0x1F528,"modules"),name(0x1F4DA,"library")),build=join(library,name(0x1F3D7,"build")),caller=join(build,name(0x1F4E5,"caller")),service=join(caller,name(0x1F6F0,"service"));
 const ts=name(0x1F7E6)+".ts",mjs=name(0x1F7E8)+".mjs";
 return{
  service:join(service,ts),session:join(service,name(0x1F511,"session"),mjs),caller:join(caller,ts),child:join(caller,name(0x1F4E4,"child"),ts),
  routing:join(overlay,name(0x1F9F0,"framework"),name(0x1F528,"modules"),name(0x1F3C3,"process"),name(0x1F4E6,"packages"),name(0x1F7E6,"typescript"),name(0x1F9ED,"routing"),ts),
 };
}
