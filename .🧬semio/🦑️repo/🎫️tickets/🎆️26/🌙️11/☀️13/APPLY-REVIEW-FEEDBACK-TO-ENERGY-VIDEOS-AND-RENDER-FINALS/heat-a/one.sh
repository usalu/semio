#!/bin/bash
# usage: one.sh <folder> <Scene>
TK="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
cd "$TK/🗑️generated/heat-a-out/m"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview
F="C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/$1/🐍️.py"
uv run --project "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim -ql -s --media_dir "$TK/🗑️generated/heat-a-out/m" "$F" $2 > "$TK/🗑️generated/heat-a-out/one-$2.log" 2>&1
echo done >> "$TK/🗑️generated/heat-a-out/one-$2.log"
