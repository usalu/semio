import os, re, sys, json, subprocess
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps
R = rgl.ROOT

def closure_files(manifest_dir_rel):
    out = subprocess.check_output(["bun", os.path.join(os.path.dirname(__file__), "owners.ts")], input=json.dumps([manifest_dir_rel]).encode(), cwd=R)
    d = json.loads(out)[manifest_dir_rel]
    return d

INC = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')

def header_insert_pos(text):
    pos = 0
    lines = text.split("\n")
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].strip() == "" or lines[i].startswith("#![")):
        pos += len(lines[i]) + 1
        i += 1
    return pos

def fixturize(manifest_dir_rel, only_files=None, dry=False):
    d = closure_files(manifest_dir_rel)
    prod = set(d["prod"]); test = [f for f in d["test"] if f not in prod and f.endswith(".rs")]
    if only_files is not None:
        test = [f for f in test if f in only_files]
    manifest = os.path.join(R, manifest_dir_rel, "Cargo.toml")
    changed = []
    for rel in test:
        p = os.path.join(R, rel)
        t = rgl.read(p)
        if not INC.search(t):
            continue
        def sub(m):
            target = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
            if target.endswith(".rs"):
                return m.group(0)
            r = os.path.relpath(target, os.path.join(R, manifest_dir_rel))
            return f'fixture_text!("{r}")'
        t2 = INC.sub(sub, t)
        if t2 == t:
            continue
        if "use semio_framework_test_fixtures::fixture_text;" not in t2 and "fixture_text" not in t.replace("include_str","") :
            pos = header_insert_pos(t2)
            t2 = t2[:pos] + "use semio_framework_test_fixtures::fixture_text;\n" + t2[pos:]
        if not dry:
            rgl.write(p, t2)
        changed.append(rel)
    return changed

if __name__ == "__main__":
    print(fixturize(sys.argv[1]))
