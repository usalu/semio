import os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
here=os.path.dirname(os.path.abspath(__file__))
path=L+"🧪️tests/🐍️oracle/🐍️.py"
s=open(path,encoding="utf8").read()
core=open(here+"/new-oracle-core.py.txt",encoding="utf8").read()
a=s.index("def declarations(");b=s.index("def function_ranges(")
s=s[:a]+s[b:]
a=s.index("def sites(");b=s.index("def scan(")
s=s[:a]+core+s[b:]
old='def named(source,policy):return any(re.search(p,source) for p in policy["mountPatterns"])'
assert s.count(old)==1
s=s.replace(old,'def named(source,policy):return any(re.search(p,source) for p in policy["mountPatterns"]+policy["rootPatterns"]+policy["rootTypes"])')
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
print("oracle ok")
