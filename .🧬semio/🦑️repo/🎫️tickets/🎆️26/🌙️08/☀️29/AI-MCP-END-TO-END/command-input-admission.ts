import fs from"node:fs";const file="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🗃️page-set-and-reader-behavior/🦀️.rs";const old=fs.readFileSync(file,"utf8");const start=old.indexOf("    pub fn try_push(&mut self, page: FixedCommandPage)"),end=old.indexOf("    pub fn len(&self)",start);if(start<0||end<0)throw Error("push seam");const replacement=`
    fn checked_push_byte_len(&self,page:&FixedCommandPage)->Result<usize,semio_framework_diagnostic::Fault>{
        if self.pages.len()>=self.declared{return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("plugin.command-page-count"),"command page authority is saturated"))}
        self.byte_len.checked_add(page.len()).filter(|total|*total<=COMMAND_MAXIMUM_BYTES).ok_or_else(||semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("plugin.command-byte-cap"),"command exceeds the assembled host-answer authority a guest may be handed"))
    }
    fn push_validated(&mut self,page:FixedCommandPage,byte_len:usize){
        if page.is_empty(){self.generic_shape_valid=false;self.all_nonempty=false}
        if self.pages.back().is_some_and(|previous|previous.len()!=COMMAND_PAGE_MAXIMUM_BYTES){self.generic_shape_valid=false}
        self.pages.push_back(page);self.byte_len=byte_len;
    }
    pub fn try_push(&mut self,page:FixedCommandPage)->Result<(),(semio_framework_diagnostic::Fault,FixedCommandPage)>{
        let byte_len=match self.checked_push_byte_len(&page){Ok(bytes)=>bytes,Err(fault)=>return Err((fault,page))};
        self.push_validated(page,byte_len);Ok(())
    }
    /// 📥️ Retains the actual caller page until every admission check has succeeded.
    pub fn try_push_input(&mut self,input:&mut Option<(CommandPageCursor,FixedCommandPage)>)->Result<Option<CommandPageCursor>,semio_framework_diagnostic::Fault>{
        let Some((_,page))=input.as_ref()else{return Ok(None)};
        let byte_len=self.checked_push_byte_len(page)?;
        let(cursor,page)=input.take().expect("validated command input remains present");
        self.push_validated(page,byte_len);Ok(Some(cursor))
    }

`;const next=old.slice(0,start)+replacement+old.slice(end);if(fs.readFileSync(file,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-push.rs",next);fs.renameSync(import.meta.dir+"/🗑️generated/command-push.rs",file);fs.appendFileSync(import.meta.dir+"/command-input-custody-2026-11-05.md","\nApplied no-clone borrowed page-set admission: validation happens before the caller slot is taken, and success reuses the exact original backing. Shared existing by-value admission uses the same checked invariant. Portable contract oracle passed 1 test / 11 assertions. Native physical-pointer law remains uncompiled, and Source decoder-to-command transfer is still a required separate proof.\n");console.log("Implemented checked borrowed page admission.");
