import sys, re, subprocess
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-H2"
asset, scale, pose, cx, cy, half = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5]), float(sys.argv[6])
s = open(f"{G}/dbg-{asset}-{scale}-{pose}.svg").read()
s = re.sub(r'viewBox="[^"]*" width="[^"]*" height="[^"]*"', f'viewBox="{cx-half} {cy-half} {2*half} {2*half}" width="{int(2*half*200)}" height="{int(2*half*200)}"', s, count=1)
open(f"{G}/crop.svg", "w").write(s)
subprocess.run(["rsvg-convert", f"{G}/crop.svg", "-o", f"{G}/crop.png"], check=True)
print("ok")
