"""👯️ Clones a kernel case's Gherkin into its `-twin` case: the same vectors, the same oracle tag,
the twin capability, and a description that says which case it doubles. The vectors must be
character-identical because the twin adapter reuses the base case's oracle handlers, and those read
the scenario's own data tables."""
import io, os, re, sys, json

ROOT = os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🧪️tests")

PLAN = json.load(io.open(sys.argv[1], encoding="utf-8"))

for item in PLAN:
    base_dir, twin_dir, capability, module = item["base"], item["twin"], item["capability"], item["module"]
    text = io.open(os.path.join(ROOT, base_dir, "🥒️.feature"), encoding="utf-8").read()
    lines = text.split("\n")
    lines[0] = f"@capability-{capability}"
    head = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
    first = next(i for i, l in enumerate(lines) if l.startswith("  @id-"))
    title = lines[head][len("Feature: "):]
    description = [
        f"Feature: The TypeScript twin of {module} answers as the reference implementation does",
        f"  `{base_dir}` measures the LaTeX kernel — {title[0].lower()}{title[1:]}. This case measures the",
        "  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on",
        "  the same vectors and against the same oracle, because a kernel that exists twice is only a",
        "  kernel if both copies answer alike.",
        "",
        "  The platform gives a case one adapter per language and one subject per scenario, so the twin",
        f"  cannot share `{base_dir}`'s adapter with the LaTeX probe subject. It gets its own case instead;",
        "  the tables below are that case's tables character for character, and the adapter reuses its",
        "  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.",
        "",
        "  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so",
        "  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.",
        "",
    ]
    body = lines[first:]
    out = lines[:head] + description + body
    joined = "\n".join(out)
    joined = re.sub(r"Given the committed probe documents? [^\n]*?\.tex and the ", "Given the ", joined)
    joined = re.sub(r"Given the committed probe documents? [^\n]*?\.tex and ", "Given ", joined)
    joined = re.sub(r"    Given the committed probe documents? [^\n]*?\.tex\n", "    Given the vectors of the case it doubles\n", joined)
    joined = joined.replace("Then the compiled probe and the reference implementation", "Then the twin kernel and the reference implementation")
    joined = joined.replace("Then the probe and the reference implementation", "Then the twin kernel and the reference implementation")
    joined = joined.replace("Then the compiled probe and ", "Then the twin kernel and ")
    os.makedirs(os.path.join(ROOT, twin_dir), exist_ok=True)
    io.open("tmp.feature", "w", encoding="utf-8", newline="\n").write(joined)
    os.replace("tmp.feature", os.path.join(ROOT, twin_dir, "🥒️.feature"))
    print("wrote", twin_dir)
