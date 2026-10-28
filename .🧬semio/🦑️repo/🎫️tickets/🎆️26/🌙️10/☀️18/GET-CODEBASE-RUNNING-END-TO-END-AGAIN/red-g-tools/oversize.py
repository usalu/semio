"""📏 Lists ticket-folder files whose Windows checkout path exceeds the legacy limit (streaming; groups by first folder below the ticket)."""
import os, subprocess, sys, collections
root = "/Users/ueli/Documents/semio"
ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
units = lambda s: len(s.encode("utf-16-le")) // 2
limit, roots = 260, ["C:\\Users\\username\\source\\repos\\semio", "C:\\git\\semio"]
proc = subprocess.Popen(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", ticket], cwd=root, stdout=subprocess.PIPE)
groups = collections.Counter(); worst = {}
for raw in proc.stdout.read().split(b"\0"):
    if not raw: continue
    path = raw.decode("utf8")
    if not os.path.exists(os.path.join(root, path)): continue
    for r in roots:
        n = len(r) + 1 + units(path)
        if n > limit:
            key = path[len(ticket) + 1:].split("/")[0] + " [" + r[:6] + "]"
            groups[key] += 1; worst[key] = max(worst.get(key, 0), n)
for key, count in groups.most_common(): print(count, worst[key], key)
