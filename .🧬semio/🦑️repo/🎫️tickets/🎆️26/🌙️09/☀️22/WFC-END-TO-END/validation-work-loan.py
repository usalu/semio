from pathlib import Path

for path in [Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs'),Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs')]:
 s=path.read_text()
 marker=' struct Recipient'
 i=s.index(marker,s.index('#[cfg(test)]'))
 s=s[:i]+''' struct Work(u64);
 impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
'''+s[i:]
 if 'workspace-receiving' in str(path):
  old='   let turn=native.with_construction_recipient'
  new='   let mut work=Work(1000000);native.with_work_recipient(&mut work,|native|{let turn=native.with_construction_recipient'
  assert old in s
  s=s.replace(old,new,1)
  old='while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}\n  }'
  new='while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}Ok::<(),ValueError>(())}).unwrap();\n  }'
  assert old in s
  s=s.replace(old,new,1)
 else:
  old='  let turn=native.with_construction_recipient'
  new='  let mut work=Work(1000000);native.with_work_recipient(&mut work,|native|{let turn=native.with_construction_recipient'
  assert old in s
  s=s.replace(old,new,1)
  old='  while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}\n }'
  new='  while native.has_retirement_owner(){native.close_retirement_recipient(grant).result.unwrap();}Ok::<(),ValueError>(())}).unwrap();\n }'
  assert old in s
  s=s.replace(old,new,1)
 path.write_text(s)
