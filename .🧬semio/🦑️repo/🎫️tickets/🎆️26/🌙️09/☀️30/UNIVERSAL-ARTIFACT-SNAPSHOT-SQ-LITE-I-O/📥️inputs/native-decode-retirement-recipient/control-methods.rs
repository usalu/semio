    /// 🫴️ Installs one explicit caller recipient before any retained decoder ownership is created.
    pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{
        if self.retirement.is_some()||!recipient.terminal_is_empty(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native decode requires one empty explicit retirement recipient"));}
        self.retirement=Some(recipient);Ok(())
    }
    /// 🪑️ Pre-admits the return slot and exact immediate wrapper allocations, preserving every refusal owner.
    pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
        if self.retirement.as_ref().is_none_or(|recipient|recipient.reserved||recipient.owner.is_some()){return Err(E::from(ValueError::new(ValueRefusalKind::OwnershipLimit,"native decode has no available explicit retirement slot")));}
        self.charge(wrapper_bytes)?;self.checkpoint()?;
        self.retirement.as_mut().unwrap().reserved=true;
        let(result,owner)=operation(self);
        let recipient=self.retirement.as_mut().unwrap();recipient.owner=owner;recipient.reserved=false;
        result
    }

