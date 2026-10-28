"""Anchored edit: an output directory leaves the watched names only while no vouched fact reads an entry at or below it; law appended to the memo suite. Usage: edit.py <root> [check|apply]."""
import pathlib,sys
root=pathlib.Path(sys.argv[1]);mode=sys.argv[2] if len(sys.argv)>2 else "apply"
d=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
code=d/"📤️relevance/🟦️.ts";law=d/"🧪️tests/🟦️.ts"
C=[(" const directories=memoDirectoriesV1(vouched);\n"," const directories=memoDirectoriesV1(vouched),read=new Set([...confirmed].filter(path=>directories.has(path)));\n"),
   (" for(const path of confirmed)if(!vouched.has(path))directories.delete(path);\n"," for(const path of confirmed)if(!read.has(path))directories.delete(path);\n")]
appendix=(pathlib.Path(__file__).parent/"law.ts.txt").read_text(encoding="utf8")
text=code.read_text(encoding="utf8");lawtext=law.read_text(encoding="utf8")
done=all(n in text for _,n in C) and appendix.strip() in lawtext
if mode=="check":print("applied" if done else "base" if all(text.count(o)==1 for o,_ in C) else "drifted");sys.exit(0)
if done:print("already applied");sys.exit(0)
for o,n in C:
  if text.count(o)!=1:print("anchor lost:",o);sys.exit(1)
  text=text.replace(o,n)
for name in ["unpackMemoRelevanceV1","classifyMemoChangesV1","packMemoRelevanceV1","type MemoRowV1"]:
  if name not in lawtext.split("\n",20)[12]:print("law import missing:",name);sys.exit(1)
code.write_text(text,encoding="utf8");law.write_text(lawtext.rstrip("\n")+"\n"+appendix,encoding="utf8");print("applied")
