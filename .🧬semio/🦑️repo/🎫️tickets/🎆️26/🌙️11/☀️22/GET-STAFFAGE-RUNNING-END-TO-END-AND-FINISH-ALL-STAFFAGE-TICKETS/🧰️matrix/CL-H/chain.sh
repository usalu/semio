#!/bin/zsh
# ⛓️ The closing chain of lane CL-H, one queue request after the other: the writes with their proofs (1200 MB), then the real test files (2500 MB).
h="${0:A:h}"
zsh "$h/q.sh" 1200 30000 proves proves.sh
zsh "$h/q.sh" 2500 14000 suites suites.sh person-queuing person-on-crutches tour-guide parent-with-stroller parent-carrying-toddler
