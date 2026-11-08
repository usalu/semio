
struct BindingSource{issuer:OriginalSourceTurnIssuer}
impl OriginalWorkRecipient for BindingSource{
 type Context<'turn,'source>=&'turn OriginalSourceTurn where 'source:'turn;
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(c.identity())}
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{self.issuer.validate(c).map_err(Into::into)}
 fn admits<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkQuote<'a>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{reply.complete(Ok(observed.checked_add(additional).is_some_and(|n|n<=1)))}
 fn settle_partial<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{reply.complete(Ok(()))}
 fn recover_partial<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{reply.pending()}
}
fn observe_effect_binding(authority:&mut OriginalWorkAuthority<BindingSource>,state:&mut OriginalWorkState,turn:&OriginalSourceTurn,expected:Option<OriginalWorkBindingIdentity>,matches:bool)->OriginalWorkBindingIdentity{
 let mut key=None;authority.with_loan(&mut &*turn,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork{state:state.take(),recipient:Some(&mut loan)};
 {let mut original=OriginalWorkBorrow::new(&mut work);let binding=original.binding_identity().unwrap();key=Some(binding);let(mut action,mut receipt,mut claim,mut marker)=(None,None,None,None);assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();let effect=scope.effect(grant(),1,&mut marker).unwrap();assert_eq!(expected.unwrap_or(binding).matches_effect(&effect),matches);}assert_eq!(claim.take().unwrap().unwrap(),0);original.cancel(&mut action).unwrap();assert_eq!(original.actual(),0);}*state=work.state.take();
 }).unwrap();key.unwrap()
}
#[test]
fn genuine_work_binding_matches_later_effects_but_rejects_new_scopes_and_issuers(){
 let issuer=OriginalSourceTurnIssuer::new();let first=issuer.issue().unwrap();let later=issuer.issue().unwrap();let mut authority=OriginalWorkAuthority::new(BindingSource{issuer});let mut state=OriginalWorkState::fresh();let key=observe_effect_binding(&mut authority,&mut state,&first,None,true);observe_effect_binding(&mut authority,&mut state,&first,Some(key),true);let mut other_scope=OriginalWorkState::fresh();observe_effect_binding(&mut authority,&mut other_scope,&first,Some(key),false);
 let source=authority.into_recipient().ok().unwrap();let mut foreign=OriginalWorkAuthority::new(source);observe_effect_binding(&mut foreign,&mut state,&first,Some(key),false);let current=observe_effect_binding(&mut foreign,&mut state,&first,None,true);observe_effect_binding(&mut foreign,&mut state,&later,Some(current),false);assert!(foreign.into_recipient().is_ok());
}
