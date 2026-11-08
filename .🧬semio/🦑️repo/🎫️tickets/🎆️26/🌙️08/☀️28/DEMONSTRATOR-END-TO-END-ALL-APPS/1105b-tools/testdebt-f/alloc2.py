import sys
sys.path.insert(0,'.')
from rep import apply
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs"
I="assert!(authority.into_port().is_ok());"
apply(P,[
 ("drop(resumed);}).unwrap();}\n","drop(resumed);}).unwrap();"+I+"}\n",1),
 ("assert_eq!(source.as_ptr(),pointer);}).unwrap();}\n","assert_eq!(source.as_ptr(),pointer);}).unwrap();"+I+"}\n",1),
 ("assert_eq!(zero.owned_bytes(),0);}).unwrap();\n}","assert_eq!(zero.owned_bytes(),0);}).unwrap();"+I+"\n}",1),
 ("drop(receipt);}).unwrap();}\n let host","drop(receipt);}).unwrap();"+I+"}\n let host",1),
 ("assert_eq!(phase_progress.load(Ordering::Relaxed),3);}).unwrap();\n}","assert_eq!(phase_progress.load(Ordering::Relaxed),3);}).unwrap();"+I+"\n}",1),
 ("drop(receipt);}).unwrap();assert!(!recipient.has_owner());","drop(receipt);}).unwrap();"+I+"assert!(!recipient.has_owner());",1),
 ("drop(receipt);}).unwrap();assert!(recipient.has_owner());","drop(receipt);}).unwrap();"+I+"assert!(recipient.has_owner());",1),
])
print("ok")
