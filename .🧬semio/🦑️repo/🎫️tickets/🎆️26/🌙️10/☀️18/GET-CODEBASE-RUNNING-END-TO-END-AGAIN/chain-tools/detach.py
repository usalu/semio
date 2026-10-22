"""🛰️ Detached launcher: `detach.py <log> <cmd...>` appends the command output to <log> and prints the pid.

The Claude desktop app kills every process it attributes to a session when it quits for an update: at 2026-10-22
22:56:54 `beforeQuitForUpdate` stopped the CLI and all detached runs died with SIGTERM within two seconds,
although they had their own Unix session. A new session does not help, so on macOS the command is started
from Terminal.app (hidden, `open -g -j`): its lineage (parent, environment, responsible app) is Terminal's, it keeps
the Documents permission that launchd jobs lack, and it survives an app update. `SEMIO_DETACH=session` forces the
plain new-session launch, which is also used on Linux, in devcontainers and on Windows.
"""
import os, shlex, subprocess, sys, tempfile, time

DROP = ("CLAUDE", "ANTHROPIC", "__CF", "XPC_", "TERM_SESSION_ID")


def posix(log: str, argv: list) -> int:
    handle = open(log, "ab", buffering=0)
    flags = {"creationflags": 0x00000008 | 0x00000200} if os.name == "nt" else {"start_new_session": True}
    return subprocess.Popen(argv, stdout=handle, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, **flags).pid


def terminal(log: str, argv: list) -> int:
    base = os.path.abspath(log) + ".launch-%d" % time.time_ns()
    pidfile = base + ".pid"
    exports = "".join("export %s=%s\n" % (k, shlex.quote(v)) for k, v in os.environ.items() if k.isidentifier() and not k.startswith(DROP))
    script = "#!/bin/sh\ntrap '' HUP\n%scd %s || exit 1\necho $$ > %s\nrm -f \"$0\"\nexec %s </dev/null >>%s 2>&1\n" % (exports, shlex.quote(os.getcwd()), shlex.quote(pidfile), " ".join(shlex.quote(a) for a in argv), shlex.quote(os.path.abspath(log)))
    path = base + ".command"
    with open(path, "w") as handle:
        handle.write(script)
    os.chmod(path, 0o700)
    started = subprocess.run(["open", "-g", "-j", "-a", "Terminal", path], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    if started.returncode != 0:
        os.unlink(path)
        raise SystemExit("open -a Terminal failed: " + started.stdout.strip())
    for _ in range(300):
        if os.path.exists(pidfile):
            text = open(pidfile).read().strip()
            if text:
                os.unlink(pidfile)
                return int(text)
        time.sleep(0.1)
    raise SystemExit("Terminal did not start %s" % path)


def main() -> None:
    log, argv = sys.argv[1], sys.argv[2:]
    open(log, "ab").close()
    if sys.platform == "darwin" and os.environ.get("SEMIO_DETACH", "terminal") != "session":
        print(terminal(log, argv))
    else:
        print(posix(log, argv))


main()
