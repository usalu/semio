#!/usr/bin/env python3
"""[DEBUG] only inside a patch window; delete the two [DEBUG] lines afterwards. Prints (a) every phase change of the Window partition birth (store-owners 🦀️.rs ensure_original_partition) and (b) every 100th measuring-pass step of the genesis Pack cursor (genesis 🦀️.rs phase 2): emitted bytes, is_complete, output_len."""
P1="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🦀️.rs"
s=open(P1).read()
a="        if let Some(owner)=self.initializing.as_mut(){\n            if owner.terminal_is_empty(){let d=WindowConfigPartitionBirth::<O>::boxed_release_demands();"
assert s.count(a)==1
b="        if let Some(owner)=self.initializing.as_mut(){\n            {static LAST:std::sync::atomic::AtomicU32=std::sync::atomic::AtomicU32::new(u32::MAX);static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let (phase,ready,closing)=owner.progress();let key=u32::from(phase)<<2|u32::from(ready)<<1|u32::from(closing);let n=N.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if LAST.swap(key,std::sync::atomic::Ordering::Relaxed)!=key||n%500==499{eprintln!(\"[DEBUG] window birth {} n={n} phase={phase} ready={ready} closing={closing} terminal={} fuel={}\",O::WINDOW_KIND_ID,owner.terminal_is_empty(),cx.fuel_remaining());}}\n            if owner.terminal_is_empty(){let d=WindowConfigPartitionBirth::<O>::boxed_release_demands();"
open(P1,"w").write(s.replace(a,b))
P2="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/🌱️genesis/🦀️.rs"
s=open(P2).read()
a="}else{self.cursor_step(cx,false)?;}}"
assert s.count(a)==1
b="}else{{static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);let n=N.fetch_add(1,std::sync::atomic::Ordering::Relaxed);if n%100==0{let c=self.cursor.as_ref().unwrap();eprintln!(\"[DEBUG] genesis measure n={n} emitted={} complete={} out_len={:?} max={}\",c.emitted_bytes(),c.is_complete(),c.output_len(),O::MAXIMUM_INITIAL_PACK_BYTES);}}self.cursor_step(cx,false)?;}}"
open(P2,"w").write(s.replace(a,b));print("applied")
