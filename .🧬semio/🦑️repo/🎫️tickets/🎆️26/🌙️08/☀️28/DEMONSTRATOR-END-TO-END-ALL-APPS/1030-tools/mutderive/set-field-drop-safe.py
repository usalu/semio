#!/usr/bin/env python3
import sys
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🎛️set-field/🦀️.rs"
s=open(P,encoding="utf-8").read()
old_t="fn terminal_is_empty(&self)->bool{self.closing&&self.authored.is_none()"
new_t="fn terminal_is_empty(&self)->bool{self.authored.is_none()"
a=s.index(" fn drop(&mut self){\n  let empty=self.closing&&")
b=s.index("\n}\n",a)
new_drop=''' fn drop(&mut self){
  let empty=self.authored.is_none()&&self.displaced.is_none()&&self.diff.is_none()&&self.inverse.is_none()&&self.inverse_close.is_none()&&self.payload.is_none()&&self.original.is_none()&&self.diff_source.is_none()&&self.retirement.is_empty();
  debug_assert!(empty||std::thread::panicking(),"retained set-field leaf dropped holding custody: close it through begin_close/close_step");
  if empty{unsafe{ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.inverse);}}
 }'''
assert old_t in s
s=s.replace(old_t,new_t,1)
a=s.index(" fn drop(&mut self){\n  let empty=self.closing&&")
b=s.index("\n}\n",a)
s=s[:a]+new_drop+s[b:]
if "--apply" in sys.argv: open(P,"w",encoding="utf-8").write(s); print("applied")
else: print("dry ok")
