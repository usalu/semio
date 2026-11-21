"""📈 Samples one guarded process's resident KiB until it exits."""
import csv,os,subprocess,sys,time

def resident(pid):
    if os.name=="nt":
        output=subprocess.run(["tasklist","/fi","PID eq "+pid,"/fo","csv","/nh"],capture_output=True,text=True).stdout
        for row in csv.reader(output.splitlines()):
            if len(row)>4 and row[1]==pid:
                return int("".join(c for c in row[4] if c.isdigit()))
        return 0
    output=subprocess.run(["ps","-p",pid,"-o","rss="],capture_output=True,text=True).stdout.strip()
    return int(output or 0)

peak=0
while True:
    used=resident(sys.argv[1])
    if not used:break
    peak=max(peak,used)
    time.sleep(.5)
print(peak)
