#!/usr/bin/env python3
"""🔁 Round trip in a scratch copy: dry run of the probe collection (plus its nested collection with --nested) per artifact, apply of the plan to a scratch copy of the artifact (files only: no twins, no builds), then the same plan again, which must be empty (python3 -I roundtrip.py [--nested] [artifact ...]; prints files, unsupported layers, parse findings of the first plan and the files of the second plan)."""
import json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REAL = os.environ.get("SEMIO_ROOT") or os.path.abspath(os.path.join(HERE, *[".."] * 11))
NORM = "✏️s/🔌️plugins/📕️norm"


def source_root():
    sys.path.insert(0, HERE)
    import core
    return core.SEMIO


def child(short, scratch, spec_path):
    sys.path.insert(0, HERE)
    import snapshot
    first = snapshot.plan_of(short, "add-collection", spec_path)
    import syntax
    out_syntax = syntax.check_plan(first.plan)
    n_first = len(first.plan.files)
    out = {"files": n_first, "unsupported": first.unsupported, "syntax": {k.rsplit("/", 1)[-1]: v for k, v in out_syntax.items()}}
    if not first.unsupported:
        first.plan.commit()
        second = snapshot.plan_of(short, "add-collection", spec_path)
        out["second"] = sorted(second.plan.files)[:5]
        out["secondCount"] = len(second.plan.files)
        out["secondUnsupported"] = second.unsupported
    print(json.dumps(out, ensure_ascii=False))


NESTED = {"name": "probeReadings", "entity": "ProbeReading", "emoji": "🧪", "doc": "d", "label": {"en": "Readings", "de": "Messungen"}, "id": {"en": "Reading ID", "de": "Messungs-ID"}, "singular": {"en": "probe reading", "de": "Probemessung"}, "kindSlug": "probe-reading", "fields": [{"name": "depth", "type": "f64", "unit": "m", "label": {"en": "Depth", "de": "Tiefe"}, "sample": 1.0}]}


def main(argv):
    if argv and argv[0] == "--child":
        return child(*argv[1:4])
    nested = "--nested" in argv
    argv = [a for a in argv if a != "--nested"]
    sys.path.insert(0, HERE)
    import probe
    src = source_root()
    for short in argv or probe.ALL:
        scratch = tempfile.mkdtemp(prefix="evolve-roundtrip-")
        try:
            art = next(d for d in os.listdir(f"{src}/{NORM}/🗿️artifacts") if d.endswith(short))
            ignore = shutil.ignore_patterns("target", "node_modules", "__pycache__", "🗑️generated")
            shutil.copytree(f"{src}/{NORM}/🗿️artifacts/{art}", f"{scratch}/{NORM}/🗿️artifacts/{art}", ignore=ignore, symlinks=True)
            shutil.copytree(f"{src}/{NORM}/🧪️tests", f"{scratch}/{NORM}/🧪️tests", ignore=ignore, symlinks=True)
            spec = f"{scratch}/spec.json"
            payload = probe.collection_spec(short)
            if nested:
                payload["collection"]["nested"] = [NESTED]
            json.dump(payload, open(spec, "w", encoding="utf8"))
            done = subprocess.run([sys.executable, "-I", HERE + "/roundtrip.py", "--child", short, scratch, spec], capture_output=True, text=True, env={**os.environ, "SEMIO_ROOT": scratch})
            line = done.stdout.strip().splitlines()[-1] if done.stdout.strip() else done.stderr.strip()[-400:]
            print(short, line, flush=True)
        finally:
            shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    main(sys.argv[1:])
