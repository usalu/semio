#!/bin/bash
HERE="$(cd "$(dirname "$0")" && pwd)"
"$HERE/vegetation-hsearch-run.sh" agave ${ITERS:-30} 50/elevation 50/front 50/plan 100/elevation 100/front 100/plan 200/elevation 200/front 200/plan
