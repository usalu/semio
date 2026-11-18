#!/bin/bash
HERE="$(cd "$(dirname "$0")" && pwd)"
ITERS=${ITERS:-14} "$HERE/vegetation-hsearch-run.sh" date-palm ${ITERS:-14} 200/elevation 200/front 200/plan 100/elevation 100/front 100/plan
