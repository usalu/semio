"""[gram3-g] maximum and 90th percentile of the footprint (MB) and of the resident set (MB) in a fp.sh sample file. usage: fpmax.py <file>..."""
import re, sys
for path in sys.argv[1:]:
    fps, rss = [], []
    for line in open(path, encoding="utf8"):
        m = re.search(r"rss (\d+) .* fp (\d+) MB", line)
        if m:
            rss.append(int(m.group(1))); fps.append(int(m.group(2)))
    fps.sort(); rss.sort()
    q = lambda v, p: v[min(len(v) - 1, int(len(v) * p))] if v else 0
    print(f"{path.split('/')[-1]}: samples {len(fps)} footprint max {fps[-1] if fps else 0} p90 {q(fps,.9)} median {q(fps,.5)} | rss max {rss[-1] if rss else 0} p90 {q(rss,.9)}")
