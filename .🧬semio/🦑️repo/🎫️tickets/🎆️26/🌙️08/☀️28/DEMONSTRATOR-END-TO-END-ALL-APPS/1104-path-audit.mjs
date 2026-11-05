import {execFileSync} from "node:child_process";
import {writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const tracked=execFileSync("repo",["tools","exec","--light","git","--","diff","--name-only","-z"],{encoding:"utf8",maxBuffer:16*1024*1024}).split("\0").filter(Boolean),untracked=execFileSync("repo",["tools","exec","--light","git","--","ls-files","--others","--exclude-standard","-z"],{encoding:"utf8",maxBuffer:16*1024*1024}).split("\0").filter(Boolean);
const relativeTicket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS";
const rows=[...new Set([...tracked,...untracked])].map(path=>({path,length:resolve(path).length})),invalid=rows.filter(row=>row.length>256),sourceInvalid=invalid.filter(row=>!row.path.startsWith(".🧬semio/🦑️repo/🎫️tickets/")),ownTicket=rows.filter(row=>row.path.startsWith(relativeTicket+"/")&&!row.path.includes("/🗑️generated/"));
function emit(){const result={version:1,checkedAt:new Date().toISOString(),count:rows.length,invalid,sourceInvalid,ownTicketMaximum:Math.max(0,...ownTicket.map(row=>row.length))};writeFileSync(join(import.meta.dirname,"🗑️generated/1104-path-audit.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify({count:rows.length,invalid:invalid.length,sourceInvalid,ownTicketMaximum:result.ownTicketMaximum},null,2));}
emit();
