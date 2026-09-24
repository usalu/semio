"""🖨️ Re-renders a registry JSON in the owner's compact house style: scalar-only arrays and objects inline, everything else indented by two."""
import json, sys
def scalar(value): return not isinstance(value, (dict, list))
def render(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value: return "{}"
        if all(scalar(v) for v in value.values()): return "{ " + ", ".join(f"{json.dumps(k, ensure_ascii=False)}: {json.dumps(v, ensure_ascii=False)}" for k, v in value.items()) + " }"
        return "{\n" + ",\n".join(f"{inner}{json.dumps(k, ensure_ascii=False)}: {render(v, depth + 1)}" for k, v in value.items()) + f"\n{pad}}}"
    if isinstance(value, list):
        if not value: return "[]"
        if all(scalar(v) for v in value): return "[" + ", ".join(json.dumps(v, ensure_ascii=False) for v in value) + "]"
        return "[\n" + ",\n".join(f"{inner}{render(v, depth + 1)}" for v in value) + f"\n{pad}]"
    return json.dumps(value, ensure_ascii=False)
for path in sys.argv[1:]:
    data = json.load(open(path, encoding="utf-8"))
    open(path, "w", encoding="utf-8").write(render(data, 0) + "\n")
