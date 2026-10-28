#!/bin/sh
# 🔬️ [obsopen] birth.sh <mirror> <file relative to 👁️observation/🧪️tests> <pattern|-> <outname> [birth=setup] [limit-seconds=400]: one observation test file beneath the genuine birth inside the mirror (pinned Bun), hard time limit, output in 🗑️generated/obsopen/<outname>.txt
export SEMIO_MIRROR_HOME="${SEMIO_MIRROR_HOME:-/private/tmp/sm}"; C="$(cd "$(dirname "$0")/.." && pwd)"; name="$1"; f="$2"; pat="$3"; out="$4"; birth="${5:-setup}"; lim="${6:-400}"
R="$SEMIO_MIRROR_HOME/$name"; R="$(cd "$R" && pwd -P)"; want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"
[ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done
O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests"; GB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
G="$C/🗑️generated/obsopen"; mkdir -p "$G" "$R/.tmp/oo"
python3 -I - "$C/mirror-tools/mirror.py" "$name" "$G/$out.txt" "$lim" "$R" "$B" "$GB" "--file=$O/$f" "--birth=$birth" "--root=$R" "--pattern=$pat" <<'PY'
import subprocess,sys,os,signal
mm,name,out,lim,R,B,GB,*args=sys.argv[1:]
args=[a for a in args if a!='--pattern=-']
env=dict(os.environ,**{k:v for k,v in os.environ.items() if k.startswith("PROBE_")},SEMIO_TEST_ARTIFACT_DIR=R+"/.tmp/oo")
cmd=["python3","-I",mm,"run",name,"--seal-allow",os.path.dirname(out),"--peers-ok","--","nice","-n","10",B,GB,*args]
with open(out,"w") as fh:
    p=subprocess.Popen(cmd,stdout=fh,stderr=subprocess.STDOUT,env=env,start_new_session=True)
    try: p.wait(int(lim))
    except subprocess.TimeoutExpired:
        fh.write("\n[DEBUG] LIMIT %ss reached, killing own process group\n"%lim); fh.flush()
        os.killpg(p.pid,signal.SIGTERM)
        try: p.wait(15)
        except subprocess.TimeoutExpired: os.killpg(p.pid,signal.SIGKILL)
sys.exit(p.returncode or 0)
PY
rc=$?; for d in $(mount | grep "$R/.cache/b/o/m/tmp/gf-" | awk '{print $1}'); do hdiutil detach "$d" >/dev/null 2>&1; done
printf '%s rc=%s: %s\n' "$out" "$rc" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$G/$out.txt" | tr -s ' \n' ' ')"; grep -E '^\(fail\)|GUARD|^error:|LIMIT' "$G/$out.txt" | cut -c1-240 | head -20
