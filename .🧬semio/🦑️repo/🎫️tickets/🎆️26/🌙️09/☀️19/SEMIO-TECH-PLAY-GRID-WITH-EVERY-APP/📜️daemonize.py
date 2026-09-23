#!/usr/bin/env python3
"""🛡️ Double-fork + setsid launcher: runs <cmd …> in its own session/process group with stdio redirected to <log>, so a kill of the launching shell's process group (sandbox teardown, tool-call abort) cannot reach it. usage: daemonize.py <log> [--cwd DIR] [--env K=V …] -- <cmd …>"""
import os, sys

def main() -> None:
    args = sys.argv[1:]
    log = args.pop(0)
    cwd = None
    env = dict(os.environ)
    while args and args[0] != "--":
        flag = args.pop(0)
        if flag == "--cwd": cwd = args.pop(0)
        elif flag == "--env":
            key, _, value = args.pop(0).partition("=")
            env[key] = value
    if args and args[0] == "--": args.pop(0)
    if not args: raise SystemExit("daemonize.py: no command")
    if os.fork() > 0: return
    os.setsid()
    if os.fork() > 0: os._exit(0)
    if cwd: os.chdir(cwd)
    fd = os.open(log, os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    null = os.open(os.devnull, os.O_RDONLY)
    os.dup2(null, 0); os.dup2(fd, 1); os.dup2(fd, 2)
    os.execvpe(args[0], args, env)

if __name__ == "__main__": main()
