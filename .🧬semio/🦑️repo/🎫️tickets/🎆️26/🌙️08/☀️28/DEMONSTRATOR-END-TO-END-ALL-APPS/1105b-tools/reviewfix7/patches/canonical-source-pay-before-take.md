# reviewfix7 patch for store4 (file claimed by store4: canonical-edit source)

File: store/🧵️canonical-edit/🧳️source/🦀️.rs, `advance_close`.

Finding 5 (real): the authority lease is taken before the work is paid; a refused `pay` drops the strong alias unpaid.

old:
  else if let Some(original)=self.pending_authority.take(){pay(effect,1)?;self.authority_close=Some(SharedControlledRetirement::lease(original));}
new:
  else if self.pending_authority.is_some(){pay(effect,1)?;let original=self.pending_authority.take().ok_or_else(||refusal("canonical pending original authority is absent"))?;self.authority_close=Some(SharedControlledRetirement::lease(original));}

Finding 6 (real, reachability unproven): phases 2 and 3 panic with `unreachable!` after `pay`; map to a typed refusal.

old (phase 2):  .unwrap_or_else(|_|unreachable!("canonical paid semantic aliases"));
new:            .map_err(|_|refusal("canonical paid semantic aliases were refused"))?;
old (phase 3):  .unwrap_or_else(|_|unreachable!("canonical paid identity aliases"));
new:            .map_err(|_|refusal("canonical paid identity aliases were refused"))?;
(`let(owner,receipt)=Cursor::admit(..)` becomes `let(owner,receipt)=Cursor::admit(..).map_err(..)?;`; admit's error type must be droppable, it is discarded today.)
