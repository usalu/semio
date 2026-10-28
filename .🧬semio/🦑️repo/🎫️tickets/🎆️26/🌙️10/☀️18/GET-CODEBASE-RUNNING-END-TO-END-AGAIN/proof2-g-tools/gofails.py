"""📋 Streams one measured Go log and prints the failing tests, the guard lines and the package verdicts: gofails.py <log>."""
import re, sys
keep = re.compile(r"^(--- FAIL|FAIL\b|ok\s|\?\s|FAIL:|\s+resolved |\s+\S+_test\.go:\d+:|panic:|Error:|\s*zz_semio)")
seen = 0
with open(sys.argv[1], encoding="utf8", errors="replace") as stream:
    for line in stream:
        if keep.search(line) and not line.startswith("[script]"):
            print(line.rstrip()[:300])
            seen += 1
            if seen > 80:
                break
