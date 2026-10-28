#!/usr/bin/env python3
"""📍 Keyed provider-edit probe for the product proof of the `providers` batch: adds a NEW library module and one import of it to the repository-tooling provider module (`📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts`, reached by product commands only through the `repository-tooling` port), and removes both byte-exactly again.
usage: python3 -I provprobe.py apply|revert|status <root>   (apply keeps a byte backup next to this script; revert restores it and deletes the probe module and its directory; file modes are kept)"""
import os, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
PORT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/🟦️.ts"
PROBE_DIR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🛠️repository-tooling/🔌️provider/📍️peeredit2-probe"
PROBE = PROBE_DIR + "/🟦️.ts"
LINE = 'import "./📍️peeredit2-probe/🟦️.ts";\n'
BACKUP = os.path.join(HERE, "provprobe.port.backup")
def main():
    verb, root = sys.argv[1], os.path.abspath(sys.argv[2])
    port, probe = os.path.join(root, PORT), os.path.join(root, PROBE)
    applied = os.path.exists(probe)
    if verb == "status": print("applied" if applied else "clean"); return
    if verb == "apply":
        if applied: sys.exit("probe already applied")
        shutil.copyfile(port, BACKUP)
        os.makedirs(os.path.dirname(probe)); open(probe, "w", encoding="utf8").write("/** 📍️ Probe module of the peeredit2 lane: a keyed provider roster change. */\nexport const peeredit2ProbeV1 = 1;\n")
        os.chmod(probe, 0o666 & ~current_umask())
        mode = stat.S_IMODE(os.stat(port).st_mode); text = open(port, encoding="utf8").read()
        open(port, "w", encoding="utf8").write(LINE + text); os.chmod(port, mode); print("probe applied"); return
    if verb == "revert":
        if not applied: sys.exit("probe not applied")
        mode = stat.S_IMODE(os.stat(port).st_mode)
        with open(BACKUP, "rb") as source, open(port, "wb") as target: target.write(source.read())
        os.chmod(port, mode); os.remove(probe); os.rmdir(os.path.join(root, PROBE_DIR)); os.remove(BACKUP); print("probe reverted"); return
    sys.exit("verb: apply|revert|status")
def current_umask():
    mask = os.umask(0); os.umask(mask); return mask
main()
