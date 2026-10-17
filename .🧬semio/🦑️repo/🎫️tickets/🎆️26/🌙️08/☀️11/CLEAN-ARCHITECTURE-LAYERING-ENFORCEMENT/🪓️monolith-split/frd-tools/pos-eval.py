import json, re, sys, os, collections
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/"
NAMES = re.compile(r'fixture_(?:text|bytes|path)|manifest_dir|repo_root|repo_text|repo_bytes|test_fixtures')
for fam in sys.argv[1:]:
    for cfg in ("lib", "test"):
        p = G + f"frd-{fam}-{os.environ.get('POS_LABEL', 'run')}-cur-{cfg}-final.err"
        if not os.path.exists(p): continue
        total = 0; hits = []; codes = collections.Counter()
        for line in open(p, encoding="utf-8", errors="replace"):
            line = line.strip()
            if not line.startswith("{"): continue
            try: d = json.loads(line)
            except Exception: continue
            if d.get("level") != "error": continue
            total += 1
            msg = d.get("message", "")
            hit = bool(NAMES.search(msg))
            for s in d.get("spans", []):
                if not s.get("is_primary"): continue
                for tx in s.get("text", []):
                    seg = tx.get("text", "")[tx.get("highlight_start", 1) - 1: tx.get("highlight_end", 1) - 1]
                    if re.match(r'\s*(?:crate::|::)?(?:semio_framework_test_fixtures::)?(?:fixture_(?:text|bytes|path)|manifest_dir|repo_root|repo_text|repo_bytes)\b', seg) or "semio_framework_test_fixtures" in seg: hit = True
                if NAMES.search(s.get("label") or ""): hit = True
            if re.search(r"cannot find macro .(fixture_|manifest_dir|repo_)", msg) or "unresolved import `semio_framework_test_fixtures" in msg: hit = True
            if hit: hits.append((d.get("code", {}) and d["code"].get("code"), msg[:100]))
        print(fam, cfg, "errors", total, "reader-related", len(hits), hits[:4])
