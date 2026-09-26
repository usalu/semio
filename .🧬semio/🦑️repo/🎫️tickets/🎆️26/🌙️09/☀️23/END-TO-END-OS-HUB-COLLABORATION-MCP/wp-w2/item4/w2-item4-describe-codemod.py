"""🪓️ W2 item 4 one-off codemod: every component's `describe` becomes the inferred Nx target (📚️library/🟨️.mjs componentTargets,
dependsOn component-dev), so the 60 hand-declared `describe` targets and the 60 per-crate `DescribeScript` wrappers are deleted.
usage: python3 w2-item4-describe-codemod.py [--apply]   (default: dry run, prints a unified diff summary per file)"""
import difflib, json, os, re, subprocess, sys
REPO = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
os.chdir(REPO)
def git_files(pattern, *paths):
    return [f for f in subprocess.run(["git", "grep", "-l", pattern, "--", *paths], capture_output=True, text=True).stdout.split("\n") if f]
changed, problems = [], []
def emit(path, old, new):
    if old == new: return
    changed.append(path)
    if APPLY:
        open(path, "w").write(new)
    else:
        diff = list(difflib.unified_diff(old.splitlines(), new.splitlines(), path, path, n=0, lineterm=""))
        print(f"--- {path}: {sum(1 for l in diff if l.startswith('-') and not l.startswith('---'))} removed, {sum(1 for l in diff if l.startswith('+') and not l.startswith('+++'))} added")
for path in git_files('"describe"', "*📋️project.json"):
    text = open(path).read()
    data = json.loads(text)
    target = data.get("targets", {}).get("describe")
    if not target or data.get("name") == "@semio-tech/os-plugin-describe-rs": continue
    if target.get("options", {}).get("command") != "bun ./📜️script.ts describe": problems.append(f"{path}: unexpected describe command"); continue
    del data["targets"]["describe"]
    key = text.find('"describe": {')
    depth, end = 0, None
    for i in range(key + len('"describe": '), len(text)):
        depth += {"{": 1, "}": -1}.get(text[i], 0)
        if depth == 0: end = i + 1; break
    before = text[:key].rstrip(" \t\n")
    after = text[end:]
    if before.endswith(","): new = before[:-1] + after
    elif after.lstrip(" \t\n").startswith(","): new = before + after.lstrip(" \t\n")[1:]
    else: new = before + after
    if json.loads(new) != data: problems.append(f"{path}: textual removal differs from the parsed removal"); continue
    emit(path, text, new)
for path in git_files("describePluginComponent\\|describeExtensionComponent", "✏️s/*📜️script.ts"):
    text = open(path).read()
    new, n_import = re.subn(r'^import \{ describe(?:Plugin|Extension)Component \} from "[^"]+🏭️fresh-component/🟦️\.ts";\n', "", text, flags=re.M)
    lazy = r'    const \{ describePluginComponent \} = await import\("[^"]+🏭️fresh-component/🟦️\.ts"\);\n'
    new, n_class = re.subn(r'\n(?:/\*\*(?:(?!\*/).)*?\*/\n)?class DescribeScript extends BundleScript \{\n  (?:async )?run\(\): (?:void|Promise<void>) \{\n(?:' + lazy + r')?    process\.exit\(describe(?:Plugin|Extension)Component\([^;]*\)\);\n  \}\n\}\n', "", new, flags=re.S)
    if n_import == 0 and re.search(lazy, text): n_import = 1
    new, n_reg = re.subn(r'\s*\.register\("describe", DescribeScript\)', "", new)
    new = re.sub(r'(<[a-z|]*)\|describe(\|[a-z|]*>|>)', r'\1\2', new)
    new = re.sub(r'<test>', "test", new)
    if (n_import, n_class, n_reg) != (1, 1, 1): problems.append(f"{path}: import={n_import} class={n_class} register={n_reg}"); continue
    for name in ("join",):
        if re.search(rf'^import \{{ {name} \}} from "node:path";\n', new, re.M) and len(re.findall(rf'\b{name}\(', new)) == 0:
            new = re.sub(rf'^import \{{ {name} \}} from "node:path";\n', "", new, flags=re.M)
    if "DescribeScript" in new or "describePluginComponent" in new or "describeExtensionComponent" in new: problems.append(f"{path}: leftover reference"); continue
    emit(path, text, new)
print(f"{'APPLIED' if APPLY else 'DRY RUN'}: {len(changed)} files changed, {len(problems)} problems")
for p in problems: print("PROBLEM", p)
