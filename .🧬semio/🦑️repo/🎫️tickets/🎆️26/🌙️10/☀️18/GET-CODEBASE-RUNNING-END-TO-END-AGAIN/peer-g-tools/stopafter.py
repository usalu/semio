"""🛑 peer-g: kills the chain driver (own pid) the moment the named stage record exists, so no further official run starts: `stopafter.py <pid> <json>`."""
import os, signal, sys, time
pid, path = int(sys.argv[1]), sys.argv[2]
while not os.path.exists(path):
    time.sleep(0.05)
os.kill(pid, signal.SIGKILL)
print("stopped", pid, time.strftime("%H:%M:%S"))
