import os, re, subprocess, sys

ROOT = r"C:\git\semio"
FF = os.path.join(ROOT, ".🧬semio", "🎓️teaching", "⚡️cache", "python", "Lib", "site-packages", "imageio_ffmpeg", "binaries", "ffmpeg-win-x86_64-v7.1.exe")
TK = os.path.join(ROOT, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️11", "☀️13", "APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS")
GEN = os.path.join(TK, "🗑️generated", "check-final-b")
VIDS = {
    "vb": os.path.join(ROOT, ".🧬semio", "🎓️teaching", "⚡️cache", "vb", "final", "de", "speaker", "rendered", "Full_Heating_Demand_speaker_de_1080p30.mp4"),
    "vd": os.path.join(ROOT, ".🧬semio", "🎓️teaching", "⚡️cache", "vd", "final", "de", "speaker", "rendered", "Full_EnergyBalance_speaker_de_1080p30.mp4"),
}


def duration(path):
    p = subprocess.run([FF, "-hide_banner", "-i", path], capture_output=True)
    err = p.stderr.decode("utf-8", errors="replace")
    m = re.search(r"Duration: (\d+):(\d+):(\d+\.\d+)", err)
    h, mi, s = m.groups()
    return int(h) * 3600 + int(mi) * 60 + float(s)


def extract(key, step=5.0, size="1280:720"):
    out = os.path.join(GEN, key)
    os.makedirs(out, exist_ok=True)
    dur = duration(VIDS[key])
    t = 0.0
    n = 0
    while t < dur:
        name = os.path.join(out, f"f_{int(t):05d}.png")
        subprocess.run([FF, "-hide_banner", "-loglevel", "error", "-y", "-ss", f"{t:.3f}", "-i", VIDS[key],
                        "-frames:v", "1", "-vf", f"scale={size}", name], check=True)
        t += step
        n += 1
    return dur, n


if __name__ == "__main__":
    key = sys.argv[1]
    if len(sys.argv) > 2 and sys.argv[2] == "one":
        ts = [float(x) for x in sys.argv[3:]]
        out = os.path.join(GEN, key + "_full")
        os.makedirs(out, exist_ok=True)
        for t in ts:
            subprocess.run([FF, "-hide_banner", "-loglevel", "error", "-y", "-ss", f"{t:.3f}", "-i", VIDS[key],
                            "-frames:v", "1", "-vf", "scale=1280:720", os.path.join(out, f"x_{t:07.2f}.png")], check=True)
        print("done", out)
    else:
        print(key, extract(key))
