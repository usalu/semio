import json
import os
import glob

TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

d = json.load(open(f"{TICKET}/🗑️generated/a4-repair-plan.json"))
plan = d["plan"]

# group unique (scope, scheme, old_uri, new_uri)
by_scope = {}
for p in plan:
    by_scope.setdefault(p["scope"], []).append(p)

total_file_replacements = 0
total_adapter_replacements = 0
report = []

for scope, entries in by_scope.items():
    case_dir = os.path.dirname(scope)
    with open(scope, "r", encoding="utf-8") as f:
        content = f.read()
    orig_content = content
    file_repl_count = 0
    for e in entries:
        scheme = e["scheme"]
        old_uri = e["old_uri"]
        new_uri = e["new_uri"]
        full_old = f"{scheme}://{old_uri}"
        full_new = f"{scheme}://{new_uri}"
        n1 = content.count(full_old)
        content = content.replace(full_old, full_new)
        # also bare backtick-quoted mention without scheme, e.g. `old_uri`
        bare_old = f"`{old_uri}`"
        bare_new = f"`{new_uri}`"
        n2 = content.count(bare_old)
        content = content.replace(bare_old, bare_new)
        file_repl_count += n1 + n2

    if content != orig_content:
        with open(scope, "w", encoding="utf-8") as f:
            f.write(content)
    total_file_replacements += file_repl_count

    # adapters in same case dir referencing the old uris
    adapter_repl_count = 0
    for adapter_path in glob.glob(os.path.join(case_dir, "*.rs")) + \
                         glob.glob(os.path.join(case_dir, "*.py")) + \
                         glob.glob(os.path.join(case_dir, "*.ts")):
        with open(adapter_path, "r", encoding="utf-8", errors="ignore") as f:
            acontent = f.read()
        aorig = acontent
        for e in entries:
            scheme = e["scheme"]
            old_uri = e["old_uri"]
            new_uri = e["new_uri"]
            full_old = f"{scheme}://{old_uri}"
            full_new = f"{scheme}://{new_uri}"
            acontent = acontent.replace(full_old, full_new)
            acontent = acontent.replace(f"`{old_uri}`", f"`{new_uri}`")
        if acontent != aorig:
            with open(adapter_path, "w", encoding="utf-8") as f:
                f.write(acontent)
            adapter_repl_count += 1
    total_adapter_replacements += adapter_repl_count

    report.append({
        "scope": scope,
        "entries": len(entries),
        "file_replacements": file_repl_count,
        "adapters_touched": adapter_repl_count,
    })

with open(f"{TICKET}/🗑️generated/a4-apply-report.json", "w") as f:
    json.dump(report, f, indent=2, ensure_ascii=False)

print("files touched:", len(by_scope))
print("total literal replacements in feature files:", total_file_replacements)
print("adapters touched:", total_adapter_replacements)
zero = [r for r in report if r["file_replacements"] == 0]
print("scopes with ZERO replacements (investigate):", len(zero))
for z in zero:
    print("  ", z)
