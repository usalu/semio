import{readFileSync,writeFileSync,renameSync}from'node:fs';import{dirname,join}from'node:path';
const file='🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs',old=readFileSync(file,'utf8');let s=old;
function rep(a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a);s=s.replace(a,b)}
rep('fn advance_body(&mut self,body:usize,grant:','fn advance_body(&mut self,body:usize,maximum_work_units:u64,grant:');
rep('cursor.demands(original,body)?','cursor.demands(original,body,maximum_work_units.checked_sub(2).unwrap_or(0))?');
rep('cursor.advance(original,body,child)','cursor.advance(original,body,maximum_work_units.checked_sub(2).unwrap_or(0),child)');
rep('fn phase_work_demand(&self,body:usize)','fn phase_work_demand(&self,body:usize,maximum_work_units:u64)');
rep('owner.advance_work_demand(source,body)?','owner.advance_work_demand(source,body,maximum_work_units.checked_sub(2).unwrap_or(0))?');
rep('let quote=self.phase_work_demand(body)?;','let maximum_work_units=receiving.available_work_units()?;if maximum_work_units==0{return Ok(ArtifactStoreOneItemPreparationStep::Blocked)}let quote=self.phase_work_demand(body,maximum_work_units)?;');
rep('self.advance_body(body,grant,None)','self.advance_body(body,maximum_work_units,grant,None)');
rep('self.advance_body(body,grant,Some(receiving))','self.advance_body(body,maximum_work_units,grant,Some(receiving))');
const tmp=join(dirname(import.meta.path),'🗑️generated/paged-projection-work-'+Date.now()+'.rs');writeFileSync(tmp,s);if(readFileSync(file,'utf8')!==old)throw Error('concurrent');renameSync(tmp,file);console.log('Paged projection uses original work view and reserves its two parent actions');
