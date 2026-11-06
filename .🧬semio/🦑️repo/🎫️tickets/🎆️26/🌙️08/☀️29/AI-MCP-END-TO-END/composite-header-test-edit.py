from pathlib import Path
import os
p=Path('🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🧪️tests/🦀️.rs')
s=p.read_text(encoding='utf8');original=s
s=s.replace('assert!(result.is_err());let actual=publisher.header_write_demand().unwrap();assert_eq!(actual.work,5);','assert!(!result);let outcome=publisher.header_outcome().unwrap();assert_eq!((outcome.0,outcome.1),(144,1));assert!(outcome.2.is_some());assert!(outcome.3.is_none()&&outcome.4.is_none());publisher.retain_header_post_failure(true,invalid()).unwrap();publisher.retain_header_post_failure(false,invalid()).unwrap();let outcome=publisher.header_outcome().unwrap();assert!(outcome.2.is_some()&&outcome.3.is_some()&&outcome.4.is_some());let actual=publisher.header_write_demand().unwrap();assert_eq!(actual.work,6);')
s=s.replace('Ok::<(),ValueError>(())}).unwrap().unwrap();','Ok::<(),ValueError>(())}).unwrap());')
s=s.replace('publisher.with_preparing_header(|_,header|{header.write(b"SAC3")','assert!(publisher.with_preparing_header(|_,header|{header.write(b"SAC3")')
s=s.replace('publisher.with_preparing_header(|capsule,header|{header.write(b"SAC3")','assert!(publisher.with_preparing_header(|capsule,header|{header.write(b"SAC3")')
assert s!=original and p.read_text(encoding='utf8')==original
tmp=p.with_name(p.name+'.terra.tmp');tmp.write_text(s,encoding='utf8',newline='')
assert p.read_text(encoding='utf8')==original;os.replace(tmp,p)
