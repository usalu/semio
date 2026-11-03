set -u
BUN=$(ls /home/ueli/.cache/semio/runtimes/bun/*/bun | head -1)
S=/mnt/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage
cd /mnt/c/git/semio
rm -rf /tmp/lxout
echo "-- with parent"; "$BUN" "$S/🔨️modules/📤️export/🎭️cast/🟦️.ts" --manifest /tmp/batch-copy.json --ids quick --out /tmp/lxout --parent $$ 2>&1 | head -5
rm -rf /tmp/lxout
echo "-- stdout piped, stdin closed, stderr pipe"; python3 -I -c "
import subprocess,sys
p=subprocess.Popen(['$BUN','$S/🔨️modules/📤️export/🎭️cast/🟦️.ts','--manifest','/tmp/batch-copy.json','--ids','quick','--out','/tmp/lxout','--parent','1'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True,start_new_session=True)
print(p.stderr.read()[:400]);print(p.wait())
"
