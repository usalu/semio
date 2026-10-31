#!/bin/bash
source "$(dirname "$0")/lib.sh"
rm -f "$G/extfix-run-final.rc"
for b in cad-spatial-shape flow-bim flow-brep flow-dictionary flow-draw flow-list flow-logic flow-math flow-primitive flow-text playbook-procedural process-concrete process-metal process-robotic process-wood sourcing-beams sourcing-slabs sourcing-windows cad-aec-building cad-aec-energy; do chk_retry "$b" final; done
echo done > "$G/extfix-run-final.rc"
