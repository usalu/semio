import re,sys
E="crate::effect"
def balanced(t,start):
    d=0
    for i in range(start,len(t)):
        c=t[i]
        if c=="(":d+=1
        elif c==")":
            d-=1
            if d==0:return i
    raise Exception("unbalanced")
def calls(t,pat,fn,expect):
    out="";pos=0;n=0
    for m in re.finditer(pat,t):
        if m.start()<pos:continue
        op=m.end()-1;cl=balanced(t,op)
        out+=t[pos:m.start()]+fn(m,t[op+1:cl]);pos=cl+1;n+=1
    out+=t[pos:]
    assert expect is None or n==expect,(pat,n,expect)
    return out,n
def conv(path,rules):
    t=open(path).read()
    for pat,fn,expect in rules:
        t,n=calls(t,pat,fn,expect);print(path,pat,n)
    open(path,"w").write(t)
admit=(r"SealedShared::admit\(",lambda m,a:"SealedShared::admit(%s,&mut None)"%a)
dup=(r"(\w+)\.try_duplicate\(",lambda m,a:"%s::run(%s,|effect|%s.try_duplicate(effect))"%(E,a,m.group(1)))
step=(r"(\w+)\.step\(",lambda m,a:"%s::settle(%s,|effect|%s.step(effect)).result"%(E,a,m.group(1)))
which=sys.argv[1]
if which=="main":
    conv("🧪️tests/🦀️.rs",[(admit[0],admit[1],10),(dup[0],dup[1],None),(step[0],step[1],None)])
if which=="work":
    conv("🎟️work/🧪️tests/🦀️.rs",[(admit[0],admit[1],None),(dup[0],dup[1],None),(step[0],step[1],None)])
if which=="frontier":
    p="🚚️frontier/🧪️tests/🦀️.rs"
    runstep=(r"(\w+)\.step\(",lambda m,a:"%s::run(%s,|effect|%s.step(effect))"%(E,a,m.group(1)))
    closestep=(r"(?<=\|\|)(cursor)\.close_step\((denied|g)\)",lambda m,a:"%s::run(%s,|effect|cursor.close_step(effect))"%(E,a))
    conv(p,[(admit[0],admit[1],None),(dup[0],dup[1],None),(runstep[0],runstep[1],None),(closestep[0],closestep[1],2)])
    t=open(p).read()
    def rep(a,b,n=1):
        global t
        assert t.count(a)==n,(a,t.count(a))
        t=t.replace(a,b)
    rep("  fn retirement(self)->Box<dyn RetirementCursor>{Box::new(PayloadCursor{text:Some(self.text)})}\n","  type RetirementCursor=PayloadCursor;\n  fn retirement_seed(self)->PayloadCursor{PayloadCursor{text:Some(self.text)}}\n")
    rep("fn retirement(self)->Box<dyn RetirementCursor>{assert!(S&&K,\"unsupported native issuer must never consume its original\");Box::new(PayloadCursor{text:Some(self.0.text)})}","type RetirementCursor=PayloadCursor;fn retirement_seed(self)->PayloadCursor{assert!(S&&K,\"unsupported native issuer must never consume its original\");PayloadCursor{text:Some(self.0.text)}}")
    rep("  fn retirement(self)->Box<dyn RetirementCursor>{retirement::leaf(self.0)}\n  fn retirement_birth_bytes(&self)->Option<usize>{Some(retirement::leaf_birth_bytes::<u64>())}\n","  type RetirementCursor=<u64 as RetireOwned>::RetirementCursor;\n  fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}\n")
    rep("  fn shared_original_birth_bytes(&self)->Option<usize>{Some(1)}\n","  fn shared_original_demands(&self)->Result<Option<RetirementDemand>,ValueError>{Ok(Some(RetirementDemand{capacity_bytes:1,depth:1,..Default::default()}))}\n")
    rep("_original:&mut Option<SharedOriginal<Self>>,_grant:RetainedCloneGrant)","_original:&mut Option<SharedOriginal<Self>>,_effect:&mut RetainedAllocationEffect<'_>)")
    rep("let(mut cursor,heap)=observe(||alias.retirement());","let(mut cursor,heap)=seed(alias);")
    rep("let(result,heap)=observe(||(issuer.admit)(OriginalSlot::Native(&mut original),g));assert_eq!(result.work,u64::from(row[\"accepted\"].as_bool().unwrap()));","let(result,heap)=crate::effect::observed(g,|effect|(issuer.admit)(OriginalSlot::Native(&mut original),effect));assert_eq!(result.work,u128::from(row[\"accepted\"].as_bool().unwrap()));")
    rep("fn grant(d:RetirementDemand)","/// 🌱️ Admits one funded boxed cursor from a resident original and returns its heap custody.\nfn seed<T:RetireOwned>(value:T)->(Box<dyn RetirementCursor>,(usize,usize)){let mut slot=Some(value);let d=retirement::retirement_cursor_birth_demand(slot.as_ref().unwrap()).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(turn,heap)=crate::effect::observed(g,|effect|retirement::admit_retirement_cursor(&mut slot,effect));(turn.result.unwrap().unwrap().0,heap)}\nfn grant(d:RetirementDemand)")
    rep("RetirementStep::Failure(error,0)=>assert_eq!(error.kind,","RetirementStep::Failure(error,0)=>assert_eq!(error.into_value_error().kind,")
    rep("maximum_copy_bytes:size_of::<SealedShared<Payload>>(),maximum_depth:1,..Default::default()}","maximum_copy_bytes:SealedShared::<Payload>::duplicate_copy_bytes(),maximum_depth:1,..Default::default()}")
    rep("RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).result.unwrap().0;let pointer","RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:SealedShared::<T>::duplicate_copy_bytes(),maximum_depth:1,..Default::default()}).result.unwrap().0;let pointer") if False else None
    open(p,"w").write(t)
