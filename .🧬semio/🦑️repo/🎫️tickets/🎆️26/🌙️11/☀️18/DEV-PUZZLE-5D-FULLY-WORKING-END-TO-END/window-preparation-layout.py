import re,sys,subprocess
p=sys.argv[1]
def lines(*args):
 with subprocess.Popen(args,stdout=subprocess.PIPE,text=True) as child:
  yield from child.stdout
  if child.wait(): raise SystemExit(child.returncode)
def symbol(line):return re.match(r"^([0-9a-f]+)\s+(?:([0-9a-f]+)\s+)?\w\s+(.+)",line)
roots=[]
for line in lines("nm","-nCS",p):
 if "drop_glue::<semio_os_store_snapshot_clone::RetainedClonePreparation<" not in line or "Puzzle5d" not in line:continue
 m=symbol(line)
 roots.append((int(m[1],16),int(m[2],16),m[3].split("::window::")[1].split(",")[0]))
for start,extent,label in roots:
 got={}
 current=0
 for line in lines("objdump","-d",f"--start-address={start}",f"--stop-address={start+extent}",p):
  if re.search(r"\tmov\s+\(%rsp\),%rdi",line):current=0
  m=re.search(r"add\s+\$0x([0-9a-f]+),%rdi",line)
  if m:current=int(m[1],16)
  m=re.search(r"\(%rip\),%rax\s+# ([0-9a-f]+)",line)
  if m:got[int(m[1],16)]=current
 targets={}
 for line in lines("readelf","-rW",p):
  m=re.match(r"([0-9a-f]+)\s+[0-9a-f]+\s+R_X86_64_RELATIVE\s+([0-9a-f]+)",line)
  if m and int(m[1],16) in got:targets[int(m[2],16)]=got[int(m[1],16)]
 rows=[]
 for line in lines("nm","-nCS",p):
  m=symbol(line)
  if not m or int(m[1],16) not in targets:continue
  name=m[3].removeprefix("core::ptr::drop_glue::<").replace("semio_s_artifact_puzzle_5d_tests::mirror::editor::puzzle5d::window::","")
  name=re.sub(r"\bsemio_\w+::","",name)
  rows.append((targets[int(m[1],16)],name[:180]))
 print(label)
 for start,name in sorted(rows):print(start,name)

