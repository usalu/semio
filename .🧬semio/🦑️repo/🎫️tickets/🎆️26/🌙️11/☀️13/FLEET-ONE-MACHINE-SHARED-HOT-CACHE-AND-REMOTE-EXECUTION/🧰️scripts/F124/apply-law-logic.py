import os,json
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
here=os.path.dirname(os.path.abspath(__file__))
def write(path,s):
    tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
path=L+"🟦️.ts";s=open(path,encoding="utf8").read()
a=s.index("/** 🔎️ The calls of one source that open a path below a cache image");b=s.index("/** 📂️ Lists the scanned source files")
s=s[:a]+open(here+"/new-sites.ts.txt",encoding="utf8").read()+s[b:];write(path,s)
path=L+"🧪️tests/🐍️oracle/🐍️.py";s=open(path,encoding="utf8").read()
a=s.index("MODIFIERS=");b=s.index("def scan(")
s=s[:a]+open(here+"/new-oracle-core.py.txt",encoding="utf8").read()+s[b:]
old='PUNCT=re.compile(r">>>=|\\.\\.\\.|===|!==|\\*\\*=|<<=|>>=|>>>|&&=|\\|\\|=|\\?\\?=|=>|==|!=|<=|>=|&&|\\|\\||\\?\\?|\\?\\.|\\+\\+|--|[-+*/%&|^]=|\\*\\*|<<|>>|.",re.S)'
if old in s:s=s.replace(old,'PUNCT=re.compile(r"\\.\\.\\.|===|!==|\\*\\*=|<<=|&&=|\\|\\|=|\\?\\?=|=>|==|!=|<=|&&|\\|\\||\\?\\?|\\?\\.|\\+\\+|--|[-+*/%&|^]=|\\*\\*|<<|.",re.S)')
# keep function_ranges: it was between declarations and sites originally
if "def function_ranges(" not in s:raise SystemExit("function_ranges lost")
write(path,s);print("applied")
