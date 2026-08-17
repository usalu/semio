import { readdirSync, readFileSync, writeFileSync, statSync } from "fs";
import { join, relative, dirname } from "path";
const paths = readFileSync("/tmp/semio-w6-paths.txt","utf8").trim().split("\n");
const el = paths[1];
const home = join(el, "ContextMenu", readdirSync(join(el,"ContextMenu")).find(n=>n.endsWith("component.tsx")));
const syms = new Set(["ContextMenu","ContextMenuItem","ContextMenuController","ContextMenuProps","ContextMenuControllerProps","ContextMenuNavDirection","TextSelectionContextMenuHost","buildTextSelectionContextMenuItems","createDOMEventBinding","getElementById","queryElement","isContextMenuPointerTarget","findCheckedContextMenuItem"]);
function walk(d,o=[]){for(const n of readdirSync(d)){const p=join(d,n);const st=statSync(p);if(st.isDirectory())walk(p,o);else if(n.endsWith(".tsx"))o.push(p);}return o;}
let n=0;
for(const file of walk(el)){
  if(file===home) continue;
  let t=readFileSync(file,"utf8");
  if(!t.includes("W3-interim")||!t.includes("index.tsx")) continue;
  const single=t.match(/^import\s*\{([^}]+)\}\s*from\s*"([^"]+index\.tsx)";/m);
  // process all barrel imports
  const re=/import\s*\{([^}]+)\}\s*from\s*"([^"]+index\.tsx)";/g;
  let m; const replaces=[];
  while((m=re.exec(t))){
    const parts=m[1].split(",").map(s=>s.trim()).filter(Boolean);
    const stay=[], move=[];
    for(const part of parts){
      const mm=part.match(/^(type\s+)?(\w+)/); if(!mm){stay.push(part);continue;}
      if(syms.has(mm[2])) move.push(part); else stay.push(part);
    }
    if(!move.length) continue;
    let r=relative(dirname(file),home).replaceAll("\\","/"); if(!r.startsWith(".")) r="./"+r;
    const direct=`import { ${move.join(", ")} } from "${r}";`;
    let newImp="";
    if(stay.length) newImp=`import { ${stay.join(", ")} } from "${m[2]}";\n${direct}`;
    else {
      // drop interim comment on previous line if present
      newImp=direct;
    }
    replaces.push([m[0], newImp, !stay.length]);
  }
  if(!replaces.length) continue;
  for(const [old,neu,dropped] of replaces){
    if(dropped){
      t=t.replace(`// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.\n${old}`, neu);
      if(t.includes(old)) t=t.replace(old, neu);
    } else t=t.replace(old, neu);
  }
  writeFileSync(file,t); n++; console.log("rewired", file);
}
console.log("total", n);
