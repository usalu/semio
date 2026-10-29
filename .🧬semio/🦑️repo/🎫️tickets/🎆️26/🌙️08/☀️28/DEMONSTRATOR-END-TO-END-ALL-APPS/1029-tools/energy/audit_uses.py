import os,re,sys
root='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations'
fix=len(sys.argv)>1 and sys.argv[1]=='fix'
def pascal(n): return ''.join(p.capitalize() for p in n.split('_'))
for dp,dn,fn in os.walk(root):
    if '🦀️.rs' in fn and os.path.basename(dp)!='🦀️rust' and 'packages' not in dp:
        # only crate roots: those having a Cargo.toml in 📦️packages/🦀️rust sibling
        if not os.path.exists(os.path.join(dp,'📦️packages','🦀️rust','Cargo.toml')): continue
        p=os.path.join(dp,'🦀️.rs'); t=open(p).read()
        mods=re.findall(r'^pub mod (\w+) \{',t,re.M)
        missing=[]
        for m in mods:
            if not re.search(r'pub use %s::\{[^}]*\b%s\b'%(m,m),t): missing.append(m)
        if missing:
            print(os.path.relpath(dp,root),missing)
            if fix:
                add=''.join('pub use %s::{%s, %s};\n'%(m,m,pascal(m)) for m in missing)
                if not t.endswith('\n'): t+='\n'
                open(p,'w').write(t+add)
