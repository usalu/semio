from pathlib import Path

root = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript")
print("root exists", root.exists())
for p in root.iterdir():
    print(repr(p.name))
    if p.name.endswith("registry"):
        for f in p.iterdir():
            print(" ", repr(f.name), f.stat().st_size)
            if "launch" in f.name:
                t = f.read_text(encoding="utf-8")
                out = Path(__file__).with_name("_w7_launch_dump.txt")
                lines = []
                for i, line in enumerate(t.splitlines(), 1):
                    if any(k in line for k in ("policy", "Policy", "generateLaunch", "Taxonomy", "Gate", "bun", "script.ts")):
                        lines.append(f"{i}:{line[:200]}")
                out.write_text("\n".join(lines[:200]), encoding="utf-8")
                print("wrote", out, "matches", len(lines))
                # also write head
                Path(__file__).with_name("_w7_launch_head.txt").write_text("\n".join(t.splitlines()[:80]), encoding="utf-8")
