"""🔎 Resolves a tracked file by path substrings. usage: from tt5_find import find; find('🖍️draw','🎨️fill/🧪️tests')"""
import subprocess
def find(*subs, ext=None):
    out = subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","--","*.ts","*.tsx"], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio").stdout.split("\n")
    r = [x for x in out if x and all(s in x for s in subs)]
    assert len(r) == 1, (subs, r[:5])
    return "/Users/ueli/Documents/semio/" + r[0]
