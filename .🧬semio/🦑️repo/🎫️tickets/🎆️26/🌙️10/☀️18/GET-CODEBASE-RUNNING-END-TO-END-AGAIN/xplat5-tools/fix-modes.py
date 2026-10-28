"""🔧 Repairs the permission bits of tracked files that a staging tool left at 0600 (`tempfile.mkstemp` creates 0600 and `os.replace` kept it): python3 -I fix-modes.py [--root DIR] [--apply]. Dry run by default (lists the files and counts). A file is repaired to what the index records: 0755 for mode 100755, 0644 for 100644; symlinks, absent files, everything below `.🧬semio/` (tickets and caches) and names that look like secrets (`.env`, keys, tokens) are skipped; only `chmod`, no content is touched, no git command that modifies anything runs (`git ls-files -s` only reads)."""
import os, re, stat, subprocess, sys
SECRET = re.compile(r'(\.env($|\.)|\.pem$|\.key$|\.p12$|\.pfx$|id_rsa|id_ed25519|token|secret|credential|password)', re.I)
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.getcwd()
apply = "--apply" in sys.argv
listing = subprocess.run(["git", "-C", root, "ls-files", "-s", "-z"], capture_output=True, check=True).stdout.split(b"\0")
wrong = []
for row in listing:
    if not row: continue
    meta, path = row.split(b"\t", 1); recorded = meta.split()[0]
    if recorded == b"120000": continue
    target = os.path.join(root, path.decode("utf8", "surrogateescape"))
    try: current = stat.S_IMODE(os.lstat(target).st_mode)
    except FileNotFoundError: continue
    relative = path.decode('utf8', 'surrogateescape')
    if relative.startswith('.🧬semio/') or SECRET.search(relative): continue
    wanted = 0o755 if recorded == b"100755" else 0o644
    if current == 0o600 and wanted != current: wrong.append((target, wanted))
for target, wanted in wrong:
    if apply: os.chmod(target, wanted)
print(("repaired" if apply else "would repair") + " %d file(s) (0600 -> index mode)" % len(wrong))
if not apply:
    for target, wanted in wrong[:15]: print(" ", oct(wanted), os.path.relpath(target, root)[-110:])
