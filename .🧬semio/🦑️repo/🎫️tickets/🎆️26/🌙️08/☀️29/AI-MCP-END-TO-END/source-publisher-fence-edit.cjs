const fs=require('fs'),path='🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs',before=fs.readFileSync(path,'utf8');let after=before;
const edits=[
 ['publisher_placed:bool,origin:','publisher_placed:bool,publisher_fenced:bool,origin:'],
 ['publisher_placed:false,origin:None','publisher_placed:false,publisher_fenced:false,origin:None'],
 ['!budget.publisher_placed&&budget.held_items==0','!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0'],
 ['retirement_demand();let copy=copy+std::mem::size_of::<bool>();','retirement_demand();let copy=copy+2*std::mem::size_of::<bool>();'],
 ['budget.publisher_placed=false;return RetainedWorkOutcome','budget.publisher_placed=false;budget.publisher_fenced=false;return RetainedWorkOutcome'],
 ['if !budget.publisher_placed{return Err(refused("publisher placement absent").into())}','if !budget.publisher_placed||budget.publisher_fenced{return Err(refused("publisher placement fenced").into())}'],
 ['let source=budget.refusal();if source.is_none(){budget.failure=Some((error.kind.clone(),"publisher receipt refused"))}','let source=budget.refusal();budget.publisher_fenced=true;']
];for(const [old,value] of edits){const expected=old.startsWith('!budget.publisher_placed')?3:2;if(after.split(old).length!==expected)throw Error('unexpected publisher fence seam: '+old);after=after.split(old).join(value)}const tmp=path+'.guest-publisher-fence';fs.writeFileSync(tmp,after,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change')}fs.renameSync(tmp,path);
