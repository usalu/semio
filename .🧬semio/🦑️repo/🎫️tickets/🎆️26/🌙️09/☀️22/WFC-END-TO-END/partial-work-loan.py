from pathlib import Path

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🧪️tests/🪢️partial-projection/🦀️.rs')
s=p.read_text().replace('NativeEncodeConstructionRecipient,NativeEncodeRetirementRecipient','NativeEncodeConstructionRecipient,NativeEncodeRetirementRecipient,NativeEncodeWorkRecipient')
anchor='fn partial(frame:&ProjectFrame,total:usize)->bool{'
work='''struct Work{remaining:u64,committed:u64}
impl NativeEncodeWorkRecipient for Work{
 fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.remaining)}
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.remaining=self.remaining.checked_sub(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original projection work exhausted"))?;self.committed=self.committed.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original projection work overflow"))?;Ok(())}
}
'''
assert anchor in s
s=s.replace(anchor,work+anchor,1)
start=s.index('  let (turn,born,released)=')
end=s.index('\n }\n}',start)
old=s[start:end]
old=old.replace('  let (turn,born,released)=','   let (turn,born,released)=',1)
old=old.replace('assert!(turn.work>0);','assert!(turn.work>0);reported.set(turn.work);',1)
old=old.replace('assert_eq!(source,original);','assert_eq!(source,original);Ok::<(),ValueError>(())',1)
s=s[:start]+'''  let mut work=Work{remaining:1000000,committed:0};let reported=Cell::new(0);
  native.with_work_recipient(&mut work,|native|{
'''+old+'''
  }).unwrap();assert!(work.committed>=reported.get());
'''+s[end:]
p.write_text(s)
