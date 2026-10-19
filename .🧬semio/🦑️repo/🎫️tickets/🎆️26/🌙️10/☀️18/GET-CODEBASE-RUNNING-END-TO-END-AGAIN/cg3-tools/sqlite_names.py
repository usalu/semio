import re,sys,subprocess,os
B="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
D=B+"/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
files=[D+"/🦀️.rs",D+"/📏️cells/🦀️.rs",D+"/🧪️tests/🦀️.rs"]
def strip(t):
    t=re.sub(r'//[^\n]*','',t)
    t=re.sub(r'/\*.*?\*/','',t,flags=re.S)
    t=re.sub(r'r#*"(?:.|\n)*?"#*','""',t)
    t=re.sub(r'"(?:\\.|[^"\\])*"','""',t)
    t=re.sub(r"'(?:\\.|[^'\\])'","''",t)
    return t
KW=set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn".split())
PRELUDE=set("Some None Ok Err Option Result Vec String Box Default Clone Copy Send Sync Sized Drop Fn FnMut FnOnce From Into ToString ToOwned Iterator IntoIterator Extend PartialEq Eq PartialOrd Ord AsRef AsMut Debug Hash Cell u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char str".split())
allids={}
local=set()
for f in files:
    t=strip(open(f,encoding='utf8').read())
    for m in re.finditer(r'\b(?:struct|enum|trait|type|fn|const|static|mod|macro_rules!)\s+([A-Za-z_]\w*)',t): local.add(m.group(1))
    for m in re.finditer(r'\bimpl(?:<[^>]*>)?\s+',t): pass
    for m in re.finditer(r'<\s*([A-Z]\w*)\s*[:,>]',t): local.add(m.group(1))
    for m in re.finditer(r"\blet\s+(?:mut\s+)?([a-z_]\w*)",t): local.add(m.group(1))
    # use-bound names
    for m in re.finditer(r'\buse\s+([^;]+);',t):
        for n in re.findall(r'(?:as\s+)?([A-Za-z_]\w*)\s*(?=[,}]|$)',m.group(1)): local.add(n)
    for m in re.finditer(r'[A-Za-z_]\w*',t):
        s=m.start()
        prev=t[max(0,s-2):s]
        if prev.endswith('.') : continue
        if prev.endswith('::') : continue
        allids.setdefault(m.group(0),set()).add(os.path.basename(os.path.dirname(f)))
cands=sorted(n for n in allids if n not in KW and n not in PRELUDE and n not in local and (n[0].isupper() or True))
print(len(cands))
up=[n for n in cands if n[0].isupper() and not n.isupper()]
print("UPPER-CAMEL", len(up)); print(" ".join(up))
cs=[n for n in cands if n.isupper() and len(n)>1]
print("CONSTS", " ".join(cs))
