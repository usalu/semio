import { readFileSync, writeFileSync } from "node:fs";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/";
for(const name of ["📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs","🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs","🔌️plugin-app-impl/🦀️.rs","📄️docs/📥️staging/🎟️lease/🦀️.rs","📄️docs/📥️staging/🧱️components/🦀️.rs"]){const path=root+name;const source=readFileSync(path,"utf8");if(!source.includes("pending_media_import_permit"))throw new Error(`missing source in ${path}`);writeFileSync(path,source.replaceAll("pending_media_import_permit","pending_reserved_producer_permit"));}
