"""[gram3-g] env LAW_OVERLAY=<dir> copies the files of that directory (paths relative to the build library, e.g. materialize.py output) into the scratch tree, for laws that read module sources from disk. env BATCH_DIRS=<dir>[,<dir>] additionally serves the anchored edits of those batches in memory (walk-g preload_generic). usage: law.py <batch dir> [<batch dir>...] -- <test file relative to a batch tree> ... : copies the trees of the batches over one scratch tree (C/🗑️generated/gram3-g/law), rewrites the relative imports that do not resolve inside it to the live library location, applies nothing live, runs `bun test` on the given files and prints the summary. Anchored edits of batches are NOT applied (laws of new modules only)."""
import json, os, re, shutil, subprocess, sys
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
ROOT = "/Users/ueli/Documents/semio/"
BUN = ROOT + ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
args = sys.argv[1:]
cut = args.index("--")
batches, tests = args[:cut], args[cut + 1:]
scratch = ROOT + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/gram3-g/law"
shutil.rmtree(scratch, ignore_errors=True)
for batch in batches:
    flat = os.path.join(batch, "tree")
    for rel, row in json.load(open(os.path.join(flat, "manifest.json"), encoding="utf8")).items():
        os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
        shutil.copyfile(os.path.join(flat, row["file"]), os.path.join(scratch, rel))
overlay = os.environ.get("LAW_OVERLAY")
if overlay:
    for base, _, names in os.walk(overlay):
        for name in names:
            target = os.path.join(scratch, LIB, "🏗️build", os.path.relpath(os.path.join(base, name), overlay))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(os.path.join(base, name), target)
top = os.path.join(scratch, "🧰️framework")
for base, _, names in os.walk(scratch):
    for name in names:
        if not name.endswith((".ts", ".mjs", ".mts")):
            continue
        path = os.path.join(base, name)
        text = open(path, encoding="utf8").read()
        live = os.path.join(ROOT, os.path.relpath(base, scratch))

        def fix(match):
            spec = match.group(2)
            if not spec.startswith(".") or os.path.exists(os.path.normpath(os.path.join(base, spec))):
                return match.group(0)
            return match.group(1) + '"' + os.path.normpath(os.path.join(live, spec)) + '"'

        new = re.sub(r'(from\s*)"([^"]+)"', fix, text)

        def fix_url(match):
            spec = match.group(1)
            if os.path.exists(os.path.normpath(os.path.join(base, spec))):
                return match.group(0)
            return 'new URL("' + os.path.normpath(os.path.join(live, spec)) + '", import.meta.url)'

        new = re.sub(r'new URL\("(\.[^"]+)",\s*import\.meta\.url\)', fix_url, new)
        if new != text:
            open(path, "w", encoding="utf8").write(new)
pre = ["--preload", ROOT + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/walk-g-tools/staged/preload_generic.ts"] if os.environ.get("BATCH_DIRS") else []
run = subprocess.run([BUN, "test", *pre, *[os.path.join(scratch, t) for t in tests]], cwd=ROOT, capture_output=True, text=True)
print((run.stdout + run.stderr)[-3500:])
sys.exit(run.returncode)
