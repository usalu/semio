import subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__))
for _ in range(4):
    print(subprocess.run(["python3", HERE + "/vegetation-inflate.py", "miscanthus-grass", "50"], capture_output=True, text=True).stdout.strip())
