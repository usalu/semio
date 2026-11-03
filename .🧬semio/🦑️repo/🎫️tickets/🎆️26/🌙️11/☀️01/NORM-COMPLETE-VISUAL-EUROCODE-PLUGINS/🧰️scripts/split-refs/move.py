"""🚚 Move named `#[path]` mounts from a base crate root into its -more root and re-export them from the base."""
import re, sys

def move(base, more, names):
    b = open(base, encoding="utf-8").read()
    m = open(more, encoding="utf-8").read()
    moved = []
    for n in names:
        pat = re.compile(r'(#\[path = "[^"]+"\]\n(?:pub )?mod %s;\n)' % re.escape(n))
        hit = pat.search(b)
        if not hit:
            print(f"skip {n}: not in base")
            continue
        b = b.replace(hit.group(1), "", 1)
        m = m.replace("//#endregion 🪢️Mounts", hit.group(1) + "//#endregion 🪢️Mounts", 1)
        moved.append(n)
    ru = re.search(r'pub use (semio_s_artifact_[\w]+_more)::\{([^}]*)\};', b)
    if moved and ru:
        have = [x.strip() for x in ru.group(2).split(",") if x.strip()]
        new = have + [n for n in moved if n not in have]
        b = b.replace(ru.group(0), f"pub use {ru.group(1)}::{{{', '.join(new)}}};")
    elif moved:
        print("NO re-export line in base", base)
    open(base, "w", encoding="utf-8").write(b)
    open(more, "w", encoding="utf-8").write(m)
    print("moved", moved)

if __name__ == "__main__":
    move(sys.argv[1], sys.argv[2], sys.argv[3:])
