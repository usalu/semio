import {readFileSync,existsSync} from "node:fs";
import {resolve,dirname} from "node:path";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing";
for(const file of [root+"/🧪️tests/🖱️selection-focus/🟦️.tsx",root+"/🧪️tests/🟦️.ts"]){if(!existsSync(file))continue;for(const match of readFileSync(file,"utf8").matchAll(/from\s+["']([^"']*paint2d-editing[^"']*)["']/g)){const target=resolve(dirname(file),match[1]!);console.log(JSON.stringify({file,target,exists:existsSync(target)}));}}
