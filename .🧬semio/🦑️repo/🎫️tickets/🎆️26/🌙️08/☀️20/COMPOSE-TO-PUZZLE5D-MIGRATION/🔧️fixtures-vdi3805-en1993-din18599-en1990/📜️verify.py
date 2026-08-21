"""✅️ Re-runs `fixtures lint`'s own rules scoped to just this lane's four trees (the CLI truncates
its repo-wide error list at 40 rows), then checks every `include_str!` target and every
`#[cfg(test)]` `#[path]` mount resolves, and that no case leaks into a tree this lane does not own."""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
TREES = {
    "vdi3805": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "en1993": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "din18599": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "en1990": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
}
NON_MUTATION_DIRS = {"💾️binary", "📝️text"}
CORE = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"]

errors, stats = [], {}
for name, rel in TREES.items():
    root = os.path.join(REPO, rel)
    aggregate = open(os.path.join(root, "🦀️component.rs"), encoding="utf-8").read()
    body = re.search(r"pub enum \w*Mutation\w* \{([\s\S]*?)\n\}", aggregate)
    variants = re.findall(r"^\s+([A-Z][A-Za-z0-9]*)\(", body.group(1), re.M) if body else []
    leaves = []
    for entry in sorted(os.listdir(root)):
        leaf_mut = os.path.join(root, entry, "🦠️mutation/🦀️component.rs")
        if entry in NON_MUTATION_DIRS or not os.path.isfile(leaf_mut):
            continue
        src = open(leaf_mut, encoding="utf-8").read()
        struct = re.search(r"^pub struct ([A-Za-z0-9]+)", src, re.M)
        leaves.append((entry, struct.group(1) if struct else None))
    by_struct = {s: d for d, s in leaves if s}
    for variant in variants:
        if variant not in by_struct:
            errors.append("{}:{}: enum variant has no mutation directory".format(rel, variant))
    covered = 0
    for leaf, _ in leaves:
        cases = [c for c in sorted(os.listdir(os.path.join(root, leaf, "🧪️tests"))) if os.path.isdir(os.path.join(root, leaf, "🧪️tests", c))] if os.path.isdir(os.path.join(root, leaf, "🧪️tests")) else []
        if not cases:
            errors.append("{}/{}: no 🧪️tests cases".format(rel, leaf))
            continue
        covered += 1
        for case in cases:
            case_dir = os.path.join(root, leaf, "🧪️tests", case)
            label = "{}/{}/{}".format(rel, leaf, case)
            outcome_file = os.path.join(case_dir, "🎯️outcome/🔣️component.json")
            rejected = False
            if os.path.exists(outcome_file):
                outcome = json.load(open(outcome_file, encoding="utf-8"))
                rejected = outcome.get("status") == "rejected"
                if outcome.get("status") not in ("applied", "rejected"):
                    errors.append("{}: bad 🎯️outcome.status".format(label))
                if rejected and not isinstance(outcome.get("code"), str):
                    errors.append("{}: rejected outcome must carry a code".format(label))
            else:
                errors.append("{}: missing 🎯️outcome".format(label))
            for relative in CORE:
                if rejected and relative.startswith("🔺️diff/"):
                    continue
                if not os.path.exists(os.path.join(case_dir, relative)):
                    errors.append("{}: missing {}".format(label, relative))
            if rejected:
                marker = os.path.join(case_dir, "🔺️diff/🚫️component.absent")
                if not os.path.exists(marker):
                    errors.append("{}: rejected case must carry 🔺️diff/🚫️component.absent".format(label))
                elif os.path.getsize(marker) != 0:
                    errors.append("{}: 🚫️component.absent must be zero bytes".format(label))
                if os.path.exists(os.path.join(case_dir, "🔺️diff/🔣️component.json")):
                    errors.append("{}: rejected case must NOT also carry a diff JSON".format(label))
            for side in ("⬅️before", "➡️after"):
                side_dir = os.path.join(case_dir, "📸️snapshot", side)
                if not os.path.isdir(side_dir):
                    errors.append("{}: missing 📸️snapshot/{}".format(label, side))
                elif not os.path.exists(os.path.join(side_dir, "🔣️component.json")):
                    errors.append("{}: 📸️snapshot/{} is missing 🔣️component.json".format(label, side))
            # every include_str! target beside the test file must exist
            test_rs = os.path.join(case_dir, "🦀️component.rs")
            if os.path.exists(test_rs):
                for target in re.findall(r'include_str!\("([^"]+)"\)', open(test_rs, encoding="utf-8").read()):
                    if not os.path.exists(os.path.join(case_dir, target)):
                        errors.append("{}: include_str! target {} does not exist".format(label, target))
            # every committed JSON must actually parse
            for dirpath, _, files in os.walk(case_dir):
                for f in files:
                    if f.endswith(".json"):
                        try:
                            json.load(open(os.path.join(dirpath, f), encoding="utf-8"))
                        except Exception as exc:
                            errors.append("{}: {} is not valid JSON: {}".format(label, f, exc))
    # every #[path] in the self-wired fixture_tests block must resolve
    block = re.search(r"//#region 🧪️FixtureTests([\s\S]*?)//#endregion 🧪️FixtureTests", aggregate)
    mounts = re.findall(r'#\[path = "([^"]+)"\]', block.group(1)) if block else []
    mods = re.findall(r"^\s+mod (\w+);", block.group(1), re.M) if block else []
    for mount in mounts:
        if not os.path.exists(os.path.join(root, mount)):
            errors.append("{}: mounted #[path] {} does not resolve".format(rel, mount))
    if len(set(mods)) != len(mods):
        errors.append("{}: duplicate mod name in fixture_tests".format(rel))
    stats[name] = (covered, len(leaves), len(mounts), len(mods))

for name, (covered, total, mounts, mods) in stats.items():
    print("{:<10} {}/{} leaves covered · {} mounted #[path] · {} mod lines".format(name, covered, total, mounts, mods))
print("scoped errors: {}".format(len(errors)))
for e in errors:
    print("  ❌️ " + e)
sys.exit(1 if errors else 0)
