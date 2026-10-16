"""Dependencies a production manifest lost/gained between the pre-edit snapshot and the live file (package names, as they appear in Cargo.lock). usage: lockdiff.py <cid> <manifest rel>"""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import union


def names(text):
    out = set()
    sec = None
    for line in text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and sec.endswith("dependencies") and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip().split(".")[0]
            if re.match(r"^[A-Za-z0-9_\-]+$", key):
                pk = re.search(r'package\s*=\s*"([^"]+)"', line)
                out.add(pk.group(1) if pk else key)
    return out


if __name__ == "__main__":
    cid, rel = sys.argv[1], sys.argv[2]
    old = names(open(f"{union.W}/{cid}/snap/{rel}", encoding="utf-8").read())
    new = names(open(f"{union.ROOT}/{rel}", encoding="utf-8").read())
    print(",".join(sorted(old - new)), "|", ",".join(sorted(new - old)))
