#!/usr/bin/env python3
"""W4 patch (NOT applied, freeze): the shared Store genesis (all ~45 WindowConfigOwner births use it) advances its Pack cursor ONE unit per lifecycle turn (`cursor_step` once in phase 2 = measuring pass, once in phase 3 = byte-write pass). The cursor is a strictly per-unit state machine (its own tests run 1e5..1e7 `advance` calls), roughly 3 units per raw byte measuring plus ~4 per byte writing, so a multi-field record needs thousands of TURNS and one turn-capped fixture/host opportunity (32768 fuel/turns) is exhausted first: the latest-wins command that waits on `window_config_store.ensure_for_view` is re-queued every turn and never starts. Fix in the shared path: a turn spends as many funded cursor units as its context still allows (bounded page-sized batch, stops on yield, completion, measured length, or a unit that posts nothing). No limit is raised; every unit is still individually funded by `built(..)`."""
P="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/🌱️genesis/🦀️.rs"
s=open(P).read()
a2="}else{self.cursor_step(cx,false)?;}}"
a3="}else{self.cursor_step(cx,true)?;}}"
assert s.count(a2)==1 and s.count(a3)==1
s=s.replace(a2,"}else{self.cursor_batch(cx,false)?;}}").replace(a3,"}else{self.cursor_batch(cx,true)?;}}")
anchor=" fn cursor_step(&mut self,cx:&mut StepContext<'_>,output:bool)"
assert s.count(anchor)==1
helper=""" /// 📦️ Spends up to one page of individually funded cursor units in this turn; stops on yield, completion, the measured length, or a unit that posted nothing.
 fn cursor_batch(&mut self,cx:&mut StepContext<'_>,output:bool)->Result<(),OriginalTurnFailure>{
  for _ in 0..GENESIS_CURSOR_UNITS_PER_TURN{
   let before=cx.retained_progress();self.cursor_step(cx,output)?;
   let cursor=self.cursor.as_ref().ok_or_else(||invalid("Store genesis cursor is absent"))?;
   if cx.should_yield()||cursor.is_complete()||(!output&&cursor.output_len().is_some())||cx.retained_progress()==before{break}
  }
  Ok(())
 }
"""
s=s.replace(anchor,helper+anchor)
c="/// 🌱️ One mounted operation retains the exact original sources"
assert s.count(c)==1
s=s.replace(c,"/// 📏️ Funded Pack cursor units one genesis turn may spend (a page-sized batch, never more than the context's own grant and fuel allow).\nconst GENESIS_CURSOR_UNITS_PER_TURN:usize=512;\n\n"+c,1)
open(P,"w").write(s);print("applied")
