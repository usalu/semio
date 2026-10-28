cd /c/git/semio
for w in section-winter section-summer bars plan-arcs facade marker; do
  timeout 300 "/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe" -I ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/toolkit-probe-anim.py" $w 2>&1 | grep -a "DEBUG\|Error\|File" | sed 's/[^ -~]//g' >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/toolkit/anim-all.txt"
done
echo finished >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/toolkit/anim-all.txt"
