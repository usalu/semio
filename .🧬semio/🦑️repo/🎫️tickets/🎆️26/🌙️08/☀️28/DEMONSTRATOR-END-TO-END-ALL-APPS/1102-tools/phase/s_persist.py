from ph import *
c="semio-framework-value-persistent"
IV='ValueError::literal(ValueRefusalKind::InvariantViolated,"%s")'
def mk(m):return '.ok_or_else(||'+IV%m+')?'
SP_="🎮️splice/🦀️.rs"
E(c,SP_,"page=children[slot].as_ref().unwrap().get();height-=1;}Ok((Some(page),rank))","page=children[slot].as_ref()"+mk("immutable splice child is absent")+".get();height-=1;}Ok((Some(page),rank))")
E(c,SP_,'.expect("original splice ancestor rank").0','.ok_or_else(||ValueError::invariant("original splice ancestor rank"))?.0')
E(c,SP_,"Err(_)=>unreachable!()}}},","Err((error,_))=>Err(error)}}},")
E(c,SP_,'_=>unreachable!("quoted immutable splice phase")}','_=>return Err(ValueError::invariant("quoted immutable splice phase"))}')
E(c,"📥️build/🦀️.rs",'_=>unreachable!("quoted original persistent preparation phase")}','_=>return Err(refusal("quoted original persistent preparation phase").with_retained_progress(p))}')
PS="🔗️persistent/🦀️.rs"
E(c,PS,'original.get().root.as_ref().expect("admitted original rank root").get()','original.get().root.as_ref()'+mk("admitted original rank root is absent")+'.get()')
E(c,PS,"page=children[slot].as_ref().unwrap().get();height-=1;}Ok((page,rank))","page=children[slot].as_ref()"+mk("immutable original child is absent")+".get();height-=1;}Ok((page,rank))")
E(c,PS,'.expect("original changed child rank").0','.ok_or_else(||ValueError::invariant("original changed child rank"))?.0')
E(c,PS,'else{unreachable!("original immutable leaf plan")}','else{return Err(ValueError::invariant("original immutable leaf plan"))}')
E(c,PS,'else{unreachable!("original immutable branch plan")}','else{return Err(ValueError::invariant("original immutable branch plan"))}')
E(c,PS,'_=>unreachable!("quoted immutable path phase")}','_=>return Err(ValueError::invariant("quoted immutable path phase"))}')
if __name__=="__main__":build()
