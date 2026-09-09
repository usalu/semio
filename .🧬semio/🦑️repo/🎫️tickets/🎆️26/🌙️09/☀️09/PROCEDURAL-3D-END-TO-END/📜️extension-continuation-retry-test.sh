#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f5f6952-6c25-4056-a743-773b3668812e/scratchpad
cd /Users/ueli/Documents/semio
for i in $(seq 1 20); do
  echo "=== attempt $i $(date) ===" >> $S/cont-test-driver.log
  CARGO_TARGET_DIR=$S/target-cont RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- flow_eval_resolve flow_tessellate_resolve retained_route_dispositions command_ids_are_unique every_printed_op_line every_command_round_trips > $S/cont-test.log 2>&1
  if grep -q "^test result:" $S/cont-test.log; then echo "attempt $i RAN THE TESTS" >> $S/cont-test-driver.log; break; fi
  if grep -q "could not compile \`semio-s-artifact-procedural-generation3d\`" $S/cont-test.log; then echo "attempt $i OWN COMPILE ERROR" >> $S/cont-test-driver.log; break; fi
  grep "could not compile" $S/cont-test.log | tail -1 >> $S/cont-test-driver.log
  sleep 120
done
echo "=== driver done $(date) ===" >> $S/cont-test-driver.log
