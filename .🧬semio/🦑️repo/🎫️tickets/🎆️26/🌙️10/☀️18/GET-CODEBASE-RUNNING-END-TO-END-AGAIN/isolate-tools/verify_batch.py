"""✅ [isolate] verifies one staged batch at <root> (live or a mirror): (1) stage.py check reports every file applied, (2) gate-g scan.ts parse-checks the staged TS/MJS files, (3) each staged test file of expected.json runs (plain `bun test`, the official envelope runner when the file asks for the parent invocation; artifacts below the ticket, or below <root>/.cache/test-sandbox when the row says `short`) and fails no test outside its recorded set and never hits the live-store fence, (4) when <root> holds the law module, its scanner finds nothing in the staged files outside the law's exception rows. Prints `VERIFY OK <batch>` or `VERIFY FAIL <batch>`. usage: verify_batch.py <staged-dir> <root>"""
import json, os, re, subprocess, sys, tempfile
here, root = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]); batch = os.path.basename(here)
C = os.path.dirname(os.path.dirname(os.path.dirname(here))); G = os.path.join(C, "🗑️generated", "isolate", "verify-" + batch); os.makedirs(G, exist_ok=True)
manifest = json.load(open(os.path.join(here, "manifest.json"), encoding="utf-8")); expected = json.load(open(os.path.join(here, "expected.json"), encoding="utf-8")); failed = []
check = subprocess.run([sys.executable, "-I", os.path.join(here, "stage.py"), "check", "--root", root], capture_output=True, text=True)
if any(not line.startswith("applied") for line in check.stdout.splitlines() if line.strip()): failed.append("batch not applied at " + root)
listing = os.path.join(G, "scan-list.txt"); open(listing, "w", encoding="utf-8").write("\n".join(rel for rel in manifest if re.search(r"\.(ts|mjs)$", rel)) + "\n")
scan = subprocess.run(["bun", os.path.join(C, "gate-g-tools", "scan.ts"), listing, os.path.join(G, "scan")], cwd=root, capture_output=True, text=True)
print("parse", scan.stdout.strip().splitlines()[-1:] if scan.stdout.strip() else scan.stderr.strip()[-200:])
if scan.returncode != 0: failed.append("parse scan")
runner = os.path.join(G, "runner-" + str(abs(hash(root)) % 10**8)); os.makedirs(runner, exist_ok=True)
open(os.path.join(runner, "proc-run.ts"), "w", encoding="utf-8").write(open(os.path.join(C, "ver-f-tools", "proc-run.ts"), encoding="utf-8").read().replace("/Users/ueli/Documents/semio", root))
def run(rel, mode):
    log = os.path.join(G, re.sub(r"[^A-Za-z0-9]+", "_", rel.encode("ascii", "ignore").decode())[-60:] + "-" + mode + "-" + str(abs(hash(rel)) % 10**6) + ".log")
    parent = os.path.join(root, ".cache", "test-sandbox", "isv-" + batch) if expected[rel].get("artifacts") == "short" else G; os.makedirs(parent, exist_ok=True)
    art = tempfile.mkdtemp(prefix="a", dir=parent); env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=art)
    command = ["nice", "-n", "5", "bun", os.path.join(runner, "proc-run.ts"), "file", os.path.join(root, rel), "--timeout", "300000"] if mode == "env" else ["nice", "-n", "5", "bun", "test", "./" + rel, "--timeout", "300000"]
    with open(log, "w", encoding="utf-8") as out: subprocess.run(command, cwd=root, env=dict(env, VF_WALL="600"), stdout=out, stderr=subprocess.STDOUT, timeout=1200)
    return open(log, encoding="utf-8", errors="replace").read()
for rel, row in sorted(expected.items()):
    text = run(rel, "plain")
    if "Original parent Script process invocation required" in text: text = run(rel, "env")
    fails = set(l[len("(fail) "):].rsplit(" [", 1)[0] for l in text.splitlines() if l.startswith("(fail)")); extra = sorted(fails - set(row["fails"])); fenced = text.count("reached the live repository store")
    passed = sum(1 for l in text.splitlines() if l.startswith("(pass)")) or int((re.findall(r"^ *(\d+) pass$", text, re.M) or ["0"])[-1]); verdict = "OK  " if not extra and not fenced and passed >= row.get("pass", 0) else "FAIL"
    if verdict == "FAIL": failed.append(rel)
    print("%s %s fails %d (recorded %d)%s%s" % (verdict, rel.split("🔨️modules/")[-1][-80:], len(fails), len(row["fails"]), " new: " + "; ".join(extra)[:240] if extra else "", " fence %d" % fenced if fenced else ""))
law = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🟦️.ts")
if os.path.exists(law):
    probe = "const law=await import(%s);const fs=await import(\"node:fs\");let n=0;const excepted=new Set(law.TEST_STORE_RULES_V1.exceptions.map(row=>row.path));for(const rel of %s){if(!fs.existsSync(%s+\"/\"+rel)||excepted.has(rel))continue;for(const f of law.scanTestStoreReachesV1(fs.readFileSync(%s+\"/\"+rel,\"utf8\"),undefined,rel)){n++;console.log(rel,f.line,f.rule);}}console.log(\"findings\",n);" % (json.dumps(law), json.dumps(list(manifest)), json.dumps(root), json.dumps(root))
    found = subprocess.run(["bun", "-e", probe], cwd=root, capture_output=True, text=True); print("law", found.stdout.strip().splitlines()[-1:] if found.stdout.strip() else found.stderr.strip()[-300:])
    if "findings 0" not in found.stdout: failed.append("law findings")
else: print("law module absent at root: static check skipped (it lands with batch `fence`)")
print(("VERIFY OK " if not failed else "VERIFY FAIL ") + batch + ("" if not failed else ": " + ", ".join(f.split("/")[-3] if "/" in f else f for f in failed)))
sys.exit(1 if failed else 0)
