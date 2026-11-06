import psutil

for process in psutil.process_iter(["pid", "ppid", "name", "cmdline", "cpu_times"]):
    try:
        info = process.info
        if (info["name"] or "").lower() not in {"cargo.exe", "rustc.exe", "rustup.exe"}:
            continue
        files = [item.path for item in process.open_files() if "package-cache" in item.path.lower()]
        command = info["cmdline"] or []
        cpu = info["cpu_times"]
        print({"pid": info["pid"], "parent": info["ppid"], "name": info["name"], "cpu": round(cpu.user + cpu.system, 2), "arguments": command[:12], "cacheHandles": files})
    except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
        continue
