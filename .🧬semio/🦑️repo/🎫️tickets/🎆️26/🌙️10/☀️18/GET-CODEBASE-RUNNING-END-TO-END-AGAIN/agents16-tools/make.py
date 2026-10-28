"""🧱️ agents16: builds a flat staged batch `staged/<batch>/` (manifest {rel:{file,base,new}}, files/, base/, stage.py from the xplat5 mode-keeping template, verify.sh) from the LIVE tree and a mirror that holds the batch.
Sources are rebuilt from live through the batch's anchored edit script (so `base` is live as it is now; `sources.sh` puts the same stack into the mirror); test files and oracle outputs are taken from the mirror (new files have base null).
A stacked batch (`after`) takes as base the live files with the edits of the batches below it applied; every batch takes the files of the foreign batches that land before it (`OVERS`) as they are once applied.
Usage: python3 -I make.py allrun|farmfree|behind <mirror-root>"""
import hashlib, json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
HC = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
FARM = LIB + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
BATCHES = {
 "allrun": {"edit": "edit_allrun.py", "sources": [HC + "📏️decision/🟦️.ts", HC + "🧬️schema/🔣️.json", HC + "🎫️gate/🟦️.ts", HC + "🔼️upgrade/🟦️.ts", HC + "📐️estimates/🟦️.ts", LIB + "🌡️host-capacity/🟦️.ts"],
            "tests": [HC + "🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py", HC + "🧪️tests/🔼️upgrade/🧫️fixtures/🔣️.json", HC + "🧪️tests/🔼️upgrade/👥️agents/🟦️.ts", HC + "🧪️tests/🔼️upgrade/👥️agents/🥒️.feature", HC + "🧪️tests/🌤️followup/🧫️fixtures/🔣️.json", HC + "🧪️tests/📐️estimates/🐍️oracle/🐍️.py", HC + "🧪️tests/📐️estimates/🧫️fixtures/🔣️.json", HC + "🧪️tests/📐️estimates/🟦️.ts", LIB + "🌡️host-capacity/🧪️tests/👥️agents/🟦️.ts"]},
 "farmfree": {"edit": "edit_farmfree.py", "sources": [FARM + "🟨️.mjs"], "tests": [FARM + "🧪️tests/🔓️release/🟦️.ts"]},
 "behind": {"edit": "edit_behind.py", "after": ["allrun"], "sources": [HC + "📏️decision/🟦️.ts", HC + "🧬️schema/🔣️.json", HC + "🎫️gate/🟦️.ts", HC + "🔼️upgrade/🟦️.ts", HC + "🌦️forecast/🟦️.ts", LIB + "🌡️host-capacity/🟦️.ts", HC + "🧪️tests/🟦️.ts"],
            "tests": [HC + "🧪️tests/🧾️blockers/🐍️oracle/🐍️.py", HC + "🧪️tests/🧾️blockers/🧫️fixtures/🔣️.json", HC + "🧪️tests/🧾️blockers/🟦️.ts", HC + "🧪️tests/🧾️blockers/🥒️.feature", LIB + "🌡️host-capacity/🧪️tests/🧾️label/🟦️.ts"]},
}
OVERS = {"allrun": ["td-leases-tools/staged/startup"], "behind": ["td-leases-tools/staged/startup"], "farmfree": ["audit7-tools/staged/contracts", "audit7-tools/staged/reclaim"]}
ABOUT = {
 "allrun": "three agents build side by side on a declared 16 GB / 8-core machine: an agent session with nothing running gets its first job past our booked cores, a growth short of cores runs with the jobs they leave and keeps its whole memory, the farm request is elastic (decided last, never a barrier, observes nothing of its own), a class sized by its own measured peaks takes no ramp slot, and a nested invocation that inherits a quiet grant works with the grown grant (not 1 job)",
 "farmfree": "a project farm pool whose workers are refused gives its host capacity grant back at once",
 "behind": "every waiting and growing progress line names what the waiter stands behind, in English and German (the runs of ours that hold the resource its reason names, or the requests served first, by the target they build, state, jobs and megabytes; the host itself otherwise); STACKED on `allrun`: apply after it, revert before it",
}


def sha(data):
    return None if data is None else hashlib.sha256(data).hexdigest()


def read(path):
    return open(path, "rb").read() if os.path.exists(path) else None


def landed(batch, rel):
    """🛬️ The bytes of a live file as they are once the batches that land before `batch` (`OVERS`, in order) are applied: a file such a batch stages takes its new bytes while live holds its base or new; a batch none of whose files reads `base` any more has landed, and live (with whatever came after it) is the base; any other state is drift and stops the build."""
    data = read(os.path.join(LIVE, rel))
    for over in OVERS.get(batch, []):
        rows = json.load(open(os.path.join(C, over, "manifest.json"), encoding="utf8"))
        if all(sha(read(os.path.join(LIVE, path))) != row["base"] for path, row in rows.items()):
            continue
        if rel in rows:
            if sha(data) not in (rows[rel]["base"], rows[rel]["new"]):
                sys.exit("DRIFT %s: live %s is neither base nor new of %s" % (batch, rel, over))
            data = read(os.path.join(C, over, "files", rows[rel]["file"]))
    return data


def main():
    batch, mirror = sys.argv[1], os.path.abspath(sys.argv[2])
    spec, final = BATCHES[batch], os.path.join(HERE, "staged", batch)
    out = final + ".building"
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    scratch = tempfile.mkdtemp(prefix="ag16-make-")
    try:
        for rel in set(spec["sources"]).union(*[BATCHES[below]["sources"] for below in spec.get("after", [])]):
            os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
            open(os.path.join(scratch, rel), "wb").write(landed(batch, rel))
        for below in spec.get("after", []):
            subprocess.run([sys.executable, "-I", os.path.join(HERE, BATCHES[below]["edit"]), scratch], check=True, stdout=subprocess.DEVNULL)
        stacked = {rel: read(os.path.join(scratch, rel)) for rel in spec["sources"]}
        subprocess.run([sys.executable, "-I", os.path.join(HERE, spec["edit"]), scratch], check=True, stdout=subprocess.DEVNULL)
        manifest, problems = {}, []
        for index, rel in enumerate(spec["sources"] + spec["tests"]):
            base = stacked[rel] if rel in stacked else landed(batch, rel)
            new = read(os.path.join(scratch, rel)) if rel in spec["sources"] else read(os.path.join(mirror, rel))
            name = "%02d%s" % (index, os.path.splitext(rel)[1])
            open(os.path.join(out, "files", name), "wb").write(new)
            if base is not None:
                open(os.path.join(out, "base", name), "wb").write(base)
            manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    head, rest = template.split('"""', 2)[0], template.split('"""', 2)[2]
    doc = '"""🧑‍🤝‍🧑 Staged batch `%s` (lane agents16): %s.\nusage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)\ncheck : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).\napply : every live file must hold its base hash (absent for new files) or already the new one, else exit 2 (drift); atomic replace that keeps the mode of the file it replaces (a new file gets 0666 minus the umask, never the 0600 of `mkstemp`).\nrevert: every file must hold the new hash (or already base), else exit 2; restores base bytes, removes new files and the directories they emptied.\nrebase after a peer edit of a source file: python3 -I ../../make.py %s <mirror with the batch>"""' % (batch, ABOUT[batch], batch)
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(head + doc + rest)
    shutil.copy2(os.path.join(HERE, "verify-%s.sh" % batch), os.path.join(out, "verify.sh"))
    shutil.rmtree(final, ignore_errors=True)
    os.replace(out, final)
    out = final
    for line in problems:
        print("PROBLEM", line)
    print("staged", batch, len(manifest), "files ->", out)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
