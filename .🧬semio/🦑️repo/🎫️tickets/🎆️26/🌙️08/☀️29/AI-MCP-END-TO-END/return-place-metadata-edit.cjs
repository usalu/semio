const fs=require('fs');
const root='🧰️framework/🔨️modules/🧵️job/';
const changes=[
 ['📬️outcome/⏸️continuation/🎟️source/🦀️.rs','destination:usize,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{','destination:usize,copy_bytes:usize,metadata:usize)->Result<bool,OriginalTurnFailure>{'],
 ['📬️outcome/⏸️continuation/🎟️source/🦀️.rs','let row_copy=original_return_claim_copy_bytes();let current_copy=','let row_copy=original_return_claim_copy_bytes().checked_add(metadata).ok_or_else(||refused("return metadata overflow"))?;let current_copy='],
 ['📦️PLACE','claim_original_sized_return(cx,&mut self.credit,destination,copy_bytes)','claim_original_sized_return(cx,&mut self.credit,destination,copy_bytes,std::mem::size_of::<Option<OriginalSourceBinding>>()+std::mem::size_of::<Option<(usize,usize)>>())'],
 ['📬️outcome/⏸️continuation/🎟️origin/🦀️.rs','?.as_ref() as *const _ as usize,0)','?.as_ref() as *const _ as usize,0,0)']
];
changes[2][0]='⏱️context/📦️return/🎟️place/🦀️.rs';
const files=new Map();for(const [name,old,value] of changes){const path=root+name;const prior=files.get(path)??fs.readFileSync(path,'utf8');if(prior.split(old).length!==2)throw Error('unexpected match: '+path+' '+old);files.set(path,prior.replace(old,value));}
for(const [path,after] of files){const before=fs.readFileSync(path,'utf8');let expected=before;for(const [name,old,value] of changes)if(root+name===path)expected=expected.replace(old,value);if(expected!==after)throw Error('source changed: '+path);const tmp=path+'.guest-place-edit';fs.writeFileSync(tmp,after,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change: '+path)}fs.renameSync(tmp,path);}
