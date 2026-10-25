"""🛰️ Headless detached launcher: `detach.py <log> <cmd...>` appends the command output to <log> and prints the pid.

The command runs in its own session (POSIX `setsid`, Windows `DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP`) with no
window, stdin from the null device and a scrubbed environment. A Claude desktop app restart for an update still
terminates the runs it attributes to a session (2026-10-22 22:56); rerun the same run-id after such a restart.
"""
import os, subprocess, sys

DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID")


def main() -> None:
    log, argv = sys.argv[1], sys.argv[2:]
    env = {k: v for k, v in os.environ.items() if not k.startswith(DROP)}
    flags = {"creationflags": 0x00000008 | 0x00000200} if os.name == "nt" else {"start_new_session": True}
    with open(log, "ab", buffering=0) as handle:
        print(subprocess.Popen(argv, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, env=env, close_fds=True, **flags).pid)


main()
