import re,sys
# usage: ld_keyed.py <diff-file> <snapshot-file>: moves the `by <Delta>Keys` markers of the diff file into `Keyed` impls of the rows in the crate that defines them (`= keyed`)
d,n=sys.argv[1:3]
t=open(d).read()
blk=re.compile(r'/// 🔑️ Keys the foreign `(\w+)` rows of their list delta\.\n#\[derive\(Clone, Debug, PartialEq\)\]\npub struct (\w+);\n\nimpl protocol::list_delta::KeyOf<\w+> for \w+ \{\n    type Key = (.*?);\n    fn key_of\(row: &\w+\) -> .*? \{ (.*) \}\n\}\n\n?')
impls=[];marks={}
for m in blk.finditer(t):
    row,mk,kt,ex=m.groups();marks[mk]=1
    impls.append(f'/// 🔑️ The identity of a `{row}` inside its list delta.\nimpl semio_framework_os_kernel::list_delta::Keyed for {row} {{\n    type Key = {kt};\n    fn key(&self) -> {kt} {{\n        let row = self;\n        {ex}\n    }}\n}}\n')
t=blk.sub('',t)
for mk in marks: t=t.replace(f'= by {mk}','= keyed')
t=re.sub(r'\n{3,}','\n\n',t)
assert not re.search(r'by \w+Keys',t),d
s=open(n).read()
if not s.endswith('\n'): s+='\n'
open(n,'w').write(s+'\n'+'\n'.join(impls))
open(d,'w').write(t)
print(d.split('artifacts/')[1][:20],len(impls))
