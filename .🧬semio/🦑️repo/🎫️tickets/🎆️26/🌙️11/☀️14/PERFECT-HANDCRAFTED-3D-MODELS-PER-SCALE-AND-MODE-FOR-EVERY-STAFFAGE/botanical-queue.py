"""🌿 Run one native botanical verification at a time against recorded source and tool digests."""
from hashlib import sha256
from pathlib import Path
import json
import os
import shutil
import subprocess
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated"
plan = out / "botanical-queue.json"
records = out / "botanical-queue"
records.mkdir(exist_ok=True)
catalog = next(ticket.parents[6].glob("🎓*/*architecture/*staffage/*catalog/*vegetation"))
names = {"willow": "weeping-willow", "ivy": "ivy-on-trellis", "bamboo": "bamboo-clump", "apple": "apple-tree", "agave": "agave", "birch": "multi-stem-birch", "miscanthus": "miscanthus-grass"}
def digest(path):
    return sha256(path.read_bytes()).hexdigest()
while True:
    work = None
    for job in json.loads(plan.read_text()):
        subject, tag, program, args, cap = job
        code = ticket / program
        source = catalog / names[subject] / "🔣️.json"
        candidate = out / (subject + "-botanical-fit.json")
        inp = source if "{src}" in args else candidate
        if not inp.exists():continue
        if inp == candidate:
            authored = records/(subject+"-author.json")
            if not authored.exists():continue
            record = json.loads(authored.read_text())
            if record.get("rc") != 0 or record.get("candidateSha256") != digest(candidate):continue
            dependencies = json.loads(record["key"])[1]
            if any(dependencies.get(x.name) != digest(x) for x in ticket.glob(subject+"-botanical-*.ts")):continue
        installed = records / (subject + "-installed.json")
        if installed.exists() and json.loads(installed.read_text())["catalog"] == digest(source) and not tag.startswith("installed-"):continue
        if tag.startswith("installed-") and not installed.exists():continue
        if tag == "publish":
            required = ["document","50","100","200","500","scenes","grid" if subject == "ivy" else "anatomy","calm","reverse"]
            review = records/(subject+"-look.json")
            if not review.exists() or json.loads(review.read_text())["sourceSha256"] != digest(candidate):continue
            if subject in {"willow","bamboo","apple"}:required.append({"willow":"direct","bamboo":"close","apple":"windows"}[subject])
            certificates = [records/(subject+"-"+name+".json") for name in required]
            if not all(cert.exists() and json.loads(cert.read_text())["rc"] == 0 and json.loads(cert.read_text())["sourceSha256"]==digest(candidate) for cert in certificates):continue
        stamp = records / (subject + "-" + tag + ".json")
        inputs = {"document": digest(inp), "script": digest(code)}
        for x in ticket.glob(subject + "-botanical-*.ts"):
            if tag == "author" or "check" in program or "sheet" in program:inputs[x.name] = digest(x)
        key = json.dumps([job, inputs], ensure_ascii=False)
        if stamp.exists() and json.loads(stamp.read_text()).get("key") == key:continue
        work = job, code, source, candidate, stamp, key, inputs
        break
    if not work:break
    (subject, tag, program, args, cap), code, source, candidate, stamp, key, inputs = work
    log = out / (subject + "-botanical-" + tag + ".log")
    vals = {"{src}": str(source), "{dst}": str(candidate)}
    env = dict(os.environ, EQUIPMENT_RSS_MIB=str(cap), EQUIPMENT_HEADROOM_PERCENT="42")
    with log.open("w") as f:
        cmd = [sys.executable, ticket/"equipment-guard.py", shutil.which("bun"), "--smol", code, *[vals.get(x,x) for x in args]]
        if tag == "publish":
            cmd = [sys.executable,code,*args]
        rc = subprocess.run(cmd, env=env, stdout=f, stderr=f).returncode
        if rc == 75 and tag in {"document","50","100","200","500"}:
            rc = subprocess.run([sys.executable,ticket/"botanical-split.py",candidate,tag], env=env, stdout=f, stderr=f).returncode
    if tag == "scenes" and rc == 0:
        body = json.loads(log.read_text().splitlines()[-1])
        if body.get("scenes") != 12 or body.get("findings") != []:rc = 1
    if tag == "publish" and rc == 0:
        (records/(subject+"-installed.json")).write_text(json.dumps({"catalog":digest(source),"candidate":digest(candidate)}))
    record = {"key": key, "rc": rc, "sourceSha256": inputs["document"], "log": log.name}
    if tag == "author" and rc == 0:record["candidateSha256"] = digest(candidate)
    tmp = stamp.with_suffix(".tmp")
    tmp.write_text(json.dumps(record, ensure_ascii=False))
    tmp.replace(stamp)
    print(subject, tag, rc, flush=True)
