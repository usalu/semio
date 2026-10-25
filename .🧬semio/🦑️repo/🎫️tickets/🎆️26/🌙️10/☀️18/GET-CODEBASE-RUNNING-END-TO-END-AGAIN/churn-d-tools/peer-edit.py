"""🧨️ A scripted peer that edits one file the way editors and other agents do while a build runs: `identity` replaces it atomically by an identical copy (new inode and ctime), `append` replaces it atomically by its bytes plus a newline, `restore` puts the original bytes back atomically. It waits `delay` seconds before each step, logs every step with its time, and always ends with the original bytes (also on SIGTERM).
usage: peer-edit.py <log.json> <file> <backup> <step:delay> ...   e.g. identity:2 append:8 restore:10"""
import hashlib, json, os, shutil, signal, sys, time

log, target, backup, steps = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
original = open(target, "rb").read()
shutil.copyfile(target, backup)
events = [{"event": "start", "time": time.time(), "sha256": hashlib.sha256(original).hexdigest(), "bytes": len(original)}]
stopped = False


def replace(data):
    temporary = target + ".peer-edit.tmp"
    stat = os.stat(target)
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.chmod(temporary, stat.st_mode & 0o7777)
    os.utime(temporary, ns=(stat.st_atime_ns, stat.st_mtime_ns))
    os.replace(temporary, target)


def stop(*_):
    global stopped
    stopped = True


signal.signal(signal.SIGTERM, stop)
signal.signal(signal.SIGINT, stop)
try:
    for step in steps:
        name, delay = step.split(":")
        deadline = time.time() + float(delay)
        while time.time() < deadline and not stopped:
            time.sleep(0.05)
        if stopped:
            break
        replace(original if name in ("identity", "restore") else original + b"\n")
        events.append({"event": name, "time": time.time()})
finally:
    replace(original)
    events.append({"event": "final-restore", "time": time.time(), "sha256": hashlib.sha256(open(target, "rb").read()).hexdigest()})
    with open(log, "w") as handle:
        json.dump(events, handle)
