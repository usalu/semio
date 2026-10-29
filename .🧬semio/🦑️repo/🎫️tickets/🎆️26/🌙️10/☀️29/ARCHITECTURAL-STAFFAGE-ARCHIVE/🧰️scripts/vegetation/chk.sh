#!/bin/bash
D=$(cd "$(dirname "$0")"; pwd); S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"; cd "$S" && repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$D/assets/$1.json" 2>&1 | grep -v "^\[" 
