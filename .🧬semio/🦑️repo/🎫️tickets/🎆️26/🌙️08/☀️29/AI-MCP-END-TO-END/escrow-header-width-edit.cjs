const fs=require('fs'),path='🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs';
const before=fs.readFileSync(path,'utf8'),old='pub const ORIGINAL_RETURN_ESCROW_HEADER_BYTES:usize=16;',value='pub const ORIGINAL_RETURN_ESCROW_HEADER_BYTES:usize=48;';
if(before.split(old).length!==2)throw Error('unexpected escrow header');const tmp=path+'.guest-header-edit';fs.writeFileSync(tmp,before.replace(old,value),{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent change')}fs.renameSync(tmp,path);
