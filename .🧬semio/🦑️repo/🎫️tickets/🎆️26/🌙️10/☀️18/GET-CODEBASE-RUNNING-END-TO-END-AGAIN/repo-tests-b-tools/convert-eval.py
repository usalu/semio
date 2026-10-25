import os,re,sys,subprocess,shutil
root="/Users/ueli/Documents/semio"; os.chdir(root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
H="🧰️framework/🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts"
BK=C+"/🗑️generated/repo-tests-b/convert-backup"; os.makedirs(BK,exist_ok=True)
pat=re.compile(r'(?<![\w.])(?:new )?Function\(\.\.\.Object\.keys\((\w+)\),([^;]*?)\+";return (\w+);"\)\(\.\.\.Object\.values\(\1\)\)')
def run(f):
  r=subprocess.run(["nice","-n","10","repo",G,"--file="+f],capture_output=True,text=False,timeout=300,env=dict(os.environ,RUSTUP_TOOLCHAIN="nightly-2026-07-20"))
  t=(r.stdout+r.stderr).decode("utf8","replace")
  p=re.findall(r"^\s*(\d+) pass",t,re.M); fl=re.findall(r"^\s*(\d+) fail",t,re.M)
  return int(p[-1]) if p else -1, int(fl[-1]) if fl else -1
for f in [l.strip() for l in open(sys.argv[1]) if l.strip()]:
  s=open(f).read(); n=len(pat.findall(s))
  if not n: print("nopat",f[-50:]); continue
  before=run(f)
  new=pat.sub(lambda m:'evaluateExtractedFunctionV1(%s,"%s",%s)'%(m.group(2),m.group(3),m.group(1)),s)
  rel=os.path.relpath(H,os.path.dirname(f))
  new=new.replace("\n","\nimport {evaluateExtractedFunctionV1} from \"%s\";\n"%rel,1) if new.startswith("import") else 'import {evaluateExtractedFunctionV1} from "%s";\n'%rel+new
  shutil.copy(f,BK+"/"+str(abs(hash(f)))+".bak")
  open(f,"w").write(new)
  after=run(f)
  keep=after[1]>=0 and (after[1]<before[1] or (after[1]==before[1] and after[0]>=before[0] and before[1]==0))
  if after[1]<0 or after[1]>before[1] or (after[1]==before[1] and after[0]<before[0]): open(f,"w").write(s); verdict="REVERT"
  else: verdict="KEEP"
  print(verdict,n,before,"->",after,f[-70:],flush=True)
