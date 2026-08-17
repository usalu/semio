#!/usr/bin/env python3
"""Categorizes a cargo check output into errors anchored in MY 12 stdio-media kinds' editor/viewer
files vs everything else (foreign/other-packet), so I don't chase errors outside my lease."""
import re, sys

MY_KINDS = ["📷️png", "📷️jpg", "🖼️bmp", "🖼️tiff", "🎞️gif", "🎨️svg", "🎥️mp4", "🎵️mp3", "🔊️wav", "📼️avi", "🌐️html", "📝️md"]

def main(path):
    text = open(path, encoding="utf-8").read()
    blocks = re.split(r'\n(?=error(?:\[|:))', text)
    mine = []
    foreign = []
    for b in blocks:
        if not b.startswith("error"):
            continue
        m = re.search(r'--> (\S+):(\d+):(\d+)', b)
        if not m:
            continue
        path_ = m.group(1)
        is_mine = ("✏️editor" in path_ or "👁️viewer" in path_) and any(k in path_ for k in MY_KINDS)
        (mine if is_mine else foreign).append((path_, b))
    print("mine:", len(mine), "foreign:", len(foreign))
    print("\n--- MINE (all) ---")
    for p, b in mine:
        print("="*100)
        print(b[:600])
    return mine, foreign

if __name__ == "__main__":
    main(sys.argv[1])
