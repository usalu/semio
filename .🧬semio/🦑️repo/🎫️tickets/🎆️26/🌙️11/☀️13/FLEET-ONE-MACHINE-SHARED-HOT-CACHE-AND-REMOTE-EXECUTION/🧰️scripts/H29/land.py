import json, os, shutil, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/ueli/Documents/semio/"
TICKET = LIVE + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/"
B = LIVE + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
AREAS = ["ready", "remote-execution", "test-file"]
LANE = "H29"

def lock_path(area):
    return TICKET + "landing.lock." + area

def acquire():
    taken = []
    for area in AREAS:
        path = lock_path(area)
        for attempt in range(60):
            try:
                os.mkdir(path)
                break
            except FileExistsError:
                try:
                    age = time.time() - os.path.getmtime(path)
                except OSError:
                    age = 0
                if age > 40 * 60:
                    shutil.rmtree(path, ignore_errors=True)
                    continue
                time.sleep(6)
        else:
            for done in taken:
                shutil.rmtree(lock_path(done), ignore_errors=True)
            raise SystemExit("lock %s held" % area)
        with open(path + "/owner", "w", encoding="utf-8") as handle:
            handle.write("%s %s\n" % (LANE, time.strftime("%Y-%m-%d %H:%M:%S")))
        taken.append(area)
    return taken

def release():
    for area in AREAS:
        path = lock_path(area)
        try:
            owner = open(path + "/owner", encoding="utf-8").read()
        except OSError:
            continue
        if owner.startswith(LANE + " "):
            shutil.rmtree(path, ignore_errors=True)

def place_new():
    manifest = json.load(open(HERE + "/new/manifest.json", encoding="utf-8"))
    for short, relative in manifest.items():
        data = open(HERE + "/new/" + short, "rb").read()
        target = B + relative
        if os.path.exists(target) and open(target, "rb").read() == data:
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        fd, temporary = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".h29-")
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
        os.replace(temporary, target)
        print("new", relative)

if __name__ == "__main__":
    command = sys.argv[1]
    if command == "release":
        AREAS[:] = ["ready", "remote-execution", "test-file"]
        release()
    elif command == "place":
        AREAS[:] = ["remote-execution", "test-file"]
        acquire()
        place_new()
        print("placed; locks held until `land.py release`")
    elif command == "land":
        acquire()
        place_new()
        sys.argv = ["apply.py", LIVE.rstrip("/"), "live"]
        exec(compile(open(HERE + "/apply.py", encoding="utf-8").read(), "apply.py", "exec"))
        print("landed; locks held until `land.py release`")
