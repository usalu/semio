"""🐍️ Independent Python oracle of the scheduling split of a narrow implementation scope (standard library only): it takes the closure the executor oracle walks, assigns every file to the scheduling overlay when its path lies under one of the named directory prefixes and to the keyed part otherwise, and lists every import edge from a keyed file into an overlay file that is not a declared port; the production code and the TypeScript compiler law must agree with it."""
import importlib.util, json, os, re, sys

EXECUTOR_ORACLE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🎬️executor", "🧪️tests", "🐍️oracle", "🐍️.py")
spec = importlib.util.spec_from_file_location("executor_oracle", EXECUTOR_ORACLE)
executor = importlib.util.module_from_spec(spec)
spec.loader.exec_module(executor)


def edges(path):
    text = open(path, encoding="utf-8").read()
    found = []
    for match in executor.STATIC.finditer(text):
        specifier = match.group(2)
        if specifier.startswith("."):
            found.append(os.path.normpath(os.path.join(os.path.dirname(path), specifier)))
    for match in executor.JSON_LITERAL.finditer(text):
        found.append(os.path.normpath(os.path.join(os.path.dirname(path), match.group(2))))
    return found


DYNAMIC = re.compile(r"""\b(?:import|require)\(\s*(["'`])([^"'`\n]+)\1\s*\)""")


def bare(path):
    text = open(path, encoding="utf-8").read()
    found = [match.group(2) for match in executor.STATIC.finditer(text)] + [match.group(2) for match in DYNAMIC.finditer(text)]
    return [specifier for specifier in found if not specifier.startswith((".", "node:", "bun:")) and specifier != "bun"]


if __name__ == "__main__":
    request = json.load(sys.stdin)
    held = executor.closure(request["entry"], request.get("roots", []))
    held |= {os.path.normpath(leaf) for leaf in request.get("leaves", []) if executor.exists(os.path.normpath(leaf))}
    prefixes = tuple(request["prefixes"])
    ports = {os.path.normpath(port) for port in request["ports"]}
    overlay = sorted(path for path in held if path.startswith(prefixes))
    keyed = sorted(path for path in held if not path.startswith(prefixes))
    violations = []
    for path in keyed:
        if path.endswith(".json"):
            continue
        for target in edges(path):
            if target in held and target.startswith(prefixes) and target not in ports:
                violations.append([path, target])
    leaves = {os.path.normpath(leaf) for leaf in request.get("leaves", [])}
    external = sorted([path, specifier] for path in held - leaves if not path.endswith(".json") for specifier in bare(path))
    print(json.dumps({"keyed": keyed, "overlay": overlay, "violations": sorted(violations), "bare": external}))
