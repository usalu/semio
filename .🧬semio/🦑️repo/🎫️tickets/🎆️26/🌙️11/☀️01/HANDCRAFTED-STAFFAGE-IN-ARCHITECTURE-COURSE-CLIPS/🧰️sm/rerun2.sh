#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
B="🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
export SM_SCENE_TIMEOUT=100 SM_TIMEOUT=500 PYTHONUTF8=1
"$T/🧰️sm/run.sh" "$B/🔌️exchange/🐍️.py" Beat5_Interfaces ""| head -1
"$T/🧰️sm/run.sh" "$B/🔗️consistency/🐍️.py" Beat6_Consistency "" | head -1
"$T/🧰️sm/run.sh" "$B/🧩️models/🐍️.py" Beat4_PartialModels "" | head -1
