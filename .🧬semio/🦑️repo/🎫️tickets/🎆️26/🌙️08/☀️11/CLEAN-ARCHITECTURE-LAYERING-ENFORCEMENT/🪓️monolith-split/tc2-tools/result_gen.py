"""tc2: renders F/RESULT-TEST-CLOSURE-T2.md from the census dumps and the verification logs (sections 3-6 are data-driven; sections 0-2 and 7-9 come from result_text.py)."""
import csv,os,sys,glob,json,re,collections
sys.path.insert(0,os.path.dirname(__file__))
F=os.path.abspath(os.path.dirname(__file__)+"/..")
G=F+"/🗑️generated"
import result_text as T
def rows(tag): return {r['crate']:r for r in csv.DictReader(open(f'{G}/tc2-{tag}/tc2-census-crates.tsv'),delimiter='\t')}
base=rows("baseline"); after=rows(sys.argv[1] if len(sys.argv)>1 else "after7")
short=lambda n:n.replace("semio-s-artifact-","")
def cell(r,l="native"): return f"{r[l+'_libTestFiles']} / {r[l+'_libTestBytes']} / {r[l+'_libTestLines']} ({r[l+'_libTestRatio']})"
out=[]
out.append(T.HEAD)
out.append("## 3. Census: test closure before and after (`bun tc2-tools/tc2-census.ts`, native; the two wasm labels are identical row by row)\n")
out.append("| crate | before (files / bytes / lines, ratio) | after |\n|---|---|---|")
for n,b in base.items(): out.append(f"| `{short(n)}` | {cell(b)} | {cell(after[n])} |")
out.append("\nCompanion crates (test closure, files / bytes / lines, ratio):\n")
out.append("| companion | closure | mounted test files |\n|---|---|---|")
specs={}
for f in sorted(glob.glob(os.path.dirname(__file__)+"/specs/*.json")):
    s=json.load(open(f)); specs[s["name"]]=s
for n,s in specs.items(): out.append(f"| `{short(n)}` | {cell(after[n])} | {len(s['files'])} (parent `{short(s['parent'])}`) |")
import result_data as D, subprocess
led=subprocess.run(["python3",os.path.dirname(__file__)+"/ledger.py"],capture_output=True,text=True).stdout.split("\n")
items=[l for l in led if l.startswith("   ")]
ledger="\n".join(["Total: "+str(len(items))+" items (file | declaration).",""]+["* `"+l.strip().replace("|","\\|")+"`" for l in items])
head=T.HEAD.replace("__VERDICT__",T.VERDICT)
body="\n".join([head]+out[1:])+"\n"+T.TAIL
body=body.replace("__ORACLE__",D.oracle()).replace("__CHECKAPPLIED__",D.checkapplied()).replace("__LEDGER__",ledger)
open(F+"/RESULT-TEST-CLOSURE-T2.md","w").write(body)
