const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';const before=fs.readFileSync(file,'utf8');
const old='returned_source:owner.holds_job(job)||entry.has_original_job_source()';
if(before.split(old).length!==2)throw Error('job cancellation receipt changed');
const after=before.replace(old,'returned_source:owner.holds_job(job)||owner.job_prepared(job)&&entry.has_original_job_source()');
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/staged-job-cancel-identity.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared exports changed');fs.renameSync(temp,file);
