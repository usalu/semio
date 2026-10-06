    /// 📦️ Transfers one actual u8 allocation without flattening, freeing or dropping nested typed payloads.
    pub fn return_bytes(&mut self,owner:&mut Vec<u8>,maximum_items:usize)->Result<bool,ValueError>{
        if maximum_items==0{return Ok(false);}let layout=Layout::array::<u8>(owner.capacity()).map_err(|_|refusal(ValueRefusalKind::OwnershipLimit,"returned byte allocation layout is invalid"))?;
        if layout.size()==0{owner.clear();return Ok(true);}let Some(slot)=self.admit(layout)?else{return Ok(false)};
        let mut allocation=ManuallyDrop::new(std::mem::take(owner));let pointer=NonNull::new(allocation.as_mut_ptr()).expect("allocated byte backing is nonnull");
        self.slots[slot]=Some(ReturnedAllocation{pointer,layout});self.retained_bytes+=layout.size();Ok(true)
    }
