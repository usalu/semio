#!/bin/zsh
# usage: post.sh <label>   static checks on the installed (live) tree; logs go to 🗑️generated/c4p34-post-<label>-*.log
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
T="$F/c4p34-tools"
G="$F/🗑️generated"
label=${1:-live}
cd "$T" || exit 2
python3 live.py > "$G/c4p34-post-$label-live.log" 2>&1; echo "live.py exit=$?" | tee -a "$G/c4p34-post-$label.exit"
python3 check.py > "$G/c4p34-post-$label-check.log" 2>&1; echo "check.py exit=$?" | tee -a "$G/c4p34-post-$label.exit"
python3 modclash.py png jpg avi pptx dxfbin dxfschema zlib > "$G/c4p34-post-$label-modclash.log" 2>&1; echo "modclash exit=$?" | tee -a "$G/c4p34-post-$label.exit"
for t in png jpg avi pptx dxfbin dxfschema zlib; do python3 defscan.py $t; done > "$G/c4p34-post-$label-defscan.log" 2>&1; echo "defscan exit=$?" | tee -a "$G/c4p34-post-$label.exit"
python3 newdirs.py > newdirs.txt; (ulimit -n 65536; bun tax.ts newdirs.txt) > "$G/c4p34-post-$label-tax.log" 2>&1; echo "tax exit=$?" | tee -a "$G/c4p34-post-$label.exit"
sed 's#.*/##' newdirs.txt > newnames.txt; bun ../c4p5-tools/rgi.mjs newnames.txt > "$G/c4p34-post-$label-rgi.log" 2>&1; tail -1 "$G/c4p34-post-$label-rgi.log"
python3 - <<'PY' > "$G/c4p34-post-$label-siblings.log" 2>&1
import os, sys, importlib
sys.path.insert(0, '.')
import c4p34 as P
V = '️'
bad = 0
for t in 'png jpg avi pptx dxfbin dxfschema zlib'.split():
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.join(P.REPO, os.path.dirname(sp['file']))
    layers = [(base, [c['folder'] for c in sp['children']] + ([sp['inner']['folder']] if sp.get('inner') else []))]
    if sp.get('inner'):
        layers.append((os.path.join(base, sp['inner']['folder']), [c['folder'] for c in sp['inner']['children']]))
    for d, new in layers:
        emos = {}
        for name in sorted(os.listdir(d)):
            if os.path.isdir(os.path.join(d, name)):
                e = name.split(V)[0]
                emos.setdefault(e, []).append(name)
        for e, names in emos.items():
            if len(names) > 1:
                bad += 1
                print('SIBLING EMOJI CLASH', t, d[-60:], names)
print('sibling emoji clashes', bad)
PY
tail -1 "$G/c4p34-post-$label-siblings.log"
python3 rustfmt_live.py live > "$G/c4p34-post-$label-rustfmt.log" 2>&1; tail -1 "$G/c4p34-post-$label-rustfmt.log"
