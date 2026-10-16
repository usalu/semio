"""⏯️ WP5 cut of `⏯️tool-run/🦀️.rs`: `is_tool_run_action_id` becomes the lift file of crate `job`. usage: CLOSURE=<closure.json> python3 carve_toolrun.py"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L, lift3
lift3.write = L.write; lift3.delete = L.delete
TR = L.PLUG + '/⏯️tool-run/🦀️.rs'
TT = L.PLUG + '/⏪️time-travel/🦀️.rs'
JOB_LIFT = L.PLUG + '/👷️job/🛬️lifted/⏯️tool-run/🦀️.rs'
def main():
    lift3.lift(TR, ['is_tool_run_action_id'], JOB_LIFT, 'lifted_job')
if __name__ == '__main__': main()
