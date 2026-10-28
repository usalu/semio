"""🔢 Counts the directories the Cargo owner discovery walk visits (skips dot names, opaque names, symlinks) and their summed depth (= ancestor lstat calls of the custody check); `python3 -I walkcount.py <root> [subdir]`."""
import os, sys, time
root = sys.argv[1]; sub = sys.argv[2] if len(sys.argv) > 2 else ''
opaque = {"node_modules","target","dist","build","🤖️generated","🗑️generated","coverage","temp","compose"}
t = time.time(); dirs = 0; depth_sum = 0; cargo = 0; top = {}
stack = [sub]
while stack:
    d = stack.pop(); dirs += 1; depth = len(d.split('/')) if d else 0; depth_sum += depth + 1
    k = d.split('/')[0] if d else ''
    try: it = os.scandir(os.path.join(root, d))
    except OSError: continue
    with it:
        for e in it:
            if e.name.startswith('.') or e.name in opaque or e.is_symlink(): continue
            p = d + '/' + e.name if d else e.name
            if e.is_dir(follow_symlinks=False): stack.append(p); top[p.split('/')[0]] = top.get(p.split('/')[0], 0) + 1
            elif e.name == 'Cargo.toml': cargo += 1
print(f'dirs {dirs} ancestor-lstats {depth_sum} Cargo.toml {cargo} in {time.time()-t:.1f}s')
for k, v in sorted(top.items(), key=lambda x: -x[1])[:12]: print(f'  {v:>8} {k}')
