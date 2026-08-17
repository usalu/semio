from pathlib import Path
node = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in node.iterdir() if x.name.endswith("puzzle"))
apps = next(x for x in puzzle.iterdir() if "apps" in x.name)
twod = next(x for x in apps.iterdir() if "2d" in x.name)
f = twod / "🎮️commands" 
# find node component
for p in twod.rglob('*.rs'):
    if 'node' in str(p) and 'commands' in str(p):
        t=p.read_text()
        print('FILE', p)
        print(t[:5000])
        print('----len', len(t))
        if len(t)>5000:
            print(t[5000:10000])
