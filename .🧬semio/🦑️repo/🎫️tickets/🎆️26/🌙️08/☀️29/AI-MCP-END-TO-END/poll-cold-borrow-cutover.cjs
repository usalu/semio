const fs=require('node:fs');const root='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/';
const changes=[
 ['🔄️turn/⏱️turn-execution-and-poll-entry/🦀️.rs','cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>','cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>',2],
 ['🔄️turn/🌀️poll-kernel-turn/🦀️.rs','cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>','cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>',1],
 ['🔄️turn/🌀️poll-kernel-turn/🦀️.rs','drive_cold_pair_ingress(runtime, cold_pair_page.as_ref(), identity, cx)','drive_cold_pair_ingress(runtime, cold_pair_page, identity, cx)',1],
 ['🧬️wit-bridge/🪢️bridge/🦀️.rs','staged.cold_pair.map(|page| *page)','staged.cold_pair.as_deref()',1],
 ['🎟️context/🌉️call/🎭️exports/🦀️.rs','input.command_page,input.cold_pair_page,input.budget,identity,cx)','input.command_page,input.cold_pair_page.as_ref(),input.budget,identity,cx)',1],
];
for(const file of new Set(changes.map(row=>row[0]))){const path=root+file,before=fs.readFileSync(path,'utf8');let after=before;
 for(const [,old,next,count] of changes.filter(row=>row[0]===file)){if(after.split(old).length-1!==count)throw Error(`unexpected marker count in ${path}`);after=after.replaceAll(old,next)}
 const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/cold-borrow-'+changes.findIndex(row=>row[0]===file)+'.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(path,'utf8')!==before)throw Error('shared source changed');fs.renameSync(temp,path);
}
