import subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__))
print(subprocess.run(["python3", HERE + "/vegetation-inflate.py", "london-plane-pollarded", "50", "100", "200"], capture_output=True, text=True).stdout.strip())
