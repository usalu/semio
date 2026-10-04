        /// 📨️ Advances one owned child preparation before a caller can publish its wire prefix.
        pub fn prepare_child_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>{
            if maximum_items==0||maximum_bytes==0{return Ok(ChildEmitPreparationStep::Pending);}
            let Some(preparation)=self.child_preparations.front_mut()else{
                if self.child_preparations.capacity()!=0{
                    let bytes=self.child_preparations.capacity().checked_mul(std::mem::size_of::<ChildEmitPreparation>()).expect("allocated preparation queue has a representable layout");
                    if bytes>maximum_bytes{return Ok(ChildEmitPreparationStep::Pending);}
                    self.child_preparations=std::collections::VecDeque::new();
                    return Ok(ChildEmitPreparationStep::Pending);
                }
                return Ok(ChildEmitPreparationStep::Ready);
            };
            let output_needs_capacity=self.child_emits.len()==self.child_emits.capacity();
            if output_needs_capacity{
                let demand=self.child_emits.capacity().checked_add(self.child_emits.len().checked_add(1).ok_or_else(||Fault::from("child-emission-output-layout-overflow"))?).and_then(|cells|cells.checked_mul(std::mem::size_of::<ChildEmit>())).ok_or_else(||Fault::from("child-emission-output-layout-overflow"))?;
                if demand>maximum_bytes{return Ok(ChildEmitPreparationStep::Pending);}
                self.child_emits.try_reserve_exact(1).map_err(|_|Fault::from("child-emission-output-allocation-refused"))?;
            }
            if preparation.owner_cell_bytes()>maximum_bytes{return Ok(ChildEmitPreparationStep::Pending);}
            match preparation.step(maximum_items,maximum_bytes)?{
                ChildEmitPreparationStep::Ready=>{
                    let child=preparation.take_ready().ok_or_else(||Fault::from("child-emission-ready-without-complete-owned-prefix"))?;
                    if !preparation.terminal_is_empty(){return Err(Fault::from("child-emission-ready-retains-source-owners"));}
                    self.child_emits.push(child);self.child_preparations.pop_front();
                    Ok(ChildEmitPreparationStep::Pending)
                },
                step=>Ok(step),
            }
        }

        /// 📏️ Exact first pending source allocation demand, with its fixed preparation cell.
        pub fn next_child_preparation_byte_demand(&mut self)->usize{
            let backing=self.child_preparations.capacity().checked_mul(std::mem::size_of::<ChildEmitPreparation>()).expect("preparation queue layout");
            let output=if self.child_emits.len()==self.child_emits.capacity(){self.child_emits.capacity().saturating_add(self.child_emits.len().saturating_add(1)).saturating_mul(std::mem::size_of::<ChildEmit>())}else{0};
            match self.child_preparations.front_mut(){Some(preparation)=>preparation.next_close_byte_demand().max(preparation.owner_cell_bytes()).max(output),None=>backing}
        }

