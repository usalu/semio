"""🎞️ agents3 dry run: stands in for `bun repo --tool=bazel run <target>`; replays the recorded gate log that matches the agent's run, in the agent's language (no build, no repo process).

`fakeruns.py <agent> <label> <target> <language>` (env `AGENTS3_FAKE_NEGATIVE=1` replays the 10-27 starved print log for agent B).
Logs: A course edit g48c-edit, B print edit g48p-edit, C dashboard no-op g48-dash2 (all after the quiet follow-up landed, with `behind` suffixes and growth waits); the negative control swaps B for g29-print (229 grow-wait rows of 60 s or more with nothing else running).
"""
import os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, ".."))
RUN = os.path.join(C, "gate-g-tools", "run")
LOGS = {"A": "g48c-edit.log", "B": "g48p-edit.log", "C": "g48-dash2.log"}
NEGATIVE = {"B": "g29-print.log"}


def main():
    agent, label, target, language = sys.argv[1:5]
    log = (NEGATIVE if os.environ.get("AGENTS3_FAKE_NEGATIVE") == "1" else {}).get(agent) or LOGS[agent]
    delay = os.environ.get("AGENTS3_FAKE_DELAY", "0.004")
    code = subprocess.run([sys.executable, "-I", os.path.join(C, "harness3-tools", "fakebuild.py"), os.path.join(RUN, log), "--delay", delay, "--language", language]).returncode
    sys.exit(code)


main()
