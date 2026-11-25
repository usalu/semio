"""🚚️ Lands the measured trial of one person as the catalog document, carrying over what other workers changed in the catalog file meanwhile: `python3 carry.py <catalog.json> <backup.json> <trial.json> [--write]`. The trial is the backup with the ops of the batch replayed (`solve.sh`); every top-level member of the asset that the ops do not own (everything but `representations` and `anchors`, and inside poses and clips nothing) is taken from the catalog file as it is now (the tag objects written catalog-wide on 2026-11-25 19:09, say). It refuses when the catalog file differs from the backup in a member the ops own beyond what the earlier ops of the batch wrote there, unless `--over` states that those earlier ops are replaced by the trial."""
import json, sys

catalog_path, backup_path, trial_path = sys.argv[1:4]
write, over = "--write" in sys.argv, "--over" in sys.argv
text = open(catalog_path, encoding="utf8").read()
catalog, backup, trial = json.loads(text), json.load(open(backup_path, encoding="utf8")), json.load(open(trial_path, encoding="utf8"))
owned = ("representations", "anchors")
foreign = [key for key in catalog["asset"] if key not in owned and catalog["asset"][key] != backup["asset"].get(key)]
touched = [key for key in backup["asset"] if key not in owned and trial["asset"].get(key) != backup["asset"][key]]
ours = [key for key in owned if catalog["asset"].get(key) != backup["asset"].get(key)]
print(f"members changed by others since the backup: {foreign or 'none'}; members beside {owned} the ops changed: {touched or 'none'}; owned members the catalog file already differs in: {ours or 'none'}")
if set(foreign) & set(touched):
    sys.exit(f"refused: both the ops and another worker changed {sorted(set(foreign) & set(touched))}")
if ours and not over:
    sys.exit("refused: the catalog file carries earlier ops; pass --over to replace them by the trial")
result = {**trial, "asset": {key: (catalog["asset"][key] if key in foreign else trial["asset"][key]) for key in trial["asset"]}}
for key in catalog["asset"]:
    if key not in result["asset"]:
        sys.exit(f"refused: the catalog file has a member the trial lacks: {key}")
for key in catalog:
    if key != "asset" and catalog[key] != trial.get(key):
        result[key] = catalog[key]
second = text.split("\n", 2)[1] if "\n" in text else ""
indent = len(second) - len(second.lstrip())
out = json.dumps(result, ensure_ascii=False, indent=indent or None) + ("\n" if text.endswith("\n") else "")
if write:
    if open(catalog_path, encoding="utf8").read() != text:
        sys.exit("refused: the catalog file changed while this ran")
    open(catalog_path, "w", encoding="utf8").write(out)
print(f"{'written' if write else 'dry run'}: {len(out)} characters, catalog had {len(text)}")
