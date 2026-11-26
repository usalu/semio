import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
assert have()
s=open(TEST,encoding="utf8").read()
a='\\n\\]/u,\'members = ["\'+member+\'"]\'));put("🧰️framework/Cargo.lock"'
a2="""'members = ["'+member+'"]'));put("🧰️framework/Cargo.lock\""""
old="""replace(/members = \\[[\\s\\S]*?\\n\\]/u,'members = ["'+member+'"]'));"""
assert s.count(old)==1,s.count(old)
new="""replace(/members = \\[[\\s\\S]*?\\n\\]/u,'members = ["'+member+'"]')+'\\n[profile.prototype]\\ninherits="dev"\\ncodegen-units=256\\n[profile.release]\\ncodegen-units=1\\n');"""
s=s.replace(old,new)
write(TEST,s);print("ok")
