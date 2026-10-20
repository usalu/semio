#!/usr/bin/env python3
"""build-native.py OUTDIR [--test]: compiles the std-only native programs of the resident service with plain rustc (no cargo, no crates).
Outputs semio-repo-watch, semio-repo-forward and faketool (test fixture) into OUTDIR; --test also runs their unit tests."""
import os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
out = os.path.abspath(sys.argv[1])
os.makedirs(out, exist_ok=True)
exe = ".exe" if os.name == "nt" else ""
sources = {"semio-repo-watch": "watch.rs", "semio-repo-forward": "forward.rs", "faketool": "faketool.rs"}
for name, source in sources.items():
    path = os.path.join(here, "dev", "native", source)
    subprocess.run(["rustc", "--edition", "2021", "-O", "--check-cfg", "cfg(semio_check_linux)", "--check-cfg", "cfg(semio_check_windows)", "-o", os.path.join(out, name + exe), path], check=True)
    if "--test" in sys.argv and name != "faketool":
        test = os.path.join(out, name + "-tests" + exe)
        subprocess.run(["rustc", "--edition", "2021", "--test", "--check-cfg", "cfg(semio_check_linux)", "--check-cfg", "cfg(semio_check_windows)", "-o", test, path], check=True)
        subprocess.run([test], check=True)
print("built", ", ".join(sources), "into", out)
