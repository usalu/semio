#!/bin/bash
# author 55 white cane: kit build, props, kit-fit 100/200 on the humans, props again
source "$(dirname "$0")/author-33-env.sh"; cd "$(dirname "$0")"
python3 author_55_cane.py build >/dev/null || exit 1
for n in 100 200; do $BUN kit-fit.ts humans/person-with-white-cane --scale $n --rounds 2 --write 2>&1 | grep "^kit-fit" | cut -c1-160; done
python3 author_55_cane.py >/dev/null || exit 1
./figure-fails.sh humans/person-with-white-cane | cut -c1-440
