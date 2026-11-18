import subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__))
for scales in (["50", "100"], ["50"]):
    print(subprocess.run(["python3", HERE + "/vegetation-inflate.py", "agave"] + scales, capture_output=True, text=True).stdout.strip())
