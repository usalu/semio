#!/usr/bin/env python3
"""usage: mixed-names.py MACTREE LINUXNODE:TREE -- file name and metadata cases between this macOS host (APFS, case-insensitive, creates NFD names) and a Linux node (Lima VM through limactl), both replicas running on the same group store. Prints one JSON line per case: what each side holds (names as hex of the UTF-8 bytes where they differ) and whether the two sides agree after settling. Cases: nfc-linux, nfd-mac, case-rename-linux, case-rename-mac, case-collision-linux, exec-linux, exec-mac, symlink-linux, symlink-mac."""
import json, os, subprocess, sys, time, unicodedata
mac, spec = sys.argv[1], sys.argv[2]; node, lin = spec.split(":", 1)
D = "names-" + str(int(time.time()))
def L(cmd): return subprocess.run(["limactl", "shell", node, "--", "sudo", "-u", "semio", "bash", "-c", cmd], capture_output=True, text=True).stdout
def lin_names(d): return sorted(x for x in L(f"ls -1 {lin}/{D}/{d} 2>/dev/null").split("\n") if x)
def mac_names(d):
    p = f"{mac}/{D}/{d}"
    return sorted(os.listdir(p)) if os.path.isdir(p) else []
def settle(check, seconds=40):
    end = time.time() + seconds
    while time.time() < end:
        if check(): return round(seconds - (end - time.time()), 1)
        time.sleep(0.25)
def show(names): return [n if n == unicodedata.normalize("NFC", n) and n.isascii() else n.encode().hex() + ":" + ("NFC" if n == unicodedata.normalize("NFC", n) else "NFD") for n in names]
def out(case, **kw): print(json.dumps({"case": case, **kw}, ensure_ascii=False), flush=True)
os.makedirs(os.path.join(mac, D), exist_ok=True)
NFC = unicodedata.normalize("NFC", "café"); NFD = unicodedata.normalize("NFD", "café")
def mk(sub): os.makedirs(os.path.join(mac, D, sub), exist_ok=True)
for sub in ("nfc-linux", "nfd-mac", "case-linux", "case-mac", "collide", "exec", "link"): mk(sub)
time.sleep(3)
L(f"mkdir -p {lin}/{D}/nfc-linux; printf x > {lin}/{D}/nfc-linux/{NFC}-linux.txt")
open(os.path.join(mac, D, "nfd-mac", NFD + "-mac.txt"), "w").write("y")
t = settle(lambda: len(mac_names("nfc-linux")) == 1 and len(lin_names("nfd-mac")) == 1)
out("nfc-linux+nfd-mac", settled_s=t, mac_has_linux_file=show(mac_names("nfc-linux")), linux_has_mac_file=show(lin_names("nfd-mac")), mac_own_file=show(mac_names("nfd-mac")), linux_own_file=show(lin_names("nfc-linux")))
L(f"mkdir -p {lin}/{D}/case-linux; printf a > {lin}/{D}/case-linux/readme.txt")
settle(lambda: mac_names("case-linux") == ["readme.txt"])
L(f"mv {lin}/{D}/case-linux/readme.txt {lin}/{D}/case-linux/README.txt")
t = settle(lambda: mac_names("case-linux") == ["README.txt"] and lin_names("case-linux") == ["README.txt"], 40)
out("case-rename-linux", settled_s=t, mac=mac_names("case-linux"), linux=lin_names("case-linux"), mac_bytes=open(os.path.join(mac, D, "case-linux", (mac_names("case-linux") or ["x"])[0])).read() if mac_names("case-linux") else None)
open(os.path.join(mac, D, "case-mac", "notes.txt"), "w").write("c")
settle(lambda: lin_names("case-mac") == ["notes.txt"])
os.rename(os.path.join(mac, D, "case-mac", "notes.txt"), os.path.join(mac, D, "case-mac", "NOTES.txt"))
t = settle(lambda: lin_names("case-mac") == ["NOTES.txt"] and mac_names("case-mac") == ["NOTES.txt"], 40)
out("case-rename-mac", settled_s=t, mac=mac_names("case-mac"), linux=lin_names("case-mac"))
L(f"mkdir -p {lin}/{D}/collide; printf 1 > {lin}/{D}/collide/Dup.txt; printf 2 > {lin}/{D}/collide/dup.txt")
time.sleep(15)
out("case-collision-linux", mac=mac_names("collide"), linux=lin_names("collide"))
L(f"mkdir -p {lin}/{D}/exec; printf '#!/bin/sh\\n' > {lin}/{D}/exec/l.sh; chmod 755 {lin}/{D}/exec/l.sh")
open(os.path.join(mac, D, "exec", "m.sh"), "w").write("#!/bin/sh\n"); os.chmod(os.path.join(mac, D, "exec", "m.sh"), 0o755)
t = settle(lambda: os.path.exists(os.path.join(mac, D, "exec", "l.sh")) and "m.sh" in lin_names("exec"))
out("exec", settled_s=t, mac_sees_linux_script_mode=oct(os.stat(os.path.join(mac, D, "exec", "l.sh")).st_mode & 0o777) if os.path.exists(os.path.join(mac, D, "exec", "l.sh")) else None, linux_sees_mac_script_mode=L(f"stat -c %a {lin}/{D}/exec/m.sh").strip())
L(f"mkdir -p {lin}/{D}/link && cd {lin}/{D}/link && printf t > target.txt && ln -s target.txt l-link")
os.symlink("target.txt", os.path.join(mac, D, "link", "m-link")) if os.path.exists(os.path.join(mac, D, "link")) else None
t = settle(lambda: os.path.islink(os.path.join(mac, D, "link", "l-link")) and L(f"test -L {lin}/{D}/link/m-link && echo y").strip() == "y")
out("symlink", settled_s=t, mac_sees_linux_link=os.readlink(os.path.join(mac, D, "link", "l-link")) if os.path.islink(os.path.join(mac, D, "link", "l-link")) else None, linux_sees_mac_link=L(f"readlink {lin}/{D}/link/m-link").strip())
print("dir", D)
