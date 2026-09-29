#!/bin/zsh
# ⏱️ LW1 (session 15): sourced by every T6 law block. `step <label> <cap-s> <cmd…>` runs one law bounded by min(cap, what is left of
# the block budget `LW1_BUDGET_S` (default 2580 s = 43 min, so one native-lane hold stays < 45 min), kills the step's whole process
# group on expiry (TERM, 30 s grace, KILL — no orphaned cargo/rustc/test processes) and prints `LW1-STEP <label> rc=<rc> wall=…`;
# a step with < 90 s left prints `SKIPPED (budget)` and is re-queued by hand in a later block; `l1=alive` on a BEGIN/result line = an
# L1 train (`wp-l1/l1-run.sh`) was writing the tree meanwhile (result suspect, re-run). `finish` exits 1 when any step failed.
LW1_BLOCK_START=$(date +%s); LW1_BUDGET_S=${LW1_BUDGET_S:-2580}; LW1_FAILED=0
l1_state() { pgrep -f "wp-l1/l1-run.sh" > /dev/null && echo alive || echo idle; }
step() {
  local label=$1 cap=$2; shift 2
  local left=$(( LW1_BLOCK_START + LW1_BUDGET_S - $(date +%s) ))
  if [ $left -lt 90 ]; then echo "LW1-STEP $label SKIPPED (budget, ${left}s left) $(date '+%T')"; LW1_FAILED=1; return 125; fi
  [ $cap -gt $left ] && cap=$left
  echo "LW1-STEP-BEGIN $label cap=${cap}s $(date '+%T') load=$(sysctl -n vm.loadavg) l1=$(l1_state)"
  local s=$(date +%s)
  perl -e '
    my $n = shift; my $p = fork(); die "fork: $!" unless defined $p;
    if ($p == 0) { setpgrp(0, 0); exec { $ARGV[0] } @ARGV; exit 127 }
    $SIG{ALRM} = sub {
      print "LW1-ALARM ${n}s: TERM to process group $p\n"; kill "TERM", -$p;
      for (1 .. 30) { last unless kill 0, -$p; sleep 1 }
      kill "KILL", -$p; waitpid($p, 0); exit 142;
    };
    alarm $n; waitpid($p, 0); exit($? & 127 ? 128 + ($? & 127) : $? >> 8);
  ' $cap "$@"
  local rc=$?
  echo "LW1-STEP $label rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%T') l1=$(l1_state)"
  [ $rc -ne 0 ] && LW1_FAILED=1
  return $rc
}
finish() { exit $LW1_FAILED; }
