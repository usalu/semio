#!/usr/bin/env python3
"""[mem3] Instruments an image's loader.js copy with phase marks (not a library file): globalThis.__m2mark(label) records RSS, ru_maxrss, JSC heap and (M2_FP != 0) the macOS physical footprint; marks L-start, L-read (snapshot read + digest), L-admit (parse + admission), L-imported (entry module graph evaluated). usage: loadermarks.py <image dir> (writes loader.marked.js)."""
import pathlib, sys

PRELUDE = r"""
{const {heapStats}=await import('bun:jsc'),cp=await import('node:child_process'),t0=performance.now(),unit={B:1/1048576,KB:1/1024,MB:1,GB:1024};globalThis.__m2=[];
globalThis.__m2mark=label=>{if(process.env.M2_GC==='1')Bun.gc(true);const m=process.memoryUsage(),h=heapStats(),row={label,t:Math.round(performance.now()-t0),rss:Math.round(m.rss/1048576),maxRss:Math.round(process.resourceUsage().maxRSS/1024),heap:Math.round(h.heapSize/1048576),extra:Math.round(h.extraMemorySize/1048576),objects:h.objectCount};
if(process.env.M2_FP!=='0'&&process.platform==='darwin')try{const text=cp.execFileSync('/usr/bin/footprint',['-p',String(process.pid)]).toString(),num=re=>{const x=re.exec(text);return x?Math.round(Number(x[1])*unit[x[2]]):undefined;};row.fp=num(/phys_footprint: ([\d.]+) (B|KB|MB|GB)/);row.fpPeak=num(/phys_footprint_peak: ([\d.]+) (B|KB|MB|GB)/);row.top=text.split('\n').filter(line=>/^\s*[\d.]+ (B|KB|MB|GB)\s+[\d.]+ (B|KB|MB|GB)\s+[\d.]+ (B|KB|MB|GB)\s+\d+\s+\S/.test(line)).slice(0,4).map(line=>line.trim().replace(/\s+/g,' ')).join(' | ');}catch{}
globalThis.__m2.push(row);process.stderr.write('[DEBUG] M2 '+JSON.stringify(row)+'\n');if(process.env.M2_STOP===label){process.stderr.write('[DEBUG] M2STOP '+JSON.stringify(globalThis.__m2)+'\n');process.exit(0);}};
globalThis.__m2mark('L-start');}
"""
ANCHORS = [
    ("check();if(hash(encoded)!==expected)throw refusal('snapshot-changed',{});", "globalThis.__m2mark('L-read');"),
    ("const entry=path.join(value.root,value.entry);", "globalThis.__m2images=images;globalThis.__m2mark('L-admit');"),
    ("const module=await import(entry);check();", "globalThis.__m2mark('L-imported');"),
]


def main() -> None:
    image = pathlib.Path(sys.argv[1])
    text = (image / "loader.js").read_text(encoding="utf-8")
    for anchor, mark in ANCHORS:
        if text.count(anchor) != 1:
            raise SystemExit(f"[DEBUG] loader anchor not exactly once: {anchor}")
        text = text.replace(anchor, anchor + mark)
    (image / "loader.marked.js").write_text(PRELUDE + text, encoding="utf-8")


if __name__ == "__main__":
    main()
