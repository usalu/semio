"""⏱️ Approximate phase timeline of a dashboard product log from its 10 s heartbeats (streams, low RAM)."""
import re, sys

beat = re.compile(r'^\[([^\]]+)\] running elapsedMs=(\d+)')
event = re.compile(r'^(\$ repo|graph: |\[bazel\] (session|plan|planning|this plan)|INFO: (Elapsed|Analyzed|From|\d+ processes)|\[\d+ / \d+\]|\[nx-native\]|\[cargo\] \d+ diag|\[cargo-provenance\]|\[cargo-target-recipe\]|cargo \d|\[capacity\]|\[lease-order\])')
script = re.compile(r'"command":"([^"]+)","stage":"([^"]+)"')
last = {}
prev_cmd = None
for n, line in enumerate(open(sys.argv[1], errors='replace'), 1):
    line = line.rstrip('\n')
    m = beat.match(line)
    if m:
        last[m.group(1)] = int(m.group(2)) // 1000
        continue
    s = script.search(line)
    if s and line.startswith('[script]'):
        cmd = s.group(1) + ':' + s.group(2)
        if cmd != prev_cmd:
            prev_cmd = cmd
            print(f"{n:6d} {dict(last)} [phase] {cmd}")
        continue
    if event.match(line):
        print(f"{n:6d} {dict(last)} {line[:170]}")
