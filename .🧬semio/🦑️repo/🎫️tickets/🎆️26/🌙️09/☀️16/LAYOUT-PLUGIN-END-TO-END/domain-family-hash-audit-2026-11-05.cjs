const fs=require('fs'),path=require('path'),crypto=require('crypto');
const t=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END";
const input=JSON.parse(fs.readFileSync(t+'/domain-family-shortening-input-2026-11-05.json','utf8'));
const changed=new Set(input.affectedFiles);
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const catalogPath='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json';
const catalog=JSON.parse(fs.readFileSync(catalogPath,'utf8'));
const updated=[];
for(const [scope,entry]of Object.entries(catalog.scopes))for(const [file,hash]of Object.entries(entry.hashes??{})){const p=path.posix.join(entry.path,file);if(changed.has(p)){const next=sha(p);entry.hashes[file]=next;updated.push({scope,file,path:p});}}
if(updated.length){fs.writeFileSync(catalogPath,JSON.stringify(catalog,null,2)+'\n');changed.add(catalogPath);}
const manifestAudit=[];
for(const m of input.mappings){const s=fs.readFileSync(m.manifest,'utf8');const name=s.match(/^name\s*=\s*"([^"]+)"/m)?.[1];if(name!==m.new)manifestAudit.push({manifest:m.manifest,expected:m.new,actual:name});}
const all=fs.readFileSync(t+'/🗑️generated/current-tracked-namespace-files-2026-11-05.bin','utf8').split('\0').filter(p=>p.endsWith('/Cargo.toml')&&!p.startsWith('.🧬semio/')&&fs.existsSync(p));
let declared=0;const buildScriptUpdates=[];
for(const p of all){let s=fs.readFileSync(p,'utf8');const next=s.replace(/(\[\[package\.metadata\.semio\.build-script\.files\]\]\s*\npath = "([^"]+)"\s*\nsha256 = ")([a-f0-9]+)(")/g,(match,before,source,hash,after)=>{declared++;if(!changed.has(source))return match;const next=sha(source);if(next!==hash)buildScriptUpdates.push({manifest:p,source});return before+next+after;});if(next!==s){fs.writeFileSync(p,next);changed.add(p);}}
fs.writeFileSync(t+'/domain-family-shortening-changed-files-2026-11-05.md','# Domain Namespace Changed Files\n\n'+[...changed].map(p=>'- '+p).join('\n')+'\n');
fs.writeFileSync(t+'/domain-family-shortening-audit-2026-11-05.md','# Domain Namespace Repair Audit\n\nCanonical OS excluding renderer, UI and plugin Rust families were shortened through explicit package mappings. No JavaScript project family names, source budgets or path limits changed.\n\n- Canonical packages: '+input.mappings.length+'; collisions checked against all current actual Cargo definitions before apply: zero.\n- Conservative visited-subset path-envelope samples authored before implementation: '+input.testPackages.length+'. They are not authoritative Cargo selected units.\n- Files changed including coupled hashes: '+changed.size+'.\n- Actual manifest mismatches after apply: '+manifestAudit.length+'.\n- Schema catalog hashes refreshed for changed listed schema source formats: '+updated.length+'.\n- Build-script declared source hashes inspected: '+declared+'; changed declared sources refreshed: '+buildScriptUpdates.length+'.\n- Historical path-envelope fixture remains unchanged.\n- Focused test and genuine product execution remain owned by the root agent; no duplicate run was launched here.\n\n## Coupled Hash Entries\n\n'+updated.map(x=>'- '+x.scope+': '+x.file).join('\n')+'\n');
console.log(JSON.stringify({canonicalPackages:input.mappings.length,manifestMismatches:manifestAudit,catalogHashes:updated.length,buildScriptDeclaredHashes:declared,buildScriptUpdates,files:changed.size}));
