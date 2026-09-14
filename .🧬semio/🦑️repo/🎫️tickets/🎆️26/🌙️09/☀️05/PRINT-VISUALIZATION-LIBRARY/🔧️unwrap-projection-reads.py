"""🔧️ Fixes the OTHER half of the `{ projection: … }` wrapper mistake in the print test adapters.

`🔧️unwrap-subject-projection.py` removed the double wrap where a scenario returned a helper's
result directly. The call sites that READ a key out of that result were left indexing the WRAPPER,
so `(await subject(ctx, "bracket.tex"))["geometry/annotation-bracket"]` is always `undefined` and the
probe's numbers never reach the comparison — the subject reports an empty array while the compiled
document holds exactly the values the oracle expects. Only helpers this file declares as returning
`Promise<{ projection: ProbeProjection }>` are rewritten. Idempotent."""
import io, os, re, sys

ROOT = "🧰️framework/🛍️products/📓️print/🧪️tests"
os.chdir(sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio")
total = 0
for case in sorted(os.listdir(ROOT)):
    path = os.path.join(ROOT, case, "🟦️.ts")
    if not os.path.isfile(path):
        continue
    text = io.open(path, encoding="utf-8").read()
    helpers = re.findall(r"async function ([A-Za-z][A-Za-z0-9]*)\([^)]*\): Promise<\{ projection: ProbeProjection \}>", text)
    if not helpers:
        continue
    names = "|".join(sorted(set(helpers), key=len, reverse=True))
    count = 0
    # 🔑 `(await helper(…))["key"]` reads a key of the wrapper instead of the projection.
    pattern = re.compile(rf"\(await ({names})\(((?:[^()]|\([^()]*\))*)\)\)\[")
    text, n = pattern.subn(lambda m: f"(await {m.group(1)}({m.group(2)})).projection[", text)
    count += n
    # 🔑 `const projection = await helper(…);` binds the wrapper, then every read below misses.
    pattern = re.compile(rf"const projection = await ({names})\(((?:[^()]|\([^()]*\))*)\);")
    text, n = pattern.subn(lambda m: f"const projection = (await {m.group(1)}({m.group(2)})).projection;", text)
    count += n
    if count:
        io.open("tmp.ts", "w", encoding="utf-8", newline="\n").write(text)
        os.replace("tmp.ts", path)
        print(f"{case}: {count}")
        total += count
print(f"total {total}")
