from pathlib import Path
import os
p=Path('🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🦀️.rs');s=p.read_text(encoding='utf8');old=s
a='self.header_ok=result.is_ok();self.header_codec_failure=result.err();self.header_state=2;'
b='self.header_ok=result.is_ok();if let Err(error)=result{self.header_codec_failure=Some(error)}self.header_state=2;'
assert s.count(a)==1;s=s.replace(a,b);assert p.read_text(encoding='utf8')==old;t=p.with_name(p.name+'.terra.tmp');t.write_text(s,encoding='utf8',newline='');assert p.read_text(encoding='utf8')==old;os.replace(t,p)
